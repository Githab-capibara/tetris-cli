//! Общие типы и перечисления для использования во всех модулях.
//!
//! Этот модуль содержит базовые типы, которые используются несколькими модулями
//! для предотвращения циклических зависимостей.
//!
//! ## Структура модуля
//! - [`Direction`] — направление движения фигуры (переэкспорт из [`crate::core`])
//! - [`RotationDirection`] — направление вращения фигуры (переэкспорт из [`crate::core`])
//! - `Position` — позиция в пространстве (переэкспорт из [`crate::core`])
//! - [`GameAction`](crate::game::types::GameAction) — игровые действия (определён в `game::types`)
//! - [`UpdateEndState`] — состояние завершения обновления
//!
//! ## Архитектурное улучшение 2026-04-02 (#22)
//! `Direction`, `RotationDirection` и `Position` определены в `crate::core` и
//! переэкспортируются здесь для обратной совместимости. `crate::core` является
//! единственным источником истины для этих типов.

// Переэкспорт базовых типов из core модуля для обратной совместимости
// Источник истины: crate::core (Direction, RotationDirection)
// Position удалён — не используется через этот re-export
pub use crate::core::{Direction, RotationDirection};

// ============================================================================
/// Состояние завершения обновления.
///
/// Возвращается методами обновления игры для указания текущего состояния.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UpdateEndState {
    /// Выход из игры.
    Quit,
    /// Проигрыш.
    Lost,
    /// Продолжить.
    Continue,
    /// Пауза.
    Pause,
    /// Победа (завершение режима спринт/марафон).
    Won,
}

impl std::fmt::Display for UpdateEndState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateEndState::Quit => write!(f, "Quit"),
            UpdateEndState::Lost => write!(f, "Lost"),
            UpdateEndState::Continue => write!(f, "Continue"),
            UpdateEndState::Pause => write!(f, "Pause"),
            UpdateEndState::Won => write!(f, "Won"),
        }
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_direction_from_core() {
        // Проверка что Direction переэкспортирован из core
        let _ = Direction::Left;
        let _ = Direction::Right;
        let _ = Direction::Down;
    }

    #[test]
    fn test_position_from_core() {
        // Position определяется в crate::core напрямую
        let pos = crate::core::Position::new(5, 10);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), 10);
    }

    #[test]
    fn test_update_end_state_debug() {
        assert_eq!(format!("{:?}", UpdateEndState::Quit), "Quit");
        assert_eq!(format!("{:?}", UpdateEndState::Lost), "Lost");
        assert_eq!(format!("{:?}", UpdateEndState::Continue), "Continue");
        assert_eq!(format!("{:?}", UpdateEndState::Pause), "Pause");
        assert_eq!(format!("{:?}", UpdateEndState::Won), "Won");
    }
}
