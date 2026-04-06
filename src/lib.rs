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

// Базовые модули (без зависимостей от других модулей проекта)
pub mod config;
pub mod constants;
pub mod core;
pub mod io_traits;
pub mod types;

// Остальные модули
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
// МАКРОСЫ (ИСПРАВЛЕНИЕ #10 - HIGH)
// ============================================================================

/// Макрос для логирования ошибок с префиксом `"[ERROR]"`.
///
/// Использует `eprintln!` так как это CLI-приложение без logger framework.
/// `eprintln!` гарантирует вывод в stderr даже при панике.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] $($arg)*");
    }};
}

/// Макрос для логирования предупреждений с префиксом `"[WARN]"`.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        eprintln!("[WARN] $($arg)*");
    }};
}

/// Макрос для логирования информации с префиксом `"[INFO]"`.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        eprintln!("[INFO] $($arg)*");
    }};
}

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
    mod test_game_logic;
    mod test_integration;
    mod test_io;
    mod test_io_errors;
    mod test_physics;

    // Игровая механика
    mod test_bag_system;
    mod test_boundary_values;
    mod test_game_bounds_check;
    mod test_game_box_array;
    mod test_game_movement;
    mod test_game_rotation;
    mod test_integration_extended;

    // Безопасность и переполнение
    mod test_game_score_overflow;
    mod test_hmac_safety;
    mod test_safe_cast;
    mod test_score_overflow_protection;
    mod test_state_validation;
}
