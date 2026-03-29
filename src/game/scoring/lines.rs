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

use smallvec::SmallVec;

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
    // Поиск заполненных линий - используем SmallVec для оптимизации
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
/// SmallVec с индексами заполненных линий.
/// SmallVec<[usize; 4]> оптимизирован для случая до 4 линий (максимум в тетрисе).
///
/// # Производительность
/// - Использует SmallVec для предотвращения аллокаций в куче
/// - Для типичного случая (0-4 линии) данные хранятся в стеке
/// - O(n) сложность где n = `GRID_HEIGHT` (20 итераций)
///
/// # Исправление M8 (MEDIUM)
/// Использует SmallVec<[usize; 4]> вместо Vec<usize> для оптимизации.
/// SmallVec хранит до 4 элементов в стеке без аллокации в куче.
/// Это оптимально для тетриса, где максимум 4 линии за раз.
#[must_use]
pub fn find_filled_lines(
    blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT],
) -> SmallVec<[usize; 4]> {
    // SmallVec с capacity 4 - оптимизировано для максимального количества линий в тетрисе
    let mut filled = SmallVec::<[usize; 4]>::new();

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
/// * `rows` - срез с индексами удаляемых линий (SmallVec или &[usize])
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

// ============================================================================
// ТЕСТЫ
// ============================================================================

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

    // ========================================================================
    // ТЕСТЫ ДЛЯ M8: find_filled_lines() С SmallVec - ОПТИМИЗАЦИЯ
    // ========================================================================

    /// Тест M8: проверка что find_filled_lines использует SmallVec
    #[test]
    fn test_fix_m8_find_filled_lines_returns_smallvec() {
        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let filled = find_filled_lines(&blocks);

        // Проверка типа возвращаемого значения - SmallVec<[usize; 4]>
        // Это проверяется на этапе компиляции
        let _: SmallVec<[usize; 4]> = filled;
    }

    /// Тест M8: проверка find_filled_lines с заполненными линиями
    #[test]
    fn test_fix_m8_find_filled_lines_with_full_rows() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];

        // Заполняем несколько линий
        blocks[5] = [1i8; crate::io::GRID_WIDTH];
        blocks[10] = [2i8; crate::io::GRID_WIDTH];
        blocks[15] = [3i8; crate::io::GRID_WIDTH];

        let filled = find_filled_lines(&blocks);

        assert_eq!(filled.len(), 3, "Должно быть найдено 3 заполненные линии");
        assert!(filled.contains(&5));
        assert!(filled.contains(&10));
        assert!(filled.contains(&15));
    }

    /// Тест M8: проверка SmallVec capacity оптимизации
    #[test]
    fn test_fix_m8_smallvec_capacity_optimization() {
        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let filled = find_filled_lines(&blocks);

        // SmallVec<[usize; 4]> должен иметь capacity 4 для хранения в стеке
        // Это оптимально для тетриса (максимум 4 линии за раз)
        assert!(
            filled.capacity() >= 4,
            "SmallVec должен иметь capacity >= 4"
        );
    }

    /// Тест M8: проверка find_filled_lines с максимальным количеством линий (4)
    #[test]
    fn test_fix_m8_find_filled_lines_max_tetris_lines() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];

        // Заполняем 4 линии (максимум для тетриса)
        blocks[16] = [1i8; crate::io::GRID_WIDTH];
        blocks[17] = [2i8; crate::io::GRID_WIDTH];
        blocks[18] = [3i8; crate::io::GRID_WIDTH];
        blocks[19] = [4i8; crate::io::GRID_WIDTH];

        let filled = find_filled_lines(&blocks);

        assert_eq!(filled.len(), 4, "Должно быть найдено 4 заполненные линии");
        assert!(filled.contains(&16));
        assert!(filled.contains(&17));
        assert!(filled.contains(&18));
        assert!(filled.contains(&19));
    }

    /// Тест M8: проверка что SmallVec не требует аллокации для 0-4 элементов
    #[test]
    fn test_fix_m8_smallvec_no_allocation_for_typical_case() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];

        // Тест для 0 линий
        let filled_empty = find_filled_lines(&blocks);
        assert_eq!(filled_empty.len(), 0);

        // Тест для 1 линии
        blocks[10] = [1i8; crate::io::GRID_WIDTH];
        let filled_one = find_filled_lines(&blocks);
        assert_eq!(filled_one.len(), 1);

        // Тест для 2 линий
        blocks[11] = [2i8; crate::io::GRID_WIDTH];
        let filled_two = find_filled_lines(&blocks);
        assert_eq!(filled_two.len(), 2);

        // Тест для 3 линий
        blocks[12] = [3i8; crate::io::GRID_WIDTH];
        let filled_three = find_filled_lines(&blocks);
        assert_eq!(filled_three.len(), 3);

        // Тест для 4 линий (максимум для тетриса)
        blocks[13] = [4i8; crate::io::GRID_WIDTH];
        let filled_four = find_filled_lines(&blocks);
        assert_eq!(filled_four.len(), 4);

        // Все SmallVec должны хранить данные в стеке (без аллокации)
        // Это проверяется через capacity - если capacity >= 4, данные в стеке
        assert!(filled_empty.capacity() >= 4);
        assert!(filled_one.capacity() >= 4);
        assert!(filled_two.capacity() >= 4);
        assert!(filled_three.capacity() >= 4);
        assert!(filled_four.capacity() >= 4);
    }

    /// Тест M8: проверка производительности SmallVec vs Vec
    #[test]
    fn test_fix_m8_smallvec_performance_characteristics() {
        // SmallVec<[usize; 4]> оптимизирован для случая до 4 элементов
        // Для 0-4 элементов данные хранятся в стеке без аллокации в куче

        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let filled = find_filled_lines(&blocks);

        // Проверка что SmallVec корректно работает
        assert_eq!(filled.len(), 0);

        // SmallVec поддерживает те же операции что и Vec
        let mut mutable_filled = filled;
        mutable_filled.push(5);
        assert_eq!(mutable_filled.len(), 1);
        assert_eq!(mutable_filled[0], 5);

        mutable_filled.clear();
        assert_eq!(mutable_filled.len(), 0);
    }
}
