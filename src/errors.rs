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
/// ## Варианты ошибок
/// - `ValidationError` - ошибка валидации данных
/// - `IoError` - ошибка ввода/вывода
/// - `ScoreOverflow` - переполнение счёта
///
/// ## Пример использования
/// ```ignore
/// use tetris_cli::errors::GameError;
///
/// fn validate_score(score: u128) -> Result<(), GameError> {
///     if score > 1_000_000 {
///         return Err(GameError::ValidationError(
///             "Счёт слишком большой".to_string()
///         ));
///     }
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
pub enum GameError {
    /// Ошибка валидации данных.
    ///
    /// Возникает при некорректных входных данных или нарушении инвариантов.
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),

    /// Ошибка ввода/вывода.
    ///
    /// Автоматически конвертируется из `std::io::Error`.
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка переполнения счёта.
    ///
    /// Возникает при попытке добавить очки, превышающие допустимый предел.
    #[error("Переполнение счёта: попытка превышения максимального значения")]
    ScoreOverflow,
}

impl GameError {
    /// Создать ошибку валидации с сообщением.
    ///
    /// # Аргументы
    /// * `message` - сообщение об ошибке
    ///
    /// # Возвращает
    /// Новый экземпляр `GameError::ValidationError`
    ///
    /// # Пример
    /// ```ignore
    /// let err = GameError::validation_error("Некорректное имя игрока");
    /// ```
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }

    /// Создать ошибку переполнения счёта.
    ///
    /// # Возвращает
    /// Новый экземпляр `GameError::ScoreOverflow`
    ///
    /// # Пример
    /// ```ignore
    /// let err = GameError::score_overflow();
    /// ```
    pub fn score_overflow() -> Self {
        Self::ScoreOverflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let err = GameError::validation_error("Тестовая ошибка");
        assert!(matches!(err, GameError::ValidationError(_)));
        assert!(err.to_string().contains("Тестовая ошибка"));
    }

    #[test]
    fn test_score_overflow() {
        let err = GameError::score_overflow();
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
