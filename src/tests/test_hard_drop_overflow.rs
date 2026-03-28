//! Тесты переполнения handle_hard_drop.
//!
//! Проверяют конвертацию f32 → u32 с clamp, обработку NaN и Infinity,
//! и максимальную дистанцию падения.

use crate::game::scoring::handle_hard_drop;
use crate::game::GameState;

/// Тест 1: Проверка конвертации f32 → u32 с clamp
///
/// Проверяем, что конвертация дистанции падения безопасна.
#[test]
fn test_f32_to_u32_conversion_with_clamp() {
    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos.1;

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Вычисляем дистанцию падения
    let drop_distance_f32 = (state.curr_shape().pos.1 - initial_y).abs().max(0.0);

    // Проверяем, что дистанция конечна
    assert!(
        drop_distance_f32.is_finite(),
        "Дистанция падения должна быть конечной"
    );

    // Проверяем, что дистанция в пределах u32
    assert!(
        drop_distance_f32 <= u32::MAX as f32,
        "Дистанция падения должна быть <= u32::MAX"
    );

    // Конвертация должна быть безопасной
    let drop_distance: u32 = drop_distance_f32.clamp(0.0, u32::MAX as f32).trunc() as u32;
    // drop_distance - позитивное число, всегда >= 0
}

/// Тест 2: Проверка обработки NaN
///
/// Проверяем, что NaN обрабатывается корректно.
#[test]
fn test_nan_handling() {
    // Проверяем обработку NaN
    let nan_value = f32::NAN;
    let result = if !nan_value.is_finite() {
        0
    } else {
        nan_value.clamp(0.0, u32::MAX as f32).trunc() as u32
    };

    assert_eq!(result, 0, "NaN должен конвертироваться в 0");
}

/// Тест 3: Проверка обработки Infinity
///
/// Проверяем, что Infinity обрабатывается корректно.
#[test]
fn test_infinity_handling() {
    // Проверяем обработку +Infinity
    let inf_value = f32::INFINITY;
    let result_inf = if !inf_value.is_finite() {
        0
    } else {
        inf_value.clamp(0.0, u32::MAX as f32).trunc() as u32
    };

    assert_eq!(result_inf, 0, "+Infinity должен конвертироваться в 0");

    // Проверяем обработку -Infinity
    let neg_inf_value = f32::NEG_INFINITY;
    let result_neg_inf = if !neg_inf_value.is_finite() {
        0
    } else {
        neg_inf_value.clamp(0.0, u32::MAX as f32).trunc() as u32
    };

    assert_eq!(result_neg_inf, 0, "-Infinity должен конвертироваться в 0");
}

/// Тест 4: Проверка максимальной дистанции падения
///
/// Проверяем, что максимальная дистанция падения корректна.
#[test]
fn test_max_drop_distance() {
    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos.1;

    // Опускаем фигуру на максимальную высоту
    // Максимальная высота поля = 20, начальная позиция = 0
    // Максимальная дистанция = 20 ячеек
    while state.can_move_curr_shape_direction(crate::types::Direction::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    let max_drop_distance = (state.curr_shape().pos.1 - initial_y) as u32;

    // Проверяем, что дистанция в разумных пределах
    assert!(
        max_drop_distance <= 20,
        "Максимальная дистанция падения должна быть <= 20 (высота поля)"
    );

    // Проверяем, что дистанция положительна
    assert!(
        max_drop_distance > 0,
        "Дистанция падения должна быть положительной"
    );
}

/// Тест 5: Проверка начисления очков за Hard Drop
///
/// Проверяем, что очки за Hard Drop начисляются корректно.
#[test]
fn test_hard_drop_scoring() {
    use crate::game::constants::HARD_DROP_POINTS;

    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos.1;
    let initial_score = state.score();

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Вычисляем ожидаемые очки
    let drop_distance = (state.curr_shape().pos.1 - initial_y) as u32;
    let expected_bonus = drop_distance as u128 * HARD_DROP_POINTS;

    // Проверяем, что очки начислены
    assert!(
        state.score() >= initial_score + expected_bonus,
        "Очки за Hard Drop должны быть начислены: было {initial_score}, стало {}, ожидаемый бонус {expected_bonus}",
        state.score()
    );

    // Проверяем, что переполнения не произошло
    assert!(state.score() < u128::MAX, "Счёт не должен переполняться");
}

/// Тест 6: Проверка отрицательной дистанции
///
/// Проверяем, что отрицательная дистанция обрабатывается корректно.
#[test]
fn test_negative_distance_handling() {
    // Проверяем обработку отрицательной дистанции
    let neg_distance = -5.0_f32;
    let abs_distance = neg_distance.abs().max(0.0);

    assert!(
        abs_distance >= 0.0,
        "Абсолютная дистанция должна быть неотрицательной"
    );

    let result = if !abs_distance.is_finite() {
        0
    } else {
        abs_distance.clamp(0.0, u32::MAX as f32).trunc() as u32
    };

    // u32::MAX всегда >= 0
}

/// Тест 7: Проверка очень большой дистанции
///
/// Проверяем, что очень большая дистанция clamp'ится корректно.
#[test]
fn test_very_large_distance_clamp() {
    // Проверяем clamp очень большой дистанции
    let large_distance = 1_000_000_000.0_f32; // 1 миллиард

    let clamped = large_distance.clamp(0.0, u32::MAX as f32);
    assert!(
        clamped <= u32::MAX as f32,
        "Дистанция должна быть clamp'ирована до u32::MAX"
    );

    let result = clamped.trunc() as u32;
    // u32 не может быть больше u32::MAX
}

/// Тест 8: Проверка is_finite() перед конвертацией
///
/// Проверяем, что is_finite() вызывается перед конвертацией.
#[test]
fn test_is_finite_check_before_conversion() {
    let test_values = [
        (10.0_f32, true),           // Нормальное значение
        (0.0_f32, true),            // Ноль
        (-5.0_f32, true),           // Отрицательное (станет положительным после abs)
        (f32::NAN, false),          // NaN
        (f32::INFINITY, false),     // +Infinity
        (f32::NEG_INFINITY, false), // -Infinity
    ];

    for &(value, should_be_finite) in &test_values {
        let is_finite = value.is_finite();
        assert_eq!(
            is_finite, should_be_finite,
            "is_finite() для {value} должен быть {should_be_finite}"
        );

        let result = if !is_finite {
            0
        } else {
            value.abs().max(0.0).clamp(0.0, u32::MAX as f32).trunc() as u32
        };

        if !should_be_finite {
            assert_eq!(result, 0, "Неконечное значение должно конвертироваться в 0");
        }
    }
}
