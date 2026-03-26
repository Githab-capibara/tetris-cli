//! Модуль валидации данных.
//!
//! Предоставляет функции и структуры для валидации:
//! - Имён игроков (ASCII/кириллица, запрещённые символы)
//! - Путей к файлам (длина, символы, symlink, path traversal)
//!
//! ## Структура модуля
//! - [`name`] — валидация имён игроков
//! - [`path`] — валидация путей к файлам

// Подмодули
pub mod name;
pub mod path;

// Re-export для удобства использования
pub use name::is_valid_name_char;
pub use path::{PathError, PathErrorKind, PathValidator};
