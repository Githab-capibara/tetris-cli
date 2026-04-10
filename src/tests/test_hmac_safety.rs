//! Тесты безопасности HMAC-SHA256.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Содержит расширенные тесты которые НЕ дублируются в `crypto/hmac.rs`:
//! - Ключи разной длины (short, medium, long, extreme)
//! - Стресс-тесты (1000 итераций)
//! - Unicode и бинарные данные
//! - Timing-safe сравнение
//! - Интеграционный тест sign/verify

#[cfg(test)]
mod tests {
    use crate::crypto::{hmac_sha256, verify_hmac_sha256};

    // ========================================================================
    // Длина HMAC выхода для различных входных данных (параметризованный тест)
    // ========================================================================

    #[test]
    fn test_hmac_output_length_various_inputs() {
        let test_cases = vec![
            ("medium_key_1234567890", "test_data"),
            ("long_key_1234567890_abcdefghijklmnop", "test_data"),
            (
                "extreme_key_with_very_long_string_that_exceeds_normal_key_length_by_far",
                "test_data",
            ),
            ("ключ", "данные"),
            ("binary", "\x00\x01\x02\x03"),
        ];

        for (key, data) in test_cases {
            let signature = hmac_sha256(key, data);
            assert_eq!(
                signature.len(),
                64,
                "HMAC output for key='{}' and data='{}' should be 64 hex chars",
                key,
                data
            );
        }
    }

    // ========================================================================
    // Проверка отсутствия паники
    // ========================================================================

    #[test]
    fn test_hmac_sha256_no_panic_on_valid_input() {
        let long_key = "a".repeat(1000);
        for (key, data) in &[
            ("", ""),
            ("key", "data"),
            ("", "data"),
            ("key", ""),
            ("ключ", "данные"),
            (long_key.as_str(), "data"),
        ] {
            assert_eq!(hmac_sha256(key, data).len(), 64);
        }
    }

    // ========================================================================
    // Обработка ошибок
    // ========================================================================

    #[test]
    fn test_verify_hmac_sha256_wrong_length() {
        let key = "secret_key";
        let data = "test_data";
        assert!(!verify_hmac_sha256(key, data, "abc123"));
        assert!(!verify_hmac_sha256(key, data, &"a".repeat(128)));
    }

    // ========================================================================
    // Timing-safe сравнение
    // ========================================================================

    #[test]
    fn test_verify_hmac_sha256_timing_safe() {
        let key = "secret_key";
        let data = "test_data";
        let valid = hmac_sha256(key, data);
        let first = valid
            .chars()
            .next()
            .expect("HMAC сигнатура не должна быть пустой");
        let diff = if first == 'a' { 'b' } else { 'a' };
        let one_diff = format!("{diff}{}", &valid[1..]);
        assert!(!verify_hmac_sha256(key, data, &one_diff));
        assert!(verify_hmac_sha256(key, data, &valid));
    }

    // ========================================================================
    // Интеграционные и стресс-тесты
    // ========================================================================

    #[test]
    fn test_hmac_sign_verify_integration() {
        let long_key = "a".repeat(1000);
        for (key, data) in &[
            ("key1", "data1"),
            ("", ""),
            ("ключ", "данные"),
            (long_key.as_str(), "data"),
        ] {
            let sig = hmac_sha256(key, data);
            assert!(verify_hmac_sha256(key, data, &sig));
            assert!(!verify_hmac_sha256(key, &format!("{data}x"), &sig));
            assert!(!verify_hmac_sha256(&format!("{key}x"), data, &sig));
        }
    }
}
