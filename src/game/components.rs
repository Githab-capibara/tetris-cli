//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные компоненты для разделения ответственности GameState.
//!
//! ## Архитектурные заметки
//! Модуль предоставляет компоненты для разделения God Object GameState:
//! - [`FigureManager`] - управление фигурами (curr_shape, next_shape, held_shape, bag)
//! - [`AnimationState`] - управление анимациями (animating_rows_mask, is_hard_dropping)
//! - [`BoardState`] / [`FieldState`] - управление полем (board, filled_lines_mask)
//!
//! Архитектурное улучшение 2026-04-01: Выделение компонентов для улучшения модульности.

// Подмодули компонентов
pub mod animation_state;
pub mod board_state;
pub mod figure_manager;

// Re-export основных типов для удобства использования
pub use super::access::{ScoreAccess as ScoreAccessTrait, ScoreMutable};
pub use super::board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard as Board,
};
pub use super::scoreboard::ScoreBoard;

// Re-export компонентов
pub use animation_state::AnimationState;
pub use board_state::BoardState;
pub use figure_manager::FigureManager;

/// Тип для состояния поля (алиас для BoardState).
///
/// ## Архитектурные заметки
/// Этот тип существует для единообразия именования с другими компонентами.
/// Для нового кода рекомендуется использовать [`BoardState`] напрямую.
pub type FieldState = BoardState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_board_component() {
        let board = Board::new();
        assert_eq!(board.get_filled_lines_mask(), 0);
    }

    #[test]
    fn test_figure_manager_component() {
        let manager = FigureManager::new();
        assert!(manager.can_hold());
        assert!(manager.held_shape().is_none());
    }

    #[test]
    fn test_animation_state_component() {
        let anim = AnimationState::new();
        assert_eq!(anim.animating_rows_mask(), 0);
        assert!(!anim.is_hard_dropping());
    }

    #[test]
    fn test_board_state_component() {
        let board = BoardState::new();
        assert_eq!(board.filled_lines_mask(), 0);
    }
}
