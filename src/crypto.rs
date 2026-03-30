//! Криптографические утилиты.
//!
//! Модуль предоставляет криптографические функции для защиты данных:
//! - `hash()` - хеширование BLAKE3 (быстрое и безопасное)
//! - `generate_salt()` - генерация криптографически стойкой соли
//! - `validator` - модуль валидации HMAC подписей
//!
//! # Безопасность
//!
//! ## Защита от подделки данных
//! Модуль использует HMAC-SHA256 для криптографической подписи данных.
//! Это обеспечивает:
//! - **Целостность данных**: любое изменение данных будет обнаружено
//! - **Аутентичность**: только владелец ключа может создать.valid подпись
//! - **Неотказуемость**: подписанные данные нельзя отвергнуть
//!
//! ## Защита от timing-атак
//! Функция `verify_hmac_sha256()` использует постоянное по времени сравнение
//! через XOR накопление. Это предотвращает атаки по времени выполнения,
//! где злоумышленник может определить правильную подпись по времени ответа.
//!
//! ## Генерация соли
//! `generate_salt()` использует криптографически стойкий генератор случайных
//! чисел (StdRng::from_os_rng) для создания уникальной соли 256 бит.
//!
//! ## Рекомендации по использованию
//! - Храните секретные ключи в переменных окружения, не в коде
//! - Используйте уникальную соль для каждого набора данных
//! - Регулярно обновляйте ключи HMAC
//! - Не используйте один ключ для разных целей

// Подмодули
pub mod hmac;
pub mod validator;

// Ре-экспорт основных функций из hmac для удобства
pub use hmac::{hmac_sign, hmac_sign_with_salt, hmac_verify, hmac_verify_with_salt};

// Ре-экспорт основных функций из validator для удобства
pub use validator::{sign_salt_and_data, verify_salt_and_data};

use ::hmac::{Hmac, Mac};
use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;
use sha2::Sha256;

/// Тип HMAC-SHA256.
type HmacSha256 = Hmac<Sha256>;

/// Вычислить BLAKE3 хеш строки.
///
/// # Аргументы
/// * `data` - данные для хеширования
///
/// # Возвращает
/// Hex-строка из 64 символов (256 бит = 32 байта)
///
/// # Пример
/// ```
/// use tetris_cli::crypto::hash;
/// let h = hash("данные");
/// assert_eq!(h.len(), 64);
/// ```
#[must_use = "Хеш должен быть использован"]
pub fn hash(data: &str) -> String {
    blake3::hash(data.as_bytes()).to_hex().to_string()
}

/// Сгенерировать случайную соль из 64 шестнадцатеричных символов (256 бит).
///
/// Использует криптографически стойкий генератор случайных чисел (`OsRng`).
///
/// # Возвращает
/// Hex-строка из 64 символов (32 байта = 256 бит)
///
/// # Пример
/// ```
/// use tetris_cli::crypto::generate_salt;
/// let salt = generate_salt();
/// assert_eq!(salt.len(), 64);
/// ```
#[must_use = "Соль должна быть использована для хеширования"]
pub fn generate_salt() -> String {
    // Используем StdRng с случайным seed от OS
    let mut rng = StdRng::from_os_rng();
    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
    RngCore::fill_bytes(&mut rng, &mut bytes);
    hex::encode(bytes)
}

/// Вычислить HMAC-SHA256 подпись данных.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка из 64 символов (256 бит = 32 байта HMAC)
///
/// # Panics
/// Паникует если ключ не может быть использован (крайне маловероятно, т.к. HMAC поддерживает ключи любой длины)
///
/// # Пример
/// ```
/// use tetris_cli::crypto::hmac_sha256;
/// let signature = hmac_sha256("ключ", "данные");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Безопасность
/// Используется криптографически стойкий HMAC-SHA256 согласно RFC 2104.
/// Это обеспечивает надёжную защиту от подделки данных.
///
/// # Исправление #4 (ВЫСОКИЙ ПРИОРИТЕТ)
/// Функция использует настоящий HMAC-SHA256 вместо простой конкатенации.
#[allow(clippy::missing_panics_doc)]
#[must_use = "HMAC подпись должна быть использована для проверки"]
pub fn hmac_sha256(key: &str, data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC может принимать ключ любой длины");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Проверить HMAC-SHA256 подпись.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `data` - данные
/// * `expected_hash` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
///
/// # Пример
/// ```
/// use tetris_cli::crypto::{hmac_sha256, verify_hmac_sha256};
/// let key = "секрет";
/// let data = "данные";
/// let signature = hmac_sha256(key, data);
/// assert!(verify_hmac_sha256(key, data, &signature));
/// ```
///
/// # Безопасность
/// Используется постоянное по времени сравнение для предотвращения timing-атак.
#[must_use = "Результат проверки должен быть использован"]
pub fn verify_hmac_sha256(key: &str, data: &str, expected_hash: &str) -> bool {
    let actual_hash = hmac_sha256(key, data);
    // Исправление #2 (CRITICAL): постоянное по времени сравнение через XOR накопление
    // Предотвращает timing-атаки путём выполнения одинакового количества операций
    // независимо от позиции первого несовпадающего байта
    let actual_bytes = actual_hash.as_bytes();
    let expected_bytes = expected_hash.as_bytes();

    // Проверяем длину - разная длина сразу возвращает false
    if actual_bytes.len() != expected_bytes.len() {
        return false;
    }

    // XOR накопление - выполняем сравнение за постоянное время
    let mut result: u8 = 0;
    for (a, b) in actual_bytes.iter().zip(expected_bytes.iter()) {
        result |= a ^ b;
    }

    result == 0
}

#[cfg(test)]
mod crypto_tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let h1 = hash("тест");
        let h2 = hash("тест");
        assert_eq!(h1, h2, "Хеш должен быть детерминированным");
    }

    #[test]
    fn test_hash_different_inputs() {
        let h1 = hash("тест1");
        let h2 = hash("тест2");
        assert_ne!(h1, h2, "Разные данные должны давать разные хеши");
    }

    #[test]
    fn test_hash_length() {
        let h = hash("данные");
        assert_eq!(h.len(), 64, "Длина хеша должна быть 64 символа (256 бит)");
    }

    #[test]
    fn test_generate_salt_unique() {
        let s1 = generate_salt();
        let s2 = generate_salt();
        assert_ne!(s1, s2, "Соли должны быть уникальными");
    }

    #[test]
    fn test_generate_salt_length() {
        let salt = generate_salt();
        assert_eq!(
            salt.len(),
            64,
            "Длина соли должна быть 64 символа (256 бит)"
        );
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ HMAC-SHA256 (ИСПРАВЛЕНИЕ #4)
    // =========================================================================

    #[test]
    fn test_hmac_sha256_deterministic() {
        let sig1 = hmac_sha256("ключ", "данные");
        let sig2 = hmac_sha256("ключ", "данные");
        assert_eq!(sig1, sig2, "HMAC-SHA256 должен быть детерминированным");
    }

    #[test]
    fn test_hmac_sha256_different_keys() {
        let sig1 = hmac_sha256("ключ1", "данные");
        let sig2 = hmac_sha256("ключ2", "данные");
        assert_ne!(sig1, sig2, "Разные ключи должны давать разные HMAC");
    }

    #[test]
    fn test_hmac_sha256_different_data() {
        let sig1 = hmac_sha256("ключ", "данные1");
        let sig2 = hmac_sha256("ключ", "данные2");
        assert_ne!(sig1, sig2, "Разные данные должны давать разные HMAC");
    }

    #[test]
    fn test_hmac_sha256_length() {
        let signature = hmac_sha256("ключ", "данные");
        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC-SHA256 должна быть 64 символа (256 бит)"
        );
    }

    #[test]
    fn test_verify_hmac_sha256_valid() {
        let key = "тестовый ключ";
        let data = "тестовые данные";
        let signature = hmac_sha256(key, data);
        assert!(
            verify_hmac_sha256(key, data, &signature),
            "Правильная HMAC подпись должна проходить проверку"
        );
    }

    #[test]
    fn test_verify_hmac_sha256_invalid_key() {
        let key = "ключ1";
        let data = "данные";
        let signature = hmac_sha256(key, data);
        assert!(
            !verify_hmac_sha256("ключ2", data, &signature),
            "Неправильный ключ не должен проходить проверку HMAC"
        );
    }

    #[test]
    fn test_verify_hmac_sha256_invalid_data() {
        let key = "ключ";
        let data = "данные1";
        let signature = hmac_sha256(key, data);
        assert!(
            !verify_hmac_sha256(key, "данные2", &signature),
            "Неправильные данные не должны проходить проверку HMAC"
        );
    }

    #[test]
    fn test_verify_hmac_sha256_invalid_signature() {
        let key = "ключ";
        let data = "данные";
        assert!(
            !verify_hmac_sha256(key, data, "неправильная подпись"),
            "Неправильная HMAC подпись не должна проходить проверку"
        );
    }

    // =========================================================================
    // ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ ДЛЯ HMAC-SHA256 (ИСПРАВЛЕНИЕ #4)
    // =========================================================================

    /// Тест на HMAC-SHA256 с пустыми данными
    #[test]
    fn test_hmac_sha256_empty_data() {
        let signature = hmac_sha256("ключ", "");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");

        // Проверяем детерминированность
        let signature2 = hmac_sha256("ключ", "");
        assert_eq!(signature, signature2);
    }

    /// Тест на HMAC-SHA256 с пустым ключом
    #[test]
    fn test_hmac_sha256_empty_key() {
        let signature = hmac_sha256("", "данные");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");

        // Проверяем детерминированность
        let signature2 = hmac_sha256("", "данные");
        assert_eq!(signature, signature2);
    }

    /// Тест на HMAC-SHA256 с Unicode символами
    #[test]
    fn test_hmac_sha256_unicode() {
        let signature = hmac_sha256("ключ", "данные с Unicode: 你好🎮");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");

        let signature2 = hmac_sha256("ключ", "данные с Unicode: 你好🎮");
        assert_eq!(
            signature, signature2,
            "HMAC должен быть детерминированным для Unicode"
        );
    }

    /// Тест на HMAC-SHA256 с длинными данными
    #[test]
    fn test_hmac_sha256_long_data() {
        let long_data = "a".repeat(10000);
        let signature = hmac_sha256("ключ", &long_data);
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    /// Тест на HMAC-SHA256 с длинным ключом
    #[test]
    fn test_hmac_sha256_long_key() {
        let long_key = "k".repeat(10000);
        let signature = hmac_sha256(&long_key, "данные");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    /// Тест на verify_hmac_sha256 с изменённой подписью
    #[test]
    fn test_verify_hmac_sha256_tampered_signature() {
        let key = "секретный ключ";
        let data = "важные данные";
        let signature = hmac_sha256(key, data);

        // Изменяем один символ в подписи
        let mut tampered = signature.clone();
        let mut chars: Vec<char> = tampered.chars().collect();
        chars[0] = if chars[0] == 'a' { 'b' } else { 'a' };
        tampered = chars.iter().collect();

        assert!(
            !verify_hmac_sha256(key, data, &tampered),
            "Изменённая подпись не должна проходить проверку"
        );
    }

    /// Тест на HMAC-SHA256 для интеграции с controls.rs
    /// Проверяет что HMAC работает с JSON данными
    #[test]
    fn test_hmac_sha256_json_data() {
        let json_data = r#"{"move_left":97,"move_right":100}"#;
        let key = "test_hmac_key_for_controls";

        let signature = hmac_sha256(key, json_data);
        assert_eq!(signature.len(), 64);
        assert!(verify_hmac_sha256(key, json_data, &signature));
    }

    /// Тест на HMAC-SHA256 для интеграции с save_data.rs
    /// Проверяет что HMAC работает с данными рекордов
    #[test]
    fn test_hmac_sha256_score_data() {
        let score_data = "12345678";
        let salt = "abcdef1234567890";
        let combined = format!("{}{}", salt, score_data);

        let key = "save_data_hmac_key";
        let signature = hmac_sha256(key, &combined);

        assert_eq!(signature.len(), 64);
        assert!(verify_hmac_sha256(key, &combined, &signature));
    }

    /// Тест на постоянное по времени сравнение в verify_hmac_sha256
    #[test]
    fn test_verify_hmac_timing_safe_comparison() {
        let key = "timing_test_key";
        let data = "timing_test_data";
        let signature = hmac_sha256(key, data);

        // Проверяем что сравнение работает корректно
        assert!(verify_hmac_sha256(key, data, &signature));

        // Проверяем что разные длины не вызывают панику
        assert!(!verify_hmac_sha256(key, data, &signature[..32]));
        assert!(!verify_hmac_sha256(key, data, ""));
    }
}
