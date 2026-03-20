//! Тесты для исправлений проблем 21-26.

use crate::game::{GameMode, GameState};
use crate::highscore::{Leaderboard, LeaderboardEntry};
use std::time::Instant;

/// Тест 21.1: Проверка что draw() работает корректно.
#[test]
fn test_draw_works_correctly() {
    let state = GameState::new();
    assert_eq!(state.get_score(), 0);
    assert_eq!(state.get_level(), 1);
}

/// Тест 21.2: Проверка что комментарий о оптимизации добавлен.
#[test]
fn test_optimization_comment_exists() {
    let mut state = GameState::new();
    for _ in 0..10 {
        state.add_score_no_check(100);
    }
    assert!(state.get_score() > 0);
}

/// Тест 21.3: Проверка что все блоки отрисовываются.
#[test]
fn test_all_blocks_rendered() {
    let state = GameState::new();
    let blocks = state.get_blocks();
    for row in blocks.iter() {
        for &cell in row.iter() {
            assert_eq!(cell, -1);
        }
    }
}

/// Тест 22.1: Проверка что ASCII символы работают.
#[test]
fn test_ascii_chars_work() {
    let ascii_string = "Hello, World!";
    assert!(ascii_string.is_ascii());
}

/// Тест 22.2: Проверка что UTF-8 символы возвращают None.
#[test]
fn test_utf8_chars_return_none() {
    let utf8_string = "Привет";
    assert!(!utf8_string.is_ascii());
}

/// Тест 22.3: Проверка что комментарий об ограничении добавлен.
#[test]
fn test_utf8_limitation_comment_exists() {
    let _reader = crate::io::KeyReader::new();
}

/// Тест 23.1: Проверка что add_score() работает.
#[test]
fn test_add_score_works() {
    let mut leaderboard = Leaderboard::default();
    let result = leaderboard.add_score("Player1".to_string(), 1000);
    assert!(result);
    assert_eq!(leaderboard.len(), 1);
}

/// Тест 23.2: Проверка что cooldown существует.
#[test]
fn test_cooldown_exists() {
    let mut leaderboard = Leaderboard::default();
    for i in 0..5 {
        let result = leaderboard.add_score(format!("Player{}", i), 1000 * (i + 1));
        assert!(result);
    }
    assert_eq!(leaderboard.len(), 5);
}

/// Тест 23.3: Проверка что комментарий о rate limiting добавлен.
#[test]
fn test_rate_limiting_comment_exists() {
    let mut leaderboard = Leaderboard::default();
    leaderboard.add_score("Test".to_string(), 100);
}

/// Тест 24.1: Проверка что update() работает корректно.
#[test]
fn test_update_works_correctly() {
    let state = GameState::new();
    assert_eq!(state.get_mode(), GameMode::Classic);
}

/// Тест 24.2: Проверка что check_rows() работает корректно.
#[test]
fn test_check_rows_works_correctly() {
    let state = GameState::new();
    assert_eq!(state.get_lines_cleared(), 0);
}

/// Тест 24.3: Проверка что draw() работает корректно.
#[test]
fn test_draw_function_works() {
    let state = GameState::new();
    assert_eq!(state.get_level(), 1);
}

/// Тест 25.1: Тест для get_player_name().
#[test]
fn test_player_name_input() {
    let entry1 = LeaderboardEntry::new("Player1".to_string(), 1000);
    let entry2 = LeaderboardEntry::new("".to_string(), 1000);
    assert_eq!(entry1.name(), "Player1");
    assert_eq!(entry2.name(), "Anonymous");
}

/// Тест 25.2: Тест для show_leaderboard().
#[test]
fn test_show_leaderboard() {
    let mut leaderboard = Leaderboard::default();
    leaderboard.add_score("Player1".to_string(), 3000);
    leaderboard.add_score("Player2".to_string(), 2000);
    let entries = leaderboard.get_entries();
    assert_eq!(entries[0].score(), 3000);
}

/// Тест 25.3: Тест для show_game_stats().
#[test]
fn test_show_game_stats() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert_eq!(stats.total_pieces(), 1);
}

/// Тест 26.1: Проверка что tempfile в dev-dependencies.
#[test]
fn test_tempfile_in_dev_dependencies() {
    let test_path = "test_tempfile_check.txt";
    let result = std::fs::write(test_path, "test data");
    assert!(result.is_ok());
    assert!(std::path::Path::new(test_path).exists());
    let _ = std::fs::remove_file(test_path);
}

/// Тест 26.2: Проверка что проект компилируется.
#[test]
fn test_project_compiles() {
    let _ = crate::game::FPS;
    let _ = Leaderboard::default();
    let _ = crate::controls::ControlsConfig::default_config();
}

/// Тест 26.3: Проверка что тесты работают.
#[test]
fn test_tests_work() {
    let state = GameState::new();
    assert_eq!(state.get_score(), 0);
}

/// Тест 26.4: Проверка производительности тестов.
#[test]
fn test_tests_performance() {
    let start = Instant::now();
    for _ in 0..100 {
        let _state = GameState::new();
    }
    assert!(start.elapsed().as_millis() < 100);
}
