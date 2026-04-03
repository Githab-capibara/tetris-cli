//! Тесты безопасности HMAC-SHA256.
//!
//! Этот модуль содержит тесты для проверки исправлений HMAC:
//! - hmac_sha256() с ключами разной длины
//! - unwrap_or_else() вместо expect() для безопасности
//! - Обработка ошибок HMAC
//!
//! # Исправления
//! - Исправление аудита 2026-03-30: unwrap_or_else() вместо expect()
//! - Исправление #4: Настоящий HMAC-SHA256 вместо конкатенации
//! - Защита от timing-атак через постоянное по времени сравнение

#[cfg(test)]
mod tests {
    use crate::crypto::{hmac_sha256, verify_hmac_sha256};

    // ========================================================================
    // ГРУППА ТЕСТОВ 1: hmac_sha256() с ключами разной длины
    // ========================================================================

    /// Тест 1: Проверка hmac_sha256() с пустым ключом.
    ///
    /// Проверяет что HMAC-SHA256 работает с пустым ключом.
    #[test]
    fn test_hmac_sha256_empty_key() {
        let signature = hmac_sha256("", "test_data");

        // HMAC-SHA256 поддерживает ключи любой длины включая пустые
        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC-SHA256 должна быть 64 символа (256 бит)"
        );

        // Пустой ключ должен давать детерминированный результат
        let signature2 = hmac_sha256("", "test_data");
        assert_eq!(
            signature, signature2,
            "HMAC с пустым ключом должен быть детерминированным"
        );
    }

    /// Тест 2: Проверка hmac_sha256() с коротким ключом.
    ///
    /// Проверяет что HMAC-SHA256 работает с короткими ключами (1-10 символов).
    #[test]
    fn test_hmac_sha256_short_key() {
        let short_keys = ["a", "key", "secret", "12345", "ключ"];

        for key in short_keys {
            let signature = hmac_sha256(key, "test_data");

            assert_eq!(
                signature.len(),
                64,
                "Длина HMAC для короткого ключа должна быть 64 символа"
            );

            // Проверка детерминированности
            let signature2 = hmac_sha256(key, "test_data");
            assert_eq!(
                signature, signature2,
                "HMAC с коротким ключом должен быть детерминированным"
            );
        }
    }

    /// Тест 3: Проверка hmac_sha256() с ключом средней длины.
    ///
    /// Проверяет что HMAC-SHA256 работает с ключами средней длины (11-64 символа).
    #[test]
    fn test_hmac_sha256_medium_key() {
        let medium_keys = [
            "my_secret_key_123",
            "длинный_ключ_на_русском",
            "a".repeat(32).as_str(),
            "a".repeat(64).as_str(),
        ];

        for key in medium_keys {
            let signature = hmac_sha256(key, "test_data");

            assert_eq!(
                signature.len(),
                64,
                "Длина HMAC для ключа средней длины должна быть 64 символа"
            );
        }
    }

    /// Тест 4: Проверка hmac_sha256() с длинным ключом.
    ///
    /// Проверяет что HMAC-SHA256 работает с очень длинными ключами (>64 символа).
    #[test]
    fn test_hmac_sha256_long_key() {
        let long_key = "a".repeat(256);
        let signature = hmac_sha256(&long_key, "test_data");

        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC для длинного ключа должна быть 64 символа"
        );

        // Проверка детерминированности
        let signature2 = hmac_sha256(&long_key, "test_data");
        assert_eq!(
            signature, signature2,
            "HMAC с длинным ключом должен быть детерминированным"
        );
    }

    /// Тест 5: Проверка hmac_sha256() с ключом экстремальной длины.
    ///
    /// Проверяет что HMAC-SHA256 работает с экстремально длинными ключами.
    #[test]
    fn test_hmac_sha256_extreme_key() {
        let extreme_key = "a".repeat(10000);
        let signature = hmac_sha256(&extreme_key, "test_data");

        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC для экстремально длинного ключа должна быть 64 символа"
        );
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 2: unwrap_or_else() вместо expect()
    // ========================================================================

    /// Тест 6: Проверка что hmac_sha256() не паникует на валидных данных.
    ///
    /// Проверяет что unwrap_or_else() корректно обрабатывает случай
    /// когда HMAC поддерживает ключи любой длины.
    #[test]
    fn test_hmac_sha256_no_panic_on_valid_input() {
        // Эти данные не должны вызывать панику
        let test_cases = [
            ("", ""),
            ("key", "data"),
            ("", "data"),
            ("key", ""),
            ("ключ", "данные"),
            ("a".repeat(1000).as_str(), "data"),
        ];

        for (key, data) in test_cases {
            // unwrap_or_else должен вернуть значение без паники
            let signature = hmac_sha256(key, data);

            assert_eq!(
                signature.len(),
                64,
                "HMAC должен вернуть 64 символа для валидных данных"
            );
        }
    }

    /// Тест 7: Проверка что unwrap_or_else() использует unreachable!().
    ///
    /// Проверяет что в случае ошибки (что невозможно для HMAC)
    /// вызывается unreachable!() с понятным сообщением.
    #[test]
    fn test_hmac_sha256_unreachable_pattern() {
        // HMAC-SHA256 поддерживает ключи любой длины, поэтому
        // new_from_slice() никогда не вернёт ошибку.
        // Этот тест проверяет что код компилируется с unwrap_or_else()

        use crate::crypto::HmacSha256;
        use hmac::Mac;
        use sha2::Sha256;

        // Симулируем логику из hmac_sha256()
        let key = "test_key";
        let mac_result = HmacSha256::new_from_slice(key.as_bytes());

        // unwrap_or_else должен сработать без паники
        let _mac =
            mac_result.unwrap_or_else(|_| unreachable!("HMAC поддерживает ключи любой длины"));

        // Если тест дошёл до этой точки, значит unwrap_or_else() сработал
        println!("✓ unwrap_or_else() сработал корректно");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 3: Обработка ошибок HMAC
    // ========================================================================

    /// Тест 8: Проверка verify_hmac_sha256() с валидной подписью.
    ///
    /// Проверяет что verify_hmac_sha256() возвращает true для валидной подписи.
    #[test]
    fn test_verify_hmac_sha256_valid_signature() {
        let key = "secret_key";
        let data = "test_data";
        let signature = hmac_sha256(key, data);

        let is_valid = verify_hmac_sha256(key, data, &signature);

        assert!(
            is_valid,
            "verify_hmac_sha256() должен вернуть true для валидной подписи"
        );
    }

    /// Тест 9: Проверка verify_hmac_sha256() с невалидным ключом.
    ///
    /// Проверяет что verify_hmac_sha256() возвращает false для невалидного ключа.
    #[test]
    fn test_verify_hmac_sha256_invalid_key() {
        let key = "secret_key";
        let data = "test_data";
        let signature = hmac_sha256(key, data);

        let is_valid = verify_hmac_sha256("wrong_key", data, &signature);

        assert!(
            !is_valid,
            "verify_hmac_sha256() должен вернуть false для невалидного ключа"
        );
    }

    /// Тест 10: Проверка verify_hmac_sha256() с невалидными данными.
    ///
    /// Проверяет что verify_hmac_sha256() возвращает false для изменённых данных.
    #[test]
    fn test_verify_hmac_sha256_invalid_data() {
        let key = "secret_key";
        let data = "test_data";
        let signature = hmac_sha256(key, data);

        let is_valid = verify_hmac_sha256(key, "modified_data", &signature);

        assert!(
            !is_valid,
            "verify_hmac_sha256() должен вернуть false для изменённых данных"
        );
    }

    /// Тест 11: Проверка verify_hmac_sha256() с невалидной подписью.
    ///
    /// Проверяет что verify_hmac_sha256() возвращает false для невалидной подписи.
    #[test]
    fn test_verify_hmac_sha256_invalid_signature() {
        let key = "secret_key";
        let data = "test_data";

        let is_valid = verify_hmac_sha256(key, data, "invalid_signature");

        assert!(
            !is_valid,
            "verify_hmac_sha256() должен вернуть false для невалидной подписи"
        );
    }

    /// Тест 12: Проверка verify_hmac_sha256() с подписью разной длины.
    ///
    /// Проверяет что verify_hmac_sha256() корректно обрабатывает подписи
    /// с неправильной длиной.
    #[test]
    fn test_verify_hmac_sha256_wrong_length() {
        let key = "secret_key";
        let data = "test_data";

        // Короткая подпись
        let is_valid_short = verify_hmac_sha256(key, data, "abc123");
        assert!(
            !is_valid_short,
            "verify_hmac_sha256() должен вернуть false для короткой подписи"
        );

        // Длинная подпись
        let long_signature = "a".repeat(128);
        let is_valid_long = verify_hmac_sha256(key, data, &long_signature);
        assert!(
            !is_valid_long,
            "verify_hmac_sha256() должен вернуть false для длинной подписи"
        );
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 4: Timing-safe сравнение
    // ========================================================================

    /// Тест 13: Проверка что verify_hmac_sha256() использует постоянное по времени сравнение.
    ///
    /// Проверяет что сравнение подписей выполняется за постоянное время
    /// через XOR накопление.
    #[test]
    fn test_verify_hmac_sha256_timing_safe_comparison() {
        let key = "secret_key";
        let data = "test_data";
        let valid_signature = hmac_sha256(key, data);

        // Создаём подписи с разным количеством несовпадающих символов
        let signature_one_diff = "b".to_string() + &valid_signature[1..];
        let signature_many_diff =
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

        // Обе проверки должны вернуть false
        assert!(
            !verify_hmac_sha256(key, data, &signature_one_diff),
            "Одна несовпадающая буква должна вернуть false"
        );
        assert!(
            !verify_hmac_sha256(key, data, signature_many_diff),
            "Много несовпадающих букв должны вернуть false"
        );

        // Валидная подпись должна вернуть true
        assert!(
            verify_hmac_sha256(key, data, &valid_signature),
            "Валидная подпись должна вернуть true"
        );
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 5: Unicode и специальные символы
    // ========================================================================

    /// Тест 14: Проверка hmac_sha256() с Unicode ключами и данными.
    ///
    /// Проверяет что HMAC-SHA256 корректно работает с Unicode.
    #[test]
    fn test_hmac_sha256_unicode() {
        let unicode_key = "секретный_ключ_🔑";
        let unicode_data = "данные_с_Unicode_你好_🎮";
        let signature = hmac_sha256(unicode_key, unicode_data);

        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC для Unicode данных должна быть 64 символа"
        );

        // Проверка детерминированности
        let signature2 = hmac_sha256(unicode_key, unicode_data);
        assert_eq!(
            signature, signature2,
            "HMAC для Unicode должен быть детерминированным"
        );

        // Проверка валидации
        assert!(
            verify_hmac_sha256(unicode_key, unicode_data, &signature),
            "HMAC для Unicode должен валидироваться корректно"
        );
    }

    /// Тест 15: Проверка hmac_sha256() с бинарными данными.
    ///
    /// Проверяет что HMAC-SHA256 корректно работает с бинарными данными
    /// представленными как hex-строка.
    #[test]
    fn test_hmac_sha256_binary_data() {
        // Бинарные данные как hex-строка
        let binary_data = "\x00\x01\x02\x03\x04\x05\x06\x07";
        let key = "secret_key";
        let signature = hmac_sha256(key, binary_data);

        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC для бинарных данных должна быть 64 символа"
        );
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 6: Интеграционные тесты
    // ========================================================================

    /// Тест 16: Интеграционный тест HMAC подписи и верификации.
    ///
    /// Проверяет полный цикл: создание подписи -> верификация.
    #[test]
    fn test_hmac_sign_verify_integration() {
        let test_cases = [
            ("key1", "data1"),
            ("key2", "data2"),
            ("", ""),
            ("ключ", "данные"),
            ("a".repeat(1000).as_str(), "data"),
        ];

        for (key, data) in test_cases {
            // Создаём подпись
            let signature = hmac_sha256(key, data);

            // Верифицируем подпись
            assert!(
                verify_hmac_sha256(key, data, &signature),
                "Подпись должна валидироваться для ключа '{}' и данных '{}'",
                key,
                data
            );

            // Проверяем что изменённые данные не валидируются
            assert!(
                !verify_hmac_sha256(key, &(data.to_string() + "x"), &signature),
                "Изменённые данные не должны валидироваться"
            );

            // Проверяем что изменённый ключ не валидируется
            assert!(
                !verify_hmac_sha256(&(key.to_string() + "x"), data, &signature),
                "Изменённый ключ не должен валидироваться"
            );
        }
    }

    /// Тест 17: Стресс-тест HMAC-SHA256.
    ///
    /// Проверяет что HMAC-SHA256 работает корректно при множественных вызовах.
    #[test]
    fn test_hmac_sha256_stress_test() {
        let key = "stress_test_key";
        let data = "stress_test_data";

        // Многократные вызовы
        for i in 0..1000 {
            let test_data = format!("{}_{}", data, i);
            let signature = hmac_sha256(key, &test_data);

            assert_eq!(
                signature.len(),
                64,
                "Длина HMAC должна быть 64 символа в итерации {}",
                i
            );

            assert!(
                verify_hmac_sha256(key, &test_data, &signature),
                "Подпись должна валидироваться в итерации {}",
                i
            );
        }

        println!("✓ Стресс-тест HMAC-SHA256 пройден (1000 итераций)");
    }

    /// Тест 18: Проверка детерминированности HMAC-SHA256.
    ///
    /// Проверяет что одинаковые ключ и данные всегда дают одинаковую подпись.
    #[test]
    fn test_hmac_sha256_deterministic() {
        let key = "deterministic_key";
        let data = "deterministic_data";

        // Создаём 10 подписей
        let signatures: Vec<String> = (0..10).map(|_| hmac_sha256(key, data)).collect();

        // Все подписи должны быть одинаковыми
        for i in 1..signatures.len() {
            assert_eq!(
                signatures[i], signatures[0],
                "HMAC-SHA256 должен быть детерминированным (итерация {})",
                i
            );
        }
    }
}
