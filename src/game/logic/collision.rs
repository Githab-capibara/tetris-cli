//! Модуль проверки столкновений.
//!
//! Проверка возможности движения и вращения фигуры.
//!
//! ## Архитектурные заметки
//! ## Исправление #3 (DRY - Don't Repeat Yourself)
//! Выделена общая функция `check_block_collision()` для устранения дублирования кода
//! между проверкой движения и проверкой вращения.
//!
//! ## Исправление C2
//! Используется Range для проверки границ вместо множественных сравнений.

use crate::game::GameState;
use crate::io::GRID_WIDTH;
use crate::types::Direction;

/// Допустимый диапазон координат X для блоков на поле.
/// Используется для проверки границ в проверке коллизий.
const VALID_X_RANGE: std::ops::Range<i16> = 0..GRID_WIDTH as i16;

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
/// # Исправление C2
/// Используется Range::contains для проверки границ вместо множественных сравнений.
///
/// # Примечания
/// - Проверка границ X: `0 <= check_x < GRID_WIDTH`
/// - Проверка заполненных ячеек: `cell == -1` означает пусто
/// - Игнорирование `Y < 0` полезно для вращения (блоки могут быть выше поля)
#[must_use]
fn check_block_collision(
    state: &GameState,
    check_x: i16,
    check_y: i16,
    ignore_above_field: bool,
) -> bool {
    // Исправление C2: используем Range::contains для проверки границ X
    if !VALID_X_RANGE.contains(&check_x) {
        return false;
    }

    // Проверка нижней границы (блоки выше поля считаются пустыми)
    // Блоки выше поля (Y < 0) не должны считаться коллизией
    // Это важно для движения влево/вправо, когда фигура появляется на поле
    if check_y < 0 {
        return true; // Нет коллизии выше поля
    }

    // Проверка наличия блока в сетке
    if state
        .get_blocks()
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
/// Проверка `check_y < 0` должна выполняться только для `Direction::Down`.
/// Блоки выше поля (отрицательный Y) не должны блокировать движение влево/вправо.
///
/// # Исправление M22 (MEDIUM)
/// Используется `.any()` с ранним выходом для оптимизации проверки коллизий.
/// Вместо цикла с return false используется итератор с .any() для раннего прерывания.
fn check_collision_direction(
    state: &GameState,
    coords: &[(i16, i16)],
    pos: (f32, f32),
    dir: Direction,
) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    // Исправление M22: используем any() для раннего выхода при обнаружении коллизии
    // any() возвращает true если хотя бы один блок имеет коллизию
    // !any() возвращает true если все блоки свободны (нет коллизий)
    !coords.iter().any(|&(coord_x, coord_y)| {
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
        // Инвертируем результат: check_block_collision возвращает true если нет коллизии
        !check_block_collision(state, check_x, check_y, ignore_above_field)
    })
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
    let curr_shape = state.curr_shape();
    check_collision_direction(state, &curr_shape.coords, curr_shape.pos, dir)
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
/// `true` если вращение возможно (прямое или с wall kick)
///
/// ## Исправление #4 (HIGH)
/// Функция делегирует логику wall kick в модуль `wall_kick.rs`.
/// Прямая проверка вращения без использования wall kick.
pub fn can_rotate_curr_shape(state: &GameState, dir: crate::types::RotationDirection) -> bool {
    // Сначала проверяем прямое вращение (без wall kick)
    let mut temp_shape = *state.curr_shape();
    temp_shape.rotate(dir);

    if check_rotation_collision(state, &temp_shape.coords, temp_shape.pos) {
        return true;
    }

    // Делегируем проверку wall kick в специализированный модуль
    // Исправление #4: устранено дублирование try_rotation_with_kicks
    super::wall_kick::try_wall_kick_offsets(state, dir).is_some()
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
