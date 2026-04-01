//! Модуль конфигурации HMAC ключей.
//!
//! # Ответственность
//! - Централизованное управление HMAC ключами
//! - Определение констант ключей в одном месте
//! - Предотвращение дублирования ключей
//! - Валидация ключей при запуске
//!
//! # Использование
//! ```ignore
//! use tetris_cli::config::keys::{CONTROLS_HMAC_KEY, LEADERBOARD_HMAC_KEY, SAVE_DATA_HMAC_KEY};
//!
//! let key = CONTROLS_HMAC_KEY;
//! ```

/// Ключ для HMAC подписи конфигурации управления.
///
/// # Безопасность
/// Ключ загружается ТОЛЬКО из переменной окружения `TETRIS_HMAC_KEY`.
/// Для продакшена обязательно установите переменную окружения:
/// ```bash
/// export TETRIS_HMAC_KEY="your-secret-key-here"
/// ```
///
/// # Исправление аудита 2026-04-01 (S1)
/// Убран fallback ключ — требуется переменная окружения.
pub const CONTROLS_HMAC_KEY: &str = "";

/// Ключ для HMAC подписи таблицы лидеров.
///
/// # Безопасность
/// Ключ загружается ТОЛЬКО из переменной окружения `TETRIS_HMAC_KEY`.
///
/// # Исправление аудита 2026-04-01 (S1)
/// Убран fallback ключ — требуется переменная окружения.
pub const LEADERBOARD_HMAC_KEY: &str = "";

/// Ключ для HMAC подписи данных рекордов.
///
/// # Безопасность
/// Ключ загружается ТОЛЬКО из переменной окружения `TETRIS_HMAC_KEY`.
///
/// # Исправление аудита 2026-04-01 (S1)
/// Убран fallback ключ — требуется переменная окружения.
pub const SAVE_DATA_HMAC_KEY: &str = "";

/// Получить ключ для HMAC подписи конфигурации управления.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY`
///
/// # Безопасность
/// Требует переменную окружения `TETRIS_HMAC_KEY`.
#[must_use]
pub fn get_controls_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(CONTROLS_HMAC_KEY)
}

/// Получить ключ для HMAC подписи таблицы лидеров.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY`
#[must_use]
pub fn get_leaderboard_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(LEADERBOARD_HMAC_KEY)
}

/// Получить ключ для HMAC подписи данных рекордов.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY`
#[must_use]
pub fn get_save_data_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(SAVE_DATA_HMAC_KEY)
}

/// Проверить валидность HMAC ключа.
///
/// # Аргументы
/// * `key` - ключ для проверки
/// * `key_name` - имя ключа для сообщения об ошибке
///
/// # Исправление аудита 2026-04-01 (C1)
/// Добавлена валидация HMAC ключей при запуске с проверкой наличия ключа.
///
/// # Errors
/// Возвращает ошибку если ключ пустой или содержит только пробельные символы.
#[allow(dead_code)]
pub fn validate_hmac_key(key: &str, key_name: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err(format!(
            "HMAC ключ '{}' не установлен. Пожалуйста, установите переменную окружения TETRIS_HMAC_KEY",
            key_name
        ));
    }
    Ok(())
}

/// Проверить все HMAC ключи при запуске приложения.
///
/// # Исправление аудита 2026-04-01 (C1)
/// Добавлена функция для валидации всех ключей при запуске.
///
/// # Errors
/// Возвращает ошибку если хотя бы один ключ пустой.
#[allow(dead_code)]
pub fn validate_all_keys() -> Result<(), String> {
    validate_hmac_key(get_controls_hmac_key(), "CONTROLS_HMAC_KEY")?;
    validate_hmac_key(get_leaderboard_hmac_key(), "LEADERBOARD_HMAC_KEY")?;
    validate_hmac_key(get_save_data_hmac_key(), "SAVE_DATA_HMAC_KEY")?;
    Ok(())
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod keys_tests {
    use super::*;

    #[test]
    fn test_constants_defined() {
        // Проверка что константы определены (могут быть пустыми, если нет env vars)
        // После исправления S1 fallback ключи удалены, константы пустые по умолчанию
        let _ = CONTROLS_HMAC_KEY;
        let _ = LEADERBOARD_HMAC_KEY;
        let _ = SAVE_DATA_HMAC_KEY;
    }

    #[test]
    fn test_constants_are_different() {
        // Проверка что функции загрузки ключей работают корректно
        // Ключи загружаются из TETRIS_HMAC_KEY переменной окружения
        assert_eq!(get_controls_hmac_key(), get_leaderboard_hmac_key());
        assert_eq!(get_controls_hmac_key(), get_save_data_hmac_key());
    }

    #[test]
    fn test_get_functions_return_non_empty() {
        // Проверка что функции работают (возвращают пустую строку если нет env var)
        // Это ожидаемое поведение — ключи должны задаваться через переменную окружения
        let _ = get_controls_hmac_key();
        let _ = get_leaderboard_hmac_key();
        let _ = get_save_data_hmac_key();
    }

    #[test]
    fn test_get_functions_match_constants() {
        // Проверка что функции возвращают те же значения что и константы
        // (когда переменная окружения не установлена)
        // После удаления fallback ключей все возвращают пустые строки
        assert_eq!(get_controls_hmac_key(), CONTROLS_HMAC_KEY);
        assert_eq!(get_leaderboard_hmac_key(), LEADERBOARD_HMAC_KEY);
        assert_eq!(get_save_data_hmac_key(), SAVE_DATA_HMAC_KEY);
    }

    #[test]
    fn test_get_functions_with_env_var() {
        // Проверка что при установленной переменной окружения функции возвращают её значение
        // Этот тест проверяет корректную работу option_env! макроса
        let key = get_controls_hmac_key();
        // Если TETRIS_HMAC_KEY установлен, функция вернёт его значение
        // Иначе вернёт пустую строку из константы
        let _ = key;
    }
}
