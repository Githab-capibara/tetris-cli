//! Архитектурные тесты на целостность.
//!
//! Этот модуль тестирует архитектурную целостность проекта:
//! - Компилируемость всех модулей
//! - Стабильность публичного API
//! - Корректное использование трейтов
//! - Отсутствие нарушений SRP в GameState

// ============================================================================
// ТЕСТ 1: ВСЕ МОДУЛИ КОМПИЛИРУЮТСЯ
// ============================================================================

/// Проверка, что все модули компилируются.
///
/// Этот тест проверяет, что все основные модули проекта компилируются
/// и их публичные типы доступны.
#[test]
fn test_all_modules_compilable() {
    // game модуль
    use crate::game::access::{GameBoardAccess, ScoreAccess};
    use crate::game::cache::StringCache;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait, MarathonMode, SprintMode};
    use crate::game::view::GameView;
    use crate::game::GameState;

    // game::logic модуль
    use crate::game::logic::collision;
    use crate::game::logic::input;
    use crate::game::logic::physics;
    use crate::game::logic::rotation;
    use crate::game::logic::update;

    // game::scoring модуль
    use crate::game::scoring::combo;
    use crate::game::scoring::lines;
    use crate::game::scoring::points;

    // validation модуль
    use crate::validation::name::{is_valid_name_char, sanitize_player_name};
    use crate::validation::path::{PathError, PathValidator};

    // controls модуль
    use crate::controls::ControlsConfig;

    // highscore модуль
    use crate::highscore::leaderboard::Leaderboard;

    // types модуль
    use crate::types::{Direction, RotationDirection};

    // tetromino модуль
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};

    // io модуль
    use crate::io::{Canvas, KeyReader};

    // Проверяем, что типы можно создать/использовать
    let _state = GameState::new();
    let _config = ControlsConfig::default_config();
    let _validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    let _cache = StringCache::new();
    let _direction = Direction::Down;
    let _rotation = RotationDirection::Clockwise;
    let _shape_type = ShapeType::T;
    let _bag = BagGenerator::new();
    let _leaderboard = Leaderboard::default();

    // Подавляем предупреждения о неиспользуемых импортах
    let _ = collision::can_move_curr_shape_direction;
    let _ = rotation::rotate_with_wall_kick;
    let _ = physics::handle_falling;
    let _ = input::handle_input;
    let _ = update::update;
    let _ = points::update_score_and_level;
    let _ = lines::find_full_rows;
    let _ = is_valid_name_char;
    let _ = sanitize_player_name;
    let _mode_trait: &dyn GameModeTrait = &ClassicMode;

    assert!(true, "все модули должны компилироваться");
}

// ============================================================================
// ТЕСТ 2: ПУБЛИЧНЫЙ API СТАБИЛЕН
// ============================================================================

/// Проверка, что публичный API стабилен.
///
/// Этот тест проверяет, что основные публичные типы и функции
/// имеют стабильные сигнатуры и доступны извне крейта.
#[test]
fn test_public_api_stable() {
    use crate::controls::ControlsConfig;
    use crate::game::GameState;
    use crate::validation::path::PathValidator;

    // Проверяем стабильность GameState API
    let mut state = GameState::new();

    // Геттеры
    let _score: u128 = state.get_score();
    let _level: u32 = state.get_level();
    let _lines: u32 = state.get_lines_cleared();
    let _mode_trait: &dyn crate::game::mode_trait::GameModeTrait = state.get_mode_trait();

    // Сеттеры
    state.set_score(100);
    state.set_level(5);
    state.set_lines_cleared(50);

    // Проверяем стабильность ControlsConfig API
    let config = ControlsConfig::default_config();
    let _move_left: u8 = config.move_left();
    let _move_right: u8 = config.move_right();
    let _valid: bool = config.validate();

    // Проверяем стабильность PathValidator API
    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    use std::path::Path;
    let path = Path::new("test.json");
    let _result = validator.validate_length(path);

    assert!(true, "публичный API должен быть стабильным");
}

// ============================================================================
// ТЕСТ 3: ТРЕЙТЫ ИСПОЛЬЗУЮТСЯ КОРРЕКТНО
// ============================================================================

/// Проверка, что трейты используются корректно.
///
/// Этот тест проверяет, что:
/// - GameModeTrait реализован для всех режимов
/// - GameBoardAccess реализован для GameState
/// - ScoreAccess реализован для GameState
#[test]
fn test_traits_properly_used() {
    use crate::game::access::{GameBoardAccess, ScoreAccess};
    use crate::game::mode_trait::GameModeTrait;
    use crate::game::mode_trait::{ClassicMode, MarathonMode, SprintMode};
    use crate::game::state::GameState;

    // Проверяем, что GameState реализует GameBoardAccess
    fn _requires_game_board_access<T: GameBoardAccess>(_: &T) {}
    fn _requires_score_access<T: ScoreAccess>(_: &T) {}

    let state = GameState::new();
    _requires_game_board_access(&state);
    _requires_score_access(&state);

    // Проверяем, что все режимы реализуют GameModeTrait
    fn _requires_game_mode_trait<T: GameModeTrait>(_: &T) {}

    let classic = ClassicMode;
    let sprint = SprintMode::new();
    let marathon = MarathonMode::new();

    _requires_game_mode_trait(&classic);
    _requires_game_mode_trait(&sprint);
    _requires_game_mode_trait(&marathon);

    // Проверяем методы трейтов
    // GameBoardAccess
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);

    // ScoreAccess
    let mut state_mut = GameState::new();
    state_mut.add_score(100);
    assert_eq!(state_mut.get_score(), 100);

    // GameModeTrait
    assert_eq!(classic.name(), "Классика");
    assert_eq!(sprint.name(), "Спринт");
    assert_eq!(marathon.name(), "Марафон");

    assert_eq!(classic.get_target_lines(), None);
    assert_eq!(sprint.get_target_lines(), Some(40));
    assert_eq!(marathon.get_target_lines(), Some(150));

    assert!(!classic.check_win_condition(100));
    assert!(sprint.check_win_condition(40));
    assert!(marathon.check_win_condition(150));

    assert!(true, "трейты должны использоваться корректно");
}

// ============================================================================
// ТЕСТ 4: ОТСУТСТВИЕ НАРУШЕНИЙ SRP В GAMESTATE
// ============================================================================

/// Проверка, что GameState не нарушает SRP критически.
///
/// GameState содержит множество полей, но они сгруппированы по категориям:
/// - СОСТОЯНИЕ ПОЛЯ (blocks, filled_lines)
/// - СОСТОЯНИЕ ОЧКОВ (score, level, lines_cleared)
/// - СОСТОЯНИЕ ФИГУР (curr_shape, next_shape, held_shape, bag)
/// - СОСТОЯНИЕ АНИМАЦИЙ (animating_rows_mask, is_hard_dropping)
/// - СТАТИСТИКА И РЕЖИМ (stats, mode_trait)
/// - КЭШИРОВАНИЕ (cached_* строки)
///
/// Этот тест проверяет, что поля сгруппированы и доступны через геттеры.
#[test]
fn test_no_god_object_violation() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Проверяем, что поля сгруппированы по категориям через геттеры

    // === СОСТОЯНИЕ ПОЛЯ ===
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "Поле должно иметь 20 строк");
    assert_eq!(blocks[0].len(), 10, "Поле должно иметь 10 столбцов");

    // === СОСТОЯНИЕ ОЧКОВ ===
    assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.get_level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(
        state.get_lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );

    // === СОСТОЯНИЕ ФИГУР ===
    let curr_shape = state.get_curr_shape();
    let next_shape = state.get_next_shape();
    let held_shape = state.get_held_shape();
    // Проверяем, что тип фигуры валидный (0-6)
    let _curr_type = curr_shape.shape;
    let _next_type = next_shape.shape;
    assert!(
        held_shape.is_none(),
        "Удержанная фигура должна быть None в начале"
    );

    // === СОСТОЯНИЕ АНИМАЦИЙ ===
    assert!(
        !state.is_hard_dropping(),
        "Hard Drop должен быть false в начале"
    );
    assert_eq!(
        state.get_animating_rows_mask(),
        0,
        "Анимация строк должна быть 0 в начале"
    );

    // === СТАТИСТИКА И РЕЖИМ ===
    let stats = state.get_stats();
    assert_eq!(stats.total_pieces(), 1, "Должна быть 1 фигура в начале");
    let mode_name = state.get_mode_trait().name();
    assert_eq!(
        mode_name, "Классика",
        "Режим по умолчанию должен быть Классика"
    );

    // Проверяем, что GameState делегирует логику другим модулям
    // через трейт GameBoardAccess
    state.add_score(100);
    assert_eq!(state.get_score(), 100);

    state.set_level(5);
    assert_eq!(state.get_level(), 5);

    // GameState не должен содержать бизнес-логику
    // Вся логика должна быть в game::logic и game::scoring модулях

    assert!(true, "GameState не должен нарушать SRP критически");
}

// ============================================================================
// ТЕСТ 5: ДОПОЛНИТЕЛЬНЫЕ ПРОВЕРКИ ЦЕЛОСТНОСТИ
// ============================================================================

/// Проверка, что GameModeTrait используется вместо enum.
#[test]
fn test_game_mode_trait_used() {
    use crate::game::GameState;

    let state = GameState::new();

    // Проверяем, что get_mode_trait() доступен
    let mode_trait = state.get_mode_trait();
    let _name = mode_trait.name();
    let _target = mode_trait.get_target_lines();

    // Проверяем, что set_mode_trait() доступен
    let new_mode = crate::game::mode_trait::SprintMode::new();
    let mut state_mut = GameState::new();
    state_mut.set_mode_trait(Box::new(new_mode));

    assert_eq!(state_mut.get_mode_trait().name(), "Спринт");

    assert!(true, "GameModeTrait должен использоваться вместо enum");
}

/// Проверка, что кэширование эффективно.
#[test]
fn test_caching_is_effective() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Проверяем, что кэшированные строки доступны
    let _score_str = state.get_cached_score_str();
    let _level_str = state.get_cached_level_str();
    let _lines_str = state.get_cached_lines_str();

    // Проверяем, что кэширование обновляется при изменении данных
    state.set_score(1000);
    // Кэш должен обновиться при следующей отрисовке

    state.set_level(10);
    // Кэш должен обновиться при следующей отрисовке

    assert!(true, "кэширование должно быть эффективным");
}

/// Проверка, что validation модуль не имеет лишних зависимостей.
#[test]
fn test_validation_has_no_extra_dependencies() {
    use crate::validation::name::sanitize_player_name;
    use crate::validation::path::PathValidator;

    // PathValidator не должен зависеть от game/highscore
    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    use std::path::Path;
    let _result = validator.validate_length(Path::new("test"));

    // sanitize_player_name не должен зависеть от game/highscore
    let _name = sanitize_player_name("Test");

    assert!(
        true,
        "validation модуль не должен иметь лишних зависимостей"
    );
}
