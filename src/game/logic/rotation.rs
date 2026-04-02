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
//!
//! ## Исправление аудита 2026-03-31
//! Удалена дублирующая функция `can_rotate_curr_shape` так как она полностью
//! дублирует функциональность из `collision::can_rotate_curr_shape`.
//! Используйте функцию из модуля `collision` для проверки вращения.

#![allow(dead_code)]

use crate::game::GameState;

/// Базовое вращение фигуры без проверки коллизий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление вращения
///
/// # Panics
/// Никогда не паникует. Внутренний `rotate()` обрабатывает переполнения.
///
/// # Примечания
/// Эта функция выполняет только вращение координат фигуры.
/// Для проверки возможности вращения используйте
/// `crate::game::logic::collision::can_rotate_curr_shape`.
pub fn rotate_shape(state: &mut GameState, dir: crate::types::RotationDirection) {
    state.get_curr_shape_mut().rotate(dir);
}
