//! Модуль конфигурации HMAC ключей.
//!
//! # Ответственность
//! - Централизованное управление HMAC ключами
//! - Определение констант ключей в одном месте
//! - Предотвращение дублирования ключей
//!
//! # Использование
//! ```ignore
//! use tetris_cli::config::keys::{CONTROLS_HMAC_KEY, LEADERBOARD_HMAC_KEY, SAVE_DATA_HMAC_KEY};
//!
//! let key = CONTROLS_HMAC_KEY;
//! ```

/// Fallback ключ для HMAC подписи конфигурации управления.
///
/// # Безопасность
/// Ключ используется только если переменная окружения `TETRIS_HMAC_KEY` не установлена.
/// Для продакшена рекомендуется устанавливать переменную окружения:
/// ```bash
/// export TETRIS_HMAC_KEY="your-secret-key-here"
/// ```
///
/// # Исправление #2 (CRITICAL)
/// Ключ определён в централизованном модуле для предотвращения дублирования.
///
/// # Исправление аудита 2026-04-01 (S1)
/// ⚠️ **ПРЕДУПРЕЖДЕНИЕ БЕЗОПАСНОСТИ**: В production используйте ТОЛЬКО переменные окружения!
/// Не храните секреты в исходном коде. Fallback ключ предназначен только для разработки.
pub const CONTROLS_HMAC_KEY: &str = "tetris-cli-controls-hmac-key";

/// Fallback ключ для HMAC подписи таблицы лидеров.
///
/// # Безопасность
/// Ключ используется только если переменная окружения `TETRIS_HMAC_KEY` не установлена.
/// Для продакшена рекомендуется устанавливать переменную окружения:
/// ```bash
/// export TETRIS_HMAC_KEY="your-secret-key-here"
/// ```
///
/// # Исправление #2 (CRITICAL)
/// Ключ определён в централизованном модуле для предотвращения дублирования.
///
/// # Исправление аудита 2026-04-01 (S1)
/// ⚠️ **ПРЕДУПРЕЖДЕНИЕ БЕЗОПАСНОСТИ**: В production используйте ТОЛЬКО переменные окружения!
/// Не храните секреты в исходном коде. Fallback ключ предназначен только для разработки.
pub const LEADERBOARD_HMAC_KEY: &str = "tetris-cli-leaderboard-hmac-key";

/// Fallback ключ для HMAC подписи данных рекордов.
///
/// # Безопасность
/// Ключ используется только если переменная окружения `TETRIS_HMAC_KEY` не установлена.
/// Для продакшена рекомендуется устанавливать переменную окружения:
/// ```bash
/// export TETRIS_HMAC_KEY="your-secret-key-here"
/// ```
///
/// # Исправление #2 (CRITICAL)
/// Ключ определён в централизованном модуле для предотвращения дублирования.
///
/// # Исправление аудита 2026-04-01 (S1)
/// ⚠️ **ПРЕДУПРЕЖДЕНИЕ БЕЗОПАСНОСТИ**: В production используйте ТОЛЬКО переменные окружения!
/// Не храните секреты в исходном коде. Fallback ключ предназначен только для разработки.
pub const SAVE_DATA_HMAC_KEY: &str = "tetris-cli-save-data-hmac-key";

/// Получить ключ для HMAC подписи конфигурации управления.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY` или fallback ключ
///
/// # Безопасность
/// Приоритет:
/// 1. Переменная окружения `TETRIS_HMAC_KEY` (если установлена)
/// 2. Fallback ключ `CONTROLS_HMAC_KEY`
///
/// # Исправление #2 (CRITICAL)
/// Функция использует централизованную константу.
#[must_use]
pub fn get_controls_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(CONTROLS_HMAC_KEY)
}

/// Получить ключ для HMAC подписи таблицы лидеров.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY` или fallback ключ
///
/// # Безопасность
/// Приоритет:
/// 1. Переменная окружения `TETRIS_HMAC_KEY` (если установлена)
/// 2. Fallback ключ `LEADERBOARD_HMAC_KEY`
///
/// # Исправление #2 (CRITICAL)
/// Функция использует централизованную константу.
#[must_use]
pub fn get_leaderboard_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(LEADERBOARD_HMAC_KEY)
}

/// Получить ключ для HMAC подписи данных рекордов.
///
/// # Возвращает
/// Ключ из переменной окружения `TETRIS_HMAC_KEY` или fallback ключ
///
/// # Безопасность
/// Приоритет:
/// 1. Переменная окружения `TETRIS_HMAC_KEY` (если установлена)
/// 2. Fallback ключ `SAVE_DATA_HMAC_KEY`
///
/// # Исправление #2 (CRITICAL)
/// Функция использует централизованную константу.
#[must_use]
pub fn get_save_data_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or(SAVE_DATA_HMAC_KEY)
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod keys_tests {
    use super::*;

    #[test]
    fn test_constants_defined() {
        // Проверка что константы определены и не пустые
        assert!(!CONTROLS_HMAC_KEY.is_empty());
        assert!(!LEADERBOARD_HMAC_KEY.is_empty());
        assert!(!SAVE_DATA_HMAC_KEY.is_empty());
    }

    #[test]
    fn test_constants_are_different() {
        // Проверка что ключи разные для разных целей
        assert_ne!(CONTROLS_HMAC_KEY, LEADERBOARD_HMAC_KEY);
        assert_ne!(CONTROLS_HMAC_KEY, SAVE_DATA_HMAC_KEY);
        assert_ne!(LEADERBOARD_HMAC_KEY, SAVE_DATA_HMAC_KEY);
    }

    #[test]
    fn test_get_functions_return_non_empty() {
        // Проверка что функции возвращают не пустые ключи
        assert!(!get_controls_hmac_key().is_empty());
        assert!(!get_leaderboard_hmac_key().is_empty());
        assert!(!get_save_data_hmac_key().is_empty());
    }

    #[test]
    fn test_get_functions_match_constants() {
        // Проверка что функции возвращают те же значения что и константы
        // (когда переменная окружения не установлена)
        assert_eq!(get_controls_hmac_key(), CONTROLS_HMAC_KEY);
        assert_eq!(get_leaderboard_hmac_key(), LEADERBOARD_HMAC_KEY);
        assert_eq!(get_save_data_hmac_key(), SAVE_DATA_HMAC_KEY);
    }
}
