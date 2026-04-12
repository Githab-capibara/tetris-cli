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
//!
//! ## Исправление аудита 2026-04-11 (Пакет 1, #12)
//! Глобальный `#![allow(dead_code)]` удалён. Каждый неиспользуемый элемент
//! помечается точечно через `#[allow(dead_code)]` при необходимости.

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};

/// Значение пустой ячейки игрового поля.
/// Используется как возвращаемое значение при выходе за границы.
const BLOCK_UNOCCUPIED: i8 = -1;

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
// PROB-119: трейты используются внутри crate (collision.rs, board.rs, scoreboard.rs),
// но clippy считает их unused при pub(crate) видимости модуля.
#[allow(dead_code)] // Публичный API трейтов для внутреннего использования crate
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
#[allow(dead_code)] // Трейт используется для полиморфного доступа к игровому полю
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
}

// ============================================================================
// РЕАЛИЗАЦИЯ ДЛЯ GameState
// ============================================================================

impl BoardReadonly for crate::game::state::GameState {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks()
    }

    /// Безопасный доступ: возвращает `BLOCK_UNOCCUPIED` при выходе за границы.
    fn get_block(&self, x: usize, y: usize) -> i8 {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(BLOCK_UNOCCUPIED)
    }

    /// Безопасная проверка: возвращает true при выходе за границы.
    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(BLOCK_UNOCCUPIED)
            == BLOCK_UNOCCUPIED
    }

    /// Безопасная проверка: возвращает false при выходе за границы.
    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_blocks()
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(BLOCK_UNOCCUPIED)
            != BLOCK_UNOCCUPIED
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
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod access_tests {
    use super::BoardMutable;
    use crate::game::state::GameState;

    /// Тест: проверка методов BoardMutable (clear_filled_lines)
    #[test]
    fn test_board_mutable_clear_lines() {
        let mut state = GameState::new();
        state.set_filled_lines(0b1111);
        let count = state.clear_filled_lines();
        assert_eq!(count, 4);
        assert_eq!(state.filled_lines(), 0);
    }
}
