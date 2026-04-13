//! Компонент состояния анимаций.
//!
//! # Ответственность
//! - Управление маской анимируемых строк (`animating_rows_mask`)
//! - Управление флагом Hard Drop (`is_hard_dropping`)
//!
//! ## Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle.
//! `AnimationState` инкапсулирует состояние анимаций и предоставляет контролируемый доступ.
//!
//! Архитектурное улучшение 2026-04-01: Выделение компонента для улучшения модульности.

/// Состояние анимаций в игре.
///
/// Инкапсулирует состояние всех анимаций и предоставляет контролируемый доступ.
///
/// ## Поля
/// - `animating_rows_mask` - битовая маска строк для анимации (мигание при очистке)
/// - `is_hard_dropping` - флаг для анимации Hard Drop
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `animation_state: AnimationState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationState {
    /// Строки для анимации (мигание при очистке).
    /// Каждый бит соответствует линии поля (бит 0 = линия 0).
    animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    is_hard_dropping: bool,
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
    /// Новый экземпляр `AnimationState` с выключенными анимациями.
    #[must_use = "Состояние анимации должно быть использовано"]
    pub const fn new() -> Self {
        Self {
            animating_rows_mask: 0,
            is_hard_dropping: false,
        }
    }

    /// Получить маску анимируемых строк.
    ///
    /// # Возвращает
    /// Битовая маска анимируемых строк
    #[must_use = "Маска анимации должна быть использована"]
    pub const fn animating_rows_mask(self) -> u32 {
        self.animating_rows_mask
    }

    /// Установить маску анимируемых строк.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска строк для анимации
    pub const fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
    }

    /// Получить флаг Hard Drop.
    ///
    /// # Возвращает
    /// `true` если активен Hard Drop
    #[must_use = "Флаг Hard Drop должен быть использован"]
    pub const fn is_hard_dropping(self) -> bool {
        self.is_hard_dropping
    }

    /// Установить флаг Hard Drop.
    ///
    /// # Аргументы
    /// * `value` - значение флага Hard Drop
    pub const fn set_is_hard_dropping(&mut self, value: bool) {
        self.is_hard_dropping = value;
    }

    /// Добавить строку в маску анимации.
    ///
    /// # Аргументы
    /// * `row` - номер строки (0-19)
    ///
    /// # Паника
    /// Паникует если `row >= 32` (выход за пределы битовой маски u32)
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::components::AnimationState;
    ///
    /// let mut state = AnimationState::new();
    /// state.add_row_to_animation(5);
    /// assert_eq!(state.animating_rows_mask(), 1 << 5);
    /// ```
    pub const fn add_row_to_animation(&mut self, row: u32) {
        assert!(row < 32);
        self.animating_rows_mask |= 1 << row;
    }

    /// Удалить строку из маски анимации.
    ///
    /// # Аргументы
    /// * `row` - номер строки (0-19)
    ///
    /// # Паника
    /// Паникует если `row >= 32` (выход за пределы битовой маски u32)
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::components::AnimationState;
    ///
    /// let mut state = AnimationState::new();
    /// state.add_row_to_animation(5);
    /// state.remove_row_from_animation(5);
    /// assert_eq!(state.animating_rows_mask(), 0);
    /// ```
    pub const fn remove_row_from_animation(&mut self, row: u32) {
        assert!(row < 32);
        self.animating_rows_mask &= !(1 << row);
    }

    /// Очистить маску анимации.
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::components::AnimationState;
    ///
    /// let mut state = AnimationState::new();
    /// state.add_row_to_animation(5);
    /// state.add_row_to_animation(10);
    /// state.clear_animation_mask();
    /// assert_eq!(state.animating_rows_mask(), 0);
    /// ```
    pub const fn clear_animation_mask(&mut self) {
        self.animating_rows_mask = 0;
    }

    /// Проверить, есть ли активные анимации.
    ///
    /// # Возвращает
    /// `true` если есть активные анимации
    #[must_use = "Результат проверки анимаций должен быть использован"]
    pub const fn has_active_animations(self) -> bool {
        self.animating_rows_mask != 0 || self.is_hard_dropping
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_state_new() {
        let state = AnimationState::new();
        assert_eq!(state.animating_rows_mask(), 0);
        assert!(!state.is_hard_dropping());
        assert!(!state.has_active_animations());
    }

    #[test]
    fn test_animation_state_add_row() {
        let mut state = AnimationState::new();
        state.add_row_to_animation(5);
        assert_eq!(state.animating_rows_mask(), 1 << 5);
    }

    #[test]
    fn test_animation_state_remove_row() {
        let mut state = AnimationState::new();
        state.add_row_to_animation(5);
        state.add_row_to_animation(10);
        state.remove_row_from_animation(5);
        assert_eq!(state.animating_rows_mask(), 1 << 10);
    }

    #[test]
    fn test_animation_state_hard_drop() {
        let mut state = AnimationState::new();
        state.set_is_hard_dropping(true);
        assert!(state.is_hard_dropping());
        assert!(state.has_active_animations());
    }

    #[test]
    fn test_animation_state_clear() {
        let mut state = AnimationState::new();
        state.add_row_to_animation(5);
        state.add_row_to_animation(10);
        state.clear_animation_mask();
        assert_eq!(state.animating_rows_mask(), 0);
    }

    #[test]
    fn test_animation_state_clone_and_equality() {
        let mut state = AnimationState::new();
        state.add_row_to_animation(3);
        state.set_is_hard_dropping(true);

        // Clone должен сохранять равенство
        let cloned = state;
        assert_eq!(state, cloned);

        // Изменение оригинала не влияет на копию
        let mut state2 = state;
        state2.set_is_hard_dropping(false);
        assert_ne!(state, state2);
    }

    #[test]
    fn test_animation_state_debug_display() {
        let state = AnimationState::new();
        let debug_str = format!("{state:?}");
        assert!(
            debug_str.contains("AnimationState"),
            "Debug должен содержать имя типа"
        );
    }
}
