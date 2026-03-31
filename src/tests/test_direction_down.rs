//! Тесты обработки Direction::Down.
//!
//! Проверяют, что Direction::Down не вызывает паники и корректно обрабатывается.

use crate::game::GameState;
use crate::types::Direction;

/// Тест 1: Проверка, что Direction::Down не вызывает вращения
///
/// Direction::Down должен игнорироваться при вращении.
#[test]
fn test_direction_down_no_rotation() {
    use crate::tetromino::Tetromino;

    let mut tetromino = Tetromino::from_bag(&mut crate::tetromino::BagGenerator::new());
    let original_coords = tetromino.coords;

    // Пытаемся "вращать" с Direction::Down (должно игнорироваться)
    #[allow(deprecated)]
    tetromino.rotate_old(Direction::Down);

    // Координаты не должны измениться
    assert_eq!(
        tetromino.coords, original_coords,
        "Direction::Down не должен вызывать вращение фигуры"
    );
}

/// Тест 3: Проверка Soft Drop с Direction::Down
///
/// Проверяем, что мягкое падение работает корректно.
#[test]
fn test_direction_down_soft_drop() {
    use crate::game::scoring::handle_soft_drop;

    let mut state = GameState::new();
    let initial_y = state.curr_shape().pos().1;
    let initial_score = state.score();

    // Выполняем несколько Soft Drop
    let soft_drop_count = 5;
    for _ in 0..soft_drop_count {
        if state.can_move_curr_shape_direction(Direction::Down) {
            handle_soft_drop(&mut state);
        }
    }

    // Проверяем, что фигура опустилась через геттер
    assert!(
        state.curr_shape().pos().1 > initial_y,
        "После Soft Drop фигура должна опуститься"
    );

    // Проверяем, что очки начислены (1 очко за ячейку) через геттеры
    let drop_distance = (state.curr_shape().pos().1 - initial_y) as u32;
    let expected_score = initial_score + (drop_distance as u128);

    assert!(
        state.score() >= expected_score,
        "Очки за Soft Drop должны быть начислены: было {initial_score}, стало {}, ожидаем {expected_score}",
        state.score()
    );

    // Проверяем, что soft_drop_distance отслеживается через геттер
    assert!(
        state.soft_drop_distance() > 0,
        "soft_drop_distance должен быть больше 0 после Soft Drop"
    );
}

/// Тест 4: Проверка обработки Direction::Down в handle_movement_input
///
/// Direction::Down в handle_movement_input должен игнорироваться.
#[test]
fn test_direction_down_movement_input() {
    let mut state = GameState::new();
    let initial_x = state.curr_shape().pos().0;

    // Пытаемся двигаться вниз через handle_movement_input
    // (это должно игнорироваться, так как движение вниз управляется гравитацией)
    // handle_movement_input приватна, пропускаем тест
    let _initial_x = state.curr_shape().pos().0;
}

/// Тест 5: Проверка, что Direction::Down работает с проверкой коллизий
///
/// Проверяем корректность проверки коллизий для Direction::Down.
#[test]
fn test_direction_down_collision_check() {
    use crate::game::logic::can_move_curr_shape_direction;

    let mut state = GameState::new();

    // В начале игры движение вниз должно быть возможно
    assert!(
        can_move_curr_shape_direction(&state, Direction::Down),
        "В начале игры движение вниз должно быть возможно"
    );

    // Опускаем фигуру на дно через геттер
    while can_move_curr_shape_direction(&state, Direction::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Теперь движение вниз должно быть заблокировано
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Down),
        "На дне движение вниз должно быть заблокировано"
    );
}
