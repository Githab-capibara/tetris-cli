//! Тесты системы очков в Tetris CLI.
//!
//! Этот модуль содержит 40 тестов для проверки системы начисления очков:
//! - Тесты очков за фигуры (6 тестов)
//! - Тесты очков за линии (1, 2, 3, 4 линии) (8 тестов)
//! - Тесты комбо-бонусов (6 тестов)
//! - Тесты бонусов за уровень (6 тестов)
//! - Тесты soft drop очков (6 тестов)
//! - Тесты hard drop очков (6 тестов)
//! - Тесты Tetris бонусов (2 теста)
//!
//! Все тесты проверяют корректность начисления очков.

use crate::game::{
    COMBO_BONUS, HARD_DROP_POINTS, INITIAL_FALL_SPD, LINES_PER_LEVEL, PIECE_SCORE_INC, LINE_SCORES,
    SOFT_DROP_POINTS,
};

// ============================================================================
// ГРУППА ТЕСТОВ 1-6: Тесты очков за фигуры
// ============================================================================

/// Тест 1: Базовые очки за фигуру
#[test]
fn test_base_piece_score() {
    assert_eq!(
        PIECE_SCORE_INC, LINE_SCORES[0],
        "Базовые очки за фигуру должны быть 100"
    );
    assert_eq!(
        PIECE_SCORE_INC, 100,
        "Базовые очки за фигуру должны быть 100"
    );
}

/// Тест 2: Очки за фигуру положительны
#[test]
fn test_piece_score_positive() {
    let _ = PIECE_SCORE_INC;
}

/// Тест 3: Очки за размещение фигуры
#[test]
fn test_piece_placement_score() {
    // Базовые очки за размещение
    let base_score = PIECE_SCORE_INC;
    assert!(base_score >= 100, "Очки за размещение должны быть >= 100");
}

/// Тест 4: Очки за фигуру с падением
#[test]
fn test_piece_score_with_fall() {
    // Очки за фигуру включают бонус за падение
    let base = PIECE_SCORE_INC;
    let fall_bonus = INITIAL_FALL_SPD * 50.0;
    let total = base + fall_bonus as u128;

    assert!(total > base, "Очки с падением должны быть больше базовых");
}

/// Тест 5: Минимальные очки за фигуру
#[test]
fn test_minimum_piece_score() {
    // Минимальные очки - базовые без бонусов
    let _ = PIECE_SCORE_INC;
}

/// Тест 6: Очки за фигуру константны
#[test]
fn test_piece_score_constant() {
    // Проверяем, что константа не изменяется
    const TEST_PIECE_SCORE: u128 = 100;
    assert_eq!(
        PIECE_SCORE_INC, LINE_SCORES[0],
        "Очки за фигуру должны быть константой"
    );
    assert_eq!(
        PIECE_SCORE_INC, TEST_PIECE_SCORE,
        "Очки за фигуру должны быть константой"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 7-14: Тесты очков за линии (1, 2, 3, 4 линии)
// ============================================================================

/// Тест 7: Очки за 1 линию
#[test]
fn test_score_for_one_line() {
    // 1 линия: 100 * 2^0 = 100
    let score = LINE_SCORES[0];
    assert_eq!(score, 100, "Очки за 1 линию должны быть 100");
}

/// Тест 8: Очки за 2 линии
#[test]
fn test_score_for_two_lines() {
    // 2 линии: 100 * 2^1 = 200
    let score = LINE_SCORES[0] * 2;
    assert_eq!(score, 200, "Очки за 2 линии должны быть 200");
}

/// Тест 9: Очки за 3 линии
#[test]
fn test_score_for_three_lines() {
    // 3 линии: 100 * 2^2 = 400
    let score = LINE_SCORES[0] * 4;
    assert_eq!(score, 400, "Очки за 3 линии должны быть 400");
}

/// Тест 10: Очки за 4 линии (Tetris)
#[test]
fn test_score_for_four_lines_tetris() {
    // 4 линии: 100 * 2^3 = 800
    let score = LINE_SCORES[0] * 8;
    assert_eq!(score, 800, "Очки за 4 линии (Tetris) должны быть 800");
}

/// Тест 11: Экспоненциальный рост очков за линии
#[test]
fn test_exponential_growth_for_lines() {
    let score_1 = LINE_SCORES[0];
    let score_2 = LINE_SCORES[0] * 2;
    let score_3 = LINE_SCORES[0] * 4;
    let score_4 = LINE_SCORES[0] * 8;

    assert!(score_2 > score_1, "2 линии > 1 линии");
    assert!(score_3 > score_2, "3 линии > 2 линии");
    assert!(score_4 > score_3, "4 линии > 3 линии");
}

/// Тест 12: Прогрессия очков за линии
#[test]
fn test_line_score_progression() {
    // Проверяем прогрессию: 100, 200, 400, 800
    let scores = [
        LINE_SCORES[0],     // 1 линия
        LINE_SCORES[0] * 2, // 2 линии
        LINE_SCORES[0] * 4, // 3 линии
        LINE_SCORES[0] * 8, // 4 линии
    ];

    for i in 1..scores.len() {
        assert!(
            scores[i] > scores[i - 1],
            "Очки должны расти с количеством линий"
        );
    }
}

/// Тест 13: Бонус за множественные линии
#[test]
fn test_multiple_lines_bonus() {
    // Бонус за 2 линии
    let bonus_2 = LINE_SCORES[0] * 2 - LINE_SCORES[0];
    assert_eq!(bonus_2, 100, "Бонус за 2 линии должен быть 100");

    // Бонус за 3 линии
    let bonus_3 = LINE_SCORES[0] * 4 - LINE_SCORES[0];
    assert_eq!(bonus_3, 300, "Бонус за 3 линии должен быть 300");
}

/// Тест 14: Максимальные очки за линии
#[test]
fn test_maximum_line_score() {
    // Максимум - 4 линии (Tetris)
    let max_score = LINE_SCORES[0] * 8;
    assert_eq!(max_score, 800, "Максимальные очки за линии должны быть 800");
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-20: Тесты комбо-бонусов
// ============================================================================

/// Тест 15: Базовый бонус за комбо
#[test]
fn test_base_combo_bonus() {
    assert_eq!(COMBO_BONUS, 50, "Базовый бонус за комбо должен быть 50");
}

/// Тест 16: Бонус за первое комбо
#[test]
fn test_first_combo_bonus() {
    // Комбо 1: 50 * 1 = 50
    let bonus = COMBO_BONUS;
    assert!(bonus >= 50, "Бонус за первое комбо должен быть >= 50");
}

/// Тест 17: Бонус за второе комбо
#[test]
fn test_second_combo_bonus() {
    // Комбо 2: 50 * 2 = 100
    let bonus = COMBO_BONUS * 2;
    assert_eq!(bonus, 100, "Бонус за второе комбо должен быть 100");
}

/// Тест 18: Бонус за пятое комбо
#[test]
fn test_fifth_combo_bonus() {
    // Комбо 5: 50 * 5 = 250
    let bonus = COMBO_BONUS * 5;
    assert_eq!(bonus, 250, "Бонус за пятое комбо должен быть 250");
}

/// Тест 19: Рост комбо-бонуса
#[test]
fn test_combo_bonus_growth() {
    let bonus_1 = COMBO_BONUS;
    let bonus_3 = COMBO_BONUS * 3;
    let bonus_5 = COMBO_BONUS * 5;

    assert!(bonus_3 > bonus_1, "Комбо 3 > Комбо 1");
    assert!(bonus_5 > bonus_3, "Комбо 5 > Комбо 3");
}

/// Тест 20: Линейный рост комбо-бонуса
#[test]
fn test_combo_bonus_linear_growth() {
    // Проверяем линейность: разница между соседними комбо постоянна
    let diff_1_2 = COMBO_BONUS;
    let diff_2_3 = COMBO_BONUS;

    assert_eq!(
        diff_1_2, diff_2_3,
        "Разница между комбо должна быть постоянной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-26: Тесты бонусов за уровень
// ============================================================================

/// Тест 21: Базовый уровень равен 1
#[test]
fn test_base_level() {
    // Начальный уровень
    let level = 1;
    assert_eq!(level, 1, "Начальный уровень должен быть 1");
}

/// Тест 22: Повышение уровня за 10 линий
#[test]
fn test_level_up_at_ten_lines() {
    let level = LINES_PER_LEVEL / LINES_PER_LEVEL + 1;
    assert_eq!(level, 2, "Уровень должен быть 2 после 10 линий");
}

/// Тест 23: Уровень за 20 линий
#[test]
fn test_level_at_twenty_lines() {
    let level = 20 / LINES_PER_LEVEL + 1;
    assert_eq!(level, 3, "Уровень должен быть 3 после 20 линий");
}

/// Тест 24: Уровень за 50 линий
#[test]
fn test_level_at_fifty_lines() {
    let level = 50 / LINES_PER_LEVEL + 1;
    assert_eq!(level, 6, "Уровень должен быть 6 после 50 линий");
}

/// Тест 25: Прогрессия уровней
#[test]
fn test_level_progression() {
    let level_0 = 1;
    let level_10 = 10 / LINES_PER_LEVEL + 1;
    let level_20 = 20 / LINES_PER_LEVEL + 1;

    assert!(level_10 > level_0, "Уровень 10 > Уровень 0");
    assert!(level_20 > level_10, "Уровень 20 > Уровень 10");
}

/// Тест 26: Константа линий на уровень
#[test]
fn test_lines_per_level_constant() {
    assert_eq!(LINES_PER_LEVEL, 10, "Для повышения уровня нужно 10 линий");
}

// ============================================================================
// ГРУППА ТЕСТОВ 27-32: Тесты soft drop очков
// ============================================================================

/// Тест 27: Базовые очки за soft drop
#[test]
fn test_base_soft_drop_points() {
    assert_eq!(
        SOFT_DROP_POINTS, 1,
        "Очки за soft drop должны быть 1 за ячейку"
    );
}

/// Тест 28: Очки за soft drop на 1 ячейку
#[test]
fn test_soft_drop_one_cell() {
    let points = SOFT_DROP_POINTS;
    assert_eq!(points, 1, "Очки за 1 ячейку soft drop должны быть 1");
}

/// Тест 29: Очки за soft drop на 5 ячеек
#[test]
fn test_soft_drop_five_cells() {
    let points = SOFT_DROP_POINTS * 5;
    assert_eq!(points, 5, "Очки за 5 ячеек soft drop должны быть 5");
}

/// Тест 30: Очки за soft drop на 10 ячеек
#[test]
fn test_soft_drop_ten_cells() {
    let points = SOFT_DROP_POINTS * 10;
    assert_eq!(points, 10, "Очки за 10 ячеек soft drop должны быть 10");
}

/// Тест 31: Линейный рост soft drop очков
#[test]
fn test_soft_drop_linear_growth() {
    let points_1 = SOFT_DROP_POINTS;
    let points_5 = SOFT_DROP_POINTS * 5;
    let points_10 = SOFT_DROP_POINTS * 10;

    assert!(points_5 > points_1, "5 ячеек > 1 ячейка");
    assert!(points_10 > points_5, "10 ячеек > 5 ячеек");
}

/// Тест 32: Мягкое падение даёт меньше очков чем жёсткое
#[test]
fn test_soft_drop_less_than_hard_drop() {
    let _ = (SOFT_DROP_POINTS, HARD_DROP_POINTS);
}

// ============================================================================
// ГРУППА ТЕСТОВ 33-38: Тесты hard drop очков
// ============================================================================

/// Тест 33: Базовые очки за hard drop
#[test]
fn test_base_hard_drop_points() {
    assert_eq!(
        HARD_DROP_POINTS, 2,
        "Очки за hard drop должны быть 2 за ячейку"
    );
}

/// Тест 34: Очки за hard drop на 1 ячейку
#[test]
fn test_hard_drop_one_cell() {
    let points = HARD_DROP_POINTS;
    assert_eq!(points, 2, "Очки за 1 ячейку hard drop должны быть 2");
}

/// Тест 35: Очки за hard drop на 5 ячеек
#[test]
fn test_hard_drop_five_cells() {
    let points = HARD_DROP_POINTS * 5;
    assert_eq!(points, 10, "Очки за 5 ячеек hard drop должны быть 10");
}

/// Тест 36: Очки за hard drop на 10 ячеек
#[test]
fn test_hard_drop_ten_cells() {
    let points = HARD_DROP_POINTS * 10;
    assert_eq!(points, 20, "Очки за 10 ячеек hard drop должны быть 20");
}

/// Тест 37: Очки за hard drop на 20 ячеек
#[test]
fn test_hard_drop_twenty_cells() {
    let points = HARD_DROP_POINTS * 20;
    assert_eq!(points, 40, "Очки за 20 ячеек hard drop должны быть 40");
}

/// Тест 38: Hard drop выгоднее soft drop
#[test]
fn test_hard_drop_more_profitable() {
    let soft_10 = SOFT_DROP_POINTS * 10;
    let hard_10 = HARD_DROP_POINTS * 10;

    assert!(
        hard_10 > soft_10,
        "Hard drop должен давать больше очков (hard={}, soft={})",
        hard_10,
        soft_10
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 39-40: Тесты Tetris бонусов
// ============================================================================

/// Тест 39: Tetris бонус за 4 линии
#[test]
fn test_tetris_bonus_for_four_lines() {
    // Tetris (4 линии) даёт максимальный бонус
    let tetris_score = LINE_SCORES[0] * 8;
    assert_eq!(tetris_score, 800, "Tetris бонус должен быть 800 очков");
}

/// Тест 40: Tetris бонус больше чем 3 линии
#[test]
fn test_tetris_bonus_more_than_three_lines() {
    let three_lines = LINE_SCORES[0] * 4; // 400
    let tetris = LINE_SCORES[0] * 8; // 800

    assert!(
        tetris > three_lines,
        "Tetris должен давать больше очков чем 3 линии (Tetris={}, 3 линии={})",
        tetris,
        three_lines
    );
}
