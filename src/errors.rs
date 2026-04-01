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

#![allow(dead_code)]

use thiserror::Error;

/// Ошибка игры.
///
/// Представляет собой основной тип ошибок для проекта tetris-cli.
/// Поддерживает автоматическую конвертацию из `std::io::Error`.
///
/// ## Варианты ошибок
/// - `ValidationError` - ошибка валидации данных
/// - `IoError` - ошибка ввода/вывода
/// - `ScoreOverflow` - переполнение счёта
#[derive(Error, Debug)]
pub enum GameError {
    /// Ошибка валидации данных.
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),

    /// Ошибка ввода/вывода.
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка переполнения счёта.
    #[error("Переполнение счёта: попытка превышения максимального значения")]
    ScoreOverflow,
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
}
