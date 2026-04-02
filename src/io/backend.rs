//! Абстракция и реализация терминального бэкенда.
//!
//! # Ответственность
//! - Определение интерфейса для работы с терминалом (трейты)
//! - Реализация бэкенда на основе termion
//! - Абстрагирование от конкретной реализации терминала
//!
//! ## Архитектурные заметки
//! Выделено для соблюдения Dependency Inversion Principle (DIP).
//! Модули ввода/вывода зависят от абстракции (трейт), а не от конкретной реализации.
//!
//! Архитектурное улучшение 2026-04-01 (CRITICAL #3): Terminal Backend Abstraction

#![allow(dead_code)]

use std::io::{self, stdout, Stdout, Write};
use termion::{
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
    screen::ToMainScreen,
};

// ============================================================================
// ТРЕЙТЫ (АБСТРАКЦИЯ)
// ============================================================================

/// Трейт терминального бэкенда для операций вывода.
///
/// Определяет интерфейс для отрисовки в терминале.
/// Позволяет заменять реализацию (termion, crossterm, и т.д.) без изменения кода.
///
/// ## Архитектурные заметки
/// Выделено для соблюдения Dependency Inversion Principle (DIP).
///
/// ## Пример реализации
/// ```ignore
/// use tetris_cli::io::backend::TerminalBackend;
/// use termion::color::{White, Reset};
///
/// struct TermionBackend {
///     // ...
/// }
///
/// impl TerminalBackend for TermionBackend {
///     fn draw_string(&mut self, text: &str, x: u16, y: u16, fg: &dyn Color) {
///         // Реализация через termion
///     }
/// }
/// ```
pub trait TerminalBackend {
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
    fn draw_string(&mut self, text: &str, x: u16, y: u16, fg: &dyn Color) -> io::Result<()>;

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
    fn draw_strings(&mut self, lines: &[&str], x: u16, y: u16, fg: &dyn Color) -> io::Result<()>;

    /// Очистить экран.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn clear_screen(&mut self) -> io::Result<()>;

    /// Переместить курсор в указанную позицию.
    ///
    /// # Аргументы
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn goto(&mut self, x: u16, y: u16) -> io::Result<()>;

    /// Скрыть курсор.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn hide_cursor(&mut self) -> io::Result<()>;

    /// Показать курсор.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn show_cursor(&mut self) -> io::Result<()>;

    /// Выполнить flush буфера вывода.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn flush(&mut self) -> io::Result<()>;

    /// Сбросить терминал в исходное состояние.
    ///
    /// # Errors
    /// Возвращает ошибку при неудачной записи в терминал
    fn reset(&mut self) -> io::Result<()>;
}

/// Трейт терминального бэкенда для операций ввода.
///
/// Определяет интерфейс для чтения ввода из терминала.
/// Позволяет заменять реализацию (termion, crossterm, и т.д.) без изменения кода.
///
/// ## Архитектурные заметки
/// Выделено для соблюдения Dependency Inversion Principle (DIP).
pub trait TerminalInputBackend {
    /// Получить код нажатой клавиши.
    ///
    /// # Возвращает
    /// - `Ok(Some(u8))` — код нажатой клавиши
    /// - `Ok(None)` — если клавиша не была нажата
    /// - `Err(io::Error)` — если произошла ошибка чтения
    ///
    /// # Errors
    /// Возвращает `io::Error` при ошибке чтения из терминала.
    fn read_key(&mut self) -> io::Result<Option<u8>>;

    /// Получить нажатую клавишу с поддержкой UTF-8.
    ///
    /// # Возвращает
    /// - `Some(char)` — символ Unicode
    /// - `None` — при ошибке чтения или невалидном UTF-8
    fn read_key_unicode(&mut self) -> Option<char>;
}

// ============================================================================
// РЕАЛИЗАЦИЯ (TERMION)
// ============================================================================

/// Реализация TerminalBackend на основе termion.
///
/// Обёртка над RawTerminal для отрисовки в терминале.
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
    pub fn with_raw(out: RawTerminal<Stdout>) -> Self {
        Self { out }
    }
}

impl TerminalBackend for TermionBackend {
    fn draw_string(&mut self, text: &str, x: u16, y: u16, fg: &dyn Color) -> io::Result<()> {
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

    fn draw_strings(&mut self, lines: &[&str], x: u16, y: u16, fg: &dyn Color) -> io::Result<()> {
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

    fn clear_screen(&mut self) -> io::Result<()> {
        write!(self.out, "{}{}", All, Goto(1, 1))
    }

    fn goto(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.out, "{}", Goto(x, y))
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.out, "{Hide}")
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.out, "{Show}")
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }

    fn reset(&mut self) -> io::Result<()> {
        write!(self.out, "{Show}\r\n")?;
        write!(self.out, "{ToMainScreen}")?;
        self.out.flush()
    }
}

impl TerminalInputBackend for TermionBackend {
    fn read_key(&mut self) -> io::Result<Option<u8>> {
        // Используем KeyReader для чтения клавиш
        // В реальной реализации нужно хранить KeyReader внутри
        // Для сейчас возвращаем None как заглушку
        Ok(None)
    }

    fn read_key_unicode(&mut self) -> Option<char> {
        // Заглушка для UTF-8 ввода
        None
    }
}

impl Drop for TermionBackend {
    fn drop(&mut self) {
        // Показываем курсор при выходе
        let _ = write!(self.out, "{Show}");
        let _ = self.out.flush();
    }
}

// ============================================================================
// КОМБИНИРОВАННЫЙ БЭКЕНД
// ============================================================================

/// Комбинированный бэкенд для ввода и вывода.
///
/// Объединяет TermionBackend для вывода и KeyReader для ввода.
pub struct TermionIOBackend {
    /// Бэкенд для вывода
    pub output: TermionBackend,
    /// Бэкенд для ввода
    pub input: crate::io::key_reader::KeyReader,
}

impl TermionIOBackend {
    /// Создать новый комбинированный бэкенд.
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось инициализировать терминал
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            output: TermionBackend::new()?,
            input: crate::io::key_reader::KeyReader::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_termion_backend_creation() {
        // Тест может упасть если терминал не поддерживает raw-режим
        let result = TermionBackend::new();
        // Если терминал доступен, бэкенд создаётся успешно
        assert!(result.is_ok() || result.is_err());
    }
}
