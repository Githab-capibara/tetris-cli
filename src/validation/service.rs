//! Свободные функции валидации данных.
//!
//! Предоставляет функции для валидации числовых значений.
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::validation::service::{validate_f32_finite, validate_u32_range};
//!
//! validate_f32_finite(1.0)?;
//! validate_u32_range(5, 1, 10)?;
//! ```

use super::ValidationError;

/// Валидировать f32 значение на конечность (не `NaN` и не Infinity).
///
/// # Аргументы
/// * `value` - значение для валидации
///
/// # Возвращает
/// - `Ok(())` если значение конечное
/// - `Err(ValidationError)` если значение `NaN` или Infinity
///
/// # Errors
/// Возвращает `ValidationError::NotFinite` если значение является `NaN` или Infinity.
///
/// # Пример
/// ```ignore
/// use tetris_cli::validation::service::validate_f32_finite;
///
/// assert!(validate_f32_finite(1.0).is_ok());
/// assert!(validate_f32_finite(f32::NAN).is_err());
/// assert!(validate_f32_finite(f32::INFINITY).is_err());
/// ```
pub fn validate_f32_finite(value: f32) -> Result<(), ValidationError> {
    if value.is_nan() || value.is_infinite() {
        return Err(ValidationError::NotFinite { value });
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
/// Возвращает `ValidationError::OutOfRange` если значение меньше min или больше max.
///
/// # Пример
/// ```ignore
/// use tetris_cli::validation::service::validate_u32_range;
///
/// assert!(validate_u32_range(5, 1, 10).is_ok());
/// assert!(validate_u32_range(0, 1, 10).is_err());
/// assert!(validate_u32_range(11, 1, 10).is_err());
/// ```
pub fn validate_u32_range(value: u32, min: u32, max: u32) -> Result<(), ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            value: f64::from(value),
            min: f64::from(min),
            max: f64::from(max),
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
/// Возвращает `ValidationError::OutOfRange` если значение меньше min или больше max.
///
/// # Пример
/// ```ignore
/// use tetris_cli::validation::service::validate_f32_range;
///
/// assert!(validate_f32_range(5.0, 1.0, 10.0).is_ok());
/// assert!(validate_f32_range(0.0, 1.0, 10.0).is_err());
/// assert!(validate_f32_range(11.0, 1.0, 10.0).is_err());
/// ```
pub fn validate_f32_range(value: f32, min: f32, max: f32) -> Result<(), ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange {
            value: f64::from(value),
            min: f64::from(min),
            max: f64::from(max),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_f32_finite_valid() {
        assert!(validate_f32_finite(0.0).is_ok());
        assert!(validate_f32_finite(1.0).is_ok());
        assert!(validate_f32_finite(-1.0).is_ok());
    }

    #[test]
    fn test_validate_f32_finite_nan() {
        let result = validate_f32_finite(f32::NAN);
        assert!(matches!(result, Err(ValidationError::NotFinite { .. })));
    }

    #[test]
    fn test_validate_u32_range_valid() {
        assert!(validate_u32_range(5, 1, 10).is_ok());
        assert!(validate_u32_range(1, 1, 10).is_ok());
        assert!(validate_u32_range(10, 1, 10).is_ok());
    }

    #[test]
    fn test_validate_u32_range_out_of_range() {
        let result = validate_u32_range(0, 1, 10);
        assert!(matches!(result, Err(ValidationError::OutOfRange { .. })));
    }
}
