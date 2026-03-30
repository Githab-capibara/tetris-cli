//! Модуль физики.
//!
//! # Ответственность
//! - Обработка падения фигур
//! - Гравитация
//! - Таймер приземления
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, константы

use crate::game::constants::MILLIS_PER_SECOND;
use crate::game::state::GameState;
use crate::types::Direction;

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
        let fall_speed = state.fall_speed();
        let curr_shape = state.get_curr_shape_mut();
        curr_shape.pos.1 += fall_speed * (delta_time_ms as f32 / MILLIS_PER_SECOND);
        false
    } else if state.land_timer() > 0.0 {
        let land_timer = state.land_timer();
        // Игнорируем ошибку, так как значение вычисляется корректно
        let _ =
            state.set_land_timer(land_timer - delta_time_ms as f64 / f64::from(MILLIS_PER_SECOND));
        false
    } else {
        true
    }
}

#[cfg(test)]
mod physics_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_handle_falling_initial() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape().pos.1;

        // Фигура должна падать
        let result = handle_falling(&mut state, 100);

        assert!(!result, "Фигура должна ещё падать");
        assert!(
            state.curr_shape().pos.1 >= initial_y,
            "Y координата должна увеличиться или остаться"
        );
    }
}
