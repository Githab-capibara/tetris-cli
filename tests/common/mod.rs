//! Общие хелперы для интеграционных тестов.
//!
//! Содержит фабричные функции и константы для уменьшения дублирования.

use tetris_cli::highscore::leaderboard::{LeaderboardEntry, ThreadSafeLeaderboardEntry};
use tetris_cli::highscore::Leaderboard;

/// Создать Leaderboard с заданным количеством записей.
#[allow(dead_code)]
pub fn create_leaderboard_with_scores(scores: &[(String, u128)]) -> Leaderboard {
    let mut lb = Leaderboard::default();
    for (name, score) in scores {
        let _ = lb.add_score(name, *score);
    }
    lb
}

/// Создать стандартный набор рекордов (5 игроков).
#[allow(dead_code)]
pub fn create_standard_leaderboard() -> Leaderboard {
    create_leaderboard_with_scores(&[
        ("Alice".to_string(), 5000),
        ("Bob".to_string(), 4000),
        ("Charlie".to_string(), 3000),
        ("Diana".to_string(), 2000),
        ("Eve".to_string(), 1000),
    ])
}

/// Создать валидную запись `LeaderboardEntry`.
#[allow(dead_code)]
pub fn create_valid_entry(name: &str, score: u128) -> LeaderboardEntry {
    LeaderboardEntry::new(name, score)
}

/// Создать потокобезопасную запись.
#[allow(dead_code)]
pub fn create_thread_safe_entry(name: &str, score: u128) -> ThreadSafeLeaderboardEntry {
    ThreadSafeLeaderboardEntry::new(name, score)
}
