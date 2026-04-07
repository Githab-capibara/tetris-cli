//! Модуль валидации данных.
//!
//! Предоставляет функции и структуры для валидации:
//! - Имён игроков (ASCII/кириллица, запрещённые символы)
//! - Путей к файлам (длина, символы, symlink, path traversal)
//!
//! ## Структура модуля
//! - [`name`] — валидация имён игроков
//! - [`path`] — валидация путей к файлам
//! - [`service`] — сервис валидации числовых значений

// Подмодули
pub mod name;
pub mod path;
pub mod service;

// Re-export для удобства использования
pub use name::is_valid_name_char;
pub use path::{PathError, PathErrorKind, PathValidator, DEFAULT_PATH_VALIDATOR};
// ValidationService заменён на свободные функции в module `service`

// ============================================================================
// ОШИБКИ ВАЛИДАЦИИ
// ============================================================================

/// Ошибка валидации числовых значений.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ValidationError {
    /// Значение не является конечным (NaN или Infinity).
    #[error("Значение {value} не является конечным (NaN/Infinity)")]
    NotFinite { value: f32 },
    /// Значение вне допустимого диапазона.
    #[error("Значение {value} вне допустимого диапазона [{min}, {max}]")]
    OutOfRange { value: f64, min: f64, max: f64 },
}
