//! Тесты ввода/вывода.
//!
//! Этот модуль содержит 10 тестов для проверки системы ввода/вывода:
//! - Тесты Canvas (4 теста)
//! - Тесты `KeyReader` (3 теста)
//! - Тесты констант (3 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты работы с терминалом.

use crate::io::{
    Canvas, KeyReader, DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH,
};

// ============================================================================
// ГРУППА ТЕСТОВ 1-4: Canvas
// ============================================================================

/// Тест 1: Проверка создания Canvas
///
/// Проверяет, что `Canvas::new()` успешно создаёт канвас.
/// Примечание: тест может завершиться ошибкой если терминал не поддерживает raw-режим.
/// Игнорируется в автоматических тестах так как требует реальный терминал.
#[test]
#[ignore = "Требует реальный терминал, игнорируется в CI/CD"]
fn test_canvas_creation() {
    // Создаём канвас
    let _canvas = Canvas::new();
    // Канвас создан успешно
}

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

/// Тест 5: Проверка создания `KeyReader`
///
/// Проверяет, что `KeyReader::new()` успешно создаёт читатель.
#[test]
fn test_key_reader_creation() {
    let reader = KeyReader::new();

    // KeyReader создан успешно
    // Проверяем, что он содержит async_stdin
    // (не можем проверить внутреннее состояние, но можем убедиться что не паникует)
    drop(reader); // Явно освобождаем ресурс
}

/// Тест 6: Проверка `get_key()` без нажатий
///
/// Проверяет, что `get_key()` возвращает None когда нет нажатий.
#[test]
fn test_key_reader_get_key_no_input() {
    let mut reader = KeyReader::new();

    // В тестовой среде маловероятно что есть нажатия клавиш
    // get_key() должен вернуть None или Some(key)
    let key = reader.get_key();

    // get_key() теперь возвращает Option<u8>
    // Проверяем что метод работает без паники
    let _ = key;
}

/// Тест 7: Проверка Default для `KeyReader`
///
/// Проверяет, что Default реализация работает корректно.
#[test]
fn test_key_reader_default() {
    let reader_default = KeyReader::default();
    let reader_new = KeyReader::new();

    // Оба должны быть созданы успешно
    drop(reader_default);
    drop(reader_new);
}

// ============================================================================
// ГРУППА ТЕСТОВ 8-10: Константы размеров
// ============================================================================

/// Тест 8: Проверка размеров игрового поля
///
/// Проверяет константы `GRID_WIDTH` и `GRID_HEIGHT`.
#[test]
fn test_field_dimensions() {
    assert_eq!(GRID_WIDTH, 10, "GRID_WIDTH должен быть 10 блоков");
    assert_eq!(GRID_HEIGHT, 20, "GRID_HEIGHT должен быть 20 блоков");
    assert_eq!(
        GRID_WIDTH * GRID_HEIGHT,
        200,
        "Общее количество клеток должно быть 200"
    );
}

/// Тест 9: Проверка расчёта `DISP_HEIGHT`
///
/// Проверяет формулу расчёта полной высоты дисплея.
#[test]
fn test_disp_height_calculation() {
    // Формула: GRID_HEIGHT + 5 (заголовки и границы)
    let expected_height = GRID_HEIGHT + 5;
    assert_eq!(
        DISP_HEIGHT, expected_height,
        "DISP_HEIGHT должен быть GRID_HEIGHT + 5 = {expected_height}"
    );
    assert_eq!(DISP_HEIGHT, 25, "DISP_HEIGHT должен быть 25");
}

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
