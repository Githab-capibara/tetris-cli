//! Модуль публичного API.
//!
//! Переэкспорт наиболее часто используемых типов для удобного импорта.
//!
//! ## Зачем нужен exports.rs
//! Этот модуль служит единой точкой входа для публичного API библиотеки.
//! Вместо импорта из разных модулей (`crate::game::state::GameState`, `crate::io::Canvas`),
//! пользователи могут импортировать всё из `crate::exports::*`.
//! Это обеспечивает стабильный публичный API — при внутренних перемещениях модулей
//! re-export в exports.rs обновляется, а публичный API остаётся неизменным.
//!
//! # Использование
//!
//! ```ignore
//! use tetris_cli::exports::{GameState, Canvas, KeyReader, Leaderboard};
//!
//! // Создание нового состояния игры
//! let mut state = GameState::new();
//! let mut canvas = Canvas::new().expect("Не удалось создать Canvas");
//! let mut input = KeyReader::new();
//!
//! // Запуск игрового цикла
//! let score = state.play(&mut canvas, &mut input, &"0".to_string());
//! ```

// ============================================================================
// ЭКСПОРТ ТИПОВ
// ============================================================================

// Экспорт типов из модуля types
pub use crate::types::{Direction, RotationDirection, UpdateEndState};

// Экспорт основного типа ошибки
pub use crate::errors::GameError;

// Экспорт основных типов из модуля game
pub use crate::game::state::GameMode;
pub use crate::game::state::GameState;
pub use crate::game::stats::GameStats;

// Экспорт типов из модуля tetromino
pub use crate::tetromino::{BagGenerator, ShapeType, Tetromino};

// Экспорт типов из модуля io
pub use crate::io::{Canvas, KeyReader};

// Экспорт трейтов из модуля io_traits
pub use crate::io_traits::{InputReader, Renderer};

// Экспорт типов из модуля highscore
pub use crate::highscore::{Leaderboard, SaveData};

// Экспорт типов из модуля controls
pub use crate::controls::ControlsConfig;

// Экспорт криптографических утилит из модуля crypto
pub use crate::crypto::{generate_salt, hash};
