//! Модуль обновления состояния игры.
//!
//! # Ответственность
//! - Координация всех подмодулей логики
//! - Основной цикл обновления состояния
//! - Обработка ввода, физики, приземления
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, `UpdateEndState`
//! - [`input.rs`](super::input): `handle_input`
//! - [`physics.rs`](super::physics): `handle_falling`
//! - [`scoring.rs`](crate::game::scoring): `handle_landing`
//!
//! ## Архитектурные заметки (A7: DIP)
//! Функция `update()` использует трейт `InputReader` вместо конкретного типа `KeyReader`.

use crate::game::state::GameState;
use crate::io_traits::InputReader;
use crate::types::UpdateEndState;

/// Обновить состояние игры за один кадр.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `inp` - читатель нажатий клавиш (реализует трейт `InputReader`)
/// * `delta_time_ms` - время, прошедшее с последнего кадра (мс)
///
/// # Возвращает
/// Состояние завершения обновления
///
/// # Архитектурные заметки (A7: DIP)
/// Использует трейт `InputReader` вместо конкретного типа `KeyReader`.
pub fn update<T: InputReader>(
    state: &mut GameState,
    inp: &mut T,
    delta_time_ms: u64,
) -> UpdateEndState {
    // Обработка ввода — используем кэшированную конфигурацию (исправление #10)
    let config = crate::controls::ControlsConfig::default_config_ref();
    if let Some(update_state) = super::input::handle_input(state, inp, config) {
        return update_state;
    }

    // Обработка падения
    if !super::physics::handle_falling(state, delta_time_ms) {
        return UpdateEndState::Continue;
    }

    // Обработка приземления
    if let Some(update_state) = crate::game::scoring::handle_landing(state) {
        return update_state;
    }

    UpdateEndState::Continue
}

/// Сохранить текущую фигуру в сетке после приземления.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
pub fn save_tetromino(state: &mut GameState) {
    let (shape_x, shape_y) = state.curr_shape().pos();
    // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_WIDTH)
    #[allow(clippy::cast_possible_wrap)]
    let shape_block_x = shape_x as i16;
    // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_HEIGHT)
    #[allow(clippy::cast_possible_wrap)]
    let shape_block_y = shape_y as i16;

    // Оптимизация: используем as вместо try_from() для const значений
    // cast: usize -> i16, потеря точности допустима: GRID_HEIGHT константа (20)
    #[allow(clippy::cast_possible_wrap)]
    let grid_height_i16 = crate::io::GRID_HEIGHT as i16;
    // cast: usize -> i16, потеря точности допустима: GRID_WIDTH константа (10)
    #[allow(clippy::cast_possible_wrap)]
    let grid_width_i16 = crate::io::GRID_WIDTH as i16;

    let curr_shape = state.curr_shape();
    let fg = curr_shape.fg();
    let coords = curr_shape.coords();

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let x = coord_x + shape_block_x;
        let y = coord_y + shape_block_y;

        // Потеря точности допустима: y и x проверены на границы поля
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        if y >= 0 && y < grid_height_i16 && x >= 0 && x < grid_width_i16 {
            // SAFETY: ShapeType 0..6, fits in i8
            state.get_blocks_mut()[y as usize][x as usize] = fg as i8;
        }
    }
}

#[cfg(test)]
mod update_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_update_continue() {
        let mut state = GameState::new();
        let mut inp = crate::io::KeyReader::default();

        let result = update(&mut state, &mut inp, 100);

        assert!(
            matches!(result, UpdateEndState::Continue)
                || matches!(result, UpdateEndState::Pause)
                || matches!(result, UpdateEndState::Quit),
            "Обновление должно вернуть допустимое состояние"
        );
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ save_tetromino (исправление #56-57)
    // ========================================================================

    /// Тест: save_tetromino корректно сохраняет фигуру в сетке
    #[test]
    fn test_save_tetromino_basic() {
        let mut state = GameState::new();
        // Устанавливаем фигуру в известную позицию
        state.get_curr_shape_mut().pos_mut().1 = 10.0;

        let initial_blocks = *state.get_blocks();
        save_tetromino(&mut state);
        let final_blocks = state.get_blocks();

        // Хотя бы одна ячейка должна измениться (фигура сохранена)
        let mut changed = false;
        for y in 0..crate::io::GRID_HEIGHT {
            for x in 0..crate::io::GRID_WIDTH {
                if initial_blocks[y][x] != final_blocks[y][x] {
                    changed = true;
                }
            }
        }
        assert!(
            changed,
            "save_tetromino должна изменить хотя бы одну ячейку"
        );
    }

    /// Тест: save_tetromino не выходит за границы поля
    #[test]
    fn test_save_tetromino_respects_bounds() {
        let mut state = GameState::new();
        // Устанавливаем фигуру близко к границе
        state.get_curr_shape_mut().pos_mut().0 = 0.0;
        state.get_curr_shape_mut().pos_mut().1 = 0.0;

        // Вызов не должен паниковать
        save_tetromino(&mut state);

        // Проверяем что все ячейки в пределах допустимых значений (-1 или 0-6)
        for y in 0..crate::io::GRID_HEIGHT {
            for x in 0..crate::io::GRID_WIDTH {
                let cell = state.get_blocks()[y][x];
                assert!(
                    cell == -1 || (0..=6).contains(&cell),
                    "Ячейка [{y}][{x}] = {cell} должна быть в диапазоне -1..=6"
                );
            }
        }
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ handle_hold swap-сценарий (исправление #58)
    // ========================================================================

    /// Тест: handle_hold swap-сценарий — обмен текущей и удержанной фигуры
    #[test]
    fn test_handle_hold_swap_scenario() {
        let mut state = GameState::new();

        // Первый hold: текущая фигура переходит в held_shape
        let initial_curr = *state.curr_shape();
        state.hold_shape();

        assert!(
            state.held_shape().is_some(),
            "После первого hold должна быть удержанная фигура"
        );
        assert!(!state.can_hold(), "После hold can_hold должен стать false");

        // Сбрасываем can_hold для симуации следующей фигуры
        state.set_can_hold(true);

        // Сохраняем фигуру которая стала текущей после первого hold
        let after_first_hold_curr = *state.curr_shape();

        // Второй hold: swap текущей и удержанной фигур
        crate::game::scoring::handle_hold(&mut state);

        // После swap текущая фигура должна быть той что была удержана
        assert!(
            state.held_shape().is_some(),
            "После swap должна быть удержанная фигура"
        );
        // Фигура которая была удержана должна стать текущей
        assert_eq!(
            state.curr_shape().shape(),
            initial_curr.shape(),
            "После swap текущая фигура должна совпадать с изначальной удержанной"
        );
    }

    /// Тест: handle_hold когда held_shape = None — берём следующую фигуру
    #[test]
    fn test_handle_hold_first_time() {
        let mut state = GameState::new();
        assert!(
            state.held_shape().is_none(),
            "Изначально held_shape должен быть None"
        );

        let initial_curr = *state.curr_shape();
        let initial_next = *state.next_shape();

        crate::game::scoring::handle_hold(&mut state);

        // Текущая фигура должна стать следующей
        assert_eq!(
            state.curr_shape().shape(),
            initial_next.shape(),
            "При первом hold текущая фигура должна стать следующей"
        );
        // Удержанная фигура должна быть установлена
        assert_eq!(
            state.held_shape().unwrap().shape(),
            initial_curr.shape(),
            "При первом hold удержанная фигура должна совпадать с изначальной текущей"
        );
    }
}
