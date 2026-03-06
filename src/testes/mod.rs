//! Модули интеграционных тестов для Tetris CLI.
//!
//! Этот модуль содержит все тесты для проверки компонентов игры:
//! - `test_controls` - тесты конфигурации управления (20 тестов)
//! - `test_game_logic` - тесты игровой логики (30 тестов)
//! - `test_tetromino` - тесты фигур (25 тестов)
//! - `test_highscore` - тесты системы рекордов (15 тестов)
//! - `test_io` - тесты ввода/вывода (10 тестов)
//! - `test_integration` - интеграционные тесты (20 тестов)
//!
//! Итого: 120 тестов

pub mod test_controls;
pub mod test_game_logic;
pub mod test_tetromino;
pub mod test_highscore;
pub mod test_io;
pub mod test_integration;
