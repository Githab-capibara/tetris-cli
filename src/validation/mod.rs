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

#![allow(dead_code)]

// Подмодули
pub mod name;
pub mod path;
pub mod service;

// Re-export для удобства использования
pub use name::is_valid_name_char;
pub use service::ValidationService;

// ============================================================================
// ОШИБКИ ВАЛИДАЦИИ
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
