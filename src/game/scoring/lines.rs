//! Модуль работы с линиями.
//!
//! # Ответственность
//! - Поиск заполненных линий
//! - Удаление заполненных линий и сдвиг верхних линий вниз
//! - Проверка и обработка заполненных линий (check_rows)
//!
//! # Зависимости
//! - [`crate::io::GRID_HEIGHT`](crate::io): высота игрового поля
//! - [`super::super::state::GameState`](super::super::state): состояние игры

use super::super::constants::{
    BELL, COMBO_BONUS, LEVEL_BONUS_MULT, LINE_SCORES, MAX_LINES_PER_CLEAR, SPD_INC,
};
use super::super::state::GameState;
use crate::io::GRID_HEIGHT;

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

#[cfg(test)]
mod lines_tests {
    use super::*;

    #[test]
    fn test_find_full_rows_empty() {
        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let (mask, count) = find_full_rows(&blocks);
        assert_eq!(mask, 0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_remove_rows_single() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        // Заполняем последнюю строку
        blocks[GRID_HEIGHT - 1] = [1i8; crate::io::GRID_WIDTH];

        remove_rows(&mut blocks, 1u32 << (GRID_HEIGHT - 1));

        // Последняя строка должна быть пустой
        assert!(blocks[GRID_HEIGHT - 1].iter().all(|&c| c == -1));
    }
}

// ============================================================================
// ПУБЛИЧНЫЕ ФУНКЦИИ ДЛЯ ОБРАБОТКИ ЛИНИЙ
// ============================================================================

/// Проверить заполненные линии и удалить их.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Возвращает
/// Количество удалённых линий
///
/// # Пример
/// ```ignore
/// use tetris_cli::game::scoring::lines::check_rows;
///
/// let mut state = GameState::new();
/// let cleared = check_rows(&mut state);
/// ```
///
/// # Архитектурные заметки
/// Эта функция была перемещена из `render.rs` в `scoring::lines` для:
/// - Улучшения разделения ответственности (отрисовка vs логика игры)
/// - Уменьшения связанности между модулями
/// - Улучшения тестируемости логики удаления линий
pub fn check_rows(state: &mut GameState) -> u32 {
    // Поиск заполненных линий
    let filled_rows = find_filled_lines(state.get_blocks());
    let remove_count = filled_rows.len() as u32;

    // Преобразуем в битовую маску для анимации
    let mut rows_mask: u32 = 0;
    for &row in &filled_rows {
        rows_mask |= 1u32 << row;
    }

    // Анимация и звук для каждой линии
    // Примечание: анимация упрощена до установки флага и звукового сигнала
    if remove_count > 0 {
        state.set_animating_rows_mask(rows_mask);
        print!("{BELL}");
        state.get_stats_mut().update_max_combo(remove_count);
    }

    // Удаление линий и сдвиг поля
    remove_lines(state.get_blocks_mut(), &filled_rows);

    // Обновление счёта, уровня и комбо
    let mut score = state.score();
    let level = state.level();
    let mut combo_counter = state.get_stats().combo_counter();

    update_score_for_lines(&mut score, level, filled_rows.len(), &mut combo_counter);

    state.set_score(score);
    state.get_stats_mut().set_combo_counter(combo_counter);

    // Обновление количества очищенных линий
    let lines_cleared = state.lines_cleared().saturating_add(remove_count);
    state.set_lines_cleared(lines_cleared);

    // Увеличение скорости игры
    let fall_speed = state.fall_speed();
    state.set_fall_speed(fall_speed + SPD_INC * remove_count as f32);

    remove_count
}

/// Поиск заполненных линий (вспомогательная функция).
///
/// # Аргументы
/// * `blocks` - игровое поле (только чтение)
///
/// # Возвращает
/// Вектор с индексами заполненных линий
///
/// # Архитектурные заметки
/// Эта функция была перемещена из `render.rs` в `scoring::lines`.
/// Сделана публичной для использования в тестах.
#[must_use]
pub fn find_filled_lines(blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT]) -> Vec<usize> {
    let mut filled = Vec::new();

    for (y, row) in blocks.iter().enumerate() {
        // Проверка: линия заполнена если все ячейки не пустые (!= -1)
        let row_full = row.iter().all(|&cell| cell != -1);
        if row_full {
            filled.push(y);
        }
    }

    filled
}

/// Удаление линий и сдвиг поля (вспомогательная функция).
///
/// # Аргументы
/// * `blocks` - игровое поле (изменяемое)
/// * `rows` - срез с индексами удаляемых линий
fn remove_lines(blocks: &mut [[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT], rows: &[usize]) {
    // Преобразуем список строк в битовую маску для совместимости с remove_rows()
    let mut rows_mask: u32 = 0;
    for &row in rows {
        rows_mask |= 1u32 << row;
    }

    // Используем существующую функцию удаления
    remove_rows(blocks, rows_mask);
}

/// Обновление счёта за удалённые линии (вспомогательная функция).
///
/// # Аргументы
/// * `score` - счёт (изменяемый)
/// * `level` - текущий уровень
/// * `rows_cleared` - количество удалённых линий
/// * `combo_counter` - счётчик комбо (изменяемый)
///
/// # Примечания
/// Формула расчёта очков:
/// - Базовые очки за линии из LINE_SCORES[rows_cleared - 1]
/// - Бонус за комбо: COMBO_BONUS × (combo_counter - 1)
/// - Бонус за уровень: LEVEL_BONUS_MULT × (level - 1)
fn update_score_for_lines(
    score: &mut u128,
    level: u32,
    rows_cleared: usize,
    combo_counter: &mut u32,
) {
    if rows_cleared > 0 {
        // Ограничение количества линий максимум 4
        let capped_rows = rows_cleared.min(MAX_LINES_PER_CLEAR as usize);

        // Начисление очков за линии
        let line_score = LINE_SCORES[capped_rows - 1];
        *score = score.saturating_add(line_score);

        // Обновление комбо
        *combo_counter = combo_counter.saturating_add(1);

        // Бонус за комбо (если комбо > 1)
        if *combo_counter > 1 {
            let combo_bonus = COMBO_BONUS.saturating_mul(u128::from(*combo_counter - 1));
            *score = score.saturating_add(combo_bonus);
        }

        // Бонус за уровень (каждые 10 линий)
        let level_bonus = LEVEL_BONUS_MULT.saturating_mul(u128::from(level - 1));
        *score = score.saturating_add(level_bonus);
    } else {
        // Сброс комбо если линии не удалены
        *combo_counter = 0;
    }
}
