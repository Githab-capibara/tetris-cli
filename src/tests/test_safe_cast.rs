//! Тесты безопасной конвертации f32 → u32.
//!
//! Этот модуль содержит параметризированные тесты для функции `safe_f32_to_u32()`:
//! - Конвертация нормальных значений
//! - Конвертация специальных значений (`NaN`, `Infinity`)
//! - Конвертация отрицательных значений
//! - Конвертация пограничных значений
//! - Стресс-тесты и интеграционные проверки

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]

use crate::game::scoring::points::safe_f32_to_u32;

/// Тест: Конвертация нормальных положительных значений.
///
/// Проверяет что обычные значения конвертируются корректно с усечением дробной части.
#[test]
fn test_safe_f32_to_u32_normal_values() {
    // Базовые значения
    let normal_cases = [
        (0.0_f32, 0_u32),
        (1.0, 1),
        (10.0, 10),
        (100.0, 100),
        (1_000.0, 1_000),
        (10_000.0, 10_000),
        (100_000.0, 100_000),
        (1_000_000.0, 1_000_000),
        (10_000_000.0, 10_000_000),
        (100_000_000.0, 100_000_000),
        (1_000_000_000.0, 1_000_000_000),
    ];
    for &(input, expected) in &normal_cases {
        assert_eq!(
            safe_f32_to_u32(input),
            expected,
            "{input}.0 должна конвертироваться в {expected}"
        );
    }

    // Значения с дробной частью (усечение)
    let truncation_cases = [
        (0.1_f32, 0_u32),
        (0.9, 0),
        (1.1, 1),
        (1.9, 1),
        (10.5, 10),
        (99.99, 99),
    ];
    for &(input, expected) in &truncation_cases {
        assert_eq!(
            safe_f32_to_u32(input),
            expected,
            "{input} должна усекаться до {expected}"
        );
    }
}

/// Тест: Конвертация специальных значений (`NaN`, `Infinity`).
///
/// Проверяет что специальные float-значения возвращают 0.
#[test]
fn test_safe_f32_to_u32_special_values() {
    let special_cases = [
        (f32::NAN, 0_u32, "NaN"),
        (-f32::NAN, 0, "-NaN"),
        (f32::INFINITY, 0, "+infinity"),
        (f32::NEG_INFINITY, 0, "-infinity"),
    ];
    for &(input, expected, name) in &special_cases {
        assert_eq!(
            safe_f32_to_u32(input),
            expected,
            "{name} должен конвертироваться в {expected}"
        );
    }

    // Проверка что is_finite() корректно определяет специальные значения
    assert!(!f32::NAN.is_finite(), "NaN должен быть не-finite");
    assert!(
        !f32::INFINITY.is_finite(),
        "+infinity должен быть не-finite"
    );
    assert!(
        !f32::NEG_INFINITY.is_finite(),
        "-infinity должен быть не-finite"
    );
}

/// Тест: Конвертация отрицательных значений.
///
/// Проверяет что все отрицательные значения возвращают 0 (кроме `-f32::MIN`).
#[test]
fn test_safe_f32_to_u32_negative_values() {
    let negative_cases = [
        (-0.0_f32, 0_u32),
        (-0.1, 0),
        (-1.0, 0),
        (-10.0, 0),
        (-100.0, 0),
        (-1_000.0, 0),
        (-1_000_000.0, 0),
        (-f32::MAX, 0),
        // -f32::MIN = +3.4e38 (огромное положительное) → saturates в u32::MAX
        (-f32::MIN, u32::MAX),
    ];
    for &(input, expected) in &negative_cases {
        assert_eq!(
            safe_f32_to_u32(input),
            expected,
            "{input} должна конвертироваться в {expected}"
        );
    }
}

/// Тест: Конвертация пограничных значений `u32::MAX`.
///
/// Проверяет корректную обработку значений вблизи `u32::MAX`.
/// f32 имеет только 24 бита мантиссы, поэтому точность ограничена.
#[test]
fn test_safe_f32_to_u32_boundary_values() {
    // Точное значение u32::MAX
    assert_eq!(
        safe_f32_to_u32(u32::MAX as f32),
        u32::MAX,
        "u32::MAX as f32 должен конвертироваться в u32::MAX"
    );

    // Значение чуть меньше u32::MAX (с учётом точности f32)
    let just_below_max = (u32::MAX as f32) - 1.0;
    let result = safe_f32_to_u32(just_below_max);
    assert!(
        result >= u32::MAX - 256,
        "Значение чуть меньше u32::MAX должно конвертироваться >= u32::MAX - 256"
    );

    // 4_294_967_294.0 округляется до 4_294_967_296.0 в f32
    let val: f32 = 4_294_967_294.0;
    assert!(
        val >= 4_294_967_295.0f32,
        "4_294_967_294.0 округляется в f32"
    );
    assert_eq!(
        safe_f32_to_u32(val),
        u32::MAX,
        "Округлённое значение saturates в u32::MAX"
    );

    // Переполнение: u32::MAX + 1
    let overflow = (u32::MAX as f32) + 1.0;
    assert_eq!(
        safe_f32_to_u32(overflow),
        u32::MAX,
        "u32::MAX + 1 должен saturating cast в u32::MAX"
    );
}

/// Тест: Конвертация очень больших чисел.
///
/// Проверяет что большие значения saturating cast в `u32::MAX`.
#[test]
fn test_safe_f32_to_u32_large_numbers() {
    let large_cases = [
        (f32::MAX, u32::MAX),
        ((u32::MAX as f32) * 2.0, u32::MAX),
        ((u32::MAX as f32) * 10.0, u32::MAX),
        ((u32::MAX as f32) * 100.0, u32::MAX),
    ];
    for &(input, expected) in &large_cases {
        assert_eq!(
            safe_f32_to_u32(input),
            expected,
            "{input} должен saturating cast в {expected}"
        );
    }
}

/// Тест: Конвертация в контексте расчёта расстояния Hard Drop.
///
/// Проверяет корректную работу в реальном игровом сценарии.
#[test]
fn test_safe_f32_to_u32_hard_drop_context() {
    let start_y = 0.0_f32;
    for &end_y in &[5.0, 10.0, 15.0, 20.0, 25.0] {
        let dist = (end_y - start_y).abs();
        assert_eq!(
            safe_f32_to_u32(dist),
            end_y as u32,
            "Расстояние Hard Drop {dist} должно конвертироваться в {end_y}"
        );
    }

    // Отрицательное расстояние через abs
    let dist_neg = (5.0_f32 - 10.0).abs();
    assert_eq!(safe_f32_to_u32(dist_neg), 5, "abs расстояние = 5");
}
