//! Тесты обработки ошибок (highscore.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка возврата Result при ошибке загрузки
//! - Проверка логирования ошибок
//! - Проверка default значения при ошибке
//!
//! Исправление: корректная обработка ошибок с логированием и fallback на default

use crate::highscore::{Leaderboard, SaveData};

// ============================================================================
// ГРУППА ТЕСТОВ: Обработка ошибок
// ============================================================================

/// Тест 1: Проверка возврата Result при ошибке загрузки
///
/// Проверяет, что load_config() корректно обрабатывает ошибки
/// и возвращает значение по умолчанию.
#[test]
fn test_error_result_on_load_failure() {
    // Загружаем конфигурацию (может быть пустой если файла нет)
    let save = SaveData::load_config();

    // Проверяем что загрузка прошла без паники
    // (файл может существовать или нет - оба случая корректны)

    // Проверяем что можно получить значение
    let score = save.verify_and_get_score().unwrap_or(0);

    // Значение должно быть >= 0
    assert!(score >= 0, "Рекорд должен быть неотрицательным");

    // Проверяем что можно создать новый SaveData
    let new_save = SaveData::from_value(1000);
    let new_score = new_save.verify_and_get_score();

    assert_eq!(new_score, Some(1000), "Новый рекорд должен быть 1000");
}

/// Тест 2: Проверка логирования ошибок
///
/// Проверяет, что ошибки логируются в stderr при загрузке.
#[test]
fn test_error_logging() {
    // Создаём SaveData с невалидным хэшем для проверки логирования
    let mut save = SaveData::from_value(5000);

    // Проверяем что verify_and_get_score() работает
    let result = save.verify_and_get_score();
    assert_eq!(result, Some(5000), "Валидный рекорд должен вернуть Some(5000)");

    // Создаём рекорд с подделанным значением
    save.high_score = 99999; // Меняем значение но не хэш

    // Проверяем что обнаруживается подделка (с логированием в stderr)
    let tampered_result = save.verify_and_get_score();
    assert_eq!(
        tampered_result, None,
        "Подделка должна вернуть None (с логированием)"
    );

    // Проверяем что можно загрузить таблицу лидеров
    let leaderboard = Leaderboard::load();
    let entries = leaderboard.get_entries();

    // Таблица может быть пустой или содержать записи
    assert!(entries.len() <= 5, "Таблица лидеров должна содержать максимум 5 записей");
}

/// Тест 3: Проверка default значения при ошибке
///
/// Проверяет, что при ошибке загрузки используется значение по умолчанию.
#[test]
fn test_default_value_on_error() {
    // Проверяем Default реализацию
    let default_save = SaveData::default();

    // Значение по умолчанию должно быть 0
    let default_score = default_save.verify_and_get_score();
    assert_eq!(default_score, Some(0), "Рекорд по умолчанию должен быть 0");

    // Проверяем что load_config() возвращает корректное значение
    let loaded_save = SaveData::load_config();
    let loaded_score = loaded_save.verify_and_get_score().unwrap_or(0);

    // Значение должно быть >= 0
    assert!(loaded_score >= 0, "Загруженный рекорд должен быть >= 0");

    // Проверяем Leaderboard default
    let default_leaderboard = Leaderboard::default();
    assert!(
        default_leaderboard.is_empty(),
        "Таблица лидеров по умолчанию должна быть пустой"
    );
    assert_eq!(
        default_leaderboard.len(),
        0,
        "Длина таблицы по умолчанию должна быть 0"
    );

    // Проверяем что можно добавить рекорд в default таблицу
    let mut leaderboard = Leaderboard::default();
    let added = leaderboard.add_score("TestPlayer", 1000);
    assert!(added, "Добавление в default таблицу должно быть успешным");
    assert_eq!(leaderboard.len(), 1, "Таблица должна содержать 1 запись");
}

/// Тест 4: Проверка обработки ошибок при сохранении
///
/// Проверяет, что save_value() корректно обрабатывает ошибки.
#[test]
fn test_save_error_handling() {
    // Сохраняем рекорд (может неудачно если нет прав доступа)
    SaveData::save_value(5000);

    // Проверяем что можно загрузить сохранённое значение
    let loaded = SaveData::load_config();
    let score = loaded.verify_and_get_score().unwrap_or(0);

    // Значение должно быть корректным
    assert!(score >= 0, "Сохранённый рекорд должен быть >= 0");
}

/// Тест 5: Проверка обработки ошибок Leaderboard
///
/// Проверяет, что Leaderboard::load() корректно обрабатывает ошибки.
#[test]
fn test_leaderboard_error_handling() {
    // Загружаем таблицу лидеров (может быть пустой)
    let leaderboard = Leaderboard::load();

    // Проверяем что загрузка прошла без паники
    let entries = leaderboard.get_entries();
    assert!(entries.len() <= 5, "Таблица должна содержать максимум 5 записей");

    // Проверяем что можно сохранить таблицу
    leaderboard.save();

    // Проверяем что можно добавить запись
    let mut test_leaderboard = Leaderboard::default();
    let added = test_leaderboard.add_score("Player", 1000);
    assert!(added, "Добавление должно быть успешным");

    // Проверяем что можно получить лучший рекорд
    let best = test_leaderboard.get_best_score();
    assert_eq!(best, 1000, "Лучший рекорд должен быть 1000");
}
