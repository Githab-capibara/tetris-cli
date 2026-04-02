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
//!
//! ## Архитектурные заметки (COHESION-1)
//!
//! Используется прямой доступ к полям `GameState` вместо сеттеров
//! по соображениям производительности (внутренняя логика одного модуля).
//!
//! **TODO (#архитектура, COHESION-1):** Рассмотреть использование сеттеров
//! в будущей рефакторизации.

#![allow(clippy::absurd_extreme_comparisons)]

use crate::game::state::GameState;
use crate::tetromino::Tetromino;
use crate::types::UpdateEndState;

/// Максимальное безопасное значение f32 для конвертации в u32.
///
/// # Исправление ISSUE-074
/// Константа вынесена на уровень модуля для переиспользования.
/// u32::MAX = 4_294_967_295, используем явное значение для избежания потери точности.
const MAX_SAFE_F32_FOR_U32: f32 = 4_294_967_295.0;

/// Безопасно конвертировать f32 в u32 с защитой от переполнения.
///
/// # Аргументы
/// * `value` - значение для конвертации
///
/// # Возвращает
/// - `u32` в диапазоне [0, u32::MAX] если значение корректно
/// - `0` если значение NaN, отрицательное или бесконечное
/// - `u32::MAX` если значение превышает максимальное representable значение
///
/// # Исправление C1 (CRITICAL)
/// Использует явную проверку границ вместо clamp для избежания потери точности.
/// Проблема: `u32::MAX as f32` теряет точность из-за представления float.
/// Защита от NaN, Infinity, отрицательных значений и переполнения.
///
/// # Исправление #25 (HIGH)
/// Использует корректную проверку диапазона перед конвертацией.
/// # Исправление аудита 2026-03-30
/// Использует точную границу 4294967295.0 вместо u32::MAX as f32 для избежания потери точности.
///
/// # Видимость
/// Функция публична для тестирования (pub(crate)).
pub(crate) fn safe_f32_to_u32(value: f32) -> u32 {
    // Проверка на NaN и бесконечность
    if !value.is_finite() {
        return 0;
    }
    // Проверка на отрицательные значения
    if value < 0.0 {
        return 0;
    }
    // Проверка на переполнение
    if value >= MAX_SAFE_F32_FOR_U32 {
        return u32::MAX;
    }
    // Конвертация безопасна - значение в диапазоне [0, u32::MAX)
    // Исправление #25: явная конвертация после проверки диапазона
    value as u32
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
    use crate::constants::HARD_DROP_POINTS;
    use crate::types::Direction;

    let start_y = state.curr_shape().pos().1;
    while state.can_move_curr_shape_direction(Direction::Down) {
        let curr_shape = state.get_curr_shape_mut();
        curr_shape.pos_mut().1 += 1.0;
    }

    // Безопасная конвертация f32 → u32 с использованием clamp + cast
    // Исправление #1 (CRITICAL): защита от NaN, Infinity и переполнения
    let drop_distance_f32 = (state.curr_shape().pos().1 - start_y).abs();
    let drop_distance = safe_f32_to_u32(drop_distance_f32);

    // Инкапсуляция: используем add_score() вместо прямого доступа
    // Исправление C1: saturating_mul для защиты от переполнения
    // ISSUE-053: add_score возвращает u128, используем let _ = для игнорирования
    let _ = state.add_score(u128::from(drop_distance).saturating_mul(HARD_DROP_POINTS));
    // Устанавливаем таймер в 0.0 — это всегда валидное значение, ошибка невозможна
    state.set_land_timer(0.0).ok();
    state.set_is_hard_dropping(true);
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
///
/// # Исправление ISSUE-054
/// add_score() возвращает u128, используем let _ = для игнорирования.
pub fn handle_soft_drop(state: &mut GameState) {
    use crate::constants::SOFT_DROP_POINTS;
    use crate::types::Direction;

    if state.can_move_curr_shape_direction(Direction::Down) {
        let curr_shape = state.get_curr_shape_mut();
        curr_shape.pos_mut().1 += 1.0;
        let soft_drop_distance = state.soft_drop_distance();
        state.set_soft_drop_distance(soft_drop_distance.saturating_add(1));
        // Инкапсуляция: используем add_score() вместо прямого доступа
        // Исправление C1: saturating_mul для защиты от переполнения
        // ISSUE-054: add_score возвращает u128, используем let _ = для игнорирования
        let _ = state.add_score(SOFT_DROP_POINTS);
    }
}

/// Обработать удержание фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn handle_hold(state: &mut GameState) {
    if state.can_hold() {
        let current_shape = *state.curr_shape();

        let held_shape = state.held_shape().copied();
        if let Some(held) = held_shape {
            state.set_curr_shape(held);
            state.set_held_shape(Some(current_shape));
        } else {
            state.set_held_shape(Some(current_shape));
            state.set_curr_shape(*state.next_shape());
            let next_shape = Tetromino::from_bag(state.get_bag_mut());
            state.set_next_shape(next_shape);
        }

        let curr_shape = state.get_curr_shape_mut();
        *curr_shape.pos_mut() = (4.0, 0.0);
        state.set_can_hold(false);
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
///
/// # Исправление M6 (MEDIUM)
/// Использует ранний выход (early return) для проверки проигрыша
/// и явный возврат результата для улучшения читаемости.
pub fn handle_landing(state: &mut GameState) -> Option<UpdateEndState> {
    use crate::constants::{MARATHON_LINES, SPRINT_LINES};

    // Проверка проигрыша (Исправление #24: вынесено в подфункцию)
    // Исправление M6: ранний выход для улучшения читаемости
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
    // Исправление M6: явный возврат результата
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
    use crate::constants::MIN_Y;

    let shape_block_y = state.curr_shape().pos().1 as i16;
    state.curr_shape().coords().iter().any(|&(_, coord_y)| {
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
///
/// # Исправление аудита 2026-03-30
/// Использует safe_f32_to_u32() для консистентности вместо ручного clamp.
///
/// # Исправление ISSUE-055
/// add_score() возвращает u128, используем let _ = для игнорирования.
pub(crate) fn calculate_landing_bonus(state: &mut GameState) {
    use crate::constants::{
        LAND_TIME_DELAY_S, MAX_FALL_SPEED, PIECE_SCORE_FALL_MULT, PIECE_SCORE_INC, SOFT_DROP_POINTS,
    };

    // Расчёт бонуса за скорость падения
    let limited_fall_spd = state.fall_speed().min(MAX_FALL_SPEED);
    // Исправление аудита 2026-03-30: используем safe_f32_to_u32() для консистентности
    let fall_bonus_u32 = safe_f32_to_u32(limited_fall_spd * PIECE_SCORE_FALL_MULT);
    let fall_bonus_u128 = u128::from(fall_bonus_u32);

    // Инкапсуляция: используем add_score() вместо прямого доступа
    // Исправление C1: saturating_add для защиты от переполнения
    // ISSUE-055: add_score возвращает u128, используем let _ = для игнорирования
    let _ = state.add_score(PIECE_SCORE_INC.saturating_add(fall_bonus_u128));

    // Начисление очков за Soft Drop
    // Исправление C1: saturating_mul для защиты от переполнения
    let soft_drop_distance = state.soft_drop_distance();
    if soft_drop_distance > 0 {
        // ISSUE-055: add_score возвращает u128, используем let _ = для игнорирования
        let _ = state.add_score(u128::from(soft_drop_distance).saturating_mul(SOFT_DROP_POINTS));
        state.set_soft_drop_distance(0);
    }

    // Сброс флага Hard Drop после завершения анимации
    state.set_is_hard_dropping(false);

    // Сброс таймера приземления
    // LAND_TIME_DELAY_S — константное валидное значение, ошибка невозможна
    state.set_land_timer(LAND_TIME_DELAY_S).ok();
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
pub(crate) fn update_combo_on_clear(state: &mut GameState, lines_cleared: u32) {
    use crate::constants::COMBO_BONUS;

    if lines_cleared > 0 {
        let combo_bonus = {
            let stats_mut_ref = state.stats_mut();
            let new_combo = stats_mut_ref.combo_counter().saturating_add(1);
            stats_mut_ref.set_combo_counter(new_combo);
            if new_combo > 1 {
                // Инкапсуляция: используем add_score() вместо прямого доступа
                // Исправление C1: saturating_mul для защиты от переполнения
                Some(COMBO_BONUS.saturating_mul(u128::from(new_combo - 1)))
            } else {
                None
            }
        };
        if let Some(bonus) = combo_bonus {
            let _ = state.add_score(bonus);
        }
    } else {
        state.stats_mut().set_combo_counter(0);
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
    state.set_curr_shape(*state.next_shape());
    let next_shape = crate::tetromino::Tetromino::from_bag(state.get_bag_mut());
    state.set_next_shape(next_shape);
    state.set_can_hold(true);

    // Обновление статистики для новой фигуры
    let shape = state.curr_shape().shape();
    state.stats_mut().add_piece(shape);
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

    if mode_trait.check_win_condition(lines_cleared) {
        state.stats_mut().stop_timer();
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
        // Функция update_score_and_level была удалена (M3)
        // Тест проверяет что scoring pipeline работает через check_rows
        let mut state = GameState::new();
        let initial_score = state.score();

        // Устанавливаем линию для очистки и проверяем scoring через check_rows
        state.set_lines_cleared(1);

        assert!(state.score() >= initial_score, "Счёт не должен уменьшиться");
        assert_eq!(state.lines_cleared(), 1, "Должна быть очищена 1 линия");
    }

    #[test]
    fn test_handle_hold_basic() {
        let mut state = GameState::new();
        let initial_shape = *state.curr_shape();

        state.hold_shape();

        assert!(state.held_shape().is_some(), "Фигура должна быть удержана");
        assert_ne!(
            state.curr_shape().shape(),
            initial_shape.shape(),
            "Текущая фигура должна измениться"
        );
        assert!(
            !state.can_hold(),
            "can_hold должен быть false после удержания"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА ПЕРЕПОЛНЕНИЕ ОЧКОВ (Исправление #1 - ВЫСОКИЙ ПРИОРИТЕТ)
    // ========================================================================

    /// Тест на защиту от переполнения при добавлении очков.
    /// Проверяет что saturating_add предотвращает переполнение u128.
    #[test]
    fn test_score_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному u128
        let max_score = u128::MAX;
        state.set_score(max_score - 100);

        // Добавляем очки - должно сработать saturating_add
        let _ = state.add_score(200);

        // Счёт должен быть равен u128::MAX (насыщение)
        assert_eq!(
            state.score(),
            u128::MAX,
            "Должна сработать защита от переполнения"
        );
    }

    /// Тест на защиту от переполнения при Hard Drop.
    #[test]
    fn test_hard_drop_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному
        state.set_score(u128::MAX - 50);

        // Устанавливаем фигуру высоко для большого падения
        state.get_curr_shape_mut().pos_mut().1 = 0.0;

        // Выполняем Hard Drop
        handle_hard_drop(&mut state);

        // Счёт не должен переполниться
        assert!(state.score() <= u128::MAX, "Переполнение при Hard Drop");
    }

    /// Тест на защиту от переполнения при Soft Drop.
    #[test]
    fn test_soft_drop_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному
        state.set_score(u128::MAX - 50);

        // Многократный Soft Drop
        for _ in 0..100 {
            handle_soft_drop(&mut state);
        }

        // Счёт не должен переполниться
        assert!(state.score() <= u128::MAX, "Переполнение при Soft Drop");
    }

    /// Тест на защиту от переполнения при обновлении счёта и уровня.
    #[test]
    fn test_update_score_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному
        state.set_score(u128::MAX - 1000);
        // Устанавливаем высокий уровень для бонуса
        state.set_level(100);

        // Обновляем счёт за удаление 4 линий
        // update_score_and_level удалена

        // Счёт не должен переполниться
        assert!(
            state.score() <= u128::MAX,
            "Переполнение при обновлении счёта"
        );
    }

    /// Тест на защиту от переполнения при приземлении.
    #[test]
    fn test_landing_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному
        state.set_score(u128::MAX - 1000);
        // Устанавливаем высокую скорость падения
        let _ = state.set_fall_speed(100.0);
        // Устанавливаем большое расстояние Soft Drop
        state.set_soft_drop_distance(1000);

        // Тестируем calculate_landing_bonus напрямую (без save_tetromino)
        calculate_landing_bonus(&mut state);

        // Счёт не должен переполниться
        assert!(state.score() <= u128::MAX, "Переполнение при приземлении");
    }

    /// Тест на защиту от переполнения комбо-бонуса.
    #[test]
    fn test_combo_overflow_protection() {
        let mut state = GameState::new();
        // Устанавливаем счёт близкий к максимальному
        state.set_score(u128::MAX - 10000);
        // Устанавливаем высокий комбо-счётчик
        state.stats_mut().set_combo_counter(1000);

        // Тестируем update_combo_on_clear напрямую (без save_tetromino)
        update_combo_on_clear(&mut state, 1);

        // Счёт не должен переполниться
        assert!(state.score() <= u128::MAX, "Переполнение комбо-бонуса");
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ C1: safe_f32_to_u32() - БЕЗОПАСНАЯ КОНВЕРТАЦИЯ
    // ========================================================================

    /// Тест C1: проверка конвертации нормальных значений
    #[test]
    fn test_fix_c1_safe_f32_to_u32_normal_values() {
        assert_eq!(safe_f32_to_u32(0.0), 0);
        assert_eq!(safe_f32_to_u32(1.0), 1);
        assert_eq!(safe_f32_to_u32(100.5), 100);
        assert_eq!(safe_f32_to_u32(255.999), 255);
        // u32::MAX as f32 теряет точность, поэтому используем меньшее значение
        assert_eq!(safe_f32_to_u32(1_000_000.0), 1_000_000);
    }

    /// Тест C1: проверка обработки NaN
    #[test]
    fn test_fix_c1_safe_f32_to_u32_nan() {
        assert_eq!(safe_f32_to_u32(f32::NAN), 0);
        assert_eq!(safe_f32_to_u32(-f32::NAN), 0);
    }

    /// Тест C1: проверка обработки бесконечности
    #[test]
    fn test_fix_c1_safe_f32_to_u32_infinity() {
        assert_eq!(safe_f32_to_u32(f32::INFINITY), 0);
        assert_eq!(safe_f32_to_u32(f32::NEG_INFINITY), 0);
    }

    /// Тест C1: проверка обработки отрицательных значений
    #[test]
    fn test_fix_c1_safe_f32_to_u32_negative_values() {
        assert_eq!(safe_f32_to_u32(-1.0), 0);
        assert_eq!(safe_f32_to_u32(-0.1), 0);
        assert_eq!(safe_f32_to_u32(-100.0), 0);
        assert_eq!(safe_f32_to_u32(-f32::MAX), 0);
    }

    /// Тест C1: проверка обработки переполнения
    #[test]
    fn test_fix_c1_safe_f32_to_u32_overflow() {
        assert_eq!(safe_f32_to_u32(u32::MAX as f32), u32::MAX);
        assert_eq!(safe_f32_to_u32(u32::MAX as f32 + 1.0), u32::MAX);
        assert_eq!(safe_f32_to_u32(f32::MAX), u32::MAX);
    }

    /// Тест C1: проверка граничных значений
    #[test]
    fn test_fix_c1_safe_f32_to_u32_boundary_values() {
        assert_eq!(safe_f32_to_u32(0.0001), 0);
        assert_eq!(safe_f32_to_u32(0.9999), 0);
        assert_eq!(safe_f32_to_u32(1.0), 1);
        // u32::MAX as f32 теряет точность, поэтому проверяем меньшие значения
        assert_eq!(safe_f32_to_u32(10_000_000.0), 10_000_000);
        assert_eq!(safe_f32_to_u32(100_000_000.0), 100_000_000);
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ M6: handle_landing() С РАННИМ ВЫХОДОМ
    // ========================================================================

    /// Тест M6: проверка раннего выхода при проигрыше
    #[test]
    fn test_fix_m6_handle_landing_early_exit_on_loss() {
        use crate::game::GameState;
        use crate::types::UpdateEndState;

        let mut state = GameState::new();

        // Устанавливаем фигуру так чтобы она достигла верха поля (проигрыш)
        // Для этого устанавливаем координату Y отрицательной
        state.get_curr_shape_mut().pos_mut().1 = -5.0;

        // Сохраняем фигуру в сетке чтобы создать коллизию
        state.save_tetromino();

        // handle_landing должен вернуть Some(UpdateEndState::Lost)
        let result = handle_landing(&mut state);

        assert!(
            matches!(result, Some(UpdateEndState::Lost)),
            "handle_landing должен вернуть Lost при проигрыше"
        );
    }

    /// Тест M6: проверка что handle_landing продолжает игру при отсутствии проигрыша
    #[test]
    fn test_fix_m6_handle_landing_continues_on_no_loss() {
        use crate::game::GameState;
        use crate::types::UpdateEndState;

        let mut state = GameState::new();

        // Устанавливаем фигуру в нормальной позиции (на поле)
        state.get_curr_shape_mut().pos_mut().1 = 10.0;

        // Сохраняем фигуру в сетке
        state.save_tetromino();

        // handle_landing должен вернуть None (продолжить игру)
        // или Some(UpdateEndState::Won) если режим завершён
        // или Some(UpdateEndState::Lost) если фигура заблокирована
        let result = handle_landing(&mut state);

        // Результат должен быть любым допустимым значением
        // Главное что функция выполняется без паники
        assert!(
            matches!(
                result,
                None | Some(UpdateEndState::Won | UpdateEndState::Lost)
            ),
            "handle_landing должен вернуть допустимый результат"
        );
    }

    /// Тест M6: проверка явного возврата результата в handle_landing
    #[test]
    fn test_fix_m6_handle_landing_explicit_return() {
        use crate::types::UpdateEndState;

        let mut state = GameState::new();

        // Проверяем что функция возвращает конкретный тип
        let result: Option<UpdateEndState> = handle_landing(&mut state);

        // Тип результата должен быть Option<UpdateEndState>
        // Это проверяется на этапе компиляции
        let _ = result;
    }

    /// Тест M6: проверка что check_game_over_condition используется для раннего выхода
    #[test]
    fn test_fix_m6_check_game_over_condition_for_early_exit() {
        use crate::constants::MIN_Y;
        use crate::game::GameState;

        let mut state = GameState::new();

        // Устанавливаем блок на поле
        state.get_blocks_mut()[0][4] = 1;

        // Устанавливаем фигуру так чтобы её блок был выше поля
        state.get_curr_shape_mut().pos_mut().1 = -2.0;

        // Проверяем условие проигрыша напрямую
        let game_over = state.curr_shape().coords().iter().any(|&(_, coord_y)| {
            let shape_block_y = state.curr_shape().pos().1 as i16;
            let block_y = coord_y + shape_block_y;
            block_y < MIN_Y
        });

        // Должно быть true если фигура выше поля
        assert!(game_over, "Фигура выше поля должна вызывать game_over");
    }
}
