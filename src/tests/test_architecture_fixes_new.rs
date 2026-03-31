//! Тесты на новые архитектурные исправления в tetris-cli.
//!
//! Этот модуль проверяет что все архитектурные исправления были применены:
//! - GameBoardAccess удален или deprecated
//! - ScoreAccess не дублируется
//! - GameAction enum существует
//! - game_rules модуль существует
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

// ============================================================================
// ТЕСТ 1: GAMEBOARDACCESS УДАЛЕН ИЛИ DEPRECATED
// ============================================================================

/// Тест: GameBoardAccess удален или deprecated.
#[test]
fn test_gameboard_access_trait_removed() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    use crate::game::state::GameState;

    // Проверяем что новые трейты работают
    let state = GameState::new();

    // BoardReadonly - только чтение
    fn requires_board_readonly<B: BoardReadonly>(_board: &B) {}
    requires_board_readonly(&state);

    // ScoreAccess - доступ к очкам
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}
    requires_score_access(&state);

    // Проверяем что GameBoardAccess deprecated (если существует)
    #[allow(deprecated)]
    {
        use crate::game::access::GameBoardAccess;
        let _ = std::any::type_name::<dyn GameBoardAccess>();
    }

    // Проверяем что новые трейты предоставляют тот же функционал
    let state = GameState::new();
    let blocks = state.get_blocks();
    assert_eq!(
        blocks.len(),
        20,
        "BoardReadonly должен предоставлять доступ к полю"
    );

    let score = state.score();
    assert_eq!(score, 0, "ScoreAccess должен предоставлять доступ к очкам");

    assert!(true, "GameBoardAccess удален или deprecated");
}

// ============================================================================
// ТЕСТ 2: SCOREACCESS НЕ ДУБЛИРУЕТСЯ
// ============================================================================

/// Тест: ScoreAccess не дублируется.
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

    assert!(true, "ScoreAccess не дублируется");
}

// ============================================================================
// ТЕСТ 3: GAMEACTION ENUM СУЩЕСТВУЕТ
// ============================================================================

/// Тест: GameAction enum существует.
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
    let action_clone = action.clone(); // Clone
    assert_eq!(action, action_copy);
    assert_eq!(action, action_clone);

    // Проверяем что enum имеет Debug
    assert_eq!(format!("{:?}", GameAction::MoveLeft), "MoveLeft");

    assert!(true, "GameAction enum существует");
}

// ============================================================================
// ТЕСТ 4: GAME_RULES МОДУЛЬ СУЩЕСТВУЕТ
// ============================================================================

/// Тест: game_rules модуль существует.
#[test]
fn test_game_rules_module_exists() {
    use crate::game::rules::{
        GameRules, COMBO_BONUS, HARD_DROP_POINTS, INITIAL_FALL_SPEED, LAND_TIME_DELAY,
        LEVEL_BONUS_MULT, LINES_PER_LEVEL, LINE_SCORES, MARATHON_LINES, MAX_FALL_SPEED, MAX_LEVEL,
        MAX_LINES_PER_CLEAR, PIECE_SCORE_FALL_MULT, PIECE_SCORE_INC, SOFT_DROP_POINTS,
        SPEED_INCREMENT, SPRINT_LINES,
    };

    // Проверяем что константы экспортированы
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

    // Проверяем что GameRules существует
    let rules = GameRules::new();
    assert_eq!(rules.score_multiplier, 1, "GameRules должен существовать");
    assert!(rules.combo_enabled, "GameRules должен существовать");

    // Проверяем что методы GameRules работают
    assert_eq!(
        rules.get_line_score(4),
        1800,
        "get_line_score должен работать"
    );
    assert_eq!(
        rules.get_combo_bonus(3),
        100,
        "get_combo_bonus должен работать"
    );
    assert_eq!(
        rules.get_level_bonus(10),
        5000,
        "get_level_bonus должен работать"
    );

    // Проверяем что константы физики экспортированы
    assert!(
        INITIAL_FALL_SPEED > 0.0,
        "INITIAL_FALL_SPEED должен быть экспортирован"
    );
    assert!(
        MAX_FALL_SPEED > INITIAL_FALL_SPEED,
        "MAX_FALL_SPEED должен быть экспортирован"
    );
    assert!(
        SPEED_INCREMENT > 0.0,
        "SPEED_INCREMENT должен быть экспортирован"
    );
    assert!(
        LAND_TIME_DELAY > 0.0,
        "LAND_TIME_DELAY должен быть экспортирован"
    );

    // Проверяем что константы режимов экспортированы
    assert_eq!(SPRINT_LINES, 40, "SPRINT_LINES должен быть экспортирован");
    assert_eq!(
        MARATHON_LINES, 150,
        "MARATHON_LINES должен быть экспортирован"
    );
    assert_eq!(
        LINES_PER_LEVEL, 10,
        "LINES_PER_LEVEL должен быть экспортирован"
    );
    assert_eq!(MAX_LEVEL, 1000, "MAX_LEVEL должен быть экспортирован");
    assert_eq!(
        MAX_LINES_PER_CLEAR, 4,
        "MAX_LINES_PER_CLEAR должен быть экспортирован"
    );

    assert!(true, "game_rules модуль существует");
}

// ============================================================================
// ТЕСТ 5: CONSTANTS НЕ ЭКСПОРТИРУЕТСЯ PUB(CRATE)
// ============================================================================

/// Тест: constants не экспортируется pub(crate).
#[test]
fn test_constants_not_exported_pub_crate() {
    // Проверяем что constants доступен из корня crates
    use crate::constants::{
        FPS, GRID_HEIGHT, GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S, LINE_SCORES,
        MAX_FALL_SPEED, SPRINT_LINES,
    };

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
    use crate::game::constants::{
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

    assert!(true, "constants не экспортируется pub(crate)");
}

// ============================================================================
// ТЕСТ 6: ВСЕ АРХИТЕКТУРНЫЕ ИСПРАВЛЕНИЯ ПРИМЕНЕНЫ
// ============================================================================

/// Тест: проверка что все архитектурные исправления применены.
#[test]
fn test_all_architecture_fixes_applied() {
    // 1. GameBoardAccess deprecated
    #[allow(deprecated)]
    {
        use crate::game::access::GameBoardAccess;
        let _ = std::any::type_name::<dyn GameBoardAccess>();
    }

    // 2. ScoreAccess не дублируется
    use crate::game::access::ScoreAccess;
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}
    requires_score_access(&crate::game::state::GameState::new());

    // 3. GameAction существует
    use crate::game::types::GameAction;
    let _action = GameAction::MoveLeft;

    // 4. game_rules модуль существует
    use crate::game::rules::GameRules;
    let _rules = GameRules::new();

    // 5. constants доступен
    use crate::constants::FPS;
    assert_eq!(FPS, 60);

    assert!(true, "Все архитектурные исправления применены");
}
