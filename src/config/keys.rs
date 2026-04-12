//! Модуль конфигурации HMAC ключей.
//!
//! Централизованное управление HMAC ключами с валидацией при запуске.
//! Если переменная окружения HMAC ключа не установлена, возвращается пустая строка
//! (удобно для dev-режима). В продакшене установите переменные окружения.

use std::sync::OnceLock;

/// Минимальная длина HMAC ключа в байтах (128 бит).
///
/// Ключи короче 16 байт считаются небезопасными согласно NIST SP 800-107.
pub const MIN_HMAC_KEY_LENGTH: usize = 16;

/// Вывести предупреждение о пустом HMAC ключе ТОЛЬКО ОДИН РАЗ при первом вызове.
fn log_once_empty_key(_key_name: &str) {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        crate::log_warn!(
            "HMAC ключ '{_key_name}' не установлен — используется пустая строка. \
             Это ослабляет HMAC защиту. Установите соответствующую переменную окружения."
        );
    });
}

/// Создать функцию получения HMAC ключа из переменной окружения.
macro_rules! define_hmac_key_getter {
    ($fn_name:ident, $env_var:literal, $static_name:ident, $doc:literal) => {
        #[doc = $doc]
        fn $fn_name() -> &'static String {
            static $static_name: OnceLock<String> = OnceLock::new();
            $static_name.get_or_init(|| {
                std::env::var($env_var).unwrap_or_else(|_| {
                    log_once_empty_key($env_var);
                    crate::log_warn!("Ошибка чтения {}: нет переменной окружения", $env_var);
                    String::new()
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
        // Без env var функции возвращают пустую строку
        assert!(get_controls_hmac_key().is_empty());
        assert!(get_leaderboard_hmac_key().is_empty());
        assert!(get_save_data_hmac_key().is_empty());
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
    /// Без env var все HMAC ключи пустые, поэтому валидация должна вернуть ошибки
    #[test]
    fn test_validate_all_keys_without_env() {
        let result = validate_all_keys();
        // Без переменных окружения все три ключа пустые, значит будут ошибки
        assert!(
            result.is_err(),
            "validate_all_keys должен вернуть ошибку без env"
        );
        let errors = result.unwrap_err();
        assert_eq!(
            errors.len(),
            3,
            "Должно быть 3 ошибки (по одной для каждого ключа)"
        );
    }
}
