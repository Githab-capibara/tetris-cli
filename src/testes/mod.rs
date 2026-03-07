//! Модули интеграционных тестов для Tetris CLI.
//!
//! Этот модуль содержит все тесты для проверки компонентов игры:
//! - `test_controls` - тесты конфигурации управления (20 тестов)
//! - `test_game_logic` - тесты игровой логики (30 тестов)
//! - `test_tetromino` - тесты фигур (25 тестов)
//! - `test_highscore` - тесты системы рекордов (15 тестов)
//! - `test_io` - тесты ввода/вывода (10 тестов)
//! - `test_integration` - интеграционные тесты (20 тестов)
//! - `test_achievements` - тесты системы достижений (20 тестов)
//! - `test_physics` - тесты физической механики (20 тестов)
//! - `test_game_extended` - расширенные тесты игровой логики (100 тестов)
//! - `test_tetromino_extended` - расширенные тесты фигур (80 тестов)
//! - `test_scoring` - тесты системы очков (50 тестов)
//! - `test_collision` - тесты столкновений (50 тестов)
//! - `test_animation` - тесты анимаций (30 тестов)
//! - `test_modes` - тесты режимов игры (40 тестов)
//! - `test_statistics` - тесты статистики (30 тестов)
//! - `test_edge_cases` - тесты граничных случаев (50 тестов)
//!
//! Итого: 690 тестов

pub mod test_achievements;
pub mod test_controls;
pub mod test_game_logic;
pub mod test_tetromino;
pub mod test_highscore;
pub mod test_io;
pub mod test_integration;
pub mod test_physics;

// Новые расширенные тесты
pub mod test_game_extended;
pub mod test_tetromino_extended;
pub mod test_scoring;
pub mod test_collision;
pub mod test_animation;
pub mod test_modes;
pub mod test_statistics;
pub mod test_edge_cases;
