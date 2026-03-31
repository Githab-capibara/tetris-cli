//! Общие типы и перечисления для использования во всех модулях.
//!
//! Этот модуль содержит базовые типы, которые используются несколькими модулями
//! для предотвращения циклических зависимостей.
//!
//! ## Структура модуля
//! - [`Direction`] — направление движения фигуры (переэкспорт из [`crate::core`])
//! - [`RotationDirection`] — направление вращения фигуры (переэкспорт из [`crate::core`])
//! - [`Position`] — позиция в пространстве (переэкспорт из [`crate::core`])
//! - [`GameAction`] — игровые действия
//! - [`UpdateEndState`] — состояние завершения обновления

// Переэкспорт базовых типов из core модуля для обратной совместимости
pub use crate::core::{Direction, Position, RotationDirection};

// ============================================================================
// GAMEACTION ENUM (Абстракция ввода)
// ============================================================================

/// Перечисление игровых действий.
///
/// Представляет абстракцию ввода, отделяя конкретные клавиши от игровых действий.
/// Используется для маппинга клавиш → действия в системе управления.
///
/// ## Архитектурные заметки
/// Введение GameAction соответствует:
/// - **Dependency Inversion Principle (DIP)** - модуль ввода зависит от абстракции
/// - **Interface Segregation Principle (ISP)** - узкоспециализированный интерфейс
/// - Уменьшает связанность между controls.rs и input.rs
///
/// ## Варианты
/// - `MoveLeft` - движение фигуры влево
/// - `MoveRight` - движение фигуры вправо
/// - `SoftDrop` - ускоренное падение
/// - `HardDrop` - мгновенное падение
/// - `RotateLeft` - вращение против часовой стрелки
/// - `RotateRight` - вращение по часовой стрелке
/// - `Hold` - удержание фигуры
/// - `Pause` - пауза
/// - `Quit` - выход в меню
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    /// Движение фигуры влево.
    MoveLeft,
    /// Движение фигуры вправо.
    MoveRight,
    /// Ускоренное падение (Soft Drop).
    SoftDrop,
    /// Мгновенное падение (Hard Drop).
    HardDrop,
    /// Вращение против часовой стрелки.
    RotateLeft,
    /// Вращение по часовой стрелке.
    RotateRight,
    /// Удержание фигуры.
    Hold,
    /// Пауза.
    Pause,
    /// Выход в меню.
    Quit,
}

impl GameAction {
    /// Проверить, является ли действие движением.
    #[must_use]
    pub const fn is_movement(self) -> bool {
        matches!(self, Self::MoveLeft | Self::MoveRight)
    }

    /// Проверить, является ли действие вращением.
    #[must_use]
    pub const fn is_rotation(self) -> bool {
        matches!(self, Self::RotateLeft | Self::RotateRight)
    }

    /// Проверить, является ли действие падением.
    #[must_use]
    pub const fn is_drop(self) -> bool {
        matches!(self, Self::SoftDrop | Self::HardDrop)
    }
}

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
    fn test_rotation_direction_from_core() {
        // Проверка что RotationDirection переэкспортирован из core
        let _ = RotationDirection::Clockwise;
        let _ = RotationDirection::CounterClockwise;
        let _ = RotationDirection::NoRotation;
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
