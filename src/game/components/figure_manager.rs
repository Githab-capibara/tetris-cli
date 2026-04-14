//! Менеджер фигур для управления состоянием фигур.
//!
//! # Ответственность
//! - Управление текущей фигурой (`curr_shape`)
//! - Управление следующей фигурой (`next_shape`)
//! - Управление удержанной фигурой (`held_shape`)
//! - Управление генератором фигур (bag)
//! - Флаг возможности удержания (`can_hold`)
//!
//! ## Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle.
//! `FigureManager` инкапсулирует состояние фигур и предоставляет контролируемый доступ.
//!
//! Архитектурное улучшение 2026-04-01: Выделение компонента для улучшения модульности.

use crate::tetromino::{BagGenerator, Tetromino};

/// Менеджер фигур в игре.
///
/// Инкапсулирует состояние всех фигур и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `curr_shape` - текущая фигура
/// - `next_shape` - следующая фигура (для предпросмотра)
/// - `held_shape` - удержанная фигура (None если ещё не использовалась)
/// - `can_hold` - можно ли ещё менять удержанную фигуру в этом ходу
/// - `bag` - генератор фигур по системе 7-bag
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
    /// Новый экземпляр `FigureManager` с инициализированными фигурами из генератора.
    #[must_use = "Менеджер фигур должен быть использован"]
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
    #[must_use]
    pub const fn curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить следующую фигуру.
    #[must_use]
    pub const fn next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить удержанную фигуру.
    #[must_use]
    pub const fn held_shape(&self) -> Option<&Tetromino> {
        self.held_shape.as_ref()
    }

    /// Получить флаг возможности удержания.
    #[must_use]
    pub const fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Получить генератор фигур.
    ///
    /// # Возвращает
    /// Ссылка на `BagGenerator`
    #[must_use = "Генератор фигур должен быть использован"]
    pub const fn bag(&self) -> &BagGenerator {
        &self.bag
    }

    /// Получить генератор фигур (мутуабельная ссылка).
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `BagGenerator`
    pub const fn bag_mut(&mut self) -> &mut BagGenerator {
        &mut self.bag
    }

    /// Установить текущую фигуру.
    ///
    /// # Аргументы
    /// * `value` - новая текущая фигура
    pub const fn set_curr_shape(&mut self, value: Tetromino) {
        self.curr_shape = value;
    }

    /// Установить следующую фигуру.
    ///
    /// # Аргументы
    /// * `value` - новая следующая фигура
    pub const fn set_next_shape(&mut self, value: Tetromino) {
        self.next_shape = value;
    }

    /// Установить удержанную фигуру.
    ///
    /// # Аргументы
    /// * `value` - новая удержанная фигура
    pub const fn set_held_shape(&mut self, value: Option<Tetromino>) {
        self.held_shape = value;
    }

    /// Установить флаг возможности удержания.
    ///
    /// # Аргументы
    /// * `value` - значение флага возможности удержания
    pub const fn set_can_hold(&mut self, value: bool) {
        self.can_hold = value;
    }

    /// Вращать текущую фигуру.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения
    pub fn rotate_curr_shape(&mut self, dir: crate::types::RotationDirection) {
        self.curr_shape.rotate(dir);
    }

    /// Применить замыкание к текущей фигуре для мутации.
    ///
    /// # Аргументы
    /// * `f` - замыкание принимающее `&mut Tetromino`
    pub fn mutate_curr_shape<F: FnOnce(&mut Tetromino)>(&mut self, f: F) {
        f(&mut self.curr_shape);
    }

    /// Применить замыкание к следующей фигуре для мутации.
    ///
    /// # Аргументы
    /// * `f` - замыкание принимающее `&mut Tetromino`
    pub fn mutate_next_shape<F: FnOnce(&mut Tetromino)>(&mut self, f: F) {
        f(&mut self.next_shape);
    }

    /// Применить замыкание к удержанной фигуре для мутации.
    ///
    /// # Аргументы
    /// * `f` - замыкание принимающее `&mut Option<Tetromino>`
    pub fn mutate_held_shape<F: FnOnce(&mut Option<Tetromino>)>(&mut self, f: F) {
        f(&mut self.held_shape);
    }

    /// Установить позицию текущей фигуры.
    ///
    /// # Аргументы
    /// * `x` - позиция по X
    /// * `y` - позиция по Y
    pub const fn set_curr_pos(&mut self, x: f32, y: f32) {
        self.curr_shape.set_pos((x, y));
    }

    /// Сместить позицию текущей фигуры по X.
    ///
    /// # Аргументы
    /// * `dx` - смещение по X
    pub fn move_curr_dx(&mut self, dx: f32) {
        self.curr_shape.pos_mut().0 += dx;
    }

    /// Сместить позицию текущей фигуры по Y.
    ///
    /// # Аргументы
    /// * `dy` - смещение по Y
    pub fn move_curr_dy(&mut self, dy: f32) {
        self.curr_shape.pos_mut().1 += dy;
    }

    /// Обновить фигуры после установки новой.
    ///
    /// Перемещает `next_shape` в `curr_shape`, генерирует новую `next_shape` из bag.
    /// Сбрасывает `can_hold` в true.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::components::FigureManager;
    ///
    /// let mut manager = FigureManager::new();
    /// let old_curr = manager.curr_shape().shape();
    /// manager.spawn_new_piece();
    /// // curr_shape теперь содержит предыдущую next_shape
    /// ```
    pub fn spawn_new_piece(&mut self) {
        self.curr_shape = self.next_shape;
        self.next_shape = Tetromino::from_bag(&mut self.bag);
        self.can_hold = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure_manager_new() {
        let manager = FigureManager::new();
        assert!(manager.can_hold());
        assert!(manager.held_shape().is_none());
    }

    #[test]
    fn test_figure_manager_getters() {
        let manager = FigureManager::new();
        let curr = manager.curr_shape();
        let next = manager.next_shape();
        assert_ne!(curr.shape(), next.shape());
    }

    #[test]
    fn test_figure_manager_setters() {
        let mut manager = FigureManager::new();
        manager.set_can_hold(false);
        assert!(!manager.can_hold());
    }

    #[test]
    fn test_figure_manager_spawn() {
        let mut manager = FigureManager::new();
        let old_next = manager.next_shape().shape();
        manager.spawn_new_piece();
        assert_eq!(manager.curr_shape().shape(), old_next);
        assert!(manager.can_hold());
    }
}
