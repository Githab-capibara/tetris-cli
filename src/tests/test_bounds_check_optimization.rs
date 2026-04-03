//! Тесты оптимизации проверок границ.
//!
//! Проверяют корректность `check_collision_direction` и производительность.

use crate::game::GameState;
use crate::types::Direction;
use serial_test::serial;

/// Тест 1: Проверка корректности `check_collision_direction`
///
/// Проверяем, что функция корректно определяет коллизии.
#[test]
fn test_check_collision_direction_correctness() {
    let state = GameState::new();

    // В начале игры движение вниз должно быть возможно
    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "Движение вниз должно быть возможно в начале игры"
    );

    // Движение влево должно быть возможно
    assert!(
        state.can_move_curr_shape_direction(Direction::Left),
        "Движение влево должно быть возможно в центре поля"
    );

    // Движение вправо должно быть возможно
    assert!(
        state.can_move_curr_shape_direction(Direction::Right),
        "Движение вправо должно быть возможно в центре поля"
    );
}

/// Тест 2: Проверка границ поля
///
/// Проверяем, что коллизии с границами определяются корректно.
#[test]
fn test_bounds_check_boundaries() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Перемещаем фигуру к левой границе
    while can_move_curr_shape_direction(&state, Direction::Left) {
        state.get_curr_shape_mut().pos().0 -= 1.0;
    }

    // Движение влево должно быть заблокировано
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Left),
        "Движение влево у левой границы должно быть заблокировано"
    );

    // Сбрасываем состояние
    let mut state = GameState::new();

    // Перемещаем фигуру к правой границе
    while can_move_curr_shape_direction(&state, Direction::Right) {
        state.get_curr_shape_mut().pos().0 += 1.0;
    }

    // Движение вправо должно быть заблокировано
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Right),
        "Движение вправо у правой границы должно быть заблокировано"
    );

    // Проверяем, что фигура в пределах поля
    assert!(
        state.curr_shape().pos().0 < GRID_WIDTH as f32,
        "Фигура должна быть в пределах поля по X"
    );
}

/// Тест 3: Проверка коллизий с другими фигурами
///
/// Проверяем, что коллизии с зафиксированными фигурами определяются.
/// Используем #[serial] для предотвращения конфликтов при параллельном выполнении.
#[test]
#[serial]
fn test_collision_with_other_pieces() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::io::GRID_HEIGHT;

    let mut state = GameState::new();

    // Находим безопасную позицию для установки блока
    // Используем фиксированную позицию вместо динамического вычисления
    let test_x = 5;
    let test_y = 10;

    // Проверяем, что позиция валидна
    assert!(
        test_x < 10 && test_y < GRID_HEIGHT,
        "Тестовая позиция должна быть валидна"
    );

    // Сохраняем текущую позицию фигуры
    let original_pos = state.curr_shape().pos();

    // Перемещаем фигуру на тестовую позицию
    state
        .get_curr_shape_mut()
        .set_pos((test_x as f32, test_y as f32));

    // Устанавливаем блок прямо под фигурой (на 1 ячейку ниже)
    state.get_blocks_mut()[test_y + 1][test_x] = 1;

    // Движение вниз должно быть заблокировано
    let can_move = can_move_curr_shape_direction(&state, Direction::Down);

    // Восстанавливаем оригинальную позицию
    let () = state.get_curr_shape_mut().set_pos(original_pos);

    assert!(
        !can_move,
        "Движение вниз должно быть заблокировано блоком на позиции ({}, {})",
        test_x,
        test_y + 1
    );
}

/// Тест 5: Проверка использования as cast вместо `try_from`
///
/// Проверяем, что оптимизация с as cast работает корректно.
#[test]
fn test_as_cast_optimization() {
    use crate::io::{GRID_HEIGHT, GRID_WIDTH};

    // Проверяем, что константы корректно конвертируются в i16
    let grid_width_i16 = GRID_WIDTH as i16;
    let grid_height_i16 = GRID_HEIGHT as i16;

    assert_eq!(grid_width_i16, 10, "GRID_WIDTH должен быть 10");
    assert_eq!(grid_height_i16, 20, "GRID_HEIGHT должен быть 20");

    // Проверяем, что конвертация безопасна
    assert!(grid_width_i16 > 0, "GRID_WIDTH должен быть положительным");
    assert!(grid_height_i16 > 0, "GRID_HEIGHT должен быть положительным");

    // Проверяем, что значения в пределах i16
    assert!(
        grid_width_i16 < i16::MAX,
        "GRID_WIDTH должен быть < i16::MAX"
    );
    assert!(
        grid_height_i16 < i16::MAX,
        "GRID_HEIGHT должен быть < i16::MAX"
    );
}

/// Тест 6: Проверка отрицательных координат
///
/// Проверяем, что отрицательные координаты обрабатываются корректно.
#[test]
fn test_negative_coords_handling() {
    use crate::game::logic::can_move_curr_shape_direction;

    let mut state = GameState::new();

    // Устанавливаем позицию на левой границе (X=0)
    state.get_curr_shape_mut().pos().0 = 0.0;

    // Движение влево должно быть заблокировано на границе
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Left),
        "Движение влево с X=0 должно быть заблокировано"
    );

    // Движение вправо должно быть возможно
    assert!(
        can_move_curr_shape_direction(&state, Direction::Right),
        "Движение вправо с X=0 должно быть возможно"
    );
}

/// Тест 7: Проверка всех направлений
///
/// Проверяем, что все направления работают корректно.
#[test]
fn test_all_directions() {
    let state = GameState::new();

    // Проверяем все три направления
    let directions = [Direction::Left, Direction::Right, Direction::Down];

    for &dir in &directions {
        let can_move = state.can_move_curr_shape_direction(dir);
        assert!(
            can_move,
            "Направление {dir:?} должно быть доступно в центре поля"
        );
    }
}
