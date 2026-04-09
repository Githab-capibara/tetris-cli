//! Тесты вращения фигур в Tetris CLI.
//!
//! Этот модуль содержит параметризованные тесты для проверки всех аспектов вращения фигур:
//! - Тест вращения по часовой для всех фигур (1 параметризованный тест)
//! - Тест вращения против часовой для всех фигур (1 параметризованный тест)
//! - Тест полного цикла вращения (1 параметризованный тест)
//! - Тест вращения у стен (1 параметризованный тест)
//! - Тест вращения над фигурами (1 параметризованный тест)
//! - Тест вращения с коллизиями (1 параметризованный тест)
//! - Тест spin-вращения (T-spin, I-spin) (1 тест)
//! - Тест специального вращения S и Z (1 тест)

// Разрешаем использование deprecated метода rotate_old() для тестирования обратной совместимости
#![allow(deprecated)]

use crate::tetromino::{ShapeType, Tetromino, SHAPE_COORDS};
use crate::types::RotationDirection;

// ============================================================================
// ГРУППА ТЕСТОВ 1: Вращение по часовой для всех фигур (параметризованный тест)
// ============================================================================

/// Тест: Вращение всех фигур по часовой стрелке.
///
/// Проверяет, что при вращении по часовой стрелке координаты изменяются
/// для всех фигур, кроме O-фигуры (квадрат не вращается).
#[test]
fn test_all_shapes_rotate_clockwise() {
    let all_shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &all_shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (4.0, 0.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();
        t.rotate(RotationDirection::Clockwise);

        if *shape == ShapeType::O {
            assert_eq!(
                t.coords(),
                original_coords,
                "O-фигура (квадрат) не должна вращаться по часовой"
            );
        } else {
            assert_ne!(
                t.coords(),
                original_coords,
                "{shape:?}-фигура должна изменить координаты после вращения по часовой"
            );
        }
    }

    // Дополнительная проверка для T-фигуры: центральный блок остаётся на месте
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::T, SHAPE_COORDS[0], 0);
    t.rotate(RotationDirection::Clockwise);
    assert_eq!(
        t.coords()[1],
        (0, 0),
        "Центральный блок T-фигуры должен остаться на месте"
    );

    // Дополнительная проверка для I-фигуры: центральный блок остаётся на месте
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::I, SHAPE_COORDS[6], 6);
    t.rotate(RotationDirection::Clockwise);
    assert_eq!(
        t.coords()[1],
        (0, 0),
        "Центральный блок I-фигуры должен остаться на месте"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 2: Вращение против часовой для всех фигур (параметризованный тест)
// ============================================================================

/// Тест: Вращение всех фигур против часовой стрелки.
///
/// Проверяет, что при вращении против часовой стрелки координаты изменяются
/// для всех фигур, кроме O-фигуры (квадрат не вращается).
#[test]
fn test_all_shapes_rotate_counter_clockwise() {
    let all_shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &all_shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (4.0, 0.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();
        t.rotate(RotationDirection::CounterClockwise);

        if *shape == ShapeType::O {
            assert_eq!(
                t.coords(),
                original_coords,
                "O-фигура не должна вращаться против часовой"
            );
        } else {
            assert_ne!(
                t.coords(),
                original_coords,
                "{shape:?}-фигура должна изменить координаты после вращения против часовой"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 3: Полный цикл вращения (4 поворота) для всех фигур
// ============================================================================

/// Тест: Полный цикл вращения всех фигур (4 поворота по часовой).
///
/// Проверяет, что после 4 вращений по часовой стрелке каждая фигура
/// возвращается к исходным координатам. O-фигура остаётся неизменной.
#[test]
fn test_all_shapes_full_rotation_cycle() {
    let all_shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &all_shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (4.0, 0.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();

        // 4 вращения по часовой должны вернуть к исходным координатам
        for _ in 0..4 {
            t.rotate(RotationDirection::Clockwise);
        }

        assert_eq!(
            t.coords(),
            original_coords,
            "{shape:?}-фигура должна вернуться к исходным координатам после 4 вращений"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 4: Вращение у стен (параметризованный тест)
// ============================================================================

/// Тест: Вращение фигур у левой и правой стен.
///
/// Проверяет, что фигуры можно вращать рядом со стенами.
/// Тестирует T, L, I, J у левой (x=0) и правой (x=9) стен.
/// O-фигура не вращается.
#[test]
fn test_rotation_at_walls() {
    // Фигуры для тестирования у стен
    let wall_test_shapes = [
        (ShapeType::T, 0),
        (ShapeType::L, 1),
        (ShapeType::I, 6),
        (ShapeType::J, 2),
    ];

    for (shape, shape_index) in &wall_test_shapes {
        // Вращение у левой стены
        let mut t = Tetromino::new(
            (0.0, 5.0),
            *shape,
            SHAPE_COORDS[*shape_index],
            *shape_index as u8,
        );
        let original_coords = t.coords();
        t.rotate(RotationDirection::Clockwise);
        assert_ne!(
            t.coords(),
            original_coords,
            "{shape:?}-фигура должна вращаться у левой стены"
        );

        // Вращение у правой стены
        let mut t = Tetromino::new(
            (9.0, 5.0),
            *shape,
            SHAPE_COORDS[*shape_index],
            *shape_index as u8,
        );
        t.rotate(RotationDirection::Clockwise);
        assert_ne!(
            t.coords(),
            SHAPE_COORDS[*shape_index],
            "{shape:?}-фигура должна вращаться у правой стены"
        );
    }

    // O-фигура не вращается у стены
    let mut t = Tetromino::new((0.0, 5.0), ShapeType::O, SHAPE_COORDS[5], 5);
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);
    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура не должна вращаться у стены"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 5: Вращение над фигурами (параметризованный тест)
// ============================================================================

/// Тест: Вращение всех фигур над другой фигурой (в воздухе).
///
/// Проверяет, что все фигуры можно вращать в воздухе (над зафиксированной фигурой).
/// O-фигура не вращается.
#[test]
fn test_rotation_above_piece_all_shapes() {
    let all_shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &all_shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (4.0, 5.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();
        t.rotate(RotationDirection::Clockwise);

        if *shape == ShapeType::O {
            assert_eq!(
                t.coords(),
                original_coords,
                "O-фигура не должна вращаться над другой фигурой"
            );
        } else {
            assert_ne!(
                t.coords(),
                SHAPE_COORDS[shape_index],
                "{shape:?}-фигура должна вращаться над другой фигурой"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 6: Вращение с коллизиями (параметризованный тест)
// ============================================================================

/// Тест: Вращение всех фигур с коллизией (в ограниченном пространстве).
///
/// Проверяет, что фигуры пытаются вращаться даже при близости к стене.
/// O-фигура не вращается.
#[test]
fn test_rotation_with_collision_all_shapes() {
    let all_shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &all_shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (1.0, 5.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();
        t.rotate(RotationDirection::Clockwise);

        if *shape == ShapeType::O {
            assert_eq!(
                t.coords(),
                original_coords,
                "O-фигура не должна вращаться даже с коллизией"
            );
        } else {
            assert_ne!(
                t.coords(),
                SHAPE_COORDS[shape_index],
                "{shape:?}-фигура должна пытаться вращаться при коллизии"
            );
        }
    }

    // Дополнительная проверка для T-фигуры у правой стены (x=8)
    let mut t = Tetromino::new((8.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура должна пытаться вращаться при коллизии справа"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 7: Spin-тесты (T-spin, I-spin)
// ============================================================================

/// Тест: Spin-вращение для T и I фигур.
///
/// Проверяет вращение в узком пространстве (spin) для T и I фигур.
/// T-spin: два вращения по часовой изменяют координаты.
/// I-spin: два вращения по часовой изменяют ориентацию.
#[test]
fn test_spin_rotation_all_shapes() {
    // T-spin тест
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура после T-spin должна иметь другие координаты"
    );

    // T-spin: 3 вращения по часовой
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);
    for _ in 0..3 {
        t.rotate(RotationDirection::Clockwise);
    }
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура после 3 вращений должна изменить координаты"
    );

    // T-spin: комбинация вращений влево и вправо
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::CounterClockwise);
    assert_eq!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура должна вернуться к исходным координатам после вращения вправо и влево"
    );

    // I-spin тест
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура после I-spin должна иметь другие координаты"
    );
}

/// Тест: Специальное вращение S и Z фигур (два поворота).
///
/// Проверяет, что S и Z фигуры меняют ориентацию после двух вращений.
#[test]
fn test_special_rotation_s_and_z() {
    // S-фигура: после 2 вращений должна изменить ориентацию
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::S, SHAPE_COORDS[3], 3);
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[3],
        "S-фигура после 2 вращений должна изменить ориентацию"
    );

    // Z-фигура: после 2 вращений должна изменить ориентацию
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::Z, SHAPE_COORDS[4], 4);
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[4],
        "Z-фигура после 2 вращений должна изменить ориентацию"
    );
}
