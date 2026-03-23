//! Криптографические утилиты.
//!
//! Этот модуль предоставляет общие криптографические функции для проекта:
//! - Хэширование BLAKE3
//! - Генерация случайной соли
//! - HMAC-подобные конструкции
//!
//! ## Пример использования
//! ```
//! use tetris_cli::crypto::{hash, generate_salt, hmac};
//!
//! // Хэширование
//! let data = "тестовые данные";
//! let h = hash(data);
//! assert_eq!(h.len(), 64); // 256 бит в hex
//!
//! // Генерация соли
//! let salt = generate_salt();
//! assert_eq!(salt.len(), 64); // 32 байта = 64 hex символа
//!
//! // HMAC
//! let key = "секретный ключ";
//! let signature = hmac(key, data);
//! ```

use rand::rngs::OsRng;
use rand::RngCore;

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
    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Вычислить HMAC (ключ + данные) используя BLAKE3.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка из 64 символов (256 бит)
///
/// # Пример
/// ```
/// use tetris_cli::crypto::hmac;
/// let signature = hmac("ключ", "данные");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Исправление #4
/// Функция помечена как `#[doc(hidden)]` так как используется только в тестах
/// и не предназначена для публичного использования.
#[must_use = "HMAC должен быть использован для проверки"]
#[doc(hidden)]
#[allow(dead_code)]
pub fn hmac(key: &str, data: &str) -> String {
    // Формируем ключ + данные для хеширования
    hash(&(key.to_string() + data))
}

/// Проверить HMAC подпись.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `data` - данные
/// * `expected_hmac` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
///
/// # Пример
/// ```
/// use tetris_cli::crypto::{hmac, verify_hmac};
/// let key = "секрет";
/// let data = "данные";
/// let signature = hmac(key, data);
/// assert!(verify_hmac(key, data, &signature));
/// ```
///
/// # Исправление #4
/// Функция помечена как `#[doc(hidden)]` так как используется только в тестах
/// и не предназначена для публичного использования.
#[must_use = "Результат проверки должен быть использован"]
#[doc(hidden)]
#[allow(dead_code)]
pub fn verify_hmac(key: &str, data: &str, expected_hmac: &str) -> bool {
    hmac(key, data) == expected_hmac
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

    #[test]
    fn test_hmac_deterministic() {
        let sig1 = hmac("ключ", "данные");
        let sig2 = hmac("ключ", "данные");
        assert_eq!(sig1, sig2, "HMAC должен быть детерминированным");
    }

    #[test]
    fn test_hmac_different_keys() {
        let sig1 = hmac("ключ1", "данные");
        let sig2 = hmac("ключ2", "данные");
        assert_ne!(sig1, sig2, "Разные ключи должны давать разные HMAC");
    }

    #[test]
    fn test_hmac_different_data() {
        let sig1 = hmac("ключ", "данные1");
        let sig2 = hmac("ключ", "данные2");
        assert_ne!(sig1, sig2, "Разные данные должны давать разные HMAC");
    }

    #[test]
    fn test_verify_hmac_valid() {
        let key = "тестовый ключ";
        let data = "тестовые данные";
        let signature = hmac(key, data);
        assert!(
            verify_hmac(key, data, &signature),
            "Правильная подпись должна проходить проверку"
        );
    }

    #[test]
    fn test_verify_hmac_invalid_key() {
        let key = "ключ1";
        let data = "данные";
        let signature = hmac(key, data);
        assert!(
            !verify_hmac("ключ2", data, &signature),
            "Неправильный ключ не должен проходить проверку"
        );
    }

    #[test]
    fn test_verify_hmac_invalid_data() {
        let key = "ключ";
        let data = "данные1";
        let signature = hmac(key, data);
        assert!(
            !verify_hmac(key, "данные2", &signature),
            "Неправильные данные не должны проходить проверку"
        );
    }

    #[test]
    fn test_verify_hmac_invalid_signature() {
        let key = "ключ";
        let data = "данные";
        assert!(
            !verify_hmac(key, data, "неправильная подпись"),
            "Неправильная подпись не должна проходить проверку"
        );
    }
}
