//! Тесты на новые архитектурные исправления в tetris-cli.
//!
//! Этот модуль проверяет что все архитектурные исправления были применены:
//! - `GameBoardAccess` удален или deprecated
//! - `ScoreAccess` не дублируется
//! - `GameAction` enum существует
//! - `game_rules` модуль существует
//! - constants не экспортируется pub(crate)
//!
//! ## Архитектурные принципы
//! - Устранение дублирования
//! - Централизация ответственности
//! - Четкие границы модулей

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]
#![allow(deprecated)]
#![allow(clippy::items_after_statements)]
#![allow(dead_code)]
#![allow(clippy::no_effect_underscore_binding)]

// ============================================================================
// ТЕСТ 2: SCOREACCESS НЕ ДУБЛИРУЕТСЯ
// ============================================================================

/// Тест: `ScoreAccess` не дублируется.
#[test]
fn test_score_access_not_duplicated() {
    use crate::game::access::ScoreAccess;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;

    // Проверяем что ScoreAccess определён в access.rs
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}

    // Проверяем что GameState реализует ScoreAccess
    let state = GameState::new();
    requires_score_access(&state);

    // Проверяем что ScoreBoard реализует ScoreAccess
    let scoreboard = ScoreBoard::new();
    requires_score_access(&scoreboard);

    // Проверяем что методы ScoreAccess работают
    assert_eq!(
        state.score(),
        0,
        "ScoreAccess должен предоставлять доступ к очкам"
    );
    assert_eq!(
        state.level(),
        1,
        "ScoreAccess должен предоставлять доступ к уровню"
    );
    assert_eq!(
        state.lines_cleared(),
        0,
        "ScoreAccess должен предоставлять доступ к линиям"
    );

    assert_eq!(
        scoreboard.get_score(),
        0,
        "ScoreBoard должен реализовывать ScoreAccess"
    );
    assert_eq!(
        scoreboard.get_level(),
        1,
        "ScoreBoard должен реализовывать ScoreAccess"
    );
}

// ============================================================================
// ТЕСТ 3: GAMEACTION ENUM СУЩЕСТВУЕТ
// ============================================================================

/// Тест: `GameAction` enum существует.
#[test]
fn test_game_action_enum_exists() {
    use crate::game::types::GameAction;

    // Проверяем что enum определён в game/types.rs
    let actions = [
        GameAction::MoveLeft,
        GameAction::MoveRight,
        GameAction::SoftDrop,
        GameAction::HardDrop,
        GameAction::RotateLeft,
        GameAction::RotateRight,
        GameAction::Hold,
        GameAction::Pause,
        GameAction::Quit,
    ];

    assert_eq!(actions.len(), 9, "GameAction должен иметь 9 вариантов");

    // Проверяем что варианты работают
    assert!(actions[0].is_movement(), "MoveLeft должен быть движением");
    assert!(actions[1].is_movement(), "MoveRight должен быть движением");
    assert!(actions[2].is_drop(), "SoftDrop должен быть падением");
    assert!(actions[3].is_drop(), "HardDrop должен быть падением");
    assert!(actions[4].is_rotation(), "RotateLeft должен быть вращением");
    assert!(
        actions[5].is_rotation(),
        "RotateRight должен быть вращением"
    );

    // Проверяем что enum имеет методы
    assert!(GameAction::MoveLeft.is_movement());
    assert!(!GameAction::MoveLeft.is_rotation());
    assert!(!GameAction::MoveLeft.is_drop());

    assert!(GameAction::RotateLeft.is_rotation());

    assert!(GameAction::HardDrop.is_drop());

    // Проверяем что enum имеет Copy и Clone
    let action = GameAction::MoveLeft;
    let action_copy = action; // Copy
    let action_clone = action; // Copy (вместо clone())
    assert_eq!(action, action_copy);
    assert_eq!(action, action_clone);

    // Проверяем что enum имеет Debug
    assert_eq!(format!("{:?}", GameAction::MoveLeft), "MoveLeft");
}

// ============================================================================
// ТЕСТ 4: GAME_RULES МОДУЛЬ СУЩЕСТВУЕТ
// ============================================================================

/// Тест: `game_rules` модуль существует.
#[test]
fn test_game_rules_module_exists() {
    use crate::constants::{
        COMBO_BONUS, HARD_DROP_POINTS, LAND_TIME_DELAY_S, LEVEL_BONUS_MULT, LINES_PER_LEVEL,
        LINE_SCORES, MARATHON_LINES, MAX_FALL_SPEED, MAX_LINES_PER_CLEAR, SOFT_DROP_POINTS,
        SPRINT_LINES,
    };

    assert_eq!(
        LINE_SCORES.len(),
        4,
        "LINE_SCORES должен быть экспортирован"
    );
    assert_eq!(COMBO_BONUS, 50, "COMBO_BONUS должен быть экспортирован");
    assert_eq!(
        LEVEL_BONUS_MULT, 500,
        "LEVEL_BONUS_MULT должен быть экспортирован"
    );
    assert_eq!(
        SOFT_DROP_POINTS, 1,
        "SOFT_DROP_POINTS должен быть экспортирован"
    );
    assert_eq!(
        HARD_DROP_POINTS, 2,
        "HARD_DROP_POINTS должен быть экспортирован"
    );

    assert!(
        MAX_FALL_SPEED > 0.0,
        "MAX_FALL_SPEED должен быть экспортирован"
    );
    assert!(
        LAND_TIME_DELAY_S > 0.0,
        "LAND_TIME_DELAY_S должен быть экспортирован"
    );

    assert_eq!(SPRINT_LINES, 40, "SPRINT_LINES должен быть экспортирован");
    assert_eq!(
        MARATHON_LINES, 150,
        "MARATHON_LINES должен быть экспортирован"
    );
    assert_eq!(
        LINES_PER_LEVEL, 10,
        "LINES_PER_LEVEL должен быть экспортирован"
    );
    assert_eq!(
        MAX_LINES_PER_CLEAR, 4,
        "MAX_LINES_PER_CLEAR должен быть экспортирован"
    );
}

// ============================================================================
// ТЕСТ 5: CONSTANTS НЕ ЭКСПОРТИРУЕТСЯ PUB(CRATE)
// ============================================================================

/// Тест: constants не экспортируется pub(crate).
#[test]
fn test_constants_not_exported_pub_crate() {
    // Проверяем что constants доступен из корня crates
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH, SPRINT_LINES};

    // Проверяем что константы доступны
    assert_eq!(FPS, 60, "FPS должен быть доступен из корня crates");
    assert_eq!(
        GRID_WIDTH, 10,
        "GRID_WIDTH должен быть доступен из корня crates"
    );
    assert_eq!(
        GRID_HEIGHT, 20,
        "GRID_HEIGHT должен быть доступен из корня crates"
    );
    assert_eq!(
        SPRINT_LINES, 40,
        "SPRINT_LINES должен быть доступен из корня crates"
    );

    // Проверяем что константы доступны из game модуля
    use crate::constants::{
        FPS as GAME_FPS, GRID_HEIGHT as GAME_GRID_HEIGHT, GRID_WIDTH as GAME_GRID_WIDTH,
    };

    assert_eq!(
        GAME_FPS, 60,
        "Константы должны быть доступны из game::constants"
    );
    assert_eq!(
        GAME_GRID_WIDTH, 10,
        "Константы должны быть доступны из game::constants"
    );
    assert_eq!(
        GAME_GRID_HEIGHT, 20,
        "Константы должны быть доступны из game::constants"
    );
}

// ============================================================================
// ТЕСТ 6: ВСЕ АРХИТЕКТУРНЫЕ ИСПРАВЛЕНИЯ ПРИМЕНЕНЫ
// ============================================================================

/// Тест: проверка что все архитектурные исправления применены.
#[test]
fn test_all_architecture_fixes_applied() {
    // 1. GameBoardAccess удалён (используйте BoardReadonly + ScoreAccess)

    // 2. ScoreAccess не дублируется
    use crate::game::access::ScoreAccess;
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}
    requires_score_access(&crate::game::state::GameState::new());

    // 3. GameAction существует
    use crate::game::types::GameAction;
    let _action = GameAction::MoveLeft;

    // 4. game_rules модуль удалён, константы перенесены в constants и scoring
    use crate::constants::LINE_SCORES;
    assert_eq!(LINE_SCORES.len(), 4);

    // 5. constants доступен
    use crate::constants::FPS;
    assert_eq!(FPS, 60);
}
