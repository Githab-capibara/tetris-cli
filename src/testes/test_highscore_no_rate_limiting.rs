//! Тесты наличия rate limiting (highscore.rs).
//!
//! Этот модуль содержит 5 тестов для проверки исправления:
//! - Rate limiting добавлен в production код
//! - Проверка добавления рекордов с ограничением (10 в минуту)
//! - Проверка блокировки после превышения лимита
//!
//! Исправление: rate limiting был удалён (YAGNI для локальной игры).
//! Тесты помечены как #[ignore] так как rate limiting больше не требуется.

use crate::highscore::Leaderboard;
use std::time::Instant;

// ============================================================================
// ГРУППА ТЕСТОВ: Наличие rate limiting
// ============================================================================

/// Тест 1: Проверка добавления рекордов без задержки
///
/// Проверяет, что рекорды можно добавлять без каких-либо задержек.
///
/// # Примечание
/// Тест игнорируется, так как rate limiting был удалён (YAGNI).
#[ignore = "Rate limiting был удалён намеренно (YAGNI для локальной игры)"]
#[test]
fn test_добавление_рекордов_без_задержки() {
    let mut leaderboard = Leaderboard::default();

    // Засекаем время перед добавлением рекорда
    let start = Instant::now();

    // Добавляем рекорд
    let result = leaderboard.add_score("Player1", 1000);

    // Проверяем что рекорд добавлен
    assert!(result, "Рекорд должен быть добавлен успешно");

    // Проверяем что выполнение заняло минимальное время (< 100мс)
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 100,
        "Добавление рекорда должно занять меньше 100мс, заняло {}мс",
        elapsed.as_millis()
    );

    // Проверяем что рекорд действительно в таблице
    assert_eq!(leaderboard.len(), 1, "Таблица должна содержать 1 запись");
}

/// Тест 2: Проверка добавления нескольких рекордов подряд
///
/// Проверяет, что можно добавлять множество рекордов подряд без rate limiting.
///
/// # Примечание
/// Тест игнорируется, так как rate limiting был удалён (YAGNI).
#[ignore = "Rate limiting был удалён намеренно (YAGNI для локальной игры)"]
#[test]
fn test_добавление_нескольких_рекордов_подряд() {
    let mut leaderboard = Leaderboard::default();

    // Засекаем время
    let start = Instant::now();

    // Добавляем 10 рекордов подряд без задержек
    for i in 0..10 {
        let result = leaderboard.add_score(&format!("Player{i}"), (i + 1) * 100);
        assert!(result, "Рекорд {i} должен быть добавлен без rate limiting");
    }

    // Проверяем время выполнения
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 500,
        "Добавление 10 рекордов должно занять меньше 500мс, заняло {}мс",
        elapsed.as_millis()
    );

    // Проверяем что таблица содержит топ-5
    assert_eq!(
        leaderboard.len(),
        5,
        "Таблица должна содержать только топ-5 записей"
    );

    // Проверяем что записи отсортированы по убыванию
    let entries = leaderboard.get_entries();
    for i in 0..entries.len() - 1 {
        assert!(
            entries[i].score() >= entries[i + 1].score(),
            "Записи должны быть отсортированы по убыванию"
        );
    }
}

/// Тест 3: Проверка наличия блокировки rate limiting
///
/// Стресс-тест: добавление рекордов для проверки наличия rate limiting.
/// После 10 записей следующие должны блокироваться.
///
/// # Примечание
/// Тест игнорируется, так как rate limiting был удалён (YAGNI).
#[ignore = "Rate limiting был удалён намеренно (YAGNI для локальной игры)"]
#[test]
fn test_наличие_блокировки_rate_limiting() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем 10 рекордов подряд (лимит)
    for i in 0..10 {
        let result = leaderboard.add_score(&format!("Player{i}"), i * 10);
        // Первые 10 должны добавиться
        assert!(
            result,
            "Рекорд {i} должен быть добавлен (в пределах лимита)"
        );
    }

    // 11-я запись должна быть отклонена из-за rate limiting
    let result_11 = leaderboard.add_score("Player11", 110);
    assert!(
        !result_11,
        "Рекорд 11 должен быть отклонён (превышен лимит rate limiting)"
    );

    // Проверяем что таблица содержит топ-5 лучших рекордов
    assert_eq!(leaderboard.len(), 5, "Таблица должна содержать топ-5");

    // Проверяем что остались только лучшие рекорды (90, 80, 70, 60, 50)
    let entries = leaderboard.get_entries();
    let min_score = entries
        .iter()
        .map(super::super::highscore::LeaderboardEntry::score)
        .min()
        .unwrap_or(0);
    assert!(
        min_score >= 50, // 5 * 10 = 50 (минимальный из топ-5: 90, 80, 70, 60, 50)
        "Минимальный рекорд в топ-5 должен быть >= 50, получен {min_score}"
    );

    // Проверяем что записи отсортированы по убыванию
    for i in 0..entries.len() - 1 {
        assert!(
            entries[i].score() >= entries[i + 1].score(),
            "Записи должны быть отсортированы по убыванию"
        );
    }
}

/// Тест 4: Проверка rate limiting в реальном времени
///
/// Проверяет что rate limiting блокирует добавление после 10 записей.
///
/// # Примечание
/// Тест игнорируется, так как rate limiting был удалён (YAGNI).
#[ignore = "Rate limiting был удалён намеренно (YAGNI для локальной игры)"]
#[test]
fn test_быстрое_добавление_в_реальном_времени() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем 10 рекордов (лимит)
    for i in 0..10 {
        let result = leaderboard.add_score(&format!("FastPlayer{i}"), 1000 + i);
        assert!(
            result,
            "Рекорд {i} должен быть добавлен (в пределах лимита)"
        );
    }

    // 11-я запись должна быть отклонена
    let result_11 = leaderboard.add_score("FastPlayer11", 1011);
    assert!(!result_11, "Рекорд 11 должен быть отклонён (rate limiting)");

    // 12-я запись тоже должна быть отклонена
    let result_12 = leaderboard.add_score("FastPlayer12", 1012);
    assert!(!result_12, "Рекорд 12 должен быть отклонён (rate limiting)");
}

/// Тест 5: Проверка что rate limiting работает корректно
///
/// Проверяет наличие rate limiting при добавлении рекордов.
///
/// # Примечание
/// Тест игнорируется, так как rate limiting был удалён (YAGNI).
#[ignore = "Rate limiting был удалён намеренно (YAGNI для локальной игры)"]
#[test]
fn test_отсутствие_скрытых_задержек() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем первый рекорд
    let result1 = leaderboard.add_score("First", 1000);
    assert!(result1, "Первый рекорд должен быть добавлен");

    // Добавляем второй рекорд
    let result2 = leaderboard.add_score("Second", 2000);
    assert!(result2, "Второй рекорд должен быть добавлен");

    // Добавляем третий рекорд
    let result3 = leaderboard.add_score("Third", 3000);
    assert!(result3, "Третий рекорд должен быть добавлен");

    // Добавляем ещё 7 рекордов (достигаем лимита в 10)
    for i in 0..7 {
        let result = leaderboard.add_score(&format!("Player{i}"), 4000 + i);
        assert!(
            result,
            "Рекорд {i} должен быть добавлен (в пределах лимита)"
        );
    }

    // Теперь пытаемся добавить ещё один рекорд - должен быть отклонён
    let result_after_limit = leaderboard.add_score("AfterLimit", 5000);
    assert!(
        !result_after_limit,
        "Рекорд после достижения лимита должен быть отклонён"
    );
}
