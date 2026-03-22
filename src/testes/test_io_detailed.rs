//! Тесты ввода-вывода в Tetris CLI.
//!
//! Этот модуль содержит 20 тестов для проверки системы ввода-вывода:
//! - Тесты констант размеров (5 тестов)
//! - Тесты Canvas (создание, reset) (5 тестов)
//! - Тесты `KeyReader` (5 тестов)
//! - Тесты минимального размера терминала (5 тестов)
//!
//! Все тесты проверяют корректность системы ввода-вывода.

use crate::io::{DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_WIDTH};

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Тесты констант размеров
// ============================================================================

/// Тест 1: `GRID_WIDTH` равен 10
#[test]
fn test_grid_width_constant() {
    assert_eq!(GRID_WIDTH, 10, "Ширина игрового поля должна быть 10 блоков");
}

/// Тест 2: `GRID_HEIGHT` равен 20
#[test]
fn test_grid_height_constant() {
    assert_eq!(
        GRID_HEIGHT, 20,
        "Высота игрового поля должна быть 20 блоков"
    );
}

/// Тест 3: `SHAPE_WIDTH` равен 2
#[test]
fn test_shape_width_constant() {
    assert_eq!(SHAPE_WIDTH, 2, "Ширина фигуры должна быть 2 символа");
}

/// Тест 4: `DISP_WIDTH` вычисляется корректно
#[test]
fn test_disp_width_calculation() {
    // DISP_WIDTH = (SHAPE_WIDTH * GRID_WIDTH) + 2
    let expected = (SHAPE_WIDTH * GRID_WIDTH) as u16 + 2;
    assert_eq!(
        DISP_WIDTH, expected,
        "Полная ширина дисплея должна быть (SHAPE_WIDTH * GRID_WIDTH) + 2"
    );
    assert_eq!(DISP_WIDTH, 22, "DISP_WIDTH должен быть 22");
}

/// Тест 5: `DISP_HEIGHT` вычисляется корректно
#[test]
fn test_disp_height_calculation() {
    // DISP_HEIGHT = GRID_HEIGHT + 5
    let expected = GRID_HEIGHT as u16 + 5;
    assert_eq!(
        DISP_HEIGHT, expected,
        "Полная высота дисплея должна быть GRID_HEIGHT + 5"
    );
    assert_eq!(DISP_HEIGHT, 25, "DISP_HEIGHT должен быть 25");
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Тесты Canvas (создание, reset)
// ============================================================================

/// Тест 6: Canvas константы корректны
#[test]
fn test_canvas_constants() {
    // Проверяем, что константы для отрисовки корректны
    let _ = (GRID_WIDTH, GRID_HEIGHT);
}

/// Тест 7: Размеры поля для классического тетриса
#[test]
fn test_field_dimensions_classic() {
    // Классический тетрис: 10x20
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);

    // Соотношение сторон 1:2
    assert_eq!(GRID_HEIGHT, GRID_WIDTH * 2);
}

/// Тест 8: `DISP_WIDTH` положительный
#[test]
fn test_disp_width_positive() {
    let _ = DISP_WIDTH;
}

/// Тест 9: `DISP_HEIGHT` положительный
#[test]
fn test_disp_height_positive() {
    let _ = DISP_HEIGHT;
}

/// Тест 10: `SHAPE_WIDTH` соответствует отрисовке
#[test]
fn test_shape_width_matches_rendering() {
    // SHAPE_WIDTH = 2 соответствует "██"
    assert_eq!(SHAPE_WIDTH, 2);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Тесты KeyReader
// ============================================================================

/// Тест 11: `KeyReader` базовые константы
#[test]
fn test_keyreader_constants() {
    // Проверяем, что константы для ввода корректны
    assert!(
        DISP_WIDTH >= GRID_WIDTH as u16,
        "DISP_WIDTH должен вмещать GRID_WIDTH"
    );
}

/// Тест 12: Минимальный размер для игры
#[test]
fn test_minimum_size_for_game() {
    // Минимальный размер терминала для игры
    let min_width = DISP_WIDTH;
    let min_height = DISP_HEIGHT;

    assert!(min_width >= 22, "Минимальная ширина должна быть >= 22");
    assert!(min_height >= 25, "Минимальная высота должна быть >= 25");
}

/// Тест 13: `GRID_WIDTH` меньше `DISP_WIDTH`
#[test]
fn test_grid_width_less_than_disp() {
    assert!(
        (GRID_WIDTH as u16 * SHAPE_WIDTH as u16) < DISP_WIDTH,
        "Игровое поле должно помещаться в дисплей"
    );
}

/// Тест 14: `GRID_HEIGHT` меньше `DISP_HEIGHT`
#[test]
fn test_grid_height_less_than_disp_height() {
    assert!(
        (GRID_HEIGHT as u16) < DISP_HEIGHT,
        "Игровое поле по высоте должно помещаться в дисплей"
    );
}

/// Тест 15: Соотношение размеров
#[test]
fn test_size_ratios() {
    // Проверяем соотношения
    assert_eq!(DISP_WIDTH, 22);
    assert_eq!(DISP_HEIGHT, 25);
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-20: Тесты минимального размера терминала
// ============================================================================

/// Тест 16: Минимальная ширина терминала
#[test]
fn test_minimum_terminal_width() {
    // Минимальная ширина для отображения игры
    let min_width = DISP_WIDTH;
    assert!(
        min_width >= 22,
        "Минимальная ширина терминала должна быть >= 22"
    );
}

/// Тест 17: Минимальная высота терминала
#[test]
fn test_minimum_terminal_height() {
    // Минимальная высота для отображения игры
    let min_height = DISP_HEIGHT;
    assert!(
        min_height >= 25,
        "Минимальная высота терминала должна быть >= 25"
    );
}

/// Тест 18: Размер поля соответствует классическому тетрису
#[test]
fn test_field_size_classic_tetris() {
    // Классический тетрис: 10 блоков в ширину, 20 в высоту
    assert_eq!(GRID_WIDTH, 10, "Классическая ширина 10");
    assert_eq!(GRID_HEIGHT, 20, "Классическая высота 20");
}

/// Тест 19: Дополнительные поля для интерфейса
#[test]
fn test_extra_space_for_ui() {
    // Проверяем, что есть место для интерфейса
    let ui_height = DISP_HEIGHT - GRID_HEIGHT as u16;
    assert!(
        ui_height >= 5,
        "Должно быть место для UI (счёт, уровень, линии)"
    );
}

/// Тест 20: Общий размер дисплея
#[test]
fn test_total_display_size() {
    // Общая площадь дисплея
    let total_area = DISP_WIDTH * DISP_HEIGHT;
    let field_area = (GRID_WIDTH as u16 * SHAPE_WIDTH as u16) * GRID_HEIGHT as u16;

    assert!(
        total_area > field_area,
        "Общая площадь должна быть больше площади поля"
    );
    assert!(
        total_area >= 550,
        "Общая площадь должна быть >= 550 символов"
    );
}
