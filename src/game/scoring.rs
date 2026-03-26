//! Модуль системы очков и уровней.
//!
//! # Ответственность
//! - Начисление очков за фигуры и линии
//! - Повышение уровня
//! - Управление комбо
//! - Поиск и удаление заполненных линий
//!
//! # Зависимости
//! - [`state.rs`](super::state): константы очков, `GameState`
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`
//!
//! ## Основные функции:
//! - [`find_full_rows()`] - поиск заполненных линий
//! - [`remove_rows()`] - удаление заполненных линий
//! - [`handle_hold()`] - обработка удержания фигуры
//!
//! ## Таблица очков
//! | Действие | Формула | Пример |
//! |----------|---------|--------|
//! | 1 линия | 100 × 2^0 | 100 |
//! | 2 линии | 100 × 2^1 | 200 |
//! | 3 линии | 100 × 2^2 | 400 |
//! | 4 линии (Tetris) | 100 × 2^3 + 1000 | 1800 |
//! | Soft Drop | 1 × ячейки | 1 за ячейку |
//! | Hard Drop | 2 × ячейки | 2 за ячейку |
//! | Комбо | 50 × (комбо - 1) | 50 за 2-е комбо |
//! | Повышение уровня | 500 × (уровень - 1) | 500 за уровень 2 |
//!
//! ## Архитектурные заметки
//! ## Разделение ответственности (Problem 2.3, 2.9)
//! Этот модуль отвечает ТОЛЬКО за начисление очков и управление линиями.
//!
//! TODO (#архитектура): Избегать прямого изменения полей GameState.
//! В настоящее время используется прямой доступ к полям для производительности.
//! Рассмотреть возможность использования методов GameState для модификации.
//!
//! ## Доступ к полям GameState
//! Модуль scoring.rs имеет доступ к `pub(crate)` полям GameState,
//! так как находится в том же модуле game. Это допустимо для внутренней логики,
//! но для нового кода рекомендуется использовать геттеры/сеттеры.
//!
//! TODO (#архитектура): Выделить систему комбо в отдельный модуль `combo_system.rs`
//! для улучшения тестируемости и переиспользования.

use super::state::{
    GameState, COMBO_BONUS, LEVEL_BONUS_MULT, LINES_PER_LEVEL, LINE_SCORES, MAX_LINES_PER_CLEAR,
    SOFT_DROP_POINTS, SPD_INC,
};
use crate::io::GRID_HEIGHT;
use crate::tetromino::Tetromino;

/// Найти все заполненные линии.
///
/// # Возвращает
/// Битовую маску заполненных линий и количество заполненных линий
///
/// # Производительность
/// O(n) сложность где n = `GRID_HEIGHT` (20 итераций).
/// Используется `.all()` с ранним выходом при обнаружении пустой ячейки.
///
/// # Исправление #6
/// Исправлено: не требуется .take() так как `row` имеет тип `[i8; GRID_WIDTH]`
/// и итерация происходит по всем элементам массива фиксированного размера.
#[must_use]
pub fn find_full_rows(blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT]) -> (u32, u32) {
    let mut rows_mask: u32 = 0;
    let mut remove_count = 0;

    // Поиск заполненных линий
    for (y, row) in blocks.iter().enumerate() {
        // Оптимизация: .all() делает ранний выход при первом false
        // Исправление #2.4: убран .take(GRID_WIDTH) как избыточный
        let row_full = row.iter().all(|&cell| cell != -1);
        if row_full {
            rows_mask |= 1 << y;
            remove_count += 1;
        }
    }

    (rows_mask, remove_count)
}

/// Удалить заполненные линии и сдвинуть верхние линии вниз.
///
/// # Аргументы
/// * `blocks` - игровое поле (изменяемое)
/// * `rows_mask` - битовая маска заполненных линий
pub fn remove_rows(blocks: &mut [[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT], rows_mask: u32) {
    // Проверка валидности rows_mask
    if rows_mask >= (1u32 << GRID_HEIGHT) {
        eprintln!(
            "Предупреждение: rows_mask ({}) выходит за пределы поля (максимум {})",
            rows_mask,
            (1u32 << GRID_HEIGHT) - 1
        );
        return;
    }

    // Подсчитываем количество строк для удаления снизу вверх
    let mut rows_removed_below = 0;

    for y in (0..GRID_HEIGHT).rev() {
        if (rows_mask & (1 << y)) != 0 {
            rows_removed_below += 1;
        } else if rows_removed_below > 0 {
            // Перемещаем строку вниз на rows_removed_below позиций
            if y + rows_removed_below < GRID_HEIGHT {
                blocks[y + rows_removed_below] = blocks[y];
            }
        }
    }

    // Заполняем верхние строки пустыми значениями (-1)
    for row in blocks.iter_mut().take(rows_removed_below) {
        *row = [-1; crate::io::GRID_WIDTH];
    }
}

/// Обновить счёт, уровень и скорость после удаления линий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `remove_count` - количество удалённых линий
pub fn update_score_and_level(state: &mut GameState, remove_count: u32) {
    if remove_count > 0 {
        // Ограничение remove_count максимум 4
        let capped_remove_count = remove_count.min(MAX_LINES_PER_CLEAR);

        // Обновление количества удалённых линий
        state.lines_cleared = state.lines_cleared.saturating_add(capped_remove_count);

        // Проверка повышения уровня (каждые 10 линий)
        let new_level = (state.lines_cleared / LINES_PER_LEVEL) + 1;
        if new_level > state.level {
            state.level = new_level;
            // Бонус за повышение уровня
            state.score = state
                .score
                .saturating_add(LEVEL_BONUS_MULT.saturating_mul(u128::from(new_level - 1)));
        }

        // Увеличение скорости игры
        state.fall_spd += SPD_INC * capped_remove_count as f32;

        // Начисление очков за линии (lookup таблица)
        if capped_remove_count > 0 {
            let line_score = LINE_SCORES[(capped_remove_count - 1) as usize];
            state.score = state.score.saturating_add(line_score);
        }
    }
}

/// Обработать приземление фигуры и начислить очки.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Возвращает
/// - `Some(UpdateEndState::Lost)` - проигрыш
/// - `Some(UpdateEndState::Won)` - победа (завершение режима)
/// - `None` - продолжить игру
pub fn handle_landing(state: &mut GameState) -> Option<super::state::UpdateEndState> {
    use super::state::{
        UpdateEndState, LAND_TIME_DELAY_S, MAX_FALL_SPEED, MIN_Y, PIECE_SCORE_FALL_MULT,
        PIECE_SCORE_INC, SOFT_DROP_POINTS,
    };

    // Проверка проигрыша
    let shape_block_y = state.curr_shape.pos.1 as i16;
    let lost = state.curr_shape.coords.iter().any(|&(_, coord_y)| {
        let block_y = coord_y + shape_block_y;
        block_y < MIN_Y
    });

    if lost {
        return Some(UpdateEndState::Lost);
    }

    // Фиксация фигуры и начисление очков
    let limited_fall_spd = state.fall_spd.min(MAX_FALL_SPEED);
    let fall_bonus = (limited_fall_spd * PIECE_SCORE_FALL_MULT)
        .max(0.0)
        .min(u32::MAX as f32);
    let fall_bonus_u128 = if fall_bonus.is_finite() {
        fall_bonus as u128
    } else {
        0
    };
    state.score = state
        .score
        .saturating_add(PIECE_SCORE_INC.saturating_add(fall_bonus_u128));

    // Начисление очков за Soft Drop
    if state.soft_drop_distance > 0 {
        state.score = state
            .score
            .saturating_add(u128::from(state.soft_drop_distance).saturating_mul(SOFT_DROP_POINTS));
        state.soft_drop_distance = 0;
    }

    // Сброс флага Hard Drop после завершения анимации
    state.is_hard_dropping = false;

    // Сохранение фигуры в сетке поля
    state.save_tetromino();

    // Проверка и удаление заполненных линий
    let lines_cleared = state.check_rows();

    // Обновление комбо
    if lines_cleared > 0 {
        state.stats.combo_counter = state.stats.combo_counter.saturating_add(1);
        if state.stats.combo_counter > 1 {
            state.score = state.score.saturating_add(
                COMBO_BONUS.saturating_mul(u128::from(state.stats.combo_counter - 1)),
            );
        }
    } else {
        state.stats.combo_counter = 0;
    }

    // Сброс таймера и переход к следующей фигуре
    state.land_timer = LAND_TIME_DELAY_S;

    // Переход к следующей фигуре
    state.curr_shape = state.next_shape;
    state.next_shape = crate::tetromino::Tetromino::from_bag(&mut state.bag);
    state.can_hold = true;

    // Обновление статистики для новой фигуры
    state.stats.add_piece(state.curr_shape.shape);

    // Проверка окончания режима спринт
    if state.mode == super::state::GameMode::Sprint
        && state.lines_cleared >= super::state::SPRINT_LINES
    {
        state.stats.stop_timer();
        return Some(UpdateEndState::Won);
    }

    // Проверка окончания режима марафон
    if state.mode == super::state::GameMode::Marathon
        && state.lines_cleared >= super::state::MARATHON_LINES
    {
        state.stats.stop_timer();
        return Some(UpdateEndState::Won);
    }

    None
}

/// Обработать удержание фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn handle_hold(state: &mut GameState) {
    if state.can_hold {
        let current_shape = state.curr_shape;

        if let Some(held) = state.held_shape {
            state.curr_shape = held;
            state.held_shape = Some(current_shape);
        } else {
            state.held_shape = Some(current_shape);
            state.curr_shape = state.next_shape;
            state.next_shape = Tetromino::from_bag(&mut state.bag);
        }

        state.curr_shape.pos = (4.0, 0.0);
        state.can_hold = false;
    }
}

/// Обработать Hard Drop (мгновенное падение).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn handle_hard_drop(state: &mut GameState) {
    use super::state::HARD_DROP_POINTS;
    use crate::types::Direction;

    let start_y = state.curr_shape.pos.1;
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.curr_shape.pos.1 += 1.0;
    }

    // Безопасная конвертация f32 → u32 с использованием clamp() + trunc()
    let drop_distance_f32 = (state.curr_shape.pos.1 - start_y).abs().max(0.0);
    let drop_distance: u32 = if drop_distance_f32.is_finite() {
        // clamp() ограничивает диапазон, trunc() отбрасывает дробную часть
        drop_distance_f32.clamp(0.0, u32::MAX as f32).trunc() as u32
    } else {
        0
    };

    state.score = state
        .score
        .saturating_add(u128::from(drop_distance).saturating_mul(HARD_DROP_POINTS));
    state.land_timer = 0.0;
    state.is_hard_dropping = true;
}

/// Обработать Soft Drop (ускоренное падение).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn handle_soft_drop(state: &mut GameState) {
    use crate::types::Direction;

    if state.can_move_curr_shape_direction(Direction::Down) {
        state.curr_shape.pos.1 += 1.0;
        state.soft_drop_distance = state.soft_drop_distance.saturating_add(1);
        // Начисляем очки за каждую ячейку падения (1 очко за ячейку)
        state.score = state.score.saturating_add(SOFT_DROP_POINTS);
    }
}
