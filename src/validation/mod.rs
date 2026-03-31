//! Модуль валидации данных.
//!
//! Предоставляет функции и структуры для валидации:
//! - Имён игроков (ASCII/кириллица, запрещённые символы)
//! - Путей к файлам (длина, символы, symlink, path traversal)
//! - Числовых значений (f32 finite, u32 range)
//!
//! ## Структура модуля
//! - [`name`] — валидация имён игроков
//! - [`path`] — валидация путей к файлам
//! - [`ValidationService`] — централизованный сервис валидации

// Подмодули
pub mod name;
pub mod path;

// Re-export для удобства использования
pub use name::is_valid_name_char;
pub use name::sanitize_player_name;

// ============================================================================
// VALIDATION SERVICE — ЦЕНТРАЛИЗОВАННЫЙ СЕРВИС ВАЛИДАЦИИ
// ============================================================================

/// Ошибка валидации числовых значений.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    /// Сообщение об ошибке.
    pub message: String,
    /// Тип ошибки.
    pub kind: ValidationErrorKind,
}

/// Типы ошибок валидации числовых значений.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorKind {
    /// Значение не является конечным (NaN или Infinity).
    NotFinite,
    /// Значение вне допустимого диапазона.
    OutOfRange,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ошибка валидации: {} ({:?})", self.message, self.kind)
    }
}

impl std::error::Error for ValidationError {}

/// Централизованный сервис валидации данных.
///
/// Предоставляет универсальные функции валидации для использования
/// во всём проекте (DRY принцип).
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::validation::ValidationService;
///
/// // Валидация f32 значения
/// ValidationService::validate_f32_finite(1.0)?;
///
/// // Валидация u32 диапазона
/// ValidationService::validate_u32_range(5, 1, 10)?;
/// ```
pub struct ValidationService;

impl ValidationService {
    /// Валидировать f32 значение на конечность (не NaN и не Infinity).
    ///
    /// # Аргументы
    /// * `value` - значение для валидации
    ///
    /// # Возвращает
    /// - `Ok(())` если значение конечное
    /// - `Err(ValidationError)` если значение NaN или Infinity
    ///
    /// # Errors
    /// Возвращает `ValidationError` с `ValidationErrorKind::NotFinite`
    /// если значение является NaN или Infinity.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::validation::{ValidationService, ValidationError};
    ///
    /// assert!(ValidationService::validate_f32_finite(1.0).is_ok());
    /// assert!(ValidationService::validate_f32_finite(f32::NAN).is_err());
    /// assert!(ValidationService::validate_f32_finite(f32::INFINITY).is_err());
    /// ```
    pub fn validate_f32_finite(value: f32) -> Result<(), ValidationError> {
        if value.is_nan() || value.is_infinite() {
            return Err(ValidationError {
                message: format!("Значение {} не является конечным (NaN/Infinity)", value),
                kind: ValidationErrorKind::NotFinite,
            });
        }
        Ok(())
    }

    /// Валидировать u32 значение на попадание в диапазон.
    ///
    /// # Аргументы
    /// * `value` - значение для валидации
    /// * `min` - минимальное допустимое значение (включительно)
    /// * `max` - максимальное допустимое значение (включительно)
    ///
    /// # Возвращает
    /// - `Ok(())` если значение в диапазоне
    /// - `Err(ValidationError)` если значение вне диапазона
    ///
    /// # Errors
    /// Возвращает `ValidationError` с `ValidationErrorKind::OutOfRange`
    /// если значение меньше min или больше max.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::validation::{ValidationService, ValidationError};
    ///
    /// assert!(ValidationService::validate_u32_range(5, 1, 10).is_ok());
    /// assert!(ValidationService::validate_u32_range(0, 1, 10).is_err());
    /// assert!(ValidationService::validate_u32_range(11, 1, 10).is_err());
    /// ```
    pub fn validate_u32_range(value: u32, min: u32, max: u32) -> Result<(), ValidationError> {
        if value < min || value > max {
            return Err(ValidationError {
                message: format!(
                    "Значение {} вне допустимого диапазона [{}, {}]",
                    value, min, max
                ),
                kind: ValidationErrorKind::OutOfRange,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod validation_service_tests {
    use super::*;

    #[test]
    fn test_validate_f32_finite_valid() {
        assert!(ValidationService::validate_f32_finite(0.0).is_ok());
        assert!(ValidationService::validate_f32_finite(1.0).is_ok());
        assert!(ValidationService::validate_f32_finite(-1.0).is_ok());
        assert!(ValidationService::validate_f32_finite(f32::MAX).is_ok());
        assert!(ValidationService::validate_f32_finite(f32::MIN).is_ok());
    }

    #[test]
    fn test_validate_f32_finite_nan() {
        let result = ValidationService::validate_f32_finite(f32::NAN);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::NotFinite);
    }

    #[test]
    fn test_validate_f32_finite_infinity() {
        let result = ValidationService::validate_f32_finite(f32::INFINITY);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::NotFinite);

        let result = ValidationService::validate_f32_finite(f32::NEG_INFINITY);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::NotFinite);
    }

    #[test]
    fn test_validate_u32_range_valid() {
        assert!(ValidationService::validate_u32_range(5, 1, 10).is_ok());
        assert!(ValidationService::validate_u32_range(1, 1, 10).is_ok());
        assert!(ValidationService::validate_u32_range(10, 1, 10).is_ok());
        assert!(ValidationService::validate_u32_range(0, 0, 0).is_ok());
    }

    #[test]
    fn test_validate_u32_range_out_of_range() {
        let result = ValidationService::validate_u32_range(0, 1, 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::OutOfRange);

        let result = ValidationService::validate_u32_range(11, 1, 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::OutOfRange);
    }
}
