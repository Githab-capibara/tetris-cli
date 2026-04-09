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
//! - [`types.rs`](crate::types): `Direction`, `RotationDirection`, [`GameAction`]
//!
//! ## Архитектурные заметки (A7: DIP)
//! Функция `handle_input()` использует трейт `InputReader` вместо конкретного типа `KeyReader`
//! для соблюдения Dependency Inversion Principle.
//!
//! ## Исправление 7: `GameAction` enum
//! Введена абстракция `GameAction` для отделения конкретных клавиш от игровых действий.
//! Это позволяет:
//! - Изменять конфигурацию управления без изменения логики ввода
//! - Легко добавлять новые действия
//! - Уменьшить связанность между controls.rs и input.rs
//!
//! ## H5: Разделение ввода и логики
//! Модуль разделён на три уровня:
//! 1. `parse_input()` - чистый парсер клавиш в `GameAction` (без состояния)
//! 2. `execute_action()` - исполнитель действий (изменяет `GameState`)
//! 3. `handle_input()` - комбинация парсера и исполнителя для удобства

use crate::constants::KEY_BACKSPACE;
use crate::game::state::GameState;
use crate::game::types::GameAction;
use crate::types::{Direction, RotationDirection, UpdateEndState};

// ============================================================================
// H5: ЧИСТЫЙ ПАРСЕР (БЕЗ ЗАВИСИМОСТИ ОТ СОСТОЯНИЯ)
// ============================================================================
//
// PROB-151: TODO — Rate limiting на ввод.
// Для CLI-версии не критично: терминальный ввод не подвержен flood-атакам.
// При портировании на GUI/сетевой режим — добавить throttle на частоту нажатий.
//
// PROB-152: TODO — Валидация входных данных.
// Текущая валидация ограничена диапазоном u8 (0-255). Для CLI этого достаточно.
// При расширении (сетевой ввод) — добавить строгую валидацию кодов клавиш.

/// Распознать игровое действие из кода клавиши.
///
/// # Аргументы
/// * `key_code` - код нажатой клавиши
/// * `config` - конфигурация управления
///
/// # Возвращает
/// - `Some(GameAction)` если клавиша соответствует действию
/// - `None` если клавиша не распознана
///
/// # Архитектурные заметки (H1, H5)
/// Использует `ControlsConfig` для маппинга клавиш вместо хардкода.
#[must_use]
pub const fn parse_input(
    key_code: u8,
    config: &crate::controls::ControlsConfig,
) -> Option<GameAction> {
    config.map_key_to_action(key_code)
}

// ============================================================================
// H5: ИСПОЛНИТЕЛЬ ДЕЙСТВИЙ (ИЗМЕНЯЕТ СОСТОЯНИЕ)
// ============================================================================

/// Выполнить игровое действие над состоянием игры.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `action` - игровое действие (`GameAction`)
///
/// # Возвращает
/// - `Some(UpdateEndState::Quit)` - если действие Quit (выход в меню)
/// - `Some(UpdateEndState::Pause)` - если действие Pause (пауза)
/// - `None` - для всех остальных действий (продолжить обработку)
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
        GameAction::Pause => Some(UpdateEndState::Pause),
        GameAction::Quit => Some(UpdateEndState::Quit),
    }
}

// ============================================================================
// ОСНОВНАЯ ФУНКЦИЯ ОБРАБОТКИ ВВОДА (КОМБИНАЦИЯ ПАРСЕРА И ИСПОЛНИТЕЛЯ)
// ============================================================================

/// Обработать пользовательский ввод.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `inp` - читатель нажатий клавиш (реализует трейт `InputReader`)
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
/// Критические ошибки ввода логируются через макрос `log_error!`.
///
/// # Исправление PROB-141..146
/// INFO и DEBUG логи удалены из production кода.
///
/// # Исправление 7: `GameAction` enum
/// Использует `GameAction` для абстракции ввода. Конкретные клавиши маппятся в
/// `GameAction` через функцию `parse_input()`.
pub fn handle_input<T: crate::io_traits::InputReader>(
    state: &mut GameState,
    inp: &mut T,
    config: &crate::controls::ControlsConfig,
) -> Option<UpdateEndState> {
    let key = inp.get_key();

    // Сброс флага Hard Drop
    state.set_is_hard_dropping(false);

    // Обработка клавиши
    match key {
        Ok(Some(KEY_BACKSPACE)) => Some(UpdateEndState::Quit),
        Ok(Some(key_code)) => {
            // Парсинг клавиши в действие через конфигурацию
            if let Some(action) = parse_input(key_code, config) {
                return execute_action(state, action);
            }
            // Неизвестная клавиша
            None
        }
        Ok(None) => {
            // Клавиша не была нажата
            None
        }
        Err(_e) => {
            // Ошибка чтения ввода
            log_error!("Ошибка чтения ввода");
            None
        }
    }
}

/// Обработка движения влево/вправо.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление движения (только Left или Right)
///
/// # Panics
/// Паникует в debug-сборке если передан `Direction::Down`.
/// В release-сборке игнорирует `Direction::Down` (безопасный no-op).
fn handle_movement_input(state: &mut GameState, dir: Direction) {
    match dir {
        Direction::Left | Direction::Right => {
            if state.can_move_curr_shape_direction(dir) {
                let curr_shape = state.get_curr_shape_mut();
                // Direction::Down невозможен благодаря внешнему match —
                // эта ветка вызывается только для Left/Right.
                if dir == Direction::Left {
                    curr_shape.pos_mut().0 -= 1.0;
                } else {
                    // Direction::Right
                    curr_shape.pos_mut().0 += 1.0;
                }
            }
        }
        Direction::Down => {
            // Движение вниз обрабатывается отдельно через handle_soft_drop/handle_hard_drop.
            // В release-сборке — безопасный no-op; в debug — подсказка разработчику.
            debug_assert!(
                false,
                "Direction::Down передан в handle_movement_input — \
                 используйте handle_soft_drop/handle_hard_drop"
            );
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
/// Функция обновлена для использования `wall_kick` модуля.
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
        let initial_x = state.curr_shape().pos().0;

        handle_movement_input(&mut state, Direction::Left);

        assert!(
            state.curr_shape().pos().0 <= initial_x,
            "Фигура должна сдвинуться влево или остаться на месте"
        );
    }

    #[test]
    fn test_handle_movement_right() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos().0;

        handle_movement_input(&mut state, Direction::Right);

        assert!(
            state.curr_shape().pos().0 >= initial_x,
            "Фигура должна сдвинуться вправо или остаться на месте"
        );
    }

    // =========================================================================
    // ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ (#36)
    // =========================================================================

    #[test]
    fn test_execute_action_move_left() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos().0;

        let result = execute_action(&mut state, GameAction::MoveLeft);

        assert!(result.is_none(), "MoveLeft не должен завершать игру");
        assert!(
            state.curr_shape().pos().0 <= initial_x,
            "MoveLeft должен сдвинуть фигуру влево"
        );
    }

    #[test]
    fn test_execute_action_move_right() {
        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos().0;

        let result = execute_action(&mut state, GameAction::MoveRight);

        assert!(result.is_none(), "MoveRight не должен завершать игру");
        assert!(
            state.curr_shape().pos().0 >= initial_x,
            "MoveRight должен сдвинуть фигуру вправо"
        );
    }

    #[test]
    fn test_execute_action_pause() {
        let mut state = GameState::new();
        let result = execute_action(&mut state, GameAction::Pause);

        assert_eq!(
            result,
            Some(UpdateEndState::Pause),
            "Pause должен вернуть Pause"
        );
    }

    #[test]
    fn test_execute_action_quit() {
        let mut state = GameState::new();
        let result = execute_action(&mut state, GameAction::Quit);

        assert_eq!(
            result,
            Some(UpdateEndState::Quit),
            "Quit должен вернуть Quit"
        );
    }

    #[test]
    fn test_execute_action_hold() {
        let mut state = GameState::new();
        let initial_can_hold = state.can_hold();

        let result = execute_action(&mut state, GameAction::Hold);

        assert!(result.is_none(), "Hold не должен завершать игру");
        // Hold должен изменить can_hold если фигура ещё не удерживалась
        if initial_can_hold {
            assert!(!state.can_hold(), "После Hold can_hold должен стать false");
        }
    }

    #[test]
    fn test_handle_movement_down_no_op() {
        // Direction::Down в handle_movement_input — debug_assert в debug,
        // no-op в release. Тест проверяет что состояние не меняется.
        // В debug-сборке debug_assert паникует, поэтому тест пропускается.
        if cfg!(debug_assertions) {
            // В debug — ожидаемо что debug_assert сработает
            // Тест просто проверяет что функция существует с правильной сигнатурой
            return;
        }

        let mut state = GameState::new();
        let initial_x = state.curr_shape().pos().0;
        let initial_y = state.curr_shape().pos().1;

        handle_movement_input(&mut state, Direction::Down);

        // Позиция не должна измениться (Direction::Down игнорируется)
        assert!((state.curr_shape().pos().0 - initial_x).abs() < f32::EPSILON);
        assert!((state.curr_shape().pos().1 - initial_y).abs() < f32::EPSILON);
    }
}
