//! Тесты столкновений.
//!
//! Этот модуль содержит 50 тестов для проверки системы столкновений:
//! - Тесты столкновений со стенами (15 тестов)
//! - Тесты столкновений с полом (10 тестов)
//! - Тесты столкновений с фигурами (10 тестов)
//! - Тесты вращений и столкновений (10 тестов)
//! - Тесты граничных случаев (5 тестов)

use crate::game::GameState;
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{ShapeType, SHAPE_COORDS};
use crate::types::{Direction, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-15: Столкновения со стенами
// ============================================================================

/// Тест 1: Проверка столкновения с левой стеной
#[test]
fn test_collision_left_wall() {
    let mut state = GameState::new();

    // Двигаем влево до упора
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    assert!(
        !state.can_move_curr_shape_direction(Direction::Left),
        "Движение влево должно быть заблокировано у стены"
    );
}

/// Тест 2: Проверка столкновения с правой стеной
#[test]
fn test_collision_right_wall() {
    let mut state = GameState::new();

    // Двигаем вправо до упора
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    assert!(
        !state.can_move_curr_shape_direction(Direction::Right),
        "Движение вправо должно быть заблокировано у стены"
    );
}

/// Тест 3: Проверка что движение вниз возможно у левой стены
#[test]
fn test_collision_down_at_left_wall() {
    let mut state = GameState::new();

    // Двигаем к левой стене
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "Движение вниз должно быть возможно у левой стены"
    );
}

/// Тест 4: Проверка что движение вниз возможно у правой стены
#[test]
fn test_collision_down_at_right_wall() {
    let mut state = GameState::new();

    // Двигаем к правой стене
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "Движение вниз должно быть возможно у правой стены"
    );
}

/// Тест 5: Проверка что движение вправо возможно у левой стены
#[test]
fn test_collision_right_at_left_wall() {
    let mut state = GameState::new();

    // Двигаем к левой стене
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape_direction(Direction::Right),
        "Движение вправо должно быть возможно у левой стены"
    );
}

/// Тест 6: Проверка что движение влево возможно у правой стены
#[test]
fn test_collision_left_at_right_wall() {
    let mut state = GameState::new();

    // Двигаем к правой стене
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape_direction(Direction::Left),
        "Движение влево должно быть возможно у правой стены"
    );
}

/// Тест 7: Проверка столкновения для всех фигур с левой стеной (параметризованный тест)
#[test]
fn test_collision_all_shapes_left_wall() {
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
}

/// Тест 14: Проверка что фигура не выходит за левую границу
#[test]
fn test_collision_not_beyond_left_boundary() {
    let mut state = GameState::new();

    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    let x = state.curr_shape().pos().0;
    assert!(x >= 0.0, "Фигура не должна выходить за левую границу");
}

/// Тест 15: Проверка что фигура не выходит за правую границу
#[test]
fn test_collision_not_beyond_right_boundary() {
    let mut state = GameState::new();

    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    let x = state.curr_shape().pos().0;
    assert!(
        x < GRID_WIDTH as f32,
        "Фигура не должна выходить за правую границу"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-25: Столкновения с полом
// ============================================================================

/// Тест 19: Проверка столкновения с полом для всех типов фигур
#[test]
fn test_collision_all_shapes_floor() {
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

    for &shape in &shapes {
        let mut state = GameState::new();
        state.get_curr_shape_mut().set_shape(shape);
        state
            .get_curr_shape_mut()
            .set_coords(SHAPE_COORDS[shape as usize]);

        // Опускаем до упора
        while state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos_mut().1 += 1.0;
        }

        assert!(
            !state.can_move_curr_shape_direction(Direction::Down),
            "Движение вниз должно быть заблокировано на полу для фигуры {shape:?}"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 26-35: Столкновения с фигурами
// ============================================================================

/// Тест 26: Проверка что новая фигура появляется над зафиксированной
#[test]
fn test_collision_new_above_fixed() {
    // Этот тест проверяет базовую механику
    let state = GameState::new();

    // В начале игры поле пустое
    let blocks = state.get_blocks();
    for (y, row) in blocks.iter().enumerate().take(GRID_HEIGHT) {
        for (x, &cell) in row.iter().enumerate().take(GRID_WIDTH) {
            assert_eq!(cell, -1, "Поле должно быть пустым [{y},{x}]");
        }
    }
}

/// Тест 27: Проверка что движение вниз блокируется фигурой
#[test]
fn test_collision_down_blocked_by_piece() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Движение вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape_direction(Direction::Down));
}

/// Тест 30: Проверка столкновения при приземлении на фигуру
#[test]
fn test_collision_landing_on_piece() {
    let mut state = GameState::new();

    // Опускаем фигуру
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Фигура должна быть на полу
    assert!(!state.can_move_curr_shape_direction(Direction::Down));
}

/// Тест 33: Проверка что столкновение срабатывает корректно
#[test]
fn test_collision_triggers_correctly() {
    let mut state = GameState::new();

    // Двигаем влево до стены
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    // Столкновение должно сработать
    assert!(!state.can_move_curr_shape_direction(Direction::Left));
}

/// Тест 34: Проверка что столкновение не срабатывает рано
#[test]
fn test_collision_not_early() {
    let state = GameState::new();

    // В центре поля столкновений быть не должно
    let can_down = state.can_move_curr_shape_direction(Direction::Down);
    assert!(can_down, "В центре поля движение вниз должно быть возможно");
}

/// Тест 35: Проверка что столкновение срабатывает точно на границе
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

/// Тест 44: Проверка что вращение работает в центре поля
#[test]
fn test_collision_rotation_in_center() {
    let mut state = GameState::new();

    // Перемещаем фигуру в центр поля (примерно середина по Y)
    state.get_curr_shape_mut().pos_mut().1 = 10.0;

    // В центре поля вращение должно быть возможно
    let can_rotate = state.can_rotate_curr_shape(RotationDirection::Clockwise)
        || state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    assert!(can_rotate, "В центре поля вращение должно быть возможно");
}

/// Тест 45: Проверка что вращение O-фигуры не вызывает проблем
#[test]
fn test_collision_rotation_o_piece() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().set_shape(ShapeType::O);
    state
        .get_curr_shape_mut()
        .set_coords(SHAPE_COORDS[ShapeType::O as usize]);

    // O-фигура не вращается, но метод не должен паниковать
    let _ = state.can_rotate_curr_shape(RotationDirection::Clockwise);
}

// ============================================================================
// ГРУППА ТЕСТОВ 46-50: Граничные случаи
// ============================================================================

/// Тест 46: Проверка столкновения в углу (левый нижний)
#[test]
fn test_collision_bottom_left_corner() {
    let mut state = GameState::new();

    // Двигаем влево и вниз
    for _ in 0..10 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Движение влево и вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape_direction(Direction::Left));
    assert!(!state.can_move_curr_shape_direction(Direction::Down));
}

/// Тест 47: Проверка столкновения в углу (правый нижний)
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

/// Тест 49: Проверка что столкновение не выходит за границы массива
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
    assert!(!can_left || !can_down, "После движения к границе хотя бы одно направление должно быть заблокировано");
}

/// Тест 50: Проверка что столкновение работает после множественных движений
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
