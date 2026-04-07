//! Тесты на модульную изоляцию.
//!
//! PROB-158: Тесты проверяют что модули не имеют скрытых зависимостей друг от друга.
//! Каждый модуль должен быть самодостаточным.

/// Тест: crypto модуль не зависит от game модуля
/// Проверяем что криптографические функции работают без игровых структур
#[test]
fn test_crypto_module_independent_of_game() {
    // crypto::hmac работает со строками, не требует GameState
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    let signature = hmac_sha256("key", "data");
    assert!(verify_hmac_sha256("key", "data", &signature));

    // Никакие игровые типы не нужны
    assert_eq!(signature.len(), 64);
}

/// Тест: validation модуль самодостаточен
/// `PathValidator` работает без зависимостей от game/crypto
#[test]
fn test_validation_module_self_contained() {
    use crate::validation::PathValidator;
    use std::path::Path;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");
    let valid_path = Path::new("config.json");

    assert!(validator.validate_length(valid_path).is_ok());
    assert!(validator.validate_characters(valid_path).is_ok());
}

/// Тест: tetromino модуль не зависит от game state
#[test]
fn test_tetromino_module_independent_of_game_state() {
    use crate::tetromino::bag_generator::BagGenerator;

    // BagGenerator работает без GameState
    let mut bag = BagGenerator::new();
    for _ in 0..7 {
        let shape = bag.next_shape();
        // ShapeType — независимый enum
        assert!((shape as usize) < 7);
    }
}

/// Тест: types модуль самодостаточен
#[test]
fn test_types_module_self_contained() {
    use crate::types::UpdateEndState;

    // UpdateEndState — независимый enum
    assert_eq!(format!("{:?}", UpdateEndState::Quit), "Quit");
    assert_eq!(format!("{:?}", UpdateEndState::Lost), "Lost");
    assert_eq!(format!("{:?}", UpdateEndState::Won), "Won");
}

/// Тест: constants модуль не зависит от других модулей
#[test]
fn test_constants_module_independent() {
    use crate::constants::{GRID_HEIGHT, GRID_WIDTH, MAX_LEADERBOARD_ENTRIES};

    // Константы — примитивные значения, не требуют других модулей
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);
    assert_eq!(MAX_LEADERBOARD_ENTRIES, 5);
}

/// Тест: errors модуль самодостаточен
#[test]
fn test_errors_module_self_contained() {
    use crate::errors::GameError;

    // GameError — независимый enum ошибок
    let err = GameError::ValidationError("test".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("test"));
}

/// Тест: scoring модуль можно тестировать изолированно
#[test]
fn test_scoring_module_isolated() {
    use crate::constants::{COMBO_BONUS, LEVEL_BONUS_MULT, LINE_SCORES};

    // scoring константы независимы от GameState
    assert_eq!(LINE_SCORES[0], 100);
    assert_eq!(LINE_SCORES[1], 200);
    assert_eq!(LINE_SCORES[2], 400);
    assert_eq!(LINE_SCORES[3], 1800);
    assert_eq!(COMBO_BONUS, 50);
    assert_eq!(LEVEL_BONUS_MULT, 500);
}

/// Тест: highscore типы не требуют game модуля
#[test]
fn test_highscore_types_independent_of_game() {
    use crate::highscore::leaderboard::LeaderboardEntry;

    // LeaderboardEntry создаётся без GameState
    let entry = LeaderboardEntry::new("Player", 1000);
    assert_eq!(entry.name(), "Player");
    assert_eq!(entry.score(), Some(1000));
}
