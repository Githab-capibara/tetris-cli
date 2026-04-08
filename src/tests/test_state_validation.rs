//! Тесты валидации `fall_speed` и `land_timer`.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит тесты для проверки валидации значений:
//! - Проверка возврата ошибки при `NaN` значении
//! - Проверка возврата ошибки при Infinity значении
//! - Проверка корректной работы с валидными значениями
//! - Проверка clamp значений в допустимых пределах
//!
//! ## Исправление H3
//! Функции `set_fall_speed()` и `set_land_timer()` проверяют значения
//! на `NaN` и Infinity, возвращая `GameError::ValidationError` при невалидных значениях.

use crate::constants::{INITIAL_FALL_SPD, LAND_TIME_DELAY_S, MAX_FALL_SPEED};
use crate::errors::GameError;
use crate::game::GameState;

/// Макрос для сравнения f32 значений с эпсилоном
macro_rules! assert_f32_eq {
    ($a:expr, $b:expr) => {
        assert!(
            ($a - $b).abs() < f32::EPSILON,
            "Ожидается {}, получено {} (разница: {})",
            $b, $a, ($a - $b).abs()
        )
    };
    ($a:expr, $b:expr, $msg:expr) => {
        assert!(
            ($a - $b).abs() < f32::EPSILON,
            "{}: ожидается {}, получено {} (разница: {})",
            $msg, $b, $a, ($a - $b).abs()
        )
    };
}

/// Макрос для сравнения f64 значений с эпсилоном
macro_rules! assert_f64_eq {
    ($a:expr, $b:expr) => {
        assert!(
            ($a - $b).abs() < f64::EPSILON,
            "Ожидается {}, получено {} (разница: {})",
            $b, $a, ($a - $b).abs()
        )
    };
    ($a:expr, $b:expr, $msg:expr) => {
        assert!(
            ($a - $b).abs() < f64::EPSILON,
            "{}: ожидается {}, получено {} (разница: {})",
            $msg, $b, $a, ($a - $b).abs()
        )
    };
}

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Валидация fall_speed (NaN и Infinity)
// ============================================================================

/// Тест 1: Проверка возврата ошибки при `NaN` значении `fall_speed`
///
/// Проверяет, что `set_fall_speed` возвращает ошибку при `NaN`.
#[test]
fn test_set_fall_speed_nan_returns_error() {
    let mut state = GameState::new();
    let nan_value = f32::NAN;

    let result = state.set_fall_speed(nan_value);

    assert!(
        result.is_err(),
        "set_fall_speed должен вернуть ошибку при NaN"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            assert!(
                msg.contains("NaN"),
                "Сообщение об ошибке должно содержать 'NaN'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }

    // Проверяем что значение не изменилось
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD, "Скорость падения не должна измениться при NaN");
}

/// Тест 2: Проверка возврата ошибки при Infinity значении `fall_speed`
///
/// Проверяет, что `set_fall_speed` возвращает ошибку при +Infinity.
#[test]
fn test_set_fall_speed_positive_infinity_returns_error() {
    let mut state = GameState::new();
    let inf_value = f32::INFINITY;

    let result = state.set_fall_speed(inf_value);

    assert!(
        result.is_err(),
        "set_fall_speed должен вернуть ошибку при Infinity"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            assert!(
                msg.contains("Infinity"),
                "Сообщение об ошибке должно содержать 'Infinity'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }

    // Проверяем что значение не изменилось
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD, "Скорость падения не должна измениться при Infinity");
}

/// Тест 3: Проверка возврата ошибки при -Infinity значении `fall_speed`
///
/// Проверяет, что `set_fall_speed` возвращает ошибку при -Infinity.
#[test]
fn test_set_fall_speed_negative_infinity_returns_error() {
    let mut state = GameState::new();
    let neg_inf_value = f32::NEG_INFINITY;

    let result = state.set_fall_speed(neg_inf_value);

    assert!(
        result.is_err(),
        "set_fall_speed должен вернуть ошибку при -Infinity"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            assert!(
                msg.contains("Infinity"),
                "Сообщение об ошибке должно содержать 'Infinity'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }
}

/// Тест 4: Проверка корректной работы с валидным `fall_speed`
///
/// Проверяет, что валидные значения устанавливаются корректно.
#[test]
fn test_set_fall_speed_valid_values() {
    let mut state = GameState::new();

    // Устанавливаем валидное значение
    let valid_value = 2.0;
    let result = state.set_fall_speed(valid_value);

    assert!(
        result.is_ok(),
        "Установка валидного значения должна быть успешной"
    );
    assert_f32_eq!(state.fall_speed(), valid_value, "Скорость падения должна быть установлена корректно");

    // Устанавливаем другое валидное значение
    let another_valid = 5.5;
    let result = state.set_fall_speed(another_valid);

    assert!(result.is_ok());
    assert_f32_eq!(state.fall_speed(), another_valid);
}

/// Тест 5: Проверка валидации `fall_speed` в допустимых пределах
///
/// Проверяет, что значения за пределами диапазона возвращают ошибку.
#[test]
fn test_set_fall_speed_clamps_to_valid_range() {
    let mut state = GameState::new();

    // Попытка установить значение ниже минимума — должна вернуть ошибку
    let below_min = INITIAL_FALL_SPD - 0.5;
    let result = state.set_fall_speed(below_min);

    assert!(
        result.is_err(),
        "Установка значения ниже минимума должна вернуть ошибку"
    );
    // Значение не должно измениться
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD);

    // Попытка установить значение выше максимума — должна вернуть ошибку
    let above_max = MAX_FALL_SPEED + 100.0;
    let result = state.set_fall_speed(above_max);

    assert!(
        result.is_err(),
        "Установка значения выше максимума должна вернуть ошибку"
    );
    // Значение не должно измениться
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD);

    // Устанавливаем значение в допустимых пределах
    let in_range = (INITIAL_FALL_SPD + MAX_FALL_SPEED) / 2.0;
    let result = state.set_fall_speed(in_range);

    assert!(result.is_ok());
    assert_f32_eq!(state.fall_speed(), in_range, "Значение в пределах должно устанавливаться корректно");
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Валидация land_timer (NaN и Infinity)
// ============================================================================

/// Тест 6: Проверка возврата ошибки при `NaN` значении `land_timer`
///
/// Проверяет, что `set_land_timer` возвращает ошибку при `NaN`.
#[test]
fn test_set_land_timer_nan_returns_error() {
    let mut state = GameState::new();
    let nan_value = f64::NAN;

    let result = state.set_land_timer(nan_value);

    assert!(
        result.is_err(),
        "set_land_timer должен вернуть ошибку при NaN"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            // Сообщение содержит "не является конечным" для NaN/Infinity
            assert!(
                msg.contains("конечным") || msg.contains("NaN") || msg.contains("неверн"),
                "Сообщение об ошибке должно описывать проблему: '{msg}'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }

    // Проверяем что значение не изменилось
    assert_f64_eq!(state.land_timer(), LAND_TIME_DELAY_S, "Таймер приземления не должен измениться при NaN");
}

/// Тест 7: Проверка возврата ошибки при Infinity значении `land_timer`
///
/// Проверяет, что `set_land_timer` возвращает ошибку при +Infinity.
#[test]
fn test_set_land_timer_positive_infinity_returns_error() {
    let mut state = GameState::new();
    let inf_value = f64::INFINITY;

    let result = state.set_land_timer(inf_value);

    assert!(
        result.is_err(),
        "set_land_timer должен вернуть ошибку при Infinity"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            assert!(
                msg.contains("конечным") || msg.contains("Infinity"),
                "Сообщение об ошибке должно описывать проблему: '{msg}'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }
}

/// Тест 8: Проверка возврата ошибки при -Infinity значении `land_timer`
///
/// Проверяет, что `set_land_timer` возвращает ошибку при -Infinity.
#[test]
fn test_set_land_timer_negative_infinity_returns_error() {
    let mut state = GameState::new();
    let neg_inf_value = f64::NEG_INFINITY;

    let result = state.set_land_timer(neg_inf_value);

    assert!(
        result.is_err(),
        "set_land_timer должен вернуть ошибку при -Infinity"
    );

    match result {
        Err(GameError::ValidationError(msg)) => {
            assert!(
                msg.contains("конечным") || msg.contains("Infinity"),
                "Сообщение об ошибке должно описывать проблему: '{msg}'"
            );
        }
        _ => panic!("Ожидалась ошибка GameError::Validation"),
    }
}

/// Тест 9: Проверка корректной работы с валидным `land_timer`
///
/// Проверяет, что валидные значения устанавливаются корректно.
#[test]
fn test_set_land_timer_valid_values() {
    let mut state = GameState::new();

    // Устанавливаем валидное значение
    let valid_value = 0.5;
    let result = state.set_land_timer(valid_value);

    assert!(
        result.is_ok(),
        "Установка валидного значения должна быть успешной"
    );
    assert_f64_eq!(state.land_timer(), valid_value, "Таймер приземления должен быть установлен корректно");

    // Устанавливаем другое валидное значение
    let another_valid = 1.0;
    let result = state.set_land_timer(another_valid);

    assert!(result.is_ok());
    assert_f64_eq!(state.land_timer(), another_valid);
}

/// Тест 10: Проверка обработки отрицательных значений `land_timer`
///
/// Проверяет, что отрицательные значения возвращают ошибку.
#[test]
fn test_set_land_timer_negative_values_clamped_to_zero() {
    let mut state = GameState::new();

    // Устанавливаем отрицательное значение — должна быть ошибка
    let negative_value = -0.5;
    let result = state.set_land_timer(negative_value);

    assert!(
        result.is_err(),
        "Установка отрицательного значения должна вернуть ошибку"
    );
    // Значение не должно измениться
    assert_f64_eq!(state.land_timer(), LAND_TIME_DELAY_S);

    // Устанавливаем ещё более отрицательное значение — тоже ошибка
    let more_negative = -100.0;
    let result = state.set_land_timer(more_negative);

    assert!(result.is_err());
    // Значение не должно измениться
    assert_f64_eq!(state.land_timer(), LAND_TIME_DELAY_S);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Краевые случаи и интеграция
// ============================================================================

/// Тест 11: Проверка граничных значений `fall_speed`
///
/// Проверяет установку точно на границах диапазона.
#[test]
fn test_set_fall_speed_boundary_values() {
    let mut state = GameState::new();

    // Устанавливаем точно на минимуме
    let result = state.set_fall_speed(INITIAL_FALL_SPD);
    assert!(result.is_ok());
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD, "Значение на минимуме должно устанавливаться");

    // Устанавливаем точно на максимуме
    let result = state.set_fall_speed(MAX_FALL_SPEED);
    assert!(result.is_ok());
    assert_f32_eq!(state.fall_speed(), MAX_FALL_SPEED, "Значение на максимуме должно устанавливаться");
}

/// Тест 12: Проверка граничных значений `land_timer`
///
/// Проверяет установку точно на границе (0.0).
#[test]
fn test_set_land_timer_boundary_values() {
    let mut state = GameState::new();

    // Устанавливаем точно 0.0
    let result = state.set_land_timer(0.0);
    assert!(result.is_ok());
    assert_f64_eq!(state.land_timer(), 0.0);

    // Устанавливаем небольшое положительное значение
    let small_value = 0.001;
    let result = state.set_land_timer(small_value);
    assert!(result.is_ok());
    assert_f64_eq!(state.land_timer(), small_value);
}

/// Тест 13: Интеграционный тест валидации в GameState
///
/// Проверяет что валидация работает в контексте GameState.
#[test]
fn test_validation_in_game_state_context() {
    let mut state = GameState::new();

    // Проверяем начальные значения
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD);
    assert_f64_eq!(state.land_timer(), LAND_TIME_DELAY_S);

    // Пытаемся установить невалидные значения
    assert!(state.set_fall_speed(f32::NAN).is_err());
    assert!(state.set_fall_speed(f32::INFINITY).is_err());
    assert!(state.set_land_timer(f64::NAN).is_err());
    assert!(state.set_land_timer(f64::INFINITY).is_err());

    // Проверяем что значения не изменились
    assert_f32_eq!(state.fall_speed(), INITIAL_FALL_SPD);
    assert_f64_eq!(state.land_timer(), LAND_TIME_DELAY_S);

    // Устанавливаем валидные значения
    assert!(state.set_fall_speed(5.0).is_ok());
    assert!(state.set_land_timer(0.2).is_ok());

    // Проверяем что значения изменились
    assert_f32_eq!(state.fall_speed(), 5.0);
    assert_f64_eq!(state.land_timer(), 0.2);
}

/// Тест 14: Проверка что валидация не вызывает паник
///
/// Проверяет отсутствие паник при различных невалидных значениях.
#[test]
fn test_validation_no_panic_on_invalid_values() {
    // Массив невалидных значений для fall_speed
    let invalid_f32_values = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY];

    for &value in &invalid_f32_values {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut s = GameState::new();
            s.set_fall_speed(value)
        }));

        assert!(
            result.is_ok(),
            "set_fall_speed({value}) не должен вызывать панику"
        );
        assert!(
            result.expect("Паника не должна возникнуть").is_err(),
            "Должна вернуться ошибка"
        );
    }

    // Массив невалидных значений для land_timer
    let invalid_f64_values = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    for &value in &invalid_f64_values {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut s = GameState::new();
            s.set_land_timer(value)
        }));

        assert!(
            result.is_ok(),
            "set_land_timer({value}) не должен вызывать панику"
        );
        assert!(
            result.expect("Паника не должна возникнуть").is_err(),
            "Должна вернуться ошибка"
        );
    }
}

/// Тест 15: Стресс-тест валидации
///
/// Проверяет валидацию при множественных вызовах.
#[test]
fn test_validation_stress_test() {
    let mut state = GameState::new();

    // Многократные вызовы с валидными и невалидными значениями
    for i in 0..1000 {
        // Чередование валидных и невалидных значений
        if i % 3 == 0 {
            // Невалидное NaN
            assert!(state.set_fall_speed(f32::NAN).is_err());
            assert!(state.set_land_timer(f64::NAN).is_err());
        } else if i % 3 == 1 {
            // Невалидное Infinity
            assert!(state.set_fall_speed(f32::INFINITY).is_err());
            assert!(state.set_land_timer(f64::INFINITY).is_err());
        } else {
            // Валидное значение
            let valid_fall = (i as f32) % MAX_FALL_SPEED;
            let valid_timer = (f64::from(i) * 0.001) % LAND_TIME_DELAY_S;
            assert!(state
                .set_fall_speed(valid_fall.max(INITIAL_FALL_SPD))
                .is_ok());
            assert!(state.set_land_timer(valid_timer).is_ok());
        }
    }

    // Финальная проверка что состояние корректно
    assert!(state.fall_speed() >= INITIAL_FALL_SPD);
    assert!(state.fall_speed() <= MAX_FALL_SPEED);
    assert!(state.land_timer() >= 0.0);
}
