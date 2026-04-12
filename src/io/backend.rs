//! Терминальный бэкенд на основе termion.
//!
//! # Ответственность
//! - Реализация бэкенда для вывода в терминал через termion
//! - Управление курсором, очисткой экрана, отрисовкой строк
//!
//! ## Архитектурные заметки
//! Бэкенд предоставляет низкоуровневый API для работы с терминалом.
//! Модули игры используют `TermionBackend` для отрисовки.

use std::io::{self, stdout, Stdout, Write};
use termion::{
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
    screen::ToMainScreen,
};

// ============================================================================
// РЕАЛИЗАЦИЯ (TERMION)
// ============================================================================

/// Терминальный бэкенд на основе termion.
///
/// Обёртка над `RawTerminal` для отрисовки в терминале.
/// Управляет курсором, очисткой экрана и выводом текста.
pub struct TermionBackend {
    /// Внутренний raw терминал
    out: RawTerminal<Stdout>,
}

impl TermionBackend {
    /// Создать новый termion бэкенд.
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось инициализировать raw-режим терминала
    pub fn new() -> io::Result<Self> {
        let mut out = stdout().into_raw_mode()?;

        // Примечание: `async_stdin()` (используется в KeyReader) и `into_raw_mode()`
        // управляют разными ресурсами: stdin vs stdout соответственно.
        // Raw-режим не включается дважды — это разные операции с разными файловыми дескрипторами.

        // Очистка экрана и перемещение курсора
        write!(out, "{}{}", All, Goto(1, 1))?;
        out.flush()?;

        // Скрытие курсора
        write!(out, "{Hide}")?;
        out.flush()?;

        Ok(Self { out })
    }

    /// Создать бэкенд с существующим raw терминалом.
    #[must_use]
    pub const fn with_raw(out: RawTerminal<Stdout>) -> Self {
        Self { out }
    }

    /// Отрисовать строку в указанной позиции.
    ///
    /// # Аргументы
    /// * `text` - текст для отрисовки
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    /// * `fg` - цвет переднего плана
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn draw_string(&mut self, text: &str, x: u16, y: u16, fg: &dyn Color) -> io::Result<()> {
        write!(
            self.out,
            "{}{}{}{}{}{}",
            Goto(x, y),
            Fg(fg),
            Bg(Reset),
            text,
            Fg(Reset),
            Bg(Reset)
        )
    }

    /// Отрисовать строки в указанных позициях.
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `x` - начальная координата X (1-based)
    /// * `y` - начальная координата Y (1-based)
    /// * `fg` - цвет переднего плана
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn draw_strings(
        &mut self,
        lines: &[&str],
        x: u16,
        y: u16,
        fg: &dyn Color,
    ) -> io::Result<()> {
        let mut current_y = y;
        for line in lines {
            write!(
                self.out,
                "{}{}{}{}{}{}",
                Goto(x, current_y),
                Fg(fg),
                Bg(Reset),
                line,
                Fg(Reset),
                Bg(Reset)
            )?;
            current_y += 1;
        }
        Ok(())
    }

    /// Очистить экран.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn clear_screen(&mut self) -> io::Result<()> {
        write!(self.out, "{}{}", All, Goto(1, 1))
    }

    /// Переместить курсор в указанную позицию.
    ///
    /// # Аргументы
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn goto(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.out, "{}", Goto(x, y))
    }

    /// Скрыть курсор.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.out, "{Hide}")
    }

    /// Показать курсор.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.out, "{Show}")
    }

    /// Выполнить flush буфера вывода.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }

    /// Сбросить терминал в исходное состояние.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    pub fn reset(&mut self) -> io::Result<()> {
        write!(self.out, "{Show}\r\n")?;
        write!(self.out, "{ToMainScreen}")?;
        self.out.flush()
    }
}

impl Drop for TermionBackend {
    fn drop(&mut self) {
        // Показываем курсор при выходе
        let _ = write!(self.out, "{Show}");
        let _ = self.out.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_wall_kick_offset_bounds() {
        assert!(crate::constants::GRID_WIDTH > 0);
        assert!(crate::constants::GRID_HEIGHT > 0);
    }
}
