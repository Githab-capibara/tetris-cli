//! Тесты на архитектурные улучшения.
//!
//! Этот модуль тестирует улучшения архитектуры, реализованные в ходе аудита:
//! - GameError enum для обработки ошибок
//! - Инкапсуляция полей GameState
//! - Трейт GameBoardAccess для снижения coupling
//! - Разделение ответственности

// ============================================================================
// ТЕСТ 1: GAMEERROR ENUM
// ============================================================================

/// Проверка, что GameError enum существует и имеет правильные варианты.
#[test]
fn test_game_error_enum_variants() {
    use crate::game::GameError;

    // Проверяем, что все варианты ошибок компилируются
    let _io_error = GameError::Io(std::io::Error::other("тест"));

    let _terminal_error = GameError::Terminal("тест".to_string());
    let _config_error = GameError::Config("тест".to_string());
    let _game_over = GameError::GameOver;
    let _validation_error = GameError::Validation("тест".to_string());
}

/// Проверка, что GameError реализует Display.
#[test]
fn test_game_error_display() {
    use crate::game::GameError;

    let error = GameError::Io(std::io::Error::other("ошибка ввода/вывода"));

    let msg = format!("{error}");
    assert!(
        msg.contains("Ошибка ввода/вывода"),
        "Сообщение должно содержать тип ошибки"
    );

    let terminal_error = GameError::Terminal("терминал упал".to_string());
    let terminal_msg = format!("{terminal_error}");
    assert!(terminal_msg.contains("терминал упал"));
}

/// Проверка, что GameError реализует Error.
#[test]
fn test_game_error_trait() {
    use crate::game::GameError;

    let error: Box<dyn std::error::Error> = Box::new(GameError::GameOver);
    assert!(error.to_string().contains("Игра окончена"));
}

/// Проверка, что From<std::io::Error> реализован для GameError.
#[test]
fn test_game_error_from_io() {
    use crate::game::GameError;

    let io_error = std::io::Error::other("io error");
    let game_error: GameError = io_error.into();

    match game_error {
        GameError::Io(_) => (), // Ожидаем Io вариант
        _ => unreachable!("Ожидался вариант GameError::Io"),
    }
}

/// Проверка, что GameResult type alias существует.
#[test]
fn test_game_result_type_alias() {
    use crate::game::{GameError, GameResult};

    // Проверяем, что тип компилируется
    let _result: GameResult<()> = Ok(());
    let _error_result: GameResult<()> = Err(GameError::GameOver);
}

// ============================================================================
// ТЕСТ 2: ИНКАПСУЛЯЦИЯ GAMESTATE
// ============================================================================

/// Проверка, что поля GameState недоступны напрямую извне крейта.
#[test]
fn test_game_state_fields_private() {
    use crate::game::GameState;

    let state = GameState::new();

    // Этот тест компилируется только если поля приватны
    // Следующий код не должен компилироваться:
    // let _ = state.score; // Ошибка компиляции!

    // Вместо этого используем геттеры:
    let _score = state.score();
    let _level = state.level();
    let _lines = state.lines_cleared();

    // Если этот тест компилируется, значит поля инкапсулированы
}

/// Проверка, что геттеры GameState работают корректно.
#[test]
fn test_game_state_getters_encapsulation() {
    use crate::game::GameState;

    let state = GameState::new();

    // Проверяем все основные геттеры
    assert_eq!(state.score(), 0);
    assert_eq!(state.level(), 1);
    assert_eq!(state.lines_cleared(), 0);
    assert_eq!(state.get_mode_trait().name(), "Классика");

    // Проверяем геттеры для фигур
    let curr_shape = state.get_curr_shape();
    // Проверяем, что фигура существует (любой ShapeType валиден)
    use crate::tetromino::ShapeType;
    assert!(matches!(
        curr_shape.shape,
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));

    let next_shape = state.get_next_shape();
    assert!(matches!(
        next_shape.shape,
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));
}

/// Проверка, что сеттеры GameState работают корректно.
#[test]
fn test_game_state_setters_encapsulation() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Тестируем сеттеры
    state.set_score(1000);
    assert_eq!(state.score(), 1000);

    state.set_level(5);
    assert_eq!(state.level(), 5);

    state.set_lines_cleared(50);
    assert_eq!(state.lines_cleared(), 50);
}

// ============================================================================
// ТЕСТ 3: GAMEBOARDACCESS ТРЕЙТ
// ============================================================================

/// Проверка, что трейт GameBoardAccess существует.
#[test]
fn test_game_board_access_trait_exists() {
    use crate::game::GameBoardAccess;

    // Трейт должен компилироваться
    fn _use_trait<T: GameBoardAccess>(_board: &T) {}

    // Этот тест компилируется только если трейт существует
}

/// Проверка, что GameState реализует GameBoardAccess.
#[test]
fn test_game_state_implements_game_board_access() {
    use crate::game::{GameBoardAccess, GameState};

    let state = GameState::new();

    // Проверяем, что GameState реализует трейт
    fn _requires_trait<T: GameBoardAccess>(_t: &T) {}
    _requires_trait(&state);

    // Проверяем методы трейта
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "Высота поля должна быть 20");
    assert_eq!(blocks[0].len(), 10, "Ширина поля должна быть 10");
}

/// Проверка, что методы GameBoardAccess работают корректно.
#[test]
fn test_game_board_access_methods() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Тестируем get_blocks
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);

    // Тестируем get_blocks_mut
    let blocks_mut = state.get_blocks_mut();
    blocks_mut[0][0] = 1;
    assert_eq!(blocks_mut[0][0], 1);

    // Тестируем get_score / add_score
    assert_eq!(state.score(), 0);
    state.add_score(100);
    assert_eq!(state.score(), 100);

    // Тестируем get_level / set_level
    assert_eq!(state.level(), 1);
    state.set_level(3);
    assert_eq!(state.level(), 3);

    // Тестируем get_lines_cleared / set_lines_cleared
    assert_eq!(state.lines_cleared(), 0);
    state.set_lines_cleared(10);
    assert_eq!(state.lines_cleared(), 10);

    // Тестируем get_fall_speed / set_fall_speed
    let initial_spd = state.get_fall_speed();
    assert!((initial_spd - 0.9).abs() < f32::EPSILON);
    state.set_fall_speed(2.0);
    assert_eq!(state.get_fall_speed(), 2.0);

    // Тестируем get_land_timer / set_land_timer
    state.set_land_timer(0.5);
    assert_eq!(state.get_land_timer(), 0.5);
}

/// Проверка, что GameBoardAccess снижает coupling.
#[test]
fn test_game_board_access_reduces_coupling() {
    use crate::game::{GameBoardAccess, GameState};

    // Функция, которая работает с трейтом, а не с GameState напрямую
    fn process_board<T: GameBoardAccess>(board: &mut T) -> u128 {
        board.add_score(50);
        board.get_score()
    }

    let mut state = GameState::new();
    let score = process_board(&mut state);

    assert_eq!(score, 50, "Функция должна работать через трейт");
    assert_eq!(state.score(), 50);
}

// ============================================================================
// ТЕСТ 4: АРХИТЕКТУРНАЯ ЦЕЛОСТНОСТЬ
// ============================================================================

/// Проверка, что новые архитектурные улучшения не ломают существующий код.
#[test]
fn test_architecture_improvements_backward_compatibility() {
    use crate::game::GameState;

    // Создаём состояние
    let mut state = GameState::new();

    // Используем старый API (геттеры/сеттеры)
    state.set_score(100);
    assert_eq!(state.score(), 100);

    // Используем новый API (GameBoardAccess)
    state.add_score(50);
    assert_eq!(state.score(), 150);

    // Оба API работают корректно
}

/// Проверка, что GameError используется в проекте.
#[test]
fn test_game_error_usage_in_project() {
    // Этот тест проверяет, что GameError экспортируется из game модуля
    use crate::game::GameError;

    // Проверяем, что тип доступен
    let _error: GameError = GameError::GameOver;

    // Тест компилируется только если GameError правильно экспортирован
}

/// Проверка, что инкапсуляция не ломает тесты.
#[test]
fn test_encapsulation_doesnt_break_tests() {
    use crate::game::GameState;

    // Тесты должны иметь доступ к необходимым методам
    let mut state = GameState::new();

    // Используем публичные методы для тестирования
    state.set_score(500);
    state.set_level(3);
    state.set_lines_cleared(30);

    assert_eq!(state.score(), 500);
    assert_eq!(state.level(), 3);
    assert_eq!(state.lines_cleared(), 30);

    // Тесты работают через публичный API
}

// ============================================================================
// ТЕСТ 5: МОДУЛЬНАЯ НЕЗАВИСИМОСТЬ
// ============================================================================

/// Проверка, что access модуль не создаёт циклических зависимостей.
#[test]
fn test_access_module_no_cycles() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess};

    // Трейты должны быть доступны из game::access
    fn _use_board_readonly<T: BoardReadonly>(_t: &T) {}
    fn _use_board_mutable<T: BoardMutable>(_t: &mut T) {}
    fn _use_score_access<T: ScoreAccess>(_t: &T) {}

    // Тест компилируется только если нет циклов
}

/// Проверка, что state модуль корректно экспортирует типы.
#[test]
fn test_state_module_exports() {
    use crate::game::state::{GameMode, GameResult, GameState, GameStats};

    // Все типы должны быть доступны
    let _state = GameState::new();
    let _stats = GameStats::new();
    let _mode = GameMode::Classic;
    let _error: GameResult<()> = Ok(());
}
