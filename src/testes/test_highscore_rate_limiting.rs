//! Тесты rate limiting для таблицы лидеров (highscore.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка cooldown 5 секунд между записями
//! - Проверка блокировки быстрых повторных записей
//! - Проверка разблокировки после истечения cooldown
//!
//! Rate limiting предотвращает спам записей в таблицу лидеров.

use crate::highscore::Leaderboard;
use std::thread;
use std::time::Duration;

// ============================================================================
// ГРУППА ТЕСТОВ: Rate limiting (5 секунд cooldown)
// ============================================================================

/// Тест 1: Проверка блокировки повторной записи быстрее 5 секунд
///
/// Проверяет, что нельзя добавить второй рекорд быстрее чем через 5 секунд.
#[test]
fn test_блокировка_быстрой_повторной_записи() {
    // Используем load() который инициализирует rate limiting
    let mut leaderboard = Leaderboard::load();

    // Добавляем первый рекорд
    let result1 = leaderboard.add_score("Player1".to_string(), 1000);
    assert!(result1, "Первый рекорд должен быть добавлен успешно");

    // Пытаемся добавить второй рекорд сразу же
    let result2 = leaderboard.add_score("Player2".to_string(), 2000);
    assert!(
        !result2,
        "Второй рекорд должен быть заблокирован rate limiting"
    );

    // Проверяем что в таблице только одна запись
    assert_eq!(
        leaderboard.len(),
        1,
        "Таблица должна содержать только 1 запись после блокировки"
    );
}

/// Тест 2: Проверка разблокировки после истечения cooldown
///
/// Проверяет, что после ожидания 5+ секунд можно добавить новый рекорд.
#[test]
fn test_разблокировка_после_истечения_cooldown() {
    // Используем load() который инициализирует rate limiting
    let mut leaderboard = Leaderboard::load();

    // Добавляем первый рекорд
    let result1 = leaderboard.add_score("Player1".to_string(), 1000);
    assert!(result1, "Первый рекорд должен быть добавлен успешно");

    // Ждём 5.1 секунд (cooldown = 5 секунд)
    thread::sleep(Duration::from_millis(5100));

    // Пытаемся добавить второй рекорд
    let result2 = leaderboard.add_score("Player2".to_string(), 2000);
    assert!(
        result2,
        "Второй рекорд должен быть добавлен после истечения cooldown"
    );

    // Проверяем что в таблице две записи
    assert_eq!(
        leaderboard.len(),
        2,
        "Таблица должна содержать 2 записи после разблокировки"
    );
}

/// Тест 3: Проверка граничного значения cooldown (ровно 5 секунд)
///
/// Проверяет поведение на границе cooldown периода.
#[test]
fn test_граничное_значение_cooldown() {
    // Используем load() который инициализирует rate limiting
    let mut leaderboard = Leaderboard::load();

    // Добавляем первый рекорд
    let result1 = leaderboard.add_score("Player1".to_string(), 1000);
    assert!(result1, "Первый рекорд должен быть добавлен успешно");

    // Ждём 4.9 секунды (чуть меньше cooldown)
    thread::sleep(Duration::from_millis(4900));

    // Пытаемся добавить второй рекорд - должен быть заблокирован
    let result2 = leaderboard.add_score("Player2".to_string(), 2000);
    assert!(
        !result2,
        "Запись должна быть заблокирована до истечения 5 секунд"
    );

    // Ждём ещё 0.3 секунды (итого 5.2 секунды)
    thread::sleep(Duration::from_millis(300));

    // Теперь должно пройти
    let result3 = leaderboard.add_score("Player3".to_string(), 3000);
    assert!(result3, "Запись должна пройти после истечения 5 секунд");

    // Проверяем что в таблице две записи (первая и третья)
    assert_eq!(leaderboard.len(), 2, "Таблица должна содержать 2 записи");
}
