//! Модуль публичного API.
//!
//! Переэкспорт наиболее часто используемых типов для удобства импорта.
//!
//! ## Пример использования
//!
//! ```ignore
//! use tetris_cli::exports::*;
//!
//! // Теперь доступны все основные типы:
//! // - Direction, RotationDirection, UpdateEndState
//! // - GameMode, GameState, GameStats
//! // - BagGenerator, ShapeType, Tetromino
//! // - Canvas, KeyReader
//! // - InputReader, Renderer
//! // - Leaderboard, LeaderboardEntry, SaveData
//! // - ControlsConfig
//! // - generate_salt, hash, hmac, verify_hmac
//! ```

// Экспорт типов из модуля types
pub use crate::types::{Direction, RotationDirection, UpdateEndState};

// Экспорт основных типов из модуля game
pub use crate::game::{GameMode, GameState, GameStats};

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
pub use crate::crypto::{generate_salt, hash, hmac, verify_hmac};
