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
//! - `test_edge_cases_stress` - стресс-тесты и граничные значения (20 тестов)
//! - `test_game_movement` - тесты движения фигур (50 тестов)
//! - `test_game_rotation` - тесты вращения фигур (50 тестов)
//! - `test_tetromino_shapes` - тесты форм фигур (40 тестов)
//! - `test_bag_system` - тесты Bag Generator (30 тестов)
//! - `test_scoring_detailed` - детальные тесты очков (40 тестов)
//! - `test_game_modes_detailed` - детальные тесты режимов (40 тестов)
//! - `test_highscore_detailed` - детальные тесты рекордов (30 тестов)
//! - `test_controls_detailed` - детальные тесты управления (25 тестов)
//! - `test_io_detailed` - детальные тесты ввода-вывода (20 тестов)
//! - `test_integration_extended` - расширенные интеграционные тесты (50 тестов)
//! - `test_highscore_u256_fix` - тесты исправления U256 (3 теста)
//! - `test_game_bounds_check` - тесты проверок границ as cast (3 теста)
//! - `test_highscore_rate_limiting` - тесты rate limiting (3 теста)
//! - `test_controls_path_validation` - тесты валидации путей (3 теста)
//! - `test_game_wall_kick` - тесты wall kick (3 теста)
//! - `test_highscore_integrity` - тесты целостности (3 теста)
//!
//! Итого: 1123 теста

pub mod test_achievements;
pub mod test_controls;
pub mod test_game_logic;
pub mod test_highscore;
pub mod test_integration;
pub mod test_io;
pub mod test_physics;
pub mod test_tetromino;

// Новые расширенные тесты
pub mod test_animation;
pub mod test_collision;
pub mod test_edge_cases;
pub mod test_game_extended;
pub mod test_modes;
pub mod test_scoring;
pub mod test_statistics;
pub mod test_tetromino_extended;

// Стресс-тесты и граничные случаи (20 тестов)
pub mod test_edge_cases_stress;

// Интеграционные тесты режимов (20 тестов)
pub mod test_modes_integration;

// Новые детальные тесты (375 тестов)
pub mod test_bag_system;
pub mod test_controls_detailed;
pub mod test_game_modes_detailed;
pub mod test_game_movement;
pub mod test_game_rotation;
pub mod test_highscore_detailed;
pub mod test_integration_extended;
pub mod test_io_detailed;
pub mod test_scoring_detailed;
pub mod test_tetromino_shapes;

// Тесты исправленных проблем (18 тестов)
pub mod test_controls_path_validation;
pub mod test_game_bounds_check;
pub mod test_game_wall_kick;
pub mod test_highscore_integrity;
pub mod test_highscore_rate_limiting;
pub mod test_highscore_u256_fix;

// Тесты верификации всех исправлений аудита (27 тестов)
pub mod test_fixes_verification;
