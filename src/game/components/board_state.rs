//! Компонент состояния поля.
//!
//! # Ответственность
//! - Управление игровым полем (board)
//! - Управление маской заполненных линий (`filled_lines_mask`)
//!
//! ## Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle.
//! Этот модуль переэкспортирует `GameBoard` из `crate::game::board` для удобства.
//!
//! Архитектурное улучшение 2026-04-01: Выделение компонента для улучшения модульности.

#![allow(dead_code)]

// Переэкспорт GameBoard из board.rs
pub use crate::game::board::GameBoard;

/// Компонент состояния поля.
///
/// Использует `GameBoard` из `super::board` для управления состоянием поля.
/// Этот модуль предоставляет удобный переэкспорт для композиции в `GameState`.
///
/// ## Архитектурные заметки
/// Для нового кода используйте `GameBoard` напрямую из `super::board`.
/// Этот модуль существует для единообразия с другими компонентами.
#[derive(Debug, Clone)]
pub struct BoardState {
    /// Внутреннее состояние поля.
    inner: GameBoard,
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardState {
    /// Создать новое состояние поля.
    ///
    /// # Возвращает
    /// Новый экземпляр `BoardState` с пустым полем.
    #[must_use = "Состояние поля должно быть использовано"]
    pub fn new() -> Self {
        Self {
            inner: GameBoard::new(),
        }
    }

    /// Получить доступ к внутреннему `GameBoard`.
    ///
    /// # Возвращает
    /// Ссылка на `GameBoard`
    #[must_use = "GameBoard должен быть использован"]
    pub fn inner(&self) -> &GameBoard {
        &self.inner
    }

    /// Получить мутуабельный доступ к внутреннему `GameBoard`.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `GameBoard`
    #[must_use = "Мутуабельный GameBoard должен быть использован"]
    pub fn inner_mut(&mut self) -> &mut GameBoard {
        &mut self.inner
    }

    /// Получить битовую маску заполненных линий.
    ///
    /// # Возвращает
    /// Битовая маска заполненных линий
    #[must_use = "Маска заполненных линий должна быть использована"]
    pub fn filled_lines_mask(&self) -> u32 {
        self.inner.get_filled_lines_mask()
    }

    /// Установить битовую маску заполненных линий.
    ///
    /// # Аргументы
    /// * `mask` - битовая маска заполненных линий
    pub fn set_filled_lines_mask(&mut self, mask: u32) {
        self.inner.set_filled_lines_mask(mask);
    }

    /// Получить количество заполненных линий.
    ///
    /// # Возвращает
    /// Количество заполненных линий
    #[must_use = "Количество заполненных линий должно быть использовано"]
    pub fn filled_lines_count(&self) -> u32 {
        self.inner.get_filled_lines_count()
    }

    /// Очистить заполненные линии.
    ///
    /// # Возвращает
    /// Количество очищенных линий
    ///
    /// # Пример
    /// ```ignore
    /// use crate::game::components::BoardState;
    ///
    /// let mut state = BoardState::new();
    /// state.set_filled_lines_mask(0b1010);
    /// let cleared = state.clear_filled_lines();
    /// assert_eq!(cleared, 2);
    /// ```
    pub fn clear_filled_lines(&mut self) -> u32 {
        self.inner.clear_filled_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_state_new() {
        let state = BoardState::new();
        assert_eq!(state.filled_lines_mask(), 0);
        assert_eq!(state.filled_lines_count(), 0);
    }

    #[test]
    fn test_board_state_mask() {
        let mut state = BoardState::new();
        state.set_filled_lines_mask(0b1010);
        assert_eq!(state.filled_lines_mask(), 0b1010);
        assert_eq!(state.filled_lines_count(), 2);
    }

    #[test]
    fn test_board_state_clear() {
        let mut state = BoardState::new();
        state.set_filled_lines_mask(0b1111);
        assert_eq!(state.clear_filled_lines(), 4);
        assert_eq!(state.filled_lines_mask(), 0);
    }
}
