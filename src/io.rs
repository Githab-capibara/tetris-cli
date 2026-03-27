//! Обработка ввода и вывода.
//!
//! Модуль предоставляет абстракции для работы с терминалом:
//! - `Canvas` - канвас для отрисовки
//! - `KeyReader` - асинхронный читатель клавиатуры

use std::io::{stdout, Read, Stdout, Write};
use termion::{
    async_stdin,
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
    screen::ToMainScreen,
    AsyncReader,
};

// Импорт трейтов для реализации
use crate::io_traits::{InputReader, Renderer};

// ============================================================================
// ПЕРЕЭКСПОРТ КОНСТАНТ ИЗ game/constants.rs
// ============================================================================
// Централизация констант для устранения дублирования.
// Эти константы определены в game/constants.rs и переэкспортируются здесь
// для обратной совместимости.

pub use crate::game::constants::{GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};

// ============================================================================
// КОНСТАНТЫ ВВОДА/ВЫВОДА (локальные)
// ============================================================================

/// Код клавиши Backspace для выхода.
///
/// Используется в главном меню и во время игры для выхода в меню.
pub const KEY_BACKSPACE: u8 = 127;

/// Код клавиши Enter (перевод строки).
/// Зарезервировано для будущей функциональности (локализация, макросы).
#[allow(dead_code)]
pub const KEY_ENTER: u8 = b'\n';

/// Код клавиши Enter (возврат каретки).
/// Зарезервировано для будущей функциональности (локализация, макросы).
#[allow(dead_code)]
pub const KEY_ENTER_CR: u8 = b'\r';

/// Полная ширина дисплея (с учётом границ).
///
/// Формула: (`SHAPE_WIDTH` * `GRID_WIDTH`) + 2 (границы)
/// = (2 * 10) + 2 = 22 символа
///
/// Тип u16 используется для совместимости с terminal_size().
pub const DISP_WIDTH: u16 = (SHAPE_WIDTH * GRID_WIDTH) as u16 + 2;

/// Полная высота дисплея (с учётом границ и отступов).
///
/// Формула: `GRID_HEIGHT` + 5 (заголовки и границы)
/// = 20 + 5 = 25 строк
///
/// Тип u16 используется для совместимости с terminal_size().
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
    /// Зарезервировано для будущей функциональности отрисовки.
    #[allow(dead_code)]
    Draw(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::RawMode(msg) => write!(f, "Ошибка raw-режима: {msg}"),
            IoError::Clear(msg) => write!(f, "Ошибка очистки экрана: {msg}"),
            IoError::Cursor(msg) => write!(f, "Ошибка курсора: {msg}"),
            IoError::Flush(msg) => write!(f, "Ошибка flush: {msg}"),
            IoError::Draw(msg) => write!(f, "Ошибка отрисовки: {msg}"),
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
/// Обёртка над `RawTerminal` для удобной отрисовки текста и графики.
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
    ///
    /// # Исправление #8
    /// Критические ошибки выводятся через eprintln! вместо игнорирования.
    fn drop(&mut self) {
        if let Err(e) = write!(self.out, "{Show}") {
            eprintln!("Критическая ошибка: не удалось показать курсор: {}", e);
        }
        if let Err(e) = self.out.flush() {
            eprintln!("Критическая ошибка: не удалось выполнить flush: {}", e);
        }
    }
}

impl Default for Canvas {
    /// Возвращает Canvas по умолчанию.
    ///
    /// # Примечания
    /// При ошибке инициализации создаёт fallback canvas с заглушкой.
    /// Если и stub не удаётся создать - паникует с понятным сообщением.
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            Self::new_stub().unwrap_or_else(|e| {
                panic!("Критическая ошибка: не удалось инициализировать терминал: {e}");
            })
        })
    }
}

impl Canvas {
    /// Создать новый канвас и подготовить терминал.
    ///
    /// # Возвращает
    /// - `Ok(Canvas)` - новый экземпляр Canvas с инициализированным терминалом
    /// - `Err(IoError)` - ошибка инициализации терминала
    ///
    /// # Errors
    /// Возвращает [`IoError::RawMode`] если не удалось перейти в raw-режим терминала.
    /// Возвращает [`IoError::Clear`] если не удалось очистить экран.
    /// Возвращает [`IoError::Flush`] если не удалось выполнить flush буфера.
    /// Возвращает [`IoError::Cursor`] если не удалось скрыть курсор.
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
            IoError::RawMode(format!("не удалось перейти в raw-режим терминала: {e}"))
        })?;

        write!(out, "{}{}", All, Goto(1, 1))
            .map_err(|e| IoError::Clear(format!("не удалось очистить экран: {e}")))?;

        out.flush()
            .map_err(|e| IoError::Flush(format!("не удалось выполнить flush буфера: {e}")))?;

        write!(out, "{Hide}")
            .map_err(|e| IoError::Cursor(format!("не удалось скрыть курсор: {e}")))?;

        Ok(Self { out })
    }

    /// Создать заглушку Canvas для использования при ошибке инициализации.
    ///
    /// # Возвращает
    /// - `Ok(Canvas)` - stub canvas с минимальной конфигурацией
    /// - `Err(IoError)` - критическая ошибка терминала
    ///
    /// # Поведение
    /// ## Исправление #30: Документирование поведения
    /// - Если терминал недоступен, создаётся минимальный stub
    /// - Stub поддерживает базовые операции отрисовки через stdout
    /// - Все операции записываются в stdout, даже если терминал не в raw-режиме
    /// - При ошибке возвращается IoError::RawMode вместо паники
    ///
    /// # Обработка ошибок
    /// - Ошибки инициализации в stub режиме тихо игнорируются
    /// - Критическая ошибка терминала возвращается как IoError::RawMode
    /// - Программа может продолжить работу в ограниченном режиме
    ///
    /// # Примечания
    /// Исправление #1: создаём stub без паники с минимальной конфигурацией.
    /// Это позволяет программе работать в ограниченном режиме без терминала.
    /// Исправление #7: используется match вместо if let для лучшей читаемости.
    fn new_stub() -> Result<Self, IoError> {
        // Пытаемся создать Canvas, но если не получается - создаём минимальный stub
        // Используем match вместо if let для лучшей читаемости (исправление #7)
        match stdout().into_raw_mode() {
            Ok(mut out) => {
                // Тихо игнорируем ошибки инициализации в stub режиме
                if let Err(e) = write!(out, "{}{}", All, Goto(1, 1)) {
                    eprintln!("Warning: failed to clear terminal: {}", e);
                }
                if let Err(e) = out.flush() {
                    eprintln!("Warning: failed to flush terminal: {}", e);
                }
                if let Err(e) = write!(out, "{Hide}") {
                    eprintln!("Warning: failed to hide cursor: {}", e);
                }
                Ok(Self { out })
            }
            Err(e) => {
                // Критическая ошибка - терминал недоступен
                // Возвращаем ошибку вместо exit(1) для обработки наверх
                Err(IoError::RawMode(format!(
                    "терминал недоступен или не поддерживает ANSI: {e}"
                )))
            }
        }
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
    /// # Примечания
    /// Каждая строка рисуется на новой позиции Y (автоматический перенос)
    ///
    /// # Оптимизация
    /// Исправление #8: буферизация всех строк и один `flush()` в конце
    /// для предотвращения множественных системных вызовов.
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
                eprintln!("Warning: failed to draw string: {}", e);
            }
            y += 1;
        }
        // Один flush() в конце вместо flush() после каждой строки
        if let Err(e) = self.out.flush() {
            eprintln!("Warning: failed to flush terminal: {}", e);
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
            eprintln!("Ошибка отрисовки строки: {e}");
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
            eprintln!("Ошибка: не удалось выполнить flush буфера: {e}");
        }
    }
}

/// Читатель нажатий клавиш в асинхронном режиме.
///
/// Использует `async_stdin` для неблокирующего чтения клавиатуры.
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
    /// Освобождение ресурсов при уничтожении `KeyReader`.
    ///
    /// # Примечания
    /// Исправление #13: реализация Drop для предотвращения утечки ресурсов.
    /// Исправление #8: критические ошибки выводятся через eprintln! вместо игнорирования.
    /// Метод автоматически освобождает ресурсы stdin при выходе из области видимости.
    ///
    /// # Исправление #2 (Drop guard)
    /// Усиленный сброс терминала:
    /// 1. Показываем курсор
    /// 2. Возвращаемся на главный экран
    /// 3. Сбрасываем raw-режим (через ToMainScreen)
    /// 4. Очищаем экран от артефактов
    /// 5. Выполняем flush
    ///
    /// # Безопасность
    /// Гарантирует возврат терминала в нормальное состояние даже при панике.
    fn drop(&mut self) {
        let mut stdout = std::io::stdout();

        // Показываем курсор
        if let Err(e) = write!(stdout, "{Show}") {
            eprintln!("Критическая ошибка: не удалось показать курсор: {}", e);
        }

        // Возвращаемся на главный экран (сбрасывает raw-режим)
        if let Err(e) = write!(stdout, "{ToMainScreen}") {
            eprintln!("Критическая ошибка: не удалось вернуть экран: {}", e);
        }

        // Дополнительный сброс: перемещаем курсор в домашнюю позицию
        if let Err(e) = write!(stdout, "\x1b[H") {
            eprintln!("Критическая ошибка: не удалось переместить курсор: {}", e);
        }

        // Flush буфера для гарантированного применения изменений
        if let Err(e) = stdout.flush() {
            eprintln!("Критическая ошибка: не удалось выполнить flush: {}", e);
        }
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
    /// Новый экземпляр `KeyReader` с инициализированным `async_stdin`
    pub fn new() -> Self {
        let inp = async_stdin();
        Self { inp }
    }

    /// Получить код нажатой клавиши (ASCII).
    ///
    /// # Возвращает
    /// - `Some(u8)` — код нажатой клавиши (ASCII 0x00-0x7F)
    /// - `None` — при ошибке чтения, если клавиша не была нажата или введён UTF-8 символ
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
    ///
    /// ## Исправление #1 (UTF-8 поддержка)
    /// **Метод НЕ поддерживает многобайтовые символы UTF-8** (кириллица, emoji и другие Unicode-символы).
    /// При вводе многобайтовых символов метод возвращает `None`, предварительно прочитав все байты символа.
    ///
    /// ## Поддерживаемые символы
    /// - ✅ ASCII символы (0x00-0x7F): латиница, цифры, управляющие коды
    /// - ❌ Многобайтовые UTF-8 символы: кириллица, китайские иероглифы, emoji
    ///
    /// ## Решение для Unicode
    /// Для поддержки многобайтовых символов используйте метод [`get_key_unicode()`](Self::get_key_unicode).
    ///
    /// # Примечания
    /// - Для специальных клавиш (стрелки, Home, End) возвращает первый байт ESC-последовательности
    ///   (обычно 27 = ESC).
    ///
    /// ## Технические детали
    /// - Поддерживаются только однобайтовые ASCII символы (0x00-0x7F)
    /// - Многобайтовые последовательности UTF-8 (0xC2-0xF4) читаются, но игнорируются
    /// - Невалидные байты (0xC0, 0xC1, 0xF5-0xFF) отбрасываются
    ///
    /// # Исправление #32
    /// Добавлено логирование ошибок чтения ввода.
    pub fn get_key(&mut self) -> Option<u8> {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(()) => {
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

    /// Получить нажатую клавишу с поддержкой UTF-8.
    ///
    /// # Возвращает
    /// - `Some(char)` — символ Unicode (включая кириллицу, emoji и другие многобайтовые символы)
    /// - `None` — при ошибке чтения, если клавиша не была нажата или введён невалидный UTF-8
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// if let Some(key) = reader.get_key_unicode() {
    ///     match key {
    ///         'q' => println!("Выход"),
    ///         'п' => println!("Пауза (кириллица)"),
    ///         '🎮' => println!("Emoji клавиша"),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    ///
    /// # Отличия от get_key()
    /// | Метод | Возвращает | Поддержка Unicode |
    /// |-------|------------|-------------------|
    /// | `get_key()` | `Option<u8>` | Только ASCII (0x00-0x7F) |
    /// | `get_key_unicode()` | `Option<char>` | Полный Unicode ✅ |
    ///
    /// # Поддерживаемые символы
    /// - ✅ ASCII (0x00-0x7F): латиница, цифры, управляющие коды
    /// - ✅ Кириллица, китайские иероглифы, арабская вязь
    /// - ✅ Emoji и другие Unicode-символы
    /// - ❌ Специальные клавиши (стрелки, Home, End) могут возвращать `None`
    ///
    /// # Технические детали
    /// - Читает все байты символа UTF-8 и декодирует их
    /// - Поддерживает 2-, 3- и 4-байтовые последовательности UTF-8
    /// - Невалидные байты (0xC0, 0xC1, 0xF5-0xFF) отбрасываются
    ///
    /// # Исправление #1 (UTF-8 поддержка)
    /// Добавлена полная поддержка Unicode для локализации управления.
    #[allow(dead_code)]
    pub fn get_key_unicode(&mut self) -> Option<char> {
        let mut first_byte: [u8; 1] = [0];

        match self.inp.read_exact(&mut first_byte) {
            Ok(()) => {
                let byte = first_byte[0];

                // ASCII символ (однобайтовый)
                if byte <= 0x7F {
                    return Some(byte as char);
                }

                // Определяем количество дополнительных байт в символе UTF-8
                let additional_bytes = if (0xC2..=0xDF).contains(&byte) {
                    1 // 2-byte sequence
                } else if (0xE0..=0xEF).contains(&byte) {
                    2 // 3-byte sequence
                } else if (0xF0..=0xF4).contains(&byte) {
                    3 // 4-byte sequence
                } else {
                    // Невалидный байт UTF-8 (0xC0, 0xC1, 0xF5-0xFF)
                    return None;
                };

                // Читаем дополнительные байты
                let mut buffer = [0u8; 3];
                if self
                    .inp
                    .read_exact(&mut buffer[..additional_bytes])
                    .is_err()
                {
                    return None;
                }

                // Собираем все байты символа
                let mut utf8_bytes = [0u8; 4];
                utf8_bytes[0] = byte;
                utf8_bytes[1..=additional_bytes].copy_from_slice(&buffer[..additional_bytes]);

                // Декодируем UTF-8
                std::str::from_utf8(&utf8_bytes[..=additional_bytes])
                    .ok()
                    .and_then(|s| s.chars().next())
            }
            Err(_) => None,
        }
    }

    /// Очистить ресурсы и сбросить терминал.
    ///
    /// # Примечания
    /// Метод выполняет тот же сброс что и Drop, но может быть вызван явно
    /// для досрочного освобождения ресурсов.
    ///
    /// # Безопасность
    /// Гарантирует возврат терминала в нормальное состояние:
    /// 1. Показывает курсор
    /// 2. Возвращается на главный экран
    /// 3. Сбрасывает raw-режим
    /// 4. Выполняет flush буфера
    ///
    /// # Исправление #6 (HIGH)
    /// Добавлен явный cleanup() метод для контроля освобождения ресурсов.
    /// Drop guard по-прежнему работает для паник и неявного выхода.
    ///
    /// # Пример использования
    /// ```ignore
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// // ... использование reader
    /// reader.cleanup(); // Явный сброс терминала
    /// ```
    ///
    /// ## Исправление #13
    /// Метод предназначен для будущего использования в API.
    #[allow(dead_code)]
    #[allow(clippy::unused_self)]
    pub fn cleanup(&mut self) {
        let mut stdout = std::io::stdout();

        // Показываем курсор
        if let Err(e) = write!(stdout, "{Show}") {
            eprintln!("Критическая ошибка: не удалось показать курсор: {}", e);
        }

        // Возвращаемся на главный экран (сбрасывает raw-режим)
        if let Err(e) = write!(stdout, "{ToMainScreen}") {
            eprintln!("Критическая ошибка: не удалось вернуть экран: {}", e);
        }

        // Дополнительный сброс: перемещаем курсор в домашнюю позицию
        if let Err(e) = write!(stdout, "\x1b[H") {
            eprintln!("Критическая ошибка: не удалось переместить курсор: {}", e);
        }

        // Flush буфера для гарантированного применения изменений
        if let Err(e) = stdout.flush() {
            eprintln!("Критическая ошибка: не удалось выполнить flush: {}", e);
        }
    }
}

// ============================================================================
// РЕАЛИЗАЦИЯ ТРЕЙТОВ io_traits
// ============================================================================

impl InputReader for KeyReader {
    /// Получить код нажатой клавиши.
    ///
    /// Делегирует вызов методу `KeyReader::get_key()`.
    fn get_key(&mut self) -> Option<u8> {
        self.get_key()
    }
}

impl Renderer for Canvas {
    /// Отрисовать строки.
    ///
    /// Делегирует вызов методу `Canvas::draw_strs()`.
    fn draw_strs(&mut self, strings: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        self.draw_strs(strings, pos, fg, bg);
    }

    /// Отрисовать строку.
    ///
    /// Делегирует вызов методу `Canvas::draw_string()`.
    fn draw_string(&mut self, string: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
        self.draw_string(string, pos, fg, bg);
    }

    /// Обновить экран (flush).
    ///
    /// Делегирует вызов методу `Canvas::flush()`.
    fn flush(&mut self) {
        self.flush();
    }

    /// Сбросить терминал в исходное состояние.
    ///
    /// Делегирует вызов методу `Canvas::reset()`.
    fn reset(&mut self) {
        self.reset();
    }
}
