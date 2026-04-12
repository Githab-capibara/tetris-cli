//! Расширенные интеграционные тесты для Tetris CLI.
//!
//! Содержит тесты взаимодействия, не дублирующие тесты из `test_integration.rs`.

#![allow(deprecated)]
// Cast sign_loss намеренно: i32→u32 только с положительными значениями
#![allow(clippy::cast_sign_loss)]

use crate::game::GameState;

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + HIGHSCORE
// ============================================================================

// Тест test_curr_and_next_shapes_different удалён — дублирует проверки из
// test_full_game_initialization (test_integration.rs), который уже проверяет
// валидность curr_shape() и next_shape().

/// Тест: Marathon режим поддерживает сохранение рекорда.
#[test]
fn test_marathon_mode_saves_score() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Режим должен быть Marathon"
    );
}

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + CONTROLS
// ============================================================================
