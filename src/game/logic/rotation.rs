//! Модуль вращения фигур.
//!
//! # Ответственность
//! - Базовое вращение фигур (без wall kick)
//! - Проверка возможности вращения
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`
//! - [`collision.rs`](super::collision): `check_rotation_collision`
//! - [`wall_kick.rs`](super::wall_kick): логика wall kick
//!
//! ## Исправление #4 (HIGH)
//! Логика wall kick перемещена в `wall_kick.rs` для устранения дублирования.
//! Этот модуль содержит только базовое вращение без смещений.

use crate::game::GameState;

/// Проверить возможность вращения текущей фигуры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение возможно (прямое или с wall kick)
///
/// # Делегирование
/// Функция делегирует проверку в `wall_kick::can_rotate_with_wall_kick`
/// для централизации логики wall kick.
///
/// ## Исправление #13
/// Функция предназначена для будущего использования в API.
#[must_use]
#[allow(dead_code)]
pub fn can_rotate_curr_shape(state: &GameState, dir: crate::types::RotationDirection) -> bool {
    super::wall_kick::can_rotate_with_wall_kick(state, dir)
}
