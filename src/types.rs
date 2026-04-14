//! Общие типы и перечисления для использования во всех модулях.
//!
//! Этот модуль содержит базовые типы, которые используются несколькими модулями
//! для предотвращения циклических зависимостей.
//!
//! ## Структура модуля
//! - `Direction`, `RotationDirection` — ре-экспорт из `crate::core` для обратной совместимости
//! - `UpdateEndState` — состояние завершения обновления

// Ре-экспорт из crate::core для обратной совместимости
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
            Self::Quit => write!(f, "Quit"),
            Self::Lost => write!(f, "Lost"),
            Self::Continue => write!(f, "Continue"),
            Self::Pause => write!(f, "Pause"),
            Self::Won => write!(f, "Won"),
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
    fn test_position_from_core() {
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
