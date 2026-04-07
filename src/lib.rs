//! # Tetris CLI — Классическая игра Тетрис для терминала
//!
//! Tetris CLI — это классическая игра Тетрис, реализованная на Rust для терминала.
//! Поддерживает все 7 типов тетрамино, систему очков, таблицу лидеров и несколько режимов игры.
//!
//! ## Быстрый старт
//!
//! ```no_run
//! use tetris_cli::app;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     app::run();
//!     Ok(())
//! }
//! ```
//!
//! ## Примеры использования
//!
//! ### Создание состояния игры
//!
//! ```
//! use tetris_cli::game::state::GameState;
//!
//! let mut state = GameState::new();
//! assert_eq!(state.score(), 0);
//! assert_eq!(state.level(), 1);
//! ```
//!
//! ### Работа с очками и уровнями
//!
//! ```
//! use tetris_cli::game::types::{Score, Level, LinesCount};
//!
//! let mut score = Score::with_value(100);
//! score.add(200);
//! assert_eq!(score.value(), 300);
//!
//! let mut level = Level::default();
//! level.increment();
//! assert_eq!(level.value(), 2);
//! ```
//!
//! ### Конфигурация управления
//!
//! ```ignore
//! use tetris_cli::controls::ControlsConfig;
//!
//! let config = ControlsConfig::default_config();
//! // Проверка что клавиша 'a' сопоставлена с MoveLeft
//! assert!(config.left() == b'a');
//! ```
//!
//! ### Генерация фигур (Bag System)
//!
//! ```ignore
//! use tetris_cli::tetromino::BagGenerator;
//!
//! let mut bag = BagGenerator::new();
//! // fill_bag() и get_bag() — приватные методы,
//! // используются внутренне при создании Tetromino
//! ```
//!
//! ## Основные модули
//!
//! - [`game`] — игровая логика, состояние, цикл, система очков
//! - [`tetromino`] — типы фигур, генератор мешка, координаты
//! - [`io`] — ввод/вывод, терминальный канвас, читатель клавиш
//! - [`highscore`] — таблица лидеров, сохранение рекордов
//! - [`controls`] — конфигурация управления
//! - [`app`] — жизненный цикл приложения
//!
//! ## Документация
//!
//! - [README](../README.md) — полное описание проекта
//! - [CONTRIBUTING](../CONTRIBUTING.md) — руководство для участников
//! - [ARCHITECTURE](../ARCHITECTURE.md) — архитектурная документация
//! - [Исходный код](https://github.com/Githab-capibara/tetris-cli)

// ============================================================================
// МОДУЛИ БИБЛИОТЕКИ
// ============================================================================
// Каждый модуль отвечает за определённую часть функциональности игры.
// Модули организованы иерархически и могут импортировать друг друга.

// Макросы — определены ДО всех модулей для #[macro_use] совместимости
#[macro_use]
mod macros;

// Базовые модули (без зависимостей от других модулей проекта)
pub mod config;
pub mod constants;
pub mod core;
pub mod io_traits;
pub mod types;

// Остальные модули
#[macro_use]
pub mod app;
pub mod controls;
pub mod crypto;
pub mod errors;
pub mod game;
pub mod highscore;
pub mod io;
pub mod menu;
pub mod tetromino;
pub mod validation;

// Модуль публичного API для удобного импорта
pub mod exports;

// Экспорт публичного API
pub use exports::*;

// ============================================================================
// МОДУЛИ ТЕСТОВ
// ============================================================================
// Интеграционные тесты для проверки всех компонентов игры.
// Тесты разбиты по категориям для лучшей организации.

#[cfg(test)]
#[allow(clippy::all)]
#[allow(unused_must_use)]
mod tests {
    // Основные юнит-тесты
    mod test_collision;
    mod test_game_logic;
    mod test_integration;
    mod test_io;
    mod test_io_errors;
    mod test_io_utf8_handling;
    mod test_physics;

    // Игровая механика
    mod test_bag_system;
    mod test_boundary_values;
    mod test_game_bounds_check;
    mod test_game_movement;
    mod test_game_rotation;
    mod test_integration_extended;

    // Безопасность и переполнение
    mod test_hmac_safety;
    mod test_safe_cast;
    mod test_score_overflow_protection;
    mod test_state_validation;

    // Пакет 9: новые тесты (PROB-156..175)
    mod test_crypto_security;
    mod test_edge_cases;
    mod test_module_isolation;
    mod test_panic_handling;
}
