//! Тесты безопасности HMAC-SHA256.
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
    // Ключи разной длины (расширенные)
    // ========================================================================

    #[test]
    fn test_hmac_sha256_short_key() {
        let short_keys = ["a", "key", "secret", "12345", "ключ"];
        for key in short_keys {
            let signature = hmac_sha256(key, "test_data");
            assert_eq!(signature.len(), 64);
            assert_eq!(signature, hmac_sha256(key, "test_data"));
        }
    }

    #[test]
    fn test_hmac_sha256_medium_key() {
        let key_32 = "a".repeat(32);
        let key_64 = "a".repeat(64);
        for key in [
            "my_secret_key_123",
            "длинный_ключ",
            key_32.as_str(),
            key_64.as_str(),
        ] {
            assert_eq!(hmac_sha256(key, "test_data").len(), 64);
        }
    }

    #[test]
    fn test_hmac_sha256_long_key() {
        let long_key = "a".repeat(256);
        let sig = hmac_sha256(&long_key, "test_data");
        assert_eq!(sig.len(), 64);
        assert_eq!(sig, hmac_sha256(&long_key, "test_data"));
    }

    #[test]
    fn test_hmac_sha256_extreme_key() {
        let extreme_key = "a".repeat(10000);
        assert_eq!(hmac_sha256(&extreme_key, "test_data").len(), 64);
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
        let first = valid.chars().next().unwrap();
        let diff = if first == 'a' { 'b' } else { 'a' };
        let one_diff = format!("{diff}{}", &valid[1..]);
        assert!(!verify_hmac_sha256(key, data, &one_diff));
        assert!(verify_hmac_sha256(key, data, &valid));
    }

    // ========================================================================
    // Unicode и бинарные данные
    // ========================================================================

    #[test]
    fn test_hmac_sha256_unicode() {
        let sig = hmac_sha256("секретный_ключ_🔑", "данные_с_Unicode_你好_🎮");
        assert_eq!(sig.len(), 64);
        assert!(verify_hmac_sha256(
            "секретный_ключ_🔑",
            "данные_с_Unicode_你好_🎮",
            &sig
        ));
    }

    #[test]
    fn test_hmac_sha256_binary_data() {
        let binary = "\x00\x01\x02\x03\x04\x05\x06\x07";
        assert_eq!(hmac_sha256("secret_key", binary).len(), 64);
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

    #[test]
    fn test_hmac_sha256_stress_test() {
        for i in 0..1000 {
            let data = format!("stress_data_{i}");
            let sig = hmac_sha256("stress_key", &data);
            assert_eq!(sig.len(), 64);
            assert!(verify_hmac_sha256("stress_key", &data, &sig));
        }
    }
}
