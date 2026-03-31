//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные компоненты для разделения ответственности GameState:
//! - [`GameBoard`] — состояние игрового поля (перемещён в [`super::board`])
//! - [`ScoreBoard`] — очки и уровни (перемещён в [`super::scoreboard`])
//!
//! ## Архитектурные заметки
//! Этот модуль создан в рамках исправления C1 (CRITICAL) для соблюдения Single Responsibility Principle.
//! GameState использует композицию этих компонентов вместо хранения всех полей напрямую.
//!
//! ## Существующие компоненты
//! - [`GameBoard`] находится в [`super::board`] — состояние поля (blocks, filled_lines_mask)
//! - [`ScoreBoard`] находится в [`super::scoreboard`] — очки и уровни (score, level, lines_cleared)

use crate::io::GRID_HEIGHT;

// Переэкспорт существующих компонентов для удобства
pub use super::access::{ScoreAccess as ScoreAccessTrait, ScoreMutable};
pub use super::board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard,
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
        let board = GameBoard::new();
        assert_eq!(board.get_filled_lines_mask(), 0);
    }
}
