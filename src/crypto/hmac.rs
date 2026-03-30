//! Модуль HMAC подписи и верификации.
//!
//! # Ответственность
//! - Централизованное управление HMAC операциями
//! - Предоставление единого API для подписи и проверки данных
//! - Устранение дублирования HMAC логики
//!
//! # Использование
//! ```ignore
//! use tetris_cli::crypto::hmac::{hmac_sign, hmac_verify};
//!
//! let signature = hmac_sign("key", "data");
//! assert!(hmac_verify("key", "data", &signature));
//! ```

use crate::crypto::{hmac_sha256, verify_hmac_sha256};

/// Вычислить HMAC подпись данных.
///
/// # Аргументы
/// * `key` - секретный ключ для HMAC
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка HMAC-SHA256 подписи (64 символа)
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::hmac_sign;
///
/// let signature = hmac_sign("secret_key", "data_to_sign");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Безопасность
/// Используется криптографически стойкий HMAC-SHA256 согласно RFC 2104.
#[must_use = "HMAC подпись должна быть использована для проверки"]
pub fn hmac_sign(key: &str, data: &str) -> String {
    hmac_sha256(key, data)
}

/// Проверить HMAC подпись данных.
///
/// # Аргументы
/// * `key` - секретный ключ для HMAC
/// * `data` - данные для проверки
/// * `signature` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::{hmac_sign, hmac_verify};
///
/// let key = "secret_key";
/// let data = "data_to_verify";
/// let signature = hmac_sign(key, data);
/// assert!(hmac_verify(key, data, &signature));
/// ```
///
/// # Безопасность
/// Используется постоянное по времени сравнение для предотвращения timing-атак.
#[must_use = "Результат проверки должен быть использован"]
pub fn hmac_verify(key: &str, data: &str, signature: &str) -> bool {
    verify_hmac_sha256(key, data, signature)
}

/// Вычислить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `salt` - соль для уникальности
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка HMAC-SHA256 подписи
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::hmac_sign_with_salt;
///
/// let signature = hmac_sign_with_salt("key", "salt", "data");
/// ```
#[must_use = "HMAC подпись должна быть использована для проверки"]
pub fn hmac_sign_with_salt(key: &str, salt: &str, data: &str) -> String {
    let salted_data = format!("{salt}{data}");
    hmac_sha256(key, &salted_data)
}

/// Проверить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `salt` - соль для уникальности
/// * `data` - данные для проверки
/// * `signature` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};
///
/// let key = "secret";
/// let salt = "random_salt";
/// let data = "data";
/// let signature = hmac_sign_with_salt(key, salt, data);
/// assert!(hmac_verify_with_salt(key, salt, data, &signature));
/// ```
#[must_use = "Результат проверки должен быть использован"]
pub fn hmac_verify_with_salt(key: &str, salt: &str, data: &str, signature: &str) -> bool {
    let salted_data = format!("{salt}{data}");
    verify_hmac_sha256(key, &salted_data, signature)
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod hmac_tests {
    use super::*;

    #[test]
    fn test_hmac_sign_basic() {
        let signature = hmac_sign("test_key", "test_data");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    #[test]
    fn test_hmac_sign_deterministic() {
        let sig1 = hmac_sign("key", "data");
        let sig2 = hmac_sign("key", "data");
        assert_eq!(sig1, sig2, "HMAC подписи должны быть детерминированными");
    }

    #[test]
    fn test_hmac_verify_valid() {
        let key = "test_key";
        let data = "test_data";
        let signature = hmac_sign(key, data);

        assert!(
            hmac_verify(key, data, &signature),
            "Подпись должна быть валидной"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_key() {
        let key = "key1";
        let data = "data";
        let signature = hmac_sign(key, data);

        assert!(
            !hmac_verify("key2", data, &signature),
            "Невалидный ключ должен возвращать false"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_data() {
        let key = "key";
        let data = "data1";
        let signature = hmac_sign(key, data);

        assert!(
            !hmac_verify(key, "data2", &signature),
            "Невалидные данные должны возвращать false"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_signature() {
        let key = "key";
        let data = "data";

        assert!(
            !hmac_verify(key, data, "invalid_signature"),
            "Невалидная подпись должна возвращать false"
        );
    }

    #[test]
    fn test_hmac_sign_with_salt_basic() {
        let signature = hmac_sign_with_salt("key", "salt", "data");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    #[test]
    fn test_hmac_sign_with_salt_deterministic() {
        let sig1 = hmac_sign_with_salt("key", "salt", "data");
        let sig2 = hmac_sign_with_salt("key", "salt", "data");
        assert_eq!(
            sig1, sig2,
            "HMAC подписи с солью должны быть детерминированными"
        );
    }

    #[test]
    fn test_hmac_verify_with_salt_valid() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let signature = hmac_sign_with_salt(key, salt, data);

        assert!(
            hmac_verify_with_salt(key, salt, data, &signature),
            "Подпись с солью должна быть валидной"
        );
    }

    #[test]
    fn test_hmac_verify_with_salt_invalid() {
        let key = "key";
        let salt = "salt";
        let data = "data";
        let signature = hmac_sign_with_salt(key, salt, data);

        assert!(
            !hmac_verify_with_salt(key, "wrong_salt", data, &signature),
            "Невалидная соль должна возвращать false"
        );
    }

    #[test]
    fn test_hmac_empty_inputs() {
        // Пустой ключ
        let sig = hmac_sign("", "data");
        assert_eq!(sig.len(), 64);

        // Пустые данные
        let sig = hmac_sign("key", "");
        assert_eq!(sig.len(), 64);

        // Пустая соль
        let sig = hmac_sign_with_salt("key", "", "data");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_hmac_unicode_inputs() {
        let signature = hmac_sign("ключ", "данные с Unicode: 你好🎮");
        assert_eq!(signature.len(), 64);

        let signature = hmac_sign_with_salt("ключ", "соль", "данные: 你好🎮");
        assert_eq!(signature.len(), 64);
    }
}
