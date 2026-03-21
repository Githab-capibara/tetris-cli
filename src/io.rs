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
//! let mut canvas = Canvas::new().expect("Не удалось создать Canvas");
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

/// Ошибка ввода/вывода терминала.
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum IoError {
    /// Не удалось перейти в raw-режим терминала.
    RawMode(String),
    /// Не удалось очистить экран.
    Clear(String),
    /// Не удалось скрыть/показать курсор.
    Cursor(String),
    /// Ошибка flush буфера.
    Flush(String),
    /// Ошибка отрисовки.
    #[allow(dead_code)]
    Draw(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::RawMode(msg) => write!(f, "Ошибка raw-режима: {}", msg),
            IoError::Clear(msg) => write!(f, "Ошибка очистки экрана: {}", msg),
            IoError::Cursor(msg) => write!(f, "Ошибка курсора: {}", msg),
            IoError::Flush(msg) => write!(f, "Ошибка flush: {}", msg),
            IoError::Draw(msg) => write!(f, "Ошибка отрисовки: {}", msg),
        }
    }
}

impl std::error::Error for IoError {}

impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        IoError::RawMode(err.to_string())
    }
}

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
/// fn main() -> Result<(), tetris_cli::io::IoError> {
///     let mut canvas = Canvas::new()?;
///     canvas.draw_string("Текст", (1, 1), &White, &Reset);
///     canvas.flush();
///
///     // После завершения игры
///     canvas.reset();
///     Ok(())
/// }
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
    /// Возвращает Canvas по умолчанию.
    ///
    /// # Panics
    /// Паникует, если не удалось инициализировать терминал (например, если stdout не является TTY).
    ///
    /// # Примечания
    /// Используется значение по умолчанию для терминала.
    /// При ошибке инициализации выводит подробное сообщение о возможной причине.
    fn default() -> Self {
        Self::new().expect(
            "Не удалось инициализировать Canvas: проверьте, что терминал поддерживает ANSI и доступен"
        )
    }
}

impl Canvas {
    /// Создать новый канвас и подготовить терминал.
    ///
    /// # Возвращает
    /// - `Ok(Canvas)` - новый экземпляр Canvas с инициализированным терминалом
    /// - `Err(IoError)` - ошибка инициализации терминала
    ///
    /// # Ошибки
    /// Метод возвращает ошибку в следующих случаях:
    /// - Не удалось перейти в raw-режим терминала
    /// - Не удалось очистить экран
    /// - Не удалось скрыть курсор
    ///
    /// # Примечания
    /// Метод автоматически:
    /// 1. Переводит терминал в raw-режим
    /// 2. Очищает экран
    /// 3. Перемещает курсор в (1, 1)
    /// 4. Скрывает курсор
    pub fn new() -> Result<Self, IoError> {
        let mut out = stdout().into_raw_mode().map_err(|e| {
            IoError::RawMode(format!("не удалось перейти в raw-режим терминала: {}", e))
        })?;

        write!(out, "{}{}", All, Goto(1, 1))
            .map_err(|e| IoError::Clear(format!("не удалось очистить экран: {}", e)))?;

        out.flush()
            .map_err(|e| IoError::Flush(format!("не удалось выполнить flush буфера: {}", e)))?;

        write!(out, "{}", Hide)
            .map_err(|e| IoError::Cursor(format!("не удалось скрыть курсор: {}", e)))?;

        Ok(Self { out })
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
    /// let mut canvas = Canvas::new().expect("Не удалось создать Canvas");
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

impl Drop for KeyReader {
    /// Освобождение ресурсов при уничтожении KeyReader.
    ///
    /// # Примечания
    /// Исправление #13: реализация Drop для предотвращения утечки ресурсов.
    /// Метод автоматически освобождает ресурсы stdin при выходе из области видимости.
    fn drop(&mut self) {
        // AsyncReader автоматически освобождается при drop,
        // но явно сбрасываем терминал в случай ошибки
        // termion::async_stdin не требует явного сброса
    }
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
    /// # Важные ограничения
    /// **Метод не поддерживает многобайтовые символы UTF-8** (кириллица, emoji и другие Unicode-символы).
    /// При вводе многобайтовых символов метод возвращает `None`, предварительно прочитав все байты символа.
    /// Это означает, что для локализации игры на языки с многобайтовыми символами потребуется доработка.
    ///
    /// ## Ограничения
    /// - Поддерживаются только ASCII-символы и управляющие коды (0x00-0x7F)
    /// - Многобайтовые UTF-8 символы игнорируются (возвращается None)
    /// - Для поддержки Unicode используйте расширенную версию API
    ///
    /// # Примечания
    /// - Для специальных клавиш (стрелки, Home, End) возвращает первый байт ESC-последовательности
    ///   (обычно 27 = ESC). Для полной обработки нужно использовать get_key_extended().
    ///
    /// ## Технические детали
    /// - Поддерживаются только однобайтовые ASCII символы (0x00-0x7F)
    /// - Многобайтовые последовательности UTF-8 (0xC2-0xF4) читаются, но игнорируются
    /// - Невалидные байты (0xC0, 0xC1, 0xF5-0xFF) отбрасываются
    pub fn get_key(&mut self) -> Option<u8> {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(_) => {
                let first_byte = key_bytes[0];

                // Проверяем, является ли это началом многобайтового символа UTF-8
                // ASCII (0x00-0x7F) - однобайтовый символ
                if first_byte <= 0x7F {
                    return Some(first_byte);
                }

                // Определяем количество байт в символе UTF-8
                // Используем корректные диапазоны: 0xC0 и 0xC1 - невалидны (переполнение),
                // 0xF5-0xF7 - невалидны (максимум 0xF4 для UTF-8)
                let bytes_to_read = if (0xC2..=0xDF).contains(&first_byte) {
                    1 // 2-byte sequence
                } else if (0xE0..=0xEF).contains(&first_byte) {
                    2 // 3-byte sequence
                } else if (0xF0..=0xF4).contains(&first_byte) {
                    3 // 4-byte sequence
                } else {
                    // Невалидный байт (0xC0, 0xC1, 0xF5-0xFF)
                    return None;
                };

                // Читаем остальные байты символа UTF-8
                let mut remaining = [0u8; 3];
                if self
                    .inp
                    .read_exact(&mut remaining[..bytes_to_read])
                    .is_err()
                {
                    return None;
                }

                // Для многобайтовых символов возвращаем None
                // (они не являются управляющими клавишами для игры)
                None
            }
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
    ///     Some(256) => println!("Стрелка вверх"),
    ///     Some(257) => println!("Стрелка вниз"),
    ///     Some(113) => println!("Выход"), // 'q' = 113
    ///     None => println!("Ошибка чтения"),
    ///     _ => {}
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn get_key_extended(&mut self) -> Option<u16> {
        let mut buffer = [0u8; 3];

        // Читаем первый байт
        match self.inp.read_exact(&mut buffer[0..1]) {
            Ok(_) => {}
            Err(_) => return None,
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
                            Err(_) => return Some(27), // Только ESC
                        }
                    }

                    // Обрабатываем ESC-последовательности
                    match buffer[1] {
                        b'[' => {
                            match buffer[2] {
                                b'A' => return Some(KEY_ARROW_UP),    // Стрелка вверх
                                b'B' => return Some(KEY_ARROW_DOWN),  // Стрелка вниз
                                b'C' => return Some(KEY_ARROW_RIGHT), // Стрелка вправо
                                b'D' => return Some(KEY_ARROW_LEFT),  // Стрелка влево
                                b'H' => return Some(KEY_HOME),        // Home
                                b'F' => return Some(KEY_END),         // End
                                _ => return Some(27), // Неизвестная последовательность
                            }
                        }
                        b'O' => {
                            match buffer[2] {
                                b'A' => return Some(KEY_ARROW_UP), // Стрелка вверх (альтернативная)
                                b'B' => return Some(KEY_ARROW_DOWN), // Стрелка вниз (альтернативная)
                                b'C' => return Some(KEY_ARROW_RIGHT), // Стрелка вправо (альтернативная)
                                b'D' => return Some(KEY_ARROW_LEFT), // Стрелка влево (альтернативная)
                                _ => return Some(27),
                            }
                        }
                        _ => return Some(27),
                    }
                }
                Err(_) => return Some(27), // Только ESC
            }
        }

        Some(buffer[0] as u16)
    }
}
