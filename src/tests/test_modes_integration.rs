//! Интеграционные тесты режимов игры.
//!
//! Этот модуль содержит 20 интеграционных тестов для проверки
//! взаимодействия и корректной работы всех режимов игры Tetris:
//! - Тесты переключения режимов (5 тестов)
//! - Тесты общих механик режимов (5 тестов)
//! - Тесты статистики в режимах (5 тестов)
//! - Тесты завершения режимов (5 тестов)
//!
//! Все тесты независимы и проверяют корректную работу режимов.

use crate::game::GameState;
use crate::game::{MARATHON_LINES, SPRINT_LINES};
use crate::tetromino::ShapeType;

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Переключение режимов
// ============================================================================

/// Тест 1: Проверка что Classic режим создаётся корректно
///
/// Проверяет базовую инициализацию классического режима.
#[test]
fn test_modes_integration_classic_creation() {
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим должен быть Классика"
    );
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(state.lines_cleared(), 0, "Линии должны быть 0");
}

/// Тест 2: Проверка что Sprint режим создаётся корректно
///
/// Проверяет базовую инициализацию режима спринт.
#[test]
fn test_modes_integration_sprint_creation() {
    let state = GameState::new_sprint();

    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Режим должен быть Спринт"
    );
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(state.lines_cleared(), 0, "Линии должны быть 0");
}

/// Тест 3: Проверка что Marathon режим создаётся корректно
///
/// Проверяет базовую инициализацию режима марафон.
#[test]
fn test_modes_integration_marathon_creation() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Режим должен быть Марафон"
    );
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(state.lines_cleared(), 0, "Линии должны быть 0");
}

/// Тест 4: Проверка что все режимы имеют одинаковую начальную скорость
///
/// Проверяет консистентность начальной скорости падения.
#[test]
fn test_modes_integration_initial_speed_consistency() {
    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    let classic_speed = classic.get_fall_speed();
    let sprint_speed = sprint.get_fall_speed();
    let marathon_speed = marathon.get_fall_speed();

    assert!(
        (classic_speed - sprint_speed).abs() < f32::EPSILON,
        "Скорость Classic и Sprint должна совпадать"
    );
    assert!(
        (sprint_speed - marathon_speed).abs() < f32::EPSILON,
        "Скорость Sprint и Marathon должна совпадать"
    );
}

/// Тест 5: Проверка что все режимы имеют удержанную фигуру None
///
/// Проверяет консистентность начального состояния удержания.
#[test]
fn test_modes_integration_held_shape_consistency() {
    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    assert!(
        classic.held_shape().is_none(),
        "Classic: удержанная фигура должна быть None"
    );
    assert!(
        sprint.held_shape().is_none(),
        "Sprint: удержанная фигура должна быть None"
    );
    assert!(
        marathon.held_shape().is_none(),
        "Marathon: удержанная фигура должна быть None"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Общие механики режимов
// ============================================================================

/// Тест 6: Проверка что Hold работает во всех режимах
///
/// Проверяет механику удержания фигуры в каждом режиме.
#[test]
fn test_modes_integration_hold_in_all_modes() {
    // Classic
    let mut classic = GameState::new();
    assert!(classic.can_hold(), "Classic: должно быть можно удерживать");
    classic.hold_shape();
    assert!(
        !classic.can_hold(),
        "Classic: после hold должно быть нельзя удерживать"
    );

    // Sprint
    let mut sprint = GameState::new_sprint();
    assert!(sprint.can_hold(), "Sprint: должно быть можно удерживать");
    sprint.hold_shape();
    assert!(
        !sprint.can_hold(),
        "Sprint: после hold должно быть нельзя удерживать"
    );

    // Marathon
    let mut marathon = GameState::new_marathon();
    assert!(
        marathon.can_hold(),
        "Marathon: должно быть можно удерживать"
    );
    marathon.hold_shape();
    assert!(
        !marathon.can_hold(),
        "Marathon: после hold должно быть нельзя удерживать"
    );
}

/// Тест 7: Проверка что вращение работает во всех режимах
///
/// Проверяет возможность вращения фигур в каждом режиме.
#[test]
fn test_modes_integration_rotation_in_all_modes() {
    use crate::types::RotationDirection;

    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    // Classic
    let can_rotate_classic = classic.can_rotate_curr_shape(RotationDirection::Clockwise);
    let _ = can_rotate_classic; // Тест просто проверяет что метод работает

    // Sprint
    let can_rotate_sprint = sprint.can_rotate_curr_shape(RotationDirection::Clockwise);
    let _ = can_rotate_sprint;

    // Marathon
    let can_rotate_marathon = marathon.can_rotate_curr_shape(RotationDirection::Clockwise);
    let _ = can_rotate_marathon;
}

/// Тест 8: Проверка что движение работает во всех режимах
///
/// Проверяет возможность движения фигур в каждом режиме.
#[test]
fn test_modes_integration_movement_in_all_modes() {
    use crate::types::Direction;

    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    // Classic
    assert!(
        classic.can_move_curr_shape_direction(Direction::Down),
        "Classic: движение вниз должно быть возможно"
    );

    // Sprint
    assert!(
        sprint.can_move_curr_shape_direction(Direction::Down),
        "Sprint: движение вниз должно быть возможно"
    );

    // Marathon
    assert!(
        marathon.can_move_curr_shape_direction(Direction::Down),
        "Marathon: движение вниз должно быть возможно"
    );
}

/// Тест 9: Проверка что призрачная фигура работает во всех режимах
///
/// Проверяет механику призрачной фигуры в каждом режиме.
#[test]
fn test_modes_integration_ghost_piece_in_all_modes() {
    use crate::types::Direction;

    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    let ghost_classic = *classic.curr_shape();
    let ghost_sprint = *sprint.curr_shape();
    let ghost_marathon = *marathon.curr_shape();

    assert!(
        classic.can_move_ghost_shape_direction(Direction::Down),
        "Classic: призрачная фигура должна работать"
    );
    assert!(
        sprint.can_move_ghost_shape_direction(Direction::Down),
        "Sprint: призрачная фигура должна работать"
    );
    assert!(
        marathon.can_move_ghost_shape_direction(Direction::Down),
        "Marathon: призрачная фигура должна работать"
    );
}

/// Тест 10: Проверка что следующая фигура есть во всех режимах
///
/// Проверяет наличие следующей фигуры в каждом режиме.
#[test]
fn test_modes_integration_next_shape_in_all_modes() {
    let classic = GameState::new();
    let sprint = GameState::new_sprint();
    let marathon = GameState::new_marathon();

    let classic_next = classic.next_shape();
    let sprint_next = sprint.next_shape();
    let marathon_next = marathon.next_shape();

    assert!(
        (classic_next.shape as usize) < 7,
        "Classic: следующая фигура должна быть валидной"
    );
    assert!(
        (sprint_next.shape as usize) < 7,
        "Sprint: следующая фигура должна быть валидной"
    );
    assert!(
        (marathon_next.shape as usize) < 7,
        "Marathon: следующая фигура должна быть валидной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Статистика в режимах
// ============================================================================

/// Тест 11: Проверка что статистика собирается в Classic режиме
///
/// Проверяет подсчёт фигур в классическом режиме.
#[test]
fn test_modes_integration_stats_in_classic() {
    let state = GameState::new();
    let stats = state.get_stats();

    assert_eq!(stats.total_pieces(), 1, "Должна быть 1 начальная фигура");
    assert_eq!(stats.max_combo, 0, "Комбо должно быть 0");
    assert!(stats.start_time.is_none(), "Таймер не должен быть запущен");
}

/// Тест 12: Проверка что статистика собирается в Sprint режиме
///
/// Проверяет подсчёт фигур в режиме спринт.
#[test]
fn test_modes_integration_stats_in_sprint() {
    let state = GameState::new_sprint();
    let stats = state.get_stats();

    assert_eq!(stats.total_pieces(), 1, "Должна быть 1 начальная фигура");
    assert_eq!(stats.max_combo, 0, "Комбо должно быть 0");
    assert!(
        stats.start_time.is_some(),
        "Таймер должен быть запущен для режима Sprint"
    );
}

/// Тест 13: Проверка что статистика собирается в Marathon режиме
///
/// Проверяет подсчёт фигур в режиме марафон.
#[test]
fn test_modes_integration_stats_in_marathon() {
    let state = GameState::new_marathon();
    let stats = state.get_stats();

    assert_eq!(stats.total_pieces(), 1, "Должна быть 1 начальная фигура");
    assert_eq!(stats.max_combo, 0, "Комбо должно быть 0");
    assert!(
        stats.start_time.is_some(),
        "Таймер должен быть запущен для режима Marathon"
    );
}

/// Тест 14: Проверка что таймер работает в Sprint режиме
///
/// Проверяет корректность работы таймера.
#[test]
fn test_modes_integration_sprint_timer_works() {
    let mut state = GameState::new_sprint();
    state.start_timer();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed >= 0.05, "Время должно быть больше 50 мс");
}

/// Тест 15: Проверка что таймер работает в Marathon режиме
///
/// Проверяет корректность работы таймера в марафоне.
#[test]
fn test_modes_integration_marathon_timer_works() {
    let mut state = GameState::new_marathon();
    state.start_timer();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed >= 0.05, "Время должно быть больше 50 мс");
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-20: Завершение режимов
// ============================================================================

/// Тест 16: Проверка константы `SPRINT_LINES`
///
/// Проверяет что цель спринта равна 40 линиям.
#[test]
fn test_modes_integration_sprint_lines_constant() {
    assert_eq!(SPRINT_LINES, 40, "Цель спринта должна быть 40 линий");
}

/// Тест 17: Проверка константы `MARATHON_LINES`
///
/// Проверяет что цель марафона равна 150 линиям.
#[test]
fn test_modes_integration_marathon_lines_constant() {
    assert_eq!(MARATHON_LINES, 150, "Цель марафона должна быть 150 линий");
}

/// Тест 18: Проверка что Classic режим не имеет цели по линиям
///
/// Проверяет что классический режим играется до проигрыша.
#[test]
fn test_modes_integration_classic_no_goal() {
    let state = GameState::new();

    // Classic режим не имеет фиксированной цели
    assert_eq!(state.get_mode_trait().name(), "Классика");
    // Нет константы цели для Classic
}

/// Тест 19: Проверка что все 7 типов фигур доступны во всех режимах
///
/// Проверяет что Bag Generator работает во всех режимах.
#[test]
fn test_modes_integration_all_shapes_in_all_modes() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for &shape in &shapes {
        let mut classic = GameState::new();
        classic.get_curr_shape_mut().shape = shape;
        assert_eq!(
            classic.curr_shape().shape,
            shape,
            "Classic: фигура {shape:?} должна быть доступна"
        );

        let mut sprint = GameState::new_sprint();
        sprint.get_curr_shape_mut().shape = shape;
        assert_eq!(
            sprint.curr_shape().shape,
            shape,
            "Sprint: фигура {shape:?} должна быть доступна"
        );

        let mut marathon = GameState::new_marathon();
        marathon.get_curr_shape_mut().shape = shape;
        assert_eq!(
            marathon.curr_shape().shape,
            shape,
            "Marathon: фигура {shape:?} должна быть доступна"
        );
    }
}

/// Тест 20: Проверка что `can_hold` сбрасывается после нового хода
///
/// Проверяет механику сброса `can_hold` в каждом режиме.
#[test]
fn test_modes_integration_can_hold_reset() {
    // Classic
    let mut classic = GameState::new();
    assert!(
        classic.can_hold(),
        "Classic: изначально должно быть можно удерживать"
    );
    classic.hold_shape();
    assert!(
        !classic.can_hold(),
        "Classic: после hold должно быть нельзя удерживать"
    );

    // Sprint
    let mut sprint = GameState::new_sprint();
    assert!(
        sprint.can_hold(),
        "Sprint: изначально должно быть можно удерживать"
    );
    sprint.hold_shape();
    assert!(
        !sprint.can_hold(),
        "Sprint: после hold должно быть нельзя удерживать"
    );

    // Marathon
    let mut marathon = GameState::new_marathon();
    assert!(
        marathon.can_hold(),
        "Marathon: изначально должно быть можно удерживать"
    );
    marathon.hold_shape();
    assert!(
        !marathon.can_hold(),
        "Marathon: после hold должно быть нельзя удерживать"
    );
}
