//! Общие типы и перечисления для использования во всех модулях.
//!
//! Этот модуль содержит базовые типы, которые используются несколькими модулями
//! для предотвращения циклических зависимостей.
//!
//! ## Структура модуля
//! - [`Direction`] — направление движения фигуры (переэкспорт из [`crate::core`])
//! - [`RotationDirection`] — направление вращения фигуры (переэкспорт из [`crate::core`])
//! - `Position` — позиция в пространстве (переэкспорт из [`crate::core`])
//! - [`GameAction`] — игровые действия (переэкспорт из [`crate::game::types`])
//! - [`UpdateEndState`] — состояние завершения обновления
//!
//! ## Архитектурное улучшение 2026-04-02 (#22)
//! `Direction`, `RotationDirection` и `Position` определены в `crate::core` и
//! переэкспортируются здесь для обратной совместимости. `crate::core` является
//! единственным источником истины для этих типов.

// Переэкспорт базовых типов из core модуля для обратной совместимости
// Источник истины: crate::core (Position, Direction, RotationDirection)
#[allow(unused_imports)]
pub use crate::core::{Direction, Position, RotationDirection};

// R1: GameAction переэкспортирован из game/types.rs для устранения дублирования
#[allow(deprecated, unused_imports)]
pub use crate::game::types::GameAction;

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
        // Проверка что Position переэкспортирован из core
        let pos = Position::new(5, 10);
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

    // ==================== Тесты для GameAction ====================

    #[test]
    fn test_game_action_variants() {
        // Проверка всех вариантов
        let _ = GameAction::MoveLeft;
        let _ = GameAction::MoveRight;
        let _ = GameAction::SoftDrop;
        let _ = GameAction::HardDrop;
        let _ = GameAction::RotateLeft;
        let _ = GameAction::RotateRight;
        let _ = GameAction::Hold;
        let _ = GameAction::Pause;
        let _ = GameAction::Quit;
    }

    #[test]
    fn test_game_action_is_movement() {
        assert!(GameAction::MoveLeft.is_movement());
        assert!(GameAction::MoveRight.is_movement());
        assert!(!GameAction::SoftDrop.is_movement());
        assert!(!GameAction::HardDrop.is_movement());
        assert!(!GameAction::RotateLeft.is_movement());
        assert!(!GameAction::RotateRight.is_movement());
        assert!(!GameAction::Hold.is_movement());
        assert!(!GameAction::Pause.is_movement());
        assert!(!GameAction::Quit.is_movement());
    }

    #[test]
    fn test_game_action_is_rotation() {
        assert!(!GameAction::MoveLeft.is_rotation());
        assert!(!GameAction::MoveRight.is_rotation());
        assert!(!GameAction::SoftDrop.is_rotation());
        assert!(!GameAction::HardDrop.is_rotation());
        assert!(GameAction::RotateLeft.is_rotation());
        assert!(GameAction::RotateRight.is_rotation());
        assert!(!GameAction::Hold.is_rotation());
        assert!(!GameAction::Pause.is_rotation());
        assert!(!GameAction::Quit.is_rotation());
    }

    #[test]
    fn test_game_action_is_drop() {
        assert!(!GameAction::MoveLeft.is_drop());
        assert!(!GameAction::MoveRight.is_drop());
        assert!(GameAction::SoftDrop.is_drop());
        assert!(GameAction::HardDrop.is_drop());
        assert!(!GameAction::RotateLeft.is_drop());
        assert!(!GameAction::RotateRight.is_drop());
        assert!(!GameAction::Hold.is_drop());
        assert!(!GameAction::Pause.is_drop());
        assert!(!GameAction::Quit.is_drop());
    }

    #[test]
    fn test_game_action_debug() {
        assert_eq!(format!("{:?}", GameAction::MoveLeft), "MoveLeft");
        assert_eq!(format!("{:?}", GameAction::HardDrop), "HardDrop");
    }

    #[test]
    fn test_game_action_copy_clone() {
        let action = GameAction::MoveLeft;
        let action_copy = action; // Copy
        let action_clone = action; // Clone

        assert_eq!(action, action_copy);
        assert_eq!(action, action_clone);
    }
}
