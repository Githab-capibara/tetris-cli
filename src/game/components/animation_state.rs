//! Компонент состояния анимаций.
//!
//! # Ответственность
//! - Управление маской анимируемых строк (animating_rows_mask)
//! - Управление флагом Hard Drop (is_hard_dropping)
//!
//! ## Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle.
//! `AnimationState` инкапсулирует состояние анимаций и предоставляет контролируемый доступ.
//!
//! Архитектурное улучшение 2026-04-01: Выделение компонента для улучшения модульности.

#![allow(dead_code)]

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
    pub fn new() -> Self {
        Self {
            animating_rows_mask: 0,
            is_hard_dropping: false,
        }
    }

    /// Получить маску анимируемых строк.
    #[must_use]
    pub fn animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Установить маску анимируемых строк.
    pub fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
    }

    /// Получить флаг Hard Drop.
    #[must_use]
    pub fn is_hard_dropping(&self) -> bool {
        self.is_hard_dropping
    }

    /// Установить флаг Hard Drop.
    pub fn set_is_hard_dropping(&mut self, value: bool) {
        self.is_hard_dropping = value;
    }

    /// Добавить строку в маску анимации.
    ///
    /// # Аргументы
    /// * `row` - номер строки (0-19)
    pub fn add_row_to_animation(&mut self, row: u32) {
        if row < 32 {
            self.animating_rows_mask |= 1 << row;
        }
    }

    /// Удалить строку из маски анимации.
    ///
    /// # Аргументы
    /// * `row` - номер строки (0-19)
    pub fn remove_row_from_animation(&mut self, row: u32) {
        if row < 32 {
            self.animating_rows_mask &= !(1 << row);
        }
    }

    /// Очистить маску анимации.
    pub fn clear_animation_mask(&mut self) {
        self.animating_rows_mask = 0;
    }

    /// Проверить, есть ли активные анимации.
    #[must_use]
    pub fn has_active_animations(&self) -> bool {
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
}
