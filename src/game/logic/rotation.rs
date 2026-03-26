//! Модуль вращения фигур.
//!
//! # Ответственность
//! - Вращение фигур с wall kick
//! - Super Rotation System (упрощённая)
//! - Проверка возможности вращения
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`
//! - [`collision.rs`](super::collision): `check_rotation_collision`

use crate::game::GameState;

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
    if super::collision::can_rotate_curr_shape(state, dir) {
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

        if super::collision::check_rotation_collision(state, &kicked_shape.coords, kicked_shape.pos)
        {
            return Some((offset_x, offset_y));
        }
    }
    None
}

#[cfg(test)]
mod rotation_tests {
    use super::*;
    use crate::game::GameState;
    use crate::types::RotationDirection;

    #[test]
    fn test_rotate_with_wall_kick_basic() {
        let mut state = GameState::new();
        let initial_coords = state.curr_shape.coords;

        let result = rotate_with_wall_kick(&mut state, RotationDirection::Clockwise);

        assert!(result, "Вращение должно быть успешным");
        assert_ne!(
            state.curr_shape.coords, initial_coords,
            "Координаты должны измениться после вращения"
        );
    }
}
