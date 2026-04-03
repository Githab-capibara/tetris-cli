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
/// # Реализации
/// Этот трейт реализован для следующих типов:
/// | Тип | Описание |
/// |-----|----------|
/// | [`GameState`](crate::game::state::GameState) | Основное состояние игры |
/// | [`GameView<'a>`](crate::game::view::GameView) | Представление игры для отрисовки |
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
/// # Реализации
/// Этот трейт реализован для следующих типов:
/// | Тип | Описание |
/// |-----|----------|
/// | [`GameState`](crate::game::state::GameState) | Основное состояние игры (единственная реализация) |
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
    #[must_use]
    #[inline]
    fn get_fall_speed(&self) -> f32 {
        0.0
    }

    /// Установить скорость падения (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError`] если значение невалидно.
    #[must_use = "Ошибка установки скорости должна быть обработана"]
    fn set_fall_speed(&mut self, _spd: f32) -> Result<(), crate::errors::GameError> {
        Err(crate::errors::GameError::ValidationError(
            "Этот тип не поддерживает установку скорости падения".to_string(),
        ))
    }

    /// Получить таймер приземления (по умолчанию 0.0).
    #[must_use]
    #[inline]
    fn get_land_timer(&self) -> f64 {
        0.0
    }

    /// Установить таймер приземления (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError`] если значение невалидно.
    #[must_use = "Ошибка установки таймера должна быть обработана"]
    fn set_land_timer(&mut self, _timer: f64) -> Result<(), crate::errors::GameError> {
        Err(crate::errors::GameError::ValidationError(
            "Этот тип не поддерживает установку таймера приземления".to_string(),
        ))
    }
}

// ============================================================================
// ТРЕЙТ SCOREACCESS (только чтение)
// ============================================================================

/// Трейт для доступа только на чтение к очкам и уровням.
///
/// Предоставляет методы для чтения очков, уровней и линий,
/// не раскрывая внутреннюю структуру [`GameState`].
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.3, 2.9, ISP)
/// Этот трейт выделяет доступ только для чтения к системе очков в отдельный интерфейс,
/// что позволяет создавать функции, работающие только с чтением очков,
/// без возможности их изменения. Соответствует Interface Segregation Principle.
///
/// ## Пример использования
/// ```ignore
/// use crate::game::access::ScoreAccess;
///
/// fn display_score<T: ScoreAccess>(score: &T) {
///     println!("Счёт: {}", score.get_score());
/// }
/// ```
pub trait ScoreAccess {
    /// Получить текущий счёт.
    ///
    /// # Реализация по умолчанию
    /// Возвращает 0; переопределите для реальных типов.
    /// Этот метод должен быть переопределён конкретными реализациями.
    #[must_use]
    #[inline]
    fn get_score(&self) -> u128 {
        0
    }
}

// ============================================================================
// ТРЕЙТ SCOREMUTABLE (чтение и запись)
// ============================================================================

/// Трейт для доступа на чтение и запись к очкам и уровням.
///
/// Расширяет [`ScoreAccess`] методами для изменения очков.
///
/// ## Архитектурные заметки
/// ## Разделение ответственности (Problem 2.3, 2.9, ISP)
/// Этот трейт расширяет [`ScoreAccess`] методами для изменения состояния.
/// Разделение на `ScoreAccess` (чтение) и `ScoreMutable` (запись) позволяет
/// следовать Interface Segregation Principle и предоставлять минимально
/// необходимый интерфейс для каждой функции.
///
/// ## Пример использования
/// ```ignore
/// use crate::game::access::ScoreMutable;
///
/// fn add_line_bonus<T: ScoreMutable>(score: &mut T, lines: u32) {
///     let bonus = lines * 100;
///     score.add_score(bonus);
/// }
/// ```
pub trait ScoreMutable: ScoreAccess {
    /// Добавить очки к текущему счёту.
    fn add_score(&mut self, points: u128);

    /// Установить счёт (для тестов).
    fn set_score(&mut self, score: u128);
}

// ============================================================================
// ISP-1: УЗКИЕ ТРЕЙТЫ ДЛЯ SCORING (INTERFACE SEGREGATION PRINCIPLE)
// ============================================================================

/// Трейт для доступа к уровням.
///
/// # ISP-1: Узкий интерфейс
/// Предоставляет только методы для работы с уровнями.
///
/// ## Методы
/// - `get_level()` — получить текущий уровень (по умолчанию возвращает 0)
/// - `set_level()` — установить новый уровень
///
/// ## Архитектурные заметки
/// Выделен для соблюдения Interface Segregation Principle.
/// Для доступа из других модулей используйте `crate::game::access::LevelAccess`.
pub trait LevelAccess {
    /// Получить текущий уровень.
    #[must_use]
    #[inline]
    fn get_level(&self) -> u32 {
        0
    }

    /// Установить текущий уровень.
    fn set_level(&mut self, level: u32);
}

/// Трейт для доступа к линиям.
///
/// # ISP-1: Узкий интерфейс
/// Предоставляет только методы для работы с линиями.
///
/// ## Методы
/// - `get_lines_cleared()` — получить количество очищенных линий (по умолчанию возвращает 0)
/// - `set_lines_cleared()` — установить количество очищенных линий
/// - `add_lines()` — добавить количество очищенных линий
///
/// ## Архитектурные заметки
/// Выделен для соблюдения Interface Segregation Principle.
/// Для доступа из других модулей используйте `crate::game::access::LinesAccess`.
pub trait LinesAccess {
    /// Получить количество очищенных линий.
    #[must_use]
    #[inline]
    fn get_lines_cleared(&self) -> u32 {
        0
    }

    /// Установить количество очищенных линий.
    fn set_lines_cleared(&mut self, lines: u32);

    /// Добавить количество очищенных линий.
    fn add_lines(&mut self, lines: u32);
}

/// Трейт для доступа к комбо.
///
/// # ISP-1: Узкий интерфейс
/// Предоставляет только методы для работы с комбо.
///
/// ## Методы
/// - `get_combo()` — получить текущий комбо (по умолчанию возвращает 0)
/// - `set_combo()` — установить текущий комбо
/// - `reset_combo()` — сбросить счётчик комбо
///
/// ## Архитектурные заметки
/// Выделен для соблюдения Interface Segregation Principle.
/// Для доступа из других модулей используйте `crate::game::access::ComboAccess`.
pub trait ComboAccess {
    /// Получить текущий комбо.
    #[must_use]
    #[inline]
    fn get_combo(&self) -> u32 {
        0
    }

    /// Установить текущий комбо.
    fn set_combo(&mut self, combo: u32);

    /// Сбросить комбо.
    fn reset_combo(&mut self);
}

// ============================================================================
// РЕАЛИЗАЦИЯ ДЛЯ GameState
// ============================================================================

impl BoardReadonly for crate::game::state::GameState {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks()
    }

    /// Безопасный доступ: возвращает -1 при выходе за границы.
    fn get_block(&self, x: usize, y: usize) -> i8 {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(-1)
    }

    /// Безопасная проверка: возвращает true при выходе за границы.
    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(-1)
            == -1
    }

    /// Безопасная проверка: возвращает false при выходе за границы.
    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(-1)
            != -1
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

    fn set_fall_speed(&mut self, spd: f32) -> Result<(), crate::errors::GameError> {
        self.set_fall_speed(spd)
    }

    fn get_land_timer(&self) -> f64 {
        self.land_timer()
    }

    fn set_land_timer(&mut self, timer: f64) -> Result<(), crate::errors::GameError> {
        self.set_land_timer(timer)
    }
}

impl ScoreAccess for crate::game::state::GameState {
    #[inline]
    fn get_score(&self) -> u128 {
        self.score()
    }
}

impl ScoreMutable for crate::game::state::GameState {
    fn add_score(&mut self, points: u128) {
        let _ = self.scoreboard_mut().add_score(points);
    }

    fn set_score(&mut self, score: u128) {
        self.scoreboard_mut().set_score(score);
    }
}

// ============================================================================
// РЕАЛИЗАЦИЯ ISP-1 ТРЕЙТОВ ДЛЯ GameState
// ============================================================================

impl LevelAccess for crate::game::state::GameState {
    #[inline]
    fn get_level(&self) -> u32 {
        self.level()
    }

    #[inline]
    fn set_level(&mut self, level: u32) {
        let () = self.set_level(level);
    }
}

impl LinesAccess for crate::game::state::GameState {
    #[inline]
    fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared()
    }

    #[inline]
    fn set_lines_cleared(&mut self, lines: u32) {
        let () = self.set_lines_cleared(lines);
    }

    #[inline]
    fn add_lines(&mut self, lines: u32) {
        let _ = self.add_lines_cleared(lines);
    }
}

impl ComboAccess for crate::game::state::GameState {
    #[inline]
    fn get_combo(&self) -> u32 {
        self.stats().combo_counter()
    }

    #[inline]
    fn set_combo(&mut self, combo: u32) {
        let () = self.stats_mut().set_combo_counter(combo);
    }

    #[inline]
    fn reset_combo(&mut self) {
        let () = self.stats_mut().set_combo_counter(0);
    }
}
