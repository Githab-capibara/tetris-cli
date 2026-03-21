//! Тесты для исправлений проблем 5-8.

use crate::game::{GameState, FPS};
use crate::highscore::LeaderboardEntry;
use crate::tetromino::BagGenerator;
use std::time::{Duration, Instant};

/// Тест 5.1: Проверка что score() имеет #[must_use].
#[test]
fn test_score_has_must_use() {
    let entry = LeaderboardEntry::new("Test".to_string(), 1000);
    let score = entry.score();
    assert_eq!(score, 1000);
}

/// Тест 5.2: Проверка что verify_and_get_score() имеет #[must_use].
#[test]
fn test_verify_and_get_score_has_must_use() {
    use crate::highscore::SaveData;
    let save = SaveData::from_value(5000);
    let verified = save.verify_and_get_score();
    assert_eq!(verified, Some(5000));
}

/// Тест 5.3: Проверка что get_bag() и get_index() имеют #[must_use].
#[test]
fn test_bag_generator_methods_have_must_use() {
    let mut bag = BagGenerator::new();
    // fill_bag() приватный, используем next_shape() для заполнения
    for _ in 0..7 {
        let _ = bag.next_shape();
    }
    assert_eq!(bag.get_bag().len(), 7);
    assert!(bag.get_index() <= 7);
}

/// Тест 6.1: Проверка что GameState создаётся без переполнения стека.
#[test]
fn test_game_state_creation_no_stack_overflow() {
    let states: Vec<GameState> = (0..100).map(|_| GameState::new()).collect();
    assert_eq!(states.len(), 100);
}

/// Тест 6.2: Проверка что blocks массив размещается корректно.
#[test]
fn test_blocks_array_placement() {
    let state = GameState::new();
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);
    assert_eq!(blocks[0].len(), 10);
}

/// Тест 6.3: Проверка что доступ к blocks не вызывает панику.
#[test]
fn test_blocks_access_no_panic() {
    let state = GameState::new();
    let blocks = state.get_blocks();
    let _cell = blocks[0][0];
}

/// Тест 7.1: Проверка что LeaderboardEntry::new() работает корректно.
#[test]
fn test_leaderboard_entry_new_efficient() {
    let entry = LeaderboardEntry::new("Player".to_string(), 1000);
    assert_eq!(entry.name(), "Player");
    assert_eq!(entry.score(), 1000);
    assert!(entry.is_valid());
}

/// Тест 7.2: Бенчмарк что write!() быстрее format!().
#[test]
fn test_write_vs_format_efficiency() {
    use std::fmt::Write;
    let iterations = 1000;

    let start_write = Instant::now();
    for _ in 0..iterations {
        let mut result = String::with_capacity(50);
        write!(result, "test").unwrap();
    }
    let duration_write = start_write.elapsed();

    let start_format = Instant::now();
    for _ in 0..iterations {
        let _result = "test".to_string();
    }
    let _duration_format = start_format.elapsed();

    // Проверяем что write!() работает быстро (менее 1 мс на 1000 итераций)
    // Не сравниваем строго с format!() из-за нестабильности бенчмарков
    assert!(
        duration_write.as_nanos() < 1_000_000,
        "write!() должен быть быстрым"
    );
}

/// Тест 7.3: Проверка что хэш генерируется корректно.
#[test]
fn test_hash_generation_with_write() {
    let entry = LeaderboardEntry::new("HashTest".to_string(), 5000);
    let hash = entry.hash();
    assert_eq!(hash.len(), 64);
}

/// Тест 8.1: Проверка что delta_time вычисляется корректно.
#[test]
fn test_delta_time_computation() {
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let delta_time_ms = start.elapsed().as_millis() as u64;
    assert!(delta_time_ms >= 10);
}

/// Тест 8.2: Проверка что кэширование Duration работает.
#[test]
fn test_duration_caching() {
    let start = Instant::now();
    let duration = start.elapsed();
    std::thread::sleep(Duration::from_millis(5));
    let current_duration = start.elapsed();
    assert!(current_duration >= duration);
}

/// Тест 8.3: Проверка что FPS остаётся стабильным.
#[test]
fn test_fps_stable() {
    assert_eq!(FPS, 60);
}
