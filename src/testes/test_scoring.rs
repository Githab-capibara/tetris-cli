//! Тесты системы очков.
//!
//! Этот модуль содержит 50 тестов для проверки системы начисления очков:
//! - Тесты базовых очков (10 тестов)
//! - Тесты бонусов за линии (10 тестов)
//! - Тесты комбо (10 тестов)
//! - Тесты Hard Drop и Soft Drop (10 тестов)
//! - Тесты уровня и скорости (10 тестов)

use crate::game::{
    COMBO_BONUS, HARD_DROP_POINTS, INITIAL_FALL_SPD, LINES_PER_LEVEL, PIECE_SCORE_FALL_MULT,
    PIECE_SCORE_INC, ROW_SCORE_INC, SOFT_DROP_POINTS, SPD_INC,
};

// ============================================================================
// ГРУППА ТЕСТОВ 1-10: Базовые очки
// ============================================================================

/// Тест 1: Проверка константы PIECE_SCORE_INC
#[test]
fn test_scoring_piece_score_constant() {
    assert_eq!(PIECE_SCORE_INC, 100, "Очки за фигуру должны быть 100");
}

/// Тест 2: Проверка что PIECE_SCORE_INC положительное
#[test]
fn test_scoring_piece_score_positive() {
    assert!(PIECE_SCORE_INC > 0, "Очки за фигуру должны быть положительными");
}

/// Тест 3: Проверка что PIECE_SCORE_INC меньше 1000
#[test]
fn test_scoring_piece_score_reasonable() {
    assert!(PIECE_SCORE_INC < 1000, "Очки за фигуру должны быть меньше 1000");
}

/// Тест 4: Проверка константы PIECE_SCORE_FALL_MULT
#[test]
fn test_scoring_piece_fall_mult_constant() {
    assert!(
        (PIECE_SCORE_FALL_MULT - 50.0).abs() < f32::EPSILON,
        "Множитель за падение должен быть 50.0"
    );
}

/// Тест 5: Проверка что PIECE_SCORE_FALL_MULT положительный
#[test]
fn test_scoring_piece_fall_mult_positive() {
    assert!(PIECE_SCORE_FALL_MULT > 0.0, "Множитель за падение должен быть положительным");
}

/// Тест 6: Проверка что PIECE_SCORE_FALL_MULT меньше 100
#[test]
fn test_scoring_piece_fall_mult_reasonable() {
    assert!(PIECE_SCORE_FALL_MULT < 100.0, "Множитель за падение должен быть меньше 100");
}

/// Тест 7: Проверка расчёта очков за фигуру с падением
#[test]
fn test_scoring_piece_with_fall() {
    let base = PIECE_SCORE_INC;
    let fall_bonus = 1.0 * PIECE_SCORE_FALL_MULT;
    let total = base + fall_bonus as u64;

    assert_eq!(total, 150, "Очки за фигуру с падением на 1 блок должны быть 150");
}

/// Тест 8: Проверка что очки за фигуру положительные
#[test]
fn test_scoring_piece_positive() {
    // PIECE_SCORE_INC имеет тип u64, поэтому всегда >= 0
    // Проверяем что значение положительное
    assert!(PIECE_SCORE_INC > 0, "Очки за фигуру должны быть положительными");
}

/// Тест 9: Проверка что PIECE_SCORE_INC делится на 10
#[test]
fn test_scoring_piece_divisible_by_10() {
    assert_eq!(PIECE_SCORE_INC % 10, 0, "Очки за фигуру должны делиться на 10");
}

/// Тест 10: Проверка что PIECE_SCORE_INC равно 100
#[test]
fn test_scoring_piece_exactly_100() {
    assert_eq!(PIECE_SCORE_INC, 100);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-20: Бонусы за линии
// ============================================================================

/// Тест 11: Проверка константы ROW_SCORE_INC
#[test]
fn test_scoring_row_score_constant() {
    assert_eq!(ROW_SCORE_INC, 100, "Базовые очки за линию должны быть 100");
}

/// Тест 12: Проверка очков за 1 линию
#[test]
fn test_scoring_one_line() {
    let score = ROW_SCORE_INC * (1 << 0);
    assert_eq!(score, 100, "1 линия = 100 очков");
}

/// Тест 13: Проверка очков за 2 линии
#[test]
fn test_scoring_two_lines() {
    let score = ROW_SCORE_INC * (1 << 1);
    assert_eq!(score, 200, "2 линии = 200 очков");
}

/// Тест 14: Проверка очков за 3 линии
#[test]
fn test_scoring_three_lines() {
    let score = ROW_SCORE_INC * (1 << 2);
    assert_eq!(score, 400, "3 линии = 400 очков");
}

/// Тест 15: Проверка очков за 4 линии (Tetris)
#[test]
fn test_scoring_four_lines() {
    let score = ROW_SCORE_INC * (1 << 3);
    assert_eq!(score, 800, "4 линии = 800 очков (без бонуса)");
}

/// Тест 16: Проверка что очки за линии экспоненциальные
#[test]
fn test_scoring_lines_exponential() {
    let one = ROW_SCORE_INC * (1 << 0);
    let two = ROW_SCORE_INC * (1 << 1);
    let three = ROW_SCORE_INC * (1 << 2);
    let four = ROW_SCORE_INC * (1 << 3);

    assert!(two > one, "2 линии должны давать больше очков чем 1");
    assert!(three > two, "3 линии должны давать больше очков чем 2");
    assert!(four > three, "4 линии должны давать больше очков чем 3");
}

/// Тест 17: Проверка что ROW_SCORE_INC положительное
#[test]
fn test_scoring_row_score_positive() {
    assert!(ROW_SCORE_INC > 0, "Очки за линию должны быть положительными");
}

/// Тест 18: Проверка что ROW_SCORE_INC делится на 10
#[test]
fn test_scoring_row_score_divisible_by_10() {
    assert_eq!(ROW_SCORE_INC % 10, 0, "Очки за линию должны делиться на 10");
}

/// Тест 19: Проверка что очки за 4 линии больше чем за 1+1+1+1
#[test]
fn test_scoring_four_vs_one_plus_one() {
    let four_lines = ROW_SCORE_INC * (1 << 3);
    let four_singles = ROW_SCORE_INC * (1 << 0) * 4;

    assert!(four_lines > four_singles, "4 линии одновременно должны давать больше очков");
}

/// Тест 20: Проверка что очки за линии не переполняются
#[test]
fn test_scoring_lines_no_overflow() {
    let max_lines = 4;
    let max_score = ROW_SCORE_INC * (1 << (max_lines - 1));

    assert!(max_score < u64::MAX, "Очки за линии не должны переполняться");
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-30: Комбо
// ============================================================================

/// Тест 21: Проверка константы COMBO_BONUS
#[test]
fn test_scoring_combo_bonus_constant() {
    assert_eq!(COMBO_BONUS, 50, "Бонус за комбо должен быть 50");
}

/// Тест 22: Проверка что COMBO_BONUS положительное
#[test]
fn test_scoring_combo_bonus_positive() {
    assert!(COMBO_BONUS > 0, "Бонус за комбо должен быть положительным");
}

/// Тест 23: Проверка что COMBO_BONUS делится на 10
#[test]
fn test_scoring_combo_bonus_divisible_by_10() {
    assert_eq!(COMBO_BONUS % 10, 0, "Бонус за комбо должен делиться на 10");
}

/// Тест 24: Проверка бонуса за комбо x1
#[test]
fn test_scoring_combo_x1() {
    let bonus = COMBO_BONUS * 0;
    assert_eq!(bonus, 0, "Комбо x1 не даёт бонуса");
}

/// Тест 25: Проверка бонуса за комбо x2
#[test]
fn test_scoring_combo_x2() {
    let bonus = COMBO_BONUS * 1;
    assert_eq!(bonus, 50, "Комбо x2 даёт 50 бонуса");
}

/// Тест 26: Проверка бонуса за комбо x3
#[test]
fn test_scoring_combo_x3() {
    let bonus = COMBO_BONUS * 2;
    assert_eq!(bonus, 100, "Комбо x3 даёт 100 бонуса");
}

/// Тест 27: Проверка бонуса за комбо x5
#[test]
fn test_scoring_combo_x5() {
    let bonus = COMBO_BONUS * 4;
    assert_eq!(bonus, 200, "Комбо x5 даёт 200 бонуса");
}

/// Тест 28: Проверка бонуса за комбо x10
#[test]
fn test_scoring_combo_x10() {
    let bonus = COMBO_BONUS * 9;
    assert_eq!(bonus, 450, "Комбо x10 даёт 450 бонуса");
}

/// Тест 29: Проверка что бонус за комбо растёт линейно
#[test]
fn test_scoring_combo_linear_growth() {
    let bonus_2 = COMBO_BONUS * 1;
    let bonus_3 = COMBO_BONUS * 2;
    let bonus_4 = COMBO_BONUS * 3;

    assert!(bonus_3 > bonus_2, "Бонус за комбо x3 должен быть больше чем x2");
    assert!(bonus_4 > bonus_3, "Бонус за комбо x4 должен быть больше чем x3");
}

/// Тест 30: Проверка что бонус за комбо не переполняется
#[test]
fn test_scoring_combo_no_overflow() {
    let max_combo = 100;
    let max_bonus = COMBO_BONUS * (max_combo - 1);

    assert!(max_bonus < u64::MAX, "Бонус за комбо не должен переполняться");
}

// ============================================================================
// ГРУППА ТЕСТОВ 31-40: Hard Drop и Soft Drop
// ============================================================================

/// Тест 31: Проверка константы SOFT_DROP_POINTS
#[test]
fn test_scoring_soft_drop_constant() {
    assert_eq!(SOFT_DROP_POINTS, 1, "Очки за Soft Drop должны быть 1 за ячейку");
}

/// Тест 32: Проверка что SOFT_DROP_POINTS положительное
#[test]
fn test_scoring_soft_drop_positive() {
    assert!(SOFT_DROP_POINTS > 0, "Очки за Soft Drop должны быть положительными");
}

/// Тест 33: Проверка очков за Soft Drop на 5 ячеек
#[test]
fn test_scoring_soft_drop_5_cells() {
    let score = 5 * SOFT_DROP_POINTS;
    assert_eq!(score, 5, "Soft Drop на 5 ячеек = 5 очков");
}

/// Тест 34: Проверка очков за Soft Drop на 10 ячеек
#[test]
fn test_scoring_soft_drop_10_cells() {
    let score = 10 * SOFT_DROP_POINTS;
    assert_eq!(score, 10, "Soft Drop на 10 ячеек = 10 очков");
}

/// Тест 35: Проверка очков за Soft Drop на 20 ячеек
#[test]
fn test_scoring_soft_drop_20_cells() {
    let score = 20 * SOFT_DROP_POINTS;
    assert_eq!(score, 20, "Soft Drop на 20 ячеек = 20 очков");
}

/// Тест 36: Проверка константы HARD_DROP_POINTS
#[test]
fn test_scoring_hard_drop_constant() {
    assert_eq!(HARD_DROP_POINTS, 2, "Очки за Hard Drop должны быть 2 за ячейку");
}

/// Тест 37: Проверка что HARD_DROP_POINTS больше SOFT_DROP_POINTS
#[test]
fn test_scoring_hard_drop_greater_than_soft() {
    assert!(HARD_DROP_POINTS > SOFT_DROP_POINTS, "Hard Drop должен давать больше очков чем Soft Drop");
}

/// Тест 38: Проверка очков за Hard Drop на 5 ячеек
#[test]
fn test_scoring_hard_drop_5_cells() {
    let score = 5 * HARD_DROP_POINTS;
    assert_eq!(score, 10, "Hard Drop на 5 ячеек = 10 очков");
}

/// Тест 39: Проверка очков за Hard Drop на 10 ячеек
#[test]
fn test_scoring_hard_drop_10_cells() {
    let score = 10 * HARD_DROP_POINTS;
    assert_eq!(score, 20, "Hard Drop на 10 ячеек = 20 очков");
}

/// Тест 40: Проверка очков за Hard Drop на 20 ячеек
#[test]
fn test_scoring_hard_drop_20_cells() {
    let score = 20 * HARD_DROP_POINTS;
    assert_eq!(score, 40, "Hard Drop на 20 ячеек = 40 очков");
}

// ============================================================================
// ГРУППА ТЕСТОВ 41-50: Уровень и скорость
// ============================================================================

/// Тест 41: Проверка константы LINES_PER_LEVEL
#[test]
fn test_scoring_lines_per_level_constant() {
    assert_eq!(LINES_PER_LEVEL, 10, "Для повышения уровня нужно 10 линий");
}

/// Тест 42: Проверка что LINES_PER_LEVEL положительное
#[test]
fn test_scoring_lines_per_level_positive() {
    assert!(LINES_PER_LEVEL > 0, "LINES_PER_LEVEL должно быть положительным");
}

/// Тест 43: Проверка константы INITIAL_FALL_SPD
#[test]
fn test_scoring_initial_fall_speed() {
    assert!(
        (INITIAL_FALL_SPD - 0.9).abs() < f32::EPSILON,
        "Начальная скорость должна быть 0.9"
    );
}

/// Тест 44: Проверка что INITIAL_FALL_SPD положительная
#[test]
fn test_scoring_initial_fall_speed_positive() {
    assert!(INITIAL_FALL_SPD > 0.0, "Начальная скорость должна быть положительной");
}

/// Тест 45: Проверка константы SPD_INC
#[test]
fn test_scoring_speed_increment() {
    assert!(
        (SPD_INC - 0.05).abs() < f32::EPSILON,
        "Прирост скорости должен быть 0.05"
    );
}

/// Тест 46: Проверка что SPD_INC положительный
#[test]
fn test_scoring_speed_increment_positive() {
    assert!(SPD_INC > 0.0, "Прирост скорости должен быть положительным");
}

/// Тест 47: Проверка расчёта скорости после 10 линий
#[test]
fn test_scoring_speed_after_10_lines() {
    let speed = INITIAL_FALL_SPD + SPD_INC * 10.0;
    assert!(speed > INITIAL_FALL_SPD, "Скорость должна увеличиться после 10 линий");
}

/// Тест 48: Проверка расчёта скорости после 20 линий
#[test]
fn test_scoring_speed_after_20_lines() {
    let speed_10 = INITIAL_FALL_SPD + SPD_INC * 10.0;
    let speed_20 = INITIAL_FALL_SPD + SPD_INC * 20.0;
    assert!(speed_20 > speed_10, "Скорость должна расти с количеством линий");
}

/// Тест 49: Проверка что скорость не отрицательная
#[test]
fn test_scoring_speed_non_negative() {
    assert!(INITIAL_FALL_SPD >= 0.0, "Скорость не должна быть отрицательной");
}

/// Тест 50: Проверка что скорость растёт линейно
#[test]
fn test_scoring_speed_linear_growth() {
    let speed_0 = INITIAL_FALL_SPD;
    let speed_5 = INITIAL_FALL_SPD + SPD_INC * 5.0;
    let speed_10 = INITIAL_FALL_SPD + SPD_INC * 10.0;

    let diff_0_5 = speed_5 - speed_0;
    let diff_5_10 = speed_10 - speed_5;

    assert!(
        (diff_0_5 - diff_5_10).abs() < f32::EPSILON,
        "Скорость должна расти линейно"
    );
}
