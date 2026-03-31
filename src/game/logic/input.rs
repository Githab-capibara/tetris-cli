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
//! - [`types.rs`](crate::types): `Direction`, `RotationDirection`, [`GameAction`](crate::game::types::GameAction)
//!
//! ## Архитектурные заметки (A7: DIP)
//! Функция `handle_input()` использует трейт `InputReader` вместо конкретного типа `KeyReader`
//! для соблюдения Dependency Inversion Principle.
//!
//! ## Исправление 7: GameAction enum
//! Введена абстракция GameAction для отделения конкретных клавиш от игровых действий.
//! Это позволяет:
//! - Изменять конфигурацию управления без изменения логики ввода
//! - Легко добавлять новые действия
//! - Уменьшить связанность между controls.rs и input.rs
//!
//! ## H5: Разделение ввода и логики
//! Модуль разделён на три уровня:
//! 1. `parse_input()` - чистый парсер клавиш в GameAction (без состояния)
//! 2. `execute_action()` - исполнитель действий (изменяет GameState)
//! 3. `handle_input()` - комбинация парсера и исполнителя для удобства

use crate::game::state::{GameState, UpdateEndState};
use crate::game::types::GameAction;
use crate::io::KEY_BACKSPACE;
use crate::types::{Direction, RotationDirection};

// ============================================================================
// H5: ЧИСТЫЙ ПАРСЕР (БЕЗ ЗАВИСИМОСТИ ОТ СОСТОЯНИЯ)
// ============================================================================

/// Распознать игровое действие из кода клавиши.
///
/// # Аргументы
/// * `key_code` - код нажатой клавиши
///
/// # Возвращает
/// - `Some(GameAction)` если клавиша соответствует действию
/// - `None` если клавиша не распознана
///
/// # Архитектурные заметки (H5)
/// Чистая функция без побочных эффектов. Не зависит от GameState.
/// Для изменения конфигурации управления нужно изменить только эту функцию.
#[must_use]
pub fn parse_input(key_code: u8) -> Option<GameAction> {
    match key_code {
        b'a' => Some(GameAction::MoveLeft),
        b'd' => Some(GameAction::MoveRight),
        b'q' => Some(GameAction::RotateLeft),
        b'e' => Some(GameAction::RotateRight),
        b'w' => Some(GameAction::HardDrop),
        b's' => Some(GameAction::SoftDrop),
        b'c' | b'C' => Some(GameAction::Hold),
        _ => None,
    }
}

// ============================================================================
// H5: ИСПОЛНИТЕЛЬ ДЕЙСТВИЙ (ИЗМЕНЯЕТ СОСТОЯНИЕ)
// ============================================================================

/// Выполнить игровое действие над состоянием игры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `action` - игровое действие
///
/// # Возвращает
/// - `Some(UpdateEndState::Quit)` - выход в меню
/// - `Some(UpdateEndState::Pause)` - пауза
/// - `None` - продолжить обработку
///
/// # Архитектурные заметки (H5)
/// Эта функция обрабатывает абстрактные действия вместо конкретных клавиш.
/// Не содержит логики парсинга ввода.
pub fn execute_action(state: &mut GameState, action: GameAction) -> Option<UpdateEndState> {
    match action {
        GameAction::MoveLeft => {
            handle_movement_input(state, Direction::Left);
            None
        }
        GameAction::MoveRight => {
            handle_movement_input(state, Direction::Right);
            None
        }
        GameAction::SoftDrop => {
            super::super::scoring::handle_soft_drop(state);
            None
        }
        GameAction::HardDrop => {
            super::super::scoring::handle_hard_drop(state);
            None
        }
        GameAction::RotateLeft => {
            handle_rotation_input(state, RotationDirection::CounterClockwise);
            None
        }
        GameAction::RotateRight => {
            handle_rotation_input(state, RotationDirection::Clockwise);
            None
        }
        GameAction::Hold => {
            super::super::scoring::handle_hold(state);
            None
        }
        GameAction::Pause | GameAction::Quit => None,
    }
}

// ============================================================================
// ОСНОВНАЯ ФУНКЦИЯ ОБРАБОТКИ ВВОДА (КОМБИНАЦИЯ ПАРСЕРА И ИСПОЛНИТЕЛЯ)
// ============================================================================

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
///
/// # Исправление 7: GameAction enum
/// Использует GameAction для абстракции ввода. Конкретные клавиши маппятся в
/// GameAction через функцию `parse_input()`.
pub fn handle_input<T: crate::io_traits::InputReader>(
    state: &mut GameState,
    inp: &mut T,
) -> Option<UpdateEndState> {
    let key = inp.get_key();

    // Сброс флага Hard Drop
    state.set_is_hard_dropping(false);

    // Обработка клавиши
    match key {
        Ok(Some(KEY_BACKSPACE)) => {
            eprintln!("[INFO] Получена клавиша выхода (Backspace)");
            Some(UpdateEndState::Quit)
        }
        Ok(Some(b'p')) => {
            eprintln!("[INFO] Получена клавиша паузы (P)");
            Some(UpdateEndState::Pause)
        }
        Ok(Some(key_code)) => {
            // Парсинг клавиши в действие
            if let Some(action) = parse_input(key_code) {
                return execute_action(state, action);
            }
            // Неизвестная клавиша
            eprintln!(
                "[DEBUG] Получена неизвестная клавиша: {:?} (0x{:02X})",
                char::from_u32(key_code as u32).unwrap_or('?'),
                key_code
            );
            None
        }
        Ok(None) => {
            // Клавиша не была нажата
            None
        }
        Err(e) => {
            // Ошибка чтения ввода
            eprintln!("[ERROR] Ошибка чтения ввода: {}", e);
            None
        }
    }
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
                curr_shape.pos_mut().0 -= 1.0;
            }
            Direction::Right => {
                let curr_shape = state.get_curr_shape_mut();
                curr_shape.pos_mut().0 += 1.0;
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
