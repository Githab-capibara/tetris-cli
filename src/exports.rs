//! Модуль публичного API.
//!
//! Переэкспорт наиболее часто используемых типов для удобного импорта.
//!
//! # Примеры использования API
//!
//! ## Быстрый старт игры
//!
//! ```ignore
//! use tetris_cli::exports::{GameState, Canvas, KeyReader};
//!
//! // Создание нового состояния игры
//! let mut state = GameState::new();
//!
//! // Инициализация канваса и читателя нажатий
//! let mut canvas = Canvas::new().expect("Не удалось создать Canvas");
//! let mut input = KeyReader::new();
//!
//! // Запуск игрового цикла
//! let score = state.play(&mut canvas, &mut input, &"0".to_string());
//! ```
//!
//! ## Работа с таблицей лидеров
//!
//! ```ignore
//! use tetris_cli::exports::Leaderboard;
//!
//! // Загрузка таблицы лидеров
//! let mut leaderboard = Leaderboard::load();
//!
//! // Добавление нового рекорда
//! leaderboard.add_score("Игрок".to_string(), 5000);
//!
//! // Сохранение изменений
//! leaderboard.save();
//! ```
//!
//! ## Настройка управления
//!
//! ```ignore
//! use tetris_cli::exports::ControlsConfig;
//!
//! // Создание кастомной конфигурации (стиль Vim: HJKL)
//! let config = ControlsConfig::custom(
//!     b'h', // влево
//!     b'l', // вправо
//!     b'j', // soft drop
//!     b'k', // hard drop
//!     b'y', // вращение влево
//!     b'u', // вращение вправо
//!     b'i', // удержание
//!     b'o', // пауза
//!     127,  // выход (Backspace)
//! );
//!
//! // Сохранение конфигурации
//! config.save_to_file("vim_controls.json").unwrap();
//! ```
//!
//! ## Криптографические утилиты
//!
//! ```ignore
//! use tetris_cli::exports::{hash, generate_salt, hmac_sha256, verify_hmac_sha256};
//!
//! // Хеширование данных
//! let data_hash = hash("важные данные");
//!
//! // Генерация соли
//! let salt = generate_salt();
//!
//! // HMAC подпись
//! let key = "секретный ключ";
//! let signature = hmac_sha256(key, "данные");
//!
//! // Проверка подписи
//! assert!(verify_hmac_sha256(key, "данные", &signature));
//! ```

// ============================================================================
// ЭКСПОРТ ТИПОВ
// ============================================================================

// Экспорт типов из модуля types
pub use crate::types::{Direction, RotationDirection, UpdateEndState};

// Экспорт основного типа ошибки
pub use crate::errors::GameError;

// Экспорт основных типов из модуля game
#[allow(deprecated)]
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
pub use crate::crypto::{generate_salt, hash, hmac_sha256, verify_hmac_sha256};
