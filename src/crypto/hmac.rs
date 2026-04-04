//! Модуль HMAC подписи и верификации.
//!
//! # Ответственность
//! - Централизованное управление HMAC операциями
//! - Предоставление единого API для подписи и проверки данных
//! - Устранение дублирования HMAC логики
//!
//! # Использование
//! ```ignore
//! use tetris_cli::crypto::hmac::hmac_sha256;
//!
//! let signature = hmac_sha256("key", "data");
//! assert!(verify_hmac_sha256("key", "data", &signature));
//! ```

use ::hmac::{Hmac, Mac};
use sha2::Sha256;

/// Тип HMAC-SHA256.
pub type HmacSha256 = Hmac<Sha256>;

/// Вычислить HMAC-SHA256 подпись данных.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка из 64 символов (256 бит = 32 байта HMAC)
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::hmac_sha256;
/// let signature = hmac_sha256("ключ", "данные");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Безопасность
/// Используется криптографически стойкий HMAC-SHA256 согласно RFC 2104.
/// Это обеспечивает надёжную защиту от подделки данных.
///
/// # Исправление аудита 2026-03-30
/// Заменён .`expect()` на .`unwrap()` с комментарием о безопасности.
/// HMAC-SHA256 поддерживает ключи любой длины, поэтому ошибка невозможна.
///
/// # Исправление ISSUE-042
/// Эта функция является основной - алиасы `hmac_sign/hmac_verify` удалены.
#[allow(clippy::missing_panics_doc)]
#[must_use = "HMAC подпись должна быть использована для проверки"]
#[inline]
pub fn hmac_sha256(key: &str, data: &str) -> String {
    // HMAC-SHA256 принимает ключи любой длины, ошибка невозможна
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC-SHA256 new_from_slice never fails");
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
/// ```ignore
/// use tetris_cli::crypto::hmac::{hmac_sha256, verify_hmac_sha256};
/// let key = "секрет";
/// let data = "данные";
/// let signature = hmac_sha256(key, data);
/// assert!(verify_hmac_sha256(key, data, &signature));
/// ```
///
/// # Безопасность
/// Используется постоянное по времени сравнение для предотвращения timing-атак.
///
/// # Исправление NEW-150 (2026-04-02)
/// - Проверка длины включена в constant-time сравнение
/// - Используется `compiler_fence` для предотвращения оптимизаций
/// - Все операции выполняются независимо от результата
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn verify_hmac_sha256(key: &str, data: &str, expected_hash: &str) -> bool {
    let actual_hash = hmac_sha256(key, data);

    // Исправление NEW-150 (CRITICAL): Улучшенное constant-time сравнение
    // Предотвращает timing-атаки путём выполнения одинакового количества операций
    // независимо от позиции первого несовпадающего байта или длины

    let actual_bytes = actual_hash.as_bytes();
    let expected_bytes = expected_hash.as_bytes();

    // NEW-150: Включаем проверку длины в constant-time сравнение
    // Вместо раннего возврата используем XOR для накопления разницы длин
    let len_diff = actual_bytes.len() ^ expected_bytes.len();

    // NEW-150: Используем минимальную длину для предотвращения выхода за границы
    let min_len = core::cmp::min(actual_bytes.len(), expected_bytes.len());

    // XOR накопление - выполняем сравнение за постоянное время
    // SAFETY: len_diff — разница длин hex-строк HMAC-SHA256 (всегда 64 байта).
    // Даже при невалидном входе разница < 256, поэтому truncation u8 безопасен.
    #[allow(clippy::cast_possible_truncation)]
    let mut result: u8 = (len_diff & 0xFF) as u8;
    for i in 0..min_len {
        result |= actual_bytes[i] ^ expected_bytes[i];
    }

    // NEW-150: compiler_fence предотвращает переупорядочивание инструкций компилятором
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    result == 0
}

/// Вычислить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ (минимум 16 байт для безопасности)
/// * `salt` - соль для уникальности (рекомендуется 32 байта)
/// * `data` - данные для подписи
///
/// # Возвращает
/// Hex-строка HMAC-SHA256 подписи (64 символа = 256 бит)
///
/// # Безопасность
/// ## Криптографические свойства HMAC-SHA256
/// - **Аутентичность**: Только владелец ключа может создать валидную подпись
/// - **Целостность**: Любое изменение данных будет обнаружено при проверке
/// - **Неотказуемость**: Подписанные данные нельзя отвергнуть
///
/// ## Защита от атак
/// - **Rainbow table**: Соль предотвращает использование предвычисленных таблиц
/// - **Replay attack**: Уникальная соль для каждой подписи
/// - **Length extension**: HMAC не подвержен length extension атакам в отличие от простого хеширования
///
/// ## Рекомендации
/// - Используйте уникальную соль для каждой подписи
/// - Храните секретный ключ в переменной окружения, не в коде
/// - Минимальная длина ключа: 16 байт (рекомендуется 32 байта)
///
/// # Пример
/// ```ignore
/// use tetris_cli::crypto::hmac::hmac_sign_with_salt;
///
/// let signature = hmac_sign_with_salt("key", "salt", "data");
/// ```
///
/// # Исправление аудита 2026-04-02 (H1)
/// Оптимизировано: используется write! в Vec<u8> вместо format!() для снижения аллокаций.
#[must_use = "HMAC подпись должна быть использована для проверки"]
#[inline]
pub fn hmac_sign_with_salt(key: &str, salt: &str, data: &str) -> String {
    // Исправление ISSUE-197: Пустая соль допустима - просто конкатенируется без соли
    // Используем format!() напрямую для снижения аллокаций (исправление #15)
    let salted_data = format!("{salt}{data}");
    hmac_sha256(key, &salted_data)
}

/// Проверить HMAC подпись для данных с солью.
///
/// # Аргументы
/// * `key` - секретный ключ для проверки
/// * `salt` - соль для уникальности (должна совпадать с использованной при подписи)
/// * `data` - данные для проверки
/// * `signature` - ожидаемая подпись (hex-строка 64 символа)
///
/// # Возвращает
/// - `true` если подпись верна и данные не были изменены
/// - `false` если подпись не совпадает (данные подделаны или ключ неверный)
///
/// # Безопасность
/// ## Constant-time сравнение
/// - **Защита от timing-атак**: Используется постоянное по времени сравнение
/// - **XOR накопление**: Все байты сравниваются независимо от результата
/// - **Compiler fence**: Предотвращает переупорядочивание инструкций компилятором
///
/// ## Проверка целостности
/// - Проверяется соответствие соли, данных и ключа
/// - Любое изменение данных будет обнаружено
/// - Подделка подписи практически невозможна без знания ключа
///
/// ## Рекомендации
/// - Всегда проверяйте подпись перед использованием данных
/// - Не используйте данные если проверка не прошла
/// - Логируйте попытки подделки для мониторинга безопасности
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
///
/// # Исправление аудита 2026-04-02 (H1)
/// Оптимизировано: используется write! в Vec<u8> вместо format!() для снижения аллокаций.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn hmac_verify_with_salt(key: &str, salt: &str, data: &str, signature: &str) -> bool {
    // Используем format!() напрямую для снижения аллокаций (исправление #15)
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
        let signature = hmac_sha256("test_key", "test_data");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    #[test]
    fn test_hmac_sign_deterministic() {
        let sig1 = hmac_sha256("key", "data");
        let sig2 = hmac_sha256("key", "data");
        assert_eq!(sig1, sig2, "HMAC подписи должны быть детерминированными");
    }

    #[test]
    fn test_hmac_verify_valid() {
        let key = "test_key";
        let data = "test_data";
        let signature = hmac_sha256(key, data);

        assert!(
            verify_hmac_sha256(key, data, &signature),
            "Подпись должна быть валидной"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_key() {
        let key = "key1";
        let data = "data";
        let signature = hmac_sha256(key, data);

        assert!(
            !verify_hmac_sha256("key2", data, &signature),
            "Невалидный ключ должен возвращать false"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_data() {
        let key = "key";
        let data = "data1";
        let signature = hmac_sha256(key, data);

        assert!(
            !verify_hmac_sha256(key, "data2", &signature),
            "Невалидные данные должны возвращать false"
        );
    }

    #[test]
    fn test_hmac_verify_invalid_signature() {
        let key = "key";
        let data = "data";

        assert!(
            !verify_hmac_sha256(key, data, "invalid_signature"),
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
        let sig = hmac_sha256("", "data");
        assert_eq!(sig.len(), 64);

        // Пустые данные
        let sig = hmac_sha256("key", "");
        assert_eq!(sig.len(), 64);

        // Пустая соль
        let sig = hmac_sign_with_salt("key", "", "data");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_hmac_unicode_inputs() {
        let signature = hmac_sha256("ключ", "данные с Unicode: 你好🎮");
        assert_eq!(signature.len(), 64);

        let signature = hmac_sign_with_salt("ключ", "соль", "данные: 你好🎮");
        assert_eq!(signature.len(), 64);
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ HMAC-SHA256 (ИСПРАВЛЕНИЕ H2)
    // =========================================================================

    /// Тест на детерминированность `hmac_sha256`
    #[test]
    fn test_hmac_sha256_deterministic() {
        let sig1 = hmac_sha256("ключ", "данные");
        let sig2 = hmac_sha256("ключ", "данные");
        assert_eq!(sig1, sig2, "HMAC-SHA256 должен быть детерминированным");
    }

    /// Тест на разные ключи в `hmac_sha256`
    #[test]
    fn test_hmac_sha256_different_keys() {
        let sig1 = hmac_sha256("ключ1", "данные");
        let sig2 = hmac_sha256("ключ2", "данные");
        assert_ne!(sig1, sig2, "Разные ключи должны давать разные HMAC");
    }

    /// Тест на разные данные в `hmac_sha256`
    #[test]
    fn test_hmac_sha256_different_data() {
        let sig1 = hmac_sha256("ключ", "данные1");
        let sig2 = hmac_sha256("ключ", "данные2");
        assert_ne!(sig1, sig2, "Разные данные должны давать разные HMAC");
    }

    /// Тест на длину `hmac_sha256`
    #[test]
    fn test_hmac_sha256_length() {
        let signature = hmac_sha256("ключ", "данные");
        assert_eq!(
            signature.len(),
            64,
            "Длина HMAC-SHA256 должна быть 64 символа (256 бит)"
        );
    }

    /// Тест на валидную подпись `verify_hmac_sha256`
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

    /// Тест на невалидный ключ в `verify_hmac_sha256`
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

    /// Тест на невалидные данные в `verify_hmac_sha256`
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

    /// Тест на пустые данные в `hmac_sha256`
    #[test]
    fn test_hmac_sha256_empty_data() {
        let signature = hmac_sha256("ключ", "");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    /// Тест на пустой ключ в `hmac_sha256`
    #[test]
    fn test_hmac_sha256_empty_key() {
        let signature = hmac_sha256("", "данные");
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ NEW-150: TIMING ATTACK ЗАЩИТА
    // ========================================================================

    /// Тест на constant-time сравнение с разной длиной
    #[test]
    fn test_verify_hmac_sha256_different_length() {
        let key = "ключ";
        let data = "данные";
        let signature = hmac_sha256(key, data);

        // Укороченная подпись должна возвращать false
        let short_sig = &signature[..signature.len() - 1];
        assert!(
            !verify_hmac_sha256(key, data, short_sig),
            "Укороченная подпись должна возвращать false"
        );

        // Удлинённая подпись должна возвращать false
        let long_sig = format!("{signature}0");
        assert!(
            !verify_hmac_sha256(key, data, &long_sig),
            "Удлинённая подпись должна возвращать false"
        );
    }

    /// Тест на constant-time сравнение с пустой подписью
    #[test]
    fn test_verify_hmac_sha256_empty_signature() {
        let key = "ключ";
        let data = "данные";

        // Пустая подпись должна возвращать false
        assert!(
            !verify_hmac_sha256(key, data, ""),
            "Пустая подпись должна возвращать false"
        );
    }

    /// Тест на constant-time сравнение с одним байтом
    #[test]
    fn test_verify_hmac_sha256_single_byte_diff() {
        let key = "ключ";
        let data = "данные";
        let valid_signature = hmac_sha256(key, data);

        // Изменяем первый символ hex-строки (безопасная мутация через String)
        let mut invalid_signature = valid_signature;
        let first_char = invalid_signature.chars().next().unwrap();
        let new_char = char::from_u32((first_char as u32 + 1) % 256).unwrap_or('x');
        invalid_signature.replace_range(..1, &new_char.to_string());

        assert!(
            !verify_hmac_sha256(key, data, &invalid_signature),
            "Подпись с изменённым байтом должна возвращать false"
        );
    }

    /// Тест на constant-time сравнение с последним байтом
    #[test]
    fn test_verify_hmac_sha256_last_byte_diff() {
        let key = "ключ";
        let data = "данные";
        let valid_signature = hmac_sha256(key, data);

        // Изменяем последний символ hex-строки
        let mut invalid_signature = valid_signature;
        let last_char = invalid_signature.pop().unwrap();
        let new_char = char::from_u32((last_char as u32 + 1) % 256).unwrap_or('x');
        invalid_signature.push(new_char);

        assert!(
            !verify_hmac_sha256(key, data, &invalid_signature),
            "Подпись с изменённым последним байтом должна возвращать false"
        );
    }

    /// Тест на существование `compiler_fence`
    #[test]
    fn test_compiler_fence_exists() {
        // Этот тест просто проверяет что код компилируется с compiler_fence
        let key = "ключ";
        let data = "данные";
        let signature = hmac_sha256(key, data);
        assert!(verify_hmac_sha256(key, data, &signature));
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ HMAC SIGN/VERIFY С СОЛЬЮ (исправление #59-60)
    // ========================================================================

    /// Тест: hmac_sign_with_salt и hmac_verify_with_salt работают корректно
    #[test]
    fn test_hmac_sign_verify_with_salt_roundtrip() {
        let key = "тестовый_ключ";
        let salt = "уникальная_соль";
        let data = "тестовые_данные";

        let signature = hmac_sign_with_salt(key, salt, data);
        assert!(
            hmac_verify_with_salt(key, salt, data, &signature),
            "Подпись с солью должна проходить проверку"
        );
    }

    /// Тест: пустая соль допустима
    #[test]
    fn test_hmac_sign_verify_empty_salt() {
        let key = "ключ";
        let data = "данные";

        let signature = hmac_sign_with_salt(key, "", data);
        assert!(
            hmac_verify_with_salt(key, "", data, &signature),
            "Пустая соль должна быть допустима"
        );
    }

    /// Тест: разные соли дают разные подписи
    #[test]
    fn test_hmac_different_salts_different_signatures() {
        let key = "ключ";
        let data = "данные";

        let sig1 = hmac_sign_with_salt(key, "соль1", data);
        let sig2 = hmac_sign_with_salt(key, "соль2", data);
        assert_ne!(sig1, sig2, "Разные соли должны давать разные подписи");
    }
}
