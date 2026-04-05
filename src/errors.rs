//! Типы ошибок для tetris-cli.
//!
//! Этот модуль предоставляет централизованные типы ошибок для обработки ошибок
//! в проекте с использованием библиотеки thiserror.
//!
//! ## Использование
//! ```ignore
//! use tetris_cli::errors::GameError;
//! use std::fs::File;
//!
//! fn load_config() -> Result<(), GameError> {
//!     let file = File::open("config.toml")?; // Автоматическая конвертация из io::Error
//!     // ...
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// Ошибка игры.
///
/// Представляет собой основной тип ошибок для проекта tetris-cli.
/// Поддерживает автоматическую конвертацию из `std::io::Error`.
///
/// # Сопоставление с образцом (Pattern Matching)
/// Этот enum предназначен для использования с pattern matching:
///
/// ```ignore
/// use tetris_cli::errors::GameError;
///
/// fn handle_error(err: &GameError) {
///     match err {
///         GameError::ValidationError(msg) => {
///             eprintln!("Ошибка валидации: {}", msg);
///         }
///         GameError::IoError(io_err) => {
///             eprintln!("Ошибка ввода/вывода: {}", io_err);
///         }
///         GameError::ScoreOverflow => {
///             eprintln!("Переполнение счёта!");
///         }
///     }
/// }
/// ```
///
/// ## Варианты ошибок
/// - `ValidationError(String)` - ошибка валидации данных (содержит сообщение)
/// - `IoError(std::io::Error)` - ошибка ввода/вывода (автоматическая конвертация)
/// - `ScoreOverflow` - переполнение счёта (без данных)
/// - `TerminalError(String)` - ошибка терминала (содержит сообщение)
/// - `InputError(String)` - ошибка ввода (содержит сообщение)
/// - `ConfigError(String)` - ошибка конфигурации (содержит сообщение)
///
/// ## Конвертация из `io::Error`
/// Этот enum содержит трейт `From<std::io::Error>` автоматически через атрибут `#[from]`:
/// ```ignore
/// use std::fs::File;
/// use tetris_cli::errors::GameError;
///
/// fn load_config() -> Result<(), GameError> {
///     let file = File::open("config.toml")?; // Автоматическая конвертация из io::Error
///     // ...
///     Ok(())
/// }
/// ```
///
/// ## M5: `PartialEq` не добавлен
/// Enum содержит `std::io::Error`, который не реализует `PartialEq`.
/// Добавление `PartialEq` невозможно без обёртки `IoError` в Arc и кастомной реализации.
///
/// ## S13: Clone не добавлен
/// Enum содержит `std::io::Error`, который не реализует `Clone`.
/// Добавление `Clone` невозможно без обёртки `IoError` в Arc.
#[derive(Error, Debug)]
pub enum GameError {
    /// Ошибка валидации данных.
    ///
    /// Содержит сообщение об ошибке валидации.
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),

    /// Ошибка ввода/вывода.
    ///
    /// Автоматически конвертируется из `std::io::Error` через трейт `From`.
    /// Атрибут `#[from]` генерирует реализацию `From<std::io::Error>` автоматически.
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка переполнения счёта.
    ///
    /// Возникает при попытке превышения максимального значения счёта (`u128::MAX`).
    #[error("Переполнение счёта: попытка превышения максимального значения")]
    ScoreOverflow,

    /// Ошибка терминала.
    ///
    /// Возникает при проблемах с терминалом (неподдерживаемый терминал, ошибки raw-режима).
    #[error("Ошибка терминала: {0}")]
    TerminalError(String),

    /// Ошибка ввода.
    ///
    /// Возникает при проблемах с вводом (недоступность клавиатуры, ошибки чтения).
    #[error("Ошибка ввода: {0}")]
    InputError(String),

    /// Ошибка конфигурации.
    ///
    /// Возникает при проблемах с конфигурацией (невалидный конфиг, ошибки загрузки).
    #[error("Ошибка конфигурации: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let err = GameError::ValidationError("Тестовая ошибка".to_string());
        assert!(matches!(err, GameError::ValidationError(_)));
        assert!(err.to_string().contains("Тестовая ошибка"));
    }

    #[test]
    fn test_score_overflow() {
        let err = GameError::ScoreOverflow;
        assert!(matches!(err, GameError::ScoreOverflow));
        assert!(err.to_string().contains("Переполнение счёта"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::other("Тест IO");
        let err: GameError = io_err.into();
        assert!(matches!(err, GameError::IoError(_)));
        assert!(err.to_string().contains("Тест IO"));
    }

    #[test]
    fn test_error_display() {
        let validation_err = GameError::ValidationError("Ошибка валидации".to_string());
        assert_eq!(
            format!("{validation_err}"),
            "Ошибка валидации: Ошибка валидации"
        );

        let overflow_err = GameError::ScoreOverflow;
        assert_eq!(
            format!("{overflow_err}"),
            "Переполнение счёта: попытка превышения максимального значения"
        );
    }

    #[test]
    fn test_terminal_error() {
        let err = GameError::TerminalError("Неподдерживаемый терминал".to_string());
        assert!(matches!(err, GameError::TerminalError(_)));
        assert!(err.to_string().contains("Неподдерживаемый терминал"));
        assert!(err.to_string().contains("Ошибка терминала"));
    }

    #[test]
    fn test_input_error() {
        let err = GameError::InputError("Клавиатура недоступна".to_string());
        assert!(matches!(err, GameError::InputError(_)));
        assert!(err.to_string().contains("Клавиатура недоступна"));
        assert!(err.to_string().contains("Ошибка ввода"));
    }

    #[test]
    fn test_config_error() {
        let err = GameError::ConfigError("Невалидный файл конфигурации".to_string());
        assert!(matches!(err, GameError::ConfigError(_)));
        assert!(err.to_string().contains("Невалидный файл конфигурации"));
        assert!(err.to_string().contains("Ошибка конфигурации"));
    }
}
