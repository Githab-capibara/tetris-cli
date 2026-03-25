//! Модуль игровой логики.
//!
//! # Ответственность
//! - Обработка ввода пользователя
//! - Обновление состояния фигур
//! - Проверка столкновений
//! - Вращение с wall kick
//! - Физика падения фигур
//!
//! # Зависимости
//! - [`state.rs`](super::state): `GameState`, `UpdateEndState`
//! - [`scoring.rs`](super::scoring): функции начисления очков
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`
//! - [`types.rs`](crate::types): `Direction`, `RotationDirection`
//!
//! ## Основные функции:
//! - [`update()`] - основной цикл обновления состояния
//! - [`handle_input()`] - обработка ввода пользователя
//! - [`can_move_curr_shape_direction()`] - проверка возможности движения
//! - [`can_rotate_curr_shape()`] - проверка возможности вращения
//! - [`rotate_with_wall_kick()`] - вращение с обходом стен
//!
//! ## Архитектурные заметки
//! TODO (#архитектура): Выделить обработку ввода в отдельный модуль `input.rs`
//! для уменьшения связанности с терминальным вводом.

use super::state::{GameState, UpdateEndState, MILLIS_PER_SECOND};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::types::{Direction, RotationDirection};

/// Таблица смещений для wall kick (Super Rotation System - упрощённая).
/// Используется при вращении фигур рядом со стенами.
pub const WALL_KICK_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),  // Влево на 1
    (1, 0),   // Вправо на 1
    (-2, 0),  // Влево на 2
    (2, 0),   // Вправо на 2
    (0, -1),  // Вверх на 1 (для случаев у пола)
    (-1, -1), // Влево и вверх
    (1, -1),  // Вправо и вверх
    (0, 1),   // Вниз на 1 (для случаев у потолка)
];

/// Обработать пользовательский ввод.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `inp` - читатель нажатий клавиш
///
/// # Возвращает
/// - `Some(UpdateEndState::Quit)` - выход в меню
/// - `Some(UpdateEndState::Pause)` - пауза
/// - `None` - продолжить обработку
pub fn handle_input(
    state: &mut GameState,
    inp: &mut crate::io::KeyReader,
) -> Option<UpdateEndState> {
    use crate::io::KEY_BACKSPACE;

    let key = inp.get_key();

    // Сброс флага Hard Drop
    state.is_hard_dropping = false;

    match key {
        Some(KEY_BACKSPACE) => return Some(UpdateEndState::Quit),
        Some(b'p') => return Some(UpdateEndState::Pause),
        Some(b'a') => handle_movement_input(state, Direction::Left),
        Some(b'd') => handle_movement_input(state, Direction::Right),
        Some(b'q') => handle_rotation_input(state, RotationDirection::CounterClockwise),
        Some(b'e') => handle_rotation_input(state, RotationDirection::Clockwise),
        Some(b'w') => super::scoring::handle_hard_drop(state),
        Some(b's') => super::scoring::handle_soft_drop(state),
        Some(b'c' | b'C') => super::scoring::handle_hold(state),
        Some(_) | None => {}
    }

    None
}

/// Обработка движения влево/вправо.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление движения
fn handle_movement_input(state: &mut GameState, dir: Direction) {
    if state.can_move_curr_shape_direction(dir) {
        match dir {
            Direction::Left => state.curr_shape.pos.0 -= 1.0,
            Direction::Right => state.curr_shape.pos.0 += 1.0,
            Direction::Down => {
                // Direction::Down обрабатывается отдельно в handle_soft_drop/handle_hard_drop
                // Эта ветка намеренно пустая — движение вниз управляется гравитацией
            }
        }
    }
}

/// Обработка вращения фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление вращения
fn handle_rotation_input(state: &mut GameState, dir: RotationDirection) {
    rotate_with_wall_kick(state, dir);
}

/// Обработать падение фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `delta_time_ms` - время, прошедшее с последнего кадра (мс)
///
/// # Возвращает
/// - `true` - фигура приземлилась, требуется обработка
/// - `false` - фигура ещё падает
pub fn handle_falling(state: &mut GameState, delta_time_ms: u64) -> bool {
    if state.can_move_curr_shape_direction(Direction::Down) {
        state.curr_shape.pos.1 += state.fall_spd * (delta_time_ms as f32 / MILLIS_PER_SECOND);
        false
    } else if state.land_timer > 0.0 {
        state.land_timer -= delta_time_ms as f64 / f64::from(MILLIS_PER_SECOND);
        false
    } else {
        true
    }
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
fn check_collision_direction(
    state: &GameState,
    coords: &[(i16, i16)],
    pos: (f32, f32),
    dir: Direction,
) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    // Оптимизация: используем as вместо try_from() для const значений
    // GRID_WIDTH и GRID_HEIGHT известны на этапе компиляции и гарантированно помещаются в i16
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_width_i16 = GRID_WIDTH as i16;
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_height_i16 = GRID_HEIGHT as i16;

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let mut check_x = coord_x + shape_block_x;
        let mut check_y = coord_y + shape_block_y;

        match dir {
            Direction::Left => check_x -= 1,
            Direction::Right => check_x += 1,
            Direction::Down => check_y += 1,
        }

        if check_x < 0 || check_x >= grid_width_i16 || check_y >= grid_height_i16 {
            return false;
        }

        if check_y >= 0 && state.blocks[check_y as usize][check_x as usize] != -1 {
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

/// Проверить возможность вращения фигуры.
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
    for &(offset_x, offset_y) in &WALL_KICK_OFFSETS {
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

/// Попытаться вратить фигуру со смещением (wall kick).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение успешно
pub fn rotate_with_wall_kick(state: &mut GameState, dir: crate::types::RotationDirection) -> bool {
    // Проверяем прямое вращение
    if can_rotate_curr_shape(state, dir) {
        state.curr_shape.rotate(dir);
        return true;
    }

    // Используем общую функцию для wall kick
    if let Some((offset_x, offset_y)) = try_rotation_with_kicks(state, dir) {
        state.curr_shape.pos.0 += offset_x as f32;
        state.curr_shape.pos.1 += offset_y as f32;
        state.curr_shape.rotate(dir);
        return true;
    }

    false
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
fn check_rotation_collision(state: &GameState, coords: &[(i16, i16)], pos: (f32, f32)) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    // Оптимизация: используем as вместо try_from() для const значений
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_width_i16 = GRID_WIDTH as i16;
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_height_i16 = GRID_HEIGHT as i16;

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let check_x = coord_x + shape_block_x;
        let check_y = coord_y + shape_block_y;

        if check_x < 0 || check_x >= grid_width_i16 || check_y >= grid_height_i16 {
            return false;
        }

        if check_y >= 0 && state.blocks[check_y as usize][check_x as usize] != -1 {
            return false;
        }
    }
    true
}

/// Сохранить текущую фигуру в сетке после приземления.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn save_tetromino(state: &mut GameState) {
    let (shape_x, shape_y) = state.curr_shape.pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    // Оптимизация: используем as вместо try_from() для const значений
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_height_i16 = GRID_HEIGHT as i16;
    #[allow(clippy::cast_possible_wrap)]
    // GRID_WIDTH и GRID_HEIGHT — константы (10 и 20), безопасно для cast
    let grid_width_i16 = GRID_WIDTH as i16;

    for coord in state.curr_shape.coords {
        let (coord_x, coord_y) = coord;
        let x = coord_x + shape_block_x;
        let y = coord_y + shape_block_y;

        if y >= 0 && y < grid_height_i16 && x >= 0 && x < grid_width_i16 {
            state.blocks[y as usize][x as usize] = state.curr_shape.fg as i8;
        }
    }
}

/// Обновить состояние игры за один кадр.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `inp` - читатель нажатий клавиш
/// * `delta_time_ms` - время, прошедшее с последнего кадра (мс)
///
/// # Возвращает
/// Состояние завершения обновления
pub fn update(
    state: &mut GameState,
    inp: &mut crate::io::KeyReader,
    delta_time_ms: u64,
) -> UpdateEndState {
    // Обработка ввода
    if let Some(update_state) = handle_input(state, inp) {
        return update_state;
    }

    // Обработка падения
    if !handle_falling(state, delta_time_ms) {
        return UpdateEndState::Continue;
    }

    // Обработка приземления
    if let Some(update_state) = super::scoring::handle_landing(state) {
        return update_state;
    }

    UpdateEndState::Continue
}
