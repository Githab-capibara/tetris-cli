//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные компоненты для разделения ответственности GameState:
//! - [`GameBoard`] — состояние игрового поля (перемещён в [`super::board`])
//! - [`ScoreBoard`] — очки и уровни (перемещён в [`super::scoreboard`])
//! - [`FigureManager`] — управление фигурами
//! - [`AnimationState`] — анимации
//! - [`GamePhase`] — фаза игры
//!
//! ## Архитектурные заметки
//! Этот модуль создан в рамках исправления C1 (CRITICAL) для соблюдения Single Responsibility Principle.
//! GameState использует композицию этих компонентов вместо хранения всех полей напрямую.
//!
//! ## Существующие компоненты
//! - [`GameBoard`] находится в [`super::board`] — состояние поля (blocks, filled_lines_mask)
//! - [`ScoreBoard`] находится в [`super::scoreboard`] — очки и уровни (score, level, lines_cleared)
//!
//! ## Новые компоненты
//! - [`FigureManager`] — управление фигурами (curr_shape, next_shape, held_shape, bag, can_hold)
//! - [`AnimationState`] — анимации (animating_rows_mask, is_hard_dropping, is_game_over)
//! - [`GamePhase`] — фаза игры (is_paused, game_complete)

use crate::io::GRID_HEIGHT;
use crate::tetromino::{BagGenerator, Tetromino};

// Переэкспорт существующих компонентов для удобства
pub use super::board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard,
};
pub use super::scoreboard::{ScoreAccess as ScoreAccessTrait, ScoreBoard, ScoreMutable};

// ============================================================================
// FIGURE MANAGER — УПРАВЛЕНИЕ ФИГУРАМИ
// ============================================================================

/// Менеджер фигур.
///
/// Инкапсулирует состояние фигур и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `curr_shape` — текущая падающая фигура
/// - `next_shape` — следующая фигура (предпросмотр)
/// - `held_shape` — удержанная фигура (None если ещё не использовалась)
/// - `can_hold` — можно ли ещё менять удержанную фигуру в этом ходу
/// - `bag` — генератор фигур по системе 7-bag
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `figure_manager: FigureManager`.
pub struct FigureManager {
    /// Текущая фигура.
    curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    can_hold: bool,
    /// Генератор фигур по системе 7-bag.
    bag: BagGenerator,
}

impl Default for FigureManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FigureManager {
    /// Создать новый менеджер фигур.
    ///
    /// # Возвращает
    /// Новый экземпляр `FigureManager` с инициализированными фигурами из мешка.
    pub fn new() -> Self {
        let mut bag = BagGenerator::new();
        let curr_shape = Tetromino::from_bag(&mut bag);
        let next_shape = Tetromino::from_bag(&mut bag);

        Self {
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            bag,
        }
    }

    /// Получить текущую фигуру.
    #[inline]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить следующую фигуру.
    #[inline]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить удержанную фигуру.
    #[inline]
    pub fn get_held_shape(&self) -> Option<&Tetromino> {
        self.held_shape.as_ref()
    }

    /// Проверить возможность удержания фигуры.
    #[inline]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Установить текущую фигуру.
    #[inline]
    pub fn set_curr_shape(&mut self, shape: Tetromino) {
        self.curr_shape = shape;
    }

    /// Установить следующую фигуру.
    #[inline]
    pub fn set_next_shape(&mut self, shape: Tetromino) {
        self.next_shape = shape;
    }

    /// Установить удержанную фигуру.
    #[inline]
    pub fn set_held_shape(&mut self, shape: Option<Tetromino>) {
        self.held_shape = shape;
    }

    /// Установить флаг возможности удержания.
    #[inline]
    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }

    /// Получить генератор фигур.
    #[inline]
    pub fn get_bag(&self) -> &BagGenerator {
        &self.bag
    }

    /// Получить генератор фигур (мутуабельная ссылка).
    #[inline]
    pub fn get_bag_mut(&mut self) -> &mut BagGenerator {
        &mut self.bag
    }

    /// Получить следующую фигуру из мешка и обновить состояние.
    ///
    /// # Возвращает
    /// Следующую фигуру из мешка
    #[inline]
    pub fn get_next_from_bag(&mut self) -> Tetromino {
        let next = Tetromino::from_bag(&mut self.bag);
        self.next_shape = next;
        self.next_shape
    }
}

// ============================================================================
// ANIMATION STATE — СОСТОЯНИЕ АНИМАЦИЙ
// ============================================================================

/// Состояние анимаций.
///
/// Инкапсулирует состояние анимаций и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `animating_rows_mask` — битовая маска строк для анимации очистки
/// - `is_hard_dropping` — флаг анимации Hard Drop
/// - `is_game_over` — флаг завершения игры
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `animation_state: AnimationState`.
pub struct AnimationState {
    /// Строки для анимации (мигание при очистке).
    animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    is_hard_dropping: bool,
    /// Флаг завершения игры.
    is_game_over: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationState {
    /// Создать новое состояние анимаций.
    ///
    /// # Возвращает
    /// Новый экземпляр `AnimationState` с нулевыми флагами.
    pub fn new() -> Self {
        Self {
            animating_rows_mask: 0,
            is_hard_dropping: false,
            is_game_over: false,
        }
    }

    /// Получить маску анимации строк.
    #[inline]
    pub fn get_animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Проверить флаг Hard Drop.
    #[inline]
    pub fn is_hard_dropping(&self) -> bool {
        self.is_hard_dropping
    }

    /// Проверить флаг завершения игры.
    #[inline]
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    /// Установить маску анимации строк.
    #[inline]
    pub fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
    }

    /// Установить флаг Hard Drop.
    #[inline]
    pub fn set_is_hard_dropping(&mut self, value: bool) {
        self.is_hard_dropping = value;
    }

    /// Установить флаг завершения игры.
    #[inline]
    pub fn set_is_game_over(&mut self, value: bool) {
        self.is_game_over = value;
    }

    /// Добавить строку в маску анимации.
    ///
    /// # Аргументы
    /// * `row` — номер строки (0..GRID_HEIGHT)
    #[inline]
    pub fn add_animating_row(&mut self, row: usize) {
        if row < GRID_HEIGHT {
            self.animating_rows_mask |= 1 << row;
        }
    }

    /// Очистить маску анимации строк.
    #[inline]
    pub fn clear_animating_rows(&mut self) {
        self.animating_rows_mask = 0;
    }
}

// ============================================================================
// GAME PHASE — ФАЗА ИГРЫ
// ============================================================================

/// Фаза игры.
///
/// Инкапсулирует состояние фазы игры и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `is_paused` — флаг паузы
/// - `game_complete` — флаг завершения игры (победа или поражение)
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `phase: GamePhase`.
pub struct GamePhase {
    /// Флаг паузы.
    is_paused: bool,
    /// Флаг завершения игры.
    game_complete: bool,
}

impl Default for GamePhase {
    fn default() -> Self {
        Self::new()
    }
}

impl GamePhase {
    /// Создать новую фазу игры.
    ///
    /// # Возвращает
    /// Новый экземпляр `GamePhase` с активным состоянием (не на паузе, не завершена).
    pub fn new() -> Self {
        Self {
            is_paused: false,
            game_complete: false,
        }
    }

    /// Проверить флаг паузы.
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Проверить флаг завершения игры.
    #[inline]
    pub fn is_game_complete(&self) -> bool {
        self.game_complete
    }

    /// Установить флаг паузы.
    #[inline]
    pub fn set_is_paused(&mut self, value: bool) {
        self.is_paused = value;
    }

    /// Установить флаг завершения игры.
    #[inline]
    pub fn set_is_game_complete(&mut self, value: bool) {
        self.game_complete = value;
    }

    /// Поставить игру на паузу.
    #[inline]
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Снять игру с паузы.
    #[inline]
    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    /// Переключить состояние паузы.
    #[inline]
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    /// Завершить игру.
    #[inline]
    pub fn complete(&mut self) {
        self.game_complete = true;
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::ShapeType;

    // ========================================================================
    // ТЕСТЫ FIGURE MANAGER
    // ========================================================================

    #[test]
    fn test_figure_manager_new() {
        let manager = FigureManager::new();
        assert!(manager.can_hold(), "Новая игра должна разрешать hold");
        // ShapeType не реализует Ord, проверяем через match
        match manager.get_curr_shape().shape {
            ShapeType::I
            | ShapeType::O
            | ShapeType::T
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::L
            | ShapeType::J => {}
        }
        match manager.get_next_shape().shape {
            ShapeType::I
            | ShapeType::O
            | ShapeType::T
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::L
            | ShapeType::J => {}
        }
        assert!(
            manager.get_held_shape().is_none(),
            "Hold должен быть пуст в начале"
        );
    }

    #[test]
    fn test_figure_manager_setters() {
        let mut manager = FigureManager::new();
        let shape = Tetromino::from_bag(&mut BagGenerator::new());

        manager.set_curr_shape(shape);
        // Проверяем что фигура установлена (любая из 7)
        match manager.get_curr_shape().shape {
            ShapeType::I
            | ShapeType::O
            | ShapeType::T
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::L
            | ShapeType::J => {}
        }

        manager.set_held_shape(Some(shape));
        assert!(manager.get_held_shape().is_some());

        manager.set_can_hold(false);
        assert!(!manager.can_hold());
    }

    #[test]
    fn test_figure_manager_get_next_from_bag() {
        let mut manager = FigureManager::new();
        let next = manager.get_next_from_bag();
        match next.shape {
            ShapeType::I
            | ShapeType::O
            | ShapeType::T
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::L
            | ShapeType::J => {}
        }
        assert_eq!(manager.get_next_shape().shape, next.shape);
    }

    // ========================================================================
    // ТЕСТЫ ANIMATION STATE
    // ========================================================================

    #[test]
    fn test_animation_state_new() {
        let anim = AnimationState::new();
        assert_eq!(anim.get_animating_rows_mask(), 0);
        assert!(!anim.is_hard_dropping());
        assert!(!anim.is_game_over());
    }

    #[test]
    fn test_animation_state_row_mask() {
        let mut anim = AnimationState::new();

        anim.add_animating_row(5);
        assert_eq!(anim.get_animating_rows_mask(), 1 << 5);

        anim.add_animating_row(10);
        assert_eq!(anim.get_animating_rows_mask(), (1 << 5) | (1 << 10));

        anim.clear_animating_rows();
        assert_eq!(anim.get_animating_rows_mask(), 0);
    }

    #[test]
    fn test_animation_state_flags() {
        let mut anim = AnimationState::new();

        anim.set_is_hard_dropping(true);
        assert!(anim.is_hard_dropping());

        anim.set_is_game_over(true);
        assert!(anim.is_game_over());
    }

    // ========================================================================
    // ТЕСТЫ GAME PHASE
    // ========================================================================

    #[test]
    fn test_game_phase_new() {
        let phase = GamePhase::new();
        assert!(!phase.is_paused());
        assert!(!phase.is_game_complete());
    }

    #[test]
    fn test_game_phase_pause_resume() {
        let mut phase = GamePhase::new();

        phase.pause();
        assert!(phase.is_paused());

        phase.resume();
        assert!(!phase.is_paused());

        phase.toggle_pause();
        assert!(phase.is_paused());

        phase.toggle_pause();
        assert!(!phase.is_paused());
    }

    #[test]
    fn test_game_phase_complete() {
        let mut phase = GamePhase::new();

        phase.complete();
        assert!(phase.is_game_complete());
    }
}
