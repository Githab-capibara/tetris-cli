//! Тесты оптимизации проверок границ.
//!
//! Проверяют корректность check_collision_direction и производительность.

use crate::game::GameState;
use crate::types::Direction;

/// Тест 1: Проверка корректности check_collision_direction
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
        state.curr_shape.pos.0 -= 1.0;
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
        state.curr_shape.pos.0 += 1.0;
    }

    // Движение вправо должно быть заблокировано
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Right),
        "Движение вправо у правой границы должно быть заблокировано"
    );

    // Проверяем, что фигура в пределах поля
    assert!(
        state.curr_shape.pos.0 < GRID_WIDTH as f32,
        "Фигура должна быть в пределах поля по X"
    );
}

/// Тест 3: Проверка коллизий с другими фигурами
///
/// Проверяем, что коллизии с зафиксированными фигурами определяются.
#[test]
fn test_collision_with_other_pieces() {
    use crate::game::logic::can_move_curr_shape_direction;

    let mut state = GameState::new();

    // Устанавливаем блок под текущей фигурой
    let x = state.curr_shape.pos.0 as usize;
    let y = (state.curr_shape.pos.1 + 2.0) as usize;

    if x < 10 && y < 20 {
        state.blocks[y][x] = 0; // Устанавливаем блок

        // Движение вниз должно быть заблокировано
        assert!(
            !can_move_curr_shape_direction(&state, Direction::Down),
            "Движение вниз должно быть заблокировано блоком"
        );
    }
}

/// Тест 4: Проверка производительности check_collision_direction
///
/// Бенчмарк: проверка коллизий должна быть быстрой.
#[test]
fn test_check_collision_performance() {
    use std::time::Instant;

    let state = GameState::new();
    let iterations = 100_000;

    let start = Instant::now();

    for _ in 0..iterations {
        let _ = state.can_move_curr_shape_direction(Direction::Down);
        let _ = state.can_move_curr_shape_direction(Direction::Left);
        let _ = state.can_move_curr_shape_direction(Direction::Right);
    }

    let elapsed = start.elapsed();

    // 100000 итераций × 3 направления должны выполняться < 200ms
    // Увеличенный порог для стабильности в CI/CD и нагруженных системах
    assert!(
        elapsed.as_millis() < 200,
        "Проверка коллизий {iterations} итераций должна выполняться < 200ms (прошло {:?})",
        elapsed
    );
}

/// Тест 5: Проверка использования as cast вместо try_from
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
    state.curr_shape.pos.0 = 0.0;

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
