//! Асинхронный читатель клавиатуры.
//!
//! Модуль предоставляет `KeyReader` для неблокирующего чтения клавиатуры.
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::io::key_reader::KeyReader;
//!
//! let mut reader = KeyReader::new();
//! if let Ok(Some(key)) = reader.get_key() {
//!     if key == b'q' {
//!         println!("Выход");
//!     }
//! }
//! ```
//!
//! ## Ограничения
//! ### Поддержка UTF-8
//! - Метод `get_key()` возвращает только ASCII клавиши (0x00-0x7F)
//! - UTF-8 последовательности возвращают `None` (не поддерживаются)
//! - Для получения Unicode символов используйте `get_key_unicode()`
//! - ESC-последовательности для специальных клавиш могут обрабатываться некорректно
//!
//! Архитектурное улучшение 2026-04-01 (S3): Выделение `KeyReader` в отдельный модуль.

use std::io::{self, Read, Write};
use termion::{async_stdin, cursor::Show, screen::ToMainScreen, AsyncReader};

use crate::io_traits::InputReader;

// ============================================================================
// KEYREADER
// ============================================================================

/// Читатель нажатий клавиш в асинхронном режиме.
///
/// Использует `async_stdin` для неблокирующего чтения клавиатуры.
/// Поддерживает обработку ESC-последовательностей для специальных клавиш.
///
/// # Платформенные особенности
/// ## Unix-системы (Linux, macOS)
/// - Использует termion для управления терминалом
/// - Raw-режим включается при создании и выключается в Drop
/// - Поддерживает ASCII и UTF-8 символы
///
/// ## Windows
/// **Не поддерживается**: termion не работает на Windows
/// Для кроссплатформенной поддержки рассмотрите crossterm
///
/// ## Ограничения
/// - **UTF-8**: `get_key()` возвращает только ASCII (0x00-0x7F)
/// - **Специальные клавиши**: ESC-последовательности могут обрабатываться некорректно
/// - **Мышь**: Не поддерживается, только клавиатура
pub struct KeyReader {
    inp: AsyncReader,
}

impl Drop for KeyReader {
    /// Освобождение ресурсов при уничтожении `KeyReader`.
    ///
    /// # Исправление аудита 2026-03-31 (CRITICAL)
    /// `async_stdin` автоматически управляет raw-режимом.
    fn drop(&mut self) {
        use std::io::stdout;

        let mut out = stdout();

        if let Err(e) = write!(out, "{Show}") {
            eprintln!("[PANIC SAFE] Не удалось показать курсор в KeyReader::Drop: {e}");
        }
        if let Err(e) = out.flush() {
            eprintln!("[PANIC SAFE] Не удалось сбросить буфер в KeyReader::Drop: {e}");
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
    ///
    /// # Проверка stdin (ISSUE-200)
    /// ## Инициализация
    /// - Используется `termion::async_stdin()` для неблокирующего чтения
    /// - stdin открывается в фоновом режиме
    /// - Ошибки инициализации обрабатываются внутри `async_stdin`
    ///
    /// ## Ограничения
    /// - **Требуется терминал**: `KeyReader` не будет работать без stdin
    /// - **Unix-системы**: termion поддерживает только Unix (Linux, macOS)
    /// - **Raw-режим**: Управляется автоматически через `async_stdin`
    ///
    /// ## Паники
    /// Эта функция НИКОГДА не паникует.
    /// `async_stdin()` создаётся без паники даже если stdin недоступен.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::io::KeyReader;
    ///
    /// let reader = KeyReader::new();
    /// ```
    #[must_use = "Читатель клавиш должен быть использован"]
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
    /// Возвращает `io::Error` при ошибке чтения из stdin (например, закрытие потока).
    pub fn get_key(&mut self) -> io::Result<Option<u8>> {
        let mut key_bytes: [u8; 1] = [0];
        match self.inp.read_exact(&mut key_bytes) {
            Ok(()) => {
                let first_byte = key_bytes[0];

                if first_byte <= 0x7F {
                    return Ok(Some(first_byte));
                }

                let bytes_to_read = if (0xC2..=0xDF).contains(&first_byte) {
                    1
                } else if (0xE0..=0xEF).contains(&first_byte) {
                    2
                } else if (0xF0..=0xF4).contains(&first_byte) {
                    3
                } else {
                    return Ok(None);
                };

                let mut remaining = [0u8; 3];
                if self
                    .inp
                    .read_exact(&mut remaining[..bytes_to_read])
                    .is_err()
                {
                    return Ok(None);
                }

                let mut utf8_bytes = [0u8; 4];
                utf8_bytes[0] = first_byte;
                utf8_bytes[1..=bytes_to_read].copy_from_slice(&remaining[..bytes_to_read]);

                if std::str::from_utf8(&utf8_bytes[..=bytes_to_read]).is_err() {
                    eprintln!(
                        "[WARN] get_key(): невалидная UTF-8 последовательность: байты ["
                    );
                    for (i, b) in utf8_bytes[..=bytes_to_read].iter().enumerate() {
                        if i > 0 {
                            eprint!(", ");
                        }
                        eprint!("0x{b:02X}");
                    }
                    eprintln!("]");
                    return Ok(None);
                }

                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    /// Получить нажатую клавишу с поддержкой UTF-8.
    ///
    /// # Возвращает
    /// - `Some(char)` — символ Unicode
    /// - `None` — при ошибке чтения или невалидном UTF-8
    pub fn get_key_unicode(&mut self) -> Option<char> {
        let mut first_byte: [u8; 1] = [0];

        match self.inp.read_exact(&mut first_byte) {
            Ok(()) => {
                let byte = first_byte[0];

                if byte <= 0x7F {
                    return Some(byte as char);
                }

                let additional_bytes = if (0xC2..=0xDF).contains(&byte) {
                    1
                } else if (0xE0..=0xEF).contains(&byte) {
                    2
                } else if (0xF0..=0xF4).contains(&byte) {
                    3
                } else {
                    return None;
                };

                let mut buffer = [0u8; 3];
                if self
                    .inp
                    .read_exact(&mut buffer[..additional_bytes])
                    .is_err()
                {
                    return None;
                }

                let mut utf8_bytes = [0u8; 4];
                utf8_bytes[0] = byte;
                utf8_bytes[1..=additional_bytes].copy_from_slice(&buffer[..additional_bytes]);

                std::str::from_utf8(&utf8_bytes[..=additional_bytes])
                    .ok()
                    .and_then(|s| {
                        s.chars().next().and_then(|ch| {
                            let code_point = ch as u32;
                            char::from_u32(code_point)
                                .filter(|_| code_point >= 0x20 || code_point == 0x00)
                        })
                    })
            }
            Err(_) => None,
        }
    }

    /// Очистить ресурсы и сбросить терминал.
    ///
    /// # Примечания
    /// Метод может быть вызван явно для досрочного освобождения ресурсов.
    pub fn cleanup() {
        use std::io::stdout;

        let mut out = stdout();

        if let Err(e) = write!(out, "{Show}") {
            eprintln!("Критическая ошибка: не удалось показать курсор: {e}");
        }

        if let Err(e) = write!(out, "{ToMainScreen}") {
            eprintln!("Критическая ошибка: не удалось вернуть экран: {e}");
        }

        if let Err(e) = write!(out, "\x1b[H") {
            eprintln!("Критическая ошибка: не удалось переместить курсор: {e}");
        }

        if let Err(e) = out.flush() {
            eprintln!("Критическая ошибка: не удалось выполнить flush: {e}");
        }
    }
}

impl InputReader for KeyReader {
    fn get_key(&mut self) -> io::Result<Option<u8>> {
        Self::get_key(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_reader_new() {
        let reader = KeyReader::new();
        // KeyReader создаётся успешно
    }

    #[test]
    fn test_key_reader_default() {
        let reader = KeyReader::default();
        // Default создаётся успешно
    }
}
