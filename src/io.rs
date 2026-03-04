//! Обработка ввода и вывода.
//!
//! Автор: Dylan Turner
//!
//! Этот модуль предоставляет абстракции для работы с терминалом:
//! - Отрисовка графики и текста
//! - Чтение нажатий клавиш в асинхронном режиме
//! - Управление курсором и цветами
//!
//! ## Структура модуля
//! - Константы размеров игрового поля
//! - `Canvas` - канвас для отрисовки в терминале
//! - `KeyReader` - асинхронный читатель клавиатуры
//!
//! ## Пример использования
//! ```no_run
//! use tetris_cli::io::{Canvas, KeyReader};
//!
//! let mut canvas = Canvas::new();
//! let mut reader = KeyReader::new();
//!
//! // Отрисовка текста
//! use termion::color::{White, Reset};
//! canvas.draw_string("Привет!", (1, 1), &White, &Reset);
//! canvas.flush();
//!
//! // Чтение клавиши
//! let key = reader.get_key();
//! ```

use std::io::{stdout, Read, Stdout, Write};
use termion::{
    async_stdin,
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
    AsyncReader,
};

/// Строковое представление блока фигуры.
///
/// Используется символ "██" (два полных блока) для отрисовки.
/// Каждый блок занимает 2 символа в ширину.
pub const SHAPE_STR: &str = "██";

/// Ширина блока в символах.
///
/// Соответствует длине SHAPE_STR.
pub const SHAPE_WIDTH: usize = 2;

/// Ширина игрового поля в блоках.
///
/// Стандартная ширина для классического тетриса - 10 блоков.
pub const GRID_WIDTH: usize = 10;

/// Высота игрового поля в блоках.
///
/// Стандартная высота для классического тетриса - 20 блоков.
pub const GRID_HEIGHT: usize = 20;

/// Полная ширина дисплея (с учётом границ).
///
/// Формула: (SHAPE_WIDTH * GRID_WIDTH) + 2 (границы)
/// = (2 * 10) + 2 = 22 символа
pub const DISP_WIDTH: u16 = (SHAPE_WIDTH * GRID_WIDTH) as u16 + 2;

/// Полная высота дисплея (с учётом границ и отступов).
///
/// Формула: GRID_HEIGHT + 5 (заголовки и границы)
/// = 20 + 5 = 25 строк
pub const DISP_HEIGHT: u16 = GRID_HEIGHT as u16 + 5;

/// Канвас для отрисовки в терминале.
///
/// Обёртка над RawTerminal для удобной отрисовки текста и графики.
/// Автоматически скрывает курсор при создании.
///
/// ## Пример использования
/// ```no_run
/// use tetris_cli::io::Canvas;
/// use termion::color::{White, Reset};
///
/// let mut canvas = Canvas::new();
/// canvas.draw_string("Текст", (1, 1), &White, &Reset);
/// canvas.flush();
///
/// // После завершения игры
/// canvas.reset();
/// ```
pub struct Canvas {
    out: RawTerminal<Stdout>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    /// Создать новый канвас и подготовить терминал.
    ///
    /// # Возвращает
    /// Новый экземпляр Canvas с инициализированным терминалом
    ///
    /// # Паника
    /// Паникует, если:
    /// - Не удалось перейти в raw-режим
    /// - Не удалось очистить экран
    /// - Не удалось скрыть курсор
    ///
    /// # Примечания
    /// Метод автоматически:
    /// 1. Переводит терминал в raw-режим
    /// 2. Очищает экран
    /// 3. Перемещает курсор в (1, 1)
    /// 4. Скрывает курсор
    pub fn new() -> Self {
        let mut out = stdout()
            .into_raw_mode()
            .expect("Не удалось перейти в raw-режим терминала");
        write!(out, "{}{}", All, Goto(1, 1)).expect("Не удалось очистить экран");
        out.flush().expect("Не удалось выполнить flush буфера");

        write!(out, "{}", Hide).expect("Не удалось скрыть курсор");

        Self { out }
    }

    /// Сбросить терминал в исходное состояние.
    ///
    /// # Примечания
    /// Метод автоматически:
    /// 1. Показывает курсор
    /// 2. Выполняет flush буфера
    ///
    /// # Важность
    /// Обязательно вызывайте этот метод перед завершением программы,
    /// чтобы вернуть терминал в нормальное состояние.
    pub fn reset(&mut self) {
        write!(self.out, "{}\r\n", Show).expect("Не удалось показать курсор");
        self.out.flush().expect("Не удалось выполнить flush буфера");
    }

    /// Отрисовать строки (статические).
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `pos` - позиция верхней левой строки (x, y)
    /// * `fg` - цвет переднего плана
    /// * `bg` - цвет фона
    ///
    /// # Примечания
    /// Каждая строка рисуется на новой позиции Y (автоматический перенос)
    pub fn draw_strs(&mut self, lines: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        let (x, mut y) = pos;
        for line in lines {
            write!(
                self.out,
                "{}{}{}{}{}{}",
                Goto(x, y),
                Fg(fg),
                Bg(bg),
                line,
                Fg(Reset),
                Bg(Reset)
            )
            .expect("Не удалось отрисовать строку");
            y += 1;
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
    /// # Пример
    /// ```no_run
    /// use tetris_cli::io::Canvas;
    /// use termion::color::{White, Reset};
    ///
    /// let mut canvas = Canvas::new();
    /// canvas.draw_string("Счёт: 100", (5, 2), &White, &Reset);
    /// canvas.flush();
    /// ```
    pub fn draw_string(&mut self, text: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        let (x, y) = pos;
        write!(
            self.out,
            "{}{}{}{}{}{}",
            Goto(x, y),
            Fg(fg),
            Bg(bg),
            text,
            Fg(Reset),
            Bg(Reset)
        )
        .expect("Не удалось отрисовать строку");
    }

    /// Обновить вывод (flush).
    ///
    /// # Важность
    /// Вызывайте этот метод после всех операций отрисовки,
    /// чтобы изменения появились на экране.
    ///
    /// # Паника
    /// Паникует, если не удалось выполнить flush буфера
    pub fn flush(&mut self) {
        self.out.flush().expect("Не удалось выполнить flush буфера");
    }
}

/// Читатель нажатий клавиш в асинхронном режиме.
///
/// Использует async_stdin для неблокирующего чтения клавиатуры.
///
/// ## Пример использования
/// ```
/// use tetris_cli::io::KeyReader;
///
/// let mut reader = KeyReader::new();
/// let key = reader.get_key();
///
/// if key == b'q' {
///     println!("Нажата клавиша Q");
/// }
/// ```
pub struct KeyReader {
    inp: AsyncReader,
}

impl Default for KeyReader {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyReader {
    /// Создать новый читатель клавиш.
    ///
    /// # Возвращает
    /// Новый экземпляр KeyReader с инициализированным async_stdin
    pub fn new() -> Self {
        let inp = async_stdin();
        Self { inp }
    }

    /// Получить код нажатой клавиши.
    ///
    /// # Возвращает
    /// Код нажатой клавиши (u8) или 0 при ошибке чтения
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// let key = reader.get_key();
    ///
    /// match key {
    ///     b'q' => println!("Выход"),
    ///     b'p' => println!("Пауза"),
    ///     _ => {}
    /// }
    /// ```
    ///
    /// # Примечания
    /// - Возвращает 0, если клавиша не была нажата
    /// - Для специальных клавиш (стрелки, Backspace) могут возвращаться
    ///   последовательности байтов
    pub fn get_key(&mut self) -> u8 {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(_) => key_bytes[0],
            Err(_) => 0,
        }
    }
}
