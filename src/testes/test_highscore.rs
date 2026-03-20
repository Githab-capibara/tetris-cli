//! Тесты системы рекордов.
//!
//! Этот модуль содержит 15 тестов для проверки системы сохранения и таблицы лидеров:
//! - Тесты SaveData (5 тестов)
//! - Тесты Leaderboard (5 тестов)
//! - Тесты хеширования (3 теста)
//! - Тесты валидации (2 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты системы рекордов.

use crate::highscore::{Leaderboard, LeaderboardEntry, SaveData};

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: SaveData
// ============================================================================

/// Тест 1: Проверка создания SaveData из значения
///
/// Проверяет, что SaveData::from_value() создаёт валидный экземпляр.
#[test]
fn test_save_data_from_value() {
    let save = SaveData::from_value(1000);

    // Проверяем через публичный метод verify_and_get_score()
    assert_eq!(
        save.verify_and_get_score(),
        Some(1000),
        "Значение рекорда должно быть 1000"
    );
}

/// Тест 2: Проверка SaveData по умолчанию
///
/// Проверяет, что Default реализация создаёт рекорд со значением 0.
#[test]
fn test_save_data_default() {
    let save = SaveData::default();

    // Проверяем, что значение по умолчанию 0
    assert_eq!(
        save.verify_and_get_score(),
        Some(0),
        "Рекорд по умолчанию должен быть 0"
    );
}

/// Тест 3: Проверка verify_and_get_score с валидным рекордом
///
/// Проверяет, что verify_and_get_score() возвращает правильное значение для валидного рекорда.
#[test]
fn test_save_data_verify_and_get_score_valid() {
    let save = SaveData::from_value(5000);

    let result = save.verify_and_get_score();
    assert_eq!(
        result,
        Some(5000),
        "verify_and_get_score() должен вернуть Some(5000) для валидного рекорда"
    );
}

/// Тест 4: Проверка Clone для SaveData
///
/// Проверяет, что клонирование SaveData создаёт точную копию.
#[test]
fn test_save_data_clone() {
    let original = SaveData::from_value(2500);
    let cloned = original.clone();

    // Проверяем через публичный метод verify_and_get_score()
    assert_eq!(
        original.verify_and_get_score(),
        cloned.verify_and_get_score(),
        "Клонированный рекорд должен совпадать"
    );
}

/// Тест 5: Проверка SaveData с разными значениями
///
/// Проверяет создание рекордов с различными значениями.
#[test]
fn test_save_data_different_values() {
    let values = [0, 100, 500, 1000, 5000, 10000, 99999];

    for &value in values.iter() {
        let save = SaveData::from_value(value);
        assert_eq!(
            save.verify_and_get_score(),
            Some(value),
            "verify_and_get_score() должен вернуть Some({}) для рекорда {}",
            value,
            value
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Leaderboard
// ============================================================================

/// Тест 6: Проверка создания пустой Leaderboard
///
/// Проверяет, что новая таблица лидеров пуста.
#[test]
fn test_leaderboard_empty() {
    let leaderboard = Leaderboard::default();

    assert!(
        leaderboard.is_empty(),
        "Новая таблица лидеров должна быть пустой"
    );
    assert_eq!(leaderboard.len(), 0, "Длина пустой таблицы должна быть 0");
}

/// Тест 7: Проверка добавления рекорда в таблицу
///
/// Проверяет, что add_score() успешно добавляет рекорд.
#[test]
fn test_leaderboard_add_score() {
    let mut leaderboard = Leaderboard::default();

    let added = leaderboard.add_score("Player1".to_string(), 1000);

    assert!(added, "Добавление первого рекорда должно быть успешным");
    assert_eq!(leaderboard.len(), 1, "Таблица должна содержать 1 запись");
}

/// Тест 8: Проверка добавления нескольких рекордов
///
/// Проверяет, что можно добавить несколько рекордов.
#[test]
fn test_leaderboard_add_multiple_scores() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player1".to_string(), 1000);
    leaderboard.add_score("Player2".to_string(), 2000);
    leaderboard.add_score("Player3".to_string(), 1500);

    assert_eq!(leaderboard.len(), 3, "Таблица должна содержать 3 записи");
}

/// Тест 9: Проверка ограничения таблицы лидеров (топ-5)
///
/// Проверяет, что таблица хранит не более 5 рекордов.
#[test]
fn test_leaderboard_max_size() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем 7 рекордов
    for i in 0..7 {
        leaderboard.add_score(format!("Player{}", i), (i + 1) * 100);
    }

    // Таблица должна содержать только 5 лучших
    assert_eq!(
        leaderboard.len(),
        5,
        "Таблица должна содержать максимум 5 записей"
    );

    // Проверяем, что остались только лучшие рекорды
    let entries = leaderboard.get_entries();
    for entry in entries {
        assert!(
            entry.score() >= 300,
            "В таблице должны остаться рекорды от 300 и выше"
        );
    }
}

/// Тест 10: Проверка сортировки таблицы лидеров
///
/// Проверяет, что рекорды сортируются по убыванию.
#[test]
fn test_leaderboard_sorting() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем рекорды в случайном порядке
    leaderboard.add_score("Player3".to_string(), 300);
    leaderboard.add_score("Player1".to_string(), 1000);
    leaderboard.add_score("Player5".to_string(), 500);
    leaderboard.add_score("Player2".to_string(), 2000);
    leaderboard.add_score("Player4".to_string(), 100);

    let entries = leaderboard.get_entries();

    // Проверяем порядок по убыванию
    assert_eq!(entries[0].score(), 2000, "Первый рекорд должен быть 2000");
    assert_eq!(entries[1].score(), 1000, "Второй рекорд должен быть 1000");
    assert_eq!(entries[2].score(), 500, "Третий рекорд должен быть 500");
    assert_eq!(entries[3].score(), 300, "Четвёртый рекорд должен быть 300");
    assert_eq!(entries[4].score(), 100, "Пятый рекорд должен быть 100");
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-13: Хеширование
// ============================================================================

/// Тест 11: Проверка создания LeaderboardEntry с хэшом
///
/// Проверяет, что каждая запись имеет уникальный хэш.
#[test]
fn test_leaderboard_entry_hash() {
    let entry1 = LeaderboardEntry::new("Player1".to_string(), 1000);
    let entry2 = LeaderboardEntry::new("Player2".to_string(), 1000);

    // Хэши должны быть разными из-за разной соли
    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хэши должны отличаться из-за разной соли"
    );
    assert!(!entry1.hash().is_empty(), "Хэш не должен быть пустым");
    assert!(!entry2.hash().is_empty(), "Хэш не должен быть пустым");
}

/// Тест 12: Проверка уникальности соли
///
/// Проверяет, что каждая запись получает уникальную соль.
#[test]
fn test_leaderboard_entry_salt_unique() {
    let entry1 = LeaderboardEntry::new("Player".to_string(), 1000);
    let entry2 = LeaderboardEntry::new("Player".to_string(), 1000);

    // Даже с одинаковыми данными хэши должны быть разными из-за разной соли
    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хэши должны отличаться из-за разной соли"
    );
}

/// Тест 13: Проверка хэширования разных значений
///
/// Проверяет, что разные значения дают разные хэши.
#[test]
fn test_hash_different_values() {
    let entry1 = LeaderboardEntry::new("Player".to_string(), 1000);
    let entry2 = LeaderboardEntry::new("Player".to_string(), 2000);

    // Хэши должны быть разными из-за разных очков
    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хэши должны отличаться для разных очков"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 14-15: Валидация
// ============================================================================

/// Тест 14: Проверка валидации LeaderboardEntry
///
/// Проверяет, что валидная запись проходит проверку is_valid().
#[test]
fn test_leaderboard_entry_validation() {
    let entry = LeaderboardEntry::new("Player".to_string(), 1000);

    assert!(
        entry.is_valid(),
        "Валидная запись должна проходить проверку"
    );
}

/// Тест 15: Проверка get_best_score
///
/// Проверяет, что get_best_score() возвращает правильный рекорд.
#[test]
fn test_leaderboard_get_best_score() {
    let mut leaderboard = Leaderboard::default();

    // Пустая таблица
    assert_eq!(
        leaderboard.get_best_score(),
        0,
        "Лучший рекорд пустой таблицы должен быть 0"
    );

    // Добавляем рекорды
    leaderboard.add_score("Player1".to_string(), 1000);
    assert_eq!(
        leaderboard.get_best_score(),
        1000,
        "Лучший рекорд должен быть 1000"
    );

    leaderboard.add_score("Player2".to_string(), 2000);
    assert_eq!(
        leaderboard.get_best_score(),
        2000,
        "Лучший рекорд должен быть 2000"
    );

    leaderboard.add_score("Player3".to_string(), 500);
    assert_eq!(
        leaderboard.get_best_score(),
        2000,
        "Лучший рекорд должен остаться 2000"
    );
}
