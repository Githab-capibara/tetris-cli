//! Тесты исправления конвертации байтов в hex (highscore.rs:37).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Замена U256::from_le_bytes на bytes.iter().map()
//! - Проверка корректной генерации hex строк
//! - Проверка уникальности и формата солей
//!
//! Исправление обеспечивает корректную конвертацию 32 байт в 64 hex символа.

use crate::highscore::{get_random_hash, LeaderboardEntry, SaveData};

// ============================================================================
// ГРУППА ТЕСТОВ: Исправление конвертации байтов в hex
// ============================================================================

/// Тест 1: Проверка корректной конвертации байтов в hex строку
///
/// Проверяет, что get_random_hash() возвращает строку из ровно 64 hex символов.
/// Все символы должны быть lowercase hex digit (0-9, a-f).
#[test]
fn test_корректная_конвертация_байт_в_hex() {
    // Генерируем хеш
    let hash = get_random_hash();

    // Проверяем длину (32 байта * 2 hex символа = 64 символа)
    assert_eq!(
        hash.len(),
        64,
        "Длина хеша должна быть ровно 64 символа (32 байта в hex)"
    );

    // Проверяем что все символы - lowercase hex цифры
    for (i, c) in hash.chars().enumerate() {
        assert!(
            c.is_ascii_hexdigit(),
            "Символ {} должен быть hex цифрой (0-9, a-f)",
            i
        );
        assert!(
            !c.is_ascii_uppercase(),
            "Символ '{}' не должен быть заглавным (требуется lowercase)",
            c
        );
    }
}

/// Тест 2: Проверка уникальности генерируемых хешей
///
/// Проверяет, что каждый вызов get_random_hash() возвращает уникальное значение.
/// Генерируем 100 хешей и проверяем что все они уникальны.
#[test]
fn test_уникальность_генерируемых_хешей() {
    const NUM_HASHES: usize = 100;
    let mut hashes = Vec::with_capacity(NUM_HASHES);

    // Генерируем 100 хешей
    for _ in 0..NUM_HASHES {
        let hash = get_random_hash();
        hashes.push(hash);
    }

    // Проверяем что все хеши уникальны
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Хеши {} и {} совпали - генератор не уникален",
                i, j
            );
        }
    }
}

/// Тест 3: Проверка использования соли в SaveData и LeaderboardEntry
///
/// Проверяет, что при создании SaveData и LeaderboardEntry
/// используются уникальные соли, даже для одинаковых данных.
#[test]
fn test_использование_соли_в_записях() {
    // Создаём два одинаковых SaveData
    let save1 = SaveData::from_value(1000);
    let save2 = SaveData::from_value(1000);

    // Значения должны совпадать (оба 1000)
    assert_eq!(
        save1.assert_hs(),
        save2.assert_hs(),
        "Значения должны совпадать (оба 1000)"
    );

    // Создаём два одинаковых LeaderboardEntry
    let entry1 = LeaderboardEntry::new("Player".to_string(), 5000);
    let entry2 = LeaderboardEntry::new("Player".to_string(), 5000);

    // Очки должны совпадать
    assert_eq!(entry1.score(), entry2.score(), "Очки должны совпадать");

    // Но хеши должны быть разными из-за разной соли
    assert_ne!(
        entry1.hash(),
        entry2.hash(),
        "Хеши должны быть разными из-за уникальной соли"
    );

    // Проверяем что обе записи валидны
    assert!(entry1.is_valid(), "Первая запись должна быть валидной");
    assert!(entry2.is_valid(), "Вторая запись должна быть валидной");
}
