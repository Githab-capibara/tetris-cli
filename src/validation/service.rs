//! Сервис валидации данных.
#![allow(dead_code)]
//!
//! Предоставляет централизованный сервис для валидации числовых значений.
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::validation::service::ValidationService;
//!
//! // Валидация f32 значения
//! ValidationService::validate_f32_finite(1.0)?;
//!
//! // Валидация u32 диапазона
//! ValidationService::validate_u32_range(5, 1, 10)?;
//! ```

use super::ValidationError;
use super::ValidationErrorKind;

/// Централизованный сервис валидации данных.
///
/// Предоставляет универсальные функции валидации для использования
/// во всём проекте (DRY принцип).
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
    /// use tetris_cli::validation::{ValidationError, service::ValidationService};
    ///
    /// assert!(ValidationService::validate_f32_finite(1.0).is_ok());
    /// assert!(ValidationService::validate_f32_finite(f32::NAN).is_err());
    /// assert!(ValidationService::validate_f32_finite(f32::INFINITY).is_err());
    /// ```
    pub fn validate_f32_finite(value: f32) -> Result<(), ValidationError> {
        if value.is_nan() || value.is_infinite() {
            return Err(ValidationError {
                message: format!("Значение {value} не является конечным (NaN/Infinity)"),
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
    /// use tetris_cli::validation::{ValidationError, service::ValidationService};
    ///
    /// assert!(ValidationService::validate_u32_range(5, 1, 10).is_ok());
    /// assert!(ValidationService::validate_u32_range(0, 1, 10).is_err());
    /// assert!(ValidationService::validate_u32_range(11, 1, 10).is_err());
    /// ```
    pub fn validate_u32_range(value: u32, min: u32, max: u32) -> Result<(), ValidationError> {
        if value < min || value > max {
            return Err(ValidationError {
                message: format!("Значение {value} вне допустимого диапазона [{min}, {max}]"),
                kind: ValidationErrorKind::OutOfRange,
            });
        }
        Ok(())
    }

    /// Валидировать f32 значение на попадание в диапазон.
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
    /// use tetris_cli::validation::{ValidationError, service::ValidationService};
    ///
    /// assert!(ValidationService::validate_f32_range(5.0, 1.0, 10.0).is_ok());
    /// assert!(ValidationService::validate_f32_range(0.0, 1.0, 10.0).is_err());
    /// assert!(ValidationService::validate_f32_range(11.0, 1.0, 10.0).is_err());
    /// ```
    pub fn validate_f32_range(value: f32, min: f32, max: f32) -> Result<(), ValidationError> {
        if value < min || value > max {
            return Err(ValidationError {
                message: format!("Значение {value} вне допустимого диапазона [{min}, {max}]"),
                kind: ValidationErrorKind::OutOfRange,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::ValidationErrorKind;

    #[test]
    fn test_validate_f32_finite_valid() {
        assert!(ValidationService::validate_f32_finite(0.0).is_ok());
        assert!(ValidationService::validate_f32_finite(1.0).is_ok());
        assert!(ValidationService::validate_f32_finite(-1.0).is_ok());
    }

    #[test]
    fn test_validate_f32_finite_nan() {
        let result = ValidationService::validate_f32_finite(f32::NAN);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::NotFinite);
    }

    #[test]
    fn test_validate_u32_range_valid() {
        assert!(ValidationService::validate_u32_range(5, 1, 10).is_ok());
        assert!(ValidationService::validate_u32_range(1, 1, 10).is_ok());
        assert!(ValidationService::validate_u32_range(10, 1, 10).is_ok());
    }

    #[test]
    fn test_validate_u32_range_out_of_range() {
        let result = ValidationService::validate_u32_range(0, 1, 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ValidationErrorKind::OutOfRange);
    }
}
