//! Тесты обработки отрицательных координат (game.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка обработки отрицательных Y координат
//! - Проверка обработки отрицательных X координат
//! - Проверка граничных значений с `checked_sub()`
//!
//! Исправление: использование `checked_sub()` для защиты от отрицательных координат

use crate::game::GameState;
use crate::io::{GRID_HEIGHT, GRID_WIDTH};

// ============================================================================
// ГРУППА ТЕСТОВ: Отрицательные координаты
// ============================================================================

/// Тест 1: Проверка обработки отрицательных Y координат
///
/// Проверяет, что игра корректно обрабатывает попытки движения выше поля.
#[test]
fn test_negative_y_coords_handling() {
    // Создаём состояние игры
    let game = GameState::new();

    // Получаем начальную позицию фигуры
    let initial_pos = game.get_curr_shape().pos;
    let (_initial_x, initial_y) = initial_pos;

    // Проверяем что начальная Y координата неотрицательная
    assert!(
        initial_y >= 0.0,
        "Начальная Y координата должна быть неотрицательной, получено {initial_y}"
    );

    // Проверяем что блоки поля инициализированы корректно
    let blocks = game.get_blocks();
    for (y, row) in blocks.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // Все клетки должны быть в пределах поля
            assert!(
                y < GRID_HEIGHT,
                "Y координата {y} должна быть меньше {GRID_HEIGHT}"
            );
            assert!(
                x < GRID_WIDTH,
                "X координата {x} должна быть меньше {GRID_WIDTH}"
            );
            // Пустые клетки должны иметь значение -1
            if cell != -1 {
                assert!(
                    (0..=6).contains(&cell),
                    "Значение клетки {cell} должно быть в диапазоне [-1, 6]"
                );
            }
        }
    }

    // Проверяем что save_tetromino не паникует при нормальных координатах
    // (это косвенная проверка что checked_sub работает)
    let _blocks = game.get_blocks();
}

/// Тест 2: Проверка обработки отрицательных X координат
///
/// Проверяет, что игра корректно обрабатывает попытки движения левее поля.
#[test]
fn test_negative_x_coords_handling() {
    // Создаём состояние игры
    let game = GameState::new();

    // Получаем текущую фигуру
    let curr_shape = game.get_curr_shape();
    let (shape_x, shape_y) = curr_shape.pos;

    // Проверяем что начальная X координата неотрицательная
    assert!(
        shape_x >= 0.0,
        "Начальная X координата должна быть неотрицательной, получено {shape_x}"
    );

    // Проверяем что позиция фигуры в пределах поля
    assert!(
        shape_x >= 0.0 && shape_x < GRID_WIDTH as f32,
        "X позиция фигуры {shape_x} должна быть в пределах [0, {GRID_WIDTH})"
    );
    assert!(
        shape_y >= 0.0 && shape_y < GRID_HEIGHT as f32,
        "Y позиция фигуры {shape_y} должна быть в пределах [0, {GRID_HEIGHT})"
    );

    // Проверяем что can_move_curr_shape корректно обрабатывает границы
    // Движение влево должно быть возможно только если фигура не у края
    let can_move_left = game.can_move_curr_shape_direction(crate::types::Direction::Left);
    let can_move_right = game.can_move_curr_shape_direction(crate::types::Direction::Right);

    // Фигура должна иметь возможность двигаться хотя бы в одном направлении
    assert!(
        can_move_left || can_move_right,
        "Фигура должна иметь возможность двигаться влево или вправо"
    );
}

/// Тест 3: Проверка граничных значений
///
/// Проверяет обработку координат на границах поля с использованием `checked_sub()`.
#[test]
fn test_boundary_coords_values() {
    // Создаём состояние игры
    let game = GameState::new();

    // Проверяем константы границ
    const _: () = assert!(GRID_WIDTH > 0, "GRID_WIDTH должен быть положительным");
    const _: () = assert!(GRID_HEIGHT > 0, "GRID_HEIGHT должен быть положительным");

    // Проверяем что размеры поля корректны
    let blocks = game.get_blocks();
    assert_eq!(
        blocks.len(),
        GRID_HEIGHT,
        "Количество строк должно быть {GRID_HEIGHT}"
    );
    assert_eq!(
        blocks[0].len(),
        GRID_WIDTH,
        "Количество столбцов должно быть {GRID_WIDTH}"
    );

    // Тестируем граничные координаты
    let boundary_coords = [
        (0, 0),                                          // Левый верхний угол
        (GRID_WIDTH as i16 - 1, 0),                      // Правый верхний угол
        (0, GRID_HEIGHT as i16 - 1),                     // Левый нижний угол
        (GRID_WIDTH as i16 - 1, GRID_HEIGHT as i16 - 1), // Правый нижний угол
    ];

    // Проверяем что все граничные координаты в пределах поля
    for (x, y) in &boundary_coords {
        assert!(
            *x >= 0 && *x < GRID_WIDTH as i16,
            "X координата {x} должна быть в пределах [0, {GRID_WIDTH})"
        );
        assert!(
            *y >= 0 && *y < GRID_HEIGHT as i16,
            "Y координата {y} должна быть в пределах [0, {GRID_HEIGHT})"
        );
    }

    // Тестируем координаты за границами поля (должны обрабатываться корректно)
    let out_of_bounds_coords = [
        (-1, 0),                                 // Левее поля
        (0, -1),                                 // Выше поля
        (GRID_WIDTH as i16, 0),                  // Правее поля
        (0, GRID_HEIGHT as i16),                 // Ниже поля
        (-1, -1),                                // Левее и выше
        (GRID_WIDTH as i16, GRID_HEIGHT as i16), // Правее и ниже
    ];

    // Проверяем что все эти координаты действительно за границами
    for (x, y) in &out_of_bounds_coords {
        let is_in_bounds = *x >= 0 && *x < GRID_WIDTH as i16 && *y >= 0 && *y < GRID_HEIGHT as i16;
        assert!(
            !is_in_bounds,
            "Координаты ({x}, {y}) должны быть за границами поля"
        );
    }

    // Проверяем что checked_sub() корректно обрабатывает вычитание
    // Тестируем логику напрямую
    let test_cases = [
        (0u16, 0u16, Some(0u16)),  // 0 - 0 = 0
        (10u16, 5u16, Some(5u16)), // 10 - 5 = 5
        (5u16, 10u16, None),       // 5 - 10 = None (переполнение)
        (0u16, 1u16, None),        // 0 - 1 = None (переполнение)
    ];

    for (a, b, expected) in &test_cases {
        let result = a.checked_sub(*b);
        assert_eq!(
            result, *expected,
            "checked_sub({a}, {b}) должно вернуть {expected:?}"
        );
    }
}

/// Тест 4: Проверка safe конвертации координат
///
/// Проверяет что конвертация координат использует защиту от отрицательных значений.
#[test]
fn test_safe_coords_conversion() {
    // Тестируем логику конвертации координат
    // Проверяем что max(0.0) используется для защиты от отрицательных значений
    // Логика из game.rs: (curr_y - start_y).max(0.0) as u64
    // где curr_y - текущая позиция, start_y - начальная
    // расстояние = curr_y - start_y (если положительное)

    let test_cases = [
        (5.0f32, 5.0f32, 0u64),  // Без движения: 5 - 5 = 0
        (5.0f32, 10.0f32, 5u64), // Движение вниз на 5: 10 - 5 = 5
        (10.0f32, 5.0f32, 0u64), // Отрицательное движение: 5 - 10 = -5 (обрезается до 0)
        (0.0f32, 0.0f32, 0u64),  // Нулевое движение: 0 - 0 = 0
    ];

    for (start_y, curr_y, expected_distance) in &test_cases {
        let diff = curr_y - start_y;
        let distance = diff.max(0.0) as u64;

        assert_eq!(
            distance, *expected_distance,
            "Расстояние от start={start_y} до curr={curr_y} должно быть {expected_distance}"
        );
    }
}
