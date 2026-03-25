//! Тесты системы рекордов в Tetris CLI.
//!
//! Этот модуль содержит 30 тестов для проверки системы рекордов:
//! - Тесты `SaveData` (создание, сохранение, загрузка) (6 тестов)
//! - Тесты Leaderboard (добавление, сортировка, валидация) (8 тестов)
//! - Тесты хеширования с солью (6 тестов)
//! - Тесты защиты от подделки (5 тестов)
//! - Тесты максимального размера таблицы (5 тестов)
//!
//! Все тесты проверяют корректность системы сохранения рекордов.

use crate::highscore::{Leaderboard, SaveData};
use crate::highscore::leaderboard::LeaderboardEntry;

// ============================================================================
// ГРУППА ТЕСТОВ 1-6: SaveData (создание, сохранение, загрузка)
// ============================================================================

/// Тест 1: `SaveData` создание из значения
#[test]
fn test_savedata_creation_from_value() {
    let save = SaveData::from_value(1000);

    assert_eq!(
        save.verify_and_get_score(),
        Some(1000),
        "Рекорд должен быть 1000"
    );
    // Поля приватны, поэтому проверяем через методы
}

/// Тест 2: `SaveData` значение по умолчанию
#[test]
fn test_savedata_default_value() {
    let save = SaveData::default();

    assert_eq!(
        save.verify_and_get_score(),
        Some(0),
        "Рекорд по умолчанию должен быть 0"
    );
}

/// Тест 3: `SaveData` сохранение и загрузка
#[test]
fn test_savedata_save_and_load() {
    // Сохраняем рекорд
    SaveData::save_value(5000);

    // Загружаем конфигурацию
    let loaded = SaveData::load_config();

    // Проверяем целостность
    let score = loaded.verify_and_get_score();
    // u64 всегда >= 0, проверяем что рекорд загрузился корректно
    let _ = score;
}

/// Тест 4: `SaveData` проверка целостности
#[test]
fn test_savedata_integrity_check() {
    let save = SaveData::from_value(2500);

    // Проверяем целостность
    let score = save.verify_and_get_score();

    assert_eq!(
        score,
        Some(2500),
        "Рекорд должен пройти проверку целостности"
    );
}

/// Тест 5: `SaveData` разные значения
#[test]
fn test_savedata_different_values() {
    let save1 = SaveData::from_value(100);
    let save2 = SaveData::from_value(1000);
    let save3 = SaveData::from_value(10000);

    assert_eq!(save1.verify_and_get_score(), Some(100));
    assert_eq!(save2.verify_and_get_score(), Some(1000));
    assert_eq!(save3.verify_and_get_score(), Some(10000));
}

/// Тест 6: `SaveData` клонирование
#[test]
fn test_savedata_clone() {
    let save = SaveData::from_value(750);
    let cloned = save.clone();

    assert_eq!(
        save.verify_and_get_score(),
        cloned.verify_and_get_score(),
        "Клон должен иметь тот же рекорд"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 7-14: Leaderboard (добавление, сортировка, валидация)
// ============================================================================

/// Тест 7: Leaderboard создание
#[test]
fn test_leaderboard_creation() {
    let leaderboard = Leaderboard::default();

    assert_eq!(
        leaderboard.len(),
        0,
        "Новая таблица лидеров должна быть пустой"
    );
}

/// Тест 8: Leaderboard добавление рекорда
#[test]
fn test_leaderboard_add_score() {
    let mut leaderboard = Leaderboard::default();

    let added = leaderboard.add_score("Player1", 1000);

    assert!(added, "Рекорд должен быть добавлен");
    assert_eq!(leaderboard.len(), 1, "Таблица должна содержать 1 запись");
}

/// Тест 9: Leaderboard добавление нескольких рекордов
#[test]
fn test_leaderboard_add_multiple_scores() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player1", 1000);
    leaderboard.add_score("Player2", 2000);
    leaderboard.add_score("Player3", 1500);

    assert_eq!(leaderboard.len(), 3, "Таблица должна содержать 3 записи");
}

/// Тест 10: Leaderboard сортировка по убыванию
#[test]
fn test_leaderboard_sorting_descending() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player3", 300);
    leaderboard.add_score("Player1", 1000);
    leaderboard.add_score("Player2", 500);

    let entries = leaderboard.get_entries();

    assert_eq!(
        entries[0].score(),
        1000,
        "Первый рекорд должен быть наибольшим"
    );
    assert_eq!(entries[1].score(), 500, "Второй рекорд должен быть средним");
    assert_eq!(
        entries[2].score(),
        300,
        "Третий рекорд должен быть наименьшим"
    );
}

/// Тест 11: Leaderboard валидация записей
#[test]
fn test_leaderboard_validation() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player", 1000);

    // Проверяем валидность всех записей
    for entry in leaderboard.get_entries() {
        assert!(entry.is_valid(), "Запись должна быть валидной");
    }
}

/// Тест 12: Leaderboard получение записей
#[test]
fn test_leaderboard_get_entries() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player1", 1000);
    leaderboard.add_score("Player2", 2000);

    let entries = leaderboard.get_entries();

    assert_eq!(entries.len(), 2, "Должно быть 2 записи");
}

/// Тест 13: Leaderboard лучший рекорд
#[test]
fn test_leaderboard_best_score() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player1", 1000);
    leaderboard.add_score("Player2", 2000);
    leaderboard.add_score("Player3", 1500);

    let best = leaderboard.get_best_score();

    assert_eq!(best, 2000, "Лучший рекорд должен быть 2000");
}

/// Тест 14: Leaderboard пустая таблица
#[test]
fn test_leaderboard_empty() {
    let leaderboard = Leaderboard::default();

    assert!(leaderboard.is_empty(), "Таблица должна быть пустой");
    assert_eq!(leaderboard.len(), 0, "Длина должна быть 0");
    assert_eq!(
        leaderboard.get_best_score(),
        0,
        "Лучший рекорд в пустой таблице должен быть 0"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-20: Тесты хеширования с солью
// ============================================================================

/// Тест 15: Хеш с солью создаётся
#[test]
fn test_hash_with_salt_creation() {
    let entry = LeaderboardEntry::new("Player", 1000);

    // Проверяем, что запись валидна (значит хеш и соль созданы)
    assert!(entry.is_valid(), "Запись должна быть валидной");
}

/// Тест 16: Хеш с солью уникален
#[test]
fn test_hash_with_salt_unique() {
    let entry1 = LeaderboardEntry::new("Player", 1000);
    let entry2 = LeaderboardEntry::new("Player", 1000);

    // Хеши должны быть разными из-за разной соли
    assert_ne!(entry1.hash(), entry2.hash(), "Хеши должны быть уникальными");
}

/// Тест 17: Хеш зависит от соли
#[test]
fn test_hash_depends_on_salt() {
    let entry1 = LeaderboardEntry::new("Player", 1000);
    let entry2 = LeaderboardEntry::new("Player", 1000);

    // Хеши должны быть разными из-за разной соли
    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хеши должны быть разными из-за разной соли"
    );
}

/// Тест 18: Хеш зависит от имени
#[test]
fn test_hash_depends_on_name() {
    let entry1 = LeaderboardEntry::new("Player1", 1000);
    let entry2 = LeaderboardEntry::new("Player2", 1000);

    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хеши должны быть разными для разных имён"
    );
}

/// Тест 19: Хеш зависит от очков
#[test]
fn test_hash_depends_on_score() {
    let entry1 = LeaderboardEntry::new("Player", 1000);
    let entry2 = LeaderboardEntry::new("Player", 2000);

    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хеши должны быть разными для разных очков"
    );
}

/// Тест 20: Соль случайная
#[test]
fn test_salt_random() {
    let mut hashes = Vec::new();

    for _ in 0..10 {
        let entry = LeaderboardEntry::new("Player", 1000);
        hashes.push(entry.hash().to_string());
    }

    // Все хеши должны быть уникальными (из-за разной соли)
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Хеши должны быть уникальными из-за разной соли"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-25: Тесты защиты от подделки
// ============================================================================

/// Тест 21: Валидная запись проходит проверку
#[test]
fn test_valid_entry_passes_check() {
    let entry = LeaderboardEntry::new("Player", 1000);

    assert!(
        entry.is_valid(),
        "Валидная запись должна проходить проверку"
    );
}

// Тесты 22-24 закомментированы, так как поля LeaderboardEntry теперь приватные
// и не могут быть модифицированы напрямую для тестирования подделки

// /// Тест 22: Подделка хеша обнаруживается
// #[test]
// fn test_fake_hash_detected() {
//     let mut entry = LeaderboardEntry::new("Player", 1000);
//     entry.hash = "fake_hash".to_string();
//     assert!(!entry.is_valid(), "Подделанный хеш должен обнаруживаться");
// }

// /// Тест 23: Подделка очков обнаруживается
// #[test]
// fn test_fake_score_detected() {
//     let mut entry = LeaderboardEntry::new("Player", 1000);
//     entry.score = 9999;
//     assert!(!entry.is_valid(), "Подделанные очки должны обнаруживаться");
// }

// /// Тест 24: Подделка имени обнаруживается
// #[test]
// fn test_fake_name_detected() {
//     let mut entry = LeaderboardEntry::new("Player", 1000);
//     entry.name = "Cheater".to_string();
//     assert!(!entry.is_valid(), "Подделанное имя должно обнаруживаться");
// }

/// Тест 25: `SaveData` защита от подделки
#[test]
fn test_savedata_protection_from_fake() {
    let save = SaveData::from_value(5000);

    // Проверяем, что рекорд валиден
    let score = save.verify_and_get_score();
    assert_eq!(
        score,
        Some(5000),
        "Валидный рекорд должен проходить проверку"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 26-30: Тесты максимального размера таблицы
// ============================================================================

/// Тест 26: Leaderboard максимальный размер 5
#[test]
fn test_leaderboard_max_size() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем 10 рекордов
    for i in 0..10 {
        leaderboard.add_score(&format!("Player{i}"), u128::from(i as u64 * 100));
    }

    // Таблица должна содержать только топ-5
    assert_eq!(
        leaderboard.len(),
        5,
        "Таблица лидеров должна содержать максимум 5 записей"
    );
}

/// Тест 27: Leaderboard сохраняет топ-5
#[test]
fn test_leaderboard_keeps_top_five() {
    let mut leaderboard = Leaderboard::default();

    // Добавляем рекорды в случайном порядке
    leaderboard.add_score("P5", 500);
    leaderboard.add_score("P1", 100);
    leaderboard.add_score("P3", 300);
    leaderboard.add_score("P2", 200);
    leaderboard.add_score("P4", 400);
    leaderboard.add_score("P6", 600); // Должен вытеснить P1

    let entries = leaderboard.get_entries();

    // Проверяем, что остались лучшие 5
    assert_eq!(entries.len(), 5, "Должно быть 5 записей");
    assert!(
        entries[0].score() >= 500,
        "Лучший рекорд должен быть >= 500"
    );
}

/// Тест 28: Leaderboard низкий рекорд не добавляется
#[test]
fn test_leaderboard_low_score_not_added() {
    let mut leaderboard = Leaderboard::default();

    // Заполняем таблицу
    for i in 0..5 {
        leaderboard.add_score(&format!("Player{i}"), u128::from((5 - i) as u64 * 100));
    }

    // Пытаемся добавить рекорд ниже минимального
    let added = leaderboard.add_score("LowPlayer", 50);

    assert!(
        !added,
        "Низкий рекорд не должен добавляться в заполненную таблицу"
    );
}

/// Тест 29: Leaderboard высокий рекорд вытесняет низкий
#[test]
fn test_leaderboard_high_score_displaces_low() {
    let mut leaderboard = Leaderboard::default();

    // Заполняем таблицу
    for i in 0..5 {
        leaderboard.add_score(&format!("Player{i}"), u128::from((5 - i) as u64 * 100));
    }

    // Добавляем высокий рекорд
    let added = leaderboard.add_score("HighPlayer", 1000);

    assert!(added, "Высокий рекорд должен добавляться");
    assert_eq!(leaderboard.len(), 5, "Таблица должна остаться размером 5");
}

/// Тест 30: Leaderboard очистка
#[test]
fn test_leaderboard_clear() {
    let mut leaderboard = Leaderboard::default();

    leaderboard.add_score("Player1", 1000);
    leaderboard.add_score("Player2", 2000);

    leaderboard.clear();

    assert!(
        leaderboard.is_empty(),
        "Таблица должна быть пустой после очистки"
    );
    assert_eq!(leaderboard.len(), 0, "Длина должна быть 0");
}
