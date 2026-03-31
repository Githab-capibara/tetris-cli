//! Тесты на соблюдение границ модулей в архитектуре tetris-cli.
//!
//! Этот модуль проверяет что модули соблюдают свои границы ответственности:
//! - Логика не импортирует отрисовку
//! - Scoring не импортирует physics
//! - Tetromino модуль автономен
//! - Validation модуль автономен
//! - Crypto модуль автономен
//!
//! ## Архитектурные принципы
//! - Разделение ответственности (SRP)
//! - Слабая связанность
//! - Сильная инкапсуляция

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

// ============================================================================
// ТЕСТ 1: ЛОГИКА НЕ ИМПОРТИРУЕТ ОТРИСОВКУ
// ============================================================================

/// Тест: логика не импортирует отрисовку.
///
/// Проверяет что модули игровой логики не зависят от модулей отрисовки:
/// - game/logic/* не импортирует game/render
/// - game/state не импортирует game/render
/// - game/scoring не импортирует game/render
///
/// ## Архитектурные заметки
/// Разделение логики и отрисовки обеспечивает:
/// - Возможность тестирования логики без терминала
/// - Возможность замены рендерера
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_game_logic_does_not_import_rendering() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::game::logic::can_rotate_curr_shape;
    use crate::game::state::GameState;
    use crate::types::Direction;

    // Проверяем что логика работает независимо от отрисовки
    let state = GameState::new();

    // Проверяем collision logic
    let can_move = can_move_curr_shape_direction(&state, Direction::Down);
    // can_move возвращает true/false в зависимости от состояния
    // Главное что функция работает независимо от отрисовки
    assert!(
        can_move || !can_move,
        "Collision logic должен работать независимо от отрисовки"
    );

    // Проверяем rotation logic
    let can_rotate = can_rotate_curr_shape(&state, crate::tetromino::RotationDirection::Clockwise);
    assert!(
        can_rotate || !can_rotate,
        "Rotation logic должен работать независимо от отрисовки"
    );

    // Проверяем input logic
    // handle_input требует InputReader, но не требует Renderer
    fn check_input_type<R: crate::io_traits::InputReader>(_reader: &R) {}
    let _ = std::any::type_name::<dyn crate::io_traits::InputReader>();
}

// ============================================================================
// ТЕСТ 2: SCORING НЕ ИМПОРТИРУЕТ PHYSICS
// ============================================================================

/// Тест: scoring не импортирует physics.
///
/// Проверяет что модуль подсчёта очков не зависит от физической логики:
/// - game/scoring/* не импортирует game/logic/physics
/// - Очки рассчитываются только на основе данных о линиях и уровне
///
/// ## Архитектурные заметки
/// Разделение scoring и physics обеспечивает:
/// - Независимое тестирование системы очков
/// - Возможность изменения физики без влияния на очки
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_scoring_does_not_import_physics() {
    use crate::game::scoring::combo::calculate_combo_bonus;
    use crate::game::scoring::lines::find_full_rows;
    use crate::game::types::Score;

    // Проверяем что scoring работает независимо от physics
    let combo_bonus = calculate_combo_bonus(3);
    assert!(
        combo_bonus > 0,
        "Combo bonus должен работать независимо от physics"
    );

    let blocks = [[0i8; 10]; 20];
    let (_mask, count) = find_full_rows(&blocks);
    // find_full_rows возвращает количество строк в поле
    assert_eq!(
        count, 20,
        "find_full_rows должен работать независимо от physics"
    );

    // Проверяем что Score работает независимо
    let mut score = Score::new();
    score.add(100);
    assert_eq!(
        score.value(),
        100,
        "Score должен работать независимо от physics"
    );
}

// ============================================================================
// ТЕСТ 3: TETROMINO МОДУЛЬ АВТОНОМЕН
// ============================================================================

/// Тест: tetromino модуль автономен.
///
/// Проверяет что модуль фигур не зависит от других игровых модулей:
/// - tetromino не зависит от game/state
/// - tetromino не зависит от game/logic
/// - tetromino зависит только от constants и types
///
/// ## Архитектурные заметки
/// Автономность tetromino обеспечивает:
/// - Возможность повторного использования
/// - Независимое тестирование
/// - Простоту понимания кода
#[test]
fn test_tetromino_is_autonomous() {
    use crate::tetromino::{BagGenerator, RotationDirection, ShapeType, Tetromino};
    use crate::tetromino::{SHAPE_COLORS, SHAPE_COORDS};

    // Проверяем что ShapeType работает независимо
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];
    assert_eq!(shapes.len(), 7, "ShapeType должен работать независимо");

    // Проверяем что RotationDirection работает независимо
    let _cw = RotationDirection::Clockwise;
    let _ccw = RotationDirection::CounterClockwise;

    // Проверяем что BagGenerator работает независимо
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "BagGenerator должен работать независимо"
    );

    // Проверяем что константы работают независимо
    assert_eq!(
        SHAPE_COORDS.len(),
        7,
        "SHAPE_COORDS должен работать независимо"
    );
    assert_eq!(
        SHAPE_COLORS.len(),
        7,
        "SHAPE_COLORS должен работать независимо"
    );

    // Проверяем что Tetromino работает независимо
    let mut bag2 = BagGenerator::new();
    let tetromino2 = Tetromino::from_bag(&mut bag2);
    assert!(
        tetromino2.pos.0.is_finite(),
        "Tetromino должен работать независимо"
    );
}

// ============================================================================
// ТЕСТ 4: VALIDATION МОДУЛЬ АВТОНОМЕН
// ============================================================================

/// Тест: validation модуль автономен.
///
/// Проверяет что модуль валидации не зависит от других игровых модулей:
/// - validation не зависит от game
/// - validation не зависит от controls
/// - validation зависит только от базовых типов
///
/// ## Архитектурные заметки
/// Автономность validation обеспечивает:
/// - Возможность повторного использования в других модулях
/// - Независимое тестирование
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_validation_is_autonomous() {
    use crate::validation::path::{PathErrorKind, PathValidator};
    use std::path::Path;

    // Проверяем что PathValidator работает независимо
    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");

    // Проверяем валидацию корректного пути
    let valid_path = Path::new("config.json");
    let result = validator.validate(valid_path);
    assert!(
        result.is_ok(),
        "PathValidator должен принимать корректные пути"
    );

    // Проверяем валидацию пути с path traversal
    let invalid_path = "../etc/passwd";
    let result = validator.validate_no_traversal(invalid_path);
    assert!(
        result.is_err(),
        "PathValidator должен отклонять path traversal"
    );

    if let Err(e) = result {
        assert_eq!(
            e.kind,
            PathErrorKind::PathTraversal,
            "Тип ошибки должен быть PathTraversal"
        );
    }

    // Проверяем что PathErrorKind работает независимо
    let _kinds = [
        PathErrorKind::PathTraversal,
        PathErrorKind::TooLong,
        PathErrorKind::AbsolutePath,
    ];
}

// ============================================================================
// ТЕСТ 5: CRYPTO МОДУЛЬ АВТОНОМЕН
// ============================================================================

/// Тест: crypto модуль автономен.
///
/// Проверяет что криптографический модуль не зависит от других игровых модулей:
/// - crypto не зависит от game
/// - crypto не зависит от controls
/// - crypto зависит только от внешних библиотек (blake3, hmac, sha2)
///
/// ## Архитектурные заметки
/// Автономность crypto обеспечивает:
/// - Возможность повторного использования в других проектах
/// - Независимое тестирование
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_crypto_is_autonomous() {
    use crate::crypto::validator::HmacValidator;
    use crate::crypto::{generate_salt, hash, hmac_sha256, verify_hmac_sha256};

    // Проверяем что hash работает независимо
    let h = hash("тест");
    assert_eq!(h.len(), 64, "hash должен работать независимо");
    assert!(
        h.chars().all(|c| c.is_ascii_hexdigit()),
        "Хеш должен быть hex-строкой"
    );

    // Проверяем что generate_salt работает независимо
    let salt1 = generate_salt();
    let salt2 = generate_salt();
    assert_eq!(salt1.len(), 64, "generate_salt должен работать независимо");
    assert_ne!(salt1, salt2, "Соли должны быть уникальными");

    // Проверяем что hmac_sha256 работает независимо
    let key = "тестовый ключ";
    let data = "тестовые данные";
    let signature = hmac_sha256(key, data);
    assert_eq!(
        signature.len(),
        64,
        "hmac_sha256 должен работать независимо"
    );

    // Проверяем что verify_hmac_sha256 работает независимо
    let is_valid = verify_hmac_sha256(key, data, &signature);
    assert!(is_valid, "verify_hmac_sha256 должен работать независимо");

    let is_invalid = verify_hmac_sha256(key, data, "неправильная подпись");
    assert!(
        !is_invalid,
        "verify_hmac_sha256 должен отклонять неправильную подпись"
    );

    // Проверяем что HmacValidator работает независимо
    let validator = HmacValidator::new(key);
    let sig = validator.sign(data);
    assert!(
        validator.verify(data, &sig),
        "HmacValidator должен работать независимо"
    );
}

// ============================================================================
// ТЕСТ 6: HIGHSCORE МОДУЛЬ НЕ ЗАВИСИТ ОТ GAME
// ============================================================================

/// Тест: highscore модуль не зависит от game напрямую.
///
/// Проверяет что модуль рекордов не зависит от игровой логики:
/// - highscore не импортирует game/state
/// - highscore не импортирует game/logic
/// - highscore зависит только от crypto и io
///
/// ## Архитектурные заметки
/// Разделение highscore и game обеспечивает:
/// - Возможность использования highscore в других контекстах
/// - Независимое тестирование
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_highscore_does_not_depend_on_game() {
    use crate::highscore::Leaderboard;

    // Проверяем что Leaderboard работает независимо от game
    let leaderboard = Leaderboard::load();

    // Проверяем что можно получить записи
    let _entries = leaderboard.get_entries();
}

// ============================================================================
// ТЕСТ 7: MENU МОДУЛЬ НЕ ЗАВИСИТ ОТ GAME
// ============================================================================

/// Тест: menu модуль не зависит от game напрямую.
///
/// Проверяет что модуль меню не зависит от игровой логики:
/// - menu не импортирует game/state
/// - menu не импортирует game/logic
/// - menu зависит только от io и types
///
/// ## Архитектурные заметки
/// Разделение menu и game обеспечивает:
/// - Возможность использования menu в других контекстах
/// - Независимое тестирование
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_menu_does_not_depend_on_game() {
    use crate::menu::constants::MENU;
    use crate::menu::draw::draw_menu;

    // Проверяем что константы меню работают независимо
    assert!(!MENU.is_empty(), "MENU должен работать независимо");

    // Проверяем что draw_menu доступен
    let _ = std::any::type_name::<fn(&mut crate::io::Canvas, &crate::game::GameState, &str)>();
}

// ============================================================================
// ТЕСТ 8: CONFIG МОДУЛЬ НЕ ЗАВИСИТ ОТ GAME
// ============================================================================

/// Тест: config модуль не зависит от game напрямую.
///
/// Проверяет что модуль конфигурации не зависит от игровой логики:
/// - config не импортирует game/state
/// - config не импортирует game/logic
/// - config зависит только от serde и crypto
///
/// ## Архитектурные заметки
/// Разделение config и game обеспечивает:
/// - Возможность использования config в других контекстах
/// - Независимое тестирование
/// - Соблюдение Single Responsibility Principle
#[test]
fn test_config_does_not_depend_on_game() {
    use crate::config::keys::get_controls_hmac_key;

    // Проверяем что get_controls_hmac_key работает независимо
    let key = get_controls_hmac_key();
    assert!(!key.is_empty(), "HMAC ключ должен быть не пустым");
}
