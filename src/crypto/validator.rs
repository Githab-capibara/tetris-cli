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
pub use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};

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
        let signature = hmac_sign_with_salt(key, salt, data).unwrap();

        assert!(hmac_verify_with_salt(key, salt, data, &signature).unwrap());
    }

    #[test]
    fn test_re_export_invalid() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";
        let wrong_signature = "invalid";

        assert!(!hmac_verify_with_salt(key, salt, data, wrong_signature).unwrap());
    }

    #[test]
    fn test_re_export_deterministic() {
        let key = "test_key";
        let salt = "test_salt";
        let data = "test_data";

        let sig1 = hmac_sign_with_salt(key, salt, data).unwrap();
        let sig2 = hmac_sign_with_salt(key, salt, data).unwrap();

        assert_eq!(sig1, sig2);
    }
}
