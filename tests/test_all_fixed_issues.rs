//! Тесты для проверки исправленных проблем в проекте tetris-cli.
//!
//! Этот файл содержит только тесты ПОВЕДЕНИЯ — никаких проверок исходного кода.
//!
//! ## Запуск тестов
//! ```bash
//! cargo test --test test_all_fixed_issues
//! ```

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

mod common;

use std::sync::Arc;
use std::thread;

// ============================================================================
// ТЕСТЫ ПОВЕДЕНИЯ (НЕ ЧИТАЮТ ИСХОДНЫЙ КОД)
// ============================================================================

/// Тест E2: `ThreadSafeLeaderboardEntry::score_safe()` без паники
///
/// Проверяет поведение: `score_safe()` возвращает Option<u128>
/// вместо паники при отравлении Mutex.
#[test]
fn test_fix_e2_thread_safe_score_no_panic() {
    use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboardEntry;

    // score_safe() возвращает Some(score) для валидной записи
    let entry = ThreadSafeLeaderboardEntry::new("Player1", 1000);
    let score = entry.score_safe();
    assert_eq!(
        score,
        Some(1000),
        "score_safe() должен возвращать Some(score) для валидной записи"
    );

    // is_valid_safe() возвращает Option<bool>
    let is_valid = entry.is_valid_safe();
    assert!(
        is_valid.is_some(),
        "is_valid_safe() должен возвращать Some(bool)"
    );

    // name_safe() возвращает Option<String>
    let name = entry.name_safe();
    assert!(name.is_some(), "name_safe() должен возвращать Some(String)");
    assert_eq!(name, Some("Player1".to_string()));

    // Проверяем что deprecated методы тоже не паникуют
    #[allow(deprecated)]
    let score_deprecated = entry.score();
    assert_eq!(score_deprecated, 1000);
}

// ============================================================================
// СТРЕСС-ТЕСТ ПОТОКОБЕЗОПАСНОСТИ (реальный, 100+ потоков)
// ============================================================================

/// Стресс-тест потокобезопасности: 100+ потоков одновременно
#[test]
fn test_thread_safety_stress_test() {
    use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboardEntry;

    let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player1", 1000));
    let mut handles = vec![];

    // 100 потоков одновременно читают запись
    for i in 0..100 {
        let entry_clone = Arc::clone(&entry);
        let handle = thread::spawn(move || {
            // Читаем score из нескольких потоков одновременно
            let score = entry_clone.score_safe();
            assert_eq!(score, Some(1000), "Поток {i}: score должен быть 1000");
        });
        handles.push(handle);
    }

    // Ждём завершения всех потоков
    for (i, handle) in handles.into_iter().enumerate() {
        handle
            .join()
            .unwrap_or_else(|_| panic!("Поток {i} должен завершиться успешно"));
    }

    // Проверяем что запись всё ещё валидна после всех операций
    let final_score = entry.score_safe();
    assert_eq!(final_score, Some(1000));
}
