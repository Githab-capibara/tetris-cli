//! Модуль игровой логики.
//!
//! # Ответственность
//! - Обработка ввода пользователя
//! - Обновление состояния фигур
//! - Проверка столкновений
//! - Вращение с wall kick
//! - Физика падения фигур
//!
//! # Подмодули
//! - [`input`] - обработка ввода пользователя
//! - [`physics`] - физика падения и гравитация
//! - [`collision`] - проверка столкновений
//! - [`rotation`] - вращение с wall kick
//! - [`update`] - координирует обновление состояния
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`, `UpdateEndState`
//! - [`scoring.rs`](crate::game::scoring): функции начисления очков
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`
//! - [`types.rs`](crate::types): `Direction`, `RotationDirection`
//!
//! ## Архитектурные заметки
//! ## Разделение ответственности (Problem 2.3, 2.9)
//! Этот модуль отвечает ТОЛЬКО за игровую логику.
//!
//! TODO (#архитектура): Выделить обработку ввода в отдельный модуль `input.rs`
//! для уменьшения связанности с терминальным вводом.

// Подмодули
pub mod collision;
pub mod input;
pub mod physics;
pub mod rotation;
pub mod update;

// Публичные экспорты из collision
pub use collision::{can_move_curr_shape_direction, can_rotate_curr_shape};

// Публичные экспорты из rotation
pub use rotation::{rotate_with_wall_kick, WALL_KICK_OFFSETS};

// Публичные экспорты из update
pub use update::{save_tetromino, update};
