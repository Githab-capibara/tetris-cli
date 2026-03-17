//! Обработка ввода и вывода.
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

/// Код клавиши Backspace для выхода.
///
/// Используется в главном меню и во время игры для выхода в меню.
pub const KEY_BACKSPACE: u8 = 127;

/// Код клавиши Enter (перевод строки).
#[allow(dead_code)]
pub const KEY_ENTER: u8 = b'\n';

/// Код клавиши Enter (возврат каретки).
#[allow(dead_code)]
pub const KEY_ENTER_CR: u8 = b'\r';

/// Специальные коды клавиш для стрелок (возвращаются get_key_extended()).
///
/// Эти коды используются при обработке ESC-последовательностей:
/// - Стрелка вверх: 256
/// - Стрелка вниз: 257
/// - Стрелка вправо: 258
/// - Стрелка влево: 259
/// - Home: 260
/// - End: 261
pub const KEY_ARROW_UP: u16 = 256;
pub const KEY_ARROW_DOWN: u16 = 257;
pub const KEY_ARROW_RIGHT: u16 = 258;
pub const KEY_ARROW_LEFT: u16 = 259;
pub const KEY_HOME: u16 = 260;
pub const KEY_END: u16 = 261;

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
/// Реализует Drop для автоматического сброса терминала при выходе или панике.
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

impl Drop for Canvas {
    /// Автоматический сброс терминала при выходе из области видимости или панике.
    ///
    /// # Примечания
    /// Метод автоматически:
    /// 1. Показывает курсор
    /// 2. Выполняет flush буфера
    ///
    /// # Важность
    /// Гарантирует, что терминал вернётся в нормальное состояние даже при панике.
    fn drop(&mut self) {
        let _ = write!(self.out, "{}", Show);
        let _ = self.out.flush();
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    /// Вспомогательная функция для сброса терминала при ошибке и выхода.
    ///
    /// # Аргументы
    /// * `message` - сообщение об ошибке
    fn exit_with_terminal_reset(message: &str) -> ! {
        eprintln!("{}", message);
        let mut stdout = stdout();
        let _ = write!(stdout, "{}", Show);
        let _ = stdout.flush();
        std::process::exit(1);
    }

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
        let mut out = match stdout().into_raw_mode() {
            Ok(term) => term,
            Err(e) => {
                Self::exit_with_terminal_reset(&format!(
                    "Ошибка: не удалось перейти в raw-режим терминала: {}",
                    e
                ));
            }
        };

        if let Err(e) = write!(out, "{}{}", All, Goto(1, 1)) {
            Self::exit_with_terminal_reset(&format!(
                "Ошибка: не удалось очистить экран: {}",
                e
            ));
        }

        if let Err(e) = out.flush() {
            Self::exit_with_terminal_reset(&format!(
                "Ошибка: не удалось выполнить flush буфера: {}",
                e
            ));
        }

        if let Err(e) = write!(out, "{}", Hide) {
            Self::exit_with_terminal_reset(&format!(
                "Ошибка: не удалось скрыть курсор: {}",
                e
            ));
        }

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
        if let Err(e) = write!(self.out, "{}\r\n", Show) {
            eprintln!("Ошибка: не удалось показать курсор: {}", e);
            return;
        }
        if let Err(e) = self.out.flush() {
            eprintln!("Ошибка: не удалось выполнить flush буфера: {}", e);
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
    /// # Примечания
    /// Каждая строка рисуется на новой позиции Y (автоматический перенос)
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
                eprintln!("Ошибка отрисовки строки: {}", e);
                return;
            }
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
            eprintln!("Ошибка отрисовки строки: {}", e);
        }
    }

    /// Обновить вывод (flush).
    ///
    /// # Важность
    /// Вызывайте этот метод после всех операций отрисовки,
    /// чтобы изменения появились на экране.
    ///
    /// # Примечания
    /// При ошибке выводит сообщение в stderr
    pub fn flush(&mut self) {
        if let Err(e) = self.out.flush() {
            eprintln!("Ошибка: не удалось выполнить flush буфера: {}", e);
        }
    }
}

/// Читатель нажатий клавиш в асинхронном режиме.
///
/// Использует async_stdin для неблокирующего чтения клавиатуры.
/// Поддерживает обработку ESC-последовательностей для специальных клавиш.
///
/// ## Пример использования
/// ```
/// use tetris_cli::io::KeyReader;
///
/// let mut reader = KeyReader::new();
/// if let Some(key) = reader.get_key() {
///     if key == b'q' {
///         println!("Нажата клавиша Q");
///     }
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
    /// - `Some(u8)` — код нажатой клавиши
    /// - `None` — при ошибке чтения или если клавиша не была нажата
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// if let Some(key) = reader.get_key() {
    ///     match key {
    ///         b'q' => println!("Выход"),
    ///         b'p' => println!("Пауза"),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    ///
    /// # Примечания
    /// - Для специальных клавиш (стрелки, Home, End) возвращает первый байт ESC-последовательности
    ///   (обычно 27 = ESC). Для полной обработки нужно использовать get_key_extended().
    pub fn get_key(&mut self) -> Option<u8> {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(_) => Some(key_bytes[0]),
            Err(_) => None,
        }
    }

    /// Получить код нажатой клавиши с обработкой ESC-последовательностей.
    ///
    /// # Возвращает
    /// - ASCII код клавиши (a-z, 0-9, и т.д.)
    /// - Специальные коды:
    ///   - 27 (ESC) - клавиша Escape
    ///   - 256-259 - стрелки (Up, Down, Left, Right)
    ///   - 260 - Home
    ///   - 261 - End
    ///   - 0 - ошибка или нет нажатий
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// let key = reader.get_key_extended();
    ///
    /// match key {
    ///     256 => println!("Стрелка вверх"),
    ///     257 => println!("Стрелка вниз"),
    ///     113 => println!("Выход"), // 'q' = 113
    ///     _ => {}
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn get_key_extended(&mut self) -> u16 {
        let mut buffer = [0u8; 3];

        // Читаем первый байт
        match self.inp.read_exact(&mut buffer[0..1]) {
            Ok(_) => {}
            Err(_) => return 0,
        }

        // Если это ESC, читаем последовательность
        if buffer[0] == 27 {
            // Пытаемся прочитать второй байт (неблокирующе)
            let mut second_byte = [0u8; 1];
            match self.inp.read_exact(&mut second_byte) {
                Ok(_) => {
                    buffer[1] = second_byte[0];

                    // Если второй байт '[' или 'O', читаем третий
                    if buffer[1] == b'[' || buffer[1] == b'O' {
                        let mut third_byte = [0u8; 1];
                        match self.inp.read_exact(&mut third_byte) {
                            Ok(_) => buffer[2] = third_byte[0],
                            Err(_) => return 27, // Только ESC
                        }
                    }

                    // Обрабатываем ESC-последовательности
                    match buffer[1] {
                        b'[' => {
                            match buffer[2] {
                                b'A' => return KEY_ARROW_UP,    // Стрелка вверх
                                b'B' => return KEY_ARROW_DOWN,  // Стрелка вниз
                                b'C' => return KEY_ARROW_RIGHT, // Стрелка вправо
                                b'D' => return KEY_ARROW_LEFT,  // Стрелка влево
                                b'H' => return KEY_HOME,        // Home
                                b'F' => return KEY_END,         // End
                                _ => return 27,                 // Неизвестная последовательность
                            }
                        }
                        b'O' => {
                            match buffer[2] {
                                b'A' => return KEY_ARROW_UP,    // Стрелка вверх (альтернативная)
                                b'B' => return KEY_ARROW_DOWN,  // Стрелка вниз (альтернативная)
                                b'C' => return KEY_ARROW_RIGHT, // Стрелка вправо (альтернативная)
                                b'D' => return KEY_ARROW_LEFT,  // Стрелка влево (альтернативная)
                                _ => return 27,
                            }
                        }
                        _ => return 27,
                    }
                }
                Err(_) => return 27, // Только ESC
            }
        }

        buffer[0] as u16
    }
}
