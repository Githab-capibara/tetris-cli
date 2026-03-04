//! Обработка ввода и вывода.
//!
//! Автор: Dylan Turner

use termion::{
    clear::All,
    cursor::{ Goto, Hide, Show },
    raw::{ RawTerminal, IntoRawMode },
    color::{ Color, Fg, Bg, Reset },
    async_stdin, AsyncReader
};
use std::io::{ Write, stdout, Stdout, Read };

/// Строковое представление блока фигуры.
pub const SHAPE_STR: &str = "██";
/// Ширина блока в символах.
pub const SHAPE_WIDTH: usize = 2;
/// Ширина игрового поля в блоках.
pub const GRID_WIDTH: usize = 10;
/// Высота игрового поля в блоках.
pub const GRID_HEIGHT: usize = 20;
/// Полная ширина дисплея (с учётом границ).
pub const DISP_WIDTH: u16 = (SHAPE_WIDTH * GRID_WIDTH) as u16 + 2;
/// Полная высота дисплея (с учётом границ и отступов).
pub const DISP_HEIGHT: u16 = GRID_HEIGHT as u16 + 5;

/// Канвас для отрисовки в терминале.
pub struct Canvas {
    out: RawTerminal<Stdout>
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    /// Создать новый канвас и подготовить терминал.
    pub fn new() -> Self {
        let mut out = stdout().into_raw_mode().expect("Не удалось перейти в raw-режим");
        write!(out, "{}{}", All, Goto(1, 1)).expect("Не удалось очистить экран");
        out.flush().expect("Не удалось выполнить flush");

        write!(out, "{}", Hide).expect("Не удалось скрыть курсор");

        Self { out }
    }

    /// Сбросить терминал в исходное состояние.
    pub fn reset(&mut self) {
        write!(self.out, "{}\r\n", Show).expect("Не удалось показать курсор");
        self.out.flush().expect("Не удалось выполнить flush");
    }

    /// Отрисовать строки (статические).
    pub fn draw_strs(
            &mut self, lines: &[&str], pos: (u16, u16),
            fg: &dyn Color, bg: &dyn Color) {
        let (x, mut y) = pos;
        for line in lines {
            write!(
                self.out, "{}{}{}{}{}{}",
                Goto(x, y), Fg(fg), Bg(bg), line,
                Fg(Reset), Bg(Reset)
            ).expect("Не удалось отрисовать строку");
            y += 1;
        }
    }

    /// Отрисовать строку (динамическую String).
    pub fn draw_string(
            &mut self, text: &str, pos: (u16, u16),
            fg: &dyn Color, bg: &dyn Color) {
        let (x, y) = pos;
        write!(
            self.out, "{}{}{}{}{}{}",
            Goto(x, y), Fg(fg), Bg(bg), text,
            Fg(Reset), Bg(Reset)
        ).expect("Не удалось отрисовать строку");
    }

    /// Обновить вывод (flush).
    pub fn flush(&mut self) {
        self.out.flush().expect("Не удалось выполнить flush");
    }
}

/// Читатель нажатий клавиш в асинхронном режиме.
pub struct KeyReader {
    inp: AsyncReader
}

impl Default for KeyReader {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyReader {
    /// Создать новый читатель клавиш.
    pub fn new() -> Self {
        let inp = async_stdin();
        Self { inp }
    }

    /// Получить код нажатой клавиши.
    /// Возвращает 0 при ошибке чтения.
    pub fn get_key(&mut self) -> u8 {
        let mut key_bytes: [u8; 1] = [ 0 ];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(_) => key_bytes[0],
            Err(_) => 0
        }
    }
}
