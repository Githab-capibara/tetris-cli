//! Модуль проверки столкновений.
//!
//! Проверка возможности движения и вращения фигуры.
//!
//! ## Архитектурные заметки
//! ## Исправление #3 (DRY - Don't Repeat Yourself)
//! Выделена общая функция `check_block_collision()` для устранения дублирования кода
//! между проверкой движения и проверкой вращения.

use crate::game::GameState;
use crate::io::GRID_WIDTH;
use crate::types::Direction;

/// Проверить столкновение одного блока с границами или другими блоками.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `check_x` - координата X блока для проверки
/// * `check_y` - координата Y блока для проверки
/// * `ignore_above_field` - игнорировать блоки выше поля (Y < 0)
///
/// # Возвращает
/// `true` если блок не сталкивается ни с чем
///
/// # Исправление #3 (DRY)
/// Общая функция для устранения дублирования между:
/// - `check_collision_direction()` - проверка движения
/// - `check_rotation_collision()` - проверка вращения
///
/// # Примечания
/// - Проверка границ X: 0 <= check_x < GRID_WIDTH
/// - Проверка заполненных ячеек: cell == -1 означает пусто
/// - Игнорирование Y < 0 полезно для вращения (блоки могут быть выше поля)
#[must_use]
fn check_block_collision(
    state: &GameState,
    check_x: i16,
    check_y: i16,
    ignore_above_field: bool,
) -> bool {
    // Проверка левой границы
    if check_x < 0 {
        return false;
    }

    // Проверка правой границы
    if check_x >= GRID_WIDTH as i16 {
        return false;
    }

    // Игнорирование блоков выше поля (для вращения)
    if ignore_above_field && check_y < 0 {
        return true; // Считаем что блока нет (нет коллизии)
    }

    // Проверка нижней границы (только для движения вниз)
    // Для вращения блоки выше поля допустимы
    if check_y < 0 {
        return true; // Считаем что блока нет (нет коллизии)
    }

    // Проверка наличия блока в сетке
    if state
        .blocks
        .get(check_y as usize)
        .and_then(|row| row.get(check_x as usize))
        .is_none_or(|&cell| cell != -1)
    {
        return false; // Столкновение с заполненной ячейкой
    }

    true // Нет столкновений
}

/// Проверить возможность движения фигуры в заданном направлении.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `coords` - координаты блоков фигуры
/// * `pos` - позиция фигуры (x, y)
/// * `dir` - направление движения
///
/// # Возвращает
/// `true` если движение возможно
///
/// # Исправление #8
/// Используется `.get()` с ранним выходом вместо множественных проверок границ.
/// # Исправление бага
/// Проверка check_y < 0 должна выполняться только для Direction::Down.
/// Блоки выше поля (отрицательный Y) не должны блокировать движение влево/вправо.
fn check_collision_direction(
    state: &GameState,
    coords: &[(i16, i16)],
    pos: (f32, f32),
    dir: Direction,
) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let mut check_x = coord_x + shape_block_x;
        let mut check_y = coord_y + shape_block_y;

        match dir {
            Direction::Left => check_x -= 1,
            Direction::Right => check_x += 1,
            Direction::Down => check_y += 1,
        }

        // Исправление #3 (DRY): используем общую функцию check_block_collision
        // Для движения вниз не игнорируем блоки выше поля (check_y < 0 блокирует движение)
        let ignore_above_field = false;
        if !check_block_collision(state, check_x, check_y, ignore_above_field) {
            return false;
        }
    }
    true
}

/// Проверить возможность движения текущей фигуры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление движения
///
/// # Возвращает
/// `true` если движение возможно
#[must_use]
pub fn can_move_curr_shape_direction(state: &GameState, dir: Direction) -> bool {
    check_collision_direction(state, &state.curr_shape.coords, state.curr_shape.pos, dir)
}

/// Проверить возможность вращения фигуры (без смещения).
///
/// # Аргументы
/// * `state` - состояние игры
/// * `coords` - координаты блоков повёрнутой фигуры
/// * `pos` - позиция фигуры
///
/// # Возвращает
/// `true` если вращение возможно
///
/// # Исправление #8
/// Используется `.get()` с ранним выходом вместо множественных проверок границ.
/// # Исправление бага
/// Блоки выше поля (отрицательный Y) допустимы при вращении.
pub fn check_rotation_collision(state: &GameState, coords: &[(i16, i16)], pos: (f32, f32)) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let check_x = coord_x + shape_block_x;
        let check_y = coord_y + shape_block_y;

        // Исправление #3 (DRY): используем общую функцию check_block_collision
        // Для вращения игнорируем блоки выше поля (check_y < 0 допустимо)
        let ignore_above_field = true;
        if !check_block_collision(state, check_x, check_y, ignore_above_field) {
            return false;
        }
    }
    true
}

/// Проверить возможность вращения текущей фигуры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение возможно
pub fn can_rotate_curr_shape(state: &GameState, dir: crate::types::RotationDirection) -> bool {
    // Сначала проверяем прямое вращение
    let mut temp_shape = state.curr_shape;
    temp_shape.rotate(dir);

    if check_rotation_collision(state, &temp_shape.coords, temp_shape.pos) {
        return true;
    }

    // Проверяем wall kick
    try_rotation_with_kicks(state, dir).is_some()
}

/// Попытаться вратить фигуру со смещением (wall kick).
///
/// Возвращает `Some((offset_x, offset_y))` если вращение успешно с указанным смещением,
/// или `None` если вращение невозможно ни с одним смещением.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// `Some((i32, i32))` с успешным смещением или `None` если вращение невозможно
fn try_rotation_with_kicks(
    state: &GameState,
    dir: crate::types::RotationDirection,
) -> Option<(i32, i32)> {
    for &(offset_x, offset_y) in &crate::game::logic::WALL_KICK_OFFSETS {
        let mut kicked_shape = state.curr_shape;
        kicked_shape.pos.0 += offset_x as f32;
        kicked_shape.pos.1 += offset_y as f32;
        kicked_shape.rotate(dir);

        if check_rotation_collision(state, &kicked_shape.coords, kicked_shape.pos) {
            return Some((offset_x, offset_y));
        }
    }
    None
}

#[cfg(test)]
mod collision_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_can_move_down_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Down));
    }

    #[test]
    fn test_can_move_left_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Left));
    }

    #[test]
    fn test_can_move_right_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Right));
    }
}
