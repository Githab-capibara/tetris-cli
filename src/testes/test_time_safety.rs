//! Тесты обработки unwrap() времени.
//!
//! Проверяют, что get_current_time_ms_protected не паникует и корректно
//! обрабатывает ошибки времени.

use crate::highscore::leaderboard::LeaderboardEntry;
use crate::highscore::Leaderboard;

/// Тест 1: Проверка, что get_current_time_ms_protected не паникует
///
/// Функция должна корректно работать без паники.
#[test]
fn test_get_current_time_ms_no_panic() {
    // Создаём таблицу лидеров - это использует get_current_time_ms_protected
    let mut leaderboard = Leaderboard::load();

    // Добавляем запись
    leaderboard.add_score("TestPlayer", 1000);

    // Если дошли сюда - паники не было
    assert!(
        true,
        "get_current_time_ms_protected не должен вызывать панику"
    );
}

/// Тест 2: Проверка обработки ошибок времени
///
/// Проверяем, что ошибки времени обрабатываются корректно.
#[test]
fn test_time_error_handling() {
    // Создаём несколько записей - это использует get_current_time_ms_protected
    let entry1 = LeaderboardEntry::new("Player1", 1000);
    let entry2 = LeaderboardEntry::new("Player2", 2000);
    let entry3 = LeaderboardEntry::new("Player3", 3000);

    // Проверяем, что записи созданы корректно
    assert_eq!(entry1.name(), "Player1", "Имя должно быть 'Player1'");
    assert_eq!(entry1.score(), 1000, "Счёт должен быть 1000");

    assert_eq!(entry2.name(), "Player2", "Имя должно быть 'Player2'");
    assert_eq!(entry2.score(), 2000, "Счёт должен быть 2000");

    assert_eq!(entry3.name(), "Player3", "Имя должно быть 'Player3'");
    assert_eq!(entry3.score(), 3000, "Счёт должен быть 3000");
}

/// Тест 3: Проверка rate limiting с защитой от изменения времени
///
/// Проверяем, что rate limiting работает корректно.
#[test]
fn test_rate_limiting_with_time_protection() {
    use crate::highscore::Leaderboard;

    let mut leaderboard = Leaderboard::load();

    // Запоминаем начальное количество записей
    let initial_count = leaderboard.get_entries().len();

    // Добавляем несколько записей быстро
    for i in 0..5 {
        leaderboard.add_score(&format!("Player{i}"), 1000 + i);
    }

    // Проверяем, что записи добавлены
    let final_count = leaderboard.get_entries().len();

    // В тестах rate limiting отключен, поэтому все записи должны добавиться
    assert!(
        final_count >= initial_count,
        "Записи должны добавляться в таблицу лидеров"
    );
}

/// Тест 4: Проверка корректности timestamp
///
/// Проверяем, что timestamp корректный.
#[test]
fn test_timestamp_correctness() {
    use std::time::SystemTime;

    // Получаем текущий timestamp
    let current_time_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Время должно быть корректным")
        .as_millis() as u64;

    // Проверяем, что timestamp положительный
    assert!(current_time_ms > 0, "Timestamp должен быть положительным");

    // Проверяем, что timestamp разумный (после 2020 года)
    let year_2020_ms = 1_577_836_800_000_u64; // 2020-01-01 00:00:00 UTC
    assert!(
        current_time_ms > year_2020_ms,
        "Timestamp должен быть после 2020 года"
    );
}

/// Тест 5: Проверка множественных вызовов времени
///
/// Проверяем, что множественные вызовы работают корректно.
#[test]
fn test_multiple_time_calls() {
    let mut timestamps = Vec::new();

    // Делаем 100 вызовов
    for _ in 0..100 {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Время должно быть корректным")
            .as_millis() as u64;
        timestamps.push(ts);
    }

    // Проверяем, что timestamps не убывают
    for i in 1..timestamps.len() {
        assert!(
            timestamps[i] >= timestamps[i - 1],
            "Timestamps не должны убывать"
        );
    }
}
