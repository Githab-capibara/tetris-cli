//! Тесты статистики игры.
//!
//! Этот модуль содержит 30 тестов для проверки системы статистики:
//! - Тесты `GameStats` (15 тестов)
//! - Тесты подсчёта фигур (10 тестов)
//! - Тесты комбо и достижений (5 тестов)

use crate::game::{GameMode, GameStats};
use crate::tetromino::ShapeType;

// ============================================================================
// ГРУППА ТЕСТОВ 1-15: GameStats
// ============================================================================

/// Тест 1: Проверка создания `GameStats`
#[test]
fn test_statistics_game_stats_creation() {
    let stats = GameStats::new();
    assert_eq!(stats.t_pieces, 0);
    assert_eq!(stats.l_pieces, 0);
    assert_eq!(stats.j_pieces, 0);
    assert_eq!(stats.s_pieces, 0);
    assert_eq!(stats.z_pieces, 0);
    assert_eq!(stats.o_pieces, 0);
    assert_eq!(stats.i_pieces, 0);
}

/// Тест 2: Проверка что `max_combo` равен 0 при создании
#[test]
fn test_statistics_max_combo_zero() {
    let stats = GameStats::new();
    assert_eq!(stats.max_combo, 0);
}

/// Тест 3: Проверка что `combo_counter` равен 0 при создании
#[test]
fn test_statistics_combo_counter_zero() {
    let stats = GameStats::new();
    assert_eq!(stats.combo_counter, 0);
}

/// Тест 4: Проверка что achievements пуст при создании
#[test]
fn test_statistics_achievements_empty() {
    let stats = GameStats::new();
    assert_eq!(stats.achievements.len(), 0);
}

/// Тест 5: Проверка что `tetris_count` равен 0 при создании
#[test]
fn test_statistics_tetris_count_zero() {
    let stats = GameStats::new();
    assert_eq!(stats.tetris_count, 0);
}

/// Тест 6: Проверка что `total_lines` равен 0 при создании
#[test]
fn test_statistics_total_lines_zero() {
    let stats = GameStats::new();
    assert_eq!(stats.total_lines, 0);
}

/// Тест 7: Проверка что `start_time` равен None при создании
#[test]
fn test_statistics_start_time_none() {
    let stats = GameStats::new();
    assert!(stats.start_time.is_none());
}

/// Тест 8: Проверка что `end_time` равен None при создании
#[test]
fn test_statistics_end_time_none() {
    let stats = GameStats::new();
    assert!(stats.end_time.is_none());
}

/// Тест 9: Проверка Clone для `GameStats`
#[test]
fn test_statistics_clone() {
    let mut original = GameStats::new();
    original.t_pieces = 5;
    original.l_pieces = 3;

    let cloned = original.clone();
    assert_eq!(cloned.t_pieces, 5);
    assert_eq!(cloned.l_pieces, 3);
}

/// Тест 10: Проверка Default для `GameStats`
#[test]
fn test_statistics_default() {
    let stats = GameStats::default();
    assert_eq!(stats.t_pieces, 0);
}

/// Тест 11: Проверка `add_piece` для T
#[test]
fn test_statistics_add_t_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::T);
    assert_eq!(stats.t_pieces, 1);
}

/// Тест 12: Проверка `add_piece` для L
#[test]
fn test_statistics_add_l_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::L);
    assert_eq!(stats.l_pieces, 1);
}

/// Тест 13: Проверка `add_piece` для J
#[test]
fn test_statistics_add_j_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::J);
    assert_eq!(stats.j_pieces, 1);
}

/// Тест 14: Проверка `add_piece` для S
#[test]
fn test_statistics_add_s_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::S);
    assert_eq!(stats.s_pieces, 1);
}

/// Тест 15: Проверка `add_piece` для Z
#[test]
fn test_statistics_add_z_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::Z);
    assert_eq!(stats.z_pieces, 1);
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-25: Подсчёт фигур
// ============================================================================

/// Тест 16: Проверка `add_piece` для O
#[test]
fn test_statistics_add_o_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::O);
    assert_eq!(stats.o_pieces, 1);
}

/// Тест 17: Проверка `add_piece` для I
#[test]
fn test_statistics_add_i_piece() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::I);
    assert_eq!(stats.i_pieces, 1);
}

/// Тест 18: Проверка `total_pieces` с одной фигурой
#[test]
fn test_statistics_total_pieces_one() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::T);
    assert_eq!(stats.total_pieces(), 1);
}

/// Тест 19: Проверка `total_pieces` с несколькими фигурами
#[test]
fn test_statistics_total_pieces_multiple() {
    let mut stats = GameStats::new();

    for _ in 0..10 {
        stats.add_piece(ShapeType::T);
    }
    for _ in 0..5 {
        stats.add_piece(ShapeType::I);
    }

    assert_eq!(stats.total_pieces(), 15);
}

/// Тест 20: Проверка `total_pieces` со всеми типами фигур
#[test]
fn test_statistics_total_pieces_all_types() {
    let mut stats = GameStats::new();

    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::L);
    stats.add_piece(ShapeType::J);
    stats.add_piece(ShapeType::S);
    stats.add_piece(ShapeType::Z);
    stats.add_piece(ShapeType::O);
    stats.add_piece(ShapeType::I);

    assert_eq!(stats.total_pieces(), 7);
}

/// Тест 21: Проверка `total_pieces` с нулём фигур
#[test]
fn test_statistics_total_pieces_zero() {
    let stats = GameStats::new();
    assert_eq!(stats.total_pieces(), 0);
}

/// Тест 22: Проверка `add_piece` увеличивает счётчик
#[test]
fn test_statistics_add_piece_increments() {
    let mut stats = GameStats::new();

    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::T);

    assert_eq!(stats.t_pieces, 3);
}

/// Тест 23: Проверка что `add_piece` не изменяет другие счётчики
#[test]
fn test_statistics_add_piece_does_not_affect_others() {
    let mut stats = GameStats::new();

    stats.add_piece(ShapeType::T);

    assert_eq!(stats.l_pieces, 0);
    assert_eq!(stats.j_pieces, 0);
    assert_eq!(stats.s_pieces, 0);
}

/// Тест 24: Проверка `total_pieces` после множественных добавлений
#[test]
fn test_statistics_total_pieces_after_many_adds() {
    let mut stats = GameStats::new();

    for _ in 0..100 {
        stats.add_piece(ShapeType::T);
    }

    assert_eq!(stats.t_pieces, 100);
    assert_eq!(stats.total_pieces(), 100);
}

/// Тест 25: Проверка что `total_pieces` корректно суммирует
#[test]
fn test_statistics_total_pieces_sum() {
    let mut stats = GameStats::new();

    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::L);
    stats.add_piece(ShapeType::L);
    stats.add_piece(ShapeType::I);

    assert_eq!(stats.t_pieces, 2);
    assert_eq!(stats.l_pieces, 2);
    assert_eq!(stats.i_pieces, 1);
    assert_eq!(stats.total_pieces(), 5);
}

// ============================================================================
// ГРУППА ТЕСТОВ 26-30: Комбо и достижения
// ============================================================================

/// Тест 26: Проверка `update_max_combo` с 1 линией
#[test]
fn test_statistics_update_max_combo_one() {
    let mut stats = GameStats::new();
    stats.update_max_combo(1);
    assert_eq!(stats.max_combo, 1);
}

/// Тест 27: Проверка `update_max_combo` с 4 линиями
#[test]
fn test_statistics_update_max_combo_four() {
    let mut stats = GameStats::new();
    stats.update_max_combo(4);
    assert_eq!(stats.max_combo, 4);
}

/// Тест 28: Проверка `update_max_combo` сохраняет максимум
#[test]
fn test_statistics_update_max_combo_keeps_max() {
    let mut stats = GameStats::new();

    stats.update_max_combo(2);
    stats.update_max_combo(1); // Не должно изменить max_combo

    assert_eq!(stats.max_combo, 2);
}

/// Тест 29: Проверка `start_timer` и `get_elapsed_time`
#[test]
fn test_statistics_timer() {
    let mut stats = GameStats::new();
    stats.start_timer();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let elapsed = stats.get_elapsed_time();
    assert!(elapsed > 0.0);
}

/// Тест 30: Проверка `check_achievements` за Tetris
#[test]
fn test_statistics_check_achievements_tetris() {
    let mut stats = GameStats::new();

    let achievements = stats.check_achievements(4, 1, GameMode::Classic);

    assert_eq!(achievements.len(), 1);
    assert_eq!(achievements[0].name, "🏆 TETRIS!");
}
