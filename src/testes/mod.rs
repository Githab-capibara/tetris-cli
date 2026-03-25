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

#![allow(clippy::needless_range_loop)]
#![allow(clippy::unused_local_helper)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::should_panic_without_expect)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! - `test_highscore_u256_fix` - тесты исправления U256 (3 теста)
//! - `test_game_bounds_check` - тесты проверок границ as cast (3 теста)
//! - `test_highscore_rate_limiting` - тесты rate limiting (3 теста)
//! - `test_controls_path_validation` - тесты валидации путей (3 теста)
//! - `test_game_wall_kick` - тесты wall kick (3 теста)
//! - `test_highscore_integrity` - тесты целостности (3 теста)
//! - `test_fixes_verification` - верификация исправлений (27 тестов)
//!
//! Тесты критических проблем (20 тестов - по 4-5 на каждую из 5 проблем)
//! - `test_controls_error_handling` - обработка ошибок (5 тестов)
//! - `test_game_negative_coords` - отрицательные координаты (4 теста)
//! - `test_game_stack_overflow` - переполнение стека (3 теста)
//! - `test_highscore_no_rate_limiting` - отсутствие rate limiting (5 тестов)
//! - `test_io_resource_leak` - утечка ресурсов (5 тестов)
//!
//! Комплексные тесты всех 32 исправленных проблем (33 теста)
//! - `test_32_fixes_comprehensive` - тесты для каждой из 32 проблем
//!
//! Итого: 1489 тестов

// pub mod test_achievements; // REMOVED: Achievement system removed
pub mod test_all_32_fixes;
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

// Тесты критических проблем (15 тестов - по 3 на каждую из 5 проблем)
pub mod test_controls_error_handling;
pub mod test_game_negative_coords;
pub mod test_game_stack_overflow;
pub mod test_game_box_array;
pub mod test_game_score_overflow;
pub mod test_game_bitmask_check_rows;
pub mod test_highscore_no_rate_limiting;
pub mod test_highscore_verify_integrity;
pub mod test_highscore_random_hash;
pub mod test_io_resource_leak;
pub mod test_io_utf8_handling;
pub mod test_highscore_error_handling;
pub mod test_tetromino_dir_down;
pub mod test_unwrap_to_expect;
pub mod test_error_propagation;
pub mod test_benchmarks; // REMOVED: benchmark methods removed
pub mod test_highscore_deprecated_assert_hs;
pub mod test_game_rotation_bounds;
pub mod test_controls_path_traversal;
pub mod test_game_score_overflow_protection;
pub mod test_fixes_must_use_stack_format;
pub mod test_fixes_bag_preview_rotate;
pub mod test_fixes_documentation_validation;
pub mod test_fixes_final_issues;

// Комплексные тесты всех 18 исправлений (54 теста)
pub mod test_all_fixes;

// Расширенные тесты верификации всех 13 исправлений (39 тестов)
pub mod test_fixes_verification_extended;

// Тесты аудита 12 исправлений (36 тестов)
pub mod test_fixes_audit;

// Тесты качества кода (24 теста - по 3 на каждую из 8 категорий)
pub mod test_code_quality;

// Тесты предотвращения регрессии исправлений аудита (54 теста - по 3 на каждую из 18 проблем)
pub mod test_audit_fixes_prevention;

// Тесты верификации 17 исправленных проблем аудита (19 тестов - по 1 на каждую проблему + интеграционный)
pub mod test_audit_fixes_verification;

// Тесты текущего аудита (12 тестов - по 3 на каждую из 4 проблем)
pub mod test_audit_current_fixes;

// Финальная верификация всех 25 исправлений (75 тестов - по 3 на каждую проблему)
pub mod test_fixes_verification_final;

// Тесты для 25 исправленных проблем (25 тестов - по 1 на каждую проблему)
pub mod test_25_fixes;

// Тесты исправлений безопасности и оптимизаций (7 тестов - 6 основных + 1 интеграционный)
pub mod test_security_fixes;

// Тесты качества кода для 6 исправлений (6 тестов + 1 интеграционный)
pub mod test_code_quality_fixes;

// Тесты для всех исправленных проблем аудита (12 тестов + 1 интеграционный)
pub mod test_audit_fixes;

// Комплексные тесты для всех 32 исправленных проблем (32 теста + 1 интеграционный)
pub mod test_32_fixes_comprehensive;

// Тесты на архитектурную целостность (16 тестов)
pub mod test_architecture;

// ============================================================================
// НОВЫЕ ТЕСТЫ ДЛЯ ВСЕХ ИСПРАВЛЕНИЙ (61 тест)
// ============================================================================

// Тесты критических ошибок (8 тестов)
pub mod test_game_stats_export;
pub mod test_constant_imports;

// Тесты логических ошибок (13 тестов)
pub mod test_hard_drop_flag;
pub mod test_direction_down;
pub mod test_wall_kick_refactor;

// Тесты оптимизаций (18 тестов)
pub mod test_sanitize_optimization;
pub mod test_string_caching;
pub mod test_bounds_check_optimization;
pub mod test_row_check_optimization;

// Тесты безопасности (16 тестов)
pub mod test_time_safety;
pub mod test_hard_drop_overflow;
pub mod test_unicode_validation;

// Тесты best practices (11 тестов)
pub mod test_must_use_attributes;
pub mod test_track_caller;
pub mod test_cfg_attr_dead_code;

// Тесты на архитектурную целостность (20 тестов)
pub mod test_architecture_integrity;

// Интеграционные тесты (10 тестов)
pub mod test_all_fixes_integration;
