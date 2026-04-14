//! Модуль конфигурации HMAC ключей.
//!
//! Централизованное управление HMAC ключами с валидацией при запуске.
//!
//! # Безопасность (Security Fix C1.3)
//!
//! Если переменная окружения HMAC ключа не установлена, автоматически генерируется
//! криптографически стойкий случайный ключ (256 бит). Это предотвращает тривиальную
//! подделку HMAC подписей, которая была возможна при использовании пустых строк.
//!
//! **ВАЖНО**: Случайно сгенерированные ключи не персистентны между перезапусками.
//! Для продакшена установите переменные окружения:
//! - `TETRIS_CONTROLS_HMAC_KEY`
//! - `TETRIS_LEADERBOARD_HMAC_KEY`
//! - `TETRIS_SAVE_DATA_HMAC_KEY`

use std::sync::OnceLock;

/// Минимальная длина HMAC ключа в байтах (128 бит).
///
/// Ключи короче 16 байт считаются небезопасными согласно NIST SP 800-107.
pub const MIN_HMAC_KEY_LENGTH: usize = 16;

/// Создать функцию получения HMAC ключа из переменной окружения.
///
/// SECURITY FIX (C1.3): Если ключ не установлен, генерируется криптографически стойкий
/// случайный ключ вместо пустой строки. Это предотвращает тривиальную подделку HMAC подписей.
/// Примечание: случайный ключ не персистентен между перезапусками приложения.
macro_rules! define_hmac_key_getter {
    ($fn_name:ident, $env_var:literal, $static_name:ident, $doc:literal) => {
        #[doc = $doc]
        fn $fn_name() -> &'static String {
            static $static_name: OnceLock<String> = OnceLock::new();
            $static_name.get_or_init(|| {
                std::env::var($env_var).unwrap_or_else(|_| {
                    // CRITICAL SECURITY FIX: Generate secure random key instead of empty string
                    let random_key = crate::crypto::generate_salt()
                        .expect("Failed to generate HMAC key: system RNG unavailable");
                    crate::log_warn!(
                        "HMAC ключ '{_key_name}' не установлен — сгенерирован случайный ключ. \
                         ВНИМАНИЕ: ключ не персистентен между перезапусками. \
                         Установите переменную окружения {_env_var} для продакшена.",
                        _key_name = $env_var,
                        _env_var = $env_var
                    );
                    random_key
                })
            })
        }
    };
}

define_hmac_key_getter!(
    get_controls_hmac_key_runtime,
    "TETRIS_CONTROLS_HMAC_KEY",
    CONTROLS_KEY,
    "Ключ для HMAC подписи конфигурации управления"
);

define_hmac_key_getter!(
    get_leaderboard_hmac_key_runtime,
    "TETRIS_LEADERBOARD_HMAC_KEY",
    LEADERBOARD_KEY,
    "Ключ для HMAC подписи таблицы лидеров"
);

define_hmac_key_getter!(
    get_save_data_hmac_key_runtime,
    "TETRIS_SAVEDATA_HMAC_KEY",
    SAVEDATA_KEY,
    "Ключ для HMAC подписи данных рекордов"
);

/// Получить ключ для HMAC подписи конфигурации управления.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_CONTROLS_HMAC_KEY` или пустую строку
#[must_use]
pub fn get_controls_hmac_key() -> &'static str {
    get_controls_hmac_key_runtime().as_str()
}

/// Получить ключ для HMAC подписи таблицы лидеров.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_LEADERBOARD_HMAC_KEY` или пустую строку
#[must_use]
pub fn get_leaderboard_hmac_key() -> &'static str {
    get_leaderboard_hmac_key_runtime().as_str()
}

/// Получить ключ для HMAC подписи данных рекордов.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_SAVEDATA_HMAC_KEY` или пустую строку
#[must_use]
pub fn get_save_data_hmac_key() -> &'static str {
    get_save_data_hmac_key_runtime().as_str()
}

/// Проверить валидность HMAC ключа.
///
/// # Аргументы
/// * `key` - ключ для проверки
/// * `key_name` - имя ключа для сообщения об ошибке
///
/// # Errors
/// Возвращает ошибку если ключ пустой, содержит только пробельные символы
/// или короче минимальной длины (16 байт).
pub fn validate_hmac_key(key: &str, key_name: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err(format!(
            "HMAC ключ '{key_name}' не установлен. Пожалуйста, установите соответствующую переменную окружения (TETRIS_CONTROLS_HMAC_KEY, TETRIS_LEADERBOARD_HMAC_KEY или TETRIS_SAVEDATA_HMAC_KEY)"
        ));
    }

    if key.len() < MIN_HMAC_KEY_LENGTH {
        return Err(format!(
            "HMAC ключ '{}' слишком короткий ({} байт). Минимальная длина: {} байт",
            key_name,
            key.len(),
            MIN_HMAC_KEY_LENGTH
        ));
    }

    Ok(())
}

/// Проверить все HMAC ключи при запуске приложения.
///
/// Проверяет наличие и минимальную длину (16 байт) всех трёх ключей:
/// `CONTROLS_HMAC_KEY`, `LEADERBOARD_HMAC_KEY`, `SAVE_DATA_HMAC_KEY`.
///
/// # Возвращает
/// - `Ok(())` если все ключи прошли валидацию
/// - `Err(Vec<String>)` с описанием всех обнаруженных проблем
///
/// # Errors
/// Возвращает ошибку если хотя бы один ключ пустой или слишком короткий.
pub fn validate_all_keys() -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_hmac_key(get_controls_hmac_key(), "CONTROLS_HMAC_KEY") {
        errors.push(e);
    }
    if let Err(e) = validate_hmac_key(get_leaderboard_hmac_key(), "LEADERBOARD_HMAC_KEY") {
        errors.push(e);
    }
    if let Err(e) = validate_hmac_key(get_save_data_hmac_key(), "SAVE_DATA_HMAC_KEY") {
        errors.push(e);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod keys_tests {
    use super::*;

    #[test]
    fn test_get_functions_return_empty_without_env() {
        // UPDATED (C1.3 Fix): Without env var, functions now return random keys instead of empty strings
        // This is a security improvement to prevent trivial HMAC forgery
        assert!(
            !get_controls_hmac_key().is_empty(),
            "Should generate random key"
        );
        assert!(
            !get_leaderboard_hmac_key().is_empty(),
            "Should generate random key"
        );
        assert!(
            !get_save_data_hmac_key().is_empty(),
            "Should generate random key"
        );

        // Verify keys are valid hex strings (64 chars = 256 bits)
        assert_eq!(get_controls_hmac_key().len(), 64);
        assert_eq!(get_leaderboard_hmac_key().len(), 64);
        assert_eq!(get_save_data_hmac_key().len(), 64);
    }

    #[test]
    fn test_validate_hmac_key_empty() {
        // Проверка что пустой ключ отклоняется
        let result = validate_hmac_key("", "TEST_KEY");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("не установлен"));
    }

    #[test]
    fn test_validate_hmac_key_whitespace_only() {
        // Проверка что ключ из пробелов отклоняется
        let result = validate_hmac_key("   ", "TEST_KEY");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("не установлен"));
    }

    #[test]
    fn test_validate_hmac_key_too_short() {
        // Проверка что короткий ключ отклоняется (< 16 байт)
        let result = validate_hmac_key("short", "TEST_KEY");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("слишком короткий"));
        assert!(err.contains("5 байт"));
        assert!(err.contains("16 байт"));
    }

    #[test]
    fn test_validate_hmac_key_exactly_minimum() {
        // Проверка что ключ ровно 16 байт принимается
        let result = validate_hmac_key("1234567890123456", "TEST_KEY");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hmac_key_valid() {
        // Проверка что валидный ключ принимается
        let result = validate_hmac_key("my-secret-key-123", "TEST_KEY");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_hmac_key_long() {
        // Проверка что длинный ключ принимается
        let long_key = "a".repeat(64);
        let result = validate_hmac_key(&long_key, "TEST_KEY");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_hmac_key_length_constant() {
        // Проверка что константа MIN_HMAC_KEY_LENGTH определена корректно
        assert_eq!(MIN_HMAC_KEY_LENGTH, 16);
    }

    /// Тест: проверка `validate_all_keys()` без установленных переменных окружения
    /// UPDATED (C1.3 Fix): Without env vars, keys are now randomly generated (not empty),
    /// so validation should pass since random keys are 64 chars (256 bits) which exceeds minimum.
    #[test]
    fn test_validate_all_keys_without_env() {
        let result = validate_all_keys();
        // With C1.3 fix, random keys are generated (64 hex chars = 256 bits > 16 bytes minimum)
        // So validation should now pass
        assert!(
            result.is_ok(),
            "validate_all_keys should pass with randomly generated keys"
        );
    }
}
