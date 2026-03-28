//! Модуль системы очков.
//!
//! # Ответственность
//! - Начисление очков за фигуры и линии
//! - Повышение уровня
//! - Расчёт очков за падение (Soft Drop, Hard Drop)
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): константы очков, `GameState`
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`
//! - [`lines.rs`](super::lines): удаление линий

use crate::game::constants::{
    LEVEL_BONUS_MULT, LINES_PER_LEVEL, LINE_SCORES, MAX_LINES_PER_CLEAR, SPD_INC,
};
use crate::game::state::{GameState, UpdateEndState};
use crate::tetromino::Tetromino;

/// Обновить счёт, уровень и скорость после удаления линий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `remove_count` - количество удалённых линий
///
/// # Инкапсуляция (Задача HIGH)
/// Использует методы GameState вместо прямого доступа к полям.
#[allow(dead_code)]
pub fn update_score_and_level(state: &mut GameState, remove_count: u32) {
    if remove_count > 0 {
        // Ограничение remove_count максимум 4
        let capped_remove_count = remove_count.min(MAX_LINES_PER_CLEAR);

        // Обновление количества удалённых линий через метод (Инкапсуляция)
        state.add_lines_cleared(capped_remove_count);

        // Проверка повышения уровня (каждые 10 линий)
        let new_level = (state.get_lines_cleared() / LINES_PER_LEVEL) + 1;
        if new_level > state.get_level() {
            state.set_level(new_level);
            // Бонус за повышение уровня
            state.add_score(LEVEL_BONUS_MULT.saturating_mul(u128::from(new_level - 1)));
        }

        // Увеличение скорости игры
        state.set_fall_spd(state.get_fall_spd() + SPD_INC * capped_remove_count as f32);

        // Начисление очков за линии (lookup таблица)
        // Исправление C1: защита от переполнения при сложении очков
        if capped_remove_count > 0 {
            let line_score = LINE_SCORES[(capped_remove_count - 1) as usize];
            state.add_score(line_score);
        }
    }
}

/// Безопасно конвертировать f32 в u32 с защитой от переполнения.
///
/// # Аргументы
/// * `value` - значение для конвертации
///
/// # Возвращает
/// - `u32` если значение в допустимом диапазоне
/// - `0` если значение NaN, отрицательное или бесконечное
/// - `u32::MAX` если значение превышает максимальное
///
/// # Исправление #1 (CRITICAL)
/// Использует clamp + cast для безопасной конвертации без паники.
/// Защита от NaN, Infinity, отрицательных значений и переполнения.
#[inline]
fn safe_f32_to_u32(value: f32) -> u32 {
    if !value.is_finite() {
        return 0;
    }
    // clamp гарантирует что значение в диапазоне [0, u32::MAX]
    // cast безопасен после clamp
    value.clamp(0.0, u32::MAX as f32) as u32
}

/// Обработать Hard Drop (мгновенное падение).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Инкапсуляция (Задача HIGH)
/// Использует методы GameState вместо прямого доступа к полям.
///
/// # Исправление C1
/// Использует saturating_mul для защиты от переполнения при начислении очков за падение.
pub fn handle_hard_drop(state: &mut GameState) {
    use crate::game::constants::HARD_DROP_POINTS;
    use crate::types::Direction;

    let start_y = state.get_curr_shape().pos.1;
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.curr_shape.pos.1 += 1.0;
    }

    // Безопасная конвертация f32 → u32 с использованием clamp + cast
    // Исправление #1 (CRITICAL): защита от NaN, Infinity и переполнения
    let drop_distance_f32 = (state.curr_shape.pos.1 - start_y).abs();
    let drop_distance = safe_f32_to_u32(drop_distance_f32);

    // Инкапсуляция: используем add_score() вместо прямого доступа
    // Исправление C1: saturating_mul для защиты от переполнения
    state.add_score(u128::from(drop_distance).saturating_mul(HARD_DROP_POINTS));
    state.land_timer = 0.0;
    state.is_hard_dropping = true;
}

/// Обработать Soft Drop (ускоренное падение).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Инкапсуляция (Задача HIGH)
/// Использует методы GameState вместо прямого доступа к полям.
///
/// # Исправление C1
/// Использует saturating_mul для защиты от переполнения при начислении очков за падение.
pub fn handle_soft_drop(state: &mut GameState) {
    use crate::game::constants::SOFT_DROP_POINTS;
    use crate::types::Direction;

    if state.can_move_curr_shape_direction(Direction::Down) {
        state.curr_shape.pos.1 += 1.0;
        state.soft_drop_distance = state.soft_drop_distance.saturating_add(1);
        // Инкапсуляция: используем add_score() вместо прямого доступа
        // Исправление C1: saturating_mul для защиты от переполнения
        state.add_score(SOFT_DROP_POINTS);
    }
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

/// Обработать приземление фигуры и начислить очки.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Возвращает
/// - `Some(UpdateEndState::Lost)` - проигрыш
/// - `Some(UpdateEndState::Won)` - победа (завершение режима)
/// - `None` - продолжить игру
///
/// # Исправление #24
/// Функция разделена на подфункции для улучшения читаемости:
/// - `check_game_over_condition()` - проверка проигрыша
/// - `calculate_landing_bonus()` - расчёт бонуса за приземление
/// - `spawn_next_tetromino()` - переход к следующей фигуре
/// - `check_mode_completion()` - проверка окончания режима
pub fn handle_landing(state: &mut GameState) -> Option<UpdateEndState> {
    use crate::game::constants::{MARATHON_LINES, SPRINT_LINES};

    // Проверка проигрыша (Исправление #24: вынесено в подфункцию)
    if check_game_over_condition(state) {
        return Some(UpdateEndState::Lost);
    }

    // Начисление очков за приземление (Исправление #24: вынесено в подфункцию)
    calculate_landing_bonus(state);

    // Сохранение фигуры в сетке поля
    state.save_tetromino();

    // Проверка и удаление заполненных линий
    let lines_cleared = state.check_rows();

    // Обновление комбо
    update_combo_on_clear(state, lines_cleared);

    // Переход к следующей фигуре (Исправление #24: вынесено в подфункцию)
    spawn_next_tetromino(state);

    // Проверка окончания режима (Исправление #24: вынесено в подфункцию)
    check_mode_completion(state, lines_cleared, SPRINT_LINES, MARATHON_LINES)
}

/// Проверить условие проигрыша.
///
/// # Аргументы
/// * `state` - состояние игры
///
/// # Возвращает
/// `true` если фигура достигла верха поля (проигрыш)
///
/// # Исправление #24
/// Выделена из `handle_landing()` для улучшения читаемости.
fn check_game_over_condition(state: &GameState) -> bool {
    use crate::game::constants::MIN_Y;

    let shape_block_y = state.curr_shape.pos.1 as i16;
    state.curr_shape.coords.iter().any(|&(_, coord_y)| {
        let block_y = coord_y + shape_block_y;
        block_y < MIN_Y
    })
}

/// Рассчитать и начислить бонус за приземление фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Исправление #24
/// Выделена из `handle_landing()` для улучшения читаемости.
///
/// # Инкапсуляция (Задача HIGH)
/// Использует методы GameState вместо прямого доступа к полям.
///
/// # Исправление C1
/// Использует saturating_add и saturating_mul для защиты от переполнения.
fn calculate_landing_bonus(state: &mut GameState) {
    use crate::game::constants::{
        LAND_TIME_DELAY_S, MAX_FALL_SPEED, PIECE_SCORE_FALL_MULT, PIECE_SCORE_INC, SOFT_DROP_POINTS,
    };

    // Расчёт бонуса за скорость падения
    let limited_fall_spd = state.get_fall_spd().min(MAX_FALL_SPEED);
    let fall_bonus = (limited_fall_spd * PIECE_SCORE_FALL_MULT)
        .max(0.0)
        .min(u32::MAX as f32);
    let fall_bonus_u128 = if fall_bonus.is_finite() {
        fall_bonus as u128
    } else {
        0
    };
    // Инкапсуляция: используем add_score() вместо прямого доступа
    // Исправление C1: saturating_add для защиты от переполнения
    state.add_score(PIECE_SCORE_INC.saturating_add(fall_bonus_u128));

    // Начисление очков за Soft Drop
    // Исправление C1: saturating_mul для защиты от переполнения
    if state.soft_drop_distance > 0 {
        state.add_score(u128::from(state.soft_drop_distance).saturating_mul(SOFT_DROP_POINTS));
        state.soft_drop_distance = 0;
    }

    // Сброс флага Hard Drop после завершения анимации
    state.is_hard_dropping = false;

    // Сброс таймера приземления
    state.land_timer = LAND_TIME_DELAY_S;
}

/// Обновить счётчик комбо после удаления линий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `lines_cleared` - количество удалённых линий
///
/// # Исправление #24
/// Выделена из `handle_landing()` для улучшения читаемости.
///
/// # Инкапсуляция (Задача HIGH)
/// Использует методы GameState вместо прямого доступа к полям.
///
/// # Исправление C1
/// Использует saturating_mul для защиты от переполнения при начислении комбо-бонуса.
fn update_combo_on_clear(state: &mut GameState, lines_cleared: u32) {
    use crate::game::constants::COMBO_BONUS;

    if lines_cleared > 0 {
        state.stats.combo_counter = state.stats.combo_counter.saturating_add(1);
        if state.stats.combo_counter > 1 {
            // Инкапсуляция: используем add_score() вместо прямого доступа
            // Исправление C1: saturating_mul для защиты от переполнения
            state.add_score(COMBO_BONUS.saturating_mul(u128::from(state.stats.combo_counter - 1)));
        }
    } else {
        state.stats.combo_counter = 0;
    }
}

/// Создать следующую фигуру и обновить статистику.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Исправление #24
/// Выделена из `handle_landing()` для улучшения читаемости.
fn spawn_next_tetromino(state: &mut GameState) {
    state.curr_shape = state.next_shape;
    state.next_shape = crate::tetromino::Tetromino::from_bag(&mut state.bag);
    state.can_hold = true;

    // Обновление статистики для новой фигуры
    state.stats.add_piece(state.curr_shape.shape);
}

/// Проверить условие окончания режима (спринт/марафон).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `lines_cleared` - количество удалённых линий
/// * `sprint_lines` - целевое количество линий для спринта
/// * `marathon_lines` - целевое количество линий для марафона
///
/// # Возвращает
/// - `Some(UpdateEndState::Won)` - режим завершён
/// - `None` - продолжить игру
///
/// # Исправление #24
/// Выделена из `handle_landing()` для улучшения читаемости.
fn check_mode_completion(
    state: &mut GameState,
    lines_cleared: u32,
    sprint_lines: u32,
    marathon_lines: u32,
) -> Option<UpdateEndState> {
    let mode_trait = state.get_mode_trait();

    if mode_trait.get_target_lines() == Some(40) && lines_cleared >= sprint_lines {
        state.stats.stop_timer();
        return Some(UpdateEndState::Won);
    }

    if mode_trait.get_target_lines() == Some(150) && lines_cleared >= marathon_lines {
        state.stats.stop_timer();
        return Some(UpdateEndState::Won);
    }

    None
}

#[cfg(test)]
mod points_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_update_score_and_level_basic() {
        let mut state = GameState::new();
        let initial_score = state.get_score();

        update_score_and_level(&mut state, 1);

        assert!(state.get_score() > initial_score, "Счёт должен увеличиться");
        assert_eq!(state.get_lines_cleared(), 1, "Должна быть очищена 1 линия");
    }

    #[test]
    fn test_handle_hold_basic() {
        let mut state = GameState::new();
        let initial_shape = *state.get_curr_shape();

        state.hold_shape();

        assert!(
            state.get_held_shape().is_some(),
            "Фигура должна быть удержана"
        );
        assert_ne!(
            state.get_curr_shape().shape,
            initial_shape.shape,
            "Текущая фигура должна измениться"
        );
        assert!(
            !state.can_hold(),
            "can_hold должен быть false после удержания"
        );
    }
}
