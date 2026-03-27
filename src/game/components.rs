//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные структуры для разделения ответственности
//! внутри `GameState`. Такое разделение улучшает инкапсуляцию и упрощает тестирование.
//!
//! ## Структуры
//! - [`GameBoard`] — состояние игрового поля
//! - [`ScoreBoard`] — состояние очков и уровней
//! - [`FigureManager`] — управление фигурами
//! - [`AnimationState`] — состояние анимаций
//!
//! ## Архитектурные заметки
//! ## Разделение GameState (Problem 2.1, C1, H1, H4, H6, M10)
//! GameState был разделён на специализированные структуры для:
//! - Улучшения инкапсуляции
//! - Уменьшения связанности
//! - Упрощения тестирования
//! - Поддержки обратной совместимости через геттеры/сеттеры

use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, Tetromino};

// ============================================================================
// GAMEBOARD — СОСТОЯНИЕ ПОЛЯ
// ============================================================================

/// Состояние игрового поля.
///
/// Отвечает за хранение и управление игровым полем:
/// - Двумерный массив блоков
/// - Битовая маска заполненных линий
///
/// ## Архитектурные заметки
/// Выделено из GameState для улучшения инкапсуляции.
/// Все поля pub(crate) для доступа из модуля game.
#[derive(Clone)]
pub struct GameBoard {
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), -1 = пусто.
    pub(crate) blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Битовая маска заполненных линий (для оптимизации).
    pub(crate) filled_lines: u32,
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoard {
    /// Создать новое игровое поле.
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            filled_lines: 0,
        }
    }

    /// Получить игровое поле (только чтение).
    #[must_use]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить игровое поле (мутуабельная ссылка).
    #[must_use]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }

    /// Получить значение ячейки.
    #[must_use]
    pub fn get_block(&self, x: usize, y: usize) -> i8 {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x]
        } else {
            -1
        }
    }

    /// Установить значение ячейки.
    pub fn set_block(&mut self, x: usize, y: usize, value: i8) {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x] = value;
        }
    }

    /// Проверить, пуста ли ячейка.
    #[must_use]
    pub fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) == -1
    }

    /// Проверить, занята ли ячейка.
    #[must_use]
    pub fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) >= 0
    }

    /// Получить битовую маску заполненных линий.
    #[must_use]
    pub fn get_filled_lines_mask(&self) -> u32 {
        self.filled_lines
    }

    /// Установить битовую маску заполненных линий.
    pub fn set_filled_lines_mask(&mut self, mask: u32) {
        self.filled_lines = mask;
    }
}

// ============================================================================
// SCOREBOARD — СОСТОЯНИЕ ОЧКОВ
// ============================================================================

/// Состояние очков и уровней.
///
/// Отвечает за хранение и управление очками:
/// - Текущий счёт
/// - Текущий уровень
/// - Количество очищенных линий
///
/// ## Архитектурные заметки
/// Выделено из GameState для улучшения инкапсуляции.
/// Предоставляет контролируемый доступ к очкам через методы.
#[derive(Clone)]
pub struct ScoreBoard {
    /// Текущий счёт.
    pub(crate) score: u128,
    /// Текущий уровень.
    pub(crate) level: u32,
    /// Количество удалённых линий.
    pub(crate) lines_cleared: u32,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoreBoard {
    /// Создать новую доску очков.
    #[must_use]
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }

    /// Получить текущий счёт.
    #[must_use]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Установить счёт.
    pub fn set_score(&mut self, score: u128) {
        self.score = score;
    }

    /// Добавить очки к текущему счёту.
    pub fn add_score(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    /// Получить текущий уровень.
    #[must_use]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Установить текущий уровень.
    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    /// Получить количество очищенных линий.
    #[must_use]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Установить количество очищенных линий.
    pub fn set_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = lines;
    }

    /// Добавить очищенные линии.
    pub fn add_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = self.lines_cleared.saturating_add(lines);
    }
}

// ============================================================================
// FIGUREMANAGER — УПРАВЛЕНИЕ ФИГУРАМИ
// ============================================================================

/// Менеджер фигур.
///
/// Отвечает за управление фигурами:
/// - Текущая фигура
/// - Следующая фигура
/// - Удержанная фигура
/// - Генератор фигур (7-bag)
///
/// ## Архитектурные заметки
/// Выделено из GameState для улучшения инкапсуляции.
/// Инкапсулирует логику управления фигурами.
#[derive(Clone)]
pub struct FigureManager {
    /// Текущая фигура.
    pub(crate) curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    pub(crate) next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    pub(crate) held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    pub(crate) can_hold: bool,
    /// Генератор фигур по системе 7-bag.
    pub(crate) bag: BagGenerator,
}

impl FigureManager {
    /// Создать новый менеджер фигур.
    ///
    /// # Аргументы
    /// * `bag` - генератор фигур (передаётся извне)
    #[must_use]
    pub fn new(bag: BagGenerator) -> Self {
        let mut bag_mut = bag;
        let curr_shape = Tetromino::from_bag(&mut bag_mut);
        let next_shape = Tetromino::from_bag(&mut bag_mut);

        Self {
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            bag: bag_mut,
        }
    }

    /// Получить текущую фигуру.
    #[must_use]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить текущую фигуру (мутуабельная ссылка).
    #[must_use]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.curr_shape
    }

    /// Установить текущую фигуру.
    pub fn set_curr_shape(&mut self, shape: Tetromino) {
        self.curr_shape = shape;
    }

    /// Получить следующую фигуру.
    #[must_use]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить следующую фигуру (мутуабельная ссылка).
    #[must_use]
    pub fn get_next_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.next_shape
    }

    /// Установить следующую фигуру.
    pub fn set_next_shape(&mut self, shape: Tetromino) {
        self.next_shape = shape;
    }

    /// Получить удержанную фигуру.
    #[must_use]
    pub fn get_held_shape(&self) -> Option<&Tetromino> {
        self.held_shape.as_ref()
    }

    /// Получить удержанную фигуру (ссылка на Option).
    #[must_use]
    #[allow(clippy::ref_option)]
    pub fn get_held_shape_ref(&self) -> &Option<Tetromino> {
        &self.held_shape
    }

    /// Установить удержанную фигуру.
    pub fn set_held_shape(&mut self, shape: Option<Tetromino>) {
        self.held_shape = shape;
    }

    /// Проверить возможность удержания фигуры.
    #[must_use]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Установить возможность удержания фигуры.
    pub fn set_can_hold(&mut self, can: bool) {
        self.can_hold = can;
    }

    /// Получить генератор фигур.
    #[must_use]
    pub fn get_bag(&self) -> &BagGenerator {
        &self.bag
    }

    /// Получить генератор фигур (мутуабельная ссылка).
    #[must_use]
    pub fn get_bag_mut(&mut self) -> &mut BagGenerator {
        &mut self.bag
    }

    /// Обработать удержание фигуры.
    ///
    /// Меняет текущую фигуру с удержанной местами.
    /// Если удержанной фигуры нет, создаёт новую из мешка.
    pub fn hold_shape(&mut self) {
        if self.can_hold {
            let current_shape = self.curr_shape;

            if let Some(held) = self.held_shape {
                self.curr_shape = held;
                self.held_shape = Some(current_shape);
            } else {
                self.held_shape = Some(current_shape);
                self.curr_shape = self.next_shape;
                self.next_shape = Tetromino::from_bag(&mut self.bag);
            }

            self.curr_shape.pos = (4.0, 0.0);
            self.can_hold = false;
        }
    }
}

impl Default for FigureManager {
    fn default() -> Self {
        Self::new(BagGenerator::new())
    }
}

// ============================================================================
// ANIMATIONSTATE — СОСТОЯНИЕ АНИМАЦИЙ
// ============================================================================

/// Состояние анимаций.
///
/// Отвечает за управление анимациями:
/// - Анимация очистки линий
/// - Анимация Hard Drop
///
/// ## Архитектурные заметки
/// Выделено из GameState для улучшения инкапсуляции.
/// Инкапсулирует состояние анимаций.
#[derive(Clone, Default)]
pub struct AnimationState {
    /// Строки для анимации (мигание при очистке).
    pub(crate) animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    pub(crate) is_hard_dropping: bool,
}

impl AnimationState {
    /// Создать новое состояние анимаций.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Получить маску анимации строк.
    #[must_use]
    pub fn get_animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Установить маску анимации строк.
    pub fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
    }

    /// Проверить, анимируется ли строка.
    #[must_use]
    pub fn is_row_animating(&self, y: usize) -> bool {
        (self.animating_rows_mask & (1 << y)) != 0
    }

    /// Проверить флаг Hard Drop.
    #[must_use]
    pub fn is_hard_dropping(&self) -> bool {
        self.is_hard_dropping
    }

    /// Установить флаг Hard Drop.
    pub fn set_hard_dropping(&mut self, dropping: bool) {
        self.is_hard_dropping = dropping;
    }

    /// Запустить анимацию очистки линий.
    ///
    /// # Аргументы
    /// * `rows_mask` - битовая маска строк для анимации
    pub fn start_clear_animation(&mut self, rows_mask: u32) {
        self.animating_rows_mask = rows_mask;
    }

    /// Остановить анимацию очистки линий.
    pub fn stop_clear_animation(&mut self) {
        self.animating_rows_mask = 0;
    }
}
