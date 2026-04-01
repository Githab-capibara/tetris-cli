//! Компоненты состояния игры.
//!
//! Этот модуль содержит специализированные компоненты для разделения ответственности GameState.

pub use super::access::{ScoreAccess as ScoreAccessTrait, ScoreMutable};
pub use super::board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard as Board,
};
pub use super::scoreboard::ScoreBoard;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_board_component() {
        let board = Board::new();
        assert_eq!(board.get_filled_lines_mask(), 0);
    }
}
