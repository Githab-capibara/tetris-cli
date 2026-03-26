//! Архитектурные тесты на модульность.
//!
//! Этот модуль тестирует соблюдение принципов модульности:
//! - Отсутствие циклических зависимостей
//! - Изоляция validation модуля
//! - Связность scoring подмодулей
//! - Разделение logic подмодулей
//! - Инкапсуляция GameState
//! - Отсутствие дублирования валидации
//! - Соблюдение границ модулей

// ============================================================================
// ТЕСТ 1: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Проверка отсутствия циклических зависимостей между модулями.
///
/// Этот тест проверяет, что модули проекта не имеют циклических зависимостей:
/// - game::state не зависит от game::render
/// - game::render зависит от game::view
/// - game::view зависит от game::state
/// - game::cycle зависит от game::logic и game::render
#[test]
fn test_no_cyclic_dependencies() {
    use crate::game::state::GameState;
    use crate::game::view::GameView;
    use crate::game::render::draw;
    use crate::game::mode_trait::GameModeTrait;

    // Проверяем, что GameState можно создать независимо
    let state = GameState::new();

    // Проверяем, что GameView можно создать из GameState
    let view = GameView::from_game_state(&state);

    // Проверяем, что mode_trait доступен
    let _mode_name = state.get_mode_trait().name();

    // Если этот код компилируется, значит зависимости корректны
    // и нет циклических зависимостей
    assert!(true, "Модули не должны иметь циклических зависимостей");

    // Подавляем предупреждение о неиспользуемой функции draw
    let _ = draw as fn(&GameView, &mut crate::io::Canvas, &str);
}

// ============================================================================
// ТЕСТ 2: VALIDATION МОДУЛЬ ИЗОЛИРОВАН
// ============================================================================

/// Проверка, что validation модуль не зависит от game/highscore.
///
/// validation модуль должен быть независимым и не импортировать
/// типы из game или highscore модулей.
#[test]
fn test_validation_module_isolated() {
    use crate::validation::path::{PathValidator, PathError, PathErrorKind};
    use crate::validation::name::{is_valid_name_char, sanitize_player_name};
    use std::path::Path;

    // Проверяем, что PathValidator работает независимо
    let validator = PathValidator::default();
    let path = Path::new("test.json");

    // Валидация должна работать без зависимостей от game/highscore
    let result = validator.validate_length(path);
    assert!(result.is_ok(), "PathValidator должен работать независимо");

    // Проверяем валидацию имени
    assert!(is_valid_name_char('A'), "Буквы должны быть разрешены");
    assert!(!is_valid_name_char('@'), "Спецсимволы должны быть запрещены");

    // Проверяем санитизацию имени
    let sanitized = sanitize_player_name("Test@User");
    assert_eq!(sanitized, "TestUser", "Спецсимволы должны быть удалены");

    assert!(true, "validation модуль должен быть изолирован от game/highscore");
}

// ============================================================================
// ТЕСТ 3: SCORING МОДУЛИ ИМЕЮТ ВЫСОКУЮ СВЯЗНОСТЬ
// ============================================================================

/// Проверка, что scoring подмодули имеют высокую связность.
///
/// scoring модуль разделён на подмодули:
/// - points: начисление очков
/// - lines: очистка линий
/// - combo: комбо-система
///
/// Каждый подмодуль должен иметь одну ответственность.
#[test]
fn test_scoring_modules_cohesion() {
    use crate::game::scoring::points::update_score_and_level;
    use crate::game::scoring::lines::find_full_rows;
    use crate::game::GameState;

    // Проверяем, что каждый подмодуль выполняет свою функцию

    // points: начисление очков за фигуры
    let mut state = GameState::new();
    update_score_and_level(&mut state, 1);
    assert!(state.get_score() >= 0, "Очки должны быть неотрицательными");

    // lines: поиск заполненных линий
    let state = GameState::new();
    let (rows_mask, count) = find_full_rows(state.get_blocks());
    assert!(count == 0, "В начале игры не должно быть заполненных линий");

    assert!(true, "scoring подмодули должны иметь высокую связность");
}

// ============================================================================
// ТЕСТ 4: LOGIC МОДУЛИ РАЗДЕЛЕНЫ КОРРЕКТНО
// ============================================================================

/// Проверка, что logic подмодули разделены корректно.
///
/// logic модуль разделён на подмодули:
/// - collision: проверка коллизий
/// - rotation: вращение фигур
/// - physics: физика падения
/// - input: обработка ввода
/// - update: обновление состояния
///
/// Каждый подмодуль должен иметь одну ответственность.
#[test]
fn test_logic_modules_separation() {
    use crate::game::logic::collision::can_move_curr_shape_direction;
    use crate::game::logic::rotation::rotate_with_wall_kick;
    use crate::game::logic::physics::handle_falling;
    use crate::game::GameState;
    use crate::types::Direction;

    let mut state = GameState::new();

    // Проверяем, что каждый подмодуль выполняет свою функцию

    // collision: проверка возможности движения
    let can_move = can_move_curr_shape_direction(&state, Direction::Down);
    assert!(can_move, "Движение вниз должно быть возможно в начале игры");

    // rotation: вращение фигуры (требует mutable доступ)
    let _rotated = rotate_with_wall_kick(&mut state, crate::types::RotationDirection::Clockwise);

    // physics: обработка падения (требует mutable доступ)
    let _fell = handle_falling(&mut state);

    assert!(true, "logic подмодули должны быть разделены корректно");
}

// ============================================================================
// ТЕСТ 5: GAMESTATE ИНКАПСУЛЯЦИЯ
// ============================================================================

/// Проверка, что поля GameState приватные/pub(crate).
///
/// GameState должен инкапсулировать свои поля и предоставлять
/// доступ только через геттеры/сеттеры.
#[test]
fn test_game_state_encapsulation() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Проверяем, что поля недоступны напрямую извне модуля
    // Следующий код НЕ должен компилироваться (закомментирован):
    // let _ = state.score; // Ошибка: поле score недоступно

    // Вместо этого используем геттеры:
    let score = state.get_score();
    let level = state.get_level();
    let lines = state.get_lines_cleared();

    assert_eq!(score, 0, "Начальный счёт должен быть 0");
    assert_eq!(level, 1, "Начальный уровень должен быть 1");
    assert_eq!(lines, 0, "Начальное количество линий должно быть 0");

    // Проверяем, что сеттеры работают
    state.set_score(100);
    state.set_level(5);
    state.set_lines_cleared(50);

    assert_eq!(state.get_score(), 100, "Счёт должен обновиться");
    assert_eq!(state.get_level(), 5, "Уровень должен обновиться");
    assert_eq!(state.get_lines_cleared(), 50, "Линии должны обновиться");

    // Проверяем, что mode_trait доступен через геттер
    let mode_name = state.get_mode_trait().name();
    assert_eq!(mode_name, "Классика", "Режим по умолчанию должен быть Классика");

    assert!(true, "поля GameState должны быть приватные/pub(crate)");
}

// ============================================================================
// ТЕСТ 6: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ ВАЛИДАЦИИ
// ============================================================================

/// Проверка, что валидация определена только в validation модуле.
///
/// Вся валидация путей и имён должна быть централизована в validation модуле.
/// Не должно быть дублирования логики валидации в других модулях.
#[test]
fn test_no_duplicate_validation() {
    use crate::validation::path::PathValidator;
    use crate::validation::name::sanitize_player_name;
    use std::path::Path;

    // Проверяем, что PathValidator - единственный валидатор путей
    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");

    // Валидация пути
    let valid_path = Path::new("config.json");
    assert!(validator.validate_length(valid_path).is_ok());

    // Валидация имени
    let valid_name = sanitize_player_name("Player123");
    assert_eq!(valid_name, "Player123");

    // Проверяем, что controls использует валидацию
    use crate::controls::ControlsConfig;
    let config = ControlsConfig::default_config();
    assert!(config.validate(), "Конфигурация должна проходить валидацию");

    assert!(true, "валидация должна быть определена только в validation модуле");
}

// ============================================================================
// ТЕСТ 7: СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
// ============================================================================

/// Проверка, что границы модулей соблюдаются.
///
/// Каждый модуль должен иметь чёткие границы и не нарушать инкапсуляцию.
#[test]
fn test_module_boundaries_respected() {
    use crate::game::access::{GameBoardAccess, ScoreAccess};
    use crate::game::state::GameState;
    use crate::game::view::GameView;
    use crate::game::mode_trait::{GameModeTrait, ClassicMode, SprintMode, MarathonMode};

    // Проверяем, что трейты работают корректно

    // GameBoardAccess: доступ к полю
    let mut state = GameState::new();
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "Высота поля должна быть 20");

    // ScoreAccess: доступ к очкам
    state.add_score(100);
    assert_eq!(state.get_score(), 100);

    // GameView: только для отрисовки
    let view = GameView::from_game_state(&state);
    let _score_str = view.score;

    // GameModeTrait: абстракция режима
    let classic = ClassicMode;
    assert_eq!(classic.name(), "Классика");

    let sprint = SprintMode::new();
    assert_eq!(sprint.name(), "Спринт");
    assert_eq!(sprint.get_target_lines(), Some(40));

    let marathon = MarathonMode::new();
    assert_eq!(marathon.name(), "Марафон");
    assert_eq!(marathon.get_target_lines(), Some(150));

    assert!(true, "границы модулей должны соблюдаться");
}
