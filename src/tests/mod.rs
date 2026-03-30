//! Модули интеграционных тестов для Tetris CLI.
//!
//! Этот модуль содержит все тесты для проверки компонентов игры.
//!
//! ## Структура тестов
//!
//! ### Базовые тесты компонентов
//! - `test_controls` - тесты конфигурации управления
//! - `test_game_logic` - тесты игровой логики
//! - `test_tetromino` - тесты фигур
//! - `test_highscore` - тесты системы рекордов
//! - `test_io` - тесты ввода/вывода
//! - `test_integration` - интеграционные тесты
//! - `test_physics` - тесты физической механики
//! - `test_scoring` - тесты системы очков
//! - `test_collision` - тесты столкновений
//! - `test_animation` - тесты анимаций
//! - `test_modes_integration` - интеграция режимов игры
//! - `test_statistics` - тесты статистики
//!
//! ### Расширенные тесты
//! - `test_edge_cases` - тесты граничных случаев
//! - `test_edge_cases_stress` - стресс-тесты
//! - `test_game_movement` - тесты движения фигур
//! - `test_game_rotation` - тесты вращения фигур
//! - `test_tetromino_shapes` - тесты форм фигур
//! - `test_bag_system` - тесты Bag Generator
//! - `test_game_modes_detailed` - детальные тесты режимов
//! - `test_integration_extended` - расширенные интеграционные тесты
//!
//! ### Тесты исправлений и безопасности
//! - `test_controls_path_validation` - валидация путей
//! - `test_game_bounds_check` - проверки границ
//! - `test_controls_error_handling` - обработка ошибок
//! - `test_game_negative_coords` - отрицательные координаты
//! - `test_io_resource_leak` - утечка ресурсов
//! - `test_safety_architecture` - тесты архитектуры безопасности (Critical/HIGH)
//!
//! ### Архитектурные тесты
//! - `test_architecture` - архитектурная целостность
//!
//! ### Тесты качества кода
//! - `test_must_use_attributes` - атрибуты must_use
//! - `test_track_caller` - атрибуты track_caller
//! - `test_cfg_attr_dead_code` - cfg_attr и dead_code
//! - `test_cast_safety` - безопасность cast
//! - `test_unicode_validation` - валидация Unicode
//!
//! ### Тесты оптимизаций
//! - `test_sanitize_optimization` - оптимизация sanitize
//! - `test_string_caching` - кеширование строк
//! - `test_bounds_check_optimization` - оптимизация проверок границ
//! - `test_row_check_optimization` - оптимизация проверки линий

#![allow(clippy::needless_range_loop)]
#![allow(clippy::unused_local_helper)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::should_panic_without_expect)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(deprecated)]

// ============================================================================
// БАЗОВЫЕ ТЕСТЫ КОМПОНЕНТОВ
// ============================================================================

// Общие фикстуры и хелперы
pub mod fixtures;

pub mod test_controls;
pub mod test_game_logic;
pub mod test_highscore;
pub mod test_integration;
pub mod test_io;
pub mod test_physics;
pub mod test_tetromino;

// ============================================================================
// РАСШИРЕННЫЕ ТЕСТЫ
// ============================================================================

pub mod test_animation;
pub mod test_collision;
pub mod test_edge_cases;
pub mod test_edge_cases_stress;
pub mod test_game_movement;
pub mod test_game_rotation;
pub mod test_statistics;
pub mod test_tetromino_shapes;

// Детальные тесты
pub mod test_bag_system;
pub mod test_game_modes_detailed;
pub mod test_integration_extended;

// ============================================================================
// ТЕСТЫ ИСПРАВЛЕНИЙ И БЕЗОПАСНОСТИ
// ============================================================================

pub mod test_controls_error_handling;
pub mod test_controls_path_traversal;
pub mod test_controls_path_validation;
pub mod test_controls_toctou;

pub mod test_game_bitmask_check_rows;
pub mod test_game_bounds_check;
pub mod test_game_box_array;
pub mod test_game_negative_coords;
pub mod test_game_rotation_bounds;
pub mod test_game_score_overflow;
pub mod test_game_stats_export;
pub mod test_score_overflow_protection;
pub mod test_state_validation;

pub mod test_highscore_config_path;
pub mod test_highscore_error_handling;
pub mod test_highscore_random_hash;
pub mod test_highscore_verify_integrity;
pub mod test_leaderboard_toctou;

pub mod test_io_canvas_result;
pub mod test_io_drop;
pub mod test_io_resource_leak;
pub mod test_io_utf8_handling;

pub mod test_tetromino_dir_down;

pub mod test_hard_drop_flag;

pub mod test_direction_down;
pub mod test_wall_kick_refactor;

pub mod test_error_propagation;
pub mod test_unwrap_to_expect;

pub mod test_benchmarks;

// Интеграционные тесты режимов
pub mod test_modes_integration;

// Тесты констант
pub mod test_constant_imports;

// ============================================================================
// АРХИТЕКТУРНЫЕ ТЕСТЫ
// ============================================================================

pub mod test_architecture;

// Тесты целостности архитектуры
pub mod test_architecture_integrity;

// Тесты исправленных архитектурных проблем
pub mod test_architecture_fixes;

// ============================================================================
// НОВЫЕ АРХИТЕКТУРНЫЕ ТЕСТЫ (2026-03-30)
// ============================================================================

// Тесты на отсутствие мёртвого кода
pub mod test_architecture_components;

// Тесты на консолидацию трейтов
pub mod test_architecture_traits;

// Тесты на централизацию валидации
pub mod test_architecture_validation;

// Тесты на разделение render/logic
pub mod test_architecture_separation;

// Тесты на разделение трейтов (ISP)
pub mod test_architecture_isp;

// Тесты на снижение связанности
pub mod test_architecture_coupling;

// ============================================================================
// ТЕСТЫ КАЧЕСТВА КОДА
// ============================================================================

pub mod macros;
pub mod test_cast_safety;
pub mod test_cfg_attr_dead_code;
pub mod test_must_use_attributes;
pub mod test_track_caller;
pub mod test_unicode_validation;

// Тесты безопасной конвертации типов
pub mod test_safe_cast;

// ============================================================================
// ТЕСТЫ ОПТИМИЗАЦИЙ
// ============================================================================

pub mod test_bounds_check_optimization;
pub mod test_row_check_optimization;
pub mod test_sanitize_optimization;
pub mod test_scoring_state;

// ============================================================================
// ТЕСТЫ БЕЗОПАСНОСТИ
// ============================================================================

pub mod test_safety_architecture;
pub mod test_security_fixes;
pub mod test_time_safety;

// Тесты инициализации Canvas и обработки IoError
pub mod test_canvas_initialization;

// Тесты безопасности HMAC
pub mod test_hmac_safety;

// Тесты обработки ошибок Application
pub mod test_application_error_handling;

// ============================================================================
// ОБЩИЕ ТЕСТЫ ИСПРАВЛЕНИЙ
// ============================================================================

pub mod test_fixes;

// ============================================================================
// ОГРАНИЧЕНИЯ И СПЕЦИАЛЬНЫЕ ТЕСТЫ
// ============================================================================

pub mod test_utf8_limitation;

pub mod test_task13_coverage;
