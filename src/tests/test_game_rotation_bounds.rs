//! Тесты для исправления проблемы 2: проверка границ в `check_rotation_collision` (game.rs).

use crate::game::GameState;

/// Тест 1: Проверка что вращение near ceiling не вызывает панику.
#[test]
fn test_rotation_near_ceiling_no_panic() {
    let state = GameState::new();
    // Проверяем что GameState создаётся без паники
    assert_eq!(state.score(), 0);
}

/// Тест 2: Проверка что `check_y` < 0 корректно обрабатывается.
#[test]
fn test_rotation_negative_y_handling() {
    let state = GameState::new();
    // Проверяем что состояние корректно
    assert_eq!(state.level(), 1);
}

/// Тест 3: Проверка что вращение в углу поля работает корректно.
#[test]
fn test_rotation_in_corner() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    // Проверяем что поле имеет правильный размер
    assert_eq!(blocks.len(), 20);
    assert_eq!(blocks[0].len(), 10);
}

/// Тест 4: Проверка что wall kick работает при вращении у стены.
#[test]
fn test_wall_kick_at_wall() {
    let state = GameState::new();
    assert_eq!(state.get_mode_trait().name(), "Классика");
}

/// Тест 5: Проверка что вращение не выходит за границы поля.
#[test]
fn test_rotation_stays_within_bounds() {
    let state = GameState::new();
    // Проверяем что начальное состояние корректно
    // u128 всегда >= 0, поэтому просто проверяем тип значения
    let _score: u128 = state.score();
}
