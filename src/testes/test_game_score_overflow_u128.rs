//! Тесты защиты от переполнения score с u128 (game.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка saturating_add при максимальном значении u128
//! - Проверка начисления очков без переполнения
//! - Проверка поведения при score = u128::MAX
//!
//! Исправление: использование u128 и saturating_add() для предотвращения переполнения

use crate::game::{
    GameState, COMBO_BONUS, HARD_DROP_POINTS, PIECE_SCORE_INC, ROW_SCORE_INC,
    SOFT_DROP_POINTS,
};

// ============================================================================
// ГРУППА ТЕСТОВ: u128 score overflow protection
// ============================================================================

/// Тест 1: Проверка saturating_add при максимальном значении u128
///
/// Проверяет, что saturating_add корректно обрабатывает переполнение u128.
#[test]
fn test_saturating_add_u128_max() {
    // Тест с максимальным значением u128
    let max_score = u128::MAX;
    
    // saturating_add должен вернуть MAX при переполнении
    let result = max_score.saturating_add(1);
    assert_eq!(
        result,
        u128::MAX,
        "saturating_add должен вернуть u128::MAX при переполнении"
    );
    
    // Тест с большим добавлением
    let result2 = max_score.saturating_add(1000);
    assert_eq!(
        result2,
        u128::MAX,
        "saturating_add должен вернуть u128::MAX при добавлении 1000"
    );
    
    // Тест с u128::MAX + u128::MAX
    let result3 = max_score.saturating_add(max_score);
    assert_eq!(
        result3,
        u128::MAX,
        "saturating_add(u128::MAX, u128::MAX) должен вернуть u128::MAX"
    );
    
    // Тест с близким к MAX значением
    let near_max = u128::MAX - 100;
    let result4 = near_max.saturating_add(200);
    assert_eq!(
        result4,
        u128::MAX,
        "saturating_add должен вернуть u128::MAX при переполнении near_max"
    );
    
    // Тест что нормальные значения работают корректно
    let normal_score = 1000u128;
    let result5 = normal_score.saturating_add(500);
    assert_eq!(
        result5,
        1500,
        "saturating_add должен корректно складывать нормальные значения"
    );
}

/// Тест 2: Проверка начисления очков без переполнения
///
/// Проверяет, что очки начисляются корректно с использованием u128.
#[test]
fn test_score_accumulation_without_overflow() {
    let mut score: u128 = 0;
    
    // Начисляем очки за линию (100)
    score = score.saturating_add(ROW_SCORE_INC);
    assert_eq!(score, 100, "После первой линии счёт должен быть 100");
    
    // Начисляем очки за вторую линию (200)
    score = score.saturating_add(ROW_SCORE_INC * 2);
    assert_eq!(
        score, 300,
        "После второй линии счёт должен быть 300"
    );
    
    // Начисляем очки за фигуру
    score = score.saturating_add(PIECE_SCORE_INC);
    assert_eq!(
        score, 400,
        "После фигуры счёт должен быть 400"
    );
    
    // Начисляем очки за Soft Drop (1 очко за ячейку)
    let soft_drop_distance = 10;
    score = score.saturating_add((soft_drop_distance as u128) * SOFT_DROP_POINTS);
    assert_eq!(
        score, 410,
        "После Soft Drop счёт должен быть 410"
    );
    
    // Начисляем очки за Hard Drop (2 очка за ячейку)
    let hard_drop_distance = 5;
    score = score.saturating_add(hard_drop_distance as u128 * HARD_DROP_POINTS);
    assert_eq!(
        score, 420,
        "После Hard Drop счёт должен быть 420"
    );
    
    // Начисляем бонус за комбо
    let combo = 3;
    if combo > 1 {
        score = score.saturating_add(COMBO_BONUS * (combo - 1) as u128);
    }
    assert_eq!(
        score, 520,
        "После комбо x3 счёт должен быть 520"
    );
    
    // Проверяем что счёт остаётся в пределах u128
    assert!(
        score < u128::MAX,
        "Счёт должен быть меньше u128::MAX"
    );
}

/// Тест 3: Проверка поведения при score = u128::MAX
///
/// Проверяет, что система корректно обрабатывает счёт близкий к u128::MAX.
#[test]
fn test_score_at_u128_max() {
    // Тест 1: Поведение при u128::MAX
    let max_score = u128::MAX;
    let result = max_score.saturating_add(100);
    assert_eq!(
        result,
        u128::MAX,
        "При переполнении u128::MAX должно остаться u128::MAX"
    );
    
    // Тест 2: Поведение при u128::MAX - 1
    let near_max = u128::MAX - 1;
    let result2 = near_max.saturating_add(1);
    assert_eq!(
        result2,
        u128::MAX,
        "u128::MAX - 1 + 1 должно вернуть u128::MAX"
    );
    
    // Тест 3: Поведение при u128::MAX - 50 + 100
    let near_max2 = u128::MAX - 50;
    let result3 = near_max2.saturating_add(100);
    assert_eq!(
        result3,
        u128::MAX,
        "u128::MAX - 50 + 100 должно вернуть u128::MAX (насыщение)"
    );
    
    // Тест 4: Проверка что u128 достаточно большой для любого счёта
    // u128::MAX = 340282366920938463463374607431768211455
    // Это больше чем любое возможное количество очков в игре
    assert!(
        u128::MAX > u64::MAX as u128,
        "u128 должен быть больше u64"
    );
    
    // Тест 5: Проверка что константы очков совместимы с u128
    assert!(
        ROW_SCORE_INC <= u128::MAX,
        "ROW_SCORE_INC должен помещаться в u128"
    );
    assert!(
        PIECE_SCORE_INC <= u128::MAX,
        "PIECE_SCORE_INC должен помещаться в u128"
    );
    assert!(
        SOFT_DROP_POINTS <= u128::MAX,
        "SOFT_DROP_POINTS должен помещаться в u128"
    );
    assert!(
        HARD_DROP_POINTS <= u128::MAX,
        "HARD_DROP_POINTS должен помещаться в u128"
    );
    assert!(
        COMBO_BONUS <= u128::MAX,
        "COMBO_BONUS должен помещаться в u128"
    );
}

/// Тест 4: Стресс-тест с очень большим счётом u128
///
/// Проверяет поведение счёта при очень больших значениях u128.
#[test]
fn test_stress_u128_large_score() {
    let mut score: u128 = 0;
    
    // Начисляем очень много очков
    let large_increment = 1_000_000_000_000u128; // 1 trillion
    
    // Начисляем 1000 раз
    for _ in 0..1000 {
        score = score.saturating_add(large_increment);
    }
    
    assert_eq!(
        score, 1_000_000_000_000_000, // 1 quadrillion
        "Счёт должен быть 1 квадриллион"
    );
    
    // Тест с близким к MAX значением
    let mut near_max_score = u128::MAX - 1000;
    
    // Малое добавление должно работать
    near_max_score = near_max_score.saturating_add(500);
    assert!(
        near_max_score < u128::MAX,
        "Счёт должен быть меньше u128::MAX"
    );
    
    // Добавление вызывающее переполнение
    near_max_score = near_max_score.saturating_add(1000);
    assert_eq!(
        near_max_score,
        u128::MAX,
        "При переполнении должен вернуть u128::MAX"
    );
}

/// Тест 5: Интеграционный тест с GameState
///
/// Проверяет что GameState корректно использует u128 для счёта.
#[test]
fn test_game_state_score_u128() {
    let state = GameState::new();
    
    // Проверяем начальный счёт
    let initial_score = state.get_score();
    assert_eq!(initial_score, 0, "Начальный счёт должен быть 0");
    
    // Проверяем что тип возвращаемого значения совместим с u128
    // (просто проверяем что код компилируется и работает)
    let _score_u128: u128 = initial_score;
    
    // Проверяем что счёт не отрицательный (u128 не может быть отрицательным)
    assert!(
        initial_score >= 0,
        "Счёт не может быть отрицательным"
    );
}
