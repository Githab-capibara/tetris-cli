//! Модуль игрового поля.
//!
//! # Ответственность
//! - Управление состоянием поля (blocks)
//! - Битовая маска заполненных линий
//! - Доступ к ячейкам поля
//!
//! # Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle (SRP).
//! `GameBoard` инкапсулирует состояние поля и предоставляет контролируемый доступ.

#![allow(dead_code)]

use crate::io::{GRID_HEIGHT, GRID_WIDTH};

// Переэкспорт трейтов доступа из access.rs для удобства
pub use super::access::{BoardMutable, BoardReadonly};

/// Состояние игрового поля.
///
/// Инкапсулирует состояние поля и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `blocks` - двумерный массив поля 10x20
/// - `filled_lines` - битовая маска заполненных линий
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `board: GameBoard`.
///
/// Архитектурное улучшение 2026-04-01: Добавлены Debug и Clone для совместимости.
#[derive(Debug, Clone)]
pub struct GameBoard {
    /// Двумерный массив игрового поля.
    /// Каждый элемент хранит индекс цвета (i8), -1 = пусто.
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Битовая маска заполненных линий.
    /// Каждый бит соответствует линии поля (бит 0 = линия 0).
    filled_lines: u32,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoard {
    /// Создать новое пустое игровое поле.
    ///
    /// # Возвращает
    /// Новый экземпляр `GameBoard` с пустым полем и нулевой маской.
    pub fn new() -> Self {
        Self {
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            filled_lines: 0,
        }
    }

    /// Получить значение ячейки поля.
    ///
    /// # Аргументы
    /// * `x` - координата X (0..GRID_WIDTH)
    /// * `y` - координата Y (0..GRID_HEIGHT)
    ///
    /// # Возвращает
    /// - `Some(i8)` - значение ячейки (-1 = пусто, 0-6 = цвет)
    /// - `None` - если координаты выходят за пределы поля
    pub fn get_block(&self, x: usize, y: usize) -> Option<i8> {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            Some(self.blocks[y][x])
        } else {
            None
        }
    }

    /// Установить значение ячейки поля.
    ///
    /// # Аргументы
    /// * `x` - координата X (0..GRID_WIDTH)
    /// * `y` - координата Y (0..GRID_HEIGHT)
    /// * `value` - значение (-1 = пусто, 0-6 = цвет)
    ///
    /// # Возвращает
    /// - `Some(())` - если ячейка успешно установлена
    /// - `None` - если координаты выходят за пределы поля
    pub fn set_block(&mut self, x: usize, y: usize, value: i8) -> Option<()> {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x] = value;
            Some(())
        } else {
            None
        }
    }

    /// Получить битовую маску заполненных линий.
    ///
    /// # Возвращает
    /// Битовая маска где каждый бит соответствует линии поля.
    pub fn get_filled_lines_mask(&self) -> u32 {
        self.filled_lines
    }

    /// Установить битовую маску заполненных линий.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска заполненных линий
    pub fn set_filled_lines_mask(&mut self, mask: u32) {
        self.filled_lines = mask;
    }

    /// Получить количество заполненных линий.
    ///
    /// # Возвращает
    /// Количество установленных битов в маске.
    pub fn get_filled_lines_count(&self) -> u32 {
        self.filled_lines.count_ones()
    }

    /// Очистить заполненные линии.
    ///
    /// # Возвращает
    /// Количество очищенных линий.
    ///
    /// # Примечания
    /// Этот метод только сбрасывает маску.
    /// Фактическое удаление линий должно выполняться отдельно.
    pub fn clear_filled_lines(&mut self) -> u32 {
        let count = self.get_filled_lines_count();
        self.filled_lines = 0;
        count
    }

    /// Получить ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Ссылка на [[i8; GRID_WIDTH]; GRID_HEIGHT]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить мутуабельную ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на [[i8; GRID_WIDTH]; GRID_HEIGHT]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }
}

impl BoardReadonly for GameBoard {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    fn get_block(&self, x: usize, y: usize) -> i8 {
        self.blocks[y][x]
    }

    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.blocks[y][x] == -1
    }

    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.blocks[y][x] != -1
    }

    fn get_filled_lines_mask(&self) -> u32 {
        self.filled_lines
    }

    fn get_filled_lines_count(&self) -> u32 {
        self.filled_lines.count_ones()
    }
}

impl BoardMutable for GameBoard {
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }

    fn set_block(&mut self, x: usize, y: usize, value: i8) {
        self.blocks[y][x] = value;
    }

    fn set_filled_lines_mask(&mut self, mask: u32) {
        self.filled_lines = mask;
    }

    fn clear_filled_lines(&mut self) -> u32 {
        let count = self.filled_lines.count_ones();
        self.filled_lines = 0;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_board_new() {
        let board = GameBoard::new();
        assert_eq!(board.get_filled_lines_mask(), 0);
        assert_eq!(board.get_filled_lines_count(), 0);
    }

    #[test]
    fn test_game_board_get_set_block() {
        let mut board = GameBoard::new();

        // Установка значения
        assert_eq!(board.set_block(5, 10, 3), Some(()));
        assert_eq!(board.get_block(5, 10), Some(3));

        // Проверка границ
        assert_eq!(board.set_block(GRID_WIDTH, 0, 1), None);
        assert_eq!(board.set_block(0, GRID_HEIGHT, 1), None);
        assert_eq!(board.get_block(GRID_WIDTH, 0), None);
        assert_eq!(board.get_block(0, GRID_HEIGHT), None);
    }

    #[test]
    fn test_game_board_filled_lines_mask() {
        let mut board = GameBoard::new();

        // Установка маски
        board.set_filled_lines_mask(0b1010);
        assert_eq!(board.get_filled_lines_mask(), 0b1010);
        assert_eq!(board.get_filled_lines_count(), 2);

        // Очистка
        assert_eq!(board.clear_filled_lines(), 2);
        assert_eq!(board.get_filled_lines_mask(), 0);
        assert_eq!(board.get_filled_lines_count(), 0);
    }

    #[test]
    fn test_game_board_blocks_access() {
        let mut board = GameBoard::new();

        // Заполнение линии
        for x in 0..GRID_WIDTH {
            board.set_block(x, 10, 1).unwrap();
        }

        // Проверка через get_blocks
        let blocks = board.get_blocks();
        for x in 0..GRID_WIDTH {
            assert_eq!(blocks[10][x], 1);
        }
    }
}
