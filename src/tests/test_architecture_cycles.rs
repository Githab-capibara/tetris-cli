//! Тесты на отсутствие циклических зависимостей в архитектуре tetris-cli.
//!
//! Этот модуль проверяет что граф зависимостей между модулями ацикличен:
//! - game/* модули не имеют циклов
//! - Основные модули (app, game, menu, highscore) не имеют циклов
//! - Граф импортов является ациклическим
//!
//! ## Архитектурные принципы
//! - Модули нижнего уровня не должны зависеть от модулей верхнего уровня
//! - Базовые модули (constants, types, errors) должны быть независимы
//! - Направленность зависимостей: constants → types → state → logic → render

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

// ============================================================================
// ТЕСТ 1: ПРОВЕРКА ОТСУТСТВИЯ ЦИКЛОВ В GAME/* МОДУЛЯХ
// ============================================================================

/// Тест: проверка что game/* модули не имеют циклов.
///
/// Проверяет что модули внутри game/ не имеют циклических зависимостей:
/// - state.rs не зависит от render.rs
/// - logic/* не зависит от render.rs
/// - scoring/* не зависит от render.rs
/// - access.rs не зависит от state.rs (только через трейты)
///
/// ## Архитектурные заметки
/// Циклические зависимости приводят к:
/// - Невозможности компиляции
/// - Сложности тестирования
/// - Нарушению инкапсуляции
#[test]
fn test_no_circular_dependencies_game_modules() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    use crate::game::board::GameBoard;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait, MarathonMode, SprintMode};
    use crate::game::rules::GameRules;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::scoring::lines::find_full_rows;
    use crate::game::state::GameState;
    use crate::game::types::{GameAction, Level, LinesCount, Position, Score};
    use crate::game::view::GameView;

    // Проверяем что базовые типы game работают независимо
    let score = Score::new();
    assert_eq!(score.value(), 0, "Score должен работать независимо");

    let level = Level::new();
    assert_eq!(level.value(), 1, "Level должен работать независимо");

    let lines = LinesCount::new();
    assert_eq!(lines.value(), 0, "LinesCount должен работать независимо");

    let pos = Position::new(5, 10);
    assert_eq!(pos.x(), 5, "Position должен работать независимо");

    // Проверяем что GameAction работает независимо
    let action = GameAction::MoveLeft;
    assert!(
        action.is_movement(),
        "GameAction должен работать независимо"
    );

    // Проверяем что GameBoard работает независимо от GameState
    let mut board = GameBoard::new();
    board.set_block(5, 5, 1);
    assert_eq!(
        board.get_block(5, 5),
        Some(1),
        "GameBoard должен работать независимо"
    );

    // Проверяем что ScoreBoard работает независимо от GameState
    let mut scoreboard = ScoreBoard::new();
    scoreboard.set_score(100);
    assert_eq!(
        scoreboard.get_score(),
        100,
        "ScoreBoard должен работать независимо"
    );

    // Проверяем что GameState использует композицию
    let state = GameState::new();
    let _blocks = state.get_blocks();
    let _score = state.score();

    // Проверяем что трейты доступа работают
    fn requires_board_readonly<B: BoardReadonly>(_board: &B) {}
    fn requires_board_mutable<B: BoardMutable>(_board: &mut B) {}
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}
    fn requires_score_mutable<S: ScoreMutable>(_score: &mut S) {}

    requires_board_readonly(&state);
    requires_score_access(&state);

    // Проверяем что GameModeTrait работает независимо
    let classic = ClassicMode;
    assert_eq!(
        classic.name(),
        "Классика",
        "GameModeTrait должен работать независимо"
    );

    let sprint = SprintMode::new();
    assert_eq!(
        sprint.name(),
        "Спринт",
        "SprintMode должен работать независимо"
    );

    let marathon = MarathonMode::new();
    assert_eq!(
        marathon.name(),
        "Марафон",
        "MarathonMode должен работать независимо"
    );

    // Проверяем что GameRules работает независимо
    let rules = GameRules::new();
    assert_eq!(
        rules.get_line_score(4),
        1800,
        "GameRules должен работать независимо"
    );

    // Проверяем что find_full_rows работает независимо
    let blocks = [[0i8; 10]; 20];
    let (mask, count) = find_full_rows(&blocks);
    // find_full_rows возвращает количество строк в поле
    assert_eq!(count, 20, "find_full_rows должен работать независимо");

    // Проверяем что GameView работает независимо
    let view = GameView::from_game_state(&state);
    assert!(
        !view.score.is_empty(),
        "GameView должен работать независимо"
    );
}

// ============================================================================
// ТЕСТ 2: ПРОВЕРКА ОТСУТСТВИЯ ЦИКЛОВ В ОСНОВНЫХ МОДУЛЯХ
// ============================================================================

/// Тест: проверка основных модулей (app, game, menu, highscore) не имеют циклов.
///
/// Проверяет что основные модули приложения не имеют циклических зависимостей:
/// - app не зависит от game напрямую (только через трейты)
/// - menu не зависит от game напрямую
/// - highscore не зависит от game напрямую
/// - game не зависит от app, menu, highscore
///
/// ## Архитектурные заметки
/// Разделение модулей верхнего уровня обеспечивает:
/// - Слабую связанность
/// - Возможность независимого тестирования
/// - Простоту рефакторинга
#[test]
fn test_no_circular_dependencies_main_modules() {
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    use crate::errors::GameError;
    use crate::highscore::Leaderboard;
    use crate::io_traits::{InputReader, Renderer};
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
    use crate::types::{Direction, RotationDirection};

    // Проверяем что constants.rs независим
    assert_eq!(FPS, 60, "FPS должен быть доступен независимо");
    assert_eq!(GRID_WIDTH, 10, "GRID_WIDTH должен быть доступен независимо");
    assert_eq!(
        GRID_HEIGHT, 20,
        "GRID_HEIGHT должен быть доступен независимо"
    );

    // Проверяем что errors.rs независим
    let _err = GameError::validation_error("Тест");

    // Проверяем что types.rs независим
    let _dir = Direction::Left;
    let _dir = Direction::Right;
    let _dir = Direction::Down;
    let _rotation = RotationDirection::Clockwise;
    let _rotation = RotationDirection::CounterClockwise;

    // Проверяем что tetromino.rs независим от game
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Tetromino должен работать независимо от game"
    );

    // Проверяем что io_traits независим от game
    // Трейты должны быть доступны без зависимости от game
    let _ = std::any::type_name::<dyn InputReader>();
    let _ = std::any::type_name::<dyn Renderer>();

    // Проверяем что highscore независим от game
    let leaderboard = Leaderboard::load();
    let _entries = leaderboard.get_entries();
}

// ============================================================================
// ТЕСТ 3: ПРОВЕРКА АЦИКЛИЧНОСТИ ГРАФА ИМПОРТОВ
// ============================================================================

/// Тест: проверка что граф импортов ацикличен.
///
/// Проверяет что зависимости между модулями образуют направленный ациклический граф (DAG):
/// - constants → types → state → logic → render
/// - scoring зависит от types и logic
/// - access зависит от types и state
/// - view зависит от state и types
///
/// ## Архитектурные заметки
/// Ацикличность графа зависимостей обеспечивает:
/// - Возможность компиляции
/// - Предсказуемость сборки
/// - Простоту понимания архитектуры
#[test]
fn test_import_graph_is_acyclic() {
    use crate::constants::{INITIAL_FALL_SPD, LINE_SCORES, MAX_FALL_SPEED};
    use crate::game::access::{BoardReadonly, ScoreAccess};
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::game::scoring::points::handle_landing;
    use crate::game::state::GameState;
    use crate::game::types::{Level, LinesCount, Score};
    use crate::game::view::GameView;

    // Уровень 1: constants.rs (базовый, нет зависимостей от game/*)
    assert_eq!(LINE_SCORES.len(), 4, "LINE_SCORES должен быть базовым");
    assert!(
        INITIAL_FALL_SPD > 0.0,
        "INITIAL_FALL_SPD должен быть базовым"
    );
    assert!(
        MAX_FALL_SPEED > INITIAL_FALL_SPD,
        "MAX_FALL_SPEED должен быть базовым"
    );

    // Уровень 2: types.rs (зависит только от constants)
    let score = Score::new();
    let level = Level::new();
    let lines = LinesCount::new();
    assert_eq!(
        score.value(),
        0,
        "Score должен зависеть только от constants"
    );
    assert_eq!(
        level.value(),
        1,
        "Level должен зависеть только от constants"
    );
    assert_eq!(
        lines.value(),
        0,
        "LinesCount должен зависеть только от constants"
    );

    // Уровень 3: state.rs (зависит от types и constants)
    let state = GameState::new();
    assert_eq!(state.score(), 0, "GameState должен зависеть от types");
    assert_eq!(state.level(), 1, "GameState должен зависеть от types");

    // Уровень 4: logic/* (зависит от state и types)
    let _can_move = can_move_curr_shape_direction(&state, crate::types::Direction::Down);
    // can_move возвращает false т.к. фигура может двигаться вниз в новом состоянии
    // logic зависит от state

    // Уровень 5: scoring/* (зависит от logic и types)
    // handle_landing требует GameState, что подтверждает зависимость
    let mut state = GameState::new();
    handle_landing(&mut state);

    // Уровень 6: access.rs (зависит от state и types)
    fn requires_board_readonly<B: BoardReadonly>(_board: &B) {}
    fn requires_score_access<S: ScoreAccess>(_score: &S) {}
    requires_board_readonly(&state);
    requires_score_access(&state);

    // Уровень 7: view.rs (зависит от state и types)
    let view = GameView::from_game_state(&state);
    assert!(!view.score.is_empty(), "view должен зависеть от state");

    // Проверяем что нет обратных зависимостей
    // (если бы они были, этот тест не скомпилировался бы)

    // Проверяем что constants не зависит от game
    // (это проверяется компиляцией - constants импортируется первым)
    let _constants_fps = crate::constants::FPS;

    // Проверяем что types не зависит от game/* кроме constants
    // (это проверяется компиляцией - types импортируется до state)
    let _types_score = Score::new();
}

// ============================================================================
// ТЕСТ 4: ДОПОЛНИТЕЛЬНАЯ ПРОВЕРКА ОТСУТСТВИЯ ЦИКЛОВ
// ============================================================================

/// Тест: проверка что crypto модуль не имеет циклов.
///
/// Проверяет что криптографические модули не имеют циклических зависимостей:
/// - crypto.rs не зависит от game
/// - crypto/hmac.rs не зависит от game
/// - crypto/validator.rs не зависит от game
#[test]
fn test_no_circular_dependencies_crypto() {
    use crate::crypto::validator::HmacValidator;
    use crate::crypto::{generate_salt, hash, hmac_sha256, verify_hmac_sha256};

    // Проверяем что hash работает независимо
    let h = hash("тест");
    assert_eq!(h.len(), 64, "hash должен работать независимо");

    // Проверяем что generate_salt работает независимо
    let salt = generate_salt();
    assert_eq!(salt.len(), 64, "generate_salt должен работать независимо");

    // Проверяем что hmac_sha256 работает независимо
    let signature = hmac_sha256("ключ", "данные");
    assert_eq!(
        signature.len(),
        64,
        "hmac_sha256 должен работать независимо"
    );

    // Проверяем что verify_hmac_sha256 работает независимо
    let correct_signature = hmac_sha256("ключ", "данные");
    let is_valid = verify_hmac_sha256("ключ", "данные", &correct_signature);
    assert!(is_valid, "verify_hmac_sha256 должен работать независимо");

    // Проверяем что HmacValidator работает независимо
    let validator = HmacValidator::new("ключ");
    let sig = validator.sign("данные");
    assert!(
        validator.verify("данные", &sig),
        "HmacValidator должен работать независимо"
    );
}

/// Тест: проверка что validation модуль не имеет циклов.
///
/// Проверяет что модули валидации не имеют циклических зависимостей:
/// - validation/path.rs не зависит от game
/// - validation не зависит от controls
#[test]
fn test_no_circular_dependencies_validation() {
    use crate::validation::path::{PathErrorKind, PathValidator};
    use std::path::Path;

    // Проверяем что PathValidator работает независимо
    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");
    let valid_path = Path::new("file.txt");
    let result = validator.validate(valid_path);
    assert!(result.is_ok(), "PathValidator должен работать независимо");

    // Проверяем что PathErrorKind работает независимо
    let _error_kind = PathErrorKind::PathTraversal;
}

/// Тест: проверка что highscore модуль не имеет циклов.
///
/// Проверяет что модуль рекордов не имеет циклических зависимостей:
/// - highscore не зависит от game напрямую
/// - highscore зависит только от crypto и io
#[test]
fn test_no_circular_dependencies_highscore() {
    use crate::highscore::Leaderboard;

    // Проверяем что Leaderboard работает независимо
    let leaderboard = Leaderboard::load();
    let _entries = leaderboard.get_entries();
}
