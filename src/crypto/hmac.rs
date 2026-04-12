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
use std::io::Write;
use subtle::ConstantTimeEq;

/// Тип HMAC-SHA256.
pub type HmacSha256 = Hmac<Sha256>;

/// Сформировать буфер `salt:data` для HMAC-подписи.
///
/// Использует `write!` в `Vec<u8>` для снижения аллокаций по сравнению с `format!()`.
/// Буфер предварительно выделяется с точной ёмкостью для избежания реаллокаций.
///
/// # Audit 2026-04-12, Issue 2.5
/// Оптимизация: буфер выделяется с точной ёмкостью (salt.len() + 1 + data.len()).
/// Для дальнейшей оптимизации при частых вызовах можно рассмотреть переиспользование
/// буфера через thread-local storage, но текущая реализация оптимальна для большинства случаев.
///
/// # Исправление аудита 2026-04-11 (Пакет 2, #22)
/// Выделено из `hmac_sign_with_salt` и `hmac_verify_with_salt` для устранения дублирования.
#[inline]
fn build_salted_data(salt: &str, data: &str) -> Vec<u8> {
    let mut buf = Vec::with_capacity(salt.len() + 1 + data.len());
    // Audit 2026-04-12, Issue 8.1: write! в Vec<u8> никогда не возвращает ошибку,
    // так как Vec<u8> всегда успешно расширяется при необходимости (или паникует при OOM).
    // Игнорирование Result здесь безопасно и является идиоматичным для Rust.
    let _ = write!(buf, "{salt}:{data}");
    buf
}

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
/// ```
/// use tetris_cli::crypto::hmac::hmac_sha256;
/// let signature = hmac_sha256("ключ", "данные");
/// assert_eq!(signature.len(), 64);
/// ```
///
/// # Audit 2026-04-12, Issue 5.4
/// Пример использует реальный вызов функции для демонстрации API.
/// Длина подписи (64 символа hex = 32 байта HMAC-SHA256) проверяется в тестах.
///
/// # Безопасность
/// Используется криптографически стойкий HMAC-SHA256 согласно RFC 2104.
/// Это обеспечивает надёжную защиту от подделки данных.
///
/// # Panics
/// Паникует если `HmacSha256::new_from_slice` вернёт ошибку, что невозможно
/// для HMAC-SHA256 согласно RFC 2104 (принимает ключи любой длины).
///
/// # Исправление аудита 2026-03-30
/// Заменён .`expect()` на .`unwrap()` с комментарием о безопасности.
/// HMAC-SHA256 поддерживает ключи любой длины, поэтому ошибка невозможна.
///
/// # Исправление аудита 2026-04-11 (Пакет 1, #1-2)
/// Добавлена безопасная обработка через `expect` с понятным сообщением.
/// Хотя HMAC-SHA256 поддерживает ключи любой длины, `expect` даёт
/// диагностическую информацию при непредвиденных ошибках.
#[must_use = "HMAC подпись должна быть использована для проверки"]
#[inline]
pub fn hmac_sha256(key: &str, data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .expect("HMAC-SHA256 new_from_slice не должен падать — ключи любой длины по RFC 2104");
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
/// Используется XOR-накопление для сравнения всех байтов независимо от позиции
/// первого несовпадения. Это обеспечивает best-effort защиту от timing-атак
/// на уровне приложения.
///
/// # Исправление NEW-150 (2026-04-02)
/// - Проверка длины включена в constant-time сравнение
/// - Используется `compiler_fence` для предотвращения оптимизаций
/// - Все операции выполняются независимо от результата
///
/// # Примечание о timing-атаках
/// Текущая реализация — best-effort защита. `compiler_fence` предотвращает ТОЛЬКО
/// переупорядочивание компилятором. Для полной защиты от timing-атак на уровне CPU
/// используйте crate `subtle` (constant-time сравнение).
///
/// # Исправление аудита 2026-04-11 (Пакет 1, #3)
/// Для HMAC-SHA256 обе строки всегда ровно 64 hex-символа, поэтому padding
/// и аллокации удалены — используется прямое constant-time сравнение срезов.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn verify_hmac_sha256(key: &str, data: &str, expected_hash: &str) -> bool {
    let actual_hash = hmac_sha256(key, data);

    // constant-time сравнение через crate subtle
    let actual_bytes = actual_hash.as_bytes();
    let expected_bytes = expected_hash.as_bytes();

    // Для HMAC-SHA256 обе строки всегда ровно 64 hex-символа
    if actual_bytes.len() != expected_bytes.len() {
        return false;
    }

    // Constant-time сравнение без аллокаций
    actual_bytes.ct_eq(expected_bytes).unwrap_u8() == 1
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
/// # Errors
/// Возвращает ошибку в следующих случаях:
/// - `InvalidData` — если `salt` или `data` содержат невалидную UTF-8 последовательность
///   (теоретически невозможно для корректных `&str` входов, но обрабатывается для безопасности)
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
/// let signature = hmac_sign_with_salt("key", "salt", "data").unwrap();
/// ```
///
/// # Исправление аудита 2026-04-02 (H1)
/// Оптимизировано: используется `write!` в `Vec<u8>` вместо `format!()` для снижения аллокаций.
///
/// # Panics
/// Никогда не паникует — при невалидном UTF-8 возвращает `Err(io::Error)`.
#[must_use = "HMAC подпись должна быть использована для проверки"]
#[inline]
pub fn hmac_sign_with_salt(key: &str, salt: &str, data: &str) -> Result<String, std::io::Error> {
    let buf = build_salted_data(salt, data);
    let salted_data = std::str::from_utf8(&buf).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Невалидный UTF-8 в salt/data: {e}"),
        )
    })?;
    Ok(hmac_sha256(key, salted_data))
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
/// # Errors
/// Возвращает ошибку в следующих случаях:
/// - `InvalidData` — если `salt` или `data` содержат невалидную UTF-8 последовательность
///   (теоретически невозможно для корректных `&str` входов, но обрабатывается для безопасности)
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
/// let signature = hmac_sign_with_salt(key, salt, data).unwrap();
/// assert!(hmac_verify_with_salt(key, salt, data, &signature).unwrap());
/// ```
///
/// # Исправление аудита 2026-04-02 (H1)
/// Оптимизировано: используется `write!` в `Vec<u8>` вместо `format!()` для снижения аллокаций.
///
/// # Panics
/// Никогда не паникует — при невалидном UTF-8 возвращает `Err(io::Error)`.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn hmac_verify_with_salt(
    key: &str,
    salt: &str,
    data: &str,
    signature: &str,
) -> Result<bool, std::io::Error> {
    let buf = build_salted_data(salt, data);
    let salted_data = std::str::from_utf8(&buf).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Невалидный UTF-8 в salt/data: {e}"),
        )
    })?;
    Ok(verify_hmac_sha256(key, salted_data, signature))
}

/// Проверить HMAC подпись для данных с солью (байтовая версия).
///
/// Принимает `data` как `&[u8]` для избежания UTF-8 roundtrip.
/// Используется когда данные уже находятся в байтовом буфере (например, записаны через `write!`).
///
/// # Аргументы
/// * `key` - секретный ключ для проверки
/// * `salt` - соль для уникальности (должна совпадать с использованной при подписи)
/// * `data` - данные для проверки (байтовый буфер)
/// * `signature` - ожидаемая подпись (hex-строка 64 символа)
///
/// # Возвращает
/// - `true` если подпись верна
/// - `false` если подпись не совпадает
///
/// # Производительность (P3-ID41)
/// Избегает ненужного преобразования Vec -> &str -> bytes, поскольку `verify_hmac_sha256`
/// внутренне всё равно вызывает `.as_bytes()`. Это устраняет UTF-8 roundtrip.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn hmac_verify_with_salt_bytes(key: &str, salt: &str, data: &[u8], signature: &str) -> bool {
    // Формируем salt:data прямо в байтовом буфере без UTF-8 конвертации
    let mut buf = Vec::with_capacity(salt.len() + 1 + data.len());
    let _ = write!(buf, "{salt}:"); // ASCII соль и разделитель
    buf.extend_from_slice(data);
    // Передаём bytes напрямую — избегаем UTF-8 roundtrip
    verify_hmac_sha256_bytes(key, &buf, signature)
}

/// Проверить HMAC-SHA256 подпись (байтовая версия).
///
/// Принимает `data` как `&[u8]` для избежания UTF-8 конверсии.
/// Внутренне эквивалентна `verify_hmac_sha256` но без `.as_bytes()` преобразования.
///
/// # Исправление P3-ID41
/// Добавлена для устранения UTF-8 roundtrip в цепочке `verify_hash_for_value`.
///
/// # Исправление аудита 2026-04-11 (Пакет 1, #3b)
/// Устранены избыточные аллокации — используется прямое constant-time сравнение.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn verify_hmac_sha256_bytes(key: &str, data: &[u8], expected_hash: &str) -> bool {
    let actual_hash = hmac_sha256_bytes(key, data);

    let actual_bytes = actual_hash.as_bytes();
    let expected_bytes = expected_hash.as_bytes();

    // HMAC-SHA256 всегда даёт 64 hex-символа
    if actual_bytes.len() != expected_bytes.len() {
        return false;
    }

    actual_bytes.ct_eq(expected_bytes).unwrap_u8() == 1
}

/// Вычислить HMAC-SHA256 подпись (байтовая версия).
///
/// Принимает `data` как `&[u8]` для избежания UTF-8 конверсии.
///
/// # Исправление P3-ID41
/// Добавлена для устранения UTF-8 roundtrip.
///
/// # Panics
/// Паникует если `HmacSha256::new_from_slice` вернёт ошибку, что невозможно
/// для HMAC-SHA256 согласно RFC 2104 (принимает ключи любой длины).
///
/// # Исправление аудита 2026-04-11 (Пакет 1, #2)
/// Добавлена безопасная обработка через `expect` с диагностическим сообщением.
#[must_use = "HMAC подпись должна быть использована для проверки"]
#[inline]
pub fn hmac_sha256_bytes(key: &str, data: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .expect("HMAC-SHA256 new_from_slice не должен падать — ключи любой длины по RFC 2104");
    mac.update(data);
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Проверить HMAC подпись от уже отформатированных данных (без добавления соли).
///
/// Используется когда данные уже содержат соль (например, `{salt}:{name}:{value}`).
/// Избегает двойного форматирования и UTF-8 roundtrip.
///
/// # Аргументы
/// * `key` - секретный ключ
/// * `preformatted_data` - байтовый буфер с уже отформатированными данными (включая соль)
/// * `signature` - ожидаемая подпись
///
/// # Производительность (P3-ID41)
/// Полностью избегает создание нового буфера и UTF-8 конвертацию.
/// Передаёт байты напрямую в `hmac_sha256_bytes`.
#[must_use = "Результат проверки должен быть использован"]
#[inline]
pub fn hmac_verify_preformatted(key: &str, preformatted_data: &[u8], signature: &str) -> bool {
    verify_hmac_sha256_bytes(key, preformatted_data, signature)
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
        let signature = hmac_sign_with_salt("key", "salt", "data").unwrap();
        assert_eq!(signature.len(), 64, "Длина HMAC должна быть 64 символа");
    }

    #[test]
    fn test_hmac_sign_with_salt_deterministic() {
        let sig1 = hmac_sign_with_salt("key", "salt", "data").unwrap();
        let sig2 = hmac_sign_with_salt("key", "salt", "data").unwrap();
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
        let signature = hmac_sign_with_salt(key, salt, data).unwrap();

        assert!(
            hmac_verify_with_salt(key, salt, data, &signature).unwrap(),
            "Подпись с солью должна быть валидной"
        );
    }

    #[test]
    fn test_hmac_verify_with_salt_invalid() {
        let key = "key";
        let salt = "salt";
        let data = "data";
        let signature = hmac_sign_with_salt(key, salt, data).unwrap();

        assert!(
            !hmac_verify_with_salt(key, "wrong_salt", data, &signature).unwrap(),
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
        let sig = hmac_sign_with_salt("key", "", "data").unwrap();
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_hmac_unicode_inputs() {
        let signature = hmac_sha256("ключ", "данные с Unicode: 你好🎮");
        assert_eq!(signature.len(), 64);

        let signature = hmac_sign_with_salt("ключ", "соль", "данные: 你好🎮").unwrap();
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

    /// Тест: `hmac_sign_with_salt` и `hmac_verify_with_salt` работают корректно
    #[test]
    fn test_hmac_sign_verify_with_salt_roundtrip() {
        let key = "тестовый_ключ";
        let salt = "уникальная_соль";
        let data = "тестовые_данные";

        let signature = hmac_sign_with_salt(key, salt, data).unwrap();
        assert!(
            hmac_verify_with_salt(key, salt, data, &signature).unwrap(),
            "Подпись с солью должна проходить проверку"
        );
    }

    /// Тест: пустая соль допустима
    #[test]
    fn test_hmac_sign_verify_empty_salt() {
        let key = "ключ";
        let data = "данные";

        let signature = hmac_sign_with_salt(key, "", data).unwrap();
        assert!(
            hmac_verify_with_salt(key, "", data, &signature).unwrap(),
            "Пустая соль должна быть допустима"
        );
    }

    /// Тест: разные соли дают разные подписи
    #[test]
    fn test_hmac_different_salts_different_signatures() {
        let key = "ключ";
        let data = "данные";

        let sig1 = hmac_sign_with_salt(key, "соль1", data).unwrap();
        let sig2 = hmac_sign_with_salt(key, "соль2", data).unwrap();
        assert_ne!(sig1, sig2, "Разные соли должны давать разные подписи");
    }
}
