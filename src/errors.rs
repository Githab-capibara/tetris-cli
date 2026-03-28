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
/// Поддерживает автоматическую конвертацию из std::io::Error.
///
/// ## Варианты ошибок
/// - `ValidationError` - ошибка валидации данных
/// - `IoError` - ошибка ввода/вывода
/// - `ConfigError` - ошибка конфигурации
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
    /// Автоматически конвертируется из std::io::Error.
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка конфигурации.
    ///
    /// Возникает при загрузке или сохранении конфигурации.
    #[error("Ошибка конфигурации: {0}")]
    ConfigError(String),
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

    /// Создать ошибку конфигурации с сообщением.
    ///
    /// # Аргументы
    /// * `message` - сообщение об ошибке
    ///
    /// # Возвращает
    /// Новый экземпляр `GameError::ConfigError`
    ///
    /// # Пример
    /// ```ignore
    /// let err = GameError::config_error("Файл конфигурации не найден");
    /// ```
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
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
    fn test_config_error() {
        let err = GameError::config_error("Тестовая конфигурация");
        assert!(matches!(err, GameError::ConfigError(_)));
        assert!(err.to_string().contains("Тестовая конфигурация"));
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

        let config_err = GameError::ConfigError("Ошибка конфигурации".to_string());
        assert_eq!(
            format!("{config_err}"),
            "Ошибка конфигурации: Ошибка конфигурации"
        );
    }
}
