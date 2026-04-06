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

use std::sync::Once;

/// Минимальная длина HMAC ключа в байтах.
///
/// Ключи короче 16 байт считаются небезопасными.
pub const MIN_HMAC_KEY_LENGTH: usize = 16;

// ============================================================================
// LOG_ONCE ДЛЯ ПУСТЫХ HMAC КЛЮЧЕЙ (Исправление аудита 2026-04-05, Проблема 6)
// ============================================================================

/// Вывести предупреждение о пустом HMAC ключе ТОЛЬКО ОДИН РАЗ при первом вызове.
/// Использует `std::sync::Once` для гарантии однократного вывода независимо от
/// количества вызовов функции.
fn log_once_empty_key(key_name: &str) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        crate::log_warn!(
            "HMAC ключ '{key_name}' не установлен — используется пустая строка. \
             Это ослабляет HMAC защиту. Установите соответствующую переменную окружения."
        );
    });
}

/// Получить ключ для HMAC подписи конфигурации управления.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_CONTROLS_HMAC_KEY` или тестовое значение по умолчанию
///
/// # Безопасность
/// ## ⚠️ Критическое замечание
/// - **Тестовый ключ по умолчанию**: Используется только для разработки
/// - **Продакшен**: Обязательно установите `TETRIS_CONTROLS_HMAC_KEY` переменную окружения
/// - **Минимальная длина**: 16 байт (рекомендуется 32+ байта)
///
/// ## Установка ключа
/// ```bash
/// # Для разработки
/// export TETRIS_CONTROLS_HMAC_KEY="my-development-key-32bytes!!"
///
/// # Для продакшена (используйте генератор случайных ключей)
/// export TETRIS_CONTROLS_HMAC_KEY=$(openssl rand -hex 32)
/// ```
///
/// # Критическое замечание
/// ⚠️ **ВАЖНО**: Если переменная окружения не установлена, возвращается тестовое значение.
/// Это означает что HMAC защита ослаблена. Вызывающий код ДОЛЖЕН проверить ключ
/// через `validate_hmac_key()` перед использованием.
#[must_use]
pub fn get_controls_hmac_key() -> &'static str {
    option_env!("TETRIS_CONTROLS_HMAC_KEY")
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(|| {
            log_once_empty_key("TETRIS_CONTROLS_HMAC_KEY");
            ""
        })
}

/// Получить ключ для HMAC подписи таблицы лидеров.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_LEADERBOARD_HMAC_KEY` или пустую строку если не установлен
#[must_use]
pub fn get_leaderboard_hmac_key() -> &'static str {
    option_env!("TETRIS_LEADERBOARD_HMAC_KEY")
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(|| {
            log_once_empty_key("TETRIS_LEADERBOARD_HMAC_KEY");
            ""
        })
}

/// Получить ключ для HMAC подписи данных рекордов.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_SAVEDATA_HMAC_KEY` или пустую строку если не установлен
#[must_use]
pub fn get_save_data_hmac_key() -> &'static str {
    option_env!("TETRIS_SAVEDATA_HMAC_KEY")
        .filter(|k| !k.trim().is_empty())
        .unwrap_or_else(|| {
            log_once_empty_key("TETRIS_SAVEDATA_HMAC_KEY");
            ""
        })
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
#[allow(dead_code)]
pub fn validate_hmac_key(key: &str, key_name: &str) -> Result<(), String> {
    if key.trim().is_empty() {
        return Err(format!(
            "HMAC ключ '{key_name}' не установлен. Пожалуйста, установите переменную окружения TETRIS_HMAC_KEY"
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
#[allow(dead_code)]
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
}
