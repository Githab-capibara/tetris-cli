//! Тесты ввода/вывода.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит тесты для проверки системы ввода/вывода:
//! - Тесты констант
//!
//! Все тесты независимы и проверяют отдельные аспекты работы с терминалом.

use crate::constants::{DISP_HEIGHT, DISP_WIDTH, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};

// ============================================================================
// ГРУППА ТЕСТОВ 1-4: Canvas
// ============================================================================

/// Тест 2: Проверка константы `SHAPE_STR`
///
/// Проверяет, что символ блока имеет правильную длину.
#[test]
fn test_shape_str_constant() {
    assert_eq!(SHAPE_STR, "██", "SHAPE_STR должен быть '██'");
    // Символы Unicode занимают 3 байта каждый в UTF-8
    assert_eq!(
        SHAPE_STR.len(),
        6,
        "Длина SHAPE_STR должна быть 6 байт (2 символа UTF-8 по 3 байта)"
    );
}

/// Тест 3: Проверка константы `SHAPE_WIDTH`
///
/// Проверяет, что ширина блока соответствует длине `SHAPE_STR`.
#[test]
fn test_shape_width_constant() {
    assert_eq!(SHAPE_WIDTH, 2, "SHAPE_WIDTH должен быть 2 символа");
    assert_eq!(
        SHAPE_WIDTH * GRID_WIDTH,
        20,
        "Полная ширина поля должна быть 20 символов"
    );
}

/// Тест 4: Проверка расчёта `DISP_WIDTH`
///
/// Проверяет формулу расчёта полной ширины дисплея.
#[test]
fn test_disp_width_calculation() {
    // Формула: (SHAPE_WIDTH * GRID_WIDTH) + 2 (границы)
    let expected_width = (SHAPE_WIDTH * GRID_WIDTH) + 2;
    assert_eq!(
        DISP_WIDTH, expected_width,
        "DISP_WIDTH должен быть (SHAPE_WIDTH * GRID_WIDTH) + 2 = {expected_width}"
    );
    assert_eq!(DISP_WIDTH, 22, "DISP_WIDTH должен быть 22");
}

// ============================================================================
// ГРУППА ТЕСТОВ 5-7: KeyReader
// ============================================================================

/// Тест 10: Проверка минимального размера терминала
///
/// Проверяет, что размеры дисплея достаточны для игры.
#[test]
fn test_terminal_minimum_size() {
    // Минимальный размер терминала должен быть не меньше DISP_WIDTH x DISP_HEIGHT
    let min_width = DISP_WIDTH;
    let min_height = DISP_HEIGHT;

    assert!(
        min_width >= 22,
        "Минимальная ширина должна быть не менее 22"
    );
    assert!(
        min_height >= 25,
        "Минимальная высота должна быть не менее 25"
    );

    // Проверяем соотношение сторон
    let aspect_ratio = min_width as f32 / min_height as f32;
    assert!(
        aspect_ratio > 0.5 && aspect_ratio < 2.0,
        "Соотношение сторон должно быть разумным (0.5-2.0), получено {aspect_ratio}"
    );
}
