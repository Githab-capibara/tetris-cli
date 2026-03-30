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
//!
//! ## Архитектурные заметки (A7: DIP)
//! Функция `handle_input()` использует трейт `InputReader` вместо конкретного типа `KeyReader`
//! для соблюдения Dependency Inversion Principle.

use crate::game::state::{GameState, UpdateEndState};
use crate::io::KEY_BACKSPACE;
use crate::io_traits::InputReader;
use crate::types::{Direction, RotationDirection};

/// Обработать пользовательский ввод.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `inp` - читатель нажатий клавиш (реализует трейт InputReader)
///
/// # Возвращает
/// - `Some(UpdateEndState::Quit)` - выход в меню
/// - `Some(UpdateEndState::Pause)` - пауза
/// - `None` - продолжить обработку
///
/// # Архитектурные заметки (A7: DIP)
/// Использует трейт `InputReader` вместо конкретного типа `KeyReader`
/// для соблюдения Dependency Inversion Principle.
///
/// # Исправление #14 (MEDIUM SEVERITY)
/// Добавлено логирование ошибок через `eprintln!()` для критических ошибок ввода.
/// Это позволяет отслеживать проблемы с вводом во время отладки.
pub fn handle_input<T: crate::io_traits::InputReader>(
    state: &mut GameState,
    inp: &mut T,
) -> Option<UpdateEndState> {
    let key = inp.get_key();

    // Сброс флага Hard Drop
    state.set_is_hard_dropping(false);

    match key {
        Some(KEY_BACKSPACE) => {
            eprintln!("[INFO] Получена клавиша выхода (Backspace)");
            return Some(UpdateEndState::Quit);
        }
        Some(b'p') => {
            eprintln!("[INFO] Получена клавиша паузы (P)");
            return Some(UpdateEndState::Pause);
        }
        Some(b'a') => handle_movement_input(state, Direction::Left),
        Some(b'd') => handle_movement_input(state, Direction::Right),
        Some(b'q') => handle_rotation_input(state, RotationDirection::CounterClockwise),
        Some(b'e') => handle_rotation_input(state, RotationDirection::Clockwise),
        Some(b'w') => super::super::scoring::handle_hard_drop(state),
        Some(b's') => super::super::scoring::handle_soft_drop(state),
        Some(b'c' | b'C') => super::super::scoring::handle_hold(state),
        Some(other_key) => {
            // Логирование неизвестной клавиши для отладки
            eprintln!(
                "[DEBUG] Получена неизвестная клавиша: {:?} (0x{:02X})",
                char::from_u32(other_key as u32).unwrap_or('?'),
                other_key
            );
        }
        None => {
            // Клавиша не была нажата или произошла ошибка чтения
            // Не логируем чтобы не спамить в консоль каждый кадр
        }
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
            Direction::Left => {
                let curr_shape = state.get_curr_shape_mut();
                curr_shape.pos.0 -= 1.0;
            }
            Direction::Right => {
                let curr_shape = state.get_curr_shape_mut();
                curr_shape.pos.0 += 1.0;
            }
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
///
/// ## Исправление #4 (HIGH)
/// Функция обновлена для использования wall_kick модуля.
fn handle_rotation_input(state: &mut GameState, dir: RotationDirection) {
    super::wall_kick::rotate_with_wall_kick(state, dir);
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_handle_movement_left() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos.0;

        handle_movement_input(&mut state, Direction::Left);

        assert!(
            state.curr_shape().pos.0 <= initial_x,
            "Фигура должна сдвинуться влево или остаться на месте"
        );
    }

    #[test]
    fn test_handle_movement_right() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos.0;

        handle_movement_input(&mut state, Direction::Right);

        assert!(
            state.curr_shape().pos.0 >= initial_x,
            "Фигура должна сдвинуться вправо или остаться на месте"
        );
    }
}
