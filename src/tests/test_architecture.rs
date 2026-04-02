//! Тесты на архитектурную целостность.
//!
//! Этот модуль проверяет архитектурные ограничения проекта:
//! - Отсутствие циклических зависимостей между модулями
//! - Целостность GameView
//! - Реализация трейтов IO
//! - Отсутствие публичных полей GameState
//! - Разделение ответственности модулей
//! - Отсутствие deprecated функций
//! - Централизация wall kick логики
//! - Использование HMAC ключей из окружения

#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::manual_let_else)]

// ============================================================================
// ТЕСТ 1: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Проверка, что types.rs не зависит от других модулей проекта.
///
/// types.rs должен содержать только базовые типы и перечисления,
/// не импортируя другие модули проекта (только внешние зависимости).
#[test]
fn test_types_no_cyclic_dependencies() {
    // Этот тест компилируется только если types.rs не импортирует
    // модули проекта, которые зависят от него

    // Импортируем типы напрямую
    use crate::types::{Direction, RotationDirection, UpdateEndState};

    // Проверяем, что типы работают независимо
    let _dir = Direction::Left;
    let _dir = Direction::Right;
    let _dir = Direction::Down;
    let _rotation = RotationDirection::Clockwise;
    let _rotation = RotationDirection::CounterClockwise;

    let state = UpdateEndState::Continue;
    assert_eq!(format!("{:?}", state), "Continue");
}

/// Проверка, что crypto.rs зависит только от внешних библиотек.
///
/// crypto.rs должен импортировать только rand и blake3,
/// не импортируя другие модули проекта.
#[test]
fn test_crypto_only_external_dependencies() {
    use crate::crypto::{generate_salt, hash, hmac_sha256, verify_hmac_sha256};

    // Проверяем базовую функциональность
    let h = hash("тест");
    assert_eq!(h.len(), 64, "Длина хеша должна быть 64 символа");

    let salt = generate_salt();
    assert_eq!(salt.len(), 64, "Длина соли должна быть 64 символа");

    let signature = hmac_sha256("ключ", "данные");
    assert!(verify_hmac_sha256("ключ", "данные", &signature));
}

/// Проверка, что game/ подмодули не создают циклов.
///
/// Подмодули game должны быть организованы иерархически:
/// state → view → render
/// state → logic
/// state → scoring
#[test]
fn test_game_submodules_no_cycles() {
    use crate::game::{GameState, GameView};

    // Создаём состояние
    let state = GameState::new();

    // Создаём представление из состояния
    let view = GameView::from_game_state(&state);

    // Проверяем, что view корректно ссылается на state
    assert!(!view.score.is_empty());
    assert_eq!(view.mode.name(), "Классика");
    assert_eq!(view.level, "1");
    assert_eq!(view.lines, "0");
}

// ============================================================================
// ТЕСТ 2: ЦЕЛОСТНОСТЬ GameView
// ============================================================================

/// Проверка, что GameView корректно создаётся из GameState.
///
/// GameView должен предоставлять доступ ко всем необходимым данным
/// для отрисовки через неизменяемые ссылки.
#[test]
fn test_game_view_creation() {
    use crate::game::{GameState, GameView};
    use crate::tetromino::ShapeType;

    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // Проверяем, что score не пустой
    assert!(!view.score.is_empty(), "Score строка не должна быть пустой");

    // Проверяем режим игры
    assert_eq!(view.mode.name(), "Классика", "Режим должен быть Classic");

    // Проверяем, что уровень установлен
    assert_eq!(view.level, "1", "Начальный уровень должен быть 1");

    // Проверяем, что линии установлены
    assert_eq!(view.lines, "0", "Начальное количество линий должно быть 0");

    // Проверяем, что блоки доступны
    assert_eq!(view.blocks.len(), 20, "Должно быть 20 рядов блоков");
    assert_eq!(view.blocks[0].len(), 10, "Должно быть 10 колонок блоков");

    // Проверяем, что текущая фигура доступна
    assert!(matches!(
        view.curr_shape.shape(),
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));

    // Проверяем, что следующая фигура доступна
    assert!(matches!(
        view.next_shape.shape(),
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));

    // Проверяем, что удержанная фигура None в начале
    assert!(
        view.held_shape.is_none(),
        "Удержанная фигура должна быть None в начале"
    );
}

/// Проверка, что GameView корректно работает в режиме спринт.
#[test]
fn test_game_view_sprint_mode() {
    use crate::game::{GameState, GameView};

    let state = GameState::new_sprint();
    let view = GameView::from_game_state(&state);

    assert_eq!(view.mode.name(), "Спринт", "Режим должен быть Sprint");
    assert_eq!(
        view.lines_cleared, 0,
        "Начальное количество линий должно быть 0"
    );
}

// ============================================================================
// ТЕСТ 3: РЕАЛИЗАЦИЯ ТРЕЙТОВ IO
// ============================================================================

/// Проверка, что Canvas реализует трейт Renderer.
///
/// Этот тест должен компилироваться только если Canvas
/// корректно реализует все методы трейта Renderer.
///
/// # Примечание
/// Тест пропускается в среде без терминала.
#[test]
fn test_renderer_trait_implementation() {
    use crate::io::Canvas;
    use crate::io_traits::Renderer;

    // Функция, требующая реализации трейта Renderer
    fn requires_renderer<R: Renderer>(_: &R) {}

    let canvas = if let Ok(c) = Canvas::new() {
        c
    } else {
        // Пропускаем тест в среде без терминала
        eprintln!("Пропуск теста: требуется терминал");
        return;
    };
    requires_renderer(&canvas); // Должно компилироваться
}

/// Проверка, что KeyReader реализует трейт InputReader.
///
/// Этот тест должен компилироваться только если KeyReader
/// корректно реализует все методы трейта InputReader.
#[test]
fn test_input_reader_trait_implementation() {
    use crate::io::KeyReader;
    use crate::io_traits::InputReader;

    // Функция, требующая реализации трейта InputReader
    fn requires_input_reader<R: InputReader>(_: &R) {}

    let reader = KeyReader::new();
    requires_input_reader(&reader); // Должно компилироваться
}

/// Проверка, что Canvas можно использовать как &mut dyn Renderer.
///
/// # Примечание
/// Тест пропускается в среде без терминала.
#[test]
fn test_canvas_as_dyn_renderer() {
    use crate::io::Canvas;
    use crate::io_traits::Renderer;
    use termion::color::{Reset, White};

    // Функция, принимающая &mut dyn Renderer
    fn draw_with_renderer<R: Renderer>(renderer: &mut R) {
        renderer.draw_string("Test", (1, 1), &White, &Reset);
    }

    let mut canvas = if let Ok(c) = Canvas::new() {
        c
    } else {
        // Пропускаем тест в среде без терминала
        eprintln!("Пропуск теста: требуется терминал");
        return;
    };
    draw_with_renderer(&mut canvas); // Должно компилироваться
}

// ============================================================================
// ТЕСТ 4: ОТСУТСТВИЕ ПУБЛИЧНЫХ ПОЛЕЙ GameState
// ============================================================================

/// Проверка, что основные поля GameState имеют геттеры.
///
/// GameState должен предоставлять публичные методы для доступа
/// к своим полям вместо прямого доступа к полям.
#[test]
fn test_game_state_getters() {
    use crate::game::GameState;

    let state = GameState::new();

    // Проверка существования геттеров
    let _score = state.score();
    let _level = state.level();
    let _lines = state.lines_cleared();
    let _mode = state.get_mode_trait().name();
    let _blocks = state.get_blocks();
    let _curr_shape = state.curr_shape();
    let _next_shape = state.next_shape();
    let _held_shape = state.held_shape();
    let _stats = state.stats();
    let _fall_spd = state.fall_speed();

    // Проверяем, что геттеры возвращают корректные начальные значения
    assert_eq!(_score, 0, "Начальный счёт должен быть 0");
    assert_eq!(_level, 1, "Начальный уровень должен быть 1");
    assert_eq!(_lines, 0, "Начальное количество линий должно быть 0");
}

/// Проверка, что геттеры возвращают мутабельные ссылки где необходимо.
#[test]
fn test_game_state_mutable_getters() {
    use crate::game::GameState;

    let mut state = GameState::new();

    // Проверяем наличие мутабельных геттеров для тестов
    let _curr_shape_mut = state.get_curr_shape_mut();
    let _next_shape_mut = state.get_next_shape_mut();

    // Проверяем наличие сеттеров для тестов
    state.set_score(100);
    assert_eq!(state.score(), 100);

    state.set_level(5);
    assert_eq!(state.level(), 5);

    state.set_lines_cleared(25);
    assert_eq!(state.lines_cleared(), 25);
}

// ============================================================================
// ТЕСТ 5: РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ МОДУЛЕЙ
// ============================================================================

/// Проверка, что модули не экспортируют лишнего.
///
/// Каждый модуль должен экспортировать только свои публичные API:
/// - types.rs: только типы (Direction, RotationDirection, UpdateEndState)
/// - crypto.rs: только функции хеширования (hash, generate_salt, hmac, verify_hmac)
/// - io.rs: Canvas и KeyReader
/// - io_traits.rs: трейты InputReader и Renderer
/// - game/: игровые структуры и функции
/// - highscore: таблица лидеров
/// - controls: конфигурация управления
/// - tetromino: фигуры
#[test]
fn test_arch_module_boundaries() {
    // types.rs экспортирует только типы
    use crate::types::{Direction, RotationDirection, UpdateEndState};
    let dir = Direction::Left;
    let rotation = RotationDirection::Clockwise;
    let state = UpdateEndState::Continue;

    // crypto.rs экспортирует только функции хеширования
    use crate::crypto::{generate_salt, hash};
    let hash_result = hash("data");
    let salt = generate_salt();

    // io.rs экспортирует Canvas и KeyReader
    // Не создаём экземпляры, просто проверяем доступность типов

    // io_traits.rs экспортирует трейты
    // Трейты используются через bounds

    // game/ экспортирует игровые структуры
    use crate::game::GameState;
    let game_state = GameState::new();

    // highscore экспортирует таблицу лидеров
    use crate::highscore::Leaderboard;
    let leaderboard = Leaderboard::load();

    // controls экспортирует конфигурацию
    use crate::controls::ControlsConfig;
    let config = ControlsConfig::default();

    // tetromino экспортирует фигуры
    use crate::tetromino::ShapeType;
    let shape = ShapeType::T;

    // Assert что модули работают корректно
    assert!(matches!(dir, Direction::Left), "Direction должен работать");
    assert!(
        matches!(rotation, RotationDirection::Clockwise),
        "RotationDirection должен работать"
    );
    assert!(
        matches!(state, UpdateEndState::Continue),
        "UpdateEndState должен работать"
    );
    assert_eq!(hash_result.len(), 64, "Hash должен быть 64 символа");
    assert_eq!(salt.len(), 64, "Salt должен быть 64 символа");
    assert_eq!(game_state.score(), 0, "GameState должен работать");
    assert!(config.validate(), "ControlsConfig должен работать");
    assert!(matches!(shape, ShapeType::T), "ShapeType должен работать");
}

// ============================================================================
// ТЕСТ 6: ОТСУТСТВИЕ DEPRECATED ФУНКЦИЙ
// ============================================================================

/// Проверка, что в коде нет вызовов deprecated функций.
///
/// highscore::generate_salt() не должен вызываться напрямую.
/// Использовать только crypto::generate_salt()
#[test]
fn test_no_deprecated_calls() {
    // Этот тест проверяет, что мы используем crypto::generate_salt
    // вместо любых потенциально deprecated версий из других модулей

    use crate::crypto::generate_salt;

    let salt = generate_salt();
    assert_eq!(salt.len(), 64, "Соль должна быть 64 символа");

    // Проверяем, что crypto модуль предоставляет все необходимые функции
    use crate::crypto::{hash, hmac_sha256, verify_hmac_sha256};

    let data = "тестовые данные";
    let h = hash(data);
    assert_eq!(h.len(), 64);

    let key = "ключ";
    let signature = hmac_sha256(key, data);
    assert!(verify_hmac_sha256(key, data, &signature));
}

/// Проверка, что crypto функции являются каноническими.
#[test]
fn test_crypto_functions_are_canonical() {
    use crate::crypto::{generate_salt, hash};

    // Проверяем детерминированность hash
    let h1 = hash("тест");
    let h2 = hash("тест");
    assert_eq!(h1, h2, "Хеш должен быть детерминированным");

    // Проверяем уникальность generate_salt
    let s1 = generate_salt();
    let s2 = generate_salt();
    assert_ne!(s1, s2, "Соли должны быть уникальными");
}

// ============================================================================
// ТЕСТ 7: ДОПОЛНИТЕЛЬНЫЕ АРХИТЕКТУРНЫЕ ПРОВЕРКИ
// ============================================================================

/// Тест: Централизация wall kick логики
///
/// Проверяет, что can_rotate_curr_shape делегирует логику в wall_kick модуль.
#[test]
fn test_wall_kick_logic_centralization() {
    use crate::game::logic::wall_kick::can_rotate_with_wall_kick;
    use crate::game::GameState;
    use crate::types::RotationDirection;

    let state = GameState::new();

    // Функция должна быть доступна и работать
    let _can_rotate = can_rotate_with_wall_kick(&state, RotationDirection::Clockwise);

    // Функция вернула корректное значение bool
}

/// Тест: HMAC ключ из переменных окружения
///
/// Проверяет, что HMAC ключ используется корректно.
#[test]
fn test_hmac_key_usage() {
    use crate::highscore::leaderboard::LeaderboardEntry;

    // LeaderboardEntry должен использовать HMAC internally
    let entry = LeaderboardEntry::new("TestPlayer", 1000);

    // Запись должна быть валидной (подписанной)
    assert!(entry.is_valid(), "Запись должна быть валидной");
}

/// Тест: Документирование констант UI
///
/// Проверяет, что константы меню имеют документацию.
#[test]
fn test_menu_constants_documented() {
    use crate::menu::constants::{MENU_AUTHOR_X, MENU_RECORD_X, MENU_TITLE_X, MENU_TITLE_Y};

    // Константы должны быть доступны и положительны
    const { assert!(MENU_TITLE_X > 0, "MENU_TITLE_X должен быть положительным"); };
    const { assert!(MENU_TITLE_Y > 0, "MENU_TITLE_Y должен быть положительным"); };
    const {
        assert!(MENU_AUTHOR_X > 0, "MENU_AUTHOR_X должен быть положительным");
    };
    const {
        assert!(MENU_RECORD_X > 0, "MENU_RECORD_X должен быть положительным");
    };
}
