//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные компоненты для разделения ответственности GameState:
//! - [`FigureState`] — состояние фигур (curr_shape, next_shape, held_shape, bag)
//! - [`BoardState`] — состояние поля (board, filled_lines)
//! - [`AnimationState`] — состояние анимаций (animating_rows_mask, is_hard_dropping)
//! - [`GameBoard`] — состояние игрового поля (перемещён в [`super::board`])
//! - [`ScoreBoard`] — очки и уровни (перемещён в [`super::scoreboard`])
//!
//! ## Архитектурные заметки
//! Этот модуль создан в рамках исправления C1 (CRITICAL) для соблюдения Single Responsibility Principle.
//! GameState использует композицию этих компонентов вместо хранения всех полей напрямую.
//!
//! ## Архитектурное улучшение 2026-04-01
//! Выделены новые компоненты для улучшения модульности:
//! - `FigureState` — инкапсуляция состояния фигур
//! - `BoardState` — обёртка над GameBoard для единообразия
//! - `AnimationState` — инкапсуляция состояния анимаций
//!
//! ## Существующие компоненты
//! - [`GameBoard`] находится в [`super::board`] — состояние поля (blocks, filled_lines_mask)
//! - [`ScoreBoard`] находится в [`super::scoreboard`] — очки и уровни (score, level, lines_cleared)

// Новые компоненты (Архитектурное улучшение 2026-04-01)
pub mod animation_state;
pub mod board_state;
pub mod figure_state;

// Переэкспорт для удобства
pub use animation_state::AnimationState;
pub use board_state::{BoardState, GameBoard};
pub use figure_state::FigureState;

// Переэкспорт существующих компонентов для удобства
pub use super::access::{ScoreAccess as ScoreAccessTrait, ScoreMutable};
pub use super::board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard as Board,
};
pub use super::scoreboard::ScoreBoard;

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_board_component() {
        let board = Board::new();
        assert_eq!(board.get_filled_lines_mask(), 0);
    }

    #[test]
    fn test_figure_state_component() {
        let figure_state = FigureState::new();
        assert!(figure_state.can_hold());
    }

    #[test]
    fn test_animation_state_component() {
        let anim_state = AnimationState::new();
        assert!(!anim_state.is_hard_dropping());
    }

    #[test]
    fn test_board_state_component() {
        let board_state = BoardState::new();
        assert_eq!(board_state.filled_lines_mask(), 0);
    }
}
