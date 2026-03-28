//! Криптографические утилиты.
//!
//! Модуль предоставляет криптографические функции:
//! - `hash()` - хеширование BLAKE3
//! - `generate_salt()` - генерация случайной соли
//! - `keyed_hash()` - подпись с ключом (не настоящий HMAC!)

use rand::rngs::StdRng;
use rand::RngCore;
use rand::SeedableRng;

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
    // Используем StdRng с случайным seed от OS
    let mut rng = StdRng::from_os_rng();
    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
    RngCore::fill_bytes(&mut rng, &mut bytes);
    hex::encode(bytes)
}

/// Вычислить keyed_hash (ключ + данные) используя BLAKE3.
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
/// use tetris_cli::crypto::keyed_hash;
/// let signature = keyed_hash("ключ", "данные");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Важное замечание
/// Это НЕ настоящий HMAC! Функция просто конкатенирует ключ и данные,
/// затем хэширует результат. Для криптографически стойкого HMAC
/// используйте специализированные библиотеки (например, hmac-sha256).
///
/// # Исправление #2
/// Функция переименована из `hmac()` в `keyed_hash()` для ясности.
///
/// # Оптимизация 4.2
/// Используется `format!("{key}{data}")` вместо `key.to_string() + data`.
#[must_use = "Keyed hash должен быть использован для проверки"]
#[doc(hidden)]
#[allow(dead_code)]
pub fn keyed_hash(key: &str, data: &str) -> String {
    // Формируем ключ + данные для хеширования
    hash(&format!("{key}{data}"))
}

/// Проверить keyed_hash подпись.
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
/// ```
/// use tetris_cli::crypto::{keyed_hash, verify_keyed_hash};
/// let key = "секрет";
/// let data = "данные";
/// let signature = keyed_hash(key, data);
/// assert!(verify_keyed_hash(key, data, &signature));
/// ```
///
/// # Важное замечание
/// Это НЕ настоящий HMAC! См. документацию к `keyed_hash()`.
///
/// # Исправление #2
/// Функция переименована из `verify_hmac()` в `verify_keyed_hash()` для ясности.
#[must_use = "Результат проверки должен быть использован"]
#[doc(hidden)]
#[allow(dead_code)]
pub fn verify_keyed_hash(key: &str, data: &str, expected_hash: &str) -> bool {
    keyed_hash(key, data) == expected_hash
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
    fn test_keyed_hash_deterministic() {
        let sig1 = keyed_hash("ключ", "данные");
        let sig2 = keyed_hash("ключ", "данные");
        assert_eq!(sig1, sig2, "Keyed hash должен быть детерминированным");
    }

    #[test]
    fn test_keyed_hash_different_keys() {
        let sig1 = keyed_hash("ключ1", "данные");
        let sig2 = keyed_hash("ключ2", "данные");
        assert_ne!(sig1, sig2, "Разные ключи должны давать разные keyed hash");
    }

    #[test]
    fn test_keyed_hash_different_data() {
        let sig1 = keyed_hash("ключ", "данные1");
        let sig2 = keyed_hash("ключ", "данные2");
        assert_ne!(sig1, sig2, "Разные данные должны давать разные keyed hash");
    }

    #[test]
    fn test_verify_keyed_hash_valid() {
        let key = "тестовый ключ";
        let data = "тестовые данные";
        let signature = keyed_hash(key, data);
        assert!(
            verify_keyed_hash(key, data, &signature),
            "Правильная подпись должна проходить проверку"
        );
    }

    #[test]
    fn test_verify_keyed_hash_invalid_key() {
        let key = "ключ1";
        let data = "данные";
        let signature = keyed_hash(key, data);
        assert!(
            !verify_keyed_hash("ключ2", data, &signature),
            "Неправильный ключ не должен проходить проверку"
        );
    }

    #[test]
    fn test_verify_keyed_hash_invalid_data() {
        let key = "ключ";
        let data = "данные1";
        let signature = keyed_hash(key, data);
        assert!(
            !verify_keyed_hash(key, "данные2", &signature),
            "Неправильные данные не должны проходить проверку"
        );
    }

    #[test]
    fn test_verify_keyed_hash_invalid_signature() {
        let key = "ключ";
        let data = "данные";
        assert!(
            !verify_keyed_hash(key, data, "неправильная подпись"),
            "Неправильная подпись не должна проходить проверку"
        );
    }
}
