//! Модуль валидации данных с использованием HMAC.
//!
//! # Ответственность
//! - Валидация HMAC подписей
//! - Проверка целостности данных
//! - Защита от подделки данных
//!
//! # Использование
//! ```ignore
//! use tetris_cli::crypto::validator::HmacValidator;
//!
//! let validator = HmacValidator::new("secret_key");
//! let signature = validator.sign("data");
//! assert!(validator.verify("data", &signature));
//! ```

use crate::crypto::{hmac_sha256, verify_hmac_sha256};

/// Валидатор HMAC подписей.
///
/// Предоставляет удобный интерфейс для подписи и проверки данных
/// с использованием HMAC-SHA256.
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::crypto::validator::HmacValidator;
///
/// let validator = HmacValidator::new("my_secret_key");
///
/// // Подпись данных
/// let data = "player_name:1000";
/// let signature = validator.sign(data);
///
/// // Проверка подписи
/// assert!(validator.verify(data, &signature));
/// ```
#[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
pub struct HmacValidator {
    /// Секретный ключ для HMAC.
    key: String,
}

impl HmacValidator {
    /// Создать новый валидатор с указанным ключом.
    ///
    /// # Аргументы
    /// * `key` - секретный ключ для HMAC подписи
    ///
    /// # Возвращает
    /// Новый экземпляр `HmacValidator`
    ///
    /// # Пример
    /// ```ignore
    /// let validator = HmacValidator::new("my_secret_key");
    /// ```
    #[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
    #[must_use]
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }

    /// Создать валидатор со случайным ключом.
    ///
    /// # Возвращает
    /// Новый экземпляр `HmacValidator` со случайным ключом
    ///
    /// # Пример
    /// ```ignore
    /// let validator = HmacValidator::generate();
    /// ```
    #[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
    #[must_use]
    pub fn generate() -> Self {
        let key = crate::crypto::generate_salt();
        Self { key }
    }

    /// Вычислить HMAC подпись данных.
    ///
    /// # Аргументы
    /// * `data` - данные для подписи
    ///
    /// # Возвращает
    /// Hex-строка HMAC-SHA256 подписи (64 символа)
    ///
    /// # Пример
    /// ```ignore
    /// let validator = HmacValidator::new("key");
    /// let signature = validator.sign("data");
    /// ```
    #[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
    #[must_use]
    pub fn sign(&self, data: &str) -> String {
        hmac_sha256(&self.key, data)
    }

    /// Проверить HMAC подпись данных.
    ///
    /// # Аргументы
    /// * `data` - данные для проверки
    /// * `signature` - ожидаемая подпись
    ///
    /// # Возвращает
    /// `true` если подпись верна
    ///
    /// # Пример
    /// ```ignore
    /// let validator = HmacValidator::new("key");
    /// let signature = validator.sign("data");
    /// assert!(validator.verify("data", &signature));
    /// ```
    ///
    /// # Безопасность
    /// Используется постоянное по времени сравнение для предотвращения timing-атак.
    #[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
    #[must_use]
    pub fn verify(&self, data: &str, signature: &str) -> bool {
        verify_hmac_sha256(&self.key, data, signature)
    }

    /// Проверить и вернуть данные если подпись верна.
    ///
    /// # Аргументы
    /// * `data` - данные для проверки
    /// * `signature` - ожидаемая подпись
    ///
    /// # Возвращает
    /// `Some(data)` если подпись верна, `None` если подпись невалидна
    ///
    /// # Пример
    /// ```ignore
    /// let validator = HmacValidator::new("key");
    /// let signature = validator.sign("data");
    /// assert_eq!(validator.verify_and_return("data", &signature), Some("data".to_string()));
    /// ```
    #[allow(dead_code)] // Публичный API для внешних пользователей библиотеки
    #[must_use]
    pub fn verify_and_return(&self, data: &str, signature: &str) -> Option<String> {
        if self.verify(data, signature) {
            Some(data.to_string())
        } else {
            None
        }
    }

    /// Получить ключ валидатора (только для тестов).
    ///
    /// # Возвращает
    /// Ссылка на секретный ключ
    #[cfg(test)]
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
}

impl Default for HmacValidator {
    /// Создать валидатор по умолчанию со случайным ключом.
    fn default() -> Self {
        Self::generate()
    }
}

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

    #[test]
    fn test_hmac_validator_creation() {
        let validator = HmacValidator::new("test_key");
        assert_eq!(validator.key(), "test_key");
    }

    #[test]
    fn test_hmac_validator_generate() {
        let validator1 = HmacValidator::generate();
        let validator2 = HmacValidator::generate();
        assert_ne!(
            validator1.key(),
            validator2.key(),
            "Ключи должны быть уникальными"
        );
    }

    #[test]
    fn test_hmac_validator_sign_verify() {
        let validator = HmacValidator::new("test_key");
        let data = "test_data";
        let signature = validator.sign(data);

        assert!(
            validator.verify(data, &signature),
            "Подпись должна быть валидной"
        );
    }

    #[test]
    fn test_hmac_validator_invalid_signature() {
        let validator = HmacValidator::new("test_key");
        let data = "test_data";
        let wrong_signature = "invalid_signature";

        assert!(
            !validator.verify(data, wrong_signature),
            "Невалидная подпись должна возвращать false"
        );
    }

    #[test]
    fn test_hmac_validator_wrong_key() {
        let validator1 = HmacValidator::new("key1");
        let validator2 = HmacValidator::new("key2");
        let data = "test_data";

        let signature = validator1.sign(data);
        assert!(
            !validator2.verify(data, &signature),
            "Подпись с другим ключом должна быть невалидной"
        );
    }

    #[test]
    fn test_hmac_validator_verify_and_return() {
        let validator = HmacValidator::new("test_key");
        let data = "test_data";
        let signature = validator.sign(data);

        assert_eq!(
            validator.verify_and_return(data, &signature),
            Some(data.to_string())
        );
    }

    #[test]
    fn test_hmac_validator_verify_and_return_invalid() {
        let validator = HmacValidator::new("test_key");
        let data = "test_data";
        let wrong_signature = "invalid";

        assert_eq!(validator.verify_and_return(data, wrong_signature), None);
    }

    #[test]
    fn test_hmac_validator_deterministic() {
        let validator = HmacValidator::new("test_key");
        let data = "test_data";

        let sig1 = validator.sign(data);
        let sig2 = validator.sign(data);

        assert_eq!(sig1, sig2, "Подписи должны быть детерминированными");
    }

    #[test]
    fn test_hmac_validator_empty_data() {
        let validator = HmacValidator::new("test_key");
        let signature = validator.sign("");

        assert_eq!(signature.len(), 64, "Длина подписи должна быть 64 символа");
        assert!(validator.verify("", &signature));
    }

    #[test]
    fn test_hmac_validator_unicode_data() {
        let validator = HmacValidator::new("test_key");
        let data = "данные с Unicode: 你好🎮";
        let signature = validator.sign(data);

        assert_eq!(signature.len(), 64);
        assert!(validator.verify(data, &signature));
    }

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
