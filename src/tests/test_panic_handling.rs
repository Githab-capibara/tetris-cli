//! Тесты на отсутствие паники в публичных API.
//!
//! PROB-157: Тесты проверяют что ключевые публичные API не паникуют
//! на невалидном или экстремальном вводе.

use std::panic::{self, AssertUnwindSafe};

use crate::game::GameState;
use crate::tetromino::bag_generator::BagGenerator;
use crate::validation::PathValidator;

/// Тест: `GameState::new()` не должен паниковать
#[test]
fn test_game_state_new_no_panic() {
    let result = panic::catch_unwind(|| GameState::new());
    assert!(result.is_ok(), "GameState::new() не должен паниковать");
}

/// Тест: `GameState::new_sprint()` не должен паниковать
#[test]
fn test_game_state_new_sprint_no_panic() {
    let result = panic::catch_unwind(|| GameState::new_sprint());
    assert!(
        result.is_ok(),
        "GameState::new_sprint() не должен паниковать"
    );
}

/// Тест: `GameState::new_marathon()` не должен паниковать
#[test]
fn test_game_state_new_marathon_no_panic() {
    let result = panic::catch_unwind(|| GameState::new_marathon());
    assert!(
        result.is_ok(),
        "GameState::new_marathon() не должен паниковать"
    );
}

/// Тест: `BagGenerator::new()` не должен паниковать
#[test]
fn test_bag_generator_new_no_panic() {
    let result = panic::catch_unwind(|| BagGenerator::new());
    assert!(result.is_ok(), "BagGenerator::new() не должен паниковать");
}

/// Тест: `PathValidator::new()` не должен паниковать
#[test]
fn test_path_validator_new_no_panic() {
    let result = panic::catch_unwind(|| PathValidator::new(255, "abc"));
    assert!(result.is_ok(), "PathValidator::new() не должен паниковать");
}

/// Тест: GameState методы доступа не паникуют на новом состоянии
#[test]
fn test_game_state_accessors_no_panic() {
    let state = GameState::new();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _score = state.score();
        let _level = state.level();
        let _lines = state.lines_cleared();
        let _stats = state.stats();
        let _board = state.board();
        let _blocks = state.get_blocks();
        let _fall_speed = state.fall_speed();
        let _land_timer = state.land_timer();
        let _soft_drop = state.soft_drop_distance();
        let _is_hard_dropping = state.is_hard_dropping();
        let _can_hold = state.can_hold();
        let _curr_shape = state.curr_shape();
        let _next_shape = state.next_shape();
        let _held_shape = state.held_shape();
    }));

    assert!(
        result.is_ok(),
        "Геттеры GameState не должны паниковать на новом состоянии"
    );
}

/// Тест: GameState сеттеры не паникуют на валидных значениях
#[test]
fn test_game_state_setters_no_panic() {
    let mut state = GameState::new();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        state.set_score(0);
        state.set_score(u128::MAX);
        state.set_score(u128::MAX / 2);

        state.set_level(1);
        state.set_level(100);
        state.set_level(u32::MAX);

        state.set_lines_cleared(0);
        state.set_lines_cleared(1000);
        state.set_lines_cleared(u32::MAX);
    }));

    assert!(
        result.is_ok(),
        "Сеттеры GameState не должны паниковать на валидных значениях"
    );
}

/// Тест: `BagGenerator::next_shape()` не паникает при множественных вызовах
#[test]
fn test_bag_generator_next_shape_no_panic() {
    let mut bag = BagGenerator::new();

    // Не используем catch_unwind с &mut — просто вызываем напрямую
    // Если бы была паника, тест упал бы здесь
    for _ in 0..100 {
        let _shape = bag.next_shape();
    }
    // Если дошли сюда — паники не было
}
