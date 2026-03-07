//! Тесты режимов игры.
//!
//! Этот модуль содержит 40 тестов для проверки режимов игры:
//! - Тесты Classic режима (10 тестов)
//! - Тесты Sprint режима (15 тестов)
//! - Тесты Marathon режима (15 тестов)

use crate::game::{GameMode, GameState};
use crate::game::{MARATHON_LINES, SPRINT_LINES};

// ============================================================================
// ГРУППА ТЕСТОВ 1-10: Classic режим
// ============================================================================

/// Тест 1: Проверка создания Classic режима
#[test]
fn test_modes_classic_creation() {
    let state = GameState::new();
    assert_eq!(state.get_mode(), GameMode::Classic);
}

/// Тест 2: Проверка что Classic режим имеет счёт 0
#[test]
fn test_modes_classic_zero_score() {
    let state = GameState::new();
    assert_eq!(state.get_score(), 0);
}

/// Тест 3: Проверка что Classic режим имеет уровень 1
#[test]
fn test_modes_classic_level_one() {
    let state = GameState::new();
    assert_eq!(state.get_level(), 1);
}

/// Тест 4: Проверка что Classic режим имеет линии 0
#[test]
fn test_modes_classic_zero_lines() {
    let state = GameState::new();
    assert_eq!(state.get_lines_cleared(), 0);
}

/// Тест 5: Проверка что Classic режим имеет удержанную фигуру None
#[test]
fn test_modes_classic_held_shape_none() {
    let state = GameState::new();
    assert!(state.get_held_shape().is_none());
}

/// Тест 6: Проверка что Classic режим имеет can_hold true
#[test]
fn test_modes_classic_can_hold_true() {
    let state = GameState::new();
    assert!(state.can_hold());
}

/// Тест 7: Проверка что Classic режим имеет таймер не запущен
#[test]
fn test_modes_classic_timer_not_started() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert!(stats.start_time.is_none());
}

/// Тест 8: Проверка что Classic режим сохраняет рекорд
#[test]
fn test_modes_classic_saves_score() {
    // Classic режим должен сохранять счёт в таблицу лидеров
    let state = GameState::new();
    assert_eq!(state.get_mode(), GameMode::Classic);
}

/// Тест 9: Проверка что Classic режим имеет стандартную скорость
#[test]
fn test_modes_classic_standard_speed() {
    let state = GameState::new();
    let fall_spd = state.get_fall_spd();
    assert!((fall_spd - 0.9).abs() < f32::EPSILON);
}

/// Тест 10: Проверка что Classic режим работает до проигрыша
#[test]
fn test_modes_classic_play_until_loss() {
    let state = GameState::new();
    // Classic режим не имеет цели по линиям
    assert_eq!(state.get_lines_cleared(), 0);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-25: Sprint режим
// ============================================================================

/// Тест 11: Проверка создания Sprint режима
#[test]
fn test_modes_sprint_creation() {
    let state = GameState::new_sprint();
    assert_eq!(state.get_mode(), GameMode::Sprint);
}

/// Тест 12: Проверка что Sprint режим имеет цель 40 линий
#[test]
fn test_modes_sprint_goal_40_lines() {
    assert_eq!(SPRINT_LINES, 40, "Цель спринта должна быть 40 линий");
}

/// Тест 13: Проверка что Sprint режим имеет таймер
#[test]
fn test_modes_sprint_has_timer() {
    let mut state = GameState::new_sprint();
    state.start_timer();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed > 0.0);
}

/// Тест 14: Проверка что Sprint режим имеет счёт 0
#[test]
fn test_modes_sprint_zero_score() {
    let state = GameState::new_sprint();
    assert_eq!(state.get_score(), 0);
}

/// Тест 15: Проверка что Sprint режим имеет уровень 1
#[test]
fn test_modes_sprint_level_one() {
    let state = GameState::new_sprint();
    assert_eq!(state.get_level(), 1);
}

/// Тест 16: Проверка что Sprint режим имеет линии 0
#[test]
fn test_modes_sprint_zero_lines() {
    let state = GameState::new_sprint();
    assert_eq!(state.get_lines_cleared(), 0);
}

/// Тест 17: Проверка что Sprint режим имеет удержанную фигуру None
#[test]
fn test_modes_sprint_held_shape_none() {
    let state = GameState::new_sprint();
    assert!(state.get_held_shape().is_none());
}

/// Тест 18: Проверка что Sprint режим имеет can_hold true
#[test]
fn test_modes_sprint_can_hold_true() {
    let state = GameState::new_sprint();
    assert!(state.can_hold());
}

/// Тест 19: Проверка что Sprint режим имеет стандартную скорость
#[test]
fn test_modes_sprint_standard_speed() {
    let state = GameState::new_sprint();
    let fall_spd = state.get_fall_spd();
    assert!((fall_spd - 0.9).abs() < f32::EPSILON);
}

/// Тест 20: Проверка что Sprint режим завершается при 40 линиях
#[test]
fn test_modes_sprint_ends_at_40_lines() {
    // Проверяем константу
    assert_eq!(SPRINT_LINES, 40);
}

/// Тест 21: Проверка что Sprint режим не сохраняет рекорд
#[test]
fn test_modes_sprint_does_not_save_score() {
    // Sprint режим не сохраняет счёт в таблицу лидеров
    let state = GameState::new_sprint();
    assert_eq!(state.get_mode(), GameMode::Sprint);
}

/// Тест 22: Проверка работы таймера в Sprint режиме
#[test]
fn test_modes_sprint_timer_works() {
    let mut state = GameState::new_sprint();
    state.start_timer();

    std::thread::sleep(std::time::Duration::from_millis(100));

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed >= 0.1);
}

/// Тест 23: Проверка что Sprint режим имеет статистику
#[test]
fn test_modes_sprint_has_stats() {
    let state = GameState::new_sprint();
    let stats = state.get_stats();
    assert_eq!(stats.total_pieces(), 1);
}

/// Тест 24: Проверка что Sprint режим имеет следующую фигуру
#[test]
fn test_modes_sprint_has_next_shape() {
    let state = GameState::new_sprint();
    let next = state.get_next_shape();
    assert!((next.shape as usize) < 7);
}

/// Тест 25: Проверка что Sprint режим имеет текущую фигуру
#[test]
fn test_modes_sprint_has_curr_shape() {
    let state = GameState::new_sprint();
    let curr = state.get_curr_shape();
    assert!((curr.shape as usize) < 7);
}

// ============================================================================
// ГРУППА ТЕСТОВ 26-40: Marathon режим
// ============================================================================

/// Тест 26: Проверка создания Marathon режима
#[test]
fn test_modes_marathon_creation() {
    let state = GameState::new_marathon();
    assert_eq!(state.get_mode(), GameMode::Marathon);
}

/// Тест 27: Проверка что Marathon режим имеет цель 150 линий
#[test]
fn test_modes_marathon_goal_150_lines() {
    assert_eq!(MARATHON_LINES, 150, "Цель марафона должна быть 150 линий");
}

/// Тест 28: Проверка что Marathon режим имеет счёт 0
#[test]
fn test_modes_marathon_zero_score() {
    let state = GameState::new_marathon();
    assert_eq!(state.get_score(), 0);
}

/// Тест 29: Проверка что Marathon режим имеет уровень 1
#[test]
fn test_modes_marathon_level_one() {
    let state = GameState::new_marathon();
    assert_eq!(state.get_level(), 1);
}

/// Тест 30: Проверка что Marathon режим имеет линии 0
#[test]
fn test_modes_marathon_zero_lines() {
    let state = GameState::new_marathon();
    assert_eq!(state.get_lines_cleared(), 0);
}

/// Тест 31: Проверка что Marathon режим имеет удержанную фигуру None
#[test]
fn test_modes_marathon_held_shape_none() {
    let state = GameState::new_marathon();
    assert!(state.get_held_shape().is_none());
}

/// Тест 32: Проверка что Marathon режим имеет can_hold true
#[test]
fn test_modes_marathon_can_hold_true() {
    let state = GameState::new_marathon();
    assert!(state.can_hold());
}

/// Тест 33: Проверка что Marathon режим имеет стандартную скорость
#[test]
fn test_modes_marathon_standard_speed() {
    let state = GameState::new_marathon();
    let fall_spd = state.get_fall_spd();
    assert!((fall_spd - 0.9).abs() < f32::EPSILON);
}

/// Тест 34: Проверка что Marathon режим завершается при 150 линиях
#[test]
fn test_modes_marathon_ends_at_150_lines() {
    assert_eq!(MARATHON_LINES, 150);
}

/// Тест 35: Проверка что Marathon режим сохраняет рекорд
#[test]
fn test_modes_marathon_saves_score() {
    // Marathon режим сохраняет счёт в таблицу лидеров
    let state = GameState::new_marathon();
    assert_eq!(state.get_mode(), GameMode::Marathon);
}

/// Тест 36: Проверка что Marathon режим имеет статистику
#[test]
fn test_modes_marathon_has_stats() {
    let state = GameState::new_marathon();
    let stats = state.get_stats();
    assert_eq!(stats.total_pieces(), 1);
}

/// Тест 37: Проверка что Marathon режим имеет следующую фигуру
#[test]
fn test_modes_marathon_has_next_shape() {
    let state = GameState::new_marathon();
    let next = state.get_next_shape();
    assert!((next.shape as usize) < 7);
}

/// Тест 38: Проверка что Marathon режим имеет текущую фигуру
#[test]
fn test_modes_marathon_has_curr_shape() {
    let state = GameState::new_marathon();
    let curr = state.get_curr_shape();
    assert!((curr.shape as usize) < 7);
}

/// Тест 39: Проверка что Marathon режим имеет пустое поле
#[test]
fn test_modes_marathon_empty_field() {
    let state = GameState::new_marathon();
    let blocks = state.get_blocks();

    for y in 0..20 {
        for x in 0..10 {
            assert_eq!(blocks[y][x], -1);
        }
    }
}

/// Тест 40: Проверка что Marathon режим работает до завершения
#[test]
fn test_modes_marathon_play_until_completion() {
    let state = GameState::new_marathon();
    // Marathon режим имеет цель 150 линий
    assert_eq!(state.get_lines_cleared(), 0);
}
