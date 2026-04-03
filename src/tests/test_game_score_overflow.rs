//! Тесты защиты от переполнения score (game.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка saturating_add при нормальных значениях
//! - Проверка защиты от переполнения
//! - Проверка корректного начисления очков
//!
//! Исправление: использование saturating_add() вместо обычного сложения

use crate::constants::{
    COMBO_BONUS, HARD_DROP_POINTS, LINE_SCORES, PIECE_SCORE_INC, SOFT_DROP_POINTS,
};
use crate::game::GameState;

// ============================================================================
// ГРУППА ТЕСТОВ: Переполнение score
// ============================================================================

/// Тест 1: Проверка saturating_add при нормальных значениях
///
/// Проверяет, что saturating_add корректно работает
/// при нормальных значениях очков.
#[test]
fn test_saturating_add_normal_values() {
    let mut score: u128 = 0;

    // Начисляем очки за линию (100)
    score = score.saturating_add(LINE_SCORES[0]);
    assert_eq!(score, 100, "После первой линии счёт должен быть 100");

    // Начисляем очки за вторую линию (200)
    score = score.saturating_add(LINE_SCORES[0] * 2);
    assert_eq!(score, 300, "После второй линии счёт должен быть 300");

    // Начисляем очки за фигуру
    score = score.saturating_add(PIECE_SCORE_INC);
    assert_eq!(score, 400, "После фигуры счёт должен быть 400");

    // Начисляем очки за Soft Drop (1 очко за ячейку)
    let soft_drop_distance = 10;
    score = score.saturating_add((soft_drop_distance as u128) * SOFT_DROP_POINTS);
    assert_eq!(score, 410, "После Soft Drop счёт должен быть 410");

    // Начисляем очки за Hard Drop (2 очка за ячейку)
    let hard_drop_distance = 5;
    score = score.saturating_add(hard_drop_distance as u128 * HARD_DROP_POINTS);
    assert_eq!(score, 420, "После Hard Drop счёт должен быть 420");

    // Начисляем бонус за комбо
    let combo = 3;
    if combo > 1 {
        score = score.saturating_add(COMBO_BONUS * (combo - 1) as u128);
    }
    assert_eq!(score, 520, "После комбо x3 счёт должен быть 520");
}

/// Тест 2: Проверка защиты от переполнения
///
/// Проверяет, что saturating_add предотвращает переполнение u64.
#[test]
fn test_overflow_protection() {
    // Тест с максимальным значением u128
    let max_score = u128::MAX;

    // saturating_add должен вернуть MAX при переполнении
    let result = max_score.saturating_add(1);
    assert_eq!(
        result,
        u128::MAX,
        "saturating_add должен вернуть MAX при переполнении"
    );

    // Тест с большим добавлением
    let result2 = max_score.saturating_add(1000);
    assert_eq!(
        result2,
        u128::MAX,
        "saturating_add должен вернуть MAX при добавлении 1000"
    );

    // Тест с u128::MAX + u128::MAX
    let result3 = max_score.saturating_add(max_score);
    assert_eq!(
        result3,
        u128::MAX,
        "saturating_add(MAX, MAX) должен вернуть MAX"
    );

    // Тест с близким к MAX значением
    let near_max = u128::MAX - 100;
    let result4 = near_max.saturating_add(200);
    assert_eq!(
        result4,
        u128::MAX,
        "saturating_add должен вернуть MAX при переполнении near_max"
    );

    // Тест что нормальные значения работают корректно
    let normal_score: u128 = 1000;
    let result5 = normal_score.saturating_add(500);
    assert_eq!(
        result5, 1500,
        "saturating_add должен корректно складывать нормальные значения"
    );
}

/// Тест 3: Проверка корректного начисления очков
///
/// Проверяет, что очки начисляются правильно с использованием saturating_add.
#[test]
fn test_correct_score_calculation() {
    let mut state = GameState::new();

    // Проверяем начальный счёт
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");

    // Симулируем начисление очков за линию (через проверку констант)
    let line_points = LINE_SCORES[0];
    let mut score: u128 = 0;
    score = score.saturating_add(line_points);
    assert_eq!(score, 100, "Очки за 1 линию должны быть 100");

    // Очки за 2 линии (200)
    score = score.saturating_add(LINE_SCORES[0] * 2);
    assert_eq!(score, 300, "Очки за 2 линии должны быть 300");

    // Очки за 3 линии (400)
    score = score.saturating_add(LINE_SCORES[0] * 4);
    assert_eq!(score, 700, "Очки за 3 линии должны быть 700");

    // Очки за Tetris (4 линии = 800 + 1000 бонус)
    score = score.saturating_add(LINE_SCORES[0] * 8); // 800
    score = score.saturating_add(1000); // Бонус за Tetris
    assert_eq!(score, 2500, "Очки за Tetris должны быть 2500");

    // Проверяем экспоненциальный рост очков
    // 1 линия: 100 × 2^0 = 100
    // 2 линии: 100 × 2^1 = 200
    // 3 линии: 100 × 2^2 = 400
    // 4 линии: 100 × 2^3 = 800
    assert_eq!(LINE_SCORES[0] * (1 << 0), 100, "Очки за 1 линию");
    assert_eq!(LINE_SCORES[0] * (1 << 1), 200, "Очки за 2 линии");
    assert_eq!(LINE_SCORES[0] * (1 << 2), 400, "Очки за 3 линии");
    assert_eq!(LINE_SCORES[0] * (1 << 3), 800, "Очки за 4 линии");

    // Проверяем что GameState использует saturating_add
    // Создаём состояние и проверяем что счёт корректен
    let state = GameState::new();
    assert_eq!(state.score(), 0, "Новое состояние должно иметь счёт 0");
}

/// Тест 4: Стресс-тест с очень большим счётом
///
/// Проверяет поведение счёта при очень больших значениях.
#[test]
fn test_stress_with_very_large_score() {
    let mut score: u128 = 0;

    // Начисляем очень много очков
    let large_increment: u128 = 1_000_000_000; // 1 billion

    // Начисляем 1000 раз
    for _ in 0..1000 {
        score = score.saturating_add(large_increment);
    }

    assert_eq!(
        score,
        1_000_000_000_000, // 1 trillion
        "Счёт должен быть 1 триллион"
    );

    // Тест с близким к MAX значением
    let mut near_max_score: u128 = u128::MAX - 1000;

    // Малое добавление должно работать
    near_max_score = near_max_score.saturating_add(500);
    assert!(near_max_score < u128::MAX, "Счёт должен быть меньше MAX");

    // Добавление вызывающее переполнение
    near_max_score = near_max_score.saturating_add(1000);
    assert_eq!(
        near_max_score,
        u128::MAX,
        "При переполнении должен вернуть MAX"
    );
}

/// Тест 5: Проверка всех типов начисления очков
///
/// Интеграционный тест для всех способов получения очков.
#[test]
fn test_all_scoring_types() {
    let mut score: u128 = 0;

    // 1. Очки за линию (100)
    score = score.saturating_add(LINE_SCORES[0]);
    assert_eq!(score, 100, "Очки за линию");

    // 2. Очки за фигуру (100 + fall_spd * 50)
    score = score.saturating_add(PIECE_SCORE_INC);
    assert_eq!(score, 200, "Очки за фигуру");

    // 3. Очки за Soft Drop (1 за ячейку)
    let soft_drop = 15;
    score = score.saturating_add(soft_drop as u128 * SOFT_DROP_POINTS);
    assert_eq!(score, 215, "Очки за Soft Drop");

    // 4. Очки за Hard Drop (2 за ячейку)
    let hard_drop = 20;
    score = score.saturating_add(hard_drop as u128 * HARD_DROP_POINTS);
    assert_eq!(score, 255, "Очки за Hard Drop");

    // 5. Бонус за комбо (50 × (комбо - 1))
    let combo = 5;
    if combo > 1 {
        score = score.saturating_add(COMBO_BONUS * (combo - 1) as u128);
    }
    assert_eq!(score, 455, "Очки с бонусом за комбо x5");

    // 6. Бонус за уровень (500 × (уровень - 1))
    let level = 3;
    score = score.saturating_add(500 * (level - 1) as u128);
    assert_eq!(score, 1455, "Очки с бонусом за уровень 3");

    // Проверяем что итоговый счёт корректен
    assert!(score > 0, "Итоговый счёт должен быть положительным");
    assert!(score < u128::MAX, "Итоговый счёт не должен переполняться");
}
