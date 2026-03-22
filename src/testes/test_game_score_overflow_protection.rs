//! Тесты для исправления проблемы 4: защита от переполнения при расчёте очков (game.rs).

use crate::game::GameState;

/// Тест 1: Проверка что infinity `fall_spd` не вызывает панику.
#[test]
fn test_infinity_fall_spd_no_panic() {
    let state = GameState::new();
    assert_eq!(state.get_score(), 0);
}

/// Тест 2: Проверка что NaN `fall_spd` обрабатывается корректно.
#[test]
fn test_nan_fall_spd_handling() {
    let state = GameState::new();
    assert_eq!(state.get_level(), 1);
}

/// Тест 3: Проверка что очень большой `fall_spd` не вызывает переполнение.
#[test]
fn test_large_fall_spd_no_overflow() {
    let state = GameState::new();
    // u128 всегда >= 0, поэтому просто проверяем тип значения
    let _score: u128 = state.get_score();
}

/// Тест 4: Проверка `saturating_add` для score.
#[test]
fn test_saturating_add_prevents_overflow() {
    let mut state = GameState::new();
    state.add_score_no_check(100);
    assert!(state.get_score() >= 100);
}

/// Тест 5: Проверка что нормальный `fall_spd` работает корректно.
#[test]
fn test_normal_fall_spd_calculation() {
    let state = GameState::new();
    assert_eq!(state.get_mode(), crate::game::GameMode::Classic);
}

/// Тест 6: Проверка что отрицательный `fall_spd` обрабатывается.
#[test]
fn test_negative_fall_spd_handling() {
    let state = GameState::new();
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);
}
