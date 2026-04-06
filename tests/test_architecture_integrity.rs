//! Тесты целостности архитектуры для проекта tetris-cli.
//!
//! Этот файл содержит ТОЛЬКО тесты ПОВЕДЕНИЯ — без чтения исходных файлов.
//! Архитектурные ограничения проверяются через публичный API, не через парсинг кода.

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

// ========================================================================
// ТЕСТЫ ПОВЕДЕНИЯ: TOCTOU ЗАЩИТА И ПОТОКОБЕЗОПАСНОСТЬ
// ========================================================================

/// Проверить что `score()` и `is_valid()` атомарны.
/// `LeaderboardEntry` намеренно !Send + !Sync (`PhantomData`<*mut ()>),
/// поэтому тестируем атомарность в одном потоке.
#[test]
fn test_thread_safe_leaderboard_entry_is_atomic() {
    let player_name = "TestPlayer";
    let score = 1000u128;

    let entry = tetris_cli::highscore::leaderboard::LeaderboardEntry::new(player_name, score);

    // Проверяем что score() возвращает корректное значение
    assert_eq!(
        entry.score(),
        Some(score),
        "score() должен возвращать корректное значение"
    );

    // Проверяем что is_valid() работает корректно
    assert!(
        entry.is_valid(),
        "is_valid() должен возвращать true для валидной записи"
    );

    // Проверяем что hash() возвращает непустую строку
    let hash = entry.hash();
    assert!(!hash.is_empty(), "hash() должен возвращать непустую строку");

    // Проверяем что name() возвращает правильное имя
    assert_eq!(
        entry.name(),
        player_name,
        "name() должен возвращать правильное имя"
    );

    // Проверяем что score() и is_valid() согласованы
    if entry.is_valid() {
        assert_eq!(
            entry.score(),
            Some(score),
            "score() должен возвращать корректное значение для валидной записи"
        );
    }

    // Тест для Leaderboard (однопоточный)
    let mut leaderboard = tetris_cli::highscore::leaderboard::Leaderboard::default();

    for i in 0..5 {
        let player_name = format!("Player_{i}");
        let score = 1000 + i as u128;

        let entry = tetris_cli::highscore::leaderboard::LeaderboardEntry::new(&player_name, score);

        // Проверяем атомарность
        assert_eq!(entry.score(), Some(score));
        assert!(entry.is_valid());

        let _ = leaderboard.add_score(&player_name, score);
    }

    assert!(
        !leaderboard.get_entries().is_empty(),
        "Записи должны быть добавлены в таблицу лидеров"
    );
}

/// Тест на целостность данных `LeaderboardEntry`.
#[test]
fn test_leaderboard_entry_thread_safety() {
    // LeaderboardEntry !Send + !Sync, поэтому тестируем целостность в одном потоке
    let mut entries = Vec::new();

    for i in 0..100 {
        let player_name = format!("Player_{i}");
        let score = i as u128 * 100;

        let entry = tetris_cli::highscore::leaderboard::LeaderboardEntry::new(&player_name, score);

        // Проверяем что запись валидна
        assert!(entry.is_valid(), "Запись должна быть валидной");
        assert_eq!(entry.score(), Some(score), "Счёт должен совпадать");
        assert_eq!(entry.name(), player_name, "Имя должно совпадать");

        entries.push(entry);
    }

    // Проверяем что все записи остаются валидными после множественных операций
    for (i, entry) in entries.iter().enumerate() {
        let expected_score = i as u128 * 100;
        assert!(entry.is_valid(), "Запись должна оставаться валидной");
        assert_eq!(
            entry.score(),
            Some(expected_score),
            "Счёт должен совпадать после множественных операций"
        );
    }

    // Тест для Leaderboard
    let mut leaderboard = tetris_cli::highscore::leaderboard::Leaderboard::default();

    for i in 0..10 {
        let player_name = format!("Thread_{i}");
        let score = i as u128 * 100;
        let _ = leaderboard.add_score(&player_name, score);
    }

    let entries = leaderboard.get_entries();
    assert!(
        !entries.is_empty(),
        "Таблица лидеров должна содержать хотя бы одну запись"
    );

    // Проверяем что все записи валидны
    for entry in entries {
        assert!(entry.is_valid(), "Все записи должны быть валидными");
    }
}
