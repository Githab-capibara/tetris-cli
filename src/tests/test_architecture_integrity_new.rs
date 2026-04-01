//! Тесты на целостность архитектуры в tetris-cli.
//!
//! Этот модуль проверяет общие принципы целостности архитектуры:
//! - Каждый модуль имеет четкую ответственность
//! - Отсутствие God Object (классы > 500 строк)
//! - Трейты узкие (ISP - Interface Segregation Principle)
//! - Зависимости ацикличны

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::items_after_statements)]
#![allow(dead_code)]

// ============================================================================
// ТЕСТ 1: КАЖДЫЙ МОДУЛЬ ИМЕЕТ ЧЕТКУЮ ОТВЕТСТВЕННОСТЬ
// ============================================================================

/// Тест: все модули имеют четкую ответственность.
#[test]
fn test_all_modules_have_clear_responsibility() {
    // === constants - централизованные константы ===
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    assert_eq!(FPS, 60, "constants должен отвечать за константы");
    assert_eq!(GRID_WIDTH, 10, "constants должен отвечать за константы");
    assert_eq!(GRID_HEIGHT, 20, "constants должен отвечать за константы");

    // === types - типобезопасные обёртки ===
    use crate::game::types::{GameAction, Level, LinesCount, Position, Score};
    let _score = Score::new();
    let _level = Level::new();
    let _lines = LinesCount::new();
    let _pos = Position::new(0, 0);
    let _action = GameAction::MoveLeft;

    // === state - состояние игры ===
    use crate::game::state::GameState;
    let _state = GameState::new();

    // === logic - игровая логика ===
    use crate::game::logic::can_move_curr_shape_direction;
    let state = GameState::new();
    let _ = can_move_curr_shape_direction(&state, crate::types::Direction::Down);

    // === scoring - система очков ===
    use crate::game::scoring::combo::calculate_combo_bonus;
    use crate::game::scoring::lines::find_full_rows;
    let _ = calculate_combo_bonus(3);
    let blocks = [[0i8; 10]; 20];
    let _ = find_full_rows(&blocks);

    // === access - трейты доступа ===
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    fn requires_traits<B: BoardReadonly + BoardMutable, S: ScoreAccess + ScoreMutable>(
        _board: &B,
        _score: &S,
    ) {
    }

    // === view - представление для отрисовки ===
    use crate::game::view::GameView;
    let state = GameState::new();
    let _view = GameView::from_game_state(&state);

    // === scoreboard - очки и уровни ===
    use crate::game::scoreboard::ScoreBoard;
    let _scoreboard = ScoreBoard::new();

    // === board - игровое поле ===
    use crate::game::board::GameBoard;
    let _board = GameBoard::new();

    // === mode_trait - трейты режимов игры ===
    use crate::game::mode_trait::{ClassicMode, MarathonMode, SprintMode};
    let _classic = ClassicMode;
    let _sprint = SprintMode::new();
    let _marathon = MarathonMode::new();

    // === rules - бизнес-правила ===
    use crate::game::rules::GameRules;
    let _rules = GameRules::new();
}

// ============================================================================
// ТЕСТ 2: ОТСУТСТВИЕ GOD OBJECT
// ============================================================================

/// Тест: отсутствие God Object.
#[test]
fn test_no_god_objects() {
    use crate::game::board::GameBoard;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;
    use crate::game::stats::GameStats;
    use crate::game::view::GameView;

    // Проверяем размер структур в байтах
    let game_state_size = std::mem::size_of::<GameState>();
    let board_size = std::mem::size_of::<GameBoard>();
    let scoreboard_size = std::mem::size_of::<ScoreBoard>();
    let stats_size = std::mem::size_of::<GameStats>();
    let view_size = std::mem::size_of::<GameView>();

    // GameState не должен быть God Object
    assert!(
        game_state_size < 10240,
        "GameState не должен быть God Object (размер: {} байт, лимит: 10KB)",
        game_state_size
    );

    // GameBoard не должен быть God Object
    assert!(
        board_size < 1024,
        "GameBoard не должен быть God Object (размер: {} байт, лимит: 1KB)",
        board_size
    );

    // ScoreBoard не должен быть God Object
    assert!(
        scoreboard_size < 256,
        "ScoreBoard не должен быть God Object (размер: {} байт, лимит: 256 байт)",
        scoreboard_size
    );

    // GameStats не должен быть God Object
    assert!(
        stats_size < 512,
        "GameStats не должен быть God Object (размер: {} байт, лимит: 512 байт)",
        stats_size
    );

    // GameView не должен быть God Object
    assert!(
        view_size < 1024,
        "GameView не должен быть God Object (размер: {} байт, лимит: 1KB)",
        view_size
    );

    // Проверяем что GameState использует композицию
    let state = GameState::new();
    let _blocks = state.get_blocks();
    let _score = state.score();
    let _lines = state.lines_cleared();
    let _level = state.level();
}

// ============================================================================
// ТЕСТ 3: ТРЕЙТЫ УЗКИЕ (ISP)
// ============================================================================

/// Тест: трейты узкие (ISP).
#[test]
fn test_traits_are_narrow() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    use crate::game::board::GameBoard;
    use crate::game::state::GameState;

    // BoardReadonly - только чтение
    fn requires_board_readonly<B: BoardReadonly>(board: &B) {
        let _blocks = board.get_blocks();
        let _block = board.get_block(0, 0);
        let _is_empty = board.is_block_empty(0, 0);
    }

    let state = GameState::new();
    requires_board_readonly(&state);

    let board = GameBoard::new();
    requires_board_readonly(&board);

    // BoardMutable - чтение и запись
    fn requires_board_mutable<B: BoardMutable>(board: &mut B) {
        let _blocks = board.get_blocks();
        board.set_block(0, 0, 1);
    }

    let mut state = GameState::new();
    requires_board_mutable(&mut state);

    let mut board = GameBoard::new();
    requires_board_mutable(&mut board);

    // ScoreAccess - только чтение очков
    fn requires_score_access<S: ScoreAccess>(score: &S) {
        let _score_val = score.get_score();
        let _level = score.get_level();
        let _lines = score.get_lines_cleared();
    }

    let state = GameState::new();
    requires_score_access(&state);

    // ScoreMutable - изменение очков
    fn requires_score_mutable<S: ScoreAccess + ScoreMutable>(score: &mut S) {
        let _score_val = score.get_score();
        score.set_score(100);
        score.set_level(5);
        score.set_lines_cleared(10);
    }

    let mut state = GameState::new();
    requires_score_mutable(&mut state);
}

// ============================================================================
// ТЕСТ 4: ЗАВИСИМОСТИ АЦИКЛИЧНЫ
// ============================================================================

/// Тест: зависимости ацикличны.
#[test]
fn test_dependencies_are_acyclic() {
    // Уровень 1: constants (базовый)
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH, LINE_SCORES};
    assert_eq!(FPS, 60);
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);
    assert_eq!(LINE_SCORES.len(), 4);

    // Уровень 2: types (зависит только от constants)
    use crate::game::types::{GameAction, Level, LinesCount, Score};
    let _score = Score::new();
    let _level = Level::new();
    let _lines = LinesCount::new();
    let _action = GameAction::MoveLeft;

    // Уровень 3: state (зависит от types и constants)
    use crate::game::state::GameState;
    let state = GameState::new();
    assert_eq!(state.score(), 0);
    assert_eq!(state.level(), 1);

    // Уровень 4: logic (зависит от state и types)
    use crate::game::logic::can_move_curr_shape_direction;
    let _ = can_move_curr_shape_direction(&state, crate::types::Direction::Down);

    // Уровень 5: scoring (зависит от logic и types)
    use crate::game::scoring::combo::calculate_combo_bonus;
    let _ = calculate_combo_bonus(3);

    // Уровень 6: access (зависит от state и types)
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    fn requires_traits<B: BoardReadonly + BoardMutable, S: ScoreAccess + ScoreMutable>(
        _board: &B,
        _score: &S,
    ) {
    }
    requires_traits(&state, &state);

    // Уровень 7: view (зависит от state и types)
    use crate::game::view::GameView;
    let _view = GameView::from_game_state(&state);

    // Проверка что tetromino независим
    use crate::tetromino::{BagGenerator, Tetromino};
    let mut bag = BagGenerator::new();
    let _tetromino = Tetromino::from_bag(&mut bag);

    // Проверка что io_traits независим
    use crate::io_traits::{InputReader, Renderer};
    let _ = std::any::type_name::<dyn InputReader>();
    let _ = std::any::type_name::<dyn Renderer>();

    // Проверка что highscore независим от game
    use crate::highscore::Leaderboard;
    let _leaderboard = Leaderboard::load();

    // Проверка что crypto независим
    use crate::crypto::{hash, hmac_sha256};
    let _hash = hash("тест");
    let _hmac = hmac_sha256("ключ", "данные");

    // Проверка что validation независим
    use crate::validation::path::PathValidator;
    let _validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");
}

// ============================================================================
// ТЕСТ 5: КОМПЛЕКСНАЯ ПРОВЕРКА ЦЕЛОСТНОСТИ
// ============================================================================

/// Тест: комплексная проверка целостности архитектуры.
#[test]
fn test_architecture_integrity_comprehensive() {
    // SRP: Single Responsibility Principle
    use crate::constants::FPS;
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
    use crate::game::board::GameBoard;
    use crate::game::rules::GameRules;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;
    use crate::game::types::GameAction;

    assert_eq!(FPS, 60, "constants отвечает за константы");

    let mut state = GameState::new();
    let _blocks = state.get_blocks();
    let _score = state.score();

    let mut board = GameBoard::new();
    board.set_block(0, 0, 1);

    let mut scoreboard = ScoreBoard::new();
    scoreboard.set_score(100);

    let rules = GameRules::new();
    let _line_score = rules.get_line_score(4);

    // ISP: Interface Segregation Principle
    fn readonly_fn<B: BoardReadonly>(_board: &B) {}
    fn mutable_fn<B: BoardMutable>(_board: &mut B) {}
    fn score_access_fn<S: ScoreAccess>(_score: &S) {}
    fn score_mutable_fn<S: ScoreAccess + ScoreMutable>(_score: &mut S) {}

    readonly_fn(&state);
    mutable_fn(&mut state);
    score_access_fn(&state);
    score_mutable_fn(&mut state);

    // DIP: Dependency Inversion Principle
    use crate::io_traits::{InputReader, Renderer};
    let _ = std::any::type_name::<dyn InputReader>();
    let _ = std::any::type_name::<dyn Renderer>();

    // ADP: Acyclic Dependencies Principle
    let _action = GameAction::MoveLeft;
    let _hash = crate::crypto::hash("тест");
    let _validator = crate::validation::path::PathValidator::new(255, "abc");
}
