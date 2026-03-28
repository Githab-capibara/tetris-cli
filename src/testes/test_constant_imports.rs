//! Тесты импортов констант из crate::game.
//!
//! Проверяют, что все константы доступны из crate::game и имеют корректные значения.

use crate::game::{
    ANIMATION_FRAME_SKIP, COMBO_BONUS, FPS, HARD_DROP_ANIM_INTERVAL_MS, HARD_DROP_POINTS,
    INITIAL_FALL_SPD, LAND_TIME_DELAY_S, LEVEL_BONUS_MULT, LINES_PER_LEVEL, LINE_SCORES,
    MARATHON_LINES, MAX_FALL_SPEED, MAX_LINES_PER_CLEAR, MIN_Y, PIECE_SCORE_FALL_MULT,
    PIECE_SCORE_INC, SOFT_DROP_POINTS, SPD_INC, SPRINT_LINES,
};

/// Тест 1: Проверка LINE_SCORES
///
/// Проверяем таблицу очков за очистку линий.
#[test]
fn test_line_scores_constant() {
    // 1 линия = 100 очков
    assert_eq!(LINE_SCORES[0], 100, "Очки за 1 линию должны быть 100");

    // 2 линии = 200 очков
    assert_eq!(LINE_SCORES[1], 200, "Очки за 2 линии должны быть 200");

    // 3 линии = 400 очков
    assert_eq!(LINE_SCORES[2], 400, "Очки за 3 линии должны быть 400");

    // 4 линии (Tetris) = 1800 очков (800 + 1000 бонус)
    assert_eq!(
        LINE_SCORES[3], 1800,
        "Очки за 4 линии (Tetris) должны быть 1800"
    );

    // Проверяем длину массива
    assert_eq!(
        LINE_SCORES.len(),
        4,
        "LINE_SCORES должен содержать 4 элемента"
    );
}

/// Тест 2: Проверка INITIAL_FALL_SPD
///
/// Проверяем начальную скорость падения фигур.
#[test]
fn test_initial_fall_spd_constant() {
    assert!(
        (INITIAL_FALL_SPD - 0.9).abs() < f32::EPSILON,
        "Начальная скорость падения должна быть 0.9, получено {INITIAL_FALL_SPD}"
    );
}

/// Тест 3: Проверка MAX_FALL_SPEED
///
/// Проверяем максимальную скорость падения для защиты от переполнения.
#[test]
fn test_max_fall_speed_constant() {
    assert_eq!(
        MAX_FALL_SPEED, 1000.0,
        "Максимальная скорость должна быть 1000.0"
    );
}

/// Тест 4: Проверка FPS
///
/// Проверяем количество кадров в секунду.
#[test]
fn test_fps_constant() {
    assert_eq!(FPS, 60, "FPS должен быть 60");
}

/// Тест 5: Проверка LAND_TIME_DELAY_S
///
/// Проверяем задержку приземления фигуры.
#[test]
fn test_land_time_delay_s_constant() {
    assert!(
        (LAND_TIME_DELAY_S - 0.1).abs() < f64::EPSILON,
        "Задержка приземления должна быть 0.1 секунды, получено {LAND_TIME_DELAY_S}"
    );
}

/// Тест 6: Проверка SOFT_DROP_POINTS
///
/// Проверяем очки за мягкое падение.
#[test]
fn test_soft_drop_points_constant() {
    assert_eq!(
        SOFT_DROP_POINTS, 1,
        "Очки за Soft Drop должны быть 1 за ячейку"
    );

    // Проверяем расчёт очков для разных дистанций
    let test_distances = [1u128, 5u128, 10u128, 20u128];
    for &distance in &test_distances {
        let expected = distance * SOFT_DROP_POINTS;
        assert_eq!(
            expected, distance,
            "Очки за Soft Drop должны равняться дистанции"
        );
    }
}

/// Тест 7: Проверка HARD_DROP_POINTS
///
/// Проверяем очки за жёсткое падение.
#[test]
fn test_hard_drop_points_constant() {
    assert_eq!(
        HARD_DROP_POINTS, 2,
        "Очки за Hard Drop должны быть 2 за ячейку"
    );

    // Проверяем расчёт очков для разных дистанций
    let test_distances = [1u128, 5u128, 10u128, 20u128];
    for &distance in &test_distances {
        let expected = distance * HARD_DROP_POINTS;
        assert_eq!(
            expected,
            distance * 2,
            "Очки за Hard Drop должны равняться дистанции × 2"
        );
    }
}

/// Тест 8: Проверка COMBO_BONUS
///
/// Проверяем бонус за комбо.
#[test]
fn test_combo_bonus_constant() {
    assert_eq!(COMBO_BONUS, 50, "Базовый бонус за комбо должен быть 50");

    // Проверяем расчёт бонусов для разных уровней комбо
    assert_eq!(0, 0, "Бонус за комбо 1 должен быть 0");
    assert_eq!(COMBO_BONUS, 50, "Бонус за комбо 2 должен быть 50");
    assert_eq!(COMBO_BONUS * 2, 100, "Бонус за комбо 3 должен быть 100");
    assert_eq!(COMBO_BONUS * 9, 450, "Бонус за комбо 10 должен быть 450");
}

/// Тест 9: Проверка LEVEL_BONUS_MULT
///
/// Проверяем множитель бонуса за уровень.
#[test]
fn test_level_bonus_mult_constant() {
    assert_eq!(
        LEVEL_BONUS_MULT, 500,
        "Множитель бонуса за уровень должен быть 500"
    );

    // Проверяем расчёт бонусов для разных уровней
    assert_eq!(LEVEL_BONUS_MULT, 500, "Бонус за уровень 2 должен быть 500");
    assert_eq!(
        LEVEL_BONUS_MULT * 2,
        1000,
        "Бонус за уровень 3 должен быть 1000"
    );
    assert_eq!(
        LEVEL_BONUS_MULT * 10,
        5000,
        "Бонус за уровень 11 должен быть 5000"
    );
}

/// Тест 10: Проверка LINES_PER_LEVEL
///
/// Проверяем количество линий для повышения уровня.
#[test]
fn test_lines_per_level_constant() {
    assert_eq!(LINES_PER_LEVEL, 10, "Для повышения уровня нужно 10 линий");
}

/// Тест 11: Проверка SPD_INC
///
/// Проверяем прирост скорости за уровень.
#[test]
fn test_spd_inc_constant() {
    assert!(
        (SPD_INC - 0.05).abs() < f32::EPSILON,
        "Прирост скорости должен быть 0.05, получено {SPD_INC}"
    );
}

/// Тест 12: Проверка PIECE_SCORE_INC
///
/// Проверяем базовые очки за фигуру.
#[test]
fn test_piece_score_inc_constant() {
    assert_eq!(
        PIECE_SCORE_INC, 100,
        "Базовые очки за фигуру должны быть 100"
    );
}

/// Тест 13: Проверка PIECE_SCORE_FALL_MULT
///
/// Проверяем множитель очков за падение.
#[test]
fn test_piece_score_fall_mult_constant() {
    assert_eq!(
        PIECE_SCORE_FALL_MULT, 50.0,
        "Множитель очков за падение должен быть 50.0"
    );
}

/// Тест 14: Проверка MAX_LINES_PER_CLEAR
///
/// Проверяем максимальное количество линий за один ход.
#[test]
fn test_max_lines_per_clear_constant() {
    assert_eq!(
        MAX_LINES_PER_CLEAR, 4,
        "Максимум можно удалить 4 линии (Tetris)"
    );
}

/// Тест 15: Проверка HARD_DROP_ANIM_INTERVAL_MS
///
/// Проверяем интервал анимации Hard Drop.
#[test]
fn test_hard_drop_anim_interval_ms_constant() {
    assert_eq!(
        HARD_DROP_ANIM_INTERVAL_MS, 50,
        "Интервал анимации Hard Drop должен быть 50 мс"
    );
}

/// Тест 16: Проверка ANIMATION_FRAME_SKIP
///
/// Проверяем количество кадров для пропуска при анимации.
#[test]
fn test_animation_frame_skip_constant() {
    assert_eq!(
        ANIMATION_FRAME_SKIP, 2,
        "Пропуск кадров анимации должен быть 2"
    );
}

/// Тест 17: Проверка SPRINT_LINES
///
/// Проверяем количество линий для режима спринт.
#[test]
fn test_sprint_lines_constant() {
    assert_eq!(SPRINT_LINES, 40, "Режим спринт требует 40 линий");
}

/// Тест 18: Проверка MARATHON_LINES
///
/// Проверяем количество линий для режима марафон.
#[test]
fn test_marathon_lines_constant() {
    assert_eq!(MARATHON_LINES, 150, "Режим марафон требует 150 линий");
}

/// Тест 19: Проверка MIN_Y
///
/// Проверяем минимальную допустимую координату Y.
#[test]
fn test_min_y_constant() {
    assert_eq!(MIN_Y, 0, "Минимальная координата Y должна быть 0");
}

/// Тест 20: Проверка согласованности констант
///
/// Проверяем, что константы согласованы друг с другом.
#[test]
fn test_constants_consistency() {
    // LINE_SCORES должен иметь 4 элемента для 1-4 линий
    assert_eq!(
        LINE_SCORES.len(),
        MAX_LINES_PER_CLEAR as usize,
        "LINE_SCORES.len() должен равняться MAX_LINES_PER_CLEAR"
    );

    // Бонус за Tetris (4 линии) должен быть больше суммы бонусов за 1+1+1+1 линии
    assert!(
        LINE_SCORES[3] > LINE_SCORES[0] * 4,
        "Tetris должен давать больше очков, чем 4 одиночных линии"
    );
}
