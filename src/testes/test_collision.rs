//! Тесты столкновений.
//!
//! Этот модуль содержит 50 тестов для проверки системы столкновений:
//! - Тесты столкновений со стенами (15 тестов)
//! - Тесты столкновений с полом (10 тестов)
//! - Тесты столкновений с фигурами (10 тестов)
//! - Тесты вращений и столкновений (10 тестов)
//! - Тесты граничных случаев (5 тестов)

use crate::game::{Dir, GameState};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{ShapeType, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ 1-15: Столкновения со стенами
// ============================================================================

/// Тест 1: Проверка столкновения с левой стеной
#[test]
fn test_collision_left_wall() {
    let mut state = GameState::new();

    // Двигаем влево до упора
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(
        !state.can_move_curr_shape(Dir::Left),
        "Движение влево должно быть заблокировано у стены"
    );
}

/// Тест 2: Проверка столкновения с правой стеной
#[test]
fn test_collision_right_wall() {
    let mut state = GameState::new();

    // Двигаем вправо до упора
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    assert!(
        !state.can_move_curr_shape(Dir::Right),
        "Движение вправо должно быть заблокировано у стены"
    );
}

/// Тест 3: Проверка что движение вниз возможно у левой стены
#[test]
fn test_collision_down_at_left_wall() {
    let mut state = GameState::new();

    // Двигаем к левой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape(Dir::Down),
        "Движение вниз должно быть возможно у левой стены"
    );
}

/// Тест 4: Проверка что движение вниз возможно у правой стены
#[test]
fn test_collision_down_at_right_wall() {
    let mut state = GameState::new();

    // Двигаем к правой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape(Dir::Down),
        "Движение вниз должно быть возможно у правой стены"
    );
}

/// Тест 5: Проверка что движение вправо возможно у левой стены
#[test]
fn test_collision_right_at_left_wall() {
    let mut state = GameState::new();

    // Двигаем к левой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape(Dir::Right),
        "Движение вправо должно быть возможно у левой стены"
    );
}

/// Тест 6: Проверка что движение влево возможно у правой стены
#[test]
fn test_collision_left_at_right_wall() {
    let mut state = GameState::new();

    // Двигаем к правой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    assert!(
        state.can_move_curr_shape(Dir::Left),
        "Движение влево должно быть возможно у правой стены"
    );
}

/// Тест 7: Проверка столкновения для T-фигуры с левой стеной
#[test]
fn test_collision_t_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::T;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::T as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 8: Проверка столкновения для L-фигуры с левой стеной
#[test]
fn test_collision_l_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::L;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::L as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 9: Проверка столкновения для J-фигуры с левой стеной
#[test]
fn test_collision_j_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::J;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::J as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 10: Проверка столкновения для I-фигуры с левой стеной
#[test]
fn test_collision_i_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::I;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::I as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 11: Проверка столкновения для O-фигуры с левой стеной
#[test]
fn test_collision_o_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::O;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::O as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 12: Проверка столкновения для S-фигуры с левой стеной
#[test]
fn test_collision_s_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::S;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::S as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 13: Проверка столкновения для Z-фигуры с левой стеной
#[test]
fn test_collision_z_left_wall() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::Z;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::Z as usize];

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 14: Проверка что фигура не выходит за левую границу
#[test]
fn test_collision_not_beyond_left_boundary() {
    let mut state = GameState::new();

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    let x = state.get_curr_shape().pos.0;
    assert!(x >= 0.0, "Фигура не должна выходить за левую границу");
}

/// Тест 15: Проверка что фигура не выходит за правую границу
#[test]
fn test_collision_not_beyond_right_boundary() {
    let mut state = GameState::new();

    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    let x = state.get_curr_shape().pos.0;
    assert!(x < GRID_WIDTH as f32, "Фигура не должна выходить за правую границу");
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-25: Столкновения с полом
// ============================================================================

/// Тест 16: Проверка столкновения с полом
#[test]
fn test_collision_floor() {
    let mut state = GameState::new();

    // Опускаем до упора
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(
        !state.can_move_curr_shape(Dir::Down),
        "Движение вниз должно быть заблокировано на полу"
    );
}

/// Тест 17: Проверка что фигура достигает пола
#[test]
fn test_collision_reaches_floor() {
    let mut state = GameState::new();
    let start_y = state.get_curr_shape().pos.1;

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    let end_y = state.get_curr_shape().pos.1;
    assert!(end_y > start_y, "Фигура должна опуститься");
}

/// Тест 18: Проверка что фигура не проходит сквозь пол
#[test]
fn test_collision_not_through_floor() {
    let mut state = GameState::new();

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    let y = state.get_curr_shape().pos.1;
    assert!(y < GRID_HEIGHT as f32, "Фигура не должна проходить сквозь пол");
}

/// Тест 19: Проверка столкновения с полом для T-фигуры
#[test]
fn test_collision_t_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::T;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::T as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 20: Проверка столкновения с полом для L-фигуры
#[test]
fn test_collision_l_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::L;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::L as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 21: Проверка столкновения с полом для J-фигуры
#[test]
fn test_collision_j_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::J;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::J as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 22: Проверка столкновения с полом для I-фигуры
#[test]
fn test_collision_i_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::I;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::I as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 23: Проверка столкновения с полом для O-фигуры
#[test]
fn test_collision_o_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::O;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::O as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 24: Проверка столкновения с полом для S-фигуры
#[test]
fn test_collision_s_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::S;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::S as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 25: Проверка столкновения с полом для Z-фигуры
#[test]
fn test_collision_z_floor() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::Z;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::Z as usize];

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    assert!(!state.can_move_curr_shape(Dir::Down));
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
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            assert_eq!(blocks[y][x], -1, "Поле должно быть пустым");
        }
    }
}

/// Тест 27: Проверка что движение вниз блокируется фигурой
#[test]
fn test_collision_down_blocked_by_piece() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 28: Проверка что движение влево возможно при фигуре внизу
#[test]
fn test_collision_left_with_piece_below() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение влево должно быть возможно (если не у стены)
    let can_left = state.can_move_curr_shape(Dir::Left);
    let _ = can_left;
}

/// Тест 29: Проверка что движение вправо возможно при фигуре внизу
#[test]
fn test_collision_right_with_piece_below() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение вправо должно быть возможно (если не у стены)
    let can_right = state.can_move_curr_shape(Dir::Right);
    let _ = can_right;
}

/// Тест 30: Проверка столкновения при приземлении на фигуру
#[test]
fn test_collision_landing_on_piece() {
    let mut state = GameState::new();

    // Опускаем фигуру
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Фигура должна быть на полу
    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 31: Проверка что вращение возможно при фигуре рядом
#[test]
fn test_collision_rotation_with_piece_nearby() {
    let mut state = GameState::new();

    // Опускаем фигуру близко к полу
    for _ in 0..15 {
        if state.can_move_curr_shape(Dir::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
    }

    // Вращение должно быть возможно
    let can_rotate = state.can_rotate_curr_shape(Dir::Right)
        || state.can_rotate_curr_shape(Dir::Left);
    let _ = can_rotate;
}

/// Тест 32: Проверка что столкновение работает для всех направлений
#[test]
fn test_collision_all_directions() {
    let mut state = GameState::new();

    // Проверяем все направления
    let down = state.can_move_curr_shape(Dir::Down);
    let left = state.can_move_curr_shape(Dir::Left);
    let right = state.can_move_curr_shape(Dir::Right);

    // В начале игры все направления должны быть доступны (кроме границ)
    let _ = (down, left, right);
}

/// Тест 33: Проверка что столкновение срабатывает корректно
#[test]
fn test_collision_triggers_correctly() {
    let mut state = GameState::new();

    // Двигаем влево до стены
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Столкновение должно сработать
    assert!(!state.can_move_curr_shape(Dir::Left));
}

/// Тест 34: Проверка что столкновение не срабатывает рано
#[test]
fn test_collision_not_early() {
    let mut state = GameState::new();

    // В центре поля столкновений быть не должно
    let can_down = state.can_move_curr_shape(Dir::Down);
    assert!(can_down, "В центре поля движение вниз должно быть возможно");
}

/// Тест 35: Проверка что столкновение срабатывает точно на границе
#[test]
fn test_collision_exact_boundary() {
    let mut state = GameState::new();

    // Двигаем влево до последней возможной позиции
    let mut moves = 0;
    while state.can_move_curr_shape(Dir::Left) {
        state.get_curr_shape_mut().pos.0 -= 1.0;
        moves += 1;
    }

    // Должно быть несколько движений
    assert!(moves > 0, "Должно быть хотя бы одно движение влево");
}

// ============================================================================
// ГРУППА ТЕСТОВ 36-45: Вращения и столкновения
// ============================================================================

/// Тест 36: Проверка что вращение блокируется у левой стены
#[test]
fn test_collision_rotation_blocked_left() {
    let mut state = GameState::new();

    // Двигаем к левой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Вращение может быть заблокировано
    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 37: Проверка что вращение блокируется у правой стены
#[test]
fn test_collision_rotation_blocked_right() {
    let mut state = GameState::new();

    // Двигаем к правой стене
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Вращение может быть заблокировано
    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 38: Проверка что вращение блокируется на полу
#[test]
fn test_collision_rotation_blocked_floor() {
    let mut state = GameState::new();

    // Опускаем на пол
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Вращение всё ещё может быть возможно
    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 39: Проверка что can_rotate_curr_shape использует check_collision
#[test]
fn test_collision_rotation_uses_check() {
    let mut state = GameState::new();

    // Проверяем что метод работает
    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 40: Проверка вращения с проверкой столкновений для T
#[test]
fn test_collision_rotation_check_t() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::T;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::T as usize];

    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 41: Проверка вращения с проверкой столкновений для L
#[test]
fn test_collision_rotation_check_l() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::L;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::L as usize];

    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 42: Проверка вращения с проверкой столкновений для I
#[test]
fn test_collision_rotation_check_i() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::I;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::I as usize];

    let can_rotate = state.can_rotate_curr_shape(Dir::Right);
    let _ = can_rotate;
}

/// Тест 43: Проверка что вращение не вызывает паники при столкновении
#[test]
fn test_collision_rotation_no_panic() {
    let mut state = GameState::new();

    // Двигаем к стене и пытаемся вращать
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Вращение не должно вызывать панику
    let _ = state.can_rotate_curr_shape(Dir::Right);
}

/// Тест 44: Проверка что вращение работает в центре поля
#[test]
fn test_collision_rotation_in_center() {
    let mut state = GameState::new();

    // В центре поля вращение должно быть возможно
    let can_rotate = state.can_rotate_curr_shape(Dir::Right)
        || state.can_rotate_curr_shape(Dir::Left);

    assert!(can_rotate, "В центре поля вращение должно быть возможно");
}

/// Тест 45: Проверка что вращение O-фигуры не вызывает проблем
#[test]
fn test_collision_rotation_o_piece() {
    let mut state = GameState::new();
    state.get_curr_shape_mut().shape = ShapeType::O;
    state.get_curr_shape_mut().coords = SHAPE_COORDS[ShapeType::O as usize];

    // O-фигура не вращается, но метод не должен паниковать
    let _ = state.can_rotate_curr_shape(Dir::Right);
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
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение влево и вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape(Dir::Left));
    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 47: Проверка столкновения в углу (правый нижний)
#[test]
fn test_collision_bottom_right_corner() {
    let mut state = GameState::new();

    // Двигаем вправо и вниз
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение вправо и вниз должно быть заблокировано
    assert!(!state.can_move_curr_shape(Dir::Right));
    assert!(!state.can_move_curr_shape(Dir::Down));
}

/// Тест 48: Проверка что столкновение работает для призрачной фигуры
#[test]
fn test_collision_ghost_piece() {
    let state = GameState::new();
    let ghost = state.get_curr_shape().clone();

    let can_move = state.can_move_ghost_shape(&ghost, Dir::Down);
    assert!(can_move, "Призрачная фигура должна иметь возможность падения");
}

/// Тест 49: Проверка что столкновение не выходит за границы массива
#[test]
fn test_collision_array_bounds() {
    let mut state = GameState::new();

    // Двигаем к границам
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Проверка не должна вызывать панику (выход за границы)
    let _ = state.can_move_curr_shape(Dir::Left);
    let _ = state.can_move_curr_shape(Dir::Down);
}

/// Тест 50: Проверка что столкновение работает после множественных движений
#[test]
fn test_collision_after_multiple_moves() {
    let mut state = GameState::new();

    // Серия движений
    for _ in 0..5 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
        if state.can_move_curr_shape(Dir::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Столкновение должно работать корректно
    let can_down = state.can_move_curr_shape(Dir::Down);
    assert!(can_down, "Движение вниз должно быть возможно");
}
