//! Трейты доступа к состоянию игры.
//!
//! Этот модуль содержит трейты для предоставления контролируемого доступа
//! к внутреннему состоянию игры без нарушения инкапсуляции.
//!
//! ## Архитектурные заметки
//! ## Трейты доступа (Problem 2.9, 2.12, 2.14)
//! Трейты используются для:
//! - Уменьшения связанности между модулями
//! - Предоставления ограниченного доступа к данным
//! - Упрощения тестирования через моки
//!
//! ## Доступные трейты:
//! - [`GameBoardAccess`] - полный доступ к состоянию игры
//!
//! TODO (#архитектура, Problem 2.9): Добавить трейт ScoreAccess для контролируемого доступа к очкам.
//! TODO (#архитектура, Problem 2.12): Добавить трейт ShapeAccess для доступа к фигурам.
//! TODO (#архитектура, Problem 2.14): Рассмотреть возможность добавления трейта AnimationAccess
//! для доступа к анимациям.
//!
//! ## Пример использования
//! ```ignore
//! use crate::game::access::GameBoardAccess;
//!
//! fn render_field<T: GameBoardAccess>(field: &T) {
//!     for y in 0..GRID_HEIGHT {
//!         for x in 0..GRID_WIDTH {
//!             let block = field.get_block(x, y);
//!             // Отрисовка блока...
//!         }
//!     }
//! }
//! ```

use crate::io::{GRID_HEIGHT, GRID_WIDTH};

/// Трейт для доступа к игровому полю.
///
/// Предоставляет методы для чтения и записи игрового поля,
/// не раскрывая внутреннюю структуру `GameState`.
///
/// ## Архитектурные заметки
/// ## Использование трейта (Problem 2.9)
/// Этот трейт позволяет создавать функции, которые работают с любым типом,
/// реализующим GameBoardAccess, что уменьшает связанность кода.
///
/// TODO (#архитектура): Добавить методы для доступа к призрачной фигуре
/// и другим производным данным.
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

// ============================================================================
// РЕАЛИЗАЦИЯ GameBoardAccess ДЛЯ GameState
// ============================================================================

impl GameBoardAccess for crate::game::state::GameState {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks()
    }

    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks_mut()
    }

    fn get_block(&self, x: usize, y: usize) -> i8 {
        self.blocks[y][x]
    }

    fn set_block(&mut self, x: usize, y: usize, value: i8) {
        self.blocks[y][x] = value;
    }

    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.blocks[y][x] == -1
    }

    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.blocks[y][x] != -1
    }

    fn get_score(&self) -> u128 {
        self.get_score()
    }

    fn add_score(&mut self, points: u128) {
        self.add_score(points);
    }

    fn get_level(&self) -> u32 {
        self.get_level()
    }

    fn set_level(&mut self, level: u32) {
        self.set_level(level);
    }

    fn get_lines_cleared(&self) -> u32 {
        self.get_lines_cleared()
    }

    fn set_lines_cleared(&mut self, lines: u32) {
        self.set_lines_cleared(lines);
    }

    fn get_fall_spd(&self) -> f32 {
        self.get_fall_spd()
    }

    fn set_fall_spd(&mut self, spd: f32) {
        self.set_fall_spd(spd);
    }

    fn get_land_timer(&self) -> f64 {
        self.get_land_timer()
    }

    fn set_land_timer(&mut self, timer: f64) {
        self.set_land_timer(timer);
    }
}

// ============================================================================
// ТРЕЙТ SCOREACCESS
// ============================================================================

/// Трейт для доступа к очкам и уровням.
///
/// Предоставляет методы для чтения и изменения очков, уровней и линий,
/// не раскрывая внутреннюю структуру `GameState`.
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.3, 2.9)
/// Этот трейт выделяет доступ к системе очков в отдельный интерфейс,
/// что позволяет создавать функции, работающие только с очками,
/// без доступа к игровому полю.
///
/// TODO (#архитектура): Использовать этот трейт в функциях начисления очков
/// вместо прямого использования GameState.
///
/// ## Пример использования
/// ```ignore
/// use crate::game::access::ScoreAccess;
///
/// fn add_line_bonus<T: ScoreAccess>(score: &mut T, lines: u32) {
///     let bonus = lines * 100;
///     score.add_score(bonus);
/// }
/// ```
#[allow(dead_code)]
pub trait ScoreAccess {
    /// Получить текущий счёт.
    fn get_score(&self) -> u128;

    /// Добавить очки к текущему счёту.
    fn add_score(&mut self, points: u128);

    /// Установить счёт (для тестов).
    fn set_score(&mut self, score: u128);

    /// Получить текущий уровень.
    fn get_level(&self) -> u32;

    /// Установить текущий уровень.
    fn set_level(&mut self, level: u32);

    /// Получить количество удалённых линий.
    fn get_lines_cleared(&self) -> u32;

    /// Установить количество удалённых линий.
    fn set_lines_cleared(&mut self, lines: u32);
}

// Реализация ScoreAccess для GameState
impl ScoreAccess for crate::game::state::GameState {
    fn get_score(&self) -> u128 {
        self.score
    }

    fn add_score(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    fn set_score(&mut self, score: u128) {
        self.score = score;
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    fn set_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = lines;
    }
}
