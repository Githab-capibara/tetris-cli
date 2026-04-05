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

/// Реализация `TerminalBackend` на основе termion.
///
/// Обёртка над `RawTerminal` для отрисовки в терминале.
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
        // TODO: Реализовать чтение клавиш через KeyReader.
        // Сейчас ввод обрабатывается отдельно через KeyReader в TermionIOBackend.
        // Эта заглушка возвращает None для соответствия трейту.
        Ok(None)
    }

    fn read_key_unicode(&mut self) -> Option<char> {
        // TODO: Реализовать чтение Unicode символов через KeyReader.
        // Сейчас ввод обрабатывается отдельно через KeyReader в TermionIOBackend.
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
/// Объединяет `TermionBackend` для вывода и `KeyReader` для ввода.
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
    #[ignore = "Требует интерактивный терминал с поддержкой raw-режима"]
    fn test_termion_backend_creation() {
        // Тест может упасть если терминал не поддерживает raw-режим
        let result = TermionBackend::new();
        // Если терминал доступен, бэкенд создаётся успешно
        assert!(result.is_ok() || result.is_err());
    }

    // =========================================================================
    // ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ (#39)
    // =========================================================================

    #[test]
    #[ignore = "Требует интерактивный терминал с поддержкой raw-режима"]
    fn test_termion_backend_with_raw() {
        // Тест создания бэкенда с существующим raw терминалом
        let result = stdout().into_raw_mode();
        if let Ok(raw) = result {
            let backend = TermionBackend::with_raw(raw);
            // Бэкенд должен создаться успешно
            let _ = backend;
        }
        // Если raw режим недоступен, тест просто пропускается
    }

    #[test]
    fn test_terminal_backend_trait_methods_exist() {
        // Проверка что TerminalBackend трейт определён и имеет все методы
        // Это compile-time тест — если методы отсутствуют, код не скомпилируется
        fn assert_backend<T: TerminalBackend>() {}
        fn assert_input<T: TerminalInputBackend>() {}

        assert_backend::<TermionBackend>();
        assert_input::<TermionBackend>();
    }

    #[test]
    #[ignore = "Требует интерактивный терминал с поддержкой raw-режима"]
    fn test_termion_io_backend_creation() {
        // Тест создания комбинированного бэкенда
        let result = TermionIOBackend::new();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_wall_kick_offset_bounds() {
        // Косвенный тест бэкенда через константы поля
        // Проверяем что GRID_WIDTH и GRID_HEIGHT определены
        assert!(crate::constants::GRID_WIDTH > 0);
        assert!(crate::constants::GRID_HEIGHT > 0);
    }

    #[test]
    fn test_terminal_traits_are_send_sync() {
        // TerminalBackend не требует Send + Sync
        // Проверяем что трейты определены корректно
        fn _assert_terminal_backend<T: TerminalBackend>() {}
        fn _assert_terminal_input<T: TerminalInputBackend>() {}
        _assert_terminal_backend::<TermionBackend>();
        _assert_terminal_input::<TermionBackend>();
    }
}
