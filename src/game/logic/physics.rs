//! Модуль физики.
//!
//! # Ответственность
//! - Обработка падения фигур
//! - Гравитация
//! - Таймер приземления
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, константы

#![allow(dead_code)]

use crate::constants::MILLIS_PER_SECOND;
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
#[allow(clippy::cast_precision_loss)]
pub fn handle_falling(state: &mut GameState, delta_time_ms: u64) -> bool {
    if state.can_move_curr_shape_direction(Direction::Down) {
        let fall_speed = state.fall_speed();
        let curr_shape = state.get_curr_shape_mut();
        // Потеря точности допустима: delta_time_ms небольшое значение (обычно 16-100 мс)
        curr_shape.pos_mut().1 += fall_speed * (delta_time_ms as f32 / MILLIS_PER_SECOND);
        false
    } else if state.land_timer() > 0.0 {
        let land_timer = state.land_timer();
        // Потеря точности допустима: MILLIS_PER_SECOND точно представляется в f64
        let new_timer = land_timer - delta_time_ms as f64 / f64::from(MILLIS_PER_SECOND);
        // H6: защита от отрицательного таймера
        // Ошибка игнорируется: LAND_TIME_DELAY_S — константное валидное значение,
        // а new_timer.max(0.0) всегда >= 0, поэтому set_land_timer не может вернуть ошибку
        state.set_land_timer(new_timer.max(0.0)).ok();
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
        let initial_y = state.curr_shape().pos().1;

        // Фигура должна падать
        let result = handle_falling(&mut state, 100);

        assert!(!result, "Фигура должна ещё падать");
        assert!(
            state.curr_shape().pos().1 >= initial_y,
            "Y координата должна увеличиться или остаться"
        );
    }

    /// Тест: land_timer не становится отрицательным
    #[test]
    fn test_handle_falling_land_timer_non_negative() {
        let mut state = GameState::new();
        state.set_land_timer(0.01).ok();

        // Большой delta_time — если фигура не падает, land_timer уменьшится
        let _ = handle_falling(&mut state, 1000);

        // land_timer всегда >= 0 благодаря .max(0.0) в handle_falling
        assert!(
            state.land_timer() >= 0.0,
            "Land timer не должен быть отрицательным"
        );
    }
}
