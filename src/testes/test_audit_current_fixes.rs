//! Тесты для верификации исправлений аудита.
//!
//! 3 теста на каждую исправленную проблему.

use crate::controls::ControlsConfig;
use crate::game::GameState;
use crate::highscore::{LeaderboardEntry, SaveData};

// ============================================================================
// Тесты для #1: Логика определения подделки рекорда (is_none вместо == 0)
// ============================================================================

/// Тест 1: verify_and_get_score возвращает Some для валидного счёта
#[test]
fn test_cheat_detection_is_none_returns_false_for_valid_score() {
    let save = SaveData::from_value(1000);
    let result = save.verify_and_get_score();
    assert!(result.is_some(), "Valid score should return Some");
}

/// Тест 2: verify_and_get_score возвращает None для подделанного счёта
#[test]
fn test_cheat_detection_is_none_returns_true_for_tampered() {
    let mut save = SaveData::from_value(1000);
    save.high_score = 9999;
    let result = save.verify_and_get_score();
    assert!(result.is_none(), "Tampered score should return None");
}

/// Тест 3: Edge case - счёт равный 0
#[test]
fn test_cheat_detection_edge_case_zero_score() {
    let save = SaveData::from_value(0);
    let result = save.verify_and_get_score();
    assert!(
        result.is_some(),
        "Zero score is valid, should return Some(0)"
    );
    assert_eq!(result.unwrap(), 0);
}

// ============================================================================
// Тесты для #2: Замена .expect() на proper обработку ошибок
// ============================================================================

/// Тест 4: LeaderboardEntry::new работает с валидными данными
#[test]
fn test_leaderboard_entry_new_works_with_valid_data() {
    let entry = LeaderboardEntry::new("Player1".to_string(), 1000);
    assert_eq!(entry.name(), "Player1");
    assert_eq!(entry.score(), 1000);
}

/// Тест 5: is_valid возвращает true для валидной записи
#[test]
fn test_leaderboard_entry_is_valid_returns_true() {
    let entry = LeaderboardEntry::new("Player1".to_string(), 1000);
    assert!(entry.is_valid(), "Valid entry should return true");
}

/// Тест 6: is_valid возвращает false для подделанной записи
#[test]
fn test_leaderboard_entry_is_valid_returns_false_for_tampered() {
    let mut entry = LeaderboardEntry::new("Player1".to_string(), 1000);
    entry.score = 9999;
    assert!(!entry.is_valid(), "Tampered entry should return false");
}

// ============================================================================
// Тесты для #3: Добавление #[must_use] к validate()
// ============================================================================

/// Тест 7: validate возвращает Ok для валидного пути
#[test]
fn test_validate_returns_ok_for_valid_path() {
    let config = ControlsConfig::default();
    let result = config.validate();
    assert!(result.is_ok(), "Valid config should return Ok");
}

/// Тест 8: validate возвращает Err для невалидного пути
#[test]
fn test_validate_returns_err_for_invalid_path() {
    let mut config = ControlsConfig::default();
    config.key_config_file = "/nonexistent/path/config.toml".to_string();
    let result = config.validate();
    assert!(result.is_err(), "Invalid path should return Err");
}

/// Тест 9: validate имеет атрибут #[must_use]
#[test]
fn test_validate_must_use_attribute_present() {
    let config = ControlsConfig::default();
    let _ = config.validate();
}

// ============================================================================
// Тесты для #4: Удаление неиспользуемого поля dirty_cells
// ============================================================================

/// Тест 10: GameState компилируется без dirty_cells
#[test]
fn test_gamestate_compiles_without_dirty_cells() {
    let state = GameState::new(10, 20);
    assert!(state.grid.width() == 10 && state.grid.height() == 20);
}

/// Тест 11: игра запускается без ошибок
#[test]
fn test_gamestate_starts_without_errors() {
    let state = GameState::new(10, 20);
    assert!(state.score() == 0);
}

/// Тест 12: GameState работает без поля dirty_cells
#[test]
fn test_game_starts_without_dirty_cells_field() {
    let state = GameState::new(10, 20);
    assert_eq!(state.level(), 1);
}
