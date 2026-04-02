//! Модуль валидации данных с использованием HMAC.
//!
//! # Ответственность
//! - Валидация HMAC подписей
//! - Проверка целостности данных
//! - Защита от подделки данных
//!
//! # Использование
//! ```ignore
//! use tetris_cli::crypto::validator::{verify_salt_and_data, sign_salt_and_data};
//!
//! let signature = sign_salt_and_data("key", "salt", "data");
//! assert!(verify_salt_and_data("key", "salt", "data", &signature));
//! ```
//!
//! # Исправление ISSUE-043
//! HmacValidator удалён - используйте напрямую hmac_sha256/verify_hmac_sha256.

use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

// ============================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ ДЛЯ ВАЛИДАЦИИ ДАННЫХ
// ============================================================================

/// Проверить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `salt` - соль
/// * `data` - данные
/// * `expected_signature` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::validator::verify_salt_and_data;
///
/// let key = "secret";
/// let salt = "random_salt";
/// let data = "player_score";
/// let signature = hmac_sha256(key, &format!("{}{}", salt, data));
/// assert!(verify_salt_and_data(key, &salt, data, &signature));
/// ```
#[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
#[must_use]
pub fn verify_salt_and_data(key: &str, salt: &str, data: &str, expected_signature: &str) -> bool {
    let salt_and_data = format!("{salt}{data}");
    verify_hmac_sha256(key, &salt_and_data, expected_signature)
}

/// Вычислить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `salt` - соль
/// * `data` - данные
///
/// # Возвращает
/// Hex-строка HMAC-SHA256 подписи
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::validator::sign_salt_and_data;
///
/// let signature = sign_salt_and_data("key", "salt", "data");
/// ```
#[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
#[must_use]
pub fn sign_salt_and_data(key: &str, salt: &str, data: &str) -> String {
    let salt_and_data = format!("{salt}{data}");
    hmac_sha256(key, &salt_and_data)
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod validator_tests {
    use super::*;

    // Тесты для вспомогательных функций
    #[test]
    fn test_verify_salt_and_data() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let signature = sign_salt_and_data(key, salt, data);

        assert!(verify_salt_and_data(key, salt, data, &signature));
    }

    #[test]
    fn test_verify_salt_and_data_invalid() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let wrong_signature = "invalid";

        assert!(!verify_salt_and_data(key, salt, data, wrong_signature));
    }

    #[test]
    fn test_sign_salt_and_data_deterministic() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";

        let sig1 = sign_salt_and_data(key, salt, data);
        let sig2 = sign_salt_and_data(key, salt, data);

        assert_eq!(sig1, sig2);
    }
}
