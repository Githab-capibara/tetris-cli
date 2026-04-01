//! Модуль ввода/вывода для работы с терминалом.
//!
//! Этот модуль предоставляет абстракции для работы с терминалом:
//! - [`Canvas`] - канвас для отрисовки
//! - [`KeyReader`] - асинхронный читатель клавиатуры
//!
//! ## Архитектурные заметки
//! Архитектурное улучшение 2026-04-01 (S3): Разделение io.rs на отдельные модули:
//! - `canvas.rs` - отрисовка в терминале
//! - `key_reader.rs` - чтение клавиатуры
//!
//! ## Пример использования
//! ```no_run
//! use tetris_cli::io::{Canvas, KeyReader};
//!
//! let mut canvas = Canvas::new().unwrap();
//! let mut reader = KeyReader::new();
//! ```

#![allow(dead_code)]

// Подмодули
pub mod canvas;
pub mod key_reader;

// Re-export основных типов для обратной совместимости
pub use canvas::Canvas;
pub use key_reader::KeyReader;

// Re-export констант
pub use crate::constants::{
    DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, KEY_BACKSPACE, SHAPE_STR, SHAPE_WIDTH,
};

// Re-export ошибок
pub use crate::errors::GameError as IoError;

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_module_exports() {
        // Проверка что экспорты работают
        let _canvas = Canvas::default();
        let _reader = KeyReader::default();
    }
}
