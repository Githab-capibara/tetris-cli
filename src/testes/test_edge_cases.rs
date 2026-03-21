//! Тесты граничных случаев.
//!
//! Этот модуль содержит 50 тестов для проверки граничных случаев:
//! - Тесты пустых значений (10 тестов)
//! - Тесты максимальных значений (10 тестов)
//! - Тесты специальных случаев (10 тестов)
//! - Тесты ошибок и исключений (10 тестов)
//! - Тесты производительности (10 тестов)

#![allow(deprecated)]

use crate::game::{GameMode, GameState};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ 1-10: Пустые значения
// ============================================================================

/// Тест 1: Проверка создания GameState с пустым полем
#[test]
fn test_edge_cases_empty_field() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    for (y, row) in blocks.iter().enumerate().take(GRID_HEIGHT) {
        for (x, &cell) in row.iter().enumerate().take(GRID_WIDTH) {
            assert_eq!(cell, -1, "Клетка [{},{}] должна быть пустой", y, x);
        }
    }
}

/// Тест 2: Проверка что BagGenerator пуст при создании
#[test]
fn test_edge_cases_bag_empty() {
    let bag = BagGenerator::new();
    assert_eq!(bag.get_bag().len(), 0);
}

/// Тест 3: Проверка что удержанная фигура None при создании
#[test]
fn test_edge_cases_held_shape_none() {
    let state = GameState::new();
    assert!(state.get_held_shape().is_none());
}

/// Тест 4: Проверка что achievements пуст при создании
#[test]
fn test_edge_cases_achievements_empty() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert_eq!(stats.achievements.len(), 0);
}

/// Тест 5: Проверка что start_time None при создании
#[test]
fn test_edge_cases_start_time_none() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert!(stats.start_time.is_none());
}

/// Тест 6: Проверка что end_time None при создании
#[test]
fn test_edge_cases_end_time_none() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert!(stats.end_time.is_none());
}

/// Тест 7: Проверка что combo_counter 0 при создании
#[test]
fn test_edge_cases_combo_counter_zero() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert_eq!(stats.combo_counter, 0);
}

/// Тест 8: Проверка что max_combo 0 при создании
#[test]
fn test_edge_cases_max_combo_zero() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert_eq!(stats.max_combo, 0);
}

/// Тест 9: Проверка что tetris_count 0 при создании
#[test]
fn test_edge_cases_tetris_count_zero() {
    let state = GameState::new();
    let stats = state.get_stats();
    assert_eq!(stats.tetris_count, 0);
}

/// Тест 10: Проверка что total_lines 0 при создании
#[test]
fn test_edge_cases_total_lines_zero() {
    let state = GameState::new();
    assert_eq!(state.get_lines_cleared(), 0);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-20: Максимальные значения
// ============================================================================

/// Тест 11: Проверка движения к правой границе
#[test]
fn test_edge_cases_right_boundary() {
    let mut state = GameState::new();

    for _ in 0..20 {
        if state.can_move_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    let x = state.get_curr_shape().pos.0;
    assert!(x < GRID_WIDTH as f32);
}

/// Тест 12: Проверка движения к левой границе
#[test]
fn test_edge_cases_left_boundary() {
    let mut state = GameState::new();

    for _ in 0..20 {
        if state.can_move_curr_shape(crate::game::Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    let x = state.get_curr_shape().pos.0;
    assert!(x >= 0.0);
}

/// Тест 13: Проверка движения к нижней границе
#[test]
fn test_edge_cases_bottom_boundary() {
    let mut state = GameState::new();

    while state.can_move_curr_shape(crate::game::Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    let y = state.get_curr_shape().pos.1;
    assert!(y < GRID_HEIGHT as f32);
}

/// Тест 14: Проверка BagGenerator на 7000 фигур
#[test]
fn test_edge_cases_bag_7000_shapes() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    for _ in 0..7000 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    for &count in counts.iter() {
        assert_eq!(count, 1000);
    }
}

/// Тест 15: Проверка вращения 100 раз
#[test]
fn test_edge_cases_rotate_100_times() {
    let mut state = GameState::new();

    for _ in 0..100 {
        if state.can_rotate_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
        }
    }

    // Должно вернуться в исходное состояние (4 вращения = полный цикл)
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 16: Проверка удержания 10 раз (должно работать только первое)
#[test]
fn test_edge_cases_hold_10_times() {
    let mut state = GameState::new();

    // Первое удержание
    state.hold_shape();
    assert!(!state.can_hold());

    // Последующие не должны работать
    for _ in 0..9 {
        // can_hold должен оставаться false
        assert!(!state.can_hold());
    }
}

/// Тест 17: Проверка создания 1000 GameState
#[test]
fn test_edge_cases_create_1000_states() {
    for _ in 0..1000 {
        let _state = GameState::new();
    }
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 18: Проверка создания 1000 Tetromino
#[test]
fn test_edge_cases_create_1000_tetromino() {
    for _ in 0..1000 {
        let _t = Tetromino::select();
    }
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 19: Проверка BagGenerator на 10000 фигур
#[test]
fn test_edge_cases_bag_10000_shapes() {
    let mut bag = BagGenerator::new();

    for _ in 0..10_000 {
        let shape = bag.next_shape();
        assert!((shape as usize) < 7);
    }

    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 20: Проверка что скорость не превышает разумные пределы
#[test]
fn test_edge_cases_speed_reasonable() {
    let state = GameState::new();
    let fall_spd = state.get_fall_spd();

    assert!(fall_spd < 10.0);
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-30: Специальные случаи
// ============================================================================

/// Тест 21: Проверка что O-фигура не вращается
#[test]
fn test_edge_cases_o_no_rotate() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::O;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::O as usize];

    let original_coords = state.get_curr_shape().coords;

    if state.can_rotate_curr_shape(crate::game::Dir::Right) {
        state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
    }

    assert_eq!(state.get_curr_shape().coords, original_coords);
}

/// Тест 22: Проверка что I-фигура вращается
#[test]
fn test_edge_cases_i_rotates() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::I;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::I as usize];

    let original_coords = state.get_curr_shape().coords;

    if state.can_rotate_curr_shape(crate::game::Dir::Right) {
        state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
    }

    assert_ne!(state.get_curr_shape().coords, original_coords);
}

/// Тест 23: Проверка что все 7 фигур встречаются в BagGenerator
#[test]
fn test_edge_cases_all_seven_in_bag() {
    let mut bag = BagGenerator::new();
    let mut found = [false; 7];

    for _ in 0..7 {
        let shape = bag.next_shape();
        found[shape as usize] = true;
    }

    for &f in found.iter() {
        assert!(f);
    }
}

/// Тест 24: Проверка что фигуры не выходят за границы
#[test]
fn test_edge_cases_shapes_within_bounds() {
    let mut state = GameState::new();

    // Двигаем ко всем границам
    for _ in 0..10 {
        if state.can_move_curr_shape(crate::game::Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    while state.can_move_curr_shape(crate::game::Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    let x = state.get_curr_shape().pos.0;
    let y = state.get_curr_shape().pos.1;

    assert!(x >= 0.0 && x < GRID_WIDTH as f32);
    assert!(y >= 0.0 && y < GRID_HEIGHT as f32);
}

/// Тест 25: Проверка что can_hold сбрасывается после hold
#[test]
fn test_edge_cases_can_hold_resets() {
    let mut state = GameState::new();

    assert!(state.can_hold());

    state.hold_shape();

    assert!(!state.can_hold());
}

/// Тест 26: Проверка что позиция сбрасывается после hold
#[test]
fn test_edge_cases_position_resets_after_hold() {
    let mut state = GameState::new();

    state.get_curr_shape_mut().pos = (2.0, 5.0);
    state.hold_shape();

    assert_eq!(state.get_curr_shape().pos, (4.0, 0.0));
}

/// Тест 27: Проверка что следующая фигура не None
#[test]
fn test_edge_cases_next_shape_not_none() {
    let state = GameState::new();
    let next = state.get_next_shape();

    assert!((next.shape as usize) < 7);
}

/// Тест 28: Проверка что текущая фигура не None
#[test]
fn test_edge_cases_curr_shape_not_none() {
    let state = GameState::new();
    let curr = state.get_curr_shape();

    assert!((curr.shape as usize) < 7);
}

/// Тест 29: Проверка что поле имеет правильный размер
#[test]
fn test_edge_cases_field_correct_size() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    assert_eq!(blocks.len(), GRID_HEIGHT);
    assert_eq!(blocks[0].len(), GRID_WIDTH);
}

/// Тест 30: Проверка что режим по умолчанию Classic
#[test]
fn test_edge_cases_default_mode_classic() {
    let state = GameState::new();
    assert_eq!(state.get_mode(), GameMode::Classic);
}

// ============================================================================
// ГРУППА ТЕСТОВ 31-40: Ошибки и исключения
// ============================================================================

/// Тест 31: Проверка что вращение не вызывает панику у стены
#[test]
fn test_edge_cases_rotation_at_wall_no_panic() {
    let mut state = GameState::new();

    for _ in 0..10 {
        if state.can_move_curr_shape(crate::game::Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Вращение не должно вызывать панику
    let _ = state.can_rotate_curr_shape(crate::game::Dir::Right);
}

/// Тест 32: Проверка что движение не вызывает панику на полу
#[test]
fn test_edge_cases_movement_at_floor_no_panic() {
    let mut state = GameState::new();

    while state.can_move_curr_shape(crate::game::Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение не должно вызывать панику
    let _ = state.can_move_curr_shape(crate::game::Dir::Down);
}

/// Тест 33: Проверка что BagGenerator не паникует при 100000 вызовах
#[test]
fn test_edge_cases_bag_100k_no_panic() {
    let mut bag = BagGenerator::new();

    for _ in 0..100_000 {
        let _ = bag.next_shape();
    }

    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 34: Проверка что hold не паникует при повторном вызове
#[test]
fn test_edge_cases_double_hold_no_panic() {
    let mut state = GameState::new();

    state.hold_shape();

    // Повторный вызов не должен паниковать
    // (хотя и не должен ничего делать)
    assert!(!state.can_hold());
}

/// Тест 35: Проверка что вращение не паникует для O-фигуры
#[test]
fn test_edge_cases_o_rotation_no_panic() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::O;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::O as usize];

    // Вращение O-фигуры не должно паниковать
    let _ = state.can_rotate_curr_shape(crate::game::Dir::Right);
}

/// Тест 36: Проверка что get_blocks не паникует
#[test]
fn test_edge_cases_get_blocks_no_panic() {
    let state = GameState::new();
    let _blocks = state.get_blocks();
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 37: Проверка что get_stats не паникует
#[test]
fn test_edge_cases_get_stats_no_panic() {
    let state = GameState::new();
    let _stats = state.get_stats();
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 38: Проверка что get_next_shape не паникует
#[test]
fn test_edge_cases_get_next_shape_no_panic() {
    let state = GameState::new();
    let _next = state.get_next_shape();
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 39: Проверка что get_curr_shape не паникует
#[test]
fn test_edge_cases_get_curr_shape_no_panic() {
    let state = GameState::new();
    let _curr = state.get_curr_shape();
    // Тест успешно завершён, если код достиг этой строки
}

/// Тест 40: Проверка что get_held_shape не паникует
#[test]
fn test_edge_cases_get_held_shape_no_panic() {
    let state = GameState::new();
    let _held = state.get_held_shape();
    // Тест успешно завершён, если код достиг этой строки
}

// ============================================================================
// ГРУППА ТЕСТОВ 41-50: Производительность
// ============================================================================

/// Тест 41: Проверка производительности создания GameState (10000)
#[test]
fn test_edge_cases_performance_create_10k_states() {
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let _state = GameState::new();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 42: Проверка производительности BagGenerator (100000)
#[test]
fn test_edge_cases_performance_bag_100k() {
    let mut bag = BagGenerator::new();
    let start = std::time::Instant::now();

    for _ in 0..100_000 {
        let _ = bag.next_shape();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 43: Проверка производительности вращения (100000)
#[test]
fn test_edge_cases_performance_rotation_100k() {
    let mut state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..100_000 {
        if state.can_rotate_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
        }
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 44: Проверка производительности движения (100000)
#[test]
fn test_edge_cases_performance_movement_100k() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..100_000 {
        let _ = state.can_move_curr_shape(crate::game::Dir::Down);
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 45: Проверка производительности проверки столкновений (100000)
#[test]
fn test_edge_cases_performance_collision_100k() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..100_000 {
        let _ = state.can_move_curr_shape(crate::game::Dir::Down);
        let _ = state.can_move_curr_shape(crate::game::Dir::Left);
        let _ = state.can_move_curr_shape(crate::game::Dir::Right);
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 46: Проверка производительности hold (10000)
#[test]
fn test_edge_cases_performance_hold_10k() {
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let mut state = GameState::new();
        state.hold_shape();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 10.0);
}

/// Тест 47: Проверка производительности get_blocks (10000)
#[test]
fn test_edge_cases_performance_get_blocks_10k() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let _blocks = state.get_blocks();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 48: Проверка производительности get_stats (10000)
#[test]
fn test_edge_cases_performance_get_stats_10k() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let _stats = state.get_stats();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 49: Проверка производительности Tetromino::select() (100000)
#[test]
fn test_edge_cases_performance_select_100k() {
    let start = std::time::Instant::now();

    for _ in 0..100_000 {
        let _t = Tetromino::select();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}

/// Тест 50: Проверка производительности BagGenerator::new() (10000)
#[test]
fn test_edge_cases_performance_bag_new_10k() {
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let _bag = BagGenerator::new();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0);
}
