//! Трейт бэкенда терминала для кроссплатформенной отрисовки.
//!
//! ## Исправление #13 (TerminalBackend абстракция)
//! Этот трейт определяет абстракцию для работы с терминалом,
//! что позволяет в будущем реализовать различные бэкенды:
//! - termion (Unix/Linux)
//! - crossterm (кроссплатформенный)
//! - windows-console (Windows)
//!
//! ## Архитектурные заметки
//! Трейт предоставляет минимальный интерфейс для:
//! - Инициализации терминала
//! - Отрисовки текста
//! - Управления курсором
//! - Сброса терминала

use std::io;

/// Результат операций терминала.
pub type TerminalResult<T> = Result<T, TerminalError>;

/// Ошибка терминала.
#[derive(Debug)]
pub enum TerminalError {
    /// Ошибка ввода/вывода.
    Io(io::Error),
    /// Не удалось инициализировать терминал.
    Initialization(String),
    /// Ошибка отрисовки.
    Render(String),
    /// Неподдерживаемая операция.
    Unsupported(String),
}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalError::Io(e) => write!(f, "Ошибка ввода/вывода: {e}"),
            TerminalError::Initialization(msg) => write!(f, "Ошибка инициализации: {msg}"),
            TerminalError::Render(msg) => write!(f, "Ошибка отрисовки: {msg}"),
            TerminalError::Unsupported(msg) => write!(f, "Неподдерживаемая операция: {msg}"),
        }
    }
}

impl std::error::Error for TerminalError {}

impl From<io::Error> for TerminalError {
    fn from(err: io::Error) -> Self {
        TerminalError::Io(err)
    }
}

/// Трейт бэкенда терминала.
///
/// Определяет минимальный интерфейс для работы с терминалом.
/// Реализации этого трейта могут использовать различные библиотеки:
/// - termion (Unix/Linux)
/// - crossterm (кроссплатформенный)
/// - windows-console (Windows)
///
/// ## Пример реализации
/// ```ignore
/// use tetris_cli::terminal_backend::{TerminalBackend, TerminalResult};
///
/// struct MyTerminalBackend {
///     // поля бэкенда
/// }
///
/// impl TerminalBackend for MyTerminalBackend {
///     fn init(&mut self) -> TerminalResult<()> {
///         // инициализация
///         Ok(())
///     }
///
///     fn draw_char(&mut self, ch: char, x: u16, y: u16) -> TerminalResult<()> {
///         // отрисовка символа
///         Ok(())
///     }
///
///     fn flush(&mut self) -> TerminalResult<()> {
///         // сброс буфера
///         Ok(())
///     }
///
///     fn reset(&mut self) -> TerminalResult<()> {
///         // сброс терминала
///         Ok(())
///     }
/// }
/// ```
pub trait TerminalBackend: Send {
    /// Инициализировать терминал.
    ///
    /// Должен вызываться перед использованием других методов.
    /// Переводит терминал в raw-режим, скрывает курсор.
    ///
    /// # Errors
    /// Возвращает ошибку, если не удалось перейти в raw-режим терминала,
    /// очистить экран или скрыть курсор.
    ///
    /// # Возвращает
    /// - `Ok(())` если инициализация успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn init(&mut self) -> TerminalResult<()>;

    /// Сбросить терминал в исходное состояние.
    ///
    /// Должен вызываться перед завершением программы.
    /// Возвращает терминал в нормальный режим, показывает курсор.
    ///
    /// # Errors
    /// Возвращает ошибку, если не удалось показать курсор или выполнить flush буфера.
    ///
    /// # Возвращает
    /// - `Ok(())` если сброс успешен
    /// - `Err(TerminalError)` если произошла ошибка
    fn reset(&mut self) -> TerminalResult<()>;

    /// Отрисовать символ в указанной позиции.
    ///
    /// # Аргументы
    /// * `ch` - символ для отрисовки
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка записи в терминал.
    ///
    /// # Возвращает
    /// - `Ok(())` если отрисовка успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn draw_char(&mut self, ch: char, x: u16, y: u16) -> TerminalResult<()>;

    /// Отрисовать строку в указанной позиции.
    ///
    /// # Аргументы
    /// * `text` - текст для отрисовки
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка записи в терминал.
    ///
    /// # Возвращает
    /// - `Ok(())` если отрисовка успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn draw_string(&mut self, text: &str, x: u16, y: u16) -> TerminalResult<()>;

    /// Отрисовать строки (многострочный текст).
    ///
    /// # Аргументы
    /// * `lines` - массив строк для отрисовки
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка записи в терминал.
    ///
    /// # Возвращает
    /// - `Ok(())` если отрисовка успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn draw_lines(&mut self, lines: &[&str], x: u16, y: u16) -> TerminalResult<()>;

    /// Очистить экран.
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка очистки экрана терминала.
    ///
    /// # Возвращает
    /// - `Ok(())` если очистка успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn clear(&mut self) -> TerminalResult<()>;

    /// Переместить курсор в указанную позицию.
    ///
    /// # Аргументы
    /// * `x` - координата X (1-based)
    /// * `y` - координата Y (1-based)
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка перемещения курсора.
    ///
    /// # Возвращает
    /// - `Ok(())` если перемещение успешно
    /// - `Err(TerminalError)` если произошла ошибка
    fn move_cursor(&mut self, x: u16, y: u16) -> TerminalResult<()>;

    /// Скрыть курсор.
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка скрытия курсора.
    ///
    /// # Возвращает
    /// - `Ok(())` если операция успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn hide_cursor(&mut self) -> TerminalResult<()>;

    /// Показать курсор.
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибки показа курсора.
    ///
    /// # Возвращает
    /// - `Ok(())` если операция успешна
    /// - `Err(TerminalError)` если произошла ошибка
    fn show_cursor(&mut self) -> TerminalResult<()>;

    /// Сбросить буфер вывода на экран.
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка flush буфера.
    ///
    /// # Возвращает
    /// - `Ok(())` если сброс успешен
    /// - `Err(TerminalError)` если произошла ошибка
    fn flush(&mut self) -> TerminalResult<()>;

    /// Получить ширину терминала в символах.
    ///
    /// # Возвращает
    /// Ширина терминала или None если не удалось получить
    fn width(&self) -> Option<u16>;

    /// Получить высоту терминала в символах.
    ///
    /// # Возвращает
    /// Высота терминала или None если не удалось получить
    fn height(&self) -> Option<u16>;
}

/// Базовая реализация по умолчанию для некоторых методов.
pub trait TerminalBackendExt: TerminalBackend {
    /// Отрисовать строку с цветом.
    ///
    /// # Аргументы
    /// * `text` - текст для отрисовки
    /// * `x` - координата X
    /// * `y` - координата Y
    /// * `fg` - цвет переднего плана (ANSI код 0-255)
    /// * `bg` - цвет фона (ANSI код 0-255)
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка записи в терминал.
    ///
    /// # Возвращает
    /// Результат отрисовки
    fn draw_colored_string(
        &mut self,
        text: &str,
        x: u16,
        y: u16,
        fg: u8,
        bg: u8,
    ) -> TerminalResult<()> {
        // Реализация по умолчанию: просто отрисовываем строку
        // Цвета игнорируются в базовой реализации
        self.draw_string(text, x, y)
    }

    /// Отрисовать рамку.
    ///
    /// # Аргументы
    /// * `x` - координата X левого верхнего угла
    /// * `y` - координата Y левого верхнего угла
    /// * `width` - ширина рамки
    /// * `height` - высота рамки
    ///
    /// # Errors
    /// Возвращает ошибку, если произошла ошибка перемещения курсора или отрисовки символов.
    ///
    /// # Возвращает
    /// Результат отрисовки
    fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) -> TerminalResult<()> {
        // Простая реализация рамки из ASCII символов
        let h_line = "─";
        let v_line = "│";
        let tl = "┌";
        let tr = "┐";
        let bl = "└";
        let br = "┘";

        // Верхняя граница
        self.move_cursor(x, y)?;
        self.draw_string(tl, x, y)?;
        for i in 1..width - 1 {
            self.draw_string(h_line, x + i, y)?;
        }
        self.draw_string(tr, x + width - 1, y)?;

        // Боковые границы
        for row in 1..height - 1 {
            self.draw_string(v_line, x, y + row)?;
            self.draw_string(v_line, x + width - 1, y + row)?;
        }

        // Нижняя граница
        self.move_cursor(x, y + height - 1)?;
        self.draw_string(bl, x, y + height - 1)?;
        for i in 1..width - 1 {
            self.draw_string(h_line, x + i, y + height - 1)?;
        }
        self.draw_string(br, x + width - 1, y + height - 1)?;

        Ok(())
    }
}

// Автоматически реализуем TerminalBackendExt для всех типов, реализующих TerminalBackend
impl<T: TerminalBackend> TerminalBackendExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Тестовая заглушка для TerminalBackend.
    struct MockTerminal {
        width: u16,
        height: u16,
        initialized: bool,
    }

    impl MockTerminal {
        fn new() -> Self {
            Self {
                width: 80,
                height: 24,
                initialized: false,
            }
        }
    }

    impl TerminalBackend for MockTerminal {
        fn init(&mut self) -> TerminalResult<()> {
            self.initialized = true;
            Ok(())
        }

        fn reset(&mut self) -> TerminalResult<()> {
            self.initialized = false;
            Ok(())
        }

        fn draw_char(&mut self, _ch: char, _x: u16, _y: u16) -> TerminalResult<()> {
            if !self.initialized {
                return Err(TerminalError::Initialization(
                    "Терминал не инициализирован".into(),
                ));
            }
            Ok(())
        }

        fn draw_string(&mut self, _text: &str, _x: u16, _y: u16) -> TerminalResult<()> {
            Ok(())
        }

        fn draw_lines(&mut self, _lines: &[&str], _x: u16, _y: u16) -> TerminalResult<()> {
            Ok(())
        }

        fn clear(&mut self) -> TerminalResult<()> {
            Ok(())
        }

        fn move_cursor(&mut self, _x: u16, _y: u16) -> TerminalResult<()> {
            Ok(())
        }

        fn hide_cursor(&mut self) -> TerminalResult<()> {
            Ok(())
        }

        fn show_cursor(&mut self) -> TerminalResult<()> {
            Ok(())
        }

        fn flush(&mut self) -> TerminalResult<()> {
            Ok(())
        }

        fn width(&self) -> Option<u16> {
            Some(self.width)
        }

        fn height(&self) -> Option<u16> {
            Some(self.height)
        }
    }

    #[test]
    fn test_mock_terminal_init() {
        let mut terminal = MockTerminal::new();
        assert!(!terminal.initialized);

        terminal.init().unwrap();
        assert!(terminal.initialized);

        terminal.reset().unwrap();
        assert!(!terminal.initialized);
    }

    #[test]
    fn test_mock_terminal_dimensions() {
        let terminal = MockTerminal::new();
        assert_eq!(terminal.width(), Some(80));
        assert_eq!(terminal.height(), Some(24));
    }

    #[test]
    fn test_terminal_error_display() {
        let io_err = TerminalError::Io(io::Error::other("test"));
        assert!(io_err.to_string().contains("Ошибка ввода/вывода"));

        let init_err = TerminalError::Initialization("test".into());
        assert!(init_err.to_string().contains("Ошибка инициализации"));

        let render_err = TerminalError::Render("test".into());
        assert!(render_err.to_string().contains("Ошибка отрисовки"));

        let unsupported_err = TerminalError::Unsupported("test".into());
        assert!(unsupported_err
            .to_string()
            .contains("Неподдерживаемая операция"));
    }
}
