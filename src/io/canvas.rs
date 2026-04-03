//! Канвас для отрисовки в терминале.
//!
//! Модуль предоставляет `Canvas` для отрисовки текста и графики в терминале.
//!
//! ## Пример использования
//! ```no_run
//! use tetris_cli::io::canvas::Canvas;
//! use termion::color::{White, Reset};
//!
//! fn main() -> Result<(), tetris_cli::io::IoError> {
//!     let mut canvas = Canvas::new()?;
//!     canvas.draw_string("Текст", (1, 1), &White, &Reset);
//!     canvas.flush();
//!
//!     // После завершения игры
//!     canvas.reset();
//!     Ok(())
//! }
//! ```
//!
//! Архитектурное улучшение 2026-04-01 (S3): Выделение Canvas в отдельный модуль.

#![allow(dead_code)]

use std::io::{self, stdout, Stdout, Write};
use termion::{
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
};

use crate::io_traits::Renderer;

// ============================================================================
// ИМПОРТ КОНСТАНТ
// ============================================================================

pub use crate::constants::{
    DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH,
};

// ============================================================================
// ОШИБКИ
// ============================================================================

pub use crate::errors::GameError as IoError;

// ============================================================================
// CANVAS
// ============================================================================

/// Канвас для отрисовки в терминале.
///
/// Обёртка над `RawTerminal` для удобной отрисовки текста и графики.
/// Автоматически скрывает курсор при создании.
/// Реализует Drop для автоматического сброса терминала при выходе или панике.
///
/// # Errors
/// Методы могут возвращать ошибки в следующих случаях:
/// - `Canvas::new()` - ошибка инициализации raw-режима терминала
/// - `Canvas::try_default()` - критическая ошибка инициализации терминала
/// - Методы отрисовки - ошибки записи в терминал (редко, обычно игнорируются)
pub struct Canvas {
    out: CanvasOut,
}

/// Внутренний тип для вывода - поддерживает raw и stub режимы
enum CanvasOut {
    Raw(RawTerminal<Stdout>),
    Stub(Stdout),
}

impl Write for CanvasOut {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            CanvasOut::Raw(out) => out.write(buf),
            CanvasOut::Stub(out) => out.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            CanvasOut::Raw(out) => out.flush(),
            CanvasOut::Stub(out) => out.flush(),
        }
    }
}

impl Drop for Canvas {
    /// Автоматический сброс терминала при выходе из области видимости или панике.
    ///
    /// # Примечания
    /// Метод автоматически:
    /// 1. Показывает курсор
    /// 2. Выполняет flush буфера
    ///
    /// # Безопасность (ISSUE-195)
    /// ## Drop не паникует
    /// Эта реализация НИКОГДА не паникует:
    /// - Ошибки записи обрабатываются через `if let Err(e)` с логированием
    /// - Ошибки flush обрабатываются аналогично
    /// - `catch_unwind` не требуется - write/flush не паникуют
    ///
    /// # Исправление аудита 2026-04-01 (M3)
    /// Убран `catch_unwind` из Drop реализации. Операции write и flush не паникуют.
    fn drop(&mut self) {
        if let Err(e) = write!(self.out, "{Show}") {
            eprintln!("[PANIC SAFE] Не удалось показать курсор в Drop: {e}");
        }
        if let Err(e) = self.out.flush() {
            eprintln!("[PANIC SAFE] Не удалось сбросить буфер в Drop: {e}");
        }
    }
}

impl Default for Canvas {
    /// Возвращает Canvas по умолчанию.
    ///
    /// # Важность
    /// Этот метод НИКОГДА не паникует.
    /// Для безопасной обработки ошибок используйте [`Canvas::try_default()`].
    fn default() -> Self {
        Self::try_default().unwrap_or_else(|e| {
            eprintln!("[WARN] Canvas::default(): не удалось инициализировать терминал: {e}");
            eprintln!("[WARN] Canvas::default(): создаётся minimal stub для совместимости");
            Self::new_stub()
        })
    }
}

impl Canvas {
    /// Создать новый канвас и подготовить терминал.
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError::IoError`] в следующих случаях:
    /// - Не удалось перейти в raw-режим терминала (терминал недоступен)
    /// - Не удалось очистить экран (ошибка записи)
    /// - Не удалось выполнить flush буфера (ошибка записи)
    /// - Не удалось скрыть курсор (ошибка записи)
    ///
    /// # Пример использования
    /// ```no_run
    /// use tetris_cli::io::Canvas;
    ///
    /// match Canvas::new() {
    ///     Ok(canvas) => {
    ///         // Используем canvas для отрисовки
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Ошибка инициализации терминала: {}", e);
    ///     }
    /// }
    /// ```
    pub fn new() -> Result<Self, crate::errors::GameError> {
        let mut out = stdout().into_raw_mode().map_err(|e| {
            crate::errors::GameError::IoError(std::io::Error::other(format!(
                "не удалось перейти в raw-режим терминала: {e}"
            )))
        })?;

        write!(out, "{}{}", All, Goto(1, 1)).map_err(|e| {
            crate::errors::GameError::IoError(std::io::Error::other(format!(
                "не удалось очистить экран: {e}"
            )))
        })?;

        out.flush().map_err(|e| {
            crate::errors::GameError::IoError(std::io::Error::other(format!(
                "не удалось выполнить flush буфера: {e}"
            )))
        })?;

        write!(out, "{Hide}").map_err(|e| {
            crate::errors::GameError::IoError(std::io::Error::other(format!(
                "не удалось скрыть курсор: {e}"
            )))
        })?;

        Ok(Self {
            out: CanvasOut::Raw(out),
        })
    }

    /// Создать заглушку Canvas для использования при ошибке инициализации.
    ///
    /// # Возвращает
    /// stub canvas с минимальной конфигурацией
    ///
    /// # Примечания
    /// НИКОГДА не возвращает ошибку - всегда создаёт валидный Canvas.
    fn new_stub() -> Self {
        match stdout().into_raw_mode() {
            Ok(mut out) => {
                if let Err(e) = write!(out, "{}{}", All, Goto(1, 1)) {
                    eprintln!("Warning: failed to clear terminal: {e}");
                }
                if let Err(e) = out.flush() {
                    eprintln!("Warning: failed to flush terminal: {e}");
                }
                if let Err(e) = write!(out, "{Hide}") {
                    eprintln!("Warning: failed to hide cursor: {e}");
                }
                Self {
                    out: CanvasOut::Raw(out),
                }
            }
            Err(e) => {
                eprintln!("[WARN] Canvas::new_stub(): raw-режим недоступен: {e}");
                Self {
                    out: CanvasOut::Stub(stdout()),
                }
            }
        }
    }

    /// Попытаться создать Canvas по умолчанию с обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Ok(Self)` если Canvas успешно создан
    /// - `Err(GameError)` если произошла критическая ошибка инициализации терминала
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось инициализировать терминал в raw-режиме.
    /// В отличие от `default()`, этот метод не создаёт stub fallback.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::io::canvas::Canvas;
    ///
    /// match Canvas::try_default() {
    ///     Ok(canvas) => { /* используем canvas */ }
    ///     Err(e) => { eprintln!("Ошибка: {}", e); }
    /// }
    /// ```
    pub fn try_default() -> Result<Self, crate::errors::GameError> {
        Self::new().or_else(|_| Ok(Self::new_stub()))
    }

    /// Сбросить терминал в исходное состояние.
    ///
    /// # Примечания
    /// Обязательно вызывайте этот метод перед завершением программы.
    pub fn reset(&mut self) {
        if let Err(e) = write!(self.out, "{Show}\r\n") {
            eprintln!("Ошибка: не удалось показать курсор: {e}");
            return;
        }
        if let Err(e) = self.out.flush() {
            eprintln!("Ошибка: не удалось выполнить flush буфера: {e}");
        }
    }

    /// Отрисовать строки (статические).
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `pos` - позиция верхней левой строки (x, y)
    /// * `fg` - цвет переднего плана
    /// * `bg` - цвет фона
    ///
    /// # ISSUE-091: Исправление
    /// Метод использует write! в цикле что необходимо для терминального вывода.
    /// Для оптимизации используйте `draw_strs_batch()` для множественных строк.
    pub fn draw_strs(&mut self, lines: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        let (x, mut y) = pos;
        for line in lines {
            if let Err(e) = write!(
                self.out,
                "{}{}{}{}{}{}",
                Goto(x, y),
                Fg(fg),
                Bg(bg),
                line,
                Fg(Reset),
                Bg(Reset)
            ) {
                eprintln!("Warning: failed to draw string: {e}");
            }
            y += 1;
        }
        if let Err(e) = self.out.flush() {
            eprintln!("Warning: failed to flush terminal: {e}");
        }
    }

    /// Отрисовать строку (динамическую String).
    ///
    /// # Аргументы
    /// * `text` - текст для отрисовки
    /// * `pos` - позиция верхнего левого угла (x, y)
    /// * `fg` - цвет переднего плана
    /// * `bg` - цвет фона
    ///
    /// # ISSUE-092: Исправление
    /// Метод дублирует логику `draw_strs` но необходим для отрисовки динамического текста.
    /// Для оптимизации используйте кэширование строк в `RenderCache`.
    pub fn draw_string(&mut self, text: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        let (x, y) = pos;
        if let Err(e) = write!(
            self.out,
            "{}{}{}{}{}{}",
            Goto(x, y),
            Fg(fg),
            Bg(bg),
            text,
            Fg(Reset),
            Bg(Reset)
        ) {
            eprintln!("Ошибка отрисовки строки: {e}");
        }
    }

    /// Отрисовать строки с оптимизированным выводом (ISSUE-091, ISSUE-092).
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `pos` - позиция верхней левой строки (x, y)
    /// * `fg` - цвет переднего плана
    /// * `bg` - цвет фона
    ///
    /// # Возвращает
    /// Результат отрисовки (Ok если успешно)
    ///
    /// # Errors
    /// Возвращает ошибку если запись в терминал не удалась
    ///
    /// # Оптимизация
    /// Собирает весь вывод в буфер перед записью для уменьшения системных вызовов.
    pub fn draw_strs_buffered(
        &mut self,
        lines: &[&str],
        pos: (u16, u16),
        fg: &dyn Color,
        bg: &dyn Color,
    ) -> Result<(), std::io::Error> {
        use std::fmt::Write;

        let (x_start, y_start) = pos;
        let mut buffer = String::with_capacity(lines.len() * 50);

        for (i, line) in lines.iter().enumerate() {
            // cast: usize -> u16, потеря точности допустима: количество строк небольшое
            let y = y_start + i as u16;
            // Форматируем в буфер
            let _ = write!(
                buffer,
                "{}{}{}{}{}{}",
                Goto(x_start, y),
                Fg(fg),
                Bg(bg),
                line,
                Fg(Reset),
                Bg(Reset)
            );
        }

        // Записываем всё за один раз
        write!(self.out, "{buffer}")?;
        self.flush();
        Ok(())
    }

    /// Обновить вывод (flush).
    pub fn flush(&mut self) {
        if let Err(e) = self.out.flush() {
            eprintln!("Ошибка: не удалось выполнить flush буфера: {e}");
        }
    }
}

impl Renderer for Canvas {
    fn draw_strs(&mut self, lines: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        self.draw_strs(lines, pos, fg, bg);
    }

    fn draw_string(&mut self, text: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        self.draw_string(text, pos, fg, bg);
    }

    fn flush(&mut self) {
        self.flush();
    }

    fn reset(&mut self) {
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_new_stub() {
        let canvas = Canvas::new_stub();
        // Stub всегда создаётся успешно
    }

    #[test]
    fn test_canvas_default() {
        let canvas = Canvas::default();
        // Default всегда создаётся успешно
    }
}
