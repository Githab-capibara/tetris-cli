//! Тесты оптимизации get_random_hash() (highscore.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка уникальности хэшей
//! - Проверка длины результата (64 символа)
//! - Проверка что только hex символы
//!
//! Исправление: использование String::with_capacity(64) + write!() вместо format!()

use crate::highscore::get_random_hash;

// ============================================================================
// ГРУППА ТЕСТОВ: Оптимизация get_random_hash()
// ============================================================================

/// Тест 1: Проверка уникальности хэшей
///
/// Проверяет, что get_random_hash() генерирует уникальные хэши.
#[test]
fn test_random_hash_uniqueness() {
    // Генерируем 100 хэшей и проверяем что они уникальны
    let mut hashes = Vec::new();

    for _ in 0..100 {
        let hash = get_random_hash();
        assert!(!hashes.contains(&hash), "Хэш должен быть уникальным");
        hashes.push(hash);
    }

    // Проверяем что все 100 хэшей уникальны
    assert_eq!(hashes.len(), 100, "Должно быть 100 уникальных хэшей");

    // Дополнительная проверка: генерируем ещё 100 хэшей
    let mut more_hashes = Vec::new();
    for _ in 0..100 {
        let hash = get_random_hash();
        more_hashes.push(hash);
    }

    // Проверяем что новые хэши тоже уникальны
    let unique_count = more_hashes.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(unique_count, 100, "Все 100 новых хэшей должны быть уникальны");
}

/// Тест 2: Проверка длины результата (64 символа)
///
/// Проверяет, что get_random_hash() возвращает строку из 64 hex символов.
#[test]
fn test_random_hash_length() {
    // Генерируем несколько хэшей и проверяем длину
    for _ in 0..10 {
        let hash = get_random_hash();
        assert_eq!(hash.len(), 64, "Длина хэша должна быть 64 символа");
    }

    // Проверяем что длина соответствует 32 байтам (64 hex символа)
    let hash = get_random_hash();
    assert_eq!(hash.len(), 64, "Хэш должен содержать 64 символа");

    // 64 hex символа = 32 байта = 256 бит
    assert_eq!(
        hash.len() / 2,
        32,
        "Хэш должен содержать 32 байта (256 бит)"
    );
}

/// Тест 3: Проверка что только hex символы
///
/// Проверяет, что get_random_hash() возвращает только шестнадцатеричные символы.
#[test]
fn test_random_hash_hex_only() {
    // Генерируем несколько хэшей и проверяем символы
    for _ in 0..10 {
        let hash = get_random_hash();

        // Проверяем что каждый символ - hex
        for (i, c) in hash.chars().enumerate() {
            assert!(
                c.is_ascii_hexdigit(),
                "Символ {} в позиции {} должен быть hex (0-9, a-f)",
                c,
                i
            );
        }

        // Проверяем что все символы в нижнем регистре
        assert_eq!(
            hash,
            hash.to_lowercase(),
            "Хэш должен быть в нижнем регистре"
        );

        // Проверяем что нет символов верхнего регистра
        assert!(
            !hash.chars().any(|c| c.is_ascii_uppercase()),
            "Хэш не должен содержать заглавные буквы"
        );
    }

    // Дополнительная проверка: все символы должны быть 0-9 или a-f
    let hash = get_random_hash();
    for c in hash.chars() {
        assert!(
            ('0'..='9').contains(&c) || ('a'..='f').contains(&c),
            "Символ {} должен быть 0-9 или a-f",
            c
        );
    }
}

/// Тест 4: Проверка производительности оптимизации
///
/// Сравнивает производительность с предварительным выделением памяти.
#[test]
fn test_random_hash_performance() {
    use std::time::Instant;

    let iterations = 1000;

    // Замеряем время генерации хэшей
    let start = Instant::now();
    for _ in 0..iterations {
        let hash = get_random_hash();
        assert_eq!(hash.len(), 64);
    }
    let elapsed = start.elapsed();

    // Проверяем что генерация работает быстро
    assert!(
        elapsed.as_millis() < 1000,
        "Генерация {} хэшей должна занять меньше 1 секунды",
        iterations
    );

    // Выводим среднее время на хэш
    let avg_time = elapsed / iterations;
    println!("Среднее время генерации хэша: {:?}", avg_time);

    // Проверяем что среднее время разумное (< 1мс на хэш)
    assert!(
        avg_time.as_micros() < 1000,
        "Среднее время должно быть меньше 1мс"
    );
}

/// Тест 5: Проверка распределения символов
///
/// Проверяет что хэш содержит равномерное распределение hex символов.
#[test]
fn test_random_hash_distribution() {
    // Генерируем много хэшей и проверяем распределение
    let mut char_counts = [0usize; 16]; // 0-9, a-f

    for _ in 0..100 {
        let hash = get_random_hash();
        for c in hash.chars() {
            let index = match c {
                '0'..='9' => c as usize - '0' as usize,
                'a'..='f' => c as usize - 'a' as usize + 10,
                _ => panic!("Невалидный hex символ: {}", c),
            };
            char_counts[index] += 1;
        }
    }

    // Проверяем что все символы встретились хотя бы несколько раз
    for (i, &count) in char_counts.iter().enumerate() {
        assert!(
            count > 10,
            "Символ {} должен встретиться хотя бы 10 раз (встретился {})",
            i,
            count
        );
    }

    // Проверяем что распределение относительно равномерное
    let total: usize = char_counts.iter().sum();
    let expected_per_char = total / 16;

    for (i, &count) in char_counts.iter().enumerate() {
        let deviation = (count as i32 - expected_per_char as i32).abs();
        assert!(
            deviation < (expected_per_char as i32),
            "Символ {} имеет слишком большое отклонение: {} (ожидалось {})",
            i,
            count,
            expected_per_char
        );
    }
}
