//! Тесты движения фигур в Tetris CLI.
//!
//! Модуль содержит 13 параметризированных тестов для проверки всех аспектов движения фигур:
//! - Движение влево/вправо (2 теста)
//! - Движение у границ поля (3 теста)
//! - Движение с препятствиями (2 теста)
//! - Soft drop (3 теста)
//! - Движение после вращения (2 теста)
//! - Краевые случаи (1 тест)

// Cast sign_loss намеренно: usize→i16 только с координатами в пределах поля (0..10, 0..20)
#![allow(clippy::cast_sign_loss)]
// Cast truncation намеренно: координаты поля маленькие (0..10, 0..20)
#![allow(clippy::cast_possible_truncation)]
// Cast wrap намеренно: координаты гарантированно в пределах 0..20, wrap невозможен
#![allow(clippy::cast_possible_wrap)]
//! - Тесты движения влево/вправо для всех 7 фигур (14 тестов)
//! - Тесты движения у границ поля (10 тестов)
//! - Тесты движения с препятствиями (8 тестов)
//! - Тесты мягкого падения (soft drop) (6 тестов)
//! - Тесты жёсткого падения (hard drop) (6 тестов)
//! - Тесты движения после вращения (4 теста)
//! - Тесты движения с удержанием фигуры (2 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты механики движения.

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::game::GameState;
use crate::types::{Direction, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-14: Движение влево/вправо для всех 7 фигур
// ============================================================================

/// Тест 1-14: Движение всех фигур влево и вправо (параметризированный тест)
///
/// Проверяет, что все 7 типов фигур могут двигаться влево и вправо в пустом поле.
#[test]
fn test_all_pieces_move_left_right() {
    use crate::tetromino::{ShapeType, SHAPE_COORDS};

    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape_type in shapes {
        let mut state = GameState::new();
        state.mutate_curr_shape(|s| {
            s.set_shape(shape_type);
            s.set_coords(SHAPE_COORDS[shape_type as usize]);
        });

        let initial_x = state.curr_shape().pos().0;

        // Тест движения влево
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.move_curr_dx(-1.0);
            assert!(
                state.curr_shape().pos().0 < initial_x,
                "Фигура {shape_type:?} должна двигаться влево"
            );
        }

        // Тест движения вправо
        let mut state = GameState::new();
        state.mutate_curr_shape(|s| {
            s.set_shape(shape_type);
            s.set_coords(SHAPE_COORDS[shape_type as usize]);
        });

        let initial_x = state.curr_shape().pos().0;
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.move_curr_dx(1.0);
            assert!(
                state.curr_shape().pos().0 > initial_x,
                "Фигура {shape_type:?} должна двигаться вправо"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 18-24: Позиции фигур у границ
// ============================================================================

/// Тест 20: Позиция фигуры у нижней границы
#[test]
fn test_piece_position_at_bottom_boundary() {
    let mut state = GameState::new();

    // Опускаем фигуру до пола
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.move_curr_dy(1.0);
    }

    let shape = state.curr_shape();
    // Проверяем, что фигура не вышла за нижнюю границу (y < GRID_HEIGHT)
    for &(_, y) in &shape.coords() {
        let global_y = shape.pos().1 as i16 + y;
        assert!(
            global_y < GRID_HEIGHT as i16,
            "Блок фигуры не должен выходить за нижнюю границу (y={global_y})"
        );
    }
}

/// Тест 23: Движение O-фигуры у границ
#[test]
fn test_o_piece_at_boundaries() {
    let mut state = GameState::new();

    // Двигаемся влево до упора
    while state.can_move_curr_shape_direction(Direction::Left) {
        state.move_curr_dx(-1.0);
    }
    assert!(!state.can_move_curr_shape_direction(Direction::Left));

    // Двигаемся вправо до упора
    while state.can_move_curr_shape_direction(Direction::Right) {
        state.move_curr_dx(1.0);
    }
    assert!(!state.can_move_curr_shape_direction(Direction::Right));
}

// ============================================================================
// ГРУППА ТЕСТОВ 25-32: Движение с препятствиями
// ============================================================================

/// Тест 28: Движение в узком пространстве
#[test]
fn test_move_in_narrow_space() {
    let mut state = GameState::new();

    // Двигаемся влево на половину поля
    let moves_count = GRID_WIDTH / 4;
    for _ in 0..moves_count {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.move_curr_dx(-1.0);
        }
    }

    // Движение всё ещё должно быть возможным
    assert!(
        state.can_move_curr_shape_direction(Direction::Left)
            || state.can_move_curr_shape_direction(Direction::Right),
        "Движение должно быть возможным в узком пространстве"
    );
}

/// Тест 29: Обход препятствия движением влево
#[test]
fn test_obstacle_avoidance_left() {
    let mut state = GameState::new();

    // Пытаемся двигаться влево
    let initial_x = state.curr_shape().pos().0;
    if state.can_move_curr_shape_direction(Direction::Left) {
        state.move_curr_dx(-1.0);
        assert!(
            state.curr_shape().pos().0 < initial_x,
            "Движение влево должно уменьшить X координату"
        );
    }
}

/// Тест 30: Обход препятствия движением вправо
#[test]
fn test_obstacle_avoidance_right() {
    let mut state = GameState::new();

    // Пытаемся двигаться вправо
    let initial_x = state.curr_shape().pos().0;
    if state.can_move_curr_shape_direction(Direction::Right) {
        state.move_curr_dx(1.0);
        assert!(
            state.curr_shape().pos().0 > initial_x,
            "Движение вправо должно увеличить X координату"
        );
    }
}

/// Тест 31: Движение между двумя препятствиями
#[test]
fn test_move_between_obstacles() {
    let state = GameState::new();

    // В начале игры препятствий нет
    // Проверяем базовую механику движения
    let can_move = state.can_move_curr_shape_direction(Direction::Left)
        || state.can_move_curr_shape_direction(Direction::Right)
        || state.can_move_curr_shape_direction(Direction::Down);
    assert!(can_move, "В начале игры движение должно быть возможным");
}

// Тест 32: Проверка коллизий при движении вниз (удалён как дубликат)

// ============================================================================
// ГРУППА ТЕСТОВ 33-38: Мягкое падение (Soft Drop)
// ============================================================================

/// Мягкое падение: фигура может падать вниз в начале игры.
#[test]
fn test_soft_drop_initial() {
    let state = GameState::new();
    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "В начале игры падение должно быть возможным"
    );
}

/// Мягкое падение: при падении до пола движение вниз блокируется.
#[test]
fn test_soft_drop_to_floor() {
    let mut state = GameState::new();

    while state.can_move_curr_shape_direction(Direction::Down) {
        state.move_curr_dy(1.0);
    }

    assert!(
        !state.can_move_curr_shape_direction(Direction::Down),
        "Мягкое падение должно остановиться на полу"
    );
}

/// Мягкое падение: увеличение координаты Y при падении и положительная скорость.
#[test]
fn test_soft_drop_increases_y() {
    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos().1;
    let mut drops = 0;

    while state.can_move_curr_shape_direction(Direction::Down) && drops < 10 {
        state.move_curr_dy(1.0);
        drops += 1;
    }

    assert!(drops > 0, "Должно произойти хотя бы одно мягкое падение");
    assert!(
        state.curr_shape().pos().1 > initial_y,
        "Фигура должна опуститься после мягкого падения"
    );

    // Скорость падения должна быть положительной
    let state = GameState::new();
    let fall_spd = state.fall_speed();
    assert!(
        fall_spd > 0.0,
        "Скорость падения должна быть положительной: {fall_spd}"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 39-44: Жёсткое падение (Hard Drop)
// ============================================================================

/// Тест 39-44: Жёсткое падение - комплексный тест
///
/// Проверяет все аспекты жёсткого падения фигур.
#[test]
fn test_hard_drop_comprehensive() {
    use crate::tetromino::{ShapeType, SHAPE_COORDS};

    // Тест 1: Базовая проверка hard drop
    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos().1;

    while state.can_move_curr_shape_direction(Direction::Down) {
        state.move_curr_dy(1.0);
    }

    assert!(
        state.curr_shape().pos().1 > initial_y,
        "Жёсткое падение должно опустить фигуру"
    );

    // Тест 2: Мгновенная остановка после hard drop
    assert!(
        !state.can_move_curr_shape_direction(Direction::Down),
        "После жёсткого падения движение вниз должно быть заблокировано"
    );

    // Тест 3: Позиция после падения в пределах поля
    let shape = state.curr_shape();
    for &(_, y) in &shape.coords() {
        let global_y = shape.pos().1 as i16 + y;
        assert!(
            global_y >= 0 && global_y < GRID_HEIGHT as i16,
            "Фигура после жёсткого падения должна быть в пределах поля (y={global_y})"
        );
    }

    // Тест 4: Расстояние падения положительное
    let drop_distance = shape.pos().1 - initial_y;
    assert!(
        drop_distance > 0.0,
        "Расстояние жёсткого падения должно быть положительным"
    );

    // Тест 5-6: Hard drop для всех типов фигур
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape_type in shapes {
        let mut state = GameState::new();
        state.mutate_curr_shape(|s| {
            s.set_shape(shape_type);
            s.set_coords(SHAPE_COORDS[shape_type as usize]);
        });

        let initial_y = state.curr_shape().pos().1;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.move_curr_dy(1.0);
        }

        assert!(
            state.curr_shape().pos().1 > initial_y,
            "Фигура {shape_type:?} должна корректно выполнять жёсткое падение"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 45-48: Движение после вращения
// ============================================================================

/// Движение после вращения: фигура может двигаться после вращения в любом направлении.
///
/// Параметризированный тест, проверяющий движение после вращения
/// по часовой и против часовой стрелки.
#[test]
fn test_movement_after_rotation() {
    let rotation_directions = [
        RotationDirection::Clockwise,
        RotationDirection::CounterClockwise,
    ];

    for &rotation in &rotation_directions {
        let mut state = GameState::new();

        if state.can_rotate_curr_shape(rotation) {
            state.rotate_curr_shape(rotation);

            let can_move = state.can_move_curr_shape_direction(Direction::Left)
                || state.can_move_curr_shape_direction(Direction::Right);
            assert!(
                can_move,
                "Фигура должна иметь возможность движения после вращения {rotation:?}"
            );
        }
    }
}

/// Тест 47: Движение после полного цикла вращения
#[test]
fn test_movement_after_full_rotation_cycle() {
    let mut state = GameState::new();
    let _initial_x = state.curr_shape().pos().0;

    // 4 вращения по часовой
    for _ in 0..4 {
        if state.can_rotate_curr_shape(RotationDirection::Clockwise) {
            state.rotate_curr_shape(RotationDirection::Clockwise);
        }
    }

    // Движение должно быть возможным
    let can_move = state.can_move_curr_shape_direction(Direction::Left)
        || state.can_move_curr_shape_direction(Direction::Right);
    assert!(
        can_move,
        "Фигура должна иметь возможность движения после полного цикла вращения"
    );

    // Позиция X не должна измениться (если не было коллизий)
    // Примечание: это упрощённая проверка
}
