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
    pub mod test_game_logic;
    pub mod test_integration;
    pub mod test_io;
    pub mod test_io_errors;
    pub mod test_physics;

    // Игровая механика
    pub mod test_bag_system;
    pub mod test_boundary_values;
    pub mod test_game_bounds_check;
    pub mod test_game_box_array;
    pub mod test_game_movement;
    pub mod test_game_rotation;
    pub mod test_integration_extended;

    // Безопасность и переполнение
    pub mod test_error_propagation;
    pub mod test_game_score_overflow;
    pub mod test_hmac_safety;
    pub mod test_safe_cast;
    pub mod test_score_overflow_protection;
    pub mod test_state_validation;
    pub mod test_unwrap_to_expect;
}
