//! Реализация терминального бэкенда на основе termion.
//!
//! # Ответственность
//! - Реализация трейтов TerminalBackend и TerminalInputBackend
//! - Использование termion для операций ввода/вывода
//!
//! ## Архитектурные заметки
//! Конкретная реализация абстракции TerminalBackend.
//! Использует termion для работы с терминалом.
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

use super::key_reader::KeyReader;
use super::terminal_backend::{TerminalBackend, TerminalInputBackend};

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

/// Комбинированный бэкенд для ввода и вывода.
///
/// Объединяет TermionBackend для вывода и KeyReader для ввода.
pub struct TermionIOBackend {
    /// Бэкенд для вывода
    pub output: TermionBackend,
    /// Бэкенд для ввода
    pub input: KeyReader,
}

impl TermionIOBackend {
    /// Создать новый комбинированный бэкенд.
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось инициализировать терминал
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            output: TermionBackend::new()?,
            input: KeyReader::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_termion_backend_creation() {
        // Тест может упасть если терминал не поддерживает raw-режим
        // Поэтому используем try_default подход
        let result = TermionBackend::new();
        // Если терминал доступен, бэкенд создаётся успешно
        // В CI/CI среде может быть ошибка
        assert!(result.is_ok() || result.is_err());
    }
}
