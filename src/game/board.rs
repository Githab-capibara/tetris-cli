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

use crate::io::{GRID_HEIGHT, GRID_WIDTH};

/// Трейт для чтения состояния поля.
///
/// Предоставляет только чтение для доступа к состоянию поля.
pub trait BoardReadonly {
    /// Получить значение ячейки поля.
    ///
    /// # Аргументы
    /// * `x` - координата X (0..GRID_WIDTH)
    /// * `y` - координата Y (0..GRID_HEIGHT)
    ///
    /// # Возвращает
    /// - `Some(i8)` - значение ячейки (-1 = пусто, 0-6 = цвет)
    /// - `None` - если координаты выходят за пределы поля
    fn get_block(&self, x: usize, y: usize) -> Option<i8>;

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

    /// Получить ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Ссылка на двумерный массив [[i8; GRID_WIDTH]; GRID_HEIGHT]
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT];
}

/// Трейт для изменения состояния поля.
///
/// Предоставляет мутуабельный доступ для изменения поля.
pub trait BoardMutable: BoardReadonly {
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
    fn set_block(&mut self, x: usize, y: usize, value: i8) -> Option<()>;

    /// Очистить заполненные линии.
    ///
    /// # Возвращает
    /// Количество очищенных линий.
    fn clear_filled_lines(&mut self) -> u32;

    /// Установить битовую маску заполненных линий.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска заполненных линий
    fn set_filled_lines_mask(&mut self, mask: u32);

    /// Получить мутуабельную ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на [[i8; GRID_WIDTH]; GRID_HEIGHT]
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT];
}

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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn get_filled_lines_mask(&self) -> u32 {
        self.filled_lines
    }

    /// Установить битовую маску заполненных линий.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска заполненных линий
    #[inline]
    pub fn set_filled_lines_mask(&mut self, mask: u32) {
        self.filled_lines = mask;
    }

    /// Получить количество заполненных линий.
    ///
    /// # Возвращает
    /// Количество установленных битов в маске.
    #[inline]
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
    #[inline]
    pub fn clear_filled_lines(&mut self) -> u32 {
        let count = self.get_filled_lines_count();
        self.filled_lines = 0;
        count
    }

    /// Получить ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Ссылка на [[i8; GRID_WIDTH]; GRID_HEIGHT]
    #[inline]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить мутуабельную ссылку на внутренний массив поля.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на [[i8; GRID_WIDTH]; GRID_HEIGHT]
    #[inline]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }
}

impl BoardReadonly for GameBoard {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_block(&self, x: usize, y: usize) -> Option<i8> {
        self.get_block(x, y)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_filled_lines_mask(&self) -> u32 {
        self.get_filled_lines_mask()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_filled_lines_count(&self) -> u32 {
        self.get_filled_lines_count()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks()
    }
}

impl BoardMutable for GameBoard {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn set_block(&mut self, x: usize, y: usize, value: i8) -> Option<()> {
        self.set_block(x, y, value)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn clear_filled_lines(&mut self) -> u32 {
        self.clear_filled_lines()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn set_filled_lines_mask(&mut self, mask: u32) {
        self.set_filled_lines_mask(mask);
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.get_blocks_mut()
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
