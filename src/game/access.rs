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
//! - [`BoardReadonly`] - только чтение игрового поля
//! - [`BoardMutable`] - чтение и запись игрового поля
//! - [`ScoreAccess`] - доступ к очкам и уровням
//!
//! ## Пример использования
//! ```ignore
//! use crate::game::access::BoardReadonly;
//!
//! fn render_field<T: BoardReadonly>(field: &T) {
//!     for y in 0..GRID_HEIGHT {
//!         for x in 0..GRID_WIDTH {
//!             let block = field.get_block(x, y);
//!             // Отрисовка блока...
//!         }
//!     }
//! }
//! ```

use crate::io::{GRID_HEIGHT, GRID_WIDTH};

// ============================================================================
// ТРЕЙТ BOARDREADONLY (только чтение)
// ============================================================================

/// Трейт для доступа только на чтение к игровому полю.
///
/// Предоставляет методы для чтения игрового поля,
/// не раскрывая внутреннюю структуру [`GameState`].
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.9)
/// Этот трейт позволяет создавать функции, которые работают с любым типом,
/// реализующим [`BoardReadonly`], что уменьшает связанность кода.
///
/// ## Пример использования
/// ```ignore
/// fn render_field<T: BoardReadonly>(field: &T) {
///     for y in 0..GRID_HEIGHT {
///         for x in 0..GRID_WIDTH {
///             let block = field.get_block(x, y);
///             // Отрисовка блока...
///         }
///     }
/// }
/// ```
pub trait BoardReadonly {
    /// Получить доступ к игровому полю (только чтение).
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT];

    /// Получить значение ячейки игрового поля.
    fn get_block(&self, x: usize, y: usize) -> i8;

    /// Проверить, пуста ли ячейка.
    fn is_block_empty(&self, x: usize, y: usize) -> bool;

    /// Проверить, занята ли ячейка.
    fn is_block_occupied(&self, x: usize, y: usize) -> bool;

    /// Получить битовую маску заполненных линий.
    ///
    /// # Возвращает
    /// Битовая маска где каждый бит соответствует линии поля.
    /// Бит установлен в 1 если линия заполнена.
    fn get_filled_lines_mask(&self) -> u32;

    /// Получить количество заполненных линий.
    ///
    /// # Возвращает
    /// Количество линий, которые необходимо удалить.
    fn get_filled_lines_count(&self) -> u32;
}

// ============================================================================
// ТРЕЙТ BOARDMUTABLE (чтение и запись)
// ============================================================================

/// Трейт для доступа на чтение и запись к игровому полю.
///
/// Предоставляет методы для чтения и записи игрового поля,
/// не раскрывая внутреннюю структуру [`GameState`].
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.9)
/// Этот трейт расширяет [`BoardReadonly`] методами для изменения поля.
///
/// ## Пример использования
/// ```ignore
/// fn place_piece<T: BoardMutable>(field: &mut T, x: usize, y: usize, value: i8) {
///     field.set_block(x, y, value);
/// }
/// ```
pub trait BoardMutable: BoardReadonly {
    /// Получить доступ к игровому полю (мутабельный).
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT];

    /// Установить значение ячейки игрового поля.
    fn set_block(&mut self, x: usize, y: usize, value: i8);

    /// Установить битовую маску заполненных линий.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска заполненных линий
    fn set_filled_lines_mask(&mut self, mask: u32);

    /// Очистить заполненные линии.
    ///
    /// # Возвращает
    /// Количество очищенных линий.
    fn clear_filled_lines(&mut self) -> u32;

    /// Получить скорость падения (по умолчанию 0.0).
    ///
    /// Этот метод имеет реализацию по умолчанию для типов которые не поддерживают скорость падения.
    fn get_fall_speed(&self) -> f32 {
        0.0
    }

    /// Установить скорость падения (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::game::state::GameError`] если значение невалидно.
    fn set_fall_speed(&mut self, _spd: f32) -> Result<(), crate::game::state::GameError> {
        Err(crate::game::state::GameError::Validation(
            "Этот тип не поддерживает установку скорости падения".to_string(),
        ))
    }

    /// Получить таймер приземления (по умолчанию 0.0).
    fn get_land_timer(&self) -> f64 {
        0.0
    }

    /// Установить таймер приземления (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::game::state::GameError`] если значение невалидно.
    fn set_land_timer(&mut self, _timer: f64) -> Result<(), crate::game::state::GameError> {
        Err(crate::game::state::GameError::Validation(
            "Этот тип не поддерживает установку таймера приземления".to_string(),
        ))
    }
}

// ============================================================================
// ТРЕЙТ SCOREACCESS (очки и уровни)
// ============================================================================

/// Трейт для доступа к очкам и уровням.
///
/// Предоставляет методы для чтения и изменения очков, уровней и линий,
/// не раскрывая внутреннюю структуру [`GameState`].
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.3, 2.9)
/// Этот трейт выделяет доступ к системе очков в отдельный интерфейс,
/// что позволяет создавать функции, работающие только с очками,
/// без доступа к игровому полю.
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

// ============================================================================
// ТРЕЙТ GAMEBOARDACCESS (объединённый - для обратной совместимости)
// ============================================================================

/// Трейт для полного доступа к игровому полю.
///
/// Объединяет [`BoardMutable`] и [`ScoreAccess`] для обратной совместимости.
///
/// ## Архитектурные заметки
/// ## Обратная совместимость
/// Этот трейт сохранён для обратной совместимости.
/// Для нового кода рекомендуется использовать специализированные трейты:
/// - [`BoardReadonly`] для чтения
/// - [`BoardMutable`] для чтения и записи
/// - [`ScoreAccess`] для очков
pub trait GameBoardAccess: BoardMutable + ScoreAccess {}

// Реализация GameBoardAccess для всех типов, реализующих [`BoardMutable`] и [`ScoreAccess`]
impl<T: BoardMutable + ScoreAccess> GameBoardAccess for T {}

// ============================================================================
// РЕАЛИЗАЦИЯ ДЛЯ GameState
// ============================================================================

impl BoardReadonly for crate::game::state::GameState {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks()
    }

    fn get_block(&self, x: usize, y: usize) -> i8 {
        self.get_blocks()[y][x]
    }

    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_blocks()[y][x] == -1
    }

    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_blocks()[y][x] != -1
    }

    fn get_filled_lines_mask(&self) -> u32 {
        self.filled_lines()
    }

    fn get_filled_lines_count(&self) -> u32 {
        self.filled_lines().count_ones()
    }
}

impl BoardMutable for crate::game::state::GameState {
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks_mut()
    }

    fn set_block(&mut self, x: usize, y: usize, value: i8) {
        self.get_blocks_mut()[y][x] = value;
    }

    fn set_filled_lines_mask(&mut self, mask: u32) {
        self.set_filled_lines(mask);
    }

    fn clear_filled_lines(&mut self) -> u32 {
        let count = self.filled_lines().count_ones();
        self.set_filled_lines(0);
        count
    }

    fn get_fall_speed(&self) -> f32 {
        self.fall_speed()
    }

    fn set_fall_speed(&mut self, spd: f32) -> Result<(), crate::game::state::GameError> {
        self.set_fall_speed(spd)
    }

    fn get_land_timer(&self) -> f64 {
        self.land_timer()
    }

    fn set_land_timer(&mut self, timer: f64) -> Result<(), crate::game::state::GameError> {
        self.set_land_timer(timer)
    }
}

impl ScoreAccess for crate::game::state::GameState {
    fn get_score(&self) -> u128 {
        self.score()
    }

    fn add_score(&mut self, points: u128) {
        self.add_score(points);
    }

    fn set_score(&mut self, score: u128) {
        self.set_score(score);
    }

    fn get_level(&self) -> u32 {
        self.level()
    }

    fn set_level(&mut self, level: u32) {
        self.set_level(level);
    }

    fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared()
    }

    fn set_lines_cleared(&mut self, lines: u32) {
        self.set_lines_cleared(lines);
    }
}
