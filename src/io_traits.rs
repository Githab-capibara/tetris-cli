//! Трейты для ввода и вывода.
//!
//! Этот модуль предоставляет абстракции для работы с вводом и выводом:
//! - [`InputReader`] — трейт для чтения ввода пользователя
//! - [`Renderer`] — трейт для отрисовки в терминале
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::io_traits::{InputReader, Renderer};
//! use tetris_cli::io::{KeyReader, Canvas};
//! use termion::color::{White, Reset};
//!
//! let mut reader: &mut dyn InputReader = &mut KeyReader::new();
//! let mut renderer: &mut dyn Renderer = &mut Canvas::new().unwrap();
//!
//! // Чтение клавиши
//! if let Ok(Some(key)) = reader.get_key() {
//!     println!("Нажата клавиша: {}", key);
//! }
//!
//! // Отрисовка
//! renderer.draw_string("Привет!", (1, 1), &White, &Reset);
//! renderer.flush();
//! ```

#![allow(dead_code)]

use std::io;
use termion::color::Color;

/// Трейт для чтения ввода пользователя.
///
/// Предоставляет абстракцию для чтения нажатий клавиш.
/// Может быть реализован для различных источников ввода.
///
/// ## Пример реализации
/// ```ignore
/// use tetris_cli::io_traits::InputReader;
/// use tetris_cli::io::KeyReader;
/// use std::io;
///
/// impl InputReader for KeyReader {
///     fn get_key(&mut self) -> io::Result<Option<u8>> {
///         self.get_key()
///     }
/// }
/// ```
pub trait InputReader {
    /// Получить код нажатой клавиши.
    ///
    /// # Возвращает
    /// - `Ok(Some(u8))` — код нажатой клавиши
    /// - `Ok(None)` — если клавиша не была нажата
    /// - `Err(io::Error)` — если произошла ошибка чтения
    ///
    /// # Errors
    /// Возвращает `io::Error` при ошибке чтения из терминала (например, при закрытии stdin).
    ///
    /// # Пример
    /// ```ignore
    /// let mut reader: &mut dyn InputReader = &mut KeyReader::new();
    /// if let Ok(Some(key)) = reader.get_key() {
    ///     match key {
    ///         b'q' => println!("Выход"),
    ///         b'p' => println!("Пауза"),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    fn get_key(&mut self) -> io::Result<Option<u8>>;
}

/// Трейт для отрисовки в терминале.
///
/// Предоставляет абстракцию для отрисовки текста и графики.
/// Может быть реализован для различных устройств вывода.
///
/// # D4: Документирование трейта Renderer
/// Этот трейт определяет интерфейс для рендеринга в терминале:
/// - `draw_strs()` - отрисовка массива строк
/// - `draw_string()` - отрисовка одиночной строки
/// - `flush()` - обновление экрана (применение изменений)
/// - `reset()` - сброс терминала в исходное состояние
///
/// ## Архитектурное назначение
/// - **Abstraction**: отделяет логику отрисовки от конкретной реализации терминала
/// - **Testability**: позволяет создавать mock-реализации для тестирования
/// - **Flexibility**: поддержка различных бэкендов (termion, crossterm, и т.д.)
///
/// ## Пример реализации
/// ```ignore
/// use tetris_cli::io_traits::Renderer;
/// use tetris_cli::io::Canvas;
/// use termion::color::{White, Reset};
///
/// impl Renderer for Canvas {
///     fn draw_strs(&mut self, strings: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
///         self.draw_strs(strings, pos, fg, bg);
///     }
///
///     fn draw_string(&mut self, string: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color) {
///         self.draw_string(string, pos, fg, bg);
///     }
///
///     fn flush(&mut self) {
///         self.flush();
///     }
///
///     fn reset(&mut self) {
///         self.reset();
///     }
/// }
/// ```
pub trait Renderer {
    /// Отрисовать строки.
    ///
    /// # Аргументы
    /// * `strings` — массив строк для отрисовки
    /// * `pos` — позиция верхней левой строки (x, y)
    /// * `fg` — цвет переднего плана
    /// * `bg` — цвет фона
    ///
    /// # Пример
    /// ```ignore
    /// let mut renderer: &mut dyn Renderer = &mut Canvas::new().unwrap();
    /// renderer.draw_strs(&["Строка 1", "Строка 2"], (1, 1), &White, &Reset);
    /// ```
    fn draw_strs(&mut self, strings: &[&str], pos: (u16, u16), fg: &dyn Color, bg: &dyn Color);

    /// Отрисовать строку.
    ///
    /// # Аргументы
    /// * `string` — текст для отрисовки
    /// * `pos` — позиция верхнего левого угла (x, y)
    /// * `fg` — цвет переднего плана
    /// * `bg` — цвет фона
    ///
    /// # Пример
    /// ```ignore
    /// let mut renderer: &mut dyn Renderer = &mut Canvas::new().unwrap();
    /// renderer.draw_string("Привет, Мир!", (5, 2), &White, &Reset);
    /// ```
    fn draw_string(&mut self, string: &str, pos: (u16, u16), fg: &dyn Color, bg: &dyn Color);

    /// Обновить экран (flush).
    ///
    /// # Важность
    /// Вызывайте этот метод после всех операций отрисовки,
    /// чтобы изменения появились на экране.
    ///
    /// # Пример
    /// ```ignore
    /// let mut renderer: &mut dyn Renderer = &mut Canvas::new().unwrap();
    /// renderer.draw_string("Текст", (1, 1), &White, &Reset);
    /// renderer.flush();
    /// ```
    fn flush(&mut self);

    /// Сбросить терминал в исходное состояние.
    ///
    /// # Важность
    /// Обязательно вызывайте этот метод перед завершением программы,
    /// чтобы вернуть терминал в нормальное состояние.
    ///
    /// # Пример
    /// ```ignore
    /// let mut renderer: &mut dyn Renderer = &mut Canvas::new().unwrap();
    /// // ... игра ...
    /// renderer.reset();
    /// ```
    fn reset(&mut self);
}
