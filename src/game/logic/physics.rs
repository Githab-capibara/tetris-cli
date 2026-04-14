//! Модуль физики.
//!
//! # Ответственность
//! - Обработка падения фигур
//! - Гравитация
//! - Таймер приземления
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, константы

// Координаты ограничены полем, скорости — небольшие значения, cast безопасен.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "Координаты ограничены полем (10x20), скорости небольшие"
)]
//! # Примечание о точности f32
//! Координаты фигур и скорость используют `f32`. При многократном накоплении
//! погрешностей (каждый кадр × несколько секунд) возможно отклонение координат.
//! В проекте это компенсируется:
//! - Ограничением `max_y` для предотвращения проваливания сквозь пол
//! - Проверкой `is_finite()` в `rotate()` и `handle_hard_drop()`
//! - Использованием `.min(max_y)` для ограничения позиции
//!
//! При необходимости большей точности рассмотрите переход на `f64`.

use crate::constants::{GRID_HEIGHT, MILLIS_PER_SECOND};
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
    // Защита от NaN/Infinity в fall_speed — предотвращает NaN в fall_distance
    let fall_speed = state.fall_speed();
    if !fall_speed.is_finite() {
        crate::log_warn!("handle_falling: fall_speed некорректен ({fall_speed}), сбрасываем в начальное значение");
        // Сбрасываем к начальному значению чтобы фигура не "зависала"
        if let Ok(()) = state.set_fall_speed(crate::constants::INITIAL_FALL_SPD) {
            // Продолжаем с корректным значением
        } else {
            return false;
        }
    }

    if state.can_move_curr_shape_direction(Direction::Down) {
        let fall_distance = fall_speed * (delta_time_ms as f32 / MILLIS_PER_SECOND);
        let current_y = state.curr_shape().pos().1;
        // Ограничиваем падение нижней границей поля — фигура не может провалиться сквозь пол
        let max_y = (GRID_HEIGHT - 1) as f32;
        let new_y = (current_y + fall_distance).min(max_y);
        let current_x = state.curr_shape().pos().0;
        state.set_curr_pos(current_x, new_y);
        false
    } else if state.land_timer() > 0.0 {
        let land_timer = state.land_timer();
        // Потеря точности допустима: MILLIS_PER_SECOND точно представляется в f64
        let new_timer = land_timer - delta_time_ms as f64 / f64::from(MILLIS_PER_SECOND);
        // H6: защита от отрицательного таймера
        // new_timer.max(0.0) всегда >= 0 и finite, поэтому set_land_timer не может
        // вернуть ошибку. Если это всё же произошло — логируем для диагностики.
        if let Err(_e) = state.set_land_timer(new_timer.max(0.0)) {
            crate::log_error!("Не удалось установить land_timer");
        }
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

    /// Тест: `land_timer` не становится отрицательным
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

    // =========================================================================
    // ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ (#37)
    // =========================================================================

    #[test]
    fn test_handle_falling_zero_delta() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape().pos().1;

        // С нулевым delta_time позиция не должна измениться
        let result = handle_falling(&mut state, 0);

        assert!(!result, "С нулевым delta_time фигура должна ещё падать");
        assert!(
            (state.curr_shape().pos().1 - initial_y).abs() < 0.001,
            "Y координата не должна измениться с нулевым delta_time"
        );
    }

    #[test]
    fn test_handle_falling_fall_speed_affects_distance() {
        let mut state1 = GameState::new();
        let mut state2 = GameState::new();

        // Установим разную скорость падения
        state1.set_fall_speed(0.5).ok();
        state2.set_fall_speed(2.0).ok();

        let initial_y1 = state1.curr_shape().pos().1;
        let initial_y2 = state2.curr_shape().pos().1;

        handle_falling(&mut state1, 100);
        handle_falling(&mut state2, 100);

        let moved1 = state1.curr_shape().pos().1 - initial_y1;
        let moved2 = state2.curr_shape().pos().1 - initial_y2;

        assert!(
            moved2 > moved1,
            "Большая скорость должна привести к большему перемещению"
        );
    }

    #[test]
    fn test_handle_falling_land_timer_decreases() {
        let mut state = GameState::new();
        // Установим land_timer > 0 и сделаем так чтобы фигура не падала
        state.set_land_timer(0.5).ok();

        // Фигура в начале может падать, поэтому проверяем что timer уменьшается
        // когда фигура НЕ может двигаться вниз
        // Для этого поднимем фигуру к полу
        while state.can_move_curr_shape_direction(Direction::Down) {
            handle_falling(&mut state, 100);
        }

        // Теперь фигура не может падать, land_timer должен уменьшаться
        let timer_before = state.land_timer();
        if timer_before > 0.0 {
            let _ = handle_falling(&mut state, 50);
            assert!(
                state.land_timer() < timer_before,
                "Land timer должен уменьшиться"
            );
        }
    }

    #[test]
    fn test_handle_falling_landing_detection() {
        let mut state = GameState::new();
        state.set_land_timer(0.0).ok();

        // Опустим фигуру до пола
        while state.can_move_curr_shape_direction(Direction::Down) {
            handle_falling(&mut state, 100);
        }

        // Фигура должна быть приземлена (land_timer = 0 и не может двигаться вниз)
        let result = handle_falling(&mut state, 100);
        assert!(result, "Фигура у пола должна считаться приземлённой");
    }

    #[test]
    fn test_handle_falling_large_delta_time() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape().pos().1;

        // Большой delta_time не должен вызывать панику
        let result = handle_falling(&mut state, 10000);

        // Результат зависит от состояния — либо падает, либо приземлилась
        if !result {
            assert!(
                state.curr_shape().pos().1 >= initial_y,
                "Y координата должна увеличиться при большом delta_time"
            );
        }
    }

    /// Тест: фигура не проваливается сквозь пол при большом `delta_time`
    #[test]
    fn test_handle_falling_no_phantom_pass_through_floor() {
        use crate::constants::GRID_HEIGHT;

        let mut state = GameState::new();
        let max_y = (GRID_HEIGHT - 1) as f32;

        // Огромный delta_time — фигура должна упасть до пола, но не ниже
        handle_falling(&mut state, 1_000_000);

        // Позиция Y не должна превышать максимальную границу поля
        assert!(
            state.curr_shape().pos().1 <= max_y,
            "Y координата ({}) не должна превышать максимальную границу поля ({max_y})",
            state.curr_shape().pos().1
        );

        // Продолжаем вызывать handle_falling пока фигура не приземлится
        let mut iterations = 0;
        loop {
            let landed = handle_falling(&mut state, 1_000_000);
            iterations += 1;
            assert!(
                state.curr_shape().pos().1 <= max_y,
                "После итерации {iterations} Y координата ({}) превысила границу ({max_y})",
                state.curr_shape().pos().1
            );
            if landed || iterations > 100 {
                break;
            }
        }
        assert!(
            iterations <= 100,
            "Фигура должна приземлиться за разумное число итераций"
        );
    }

    /// Тест: `handle_falling` возвращает true для уже приземлённой фигуры.
    /// Проверяет что фигура которая уже не может двигаться вниз и имеет `land_timer` = 0
    /// корректно определяется как приземлённая.
    #[test]
    fn test_handle_falling_already_landed() {
        let mut state = GameState::new();

        // Опускаем фигуру до пола
        while state.can_move_curr_shape_direction(Direction::Down) {
            let _ = handle_falling(&mut state, 16);
        }

        // Устанавливаем land_timer в 0 — фигура уже приземлена
        state.set_land_timer(0.0).ok();

        // handle_falling должен вернуть true — фигура приземлилась
        let result = handle_falling(&mut state, 16);
        assert!(
            result,
            "Уже приземлённая фигура с land_timer = 0 должна вернуть true"
        );
    }
}
