//! Модуль конфигурации HMAC ключей.
//!
//! # Ответственность
//! - Централизованное управление HMAC ключами
//! - Определение констант ключей в одном месте
//! - Предотвращение дублирования ключей
//! - Валидация ключей при запуске
//!
//! # Fallback на пустой ключ (намеренное поведение)
//! Если переменная окружения HMAC ключа не установлена, возвращается пустая строка.
//! Это намеренное решение для dev-режима: приложение продолжает работать без HMAC ключа,
//! что удобно для локальной разработки и тестирования.
//! В продакшене ОБЯЗАТЕЛЬНО установите переменные окружения через `validate_all_keys()`.
//! Пустой HMAC ключ ослабляет защиту от подделки — это логируется через `log_once_empty_key`.
//!
//! # Использование
//! ```ignore
//! use tetris_cli::config::keys::{CONTROLS_HMAC_KEY, LEADERBOARD_HMAC_KEY, SAVE_DATA_HMAC_KEY};
//!
//! let key = CONTROLS_HMAC_KEY;
//! ```

use std::sync::OnceLock;

/// Минимальная длина HMAC ключа в байтах.
///
/// Ключи короче 16 байт считаются небезопасными.
pub const MIN_HMAC_KEY_LENGTH: usize = 16;

// ============================================================================
// LOG_ONCE ДЛЯ ПУСТЫХ HMAC КЛЮЧЕЙ (Исправление аудита 2026-04-05, Проблема 6)
// ============================================================================

/// Вывести предупреждение о пустом HMAC ключе ТОЛЬКО ОДИН РАЗ при первом вызове.
/// Использует `OnceLock` для гарантии однократного вывода независимо от
/// количества вызовов функции.
fn log_once_empty_key(_key_name: &str) {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        crate::log_warn!(
            "HMAC ключ '{key_name}' не установлен — используется пустая строка. \
             Это ослабляет HMAC защиту. Установите соответствующую переменную окружения."
        );
    });
}

/// Получить HMAC ключ из переменной окружения (общая функция).
///
/// # Аргументы
/// * `env_var_name` — имя переменной окружения
/// * `log_env_name` — имя ключа для логирования
///
/// # Возвращает
/// Статическую ссылку на строку ключа
fn get_hmac_key_runtime(env_var_name: &str, log_env_name: &str) -> &'static String {
    static CONTROLS_KEY: OnceLock<String> = OnceLock::new();
    static LEADERBOARD_KEY: OnceLock<String> = OnceLock::new();
    static SAVEDATA_KEY: OnceLock<String> = OnceLock::new();
    static FALLBACK: OnceLock<String> = OnceLock::new();

    // Выбираем нужный OnceLock через замыкание для избежания дублирования
    let get_or_init_fn = || {
        #[allow(unused_variables)]
        let key = std::env::var(env_var_name).unwrap_or_else(|e| {
            log_once_empty_key(log_env_name);
            crate::log_warn!("Ошибка чтения {env_var_name}: {e}");
            String::new()
        });
        if key.is_empty() {
            crate::log_warn!("HMAC ключ {log_env_name} не установлен — данные не защищены");
        }
        key
    };

    match env_var_name {
        "TETRIS_CONTROLS_HMAC_KEY" => CONTROLS_KEY.get_or_init(get_or_init_fn),
        "TETRIS_LEADERBOARD_HMAC_KEY" => LEADERBOARD_KEY.get_or_init(get_or_init_fn),
        "TETRIS_SAVEDATA_HMAC_KEY" => SAVEDATA_KEY.get_or_init(get_or_init_fn),
        _ => {
            crate::log_error!("Неизвестный HMAC ключ: {env_var_name}");
            FALLBACK.get_or_init(String::new)
        }
    }
}

/// Получить ключ для HMAC подписи конфигурации управления из переменной окружения.
fn get_controls_hmac_key_runtime() -> &'static String {
    get_hmac_key_runtime("TETRIS_CONTROLS_HMAC_KEY", "TETRIS_CONTROLS_HMAC_KEY")
}

/// Получить ключ для HMAC подписи таблицы лидеров из переменной окружения.
fn get_leaderboard_hmac_key_runtime() -> &'static String {
    get_hmac_key_runtime("TETRIS_LEADERBOARD_HMAC_KEY", "TETRIS_LEADERBOARD_HMAC_KEY")
}

/// Получить ключ для HMAC подписи данных рекордов из переменной окружения.
fn get_save_data_hmac_key_runtime() -> &'static String {
    get_hmac_key_runtime("TETRIS_SAVEDATA_HMAC_KEY", "TETRIS_SAVEDATA_HMAC_KEY")
}

/// Получить ключ для HMAC подписи конфигурации управления.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_CONTROLS_HMAC_KEY` или пустую строку если не установлен
///
/// # Исправление #6
/// Использует runtime `env::var()` вместо compile-time `option_env!`.
#[must_use]
pub fn get_controls_hmac_key() -> &'static str {
    get_controls_hmac_key_runtime().as_str()
}

/// Получить ключ для HMAC подписи таблицы лидеров.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_LEADERBOARD_HMAC_KEY` или пустую строку если не установлен
///
/// # Исправление #6
/// Использует runtime `env::var()` вместо compile-time `option_env!`.
#[must_use]
pub fn get_leaderboard_hmac_key() -> &'static str {
    get_leaderboard_hmac_key_runtime().as_str()
}

/// Получить ключ для HMAC подписи данных рекордов.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_SAVEDATA_HMAC_KEY` или пустую строку если не установлен
///
/// # Исправление #6
/// Использует runtime `env::var()` вместо compile-time `option_env!`.
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
/// # Исправление аудита 2026-04-01 (C1)
/// Добавлена валидация HMAC ключей при запуске с проверкой наличия ключа.
///
/// # Исправление NEW-146 (2026-04-02)
/// Добавлена проверка минимальной длины ключа (16 байт).
///
/// # Errors
/// Возвращает ошибку если ключ пустой, содержит только пробельные символы
/// или короче минимальной длины.
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
/// # Правила валидации
/// ## Проверка для каждого ключа
/// 1. **Наличие**: Ключ не должен быть пустым или содержать только пробелы
/// 2. **Минимальная длина**: Ключ должен быть не короче `MIN_HMAC_KEY_LENGTH` (16 байт)
/// 3. **Проверка всех трёх ключей**:
///    - `CONTROLS_HMAC_KEY` - для конфигурации управления
///    - `LEADERBOARD_HMAC_KEY` - для таблицы лидеров
///    - `SAVE_DATA_HMAC_KEY` - для данных рекордов
///
/// ## Критерии отказа
/// - Пустой ключ (только пробелы)
/// - Ключ короче 16 байт
/// - Отсутствие переменной окружения `TETRIS_HMAC_KEY`
///
/// # Возвращает
/// - `Ok(())` если все ключи прошли валидацию
/// - `Err(Vec<String>)` с описанием всех обнаруженных проблем
///
/// # Исправление аудита 2026-04-01 (C1)
/// Добавлена функция для валидации всех ключей при запуске.
///
/// # Намеренное поведение (Исправление аудита 2026-04-02)
/// Эта функция возвращает `Result` и вызывающий код (`Application::new()`)
/// обрабатывает ошибку логированием предупреждения. Приложение продолжает
/// работу с пустым HMAC ключом — это корректно для dev-режима.
/// В продакшене рекомендуется устанавливать HMAC ключи через переменные окружения.
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
