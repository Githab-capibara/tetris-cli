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
//! - [`update`] - вращение фигур и координирование состояния
//! - [`wall_kick`] - вращение с wall kick (централизованная логика)
//!
//! Координирование обновления состояния осуществляется через функцию `update()` из модуля `update`.
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
//! Обработка ввода уже выделена в отдельный модуль [`input`].
//!
//! ## Исправление #4 (HIGH)
//! Логика wall kick выделена в отдельный модуль `wall_kick.rs`
//! для устранения дублирования кода между collision.rs и rotation.rs.

// Подмодули
pub mod collision;
pub mod input;
pub mod physics;
pub mod update;
pub mod wall_kick;

// Публичные экспорты из collision
pub use collision::{can_move_curr_shape_direction, can_rotate_curr_shape};

// Публичные экспорты из wall_kick (централизованная логика wall kick)
// Исправление #13: удалены неиспользуемые импорты
pub use wall_kick::rotate_with_wall_kick;

// Публичные экспорты из update
pub use update::{save_tetromino, update as update_state};
