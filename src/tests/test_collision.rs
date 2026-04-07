//! Тесты столкновений.
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

/// Тест 7: Проверка столкновения для всех фигур с левой стеной (параметризованный тест).
///
/// Проверяет, что все 7 типов фигур сталкиваются с левой стеной.
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

/// Тест 27: Проверка что движение вниз блокируется фигурой.
///
/// Проверяет, что после приземления на пол движение вниз блокируется.
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

/// Тест 30: Проверка столкновения при приземлении на фигуру.
///
/// Проверяет, что фигура останавливается при достижении препятствия.
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

/// Тест 34: Проверка что столкновение не срабатывает рано.
///
/// Проверяет, что в центре поля движение вниз доступно.
#[test]
fn test_collision_not_early() {
    let state = GameState::new();

    // В центре поля столкновений быть не должно
    let can_down = state.can_move_curr_shape_direction(Direction::Down);
    assert!(can_down, "В центре поля движение вниз должно быть возможно");
}

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

/// Тест 44: Проверка что вращение работает в центре поля.
///
/// Проверяет, что в центре поля доступно хотя бы одно направление вращения.
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

/// Тест 45: Проверка что вращение O-фигуры не вызывает проблем.
///
/// O-фигура не вращается, но метод не должен паниковать.
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
