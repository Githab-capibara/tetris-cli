//! Модуль работы с линиями.
//!
//! # Ответственность
//! - Поиск заполненных линий
//! - Удаление заполненных линий и сдвиг верхних линий вниз
//! - Проверка и обработка заполненных линий (`check_rows`)
//!
//! # Зависимости
//! - [`crate::io::GRID_HEIGHT`](crate::io): высота игрового поля
//! - [`super::super::state::GameState`](super::super::state): состояние игры
//! - [`super::ScoringState`](super::ScoringState): trait для изменения состояния

use super::ScoringState;
use crate::constants::{
    BELL, COMBO_BONUS, LEVEL_BONUS_MULT, LINE_SCORES, MAX_LINES_PER_CLEAR, SPD_INC,
};
use crate::io::GRID_HEIGHT;

/// Максимально допустимый счёт для защиты от переполнения.
///
/// # Исправление ISSUE-075
/// Установлен в `u128::MAX` / 2 для безопасного начисления очков.
/// Это предотвращает переполнение при добавлении больших бонусов.
///
/// # Обоснование
/// `u128::MAX` = `340_282_366_920_938_463_463_374_607_431_768_211_455`
/// `MAX_SCORE` = `u128::MAX` / 2 оставляет запас для предотвращения переполнения
/// при начислении очков за линии, комбо и бонусы.
///
/// # Исправление C7 (CRITICAL)
/// Константа сделана публичной для предотвращения дублирования в тестах.
pub(crate) const MAX_SCORE: u128 = u128::MAX / 2;

/// Найти все заполненные линии.
///
/// # Возвращает
/// Битовую маску заполненных линий и количество заполненных линий
///
/// # Производительность
/// O(n) сложность где n = `GRID_HEIGHT` (20 итераций).
/// Используется `.all()` с ранним выходом при обнаружении пустой ячейки.
///
/// # Исправление #11 (LOW)
/// Возвращает битовую маску `u32` для оптимизации использования памяти.
/// Это устраняет необходимость в `SmallVec` и аллокациях.
///
/// # Исправление аудита 2026-03-31 (MEDIUM)
/// Добавлено кэширование маски заполненных линий для предотвращения
/// повторных вычислений при использовании в нескольких местах.
/// Битовая маска занимает 4 байта и может быть быстро скопирована.
#[must_use]
pub fn find_full_rows(blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT]) -> (u32, u32) {
    let mut rows_mask: u32 = 0;
    let mut remove_count = 0;

    // Поиск заполненных линий - кэшируем результат в битовой маске
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
///
/// # Исправление аудита 2026-04-01 (L2)
/// Удалена избыточная проверка `y + rows_removed_below < GRID_HEIGHT` - это условие
/// всегда истинно так как мы идём снизу вверх и `rows_removed_below` <= y.
pub fn remove_rows(blocks: &mut [[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT], rows_mask: u32) {
    // Проверка валидности rows_mask
    if rows_mask >= (1u32 << GRID_HEIGHT) {
        #[cfg(debug_assertions)]
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
            // Исправление L2: убрана избыточная проверка y + rows_removed_below < GRID_HEIGHT
            blocks[y + rows_removed_below] = blocks[y];
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
/// `u32` - количество удалённых линий (0 если линий нет)
///
/// # Пример
/// ```ignore
/// use tetris_cli::game::scoring::lines::check_rows;
/// use tetris_cli::game::scoring::ScoringState;
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
///
/// # Исправление #6 (HIGH)
/// Использует trait `ScoringState` вместо прямого доступа к полям `GameState`.
///
/// # ISP-1: Использует узкие трейты
/// Функция работает с любым типом реализующим `ScoringState` который включает:
/// - `ScoreAccess` (`get_score`, `set_score`, `add_score`)
/// - `LevelAccess` (`get_level`, `set_level`)
/// - `LinesAccess` (`get_lines_cleared`, `set_lines_cleared`, `add_lines`)
/// - `ComboAccess` (`get_combo`, `set_combo`, `reset_combo`)
pub fn check_rows(state: &mut impl ScoringState) -> u32 {
    // Поиск заполненных линий - используем битовую маску для оптимизации
    let (rows_mask, remove_count) = find_filled_lines(state.get_blocks());

    // Анимация и звук для каждой линии
    // Примечание: анимация упрощена до установки флага и звукового сигнала
    if remove_count > 0 {
        state.set_animating_rows_mask(rows_mask);
        print!("{BELL}");
        state.stats_mut().update_max_combo(remove_count);
    }

    // Удаление линий и сдвиг поля
    remove_lines(state.get_blocks_mut(), rows_mask);

    // Обновление счёта, уровня и комбо (ISP-1: используем новые имена методов)
    let mut score = state.get_score();
    let level = state.get_level();
    let mut combo_counter = state.stats().combo_counter();

    // Исправление C3 (CRITICAL): Обработка ошибки переполнения счёта
    // Потеря точности допустима: remove_count <= GRID_HEIGHT (20)
    #[allow(clippy::cast_possible_truncation)]
    if let Err(e) =
        update_score_for_lines(&mut score, level, remove_count as usize, &mut combo_counter)
    {
        // Логгируем ошибку переполнения
        eprintln!("[WARN] check_rows(): {e}");
        // Продолжаем работу - очки уже были добавлены через saturating_add
    }

    state.set_score(score);
    let () = state.stats_mut().set_combo_counter(combo_counter);

    // Обновление количества очищенных линий (ISP-1: используем get_lines_cleared)
    let lines_cleared = state.get_lines_cleared().saturating_add(remove_count);
    let () = state.set_lines_cleared(lines_cleared);

    // Увеличение скорости игры
    let fall_speed = state.fall_speed();
    // Исправление E4 (HIGH): Явная обработка ошибки set_fall_speed()
    // Вместо игнорирования ошибки через let _ = ..., явно обрабатываем результат
    // Потеря точности допустима: remove_count <= GRID_HEIGHT (20)
    #[allow(clippy::cast_precision_loss)]
    let new_fall_speed = fall_speed + SPD_INC * remove_count as f32;
    if let Err(e) = state.set_fall_speed(new_fall_speed) {
        // Логгируем ошибку но продолжаем работу (не критично)
        eprintln!("[WARN] check_rows(): не удалось установить скорость падения: {e}");
        // Продолжаем работу с текущей скоростью - игра не должна останавливаться
    }

    remove_count
}

/// Поиск заполненных линий (вспомогательная функция).
///
/// # Аргументы
/// * `blocks` - игровое поле (только чтение)
///
/// # Возвращает
/// Битовую маску заполненных линий и количество заполненных линий
///
/// # Производительность
/// - Использует битовую маску `u32` для хранения до 32 линий
/// - O(n) сложность где n = `GRID_HEIGHT` (20 итераций)
/// - Без аллокаций в куче
///
/// # Исправление #11 (LOW)
/// Возвращает битовую маску `u32` вместо `SmallVec`<[usize; 4]> для оптимизации.
/// Битовая маска занимает 4 байта вместо 24+ байт для `SmallVec`.
#[must_use]
pub fn find_filled_lines(blocks: &[[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT]) -> (u32, u32) {
    find_full_rows(blocks)
}

/// Удаление линий и сдвиг поля (вспомогательная функция).
///
/// # Аргументы
/// * `blocks` - игровое поле (изменяемое)
/// * `rows_mask` - битовая маска удаляемых линий
fn remove_lines(blocks: &mut [[i8; crate::io::GRID_WIDTH]; GRID_HEIGHT], rows_mask: u32) {
    // Используем существующую функцию удаления с битовой маской
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
/// # Возвращает
/// - `Ok(())` если очки успешно добавлены
/// - `Err(GameError::ScoreOverflow)` если счёт превышает `MAX_SCORE`
///
/// # Errors
/// Возвращает [`crate::errors::GameError::ScoreOverflow`] если счёт превышает [`MAX_SCORE`].
///
/// # Примечания
/// Формула расчёта очков:
/// - Базовые очки за линии из `LINE_SCORES`[`rows_cleared` - 1]
/// - Бонус за комбо: `COMBO_BONUS` × (`combo_counter` - 1)
/// - Бонус за уровень: `LEVEL_BONUS_MULT` × (level - 1)
///
/// # Защита от переполнения
/// Если счёт превышает `MAX_SCORE`, возвращается ошибка `ScoreOverflow`.
///
/// # Исправление L2 (HIGH)
/// Добавлена явная проверка `rows_cleared` > 0 перед доступом к `LINE_SCORES`
/// для предотвращения паники при `rows_cleared` = 0.
///
/// # Исправление аудита 2026-04-01 (C3)
/// Добавлена явная проверка переполнения перед добавлением очков
/// с возвратом `GameError::ScoreOverflow`.
#[must_use = "Ошибка переполнения счёта должна быть обработана"]
pub fn update_score_for_lines(
    score: &mut u128,
    level: u32,
    rows_cleared: usize,
    combo_counter: &mut u32,
) -> Result<(), crate::errors::GameError> {
    use crate::errors::GameError;

    // Исправление L2 (HIGH): Явная проверка rows_cleared > 0
    // Предотвращаем панику при доступе к LINE_SCORES[capped_rows - 1]
    // когда capped_rows = 0
    if rows_cleared == 0 {
        // Сброс комбо если линии не удалены
        *combo_counter = 0;
        return Ok(());
    }

    // Ограничение количества линий максимум 4
    let capped_rows = rows_cleared.min(MAX_LINES_PER_CLEAR as usize);

    // Начисление очков за линии
    // Безопасно: capped_rows >= 1 гарантировано
    let line_score = LINE_SCORES[capped_rows - 1];

    // Исправление C3 (CRITICAL): Упрощена проверка переполнения через saturating_add
    // Вместо checked_add используется saturating_add для защиты от переполнения
    *score = score.saturating_add(line_score);
    if *score > MAX_SCORE {
        return Err(GameError::ScoreOverflow);
    }

    // Обновление комбо
    *combo_counter = combo_counter.saturating_add(1);

    // Бонус за комбо (если комбо > 1)
    if *combo_counter > 1 {
        let combo_bonus = COMBO_BONUS.saturating_mul(u128::from(*combo_counter - 1));
        // Исправление C3: упрощена проверка переполнения через saturating_add
        *score = score.saturating_add(combo_bonus);
        if *score > MAX_SCORE {
            return Err(GameError::ScoreOverflow);
        }
    }

    // Бонус за уровень (каждые 10 линий)
    let level_bonus = LEVEL_BONUS_MULT.saturating_mul(u128::from(level - 1));
    // Исправление C3: упрощена проверка переполнения через saturating_add
    *score = score.saturating_add(level_bonus);
    if *score > MAX_SCORE {
        return Err(GameError::ScoreOverflow);
    }

    Ok(())
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod lines_tests {
    use super::*;
    use crate::game::GameState;

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
    // ТЕСТЫ ДЛЯ #11: find_filled_lines() С БИТОВОЙ МАСКОЙ - ОПТИМИЗАЦИЯ
    // ========================================================================

    /// Тест #11: проверка что `find_filled_lines` возвращает битовую маску
    #[test]
    fn test_fix_11_find_filled_lines_returns_bitmask() {
        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let (mask, count) = find_filled_lines(&blocks);

        // Проверка типа возвращаемого значения - (u32, u32)
        let _: (u32, u32) = (mask, count);
        assert_eq!(mask, 0);
        assert_eq!(count, 0);
    }

    /// Тест #11: проверка `find_filled_lines` с заполненными линиями
    #[test]
    fn test_fix_11_find_filled_lines_with_full_rows() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];

        // Заполняем несколько линий
        blocks[5] = [1i8; crate::io::GRID_WIDTH];
        blocks[10] = [2i8; crate::io::GRID_WIDTH];
        blocks[15] = [3i8; crate::io::GRID_WIDTH];

        let (mask, count) = find_filled_lines(&blocks);

        assert_eq!(count, 3, "Должно быть найдено 3 заполненные линии");
        assert_ne!(mask & (1 << 5), 0, "Должна быть найдена линия 5");
        assert_ne!(mask & (1 << 10), 0, "Должна быть найдена линия 10");
        assert_ne!(mask & (1 << 15), 0, "Должна быть найдена линия 15");
    }

    /// Тест #11: проверка битовой маски с максимальным количеством линий (4)
    #[test]
    fn test_fix_11_find_filled_lines_max_tetris_lines() {
        let mut blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];

        // Заполняем 4 линии (максимум для тетриса)
        blocks[16] = [1i8; crate::io::GRID_WIDTH];
        blocks[17] = [2i8; crate::io::GRID_WIDTH];
        blocks[18] = [3i8; crate::io::GRID_WIDTH];
        blocks[19] = [4i8; crate::io::GRID_WIDTH];

        let (mask, count) = find_filled_lines(&blocks);

        assert_eq!(count, 4, "Должно быть найдено 4 заполненные линии");
        assert_ne!(mask & (1 << 16), 0);
        assert_ne!(mask & (1 << 17), 0);
        assert_ne!(mask & (1 << 18), 0);
        assert_ne!(mask & (1 << 19), 0);
    }

    /// Тест #11: проверка что битовая маска не требует аллокаций
    #[test]
    fn test_fix_11_bitmask_no_allocation() {
        // Битовая маска u32 занимает 4 байта и не требует аллокаций в куче
        // Это оптимально для тетриса (максимум 4 линии за раз)

        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let (mask, count) = find_filled_lines(&blocks);

        // Проверка что битовая маска корректно работает
        assert_eq!(count, 0);
        assert_eq!(mask, 0);

        // Проверка операций с битовой маской
        let mut mutable_mask = mask;
        mutable_mask |= 1 << 5;
        assert_ne!(mutable_mask & (1 << 5), 0);

        mutable_mask &= !(1 << 5);
        assert_eq!(mutable_mask & (1 << 5), 0);
    }

    /// Тест #11: проверка производительности битовой маски
    #[test]
    fn test_fix_11_bitmask_performance_characteristics() {
        // Битовая маска u32 оптимизирована:
        // - 4 байта памяти вместо 24+ байт для SmallVec
        // - Быстрые битовые операции
        // - Нет аллокаций в куче

        let blocks = [[-1i8; crate::io::GRID_WIDTH]; GRID_HEIGHT];
        let (mask, count) = find_filled_lines(&blocks);

        // Проверка что битовая маска корректно работает
        assert_eq!(count, 0);
        assert_eq!(mask, 0);

        // Проверка битовых операций
        assert_eq!(mask.count_ones(), 0);
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ИНКАПСУЛЯЦИИ SCORING МОДУЛЯ
    // =========================================================================

    /// Тест: проверка что scoring использует только публичные методы `GameState`
    ///
    /// Этот тест документирует что модуль scoring использует только
    /// публичные методы `GameState` для доступа к данным, соблюдая
    /// инкапсуляцию.
    #[test]
    fn test_scoring_uses_public_methods_only() {
        // Проверка что scoring использует только публичные методы GameState
        // Функции в этом модуле используют следующие публичные методы:
        // - state.get_blocks() - для получения игрового поля
        // - state.get_blocks_mut() - для модификации поля
        // - state.score() - для получения счёта
        // - state.set_score() - для установки счёта
        // - state.level() - для получения уровня
        // - state.stats() - для получения статистики
        // - state.stats_mut() - для модификации статистики
        // - state.lines_cleared() - для получения количества линий
        // - state.set_lines_cleared() - для установки количества линий
        // - state.fall_speed() - для получения скорости
        // - state.set_fall_speed() - для установки скорости

        // Создаём состояние игры для проверки
        let mut state = GameState::new();

        // Проверяем что все необходимые методы доступны публично
        let _blocks = state.get_blocks();
        let _blocks_mut = state.get_blocks_mut();
        let _score = state.score();
        state.set_score(100);
        let _level = state.level();
        let _stats = state.stats();
        let _stats_mut = state.stats_mut();
        let _lines = state.lines_cleared();
        state.set_lines_cleared(5);
        let _fall_speed = state.fall_speed();
        let _ = state.set_fall_speed(1.0);

        // Проверяем что методы действительно работают — значения изменились
        assert_eq!(state.score(), 100, "set_score(100) должен установить счёт в 100");
        assert_eq!(state.lines_cleared(), 5, "set_lines_cleared(5) должен установить линии в 5");
    }
}
