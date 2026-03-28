//! Модуль публичного API.
//!
//! Переэкспорт наиболее часто используемых типов.

// Экспорт типов из модуля types
pub use crate::types::{Direction, RotationDirection, UpdateEndState};

// Экспорт основного типа ошибки
pub use crate::errors::GameError;

// Экспорт основных типов из модуля game
pub use crate::game::state::{GameMode, GameState, GameStats};

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
pub use crate::crypto::{generate_salt, hash, keyed_hash, verify_keyed_hash};
