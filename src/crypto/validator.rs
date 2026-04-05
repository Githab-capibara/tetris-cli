//! Модуль валидации данных с использованием HMAC.
//!
//! # Ответственность
//! - Ре-экспорт HMAC функций для обратной совместимости
//! - Проверка целостности данных
//! - Защита от подделки данных
//!
//! # Использование
//! ```ignore
//! use tetris_cli::crypto::validator::{hmac_sign_with_salt, hmac_verify_with_salt};
//!
//! let signature = hmac_sign_with_salt("key", "salt", "data");
//! assert!(hmac_verify_with_salt("key", "salt", "data", &signature));
//! ```
//!
//! # Исправление ISSUE-043
//! `HmacValidator` удалён — используйте напрямую `hmac_sha256/verify_hmac_sha256`.
//!
//! # Исправление аудита 2026-04-02 (#21)
//! Функции `sign_salt_and_data` и `verify_salt_and_data` были дубликатами
//! `hmac_sign_with_salt` и `hmac_verify_with_salt` из `hmac.rs`.
//! Теперь они заменены на re-export для устранения дублирования.

// Re-export из hmac.rs для устранения дублирования (#21)
#[allow(unused_imports)]
pub use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};

// Алиасы для обратной совместимости (deprecated)
/// Алиас для `hmac_sign_with_salt` (обратная совместимость).
///
/// # Устарело
/// Используйте [`hmac_sign_with_salt`] напрямую.
#[deprecated(since = "0.96.15", note = "Используйте hmac_sign_with_salt из hmac.rs")]
#[must_use = "Результат HMAC подписи должен быть использован"]
#[allow(dead_code)]
pub fn sign_salt_and_data(key: &str, salt: &str, data: &str) -> String {
    hmac_sign_with_salt(key, salt, data)
}

/// Алиас для `hmac_verify_with_salt` (обратная совместимость).
///
/// # Устарело
/// Используйте [`hmac_verify_with_salt`] напрямую.
#[deprecated(
    since = "0.96.15",
    note = "Используйте hmac_verify_with_salt из hmac.rs"
)]
#[must_use = "Результат проверки HMAC должен быть использован"]
#[allow(dead_code)]
pub fn verify_salt_and_data(key: &str, salt: &str, data: &str, expected_signature: &str) -> bool {
    hmac_verify_with_salt(key, salt, data, expected_signature)
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod validator_tests {
    use super::*;

    #[test]
    fn test_re_export_sign_and_verify() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let signature = hmac_sign_with_salt(key, salt, data);

        assert!(hmac_verify_with_salt(key, salt, data, &signature));
    }

    #[test]
    fn test_re_export_invalid() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let wrong_signature = "invalid";

        assert!(!hmac_verify_with_salt(key, salt, data, wrong_signature));
    }

    #[test]
    fn test_re_export_deterministic() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";

        let sig1 = hmac_sign_with_salt(key, salt, data);
        let sig2 = hmac_sign_with_salt(key, salt, data);

        assert_eq!(sig1, sig2);
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_aliases_still_work() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let signature = sign_salt_and_data(key, salt, data);

        assert!(verify_salt_and_data(key, salt, data, &signature));
    }
}
