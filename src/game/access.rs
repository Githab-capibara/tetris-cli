//! Трейты доступа к состоянию игры.
//!
//! Этот модуль содержит трейты для предоставления контролируемого доступа
//! к внутреннему состоянию игры без нарушения инкапсуляции.
//!
//! ## Архитектурные заметки
//! Трейты используются для:
//! - Уменьшения связанности между модулями
//! - Предоставления ограниченного доступа к данным
//! - Упрощения тестирования через моки
//!
//! ## Доступные трейты:
//! - [`GameBoardAccess`] - полный доступ к состоянию игры

use crate::io::{GRID_HEIGHT, GRID_WIDTH};

/// Трейт для доступа к игровому полю.
///
/// Предоставляет методы для чтения и записи игрового поля,
/// не раскрывая внутреннюю структуру `GameState`.
///
/// ## Пример использования
/// ```ignore
/// fn render_field<T: GameBoardAccess>(field: &T) {
///     for y in 0..GRID_HEIGHT {
///         for x in 0..GRID_WIDTH {
///             let block = field.get_block(x, y);
///             // Отрисовка блока...
///         }
///     }
/// }
/// ```
pub trait GameBoardAccess {
    /// Получить доступ к игровому полю (только чтение).
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT];

    /// Получить доступ к игровому полю (мутабельный).
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT];

    /// Получить значение ячейки игрового поля.
    fn get_block(&self, x: usize, y: usize) -> i8;

    /// Установить значение ячейки игрового поля.
    fn set_block(&mut self, x: usize, y: usize, value: i8);

    /// Проверить, пуста ли ячейка.
    fn is_block_empty(&self, x: usize, y: usize) -> bool;

    /// Проверить, занята ли ячейка.
    fn is_block_occupied(&self, x: usize, y: usize) -> bool;

    /// Получить текущий счёт.
    fn get_score(&self) -> u128;

    /// Добавить очки к текущему счёту.
    fn add_score(&mut self, points: u128);

    /// Получить текущий уровень.
    fn get_level(&self) -> u32;

    /// Установить текущий уровень.
    fn set_level(&mut self, level: u32);

    /// Получить количество удалённых линий.
    fn get_lines_cleared(&self) -> u32;

    /// Установить количество удалённых линий.
    fn set_lines_cleared(&mut self, lines: u32);

    /// Получить скорость падения.
    fn get_fall_spd(&self) -> f32;

    /// Установить скорость падения.
    fn set_fall_spd(&mut self, spd: f32);

    /// Получить таймер приземления.
    fn get_land_timer(&self) -> f64;

    /// Установить таймер приземления.
    fn set_land_timer(&mut self, timer: f64);
}
