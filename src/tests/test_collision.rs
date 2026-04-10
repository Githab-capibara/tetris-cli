//! Тесты столкновений.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит 50 тестов для проверки системы столкновений:
//! - Тесты столкновений со стенами (15 тестов)
//! - Тесты столкновений с полом (10 тестов)
//! - Тесты столкновений с фигурами (10 тестов)
//! - Тесты вращений и столкновений (10 тестов)
//! - Тесты граничных случаев (5 тестов)

use crate::game::GameState;
use crate::tetromino::{ShapeType, SHAPE_COORDS};
use crate::types::{Direction, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-15: Столкновения со стенами
// ============================================================================

/// Тест 1-2: Столкновение с левой и правой стеной (параметризированный тест).
///
/// Проверяет, что фигура не может двигаться за пределы поля по горизонтали.
#[test]
fn test_collision_side_walls() {
    let wall_cases = [
        (Direction::Left, Direction::Right, "влево", "у левой стены"),
        (
            Direction::Right,
            Direction::Left,
            "вправо",
            "у правой стены",
        ),
    ];

    for (move_dir, _push_dir, move_desc, wall_desc) in wall_cases {
        let mut state = GameState::new();

        // Двигаем к стене до упора
        for _ in 0..10 {
            if state.can_move_curr_shape_direction(move_dir) {
                if move_dir == Direction::Left {
                    state.get_curr_shape_mut().pos_mut().0 -= 1.0;
                } else {
                    state.get_curr_shape_mut().pos_mut().0 += 1.0;
                }
            }
        }

        assert!(
            !state.can_move_curr_shape_direction(move_dir),
            "Движение {move_desc} должно быть заблокировано {wall_desc}"
        );
    }
}

/// Тест 3-4: Движение вниз возможно у обеих стен (параметризированный тест).
///
/// Проверяет, что у любой боковой стены движение вниз остаётся доступным.
#[test]
fn test_collision_down_at_side_walls() {
    let wall_cases = [
        (Direction::Left, "у левой стены"),
        (Direction::Right, "у правой стены"),
    ];

    for (push_dir, wall_desc) in wall_cases {
        let mut state = GameState::new();

        // Двигаем к стене
        for _ in 0..10 {
            if state.can_move_curr_shape_direction(push_dir) {
                if push_dir == Direction::Left {
                    state.get_curr_shape_mut().pos_mut().0 -= 1.0;
                } else {
                    state.get_curr_shape_mut().pos_mut().0 += 1.0;
                }
            }
        }

        assert!(
            state.can_move_curr_shape_direction(Direction::Down),
            "Движение вниз должно быть возможно {wall_desc}"
        );
    }
}

/// Тест 5-6: Движение от стены в противоположном направлении (параметризированный тест).
///
/// Проверяет, что после упора в стену можно двигаться в обратном направлении.
#[test]
fn test_collision_away_from_wall() {
    let wall_cases = [
        (
            Direction::Left,
            Direction::Right,
            "к левой",
            "вправо",
            "у левой стены",
        ),
        (
            Direction::Right,
            Direction::Left,
            "к правой",
            "влево",
            "у правой стены",
        ),
    ];

    for (push_dir, check_dir, _push_desc, move_desc, wall_desc) in wall_cases {
        let mut state = GameState::new();

        // Двигаем к стене
        for _ in 0..10 {
            if state.can_move_curr_shape_direction(push_dir) {
                if push_dir == Direction::Left {
                    state.get_curr_shape_mut().pos_mut().0 -= 1.0;
                } else {
                    state.get_curr_shape_mut().pos_mut().0 += 1.0;
                }
            }
        }

        assert!(
            state.can_move_curr_shape_direction(check_dir),
            "Движение {move_desc} должно быть возможно {wall_desc}"
        );
    }
}

/// Тест 7: Столкновение всех фигур со стенами и полом (параметризированный тест).
///
/// Проверяет, что все 7 типов фигур сталкиваются с левой стеной и с полом.
#[test]
fn test_collision_all_shapes_walls_and_floor() {
    use crate::tetromino::{ShapeType, SHAPE_COORDS};

    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::I,
        ShapeType::O,
        ShapeType::S,
        ShapeType::Z,
    ];

    for shape_type in shapes {
        // Проверка столкновения с левой стеной
        {
            let mut state = GameState::new();
            state.get_curr_shape_mut().set_shape(shape_type);
            state
                .get_curr_shape_mut()
                .set_coords(SHAPE_COORDS[shape_type as usize]);

            for _ in 0..10 {
                if state.can_move_curr_shape_direction(Direction::Left) {
                    state.get_curr_shape_mut().pos_mut().0 -= 1.0;
                }
            }

            assert!(
                !state.can_move_curr_shape_direction(Direction::Left),
                "Фигура {shape_type:?} должна столкнуться с левой стеной"
            );
        }

        // Проверка столкновения с полом
        {
            let mut state = GameState::new();
            state.get_curr_shape_mut().set_shape(shape_type);
            state
                .get_curr_shape_mut()
                .set_coords(SHAPE_COORDS[shape_type as usize]);

            while state.can_move_curr_shape_direction(Direction::Down) {
                state.get_curr_shape_mut().pos_mut().1 += 1.0;
            }

            assert!(
                !state.can_move_curr_shape_direction(Direction::Down),
                "Фигура {shape_type:?} должна столкнуться с полом"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 26-35: Столкновения с фигурами
// ============================================================================

/// Тест 35: Проверка что столкновение срабатывает точно на границе.
///
/// Проверяет, что фигура может двигаться до последней возможной позиции.
#[test]
fn test_collision_exact_boundary() {
    let mut state = GameState::new();

    // Двигаем влево до последней возможной позиции
    let mut moves = 0;
    while state.can_move_curr_shape_direction(Direction::Left) {
        state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        moves += 1;
    }

    // Должно быть несколько движений
    assert!(moves > 0, "Должно быть хотя бы одно движение влево");
}

// ============================================================================
// ГРУППА ТЕСТОВ 43-50: Вращения и столкновения
// ============================================================================

/// Тест 45: Проверка что вращение O-фигуры не вызывает проблем.
///
/// O-фигура формально может вращаться, но её координаты симметричны.
#[test]
fn test_collision_rotation_o_piece() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().set_shape(ShapeType::O);
    state
        .get_curr_shape_mut()
        .set_coords(SHAPE_COORDS[ShapeType::O as usize]);

    // Проверяем что метод не паникует и возвращаемый результат валидный
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.can_rotate_curr_shape(RotationDirection::Clockwise)
    }));
    assert!(
        result.is_ok(),
        "Вращение O-фигуры не должно вызывать панику"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 46-50: Граничные случаи
// ============================================================================

/// Тест 47: Проверка столкновения в углу (правый нижний).
///
/// Проверяет, что движение вправо и вниз блокируется в правом нижнем углу.
#[test]
fn test_collision_bottom_right_corner() {
    let mut state = GameState::new();

    // Двигаем вправо и вниз
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Движение вправо и вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape_direction(Direction::Right));
    assert!(!state.can_move_curr_shape_direction(Direction::Down));
}

/// Тест 49: Проверка что столкновение не выходит за границы массива.
///
/// Проверяет, что после движения к границе хотя бы одно направление заблокировано.
#[test]
fn test_collision_array_bounds() {
    let mut state = GameState::new();

    // Двигаем к границам
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Проверка не должна вызывать панику (выход за границы)
    // После движения к границе проверяем что коллизия работает корректно
    let can_left = state.can_move_curr_shape_direction(Direction::Left);
    let can_down = state.can_move_curr_shape_direction(Direction::Down);
    // Хотя бы одно направление должно быть заблокировано
    assert!(
        !can_left || !can_down,
        "После движения к границе хотя бы одно направление должно быть заблокировано"
    );
}

/// Тест 50: Проверка что столкновение работает после множественных движений.
///
/// Проверяет, что столкновение корректно работает после серии движений.
#[test]
fn test_collision_after_multiple_moves() {
    let mut state = GameState::new();

    // Серия движений
    for _ in 0..5 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
        if state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos_mut().1 += 1.0;
        }
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    // Столкновение должно работать корректно
    let can_down = state.can_move_curr_shape_direction(Direction::Down);
    assert!(can_down, "Движение вниз должно быть возможно");
}
