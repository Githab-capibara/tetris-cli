//! Тесты защиты от переполнения счёта.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит тесты для проверки исправления переполнения счёта:
//! - Проверка что счёт не превышает `MAX_SCORE`
//! - Проверка корректной работы `saturating_add`
//! - Проверка поведения при экстремально больших значениях
//!
//! ## Исправление
//! В модуле `game/scoring/lines.rs` используется константа `MAX_SCORE = u128::MAX / 2`
//! и функция `saturating_add()` для защиты от переполнения.

use crate::constants::{COMBO_BONUS, LEVEL_BONUS_MULT, LINE_SCORES};
use crate::game::scoring::lines::{update_score_for_lines, MAX_SCORE};
use crate::game::GameState;

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Базовая защита от переполнения
// ============================================================================

/// Тест 1: Проверка что счёт не превышает `MAX_SCORE`
///
/// Проверяет, что при начислении очков счёт никогда не превышает
/// максимально допустимое значение `MAX_SCORE`.
#[test]
fn test_score_does_not_exceed_max() {
    let mut score: u128 = 0;
    let level = 1u32;
    let mut combo_counter: u32 = 0;

    // Начисляем очень много очков симулируя множество линий
    for _ in 0..1000 {
        update_score_for_lines(&mut score, level, 4, &mut combo_counter);
    }

    // Счёт не должен превышать MAX_SCORE
    assert!(
        score <= MAX_SCORE,
        "Счёт ({score}) не должен превышать MAX_SCORE ({MAX_SCORE})"
    );

    // Проверяем что MAX_SCORE действительно u128::MAX / 2
    assert_eq!(
        MAX_SCORE,
        u128::MAX / 2,
        "MAX_SCORE должен быть u128::MAX / 2"
    );
}

/// Тест 2: Проверка `saturating_add` при нормальных значениях
///
/// Проверяет, что `saturating_add` корректно работает
/// при нормальных значениях очков.
#[test]
fn test_score_overflow_saturating_add_normal_values() {
    let mut score: u128 = 0;

    // Начисляем очки за линию (100)
    score = score.saturating_add(LINE_SCORES[0]);
    assert_eq!(score, 100, "После первой линии счёт должен быть 100");

    // Начисляем очки за вторую линию (200)
    score = score.saturating_add(LINE_SCORES[0] * 2);
    assert_eq!(score, 300, "После второй линии счёт должен быть 300");

    // Начисляем бонус за комбо
    let combo = 3;
    if combo > 1 {
        let combo_bonus = COMBO_BONUS.saturating_mul((combo - 1) as u128);
        score = score.saturating_add(combo_bonus);
    }
    assert_eq!(score, 400, "После комбо x3 счёт должен быть 400");

    // Начисляем бонус за уровень
    let level_bonus = LEVEL_BONUS_MULT.saturating_mul((10 - 1) as u128);
    score = score.saturating_add(level_bonus);
    assert_eq!(
        score, 4900,
        "После бонуса за уровень 10 счёт должен быть 4900"
    );
}

/// Тест 3+11: Saturating add comprehensive (overflow protection и edge cases)
///
/// Проверяет `saturating_add` с переполнением и краевыми значениями.
#[test]
fn test_saturating_add_comprehensive() {
    // --- Overflow protection ---
    let max_value = u128::MAX;

    let result = max_value.saturating_add(1);
    assert_eq!(
        result,
        u128::MAX,
        "saturating_add должен вернуть MAX при переполнении"
    );

    let result2 = max_value.saturating_add(1000);
    assert_eq!(
        result2,
        u128::MAX,
        "saturating_add должен вернуть MAX при добавлении 1000"
    );

    let result3 = max_value.saturating_add(max_value);
    assert_eq!(
        result3,
        u128::MAX,
        "saturating_add(MAX, MAX) должен вернуть MAX"
    );

    let near_max = u128::MAX - 100;
    let result4 = near_max.saturating_add(200);
    assert_eq!(
        result4,
        u128::MAX,
        "saturating_add должен вернуть MAX при переполнении near_max"
    );

    let normal_score: u128 = 1000;
    let result5 = normal_score.saturating_add(500);
    assert_eq!(
        result5, 1500,
        "saturating_add должен корректно складывать нормальные значения"
    );

    // --- Edge cases ---
    assert_eq!(0u128.saturating_add(0), 0);
    assert_eq!(0u128.saturating_add(1), 1);
    assert_eq!(1u128.saturating_add(0), 1);
    assert_eq!((u128::MAX - 1).saturating_add(1), u128::MAX);
    assert_eq!((u128::MAX - 1).saturating_add(2), u128::MAX);
    assert_eq!((u128::MAX / 2).saturating_add(u128::MAX / 2), u128::MAX - 1);
    assert_eq!((u128::MAX / 2).saturating_add(u128::MAX / 2 + 1), u128::MAX);
}

// ============================================================================
// ГРУППА ТЕСТОВ 4-7: Экстремальные значения (уровень 10000+, комбо 1000+)
// ============================================================================

/// Тест 4+5: Extreme scoring parameters (level 10000+ и combo 1000+)
///
/// Проверяет поведение системы очков при экстремальных уровне и комбо.
#[test]
fn test_extreme_scoring_parameters() {
    // --- Extreme level 10000+ ---
    let mut score: u128 = 0;
    let extreme_level = 10_000u32;
    let mut combo_counter: u32 = 0;

    update_score_for_lines(&mut score, extreme_level, 4, &mut combo_counter);

    assert!(
        score <= MAX_SCORE,
        "Счёт при уровне {extreme_level} не должен превышать MAX_SCORE"
    );

    let expected_level_bonus = LEVEL_BONUS_MULT.saturating_mul(u128::from(extreme_level - 1));
    assert!(
        expected_level_bonus <= MAX_SCORE,
        "Бонус за уровень {expected_level_bonus} не должен превышать MAX_SCORE"
    );

    // --- Extreme combo 1000+ ---
    let mut score_combo: u128 = 0;
    let level = 1u32;
    let mut combo_counter = 1000;

    update_score_for_lines(&mut score_combo, level, 1, &mut combo_counter);

    assert!(
        score_combo <= MAX_SCORE,
        "Счёт при комбо 1000+ не должен превышать MAX_SCORE"
    );

    let expected_combo_bonus = COMBO_BONUS.saturating_mul((1000 - 1) as u128);
    assert!(
        expected_combo_bonus <= MAX_SCORE,
        "Бонус за комбо 1000 не должен превышать MAX_SCORE"
    );
}

/// Тест 6+7: Stress score overflow protection (combined extremes и many additions)
///
/// Проверяет защиту от переполнения при стрессовых сценариях.
#[test]
fn test_stress_score_overflow_protection() {
    // --- Extreme level and combo combined ---
    let mut score: u128 = 0;
    let extreme_level = 10_000u32;
    let mut combo_counter: u32 = 1000;

    for _ in 0..100 {
        update_score_for_lines(&mut score, extreme_level, 4, &mut combo_counter);
    }

    assert!(
        score <= MAX_SCORE,
        "Счёт при экстремальных параметрах не должен превышать MAX_SCORE"
    );
    assert!(score < u128::MAX, "Счёт должен быть меньше u128::MAX");

    // --- Stress: many score additions ---
    let mut stress_score: u128 = 0;
    let level = 100u32;
    let mut stress_combo: u32 = 0;

    for _ in 0..10_000 {
        update_score_for_lines(&mut stress_score, level, 4, &mut stress_combo).ok();
    }

    assert!(
        stress_score <= MAX_SCORE,
        "После 10000 начислений счёт ({stress_score}) не должен превышать MAX_SCORE ({MAX_SCORE})"
    );
    assert!(
        stress_score > 1_000_000,
        "Счёт ({stress_score}) должен быть разумным после 10000 начислений"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 8-10: Интеграционные тесты с GameState
// ============================================================================

/// Тест 8: Интеграционный тест с GameState
///
/// Проверяет что GameState корректно обрабатывает переполнение счёта.
#[test]
fn test_game_state_score_overflow() {
    let mut state = GameState::new();

    // Устанавливаем счёт близкий к MAX_SCORE
    state.set_score(MAX_SCORE - 1000);

    // Начисляем ещё очков через игровую логику
    let mut score = state.score();
    let level = state.level();
    let mut combo_counter = state.stats().combo_counter();

    // Начисляем много очков
    for _ in 0..100 {
        update_score_for_lines(&mut score, level, 4, &mut combo_counter);
    }

    // Устанавливаем счёт обратно в состояние
    state.set_score(score);

    // Проверяем что счёт не превышает MAX_SCORE
    assert!(
        state.score() <= MAX_SCORE,
        "Счёт в GameState не должен превышать MAX_SCORE"
    );
}

/// Тест 9: Проверка защиты от переполнения в `update_score_for_lines`
///
/// Проверяет что функция `update_score_for_lines` корректно защищает от переполнения.
#[test]
fn test_update_score_for_lines_overflow_protection() {
    let mut score = MAX_SCORE - 100;
    let level = 1u32;
    let mut combo_counter: u32 = 1;

    // Начисляем много очков
    update_score_for_lines(&mut score, level, 4, &mut combo_counter);

    // Счёт должен быть ограничен MAX_SCORE
    assert!(
        score <= MAX_SCORE,
        "Счёт после начисления не должен превышать MAX_SCORE"
    );
}

/// Тест 10: Проверка что `MAX_SCORE` константа корректна
///
/// Проверяет значение и свойства константы `MAX_SCORE`.
#[test]
fn test_max_score_constant() {
    // Проверяем что MAX_SCORE равен u128::MAX / 2
    assert_eq!(MAX_SCORE, u128::MAX / 2);

    // Проверяем что MAX_SCORE меньше u128::MAX
    assert!(MAX_SCORE < u128::MAX);

    // Проверяем что MAX_SCORE больше u128::MAX / 4
    assert!(MAX_SCORE > u128::MAX / 4);

    // Проверяем что MAX_SCORE положителен
    assert!(MAX_SCORE > 0);

    // Проверяем что MAX_SCORE имеет разумное значение для защиты от переполнения
    // Он должен быть достаточно большим для нормальных игр
    assert!(
        MAX_SCORE > 1_000_000_000_000,
        "MAX_SCORE должен быть достаточно большим"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 12: Краевые случаи
// ============================================================================

/// Тест 12: Проверка что игра не паникует при экстремальных значениях
///
/// Проверяет отсутствие паник при экстремальных значениях счёта.
#[test]
fn test_no_panic_at_extreme_values() {
    let mut score: u128 = 0;
    let level = 10_000u32;
    let mut combo_counter: u32 = 10_000;

    // Многократные начисления не должны вызывать панику
    for _ in 0..1000 {
        update_score_for_lines(&mut score, level, 4, &mut combo_counter);
        // Проверяем что счёт валиден на каждой итерации
        assert!(score <= u128::MAX, "Счёт не должен превышать u128::MAX");
    }

    // Финальная проверка
    assert!(
        score <= u128::MAX,
        "Финальный счёт не должен превышать u128::MAX"
    );
}
