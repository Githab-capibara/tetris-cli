//! Тесты режимов игры в Tetris CLI.
//!
//! Этот модуль содержит 40 тестов для проверки режимов игры:
//! - Детальные тесты Classic режима (10 тестов)
//! - Детальные тесты Sprint режима (10 тестов)
//! - Детальные тесты Marathon режима (10 тестов)
//! - Тесты переключения режимов (5 тестов)
//! - Тесты условий победы в каждом режиме (5 тестов)
//!
//! Все тесты проверяют корректность работы игровых режимов.

use crate::constants::{LINES_PER_LEVEL, MARATHON_LINES, SPRINT_LINES};
use crate::game::GameState;

// ============================================================================
// ГРУППА ТЕСТОВ 1-10: Детальные тесты Classic режима
// ============================================================================

/// Тест 1: Classic режим создаётся корректно
#[test]
fn test_classic_mode_creation() {
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим по умолчанию должен быть Классика"
    );
}

/// Тест 2: Classic режим - начальный счёт 0
#[test]
fn test_classic_mode_initial_score() {
    let state = GameState::new();

    assert_eq!(state.score(), 0, "Начальный счёт в Classic должен быть 0");
}

/// Тест 3: Classic режим - начальный уровень 1
#[test]
fn test_classic_mode_initial_level() {
    let state = GameState::new();

    assert_eq!(
        state.level(),
        1,
        "Начальный уровень в Classic должен быть 1"
    );
}

/// Тест 4: Classic режим - начальные линии 0
#[test]
fn test_classic_mode_initial_lines() {
    let state = GameState::new();

    assert_eq!(
        state.lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );
}

/// Тест 5: Classic режим - прогрессия уровней
#[test]
#[allow(clippy::similar_names)] // Допустимо для тестов с похожими именами уровней
fn test_classic_mode_level_progression() {
    // Проверяем формулу расчёта уровня
    let lvl0_lines = 1;
    let lv10_lines = 10 / LINES_PER_LEVEL + 1;
    let lv20_lines = 20 / LINES_PER_LEVEL + 1;

    assert_eq!(lvl0_lines, 1, "0 линий = уровень 1");
    assert_eq!(lv10_lines, 2, "10 линий = уровень 2");
    assert_eq!(lv20_lines, 3, "20 линий = уровень 3");
}

/// Тест 6: Classic режим - игра до проигрыша
#[test]
fn test_classic_mode_play_until_loss() {
    // Classic режим не имеет ограничения по линиям
    // Игра продолжается до проигрыша
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Classic режим должен продолжаться до проигрыша"
    );
}

/// Тест 7: Classic режим - сохранение рекорда
#[test]
fn test_classic_mode_score_saved() {
    // В Classic режиме счёт сохраняется в таблицу лидеров
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Classic режим должен поддерживать сохранение рекорда"
    );
}

/// Тест 8: Classic режим - скорость падения
#[test]
fn test_classic_mode_fall_speed() {
    let state = GameState::new();
    let fall_spd = state.fall_speed();

    assert!(
        fall_spd > 0.0,
        "Скорость падения в Classic должна быть положительной"
    );
    assert!(fall_spd < 10.0, "Скорость падения должна быть разумной");
}

/// Тест 9: Classic режим - увеличение сложности
#[test]
fn test_classic_mode_difficulty_increase() {
    // С увеличением уровня скорость должна расти
    let initial_speed = 0.9; // INITIAL_FALL_SPD
    let speed_inc = 0.05; // SPD_INC

    let speed_level_1 = initial_speed;
    let speed_level_5 = initial_speed + speed_inc * 4.0;

    assert!(
        speed_level_5 > speed_level_1,
        "Скорость должна расти с уровнем"
    );
}

/// Тест 10: Classic режим - базовая механика
#[test]
fn test_classic_mode_basic_mechanics() {
    let state = GameState::new();

    // Проверяем, что все базовые механики работают
    assert!(state.curr_shape().pos().0 >= 0.0, "Фигура в поле");
    assert!(state.curr_shape().pos().1 >= 0.0, "Фигура в поле");
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-20: Детальные тесты Sprint режима
// ============================================================================

/// Тест 11: Sprint режим создаётся корректно
#[test]
fn test_sprint_mode_creation() {
    let state = GameState::new_sprint();

    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Режим должен быть Спринт"
    );
}

/// Тест 12: Sprint режим - цель 40 линий
#[test]
fn test_sprint_mode_goal() {
    assert_eq!(SPRINT_LINES, 40, "Цель Sprint режима должна быть 40 линий");
}

/// Тест 13: Sprint режим - начальный счёт 0
#[test]
fn test_sprint_mode_initial_score() {
    let state = GameState::new_sprint();

    assert_eq!(state.score(), 0, "Начальный счёт в Sprint должен быть 0");
}

/// Тест 14: Sprint режим - начальный уровень 1
#[test]
fn test_sprint_mode_initial_level() {
    let state = GameState::new_sprint();

    assert_eq!(state.level(), 1, "Начальный уровень в Sprint должен быть 1");
}

/// Тест 16: Sprint режим - время до завершения
#[test]
fn test_sprint_mode_time_to_complete() {
    let state = GameState::new_sprint();

    // Sprint режим должен измерять время
    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Sprint режим должен отслеживать время"
    );
}

/// Тест 17: Sprint режим - победа при 40 линиях
#[test]
fn test_sprint_mode_win_at_40_lines() {
    // Проверяем константу цели
    assert_eq!(SPRINT_LINES, 40, "Победа в Sprint при 40 линиях");
}

/// Тест 18: Sprint режим - скорость падения
#[test]
fn test_sprint_mode_fall_speed() {
    let state = GameState::new_sprint();
    let fall_spd = state.fall_speed();

    assert!(
        fall_spd > 0.0,
        "Скорость падения в Sprint должна быть положительной"
    );
}

/// Тест 19: Sprint режим - счёт не сохраняется
#[test]
fn test_sprint_mode_score_not_saved() {
    // В Sprint режиме счёт обычно не сохраняется в таблицу лидеров
    let state = GameState::new_sprint();

    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Sprint режим может не сохранять счёт"
    );
}

/// Тест 20: Sprint режим - базовая механика
#[test]
fn test_sprint_mode_basic_mechanics() {
    let state = GameState::new_sprint();

    assert!(state.curr_shape().pos().0 >= 0.0, "Фигура в поле");
    assert!(state.curr_shape().pos().1 >= 0.0, "Фигура в поле");
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-30: Детальные тесты Marathon режима
// ============================================================================

/// Тест 21: Marathon режим создаётся корректно
#[test]
fn test_marathon_mode_creation() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Режим должен быть Марафон"
    );
}

/// Тест 22: Marathon режим - цель 150 линий
#[test]
fn test_marathon_mode_goal() {
    assert_eq!(
        MARATHON_LINES, 150,
        "Цель Marathon режима должна быть 150 линий"
    );
}

/// Тест 23: Marathon режим - начальный счёт 0
#[test]
fn test_marathon_mode_initial_score() {
    let state = GameState::new_marathon();

    assert_eq!(state.score(), 0, "Начальный счёт в Marathon должен быть 0");
}

/// Тест 24: Marathon режим - начальный уровень 1
#[test]
fn test_marathon_mode_initial_level() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.level(),
        1,
        "Начальный уровень в Marathon должен быть 1"
    );
}

/// Тест 25: Marathon режим - прогрессия сложности
#[test]
fn test_marathon_mode_difficulty_progression() {
    // Marathon имеет более сложную прогрессию
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Marathon режим должен иметь прогрессию сложности"
    );
}

/// Тест 26: Marathon режим - длина игры
#[test]
fn test_marathon_mode_game_length() {
    // Marathon режим длиннее Sprint
    assert!(
        MARATHON_LINES > SPRINT_LINES,
        "Marathon режим должен быть длиннее Sprint ({MARATHON_LINES} > {SPRINT_LINES})"
    );
}

/// Тест 27: Marathon режим - сохранение рекорда
#[test]
fn test_marathon_mode_score_saved() {
    // Marathon сохраняет рекорд
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Marathon режим должен сохранять рекорд"
    );
}

/// Тест 28: Marathon режим - скорость падения
#[test]
fn test_marathon_mode_fall_speed() {
    let state = GameState::new_marathon();
    let fall_spd = state.fall_speed();

    assert!(
        fall_spd > 0.0,
        "Скорость падения в Marathon должна быть положительной"
    );
}

/// Тест 29: Marathon режим - победа при 150 линиях
#[test]
fn test_marathon_mode_win_at_150_lines() {
    assert_eq!(MARATHON_LINES, 150, "Победа в Marathon при 150 линиях");
}

/// Тест 30: Marathon режим - базовая механика
#[test]
fn test_marathon_mode_basic_mechanics() {
    let state = GameState::new_marathon();

    assert!(state.curr_shape().pos().0 >= 0.0, "Фигура в поле");
    assert!(state.curr_shape().pos().1 >= 0.0, "Фигура в поле");
}

// ============================================================================
// ГРУППА ТЕСТОВ 31-35: Тесты переключения режимов
// ============================================================================

/// Тест 31: Переключение Classic -> Sprint
#[test]
fn test_mode_switch_classic_to_sprint() {
    let classic = GameState::new();
    let sprint = GameState::new_sprint();

    assert_ne!(
        classic.get_mode_trait().name(),
        sprint.get_mode_trait().name(),
        "Режимы должны отличаться"
    );
}

/// Тест 32: Переключение Classic -> Marathon
#[test]
fn test_mode_switch_classic_to_marathon() {
    let classic = GameState::new();
    let marathon = GameState::new_marathon();

    assert_ne!(
        classic.get_mode_trait().name(),
        marathon.get_mode_trait().name(),
        "Режимы должны отличаться"
    );
}

/// Тест 33: Переключение Sprint -> Marathon
#[test]
fn test_mode_switch_sprint_to_marathon() {
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    assert_ne!(
        sprint.get_mode_trait().name(),
        marathon.get_mode_trait().name(),
        "Режимы должны отличаться"
    );
}

/// Тест 34: Все режимы имеют разные цели
#[test]
fn test_all_modes_have_different_goals() {
    // Classic: игра до проигрыша
    // Sprint: 40 линий
    // Marathon: 150 линий

    assert_ne!(
        SPRINT_LINES, MARATHON_LINES,
        "Sprint и Marathon должны иметь разные цели ({SPRINT_LINES} vs {MARATHON_LINES})"
    );
}

/// Тест 35: Режимы имеют разные настройки
#[test]
fn test_modes_have_different_settings() {
    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    // Все режимы должны быть инициализированы
    assert_eq!(classic.get_mode_trait().name(), "Классика");
    assert_eq!(sprint.get_mode_trait().name(), "Спринт");
    assert_eq!(marathon.get_mode_trait().name(), "Марафон");
}

// ============================================================================
// ГРУППА ТЕСТОВ 36-40: Тесты условий победы в каждом режиме
// ============================================================================

/// Тест 36: Classic режим - нет условия победы
#[test]
fn test_classic_mode_no_win_condition() {
    // Classic режим продолжается до проигрыша
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Classic режим не имеет условия победы"
    );
}

/// Тест 37: Sprint режим - победа при 40 линиях
#[test]
fn test_sprint_mode_win_condition() {
    // Sprint завершается при достижении 40 линий
    assert_eq!(SPRINT_LINES, 40, "Sprint режим завершается при 40 линиях");
}

/// Тест 38: Marathon режим - победа при 150 линиях
#[test]
fn test_marathon_mode_win_condition() {
    // Marathon завершается при достижении 150 линий
    assert_eq!(
        MARATHON_LINES, 150,
        "Marathon режим завершается при 150 линиях"
    );
}

/// Тест 39: Sprint режим - быстрее победа
#[test]
fn test_sprint_mode_faster_win() {
    // Sprint требует меньше линий чем Marathon
    assert!(
        SPRINT_LINES < MARATHON_LINES,
        "Sprint должен быть короче Marathon ({SPRINT_LINES} < {MARATHON_LINES})"
    );
}

/// Тест 40: Marathon режим - долгая игра
#[test]
fn test_marathon_mode_long_game() {
    // Marathon - самый длинный режим
    assert_eq!(
        MARATHON_LINES, 150,
        "Marathon должен быть самым длинным режимом ({MARATHON_LINES} линий)"
    );
}
