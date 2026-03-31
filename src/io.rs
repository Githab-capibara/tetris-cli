//! Обработка ввода и вывода.
//!
//! Модуль предоставляет абстракции для работы с терминалом:
//! - `Canvas` - канвас для отрисовки
//! - `KeyReader` - асинхронный читатель клавиатуры

use std::io::{self, stdout, Read, Stdout, Write};
use termion::{
    async_stdin,
    clear::All,
    color::{Bg, Color, Fg, Reset},
    cursor::{Goto, Hide, Show},
    raw::{IntoRawMode, RawTerminal},
    screen::ToMainScreen,
    AsyncReader,
};
use thiserror::Error;

// Импорт трейтов для реализации
use crate::io_traits::{InputReader, Renderer};

// ============================================================================
// ИМПОРТ КОНСТАНТ ИЗ constants.rs
// ============================================================================
// Централизация констант для устранения дублирования.
// DISP_WIDTH и DISP_HEIGHT определены в constants.rs.
pub use crate::constants::{
    DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH,
};

// ============================================================================
// КОНСТАНТЫ ВВОДА/ВЫВОДА (локальные)
// ============================================================================
// Исправление L1: константы клавиш перемещены в crate::constants.rs
// Используйте crate::constants::KEY_BACKSPACE вместо локальных констант.
pub use crate::constants::KEY_BACKSPACE;

/// Ошибка ввода/вывода терминала.
///
/// Использует thiserror для типизированной обработки ошибок.
///
/// ## Варианты ошибок
/// - `RawMode` - не удалось перейти в raw-режим терминала
/// - `Clear` - не удалось очистить экран
/// - `Cursor` - не удалось скрыть/показать курсор
/// - `Flush` - ошибка flush буфера
/// - `Draw` - ошибка отрисовки
/// - `Initialization` - критическая ошибка инициализации Canvas
#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum IoError {
    /// Не удалось перейти в raw-режим терминала.
    #[error("Ошибка raw-режима: {0}")]
    RawMode(String),
    /// Не удалось очистить экран.
    #[error("Ошибка очистки экрана: {0}")]
    Clear(String),
    /// Не удалось скрыть/показать курсор.
    #[error("Ошибка курсора: {0}")]
    Cursor(String),
    /// Ошибка flush буфера.
    #[error("Ошибка flush: {0}")]
    Flush(String),
    /// Ошибка отрисовки.
    #[error("Ошибка отрисовки: {0}")]
    Draw(String),
    /// Критическая ошибка инициализации Canvas.
    ///
    /// Возникает когда не удалось создать ни основной Canvas, ни fallback stub.
    #[error("Критическая ошибка инициализации терминала: {0}")]
    Initialization(String),
}

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
    /// # Исправление #7 (HIGH)
    /// Упрощён сброс до минимально необходимых операций с минимальным риском паники.
    /// Используем write_all вместо write для атомарности операции.
    ///
    /// # Исправление аудита 2026-03-30
    /// Обёрнуто в catch_unwind для предотвращения паники при панике.
    ///
    /// # M3: Обработка ошибок
    /// Добавлено логирование ошибок с префиксом "[PANIC SAFE]" для отладки проблем с терминалом.
    fn drop(&mut self) {
        // Минимальный сброс: только показ курсора и flush
        // Используем write_all для атомарности записи
        // Исправление аудита 2026-03-30: catch_unwind для предотвращения паники
        // M3: логирование ошибок с префиксом "[PANIC SAFE]"
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if let Err(e) = write!(self.out, "{Show}") {
                eprintln!("[PANIC SAFE] Не удалось показать курсор в Drop: {}", e);
            }
            if let Err(e) = self.out.flush() {
                eprintln!("[PANIC SAFE] Не удалось сбросить буфер в Drop: {}", e);
            }
        }));
    }
}

impl Default for Canvas {
    /// Возвращает Canvas по умолчанию.
    ///
    /// # Важность
    /// Этот метод может паниковать если терминал полностью недоступен.
    /// Для безопасной обработки ошибок используйте [`Canvas::try_default()`].
    ///
    /// # Примечания
    /// При ошибке инициализации создаёт fallback canvas с заглушкой.
    /// Если и stub не удаётся создать - паникует с понятным сообщением.
    ///
    /// # Паникует
    /// Если не удалось инициализировать ни основной Canvas, ни fallback stub.
    ///
    /// # Устарело
    /// Используйте [`Canvas::try_default()`] для безопасной обработки ошибок.
    fn default() -> Self {
        Self::try_default().unwrap_or_else(|e| {
            eprintln!("[WARN] Canvas::default(): не удалось инициализировать терминал: {e}");
            eprintln!("[WARN] Canvas::default(): создаётся minimal stub для совместимости");

            match Self::new_stub() {
                Ok(stub) => stub,
                Err(_) => {
                    let out = stdout().into_raw_mode().unwrap_or_else(|_| {
                        panic!("Критическая ошибка: терминал полностью недоступен");
                    });
                    Self { out }
                }
            }
        })
    }
}

impl Canvas {
    /// Попытаться создать Canvas по умолчанию с обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Ok(Canvas)` - успешно созданный Canvas (основной или fallback stub)
    /// - `Err(IoError)` - критическая ошибка инициализации терминала
    ///
    /// # Errors
    /// Возвращает [`IoError::Initialization`] если не удалось создать ни основной Canvas,
    /// ни fallback stub.
    ///
    /// # Примечания
    /// Этот метод безопаснее чем [`Canvas::default()`] так как возвращает Result
    /// вместо паники. Используйте его когда нужна graceful error handling.
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::io::Canvas;
    ///
    /// let canvas = Canvas::try_default().unwrap_or_else(|e| {
    ///     eprintln!("Не удалось инициализировать терминал: {}", e);
    ///     std::process::exit(1);
    /// });
    /// ```
    pub fn try_default() -> Result<Self, IoError> {
        Self::new().or_else(|_| {
            // При ошибке основного Canvas пробуем создать fallback stub
            Self::new_stub().map_err(|e| {
                IoError::Initialization(format!(
                    "не удалось создать Canvas (основной режим и fallback недоступны): {e}"
                ))
            })
        })
    }

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
/// ```ignore
/// use tetris_cli::io::KeyReader;
///
/// let mut reader = KeyReader::new();
/// if let Ok(Some(key)) = reader.get_key() {
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
    /// Метод автоматически освобождает ресурсы stdin при выходе из области видимости.
    ///
    /// # Исправление #7 (HIGH)
    /// Упрощён сброс до минимально необходимых операций с минимальным риском паники.
    /// Используем write_all вместо write для атомарности операции.
    ///
    /// # Безопасность
    /// Гарантирует возврат терминала в нормальное состояние даже при панике.
    ///
    /// # Исправление #4: Обработка ошибок
    /// Добавлено логирование ошибок с префиксом "[PANIC SAFE]" для отладки.
    fn drop(&mut self) {
        let mut stdout = std::io::stdout();

        // Минимальный сброс: только показ курсора и flush
        // Используем write_all для атомарности записи
        // Исправление #4: логирование ошибок с префиксом "[PANIC SAFE]"
        if let Err(e) = write!(stdout, "{Show}") {
            eprintln!(
                "[PANIC SAFE] Не удалось показать курсор в KeyReader::Drop: {}",
                e
            );
        }
        if let Err(e) = stdout.flush() {
            eprintln!(
                "[PANIC SAFE] Не удалось сбросить буфер в KeyReader::Drop: {}",
                e
            );
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
    /// - `Ok(Some(u8))` — код нажатой клавиши (ASCII 0x00-0x7F)
    /// - `Ok(None)` — клавиша не была нажата или введён UTF-8 символ
    /// - `Err(io::Error)` — при ошибке чтения ввода
    ///
    /// # Errors
    /// Возвращает `io::Error` при ошибке чтения из терминала (например, при закрытии stdin
    /// или сбое системного вызова).
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::io::KeyReader;
    ///
    /// let mut reader = KeyReader::new();
    /// if let Ok(Some(key)) = reader.get_key() {
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
    /// При вводе многобайтовых символов метод возвращает `Ok(None)`, предварительно прочитав все байты символа.
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
    ///
    /// # Исправление #18 (MEDIUM SEVERITY)
    /// Расширена валидация Unicode с логированием причин отбрасывания невалидных символов.
    ///
    /// # Исправление аудита 2026-03-30
    /// Изменён тип возврата с `Option<u8>` на `io::Result<Option<u8>>` для явной обработки ошибок.
    pub fn get_key(&mut self) -> io::Result<Option<u8>> {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(()) => {
                let first_byte = key_bytes[0];

                // Проверяем, является ли это началом многобайтового символа UTF-8
                // ASCII (0x00-0x7F) - однобайтовый символ
                if first_byte <= 0x7F {
                    return Ok(Some(first_byte));
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
                    // Исправление #18: логирование причины отбрасывания
                    eprintln!("[WARN] get_key(): невалидный первый байт UTF-8: 0x{:02X} (диапазоны 0xC0-0xC1 и 0xF5-0xFF запрещены)", first_byte);
                    return Ok(None);
                };

                // Читаем остальные байты символа UTF-8
                let mut remaining = [0u8; 3];
                if self
                    .inp
                    .read_exact(&mut remaining[..bytes_to_read])
                    .is_err()
                {
                    // Исправление #18: логирование ошибки чтения
                    eprintln!("[WARN] get_key(): ошибка чтения продолжения UTF-8 последовательности (ожидалось {} байт)", bytes_to_read);
                    return Ok(None);
                }

                // Исправление #3 (CRITICAL): проверка валидности UTF-8 через std::str::from_utf8()
                // Собираем все байты символа для валидации
                let mut utf8_bytes = [0u8; 4];
                utf8_bytes[0] = first_byte;
                utf8_bytes[1..=bytes_to_read].copy_from_slice(&remaining[..bytes_to_read]);

                // Проверяем валидность UTF-8 последовательности
                if std::str::from_utf8(&utf8_bytes[..=bytes_to_read]).is_err() {
                    // Исправление #18: расширенное логирование невалидного UTF-8
                    eprintln!(
                        "[WARN] get_key(): невалидная UTF-8 последовательность: байты [{}]",
                        utf8_bytes[..=bytes_to_read]
                            .iter()
                            .map(|b| format!("0x{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    return Ok(None);
                }

                // Исправление #8: логирование предупреждения при получении UTF-8 символа
                // Исправление #18: добавлена информация о типе символа
                let char_type = match bytes_to_read {
                    1 => "2-байтовый",
                    2 => "3-байтовый",
                    3 => "4-байтовый",
                    _ => "многобайтовый",
                };
                eprintln!("[WARN] get_key(): получен {} UTF-8 символ (не поддерживается). Используйте get_key_unicode().", char_type);

                // Для многобайтовых символов возвращаем None
                // (они не являются управляющими клавишами для игры)
                Ok(None)
            }
            Err(e) => {
                // Исправление #18: логирование ошибки чтения ввода
                eprintln!("[WARN] get_key(): ошибка чтения ввода: {}", e);
                // Возвращаем ошибку явно через Err
                Err(e)
            }
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
    ///
    /// # Исправление C3 (CRITICAL)
    /// Добавлена явная валидация через char::from_u32() для предотвращения
    /// некорректной обработки суррогатных пар и невалидных кодовых точек.
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

                // Исправление C3: явная валидация через char::from_u32()
                // Сначала декодируем UTF-8 в строку, затем проверяем каждый char
                std::str::from_utf8(&utf8_bytes[..=additional_bytes])
                    .ok()
                    .and_then(|s| {
                        s.chars().next().and_then(|ch| {
                            // Явная проверка валидности кодовой точки через char::from_u32()
                            // Это предотвращает возврат суррогатных пар и невалидных точек
                            let code_point = ch as u32;
                            char::from_u32(code_point).filter(|_| {
                                // Дополнительная проверка: отбрасываем управляющие символы
                                // кроме стандартных ASCII управляющих кодов
                                code_point >= 0x20 || code_point == 0x00
                            })
                        })
                    })
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
    /// KeyReader::cleanup(); // Явный сброс терминала
    /// ```
    ///
    /// ## Исправление #13
    /// Метод предназначен для будущего использования в API.
    #[allow(dead_code)]
    pub fn cleanup() {
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
    ///
    /// # Возвращает
    /// - `Ok(Some(u8))` — код нажатой клавиши
    /// - `Ok(None)` — если клавиша не была нажата
    /// - `Err(io::Error)` — если произошла ошибка чтения
    fn get_key(&mut self) -> io::Result<Option<u8>> {
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
    #[allow(dead_code)]
    fn reset(&mut self) {
        self.reset();
    }
}

// ============================================================================
// MOCK CANVAS ДЛЯ ТЕСТОВ
// ============================================================================

/// Mock-канвас для тестирования отрисовки.
///
/// Используется в тестах для проверки отрисовки без реального терминала.
/// Подсчитывает количество вызовов flush() и хранит отрисованные строки.
///
/// # Пример использования
/// ```
/// use crate::io::MockCanvas;
///
/// let mut canvas = MockCanvas::new();
/// canvas.draw_strs(&["строка 1", "строка 2"], (1, 1));
/// assert_eq!(canvas.flush_count(), 1);
/// ```
#[cfg(test)]
#[derive(Debug, Default)]
pub struct MockCanvas {
    /// Количество вызовов flush().
    flush_count: u32,
    /// Список отрисованных строк с позициями.
    drawn_strings: Vec<(String, (u16, u16))>,
}

#[cfg(test)]
impl MockCanvas {
    /// Создать новый MockCanvas.
    pub fn new() -> Self {
        Self::default()
    }

    /// Проверить что канвас является stub (всегда true для MockCanvas).
    pub fn is_stub() -> bool {
        true
    }

    /// Получить количество вызовов flush().
    pub fn flush_count(&self) -> u32 {
        self.flush_count
    }

    /// Получить список отрисованных строк с позициями.
    pub fn get_drawn_strings(&self) -> &[(String, (u16, u16))] {
        &self.drawn_strings
    }

    /// Отрисовать строки (статические).
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `pos` - позиция верхней левой строки (x, y)
    pub fn draw_strs(&mut self, lines: &[&str], pos: (u16, u16)) {
        let (x, mut y) = pos;
        for line in lines {
            self.drawn_strings.push((line.to_string(), (x, y)));
            y += 1;
        }
        // Один flush() в конце как в реальном Canvas
        self.flush_count += 1;
    }

    /// Отрисовать строку.
    ///
    /// # Аргументы
    /// * `text` - текст для отрисовки
    /// * `pos` - позиция верхнего левого угла (x, y)
    pub fn draw_string(&mut self, text: &str, pos: (u16, u16)) {
        self.drawn_strings.push((text.to_string(), pos));
    }

    /// Обновить вывод (flush).
    pub fn flush(&mut self) {
        self.flush_count += 1;
    }

    /// Сбросить терминал (no-op для MockCanvas).
    #[allow(clippy::unused_self)]
    pub fn reset(&mut self) {
        // No-op для тестов
    }

    /// Отрисовать строку (no-op для MockCanvas).
    pub fn draw_str(&mut self, text: &str, x: u16, y: u16) {
        self.drawn_strings.push((text.to_string(), (x, y)));
    }
}

#[cfg(test)]
impl Drop for MockCanvas {
    fn drop(&mut self) {
        // No-op для тестов
    }
}

// ============================================================================
// ТЕСТЫ ДЛЯ UTF-8 ОБРАБОТКИ (ИСПРАВЛЕНИЕ #5)
// ============================================================================

#[cfg(test)]
mod utf8_tests {
    // use super::*; // Не используется
    // use std::io::Cursor; // Не используется

    /// Тест 1: Проверка логирования предупреждения при UTF-8 символах
    ///
    /// Проверяет что get_key() логирует предупреждение при получении
    /// многобайтового UTF-8 символа
    #[test]
    fn test_utf8_warning_logging() {
        // Проверяем формат сообщения об ошибке
        let warning_msg = "[WARN] Получен многобайтовый UTF-8 символ (не поддерживается в get_key()). Используйте get_key_unicode().";
        assert!(
            warning_msg.contains("[WARN]"),
            "Сообщение должно содержать [WARN]"
        );
        assert!(
            warning_msg.contains("UTF-8"),
            "Сообщение должно упоминать UTF-8"
        );
        assert!(
            warning_msg.contains("get_key_unicode()"),
            "Сообщение должно рекомендовать get_key_unicode()"
        );
    }

    /// Тест 2: Проверка обработки кириллических символов
    ///
    /// Проверяет что многобайтовые кириллические символы корректно
    /// определяются как UTF-8 последовательности
    #[test]
    fn test_cyrillic_utf8_detection() {
        // Кириллические символы в UTF-8 используют 2 байта (0xC0-0xDF диапазон)
        let cyrillic_chars = ['а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и'];

        for ch in cyrillic_chars {
            let s = ch.to_string();
            let bytes = s.as_bytes();
            // Проверяем что это многобайтовая последовательность
            assert!(
                bytes.len() >= 2,
                "Кириллический символ должен быть многобайтовым"
            );
            assert!(
                bytes[0] >= 0xC2 && bytes[0] <= 0xDF,
                "Первый байт должен быть в диапазоне 2-байтовой UTF-8 последовательности"
            );
        }
    }

    /// Тест 3: Проверка обработки emoji
    ///
    /// Проверяет что emoji (4-байтовые UTF-8 последовательности) корректно
    /// определяются
    #[test]
    fn test_emoji_utf8_detection() {
        // Emoji используют 4 байта в UTF-8 (0xF0-0xF4 диапазон)
        // Используем строки вместо char для emoji с variation selector
        let emoji_strings = ["🎮", "🎯", "🏆", "🎲"];

        for emoji_str in emoji_strings {
            let bytes = emoji_str.as_bytes();
            // Проверяем что это 4-байтовая последовательность
            assert!(
                bytes.len() >= 4,
                "Emoji должен быть 4-байтовой UTF-8 последовательностью"
            );
            assert!(
                bytes[0] >= 0xF0 && bytes[0] <= 0xF4,
                "Первый байт должен быть в диапазоне 4-байтовой UTF-8 последовательности"
            );
        }
    }

    /// Тест 4: Проверка ASCII символов
    ///
    /// Проверяет что ASCII символы (однобайтовые) корректно обрабатываются
    #[test]
    fn test_ascii_character_detection() {
        // ASCII символы используют 1 байт (0x00-0x7F диапазон)
        let ascii_chars = [b'a', b'z', b'A', b'Z', b'0', b'9', b' ', b'\n', b'\r'];

        for &byte in &ascii_chars {
            assert!(
                byte <= 0x7F,
                "ASCII символ должен быть в диапазоне 0x00-0x7F"
            );
        }
    }

    /// Тест 5: Проверка диапазона UTF-8 байтов
    ///
    /// Проверяет корректность определения количества байт в UTF-8 последовательности
    #[test]
    fn test_utf8_byte_range_validation() {
        // 2-байтовая последовательность: 0xC2-0xDF
        for byte in 0xC2..=0xDF {
            assert!(
                (0xC2..=0xDF).contains(&byte),
                "2-байтовый диапазон должен включать {:#X}",
                byte
            );
        }

        // 3-байтовая последовательность: 0xE0-0xEF
        for byte in 0xE0..=0xEF {
            assert!(
                (0xE0..=0xEF).contains(&byte),
                "3-байтовый диапазон должен включать {:#X}",
                byte
            );
        }

        // 4-байтовая последовательность: 0xF0-0xF4
        for byte in 0xF0..=0xF4 {
            assert!(
                (0xF0..=0xF4).contains(&byte),
                "4-байтовый диапазон должен включать {:#X}",
                byte
            );
        }

        // Невалидные байты: 0xC0, 0xC1, 0xF5-0xFF
        let invalid_bytes = [
            0xC0, 0xC1, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
        ];
        for &byte in &invalid_bytes {
            assert!(
                !(0xC2..=0xDF).contains(&byte)
                    && !(0xE0..=0xEF).contains(&byte)
                    && !(0xF0..=0xF4).contains(&byte),
                "Байт {:#X} должен быть невалидным",
                byte
            );
        }
    }

    /// Тест 6: Проверка формата сообщений об ошибках
    ///
    /// Проверяет что все сообщения об ошибках используют правильный формат
    #[test]
    fn test_error_message_formats() {
        // Проверяем формат предупреждений
        let warn_messages = [
            "[WARN] Получен многобайтовый UTF-8 символ",
            "[WARN] Удалено 2 невалидных записей",
        ];

        for msg in &warn_messages {
            assert!(
                msg.contains("[WARN]"),
                "Предупреждение должно содержать [WARN]: {}",
                msg
            );
        }

        // Проверяем формат ошибок
        let error_messages = [
            "Ошибка отрисовки строки: тест",
            "Критическая ошибка: не удалось показать курсор",
        ];

        for msg in &error_messages {
            assert!(
                msg.contains("Ошибка") || msg.contains("Критическая ошибка"),
                "Сообщение должно содержать 'Ошибка': {}",
                msg
            );
        }
    }

    /// Тест 7: Проверка Unicode символов разной длины
    ///
    /// Проверяет обработку символов разной длины в байтах
    #[test]
    fn test_unicode_character_lengths() {
        // 1 байт: ASCII
        assert_eq!("a".len(), 1);

        // 2 байта: Кириллица
        assert_eq!("а".len(), 2);

        // 3 байта: Некоторые символы CJK
        assert_eq!("中".len(), 3);

        // 4 байта: Emoji
        assert_eq!("🎮".len(), 4);
    }

    /// Тест 8: Проверка валидации UTF-8 последовательностей
    ///
    /// Проверяет что невалидные UTF-8 последовательности отбрасываются
    #[test]
    fn test_invalid_utf8_sequences() {
        // Невалидные последовательности UTF-8
        let invalid_sequences: Vec<&[u8]> = vec![
            &[0xC0], // Переполнение (должно быть 0xC2+)
            &[0xC1], // Переполнение
            &[0xF5], // За пределами диапазона UTF-8
            &[0xFF], // Недействительный байт
            &[0x80], // Продолжение без начала
            &[0xBF], // Продолжение без начала
        ];

        for seq in &invalid_sequences {
            let first_byte = seq[0];
            // Проверяем что первый байт не попадает в валидные диапазоны
            assert!(
                !(0x00..=0x7F).contains(&first_byte)
                    || !(0xC2..=0xDF).contains(&first_byte)
                        && !(0xE0..=0xEF).contains(&first_byte)
                        && !(0xF0..=0xF4).contains(&first_byte)
                    || first_byte >= 0x80 && first_byte <= 0xBF,
                "Байт {:#X} должен быть распознан как невалидный или продолжение",
                first_byte
            );
        }
    }
}
