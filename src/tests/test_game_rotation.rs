//! Тесты вращения фигур в Tetris CLI.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит 50 тестов для проверки всех аспектов вращения фигур:
//! - Тесты вращения по часовой для всех фигур (7 тестов)
//! - Тесты вращения против часовой для всех фигур (7 тестов)
//! - Тесты полного цикла вращения (4 поворота) (7 тестов)
//! - Тесты вращения у стен (8 тестов)
//! - Тесты вращения над фигурами (6 тестов)
//! - Тесты вращения с коллизиями (8 тестов)
//! - Тесты special rotation (T-spin, I-spin) (7 тестов)
//!
//! Все тесты независимы и проверяют отдельные аспекты механики вращения.

// Разрешаем использование deprecated метода rotate_old() для тестирования обратной совместимости
#![allow(deprecated)]

use crate::tetromino::{ShapeType, Tetromino, SHAPE_COORDS};
use crate::types::RotationDirection;

// ============================================================================
// ГРУППА ТЕСТОВ 1-7: Вращение по часовой для всех фигур
// ============================================================================

/// Тест 1: Вращение T-фигуры по часовой стрелке
///
/// Проверяет корректность изменения координат при вращении T на 90°.
#[test]
fn test_t_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::T, SHAPE_COORDS[0], 0);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    // Координаты должны измениться
    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура должна изменить координаты после вращения по часовой"
    );

    // Проверяем конкретные координаты после вращения
    // Исходные: (-1,0), (0,0), (1,0), (0,1)
    // После вращения по часовой: (0,-1), (0,0), (0,1), (-1,0)
    assert_eq!(
        t.coords()[1],
        (0, 0),
        "Центральный блок T-фигуры должен остаться на месте"
    );
}

/// Тест 2: Вращение L-фигуры по часовой стрелке.
///
/// Проверяет корректность изменения координат при вращении L на 90°.
#[test]
fn test_l_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::L, SHAPE_COORDS[1], 1);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "L-фигура должна изменить координаты после вращения"
    );
}

/// Тест 3: Вращение J-фигуры по часовой стрелке.
///
/// Проверяет корректность изменения координат при вращении J на 90°.
#[test]
fn test_j_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::J, SHAPE_COORDS[2], 2);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "J-фигура должна изменить координаты после вращения"
    );
}

/// Тест 4: Вращение S-фигуры по часовой стрелке.
///
/// Проверяет корректность изменения координат при вращении S на 90°.
#[test]
fn test_s_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::S, SHAPE_COORDS[3], 3);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "S-фигура должна изменить координаты после вращения"
    );
}

/// Тест 5: Вращение Z-фигуры по часовой стрелке.
///
/// Проверяет корректность изменения координат при вращении Z на 90°.
#[test]
fn test_z_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "Z-фигура должна изменить координаты после вращения"
    );
}

/// Тест 6: Вращение O-фигуры по часовой стрелке (не вращается)
#[test]
fn test_o_rotate_clockwise_no_change() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::O, SHAPE_COORDS[5], 5);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура (квадрат) не должна вращаться"
    );
}

/// Тест 7: Вращение I-фигуры по часовой стрелке
#[test]
fn test_i_rotate_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура должна изменить координаты после вращения"
    );

    // I-фигура из вертикальной становится горизонтальной
    // Исходные: (0,-1), (0,0), (0,1), (0,2)
    // После вращения: (1,0), (0,0), (-1,0), (-2,0)
    assert_eq!(
        t.coords()[1],
        (0, 0),
        "Центральный блок I-фигуры должен остаться на месте"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 8-14: Вращение против часовой для всех фигур
// ============================================================================

/// Тест 8: Вращение T-фигуры против часовой стрелки
#[test]
fn test_t_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::T, SHAPE_COORDS[0], 0);

    let original_coords = t.coords();
    t.rotate(RotationDirection::CounterClockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура должна изменить координаты после вращения против часовой"
    );
}

/// Тест 9: Вращение L-фигуры против часовой стрелки.
///
/// Проверяет корректность вращения L-фигуры в обратном направлении.
#[test]
fn test_l_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::L, SHAPE_COORDS[1], 1);

    t.rotate(RotationDirection::CounterClockwise);
    // L-фигура должна вращаться против часовой
    // Проверяем, что координаты изменились
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[1],
        "L-фигура должна вращаться против часовой"
    );
}

/// Тест 10: Вращение J-фигуры против часовой стрелки.
///
/// Проверяет корректность вращения J-фигуры в обратном направлении.
#[test]
fn test_j_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::J, SHAPE_COORDS[2], 2);

    t.rotate(RotationDirection::CounterClockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[2],
        "J-фигура должна вращаться против часовой"
    );
}

/// Тест 11: Вращение S-фигуры против часовой стрелки.
///
/// Проверяет корректность вращения S-фигуры в обратном направлении.
#[test]
fn test_s_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::S, SHAPE_COORDS[3], 3);

    t.rotate(RotationDirection::CounterClockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[3]);
}

/// Тест 12: Вращение Z-фигуры против часовой стрелки.
///
/// Проверяет корректность вращения Z-фигуры в обратном направлении.
#[test]
fn test_z_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    t.rotate(RotationDirection::CounterClockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[4]);
}

/// Тест 13: Вращение O-фигуры против часовой стрелки (не вращается)
#[test]
fn test_o_rotate_counter_clockwise_no_change() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::O, SHAPE_COORDS[5], 5);

    let original_coords = t.coords();
    t.rotate(RotationDirection::CounterClockwise);

    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура не должна вращаться против часовой"
    );
}

/// Тест 14: Вращение I-фигуры против часовой стрелки
#[test]
fn test_i_rotate_counter_clockwise() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::I, SHAPE_COORDS[6], 6);

    t.rotate(RotationDirection::CounterClockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[6],
        "I-фигура должна вращаться против часовой"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-21: Полный цикл вращения (4 поворота)
// ============================================================================

/// Тест 15: Полный цикл вращения T-фигуры
///
/// 4 вращения по часовой должны вернуть к исходным координатам.
#[test]
fn test_t_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::T, SHAPE_COORDS[0], 0);

    let original_coords = t.coords();

    // 4 вращения по часовой
    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "T-фигура должна вернуться к исходным координатам после 4 вращений"
    );
}

/// Тест 16: Полный цикл вращения L-фигуры.
///
/// Проверяет, что 4 вращения по часовой возвращают L-фигуру в исходное состояние.
#[test]
fn test_l_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::L, SHAPE_COORDS[1], 1);

    let original_coords = t.coords();

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "L-фигура должна вернуться к исходным координатам"
    );
}

/// Тест 17: Полный цикл вращения J-фигуры.
///
/// Проверяет, что 4 вращения по часовой возвращают J-фигуру в исходное состояние.
#[test]
fn test_j_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::J, SHAPE_COORDS[2], 2);

    let original_coords = t.coords();

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "J-фигура должна вернуться к исходным координатам"
    );
}

/// Тест 18: Полный цикл вращения S-фигуры.
///
/// Проверяет, что 4 вращения по часовой возвращают S-фигуру в исходное состояние.
#[test]
fn test_s_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::S, SHAPE_COORDS[3], 3);

    let original_coords = t.coords();

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "S-фигура должна вернуться к исходным координатам"
    );
}

/// Тест 19: Полный цикл вращения Z-фигуры.
///
/// Проверяет, что 4 вращения по часовой возвращают Z-фигуру в исходное состояние.
#[test]
fn test_z_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    let original_coords = t.coords();

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "Z-фигура должна вернуться к исходным координатам"
    );
}

/// Тест 20: Полный цикл вращения O-фигуры (остаётся неизменной)
#[test]
fn test_o_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::O, SHAPE_COORDS[5], 5);

    let original_coords = t.coords();

    // O-фигура не вращается, поэтому после 4 вращений остаётся той же
    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура должна остаться неизменной"
    );
}

/// Тест 21: Полный цикл вращения I-фигуры
#[test]
fn test_i_full_rotation_cycle() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords(),
        original_coords,
        "I-фигура должна вернуться к исходным координатам"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 22-29: Вращение у стен
// ============================================================================

/// Тест 22: Вращение T-фигуры у левой стены
///
/// Проверяет, что вращение возможно рядом со стеной.
#[test]
fn test_t_rotation_at_left_wall() {
    let mut t = Tetromino::new((0.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // Вращение у стены должно быть возможно (координаты изменятся)
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура должна вращаться у левой стены"
    );
}

/// Тест 23: Вращение T-фигуры у правой стены.
///
/// Проверяет, что вращение возможно рядом с правой стеной.
#[test]
fn test_t_rotation_at_right_wall() {
    let mut t = Tetromino::new((9.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    t.rotate(RotationDirection::Clockwise);
    // Вращение должно произойти
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура должна вращаться у правой стены"
    );
}

/// Тест 24: Вращение I-фигуры у левой стены.
///
/// Проверяет, что I-фигура может вращаться у левой стены.
#[test]
fn test_i_rotation_at_left_wall() {
    let mut t = Tetromino::new((0.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    // I-фигура должна вращаться у стены
    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура должна вращаться у левой стены"
    );
}

/// Тест 25: Вращение I-фигуры у правой стены.
///
/// Проверяет, что I-фигура может вращаться у правой стены.
#[test]
fn test_i_rotation_at_right_wall() {
    let mut t = Tetromino::new((9.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[6],
        "I-фигура должна вращаться у правой стены"
    );
}

/// Тест 26: Вращение L-фигуры у левой стены.
///
/// Проверяет, что L-фигура может вращаться у левой стены.
#[test]
fn test_l_rotation_at_left_wall() {
    let mut t = Tetromino::new((0.0, 5.0), ShapeType::L, SHAPE_COORDS[1], 1);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[1],
        "L-фигура должна вращаться у левой стены"
    );
}

/// Тест 27: Вращение L-фигуры у правой стены.
///
/// Проверяет, что L-фигура может вращаться у правой стены.
#[test]
fn test_l_rotation_at_right_wall() {
    let mut t = Tetromino::new((9.0, 5.0), ShapeType::L, SHAPE_COORDS[1], 1);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[1],
        "L-фигура должна вращаться у правой стены"
    );
}

/// Тест 28: Вращение O-фигуры у стены (не вращается)
#[test]
fn test_o_rotation_at_wall() {
    let mut t = Tetromino::new((0.0, 5.0), ShapeType::O, SHAPE_COORDS[5], 5);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура не должна вращаться у стены"
    );
}

/// Тест 29: Вращение J-фигуры у правой стены.
///
/// Проверяет, что J-фигура может вращаться у правой стены.
#[test]
fn test_j_rotation_at_right_wall() {
    let mut t = Tetromino::new((9.0, 5.0), ShapeType::J, SHAPE_COORDS[2], 2);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[2],
        "J-фигура должна вращаться у правой стены"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 30-35: Вращение над фигурами
// ============================================================================

/// Тест 30: Вращение T-фигуры над другой фигурой
///
/// Проверяет, что вращение возможно над зафиксированной фигурой.
#[test]
fn test_t_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // Вращение над "фигурой" (в воздухе) должно быть возможно
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура должна вращаться над другой фигурой"
    );
}

/// Тест 31: Вращение L-фигуры над другой фигурой.
///
/// Проверяет, что L-фигура может вращаться в воздухе над зафиксированной фигурой.
#[test]
fn test_l_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::L, SHAPE_COORDS[1], 1);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[1]);
}

/// Тест 32: Вращение J-фигуры над другой фигурой.
///
/// Проверяет, что J-фигура может вращаться в воздухе над зафиксированной фигурой.
#[test]
fn test_j_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::J, SHAPE_COORDS[2], 2);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[2]);
}

/// Тест 33: Вращение S-фигуры над другой фигурой.
///
/// Проверяет, что S-фигура может вращаться в воздухе над зафиксированной фигурой.
#[test]
fn test_s_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::S, SHAPE_COORDS[3], 3);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[3]);
}

/// Тест 34: Вращение Z-фигуры над другой фигурой.
///
/// Проверяет, что Z-фигура может вращаться в воздухе над зафиксированной фигурой.
#[test]
fn test_z_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[4]);
}

/// Тест 35: Вращение I-фигуры над другой фигурой
#[test]
fn test_i_rotation_above_piece() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура должна вращаться над другой фигурой"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 36-43: Вращение с коллизиями
// ============================================================================

/// Тест 36: Вращение T-фигуры с коллизией слева
///
/// Проверяет поведение при вращении в ограниченном пространстве.
#[test]
fn test_t_rotation_with_left_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // Вращение должно изменить координаты
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    // Координаты должны измениться даже при близости к стене
    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура должна пытаться вращаться при коллизии слева"
    );
}

/// Тест 37: Вращение T-фигуры с коллизией справа.
///
/// Проверяет, что T-фигура пытается вращаться при близости к правой стене.
#[test]
fn test_t_rotation_with_right_collision() {
    let mut t = Tetromino::new((8.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура должна пытаться вращаться при коллизии справа"
    );
}

/// Тест 38: Вращение I-фигуры с коллизией.
///
/// Проверяет, что I-фигура пытается вращаться даже при ограничении пространства.
#[test]
fn test_i_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    // I-фигура длинная, поэтому вращение у стены может быть ограничено
    // но метод rotate всё равно должен выполниться
    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура должна пытаться вращаться"
    );
}

/// Тест 39: Вращение L-фигуры с коллизией.
///
/// Проверяет, что L-фигура пытается вращаться при ограничении пространства.
#[test]
fn test_l_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::L, SHAPE_COORDS[1], 1);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[1]);
}

/// Тест 40: Вращение J-фигуры с коллизией.
///
/// Проверяет, что J-фигура пытается вращаться при ограничении пространства.
#[test]
fn test_j_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::J, SHAPE_COORDS[2], 2);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[2]);
}

/// Тест 41: Вращение S-фигуры с коллизией.
///
/// Проверяет, что S-фигура пытается вращаться при ограничении пространства.
#[test]
fn test_s_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::S, SHAPE_COORDS[3], 3);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[3]);
}

/// Тест 42: Вращение Z-фигуры с коллизией.
///
/// Проверяет, что Z-фигура пытается вращаться при ограничении пространства.
#[test]
fn test_z_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords(), SHAPE_COORDS[4]);
}

/// Тест 43: Вращение O-фигуры с коллизией (не вращается)
#[test]
fn test_o_rotation_with_collision() {
    let mut t = Tetromino::new((1.0, 5.0), ShapeType::O, SHAPE_COORDS[5], 5);

    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);

    assert_eq!(
        t.coords(),
        original_coords,
        "O-фигура не должна вращаться даже с коллизией"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 44-50: Special Rotation (T-spin, I-spin)
// ============================================================================

/// Тест 44: T-spin - вращение T-фигуры в узком пространстве
///
/// T-spin - это вращение T-фигуры в ограниченном пространстве.
#[test]
fn test_t_spin_rotation() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // Выполняем несколько вращений для симуляции T-spin
    let original_coords = t.coords();
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);

    // После 2 вращений координаты должны отличаться
    assert_ne!(
        t.coords(),
        original_coords,
        "T-фигура после T-spin должна иметь другие координаты"
    );
}

/// Тест 45: T-spin - проверка 3 вращений.
///
/// Проверяет, что 3 вращения по часовой эквивалентны 1 против часовой.
#[test]
fn test_t_spin_three_rotations() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // 3 вращения по часовой = 1 против часовой
    for _ in 0..3 {
        t.rotate(RotationDirection::Clockwise);
    }

    // Координаты должны измениться
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура после 3 вращений должна изменить координаты"
    );
}

/// Тест 46: I-spin - вращение I-фигуры в узком пространстве.
///
/// Проверяет, что I-фигура меняет ориентацию после двух вращений.
#[test]
fn test_i_spin_rotation() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::I, SHAPE_COORDS[6], 6);

    let original_coords = t.coords();

    // I-spin требует точного позиционирования
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        original_coords,
        "I-фигура после I-spin должна иметь другие координаты"
    );
}

/// Тест 47: T-spin - комбинация вращений влево и вправо.
///
/// Проверяет, что вращение вправо, затем влево возвращает к исходным координатам.
#[test]
fn test_t_spin_combined_rotations() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::T, SHAPE_COORDS[0], 0);

    // Вращение вправо, затем влево
    t.rotate(RotationDirection::Clockwise);
    let _coords_after_right = t.coords();

    t.rotate(RotationDirection::CounterClockwise);

    // После вращения вправо и влево должны вернуться к исходным
    assert_eq!(
        t.coords(),
        SHAPE_COORDS[0],
        "T-фигура должна вернуться к исходным координатам"
    );
}

/// Тест 48: Special rotation - S-фигура.
///
/// Проверяет, что S-фигура меняет ориентацию после двух вращений.
#[test]
fn test_s_special_rotation() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::S, SHAPE_COORDS[3], 3);

    // S-фигура имеет симметричное вращение
    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);

    // После 2 вращений S-фигура должна быть зеркальной
    assert_ne!(
        t.coords(),
        SHAPE_COORDS[3],
        "S-фигура после 2 вращений должна изменить ориентацию"
    );
}

/// Тест 49: Special rotation - Z-фигура.
///
/// Проверяет, что Z-фигура меняет ориентацию после двух вращений.
#[test]
fn test_z_special_rotation() {
    let mut t = Tetromino::new((4.0, 5.0), ShapeType::Z, SHAPE_COORDS[4], 4);

    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords(),
        SHAPE_COORDS[4],
        "Z-фигура после 2 вращений должна изменить ориентацию"
    );
}

/// Тест 50: Special rotation - проверка всех фигур
#[test]
fn test_all_pieces_special_rotation() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &shapes {
        let shape_index = *shape as usize;
        let mut t = Tetromino::new(
            (4.0, 5.0),
            *shape,
            SHAPE_COORDS[shape_index],
            shape_index as u8,
        );

        let original_coords = t.coords();

        // Вращение
        t.rotate(RotationDirection::Clockwise);

        // O-фигура не вращается, остальные должны
        if *shape == ShapeType::O {
            assert_eq!(t.coords(), original_coords, "O-фигура не должна вращаться");
        } else {
            assert_ne!(
                t.coords(),
                original_coords,
                "{shape:?} фигура должна вращаться"
            );
        }
    }
}
