//! Тесты отсутствия rate limiting (highscore.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Rate limiting удалён из production кода
//! - Проверка добавления рекордов без задержки
//! - Проверка добавления нескольких рекордов подряд
//!
//! Исправление: rate limiting удалён, теперь можно добавлять рекорды без задержек

use crate::highscore::Leaderboard;
use std::time::Instant;

// ============================================================================
// ГРУППА ТЕСТОВ: Отсутствие rate limiting
// ============================================================================

/// Тест 1: Проверка добавления рекордов без задержки
///
/// Проверяет, что рекорды можно добавлять без каких-либо задержек.
#[test]
fn test_добавление_рекордов_без_задержки() {
    let mut leaderboard = Leaderboard::default();

    // Засекаем время перед добавлением рекорда
    let start = Instant::now();

    // Добавляем рекорд
    let result = leaderboard.add_score("Player1".to_string(), 1000);

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

    assert!(true, "Рекорд добавлен без задержки");
}

/// Тест 2: Проверка добавления нескольких рекордов подряд
///
/// Проверяет, что можно добавлять множество рекордов подряд без rate limiting.
#[test]
fn test_добавление_нескольких_рекордов_подряд() {
    let mut leaderboard = Leaderboard::default();

    // Засекаем время
    let start = Instant::now();

    // Добавляем 10 рекордов подряд без задержек
    for i in 0..10 {
        let result = leaderboard.add_score(format!("Player{}", i), (i + 1) * 100);
        assert!(
            result,
            "Рекорд {} должен быть добавлен без rate limiting",
            i
        );
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

    assert!(true, "10 рекордов добавлены подряд без rate limiting");
}

/// Тест 3: Проверка отсутствия блокировки rate limiting
///
/// Стресс-тест: добавление 100 рекордов для проверки отсутствия rate limiting.
#[test]
fn test_отсутствие_блокировки_rate_limiting() {
    let mut leaderboard = Leaderboard::default();

    // Засекаем время
    let start = Instant::now();

    // Добавляем 100 рекордов подряд
    // При наличии rate limiting это заняло бы много времени
    for i in 0..100 {
        let result = leaderboard.add_score(format!("Player{}", i), i * 10);
        // Все рекорды должны добавиться (rate limiting отключён)
        assert!(
            result || leaderboard.len() == 5,
            "Рекорд {} должен быть добавлен или таблица уже полная",
            i
        );
    }

    // Проверяем время выполнения
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 2,
        "Добавление 100 рекордов должно занять меньше 2 секунд, заняло {}с",
        elapsed.as_secs()
    );

    // Проверяем что таблица содержит топ-5 лучших рекордов
    assert_eq!(leaderboard.len(), 5, "Таблица должна содержать топ-5");

    // Проверяем что остались только лучшие рекорды
    let entries = leaderboard.get_entries();
    let min_score = entries.iter().map(|e| e.score()).min().unwrap_or(0);
    assert!(
        min_score >= 950, // 95 * 10 = 950 (минимальный из топ-5)
        "Минимальный рекорд в топ-5 должен быть >= 950, получен {}",
        min_score
    );

    assert!(true, "100 рекордов добавлены без rate limiting блокировки");
}

/// Тест 4: Проверка быстрого добавления в реальном времени
///
/// Проверяет что добавление рекордов не блокируется в реальном времени.
#[test]
fn test_быстрое_добавление_в_реальном_времени() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем рекорды в цикле с минимальными интервалами
    let mut timestamps: Vec<std::time::Duration> = Vec::new();

    for i in 0..20 {
        let before = Instant::now();
        let _ = leaderboard.add_score(format!("FastPlayer{}", i), 1000 + i);
        let elapsed = before.elapsed();
        timestamps.push(elapsed);
    }

    // Проверяем что все добавления заняли меньше 50мс каждое
    for (i, &ts) in timestamps.iter().enumerate() {
        assert!(
            ts.as_millis() < 50,
            "Добавление рекорда {} заняло {}мс (должно быть < 50мс)",
            i,
            ts.as_millis()
        );
    }

    // Проверяем среднее время
    let avg_ms: u128 = timestamps.iter().map(|t| t.as_millis()).sum::<u128>() / timestamps.len() as u128;
    assert!(
        avg_ms < 10,
        "Среднее время добавления должно быть < 10мс, получено {}мс",
        avg_ms
    );

    assert!(true, "Быстрое добавление в реальном времени работает без задержек");
}

/// Тест 5: Проверка что нет скрытых задержек
///
/// Проверяет отсутствие скрытых задержек при добавлении рекордов.
#[test]
fn test_отсутствие_скрытых_задержек() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем первый рекорд
    let start1 = Instant::now();
    let _ = leaderboard.add_score("First".to_string(), 1000);
    let time1 = start1.elapsed();

    // Добавляем второй рекорд сразу после первого
    let start2 = Instant::now();
    let _ = leaderboard.add_score("Second".to_string(), 2000);
    let time2 = start2.elapsed();

    // Добавляем третий рекорд
    let start3 = Instant::now();
    let _ = leaderboard.add_score("Third".to_string(), 3000);
    let time3 = start3.elapsed();

    // Проверяем что все добавления заняли примерно одинаковое время
    // (нет накопительных задержек или cooldown)
    let max_time = time1.max(time2).max(time3);
    let min_time = time1.min(time2).min(time3);

    assert!(
        max_time.as_millis() < 100,
        "Максимальное время добавления должно быть < 100мс"
    );

    // Проверяем что разница между самым быстрым и самым медленным невелика
    let diff = max_time.saturating_sub(min_time);
    assert!(
        diff.as_millis() < 50,
        "Разница во времени добавления должна быть < 50мс, получена {}мс",
        diff.as_millis()
    );

    assert!(true, "Скрытые задержки отсутствуют");
}
