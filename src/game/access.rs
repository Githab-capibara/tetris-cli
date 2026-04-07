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

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};

// ============================================================================
// ТРЕЙТ BOARDREADONLY (только чтение)
// ============================================================================

/// Трейт для доступа только на чтение к игровому полю.
///
/// Предоставляет методы для чтения игрового поля,
/// не раскрывая внутреннюю структуру [`GameState`](crate::game::state::GameState).
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
// PROB-119: трейты используются внутри crate (collision.rs, board.rs),
// но clippy считает их unused при pub(crate) видимости модуля.
#[allow(dead_code)]
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
/// не раскрывая внутреннюю структуру [`GameState`](crate::game::state::GameState).
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
// PROB-119: трейт используется внутри crate (board.rs, GameState impl)
#[allow(dead_code)]
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
    ///
    /// # Почему возврат 0.0 допустим
    /// Трейт `BoardMutable` предназначен для различных реализаций, не все из которых
    /// поддерживают физику падения. Например, `GameView` реализует `BoardMutable` для
    /// тестирования, но не имеет физической модели. Возврат 0.0 — безопасное значение
    /// по умолчанию, которое не влияет на логику неигровых реализаций.
    #[must_use]
    #[inline]
    fn get_fall_speed(&self) -> f32 {
        0.0
    }

    /// Установить скорость падения (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError`] если значение невалидно.
    ///
    /// # Почему возврат ошибки допустим
    /// Реализации по умолчанию для трейтов, не поддерживающих физику, возвращают ошибку.
    /// Это позволяет иметь единый интерфейс для всех типов, при этом конкретные реализации
    /// (например, `GameState`) переопределяют метод с реальной логикой.
    /// Для `GameView` и других неигровых типов ошибка сигнализирует о том, что операция
    /// не применима — это ожидаемое поведение, а не баг.
    #[must_use = "Ошибка установки скорости должна быть обработана"]
    fn set_fall_speed(&mut self, _spd: f32) -> Result<(), crate::errors::GameError> {
        Err(crate::errors::GameError::ValidationError(
            "Этот тип не поддерживает установку скорости падения".to_string(),
        ))
    }

    /// Получить таймер приземления (по умолчанию 0.0).
    ///
    /// # Почему возврат 0.0 допустим
    /// Аналогично `get_fall_speed()` — не все реализации трейта поддерживают таймер.
    /// Возврат 0.0 — безопасное значение по умолчанию.
    #[must_use]
    #[inline]
    fn get_land_timer(&self) -> f64 {
        0.0
    }

    /// Установить таймер приземления (по умолчанию возвращает ошибку).
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError`] если значение невалидно.
    ///
    /// # Почему возврат ошибки допустим
    /// Аналогично `set_fall_speed()` — не все реализации трейта поддерживают таймер.
    /// Ошибка сигнализирует что операция не применима для данного типа.
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
/// не раскрывая внутреннюю структуру [`GameState`](crate::game::state::GameState).
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
        // Исправление S4: проверка границ перед записью для безопасности
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.get_blocks_mut()[y][x] = value;
        }
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
        // add_score возвращает u128 (новый счёт), а не Result — saturating_add защищает от переполнения
        let _ = self.scoreboard_mut().add_score(points);
    }

    fn set_score(&mut self, score: u128) {
        self.scoreboard_mut().set_score(score);
    }
}
