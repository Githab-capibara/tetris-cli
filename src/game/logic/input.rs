//! Модуль обработки ввода.
//!
//! # Ответственность
//! - Обработка пользовательского ввода
//! - Движение фигур (влево, вправо)
//! - Вращение фигур
//! - Special действия (Hard Drop, Soft Drop, Hold, Pause, Quit)
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, `UpdateEndState`
//! - [`scoring.rs`](super::super::scoring): функции начисления очков
//! - [`types.rs`](crate::types): `Direction`, `RotationDirection`

use crate::game::state::{GameState, UpdateEndState};
use crate::io::KEY_BACKSPACE;
use crate::types::{Direction, RotationDirection};

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
        Some(b'w') => super::super::scoring::handle_hard_drop(state),
        Some(b's') => super::super::scoring::handle_soft_drop(state),
        Some(b'c' | b'C') => super::super::scoring::handle_hold(state),
        Some(_) | None => {}
    }

    None
}

/// Обработка движения влево/вправо.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление движения
///
/// # Исправление #4
/// Удалена ветка Direction::Down как dead code — движение вниз
/// обрабатывается отдельно в handle_soft_drop/handle_hard_drop.
fn handle_movement_input(state: &mut GameState, dir: Direction) {
    if state.can_move_curr_shape_direction(dir) {
        match dir {
            Direction::Left => state.curr_shape.pos.0 -= 1.0,
            Direction::Right => state.curr_shape.pos.0 += 1.0,
            // Direction::Down обрабатывается отдельно в handle_soft_drop/handle_hard_drop
            Direction::Down => {}
        }
    }
}

/// Обработка вращения фигуры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление вращения
fn handle_rotation_input(state: &mut GameState, dir: RotationDirection) {
    super::rotation::rotate_with_wall_kick(state, dir);
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_handle_movement_left() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape.pos.0;

        handle_movement_input(&mut state, Direction::Left);

        assert!(
            state.curr_shape.pos.0 <= initial_x,
            "Фигура должна сдвинуться влево или остаться на месте"
        );
    }

    #[test]
    fn test_handle_movement_right() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape.pos.0;

        handle_movement_input(&mut state, Direction::Right);

        assert!(
            state.curr_shape.pos.0 >= initial_x,
            "Фигура должна сдвинуться вправо или остаться на месте"
        );
    }
}
