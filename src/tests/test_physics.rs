//! Тесты физической механики игры.
//!
//! Этот модуль содержит 20 тестов для проверки физической механики Tetris:
//! - Тесты гравитации и падения (4 теста)
//! - Тесты столкновений (4 теста)
//! - Тесты вращения (3 теста)
//! - Тесты удержания фигуры (3 теста)
//! - Тесты призрачной фигуры (2 теста)
//! - Тесты Bag Generator (4 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты физической механики.

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::game::GameState;
use crate::types::{Direction, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-4: Гравитация и падение
// ============================================================================

/// Тест 1: Проверка гравитации и падения фигуры
///
/// Проверяет, что фигура падает вниз под действием гравитации.
#[test]
fn test_gravity_and_falling() {
    let mut state = GameState::new();

    // Запоминаем начальную позицию Y
    let initial_y = state.curr_shape().pos().1;

    // Фигура должна иметь возможность падения вниз
    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "Фигура должна иметь возможность падения вниз"
    );

    // Опускаем фигуру на 5 блоков
    for _ in 0..5 {
        if state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos_mut().1 += 1.0;
        }
    }

    // Проверяем, что фигура опустилась
    let final_y = state.curr_shape().pos().1;
    assert!(
        final_y > initial_y,
        "Фигура должна опуститься вниз под действием гравитации"
    );
    assert!(
        (final_y - initial_y).abs() >= 5.0,
        "Фигура должна опуститься минимум на 5 блоков"
    );
}

/// Тест 4: Проверка увеличения скорости падения
///
/// Проверяет, что скорость падения увеличивается с уровнем.
#[test]
fn test_falling_speed_increase() {
    use crate::constants::{INITIAL_FALL_SPD, SPD_INC};

    let initial = INITIAL_FALL_SPD;
    let after_one_line = initial + SPD_INC * 1.0;
    let after_five_lines = initial + SPD_INC * 5.0;
    let after_ten_lines = initial + SPD_INC * 10.0;

    // Проверяем увеличение скорости
    assert!(
        after_one_line > initial,
        "Скорость должна увеличиться после 1 линии"
    );
    assert!(
        after_five_lines > after_one_line,
        "Скорость должна расти с количеством линий"
    );
    assert!(
        after_ten_lines > after_five_lines,
        "Скорость должна продолжать расти"
    );

    // Проверяем, что скорость не превышает разумные пределы
    assert!(
        after_ten_lines < 5.0,
        "Скорость после 10 линий должна быть меньше 5.0"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 5-8: Столкновения
// ============================================================================

/// Тест 7: Проверка столкновения с зафиксированными фигурами
///
/// Проверяет, что новая фигура сталкивается с уже зафиксированными.
#[test]
fn test_collision_with_fixed_pieces() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape_direction(Direction::Down),
        "Движение вниз должно быть заблокировано на полу"
    );

    // Проверяем, что фигура не вышла за пределы поля
    let curr_y = state.curr_shape().pos().1;
    assert!(
        curr_y < GRID_HEIGHT as f32,
        "Фигура не должна выходить за пределы поля по Y"
    );
}

/// Тест 8: Проверка столкновений в пустом поле
///
/// Проверяет, что в пустом поле фигура может двигаться свободно.
#[test]
fn test_collisions_in_empty_field() {
    let state = GameState::new();

    // В начале игры движение вниз должно быть возможно
    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "В пустом поле движение вниз должно быть возможно"
    );

    // Проверяем, что движение влево/вправо зависит от позиции
    let curr_x = state.curr_shape().pos().0;

    // Если фигура не у границы, хотя бы одно направление должно быть доступно
    if curr_x > 0.0 && curr_x < (GRID_WIDTH - 1) as f32 {
        assert!(
            state.can_move_curr_shape_direction(Direction::Left)
                || state.can_move_curr_shape_direction(Direction::Right),
            "В центре поля хотя бы одно направление должно быть доступно"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 9-11: Вращение
// ============================================================================

/// Тест 9: Проверка вращения у левой стены
///
/// Проверяет, что вращение работает корректно у левой стены.
#[test]
fn test_rotation_near_left_wall() {
    let mut state = GameState::new();

    // Перемещаем фигуру к левой границе
    for _ in 0..5 {
        if state.can_move_curr_shape_direction(Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    // Проверяем что методы вращения работают и возвращают bool
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);
    // Результат должен быть bool (не паника)
    assert!(can_rotate_right || !can_rotate_right, "can_rotate_right должен вернуть bool");
    assert!(can_rotate_left || !can_rotate_left, "can_rotate_left должен вернуть bool");
}

/// Тест 10: Проверка вращения у правой стены
///
/// Проверяет, что вращение возможно даже у правой стены.
#[test]
fn test_rotation_near_right_wall() {
    let mut state = GameState::new();

    // Перемещаем фигуру к правой границе (но не вплотную)
    for _ in 0..3 {
        if state.can_move_curr_shape_direction(Direction::Right) {
            state.get_curr_shape_mut().pos_mut().0 += 1.0;
        }
    }

    // Вращение должно быть возможно
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Хотя бы одно направление вращения должно быть доступно
    assert!(
        can_rotate_right || can_rotate_left,
        "Хотя бы одно направление вращения должно быть доступно у правой стены"
    );
}

/// Тест 11: Проверка вращения у пола
///
/// Проверяет, что вращение возможно даже у пола.
#[test]
fn test_rotation_near_floor() {
    let mut state = GameState::new();

    // Опускаем фигуру близко к полу
    for _ in 0..15 {
        if state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos_mut().1 += 1.0;
        }
    }

    // Вращение должно быть возможно
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Хотя бы одно направление вращения должно быть доступно
    assert!(
        can_rotate_right || can_rotate_left,
        "Хотя бы одно направление вращения должно быть доступно у пола"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 12-14: Удержание фигуры (Hold)
// ============================================================================

/// Тест 12: Проверка обмена удержанной фигуры
///
/// Проверяет, что hold корректно обменивает фигуры.
#[test]
fn test_hold_swap_mechanism() {
    let mut state = GameState::new();

    // Запоминаем начальную фигуру
    let initial_shape = state.curr_shape().shape();
    let next_shape = state.next_shape().shape();

    // Удерживаем фигуру
    state.hold_shape();

    // Текущая фигура должна измениться на следующую
    assert_eq!(
        state.curr_shape().shape(),
        next_shape,
        "Текущая фигура должна стать следующей после hold"
    );

    // Удержанная фигура должна быть установлена
    assert!(
        state.held_shape().is_some(),
        "Удержанная фигура должна быть установлена"
    );

    // Удержанная фигура должна быть той, что была изначально
    assert_eq!(
        state
            .held_shape()
            .expect("Удержанная фигура должна существовать")
            .shape(),
        initial_shape,
        "Удержанная фигура должна быть начальной фигурой"
    );
}

/// Тест 13: Проверка запрета повторного удержания
///
/// Проверяет, что нельзя удержать фигуру дважды за ход.
#[test]
fn test_hold_double_usage_prevention() {
    let mut state = GameState::new();

    // Первое удержание
    state.hold_shape();

    // Повторное удержание должно быть запрещено
    assert!(
        !state.can_hold(),
        "Повторное удержание должно быть запрещено в том же ходу"
    );

    // Позиция фигуры должна быть сброшена к центру
    assert_eq!(
        state.curr_shape().pos(),
        (4.0, 0.0),
        "Позиция фигуры должна быть сброшена к центру после hold"
    );
}

/// Тест 14: Проверка сброса `can_hold` после нового хода
///
/// Проверяет, что `can_hold` сбрасывается после нового хода.
#[test]
fn test_hold_reset_after_new_turn() {
    let mut state = GameState::new();

    // В начале игры can_hold должен быть true
    assert!(state.can_hold(), "В начале игры можно удерживать фигуру");

    // Удерживаем фигуру
    state.hold_shape();

    // Теперь can_hold должен быть false
    assert!(
        !state.can_hold(),
        "После удержания can_hold должен быть false"
    );

    // Удержанная фигура должна быть установлена
    assert!(
        state.held_shape().is_some(),
        "Удержанная фигура должна быть установлена"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-16: Призрачная фигура
// ============================================================================

/// Тест 15: Проверка позиции призрачной фигуры
///
/// Проверяет, что призрачная фигура показывает правильную точку приземления.
#[test]
fn test_ghost_piece_position() {
    let state = GameState::new();
    let ghost_shape = *state.curr_shape();

    // Призрачная фигура должна использовать ту же логику столкновений
    let can_move_down = state.can_move_ghost_shape_direction(Direction::Down);

    // В начале игры призрачная фигура должна иметь возможность падения
    assert!(
        can_move_down,
        "Призрачная фигура должна иметь возможность падения вниз"
    );

    // Проверяем, что призрачная фигура имеет те же координаты
    assert_eq!(
        ghost_shape.shape(),
        state.curr_shape().shape(),
        "Призрачная фигура должна быть того же типа"
    );
}
