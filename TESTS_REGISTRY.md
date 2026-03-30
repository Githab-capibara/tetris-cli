# 📋 TESTS REGISTRY - Tetris CLI

**Дата последней актуализации:** 30 марта 2026 г. (архитектурные улучшения)
**Версия проекта:** 23.96.26
**Всего тестов:** 1227+ (проходят 100%)
**Всего файлов тестов:** 89

---

## 📊 ТЕКУЩАЯ СТАТИСТИКА

### Изменения после архитектурных улучшений (30 марта 2026):

#### Новые архитектурные тесты (67 тестов):

**Тесты архитектурных компонентов (7 тестов):**
- `test_architecture_components.rs` — проверка отсутствия мёртвого кода:
  - `test_figure_manager_not_used` — FigureManager не используется
  - `test_animation_state_not_used` — AnimationState не используется
  - `test_game_phase_not_used` — GamePhase не используется
  - `test_no_unused_components_in_components_rs` — нет мёртвого кода
  - `test_game_board_is_used` — GameBoard используется
  - `test_score_board_is_used` — ScoreBoard используется
  - `test_no_dead_code_in_components_module` — нет мёртвого кода в трейтах

**Тесты архитектурных трейтов (8 тестов):**
- `test_architecture_traits.rs` — консолидация трейтов в access.rs:
  - `test_board_readonly_defined_only_in_access` — BoardReadonly в access.rs
  - `test_board_mutable_defined_only_in_access` — BoardMutable в access.rs
  - `test_no_duplicate_traits_in_board_rs` — нет дублирования в board.rs
  - `test_board_rs_imports_traits_from_access` — board.rs импортирует трейты
  - `test_traits_reexported_from_access` — переэкспорт из access.rs
  - `test_score_access_defined_only_in_access` — ScoreAccess в access.rs
  - `test_no_duplicate_traits_in_scoreboard_rs` — нет дублирования в scoreboard.rs
  - `test_all_access_traits_consolidated_in_access` — все трейты в access.rs

**Тесты архитектурной валидации (12 тестов):**
- `test_architecture_validation.rs` — централизация валидации:
  - `test_validation_service_exists` — ValidationService существует
  - `test_validation_service_structure` — правильная структура
  - `test_validate_f32_finite_used_in_set_fall_speed` — валидация fall_speed
  - `test_validate_f32_finite_used_in_set_land_timer` — валидация land_timer
  - `test_validate_u32_range_exists_and_works` — validate_u32_range работает
  - `test_validate_u32_range_used_in_set_fall_speed` — используется в set_fall_speed
  - `test_no_duplicate_validation_logic` — нет дублирования валидации
  - `test_validation_error_used_for_all_validation_errors` — ValidationError используется
  - `test_all_validation_centralized_in_validation_service` — централизация
  - `test_validation_follows_dry_principle` — следует DRY принципу
  - `test_validation_error_used_consistently` — консистентное использование
  - `test_validation_integration_with_game_state` — интеграция с GameState

**Тесты разделения ответственности (9 тестов):**
- `test_architecture_separation.rs` — разделение render/logic:
  - `test_check_rows_not_called_in_render_rs` — check_rows не в render.rs
  - `test_render_rs_does_not_import_check_rows` — render.rs не импортирует
  - `test_render_function_does_not_contain_line_clearing_logic` — нет логики линий
  - `test_render_rs_does_not_contain_line_removal_functions` — нет функций удаления
  - `test_line_logic_in_scoring_lines_rs` — логика линий в scoring/lines.rs
  - `test_check_rows_in_scoring_lines_rs` — check_rows в scoring/lines.rs
  - `test_logic_update_rs_coordinates_line_logic` — logic/update.rs координирует
  - `test_handle_landing_calls_check_rows` — handle_landing вызывает check_rows
  - `test_render_and_logic_are_separated` — render и logic разделены

**Тесты Interface Segregation Principle (13 тестов):**
- `test_architecture_isp.rs` — разделение трейтов:
  - `test_score_access_contains_only_score_methods` — ScoreAccess: только очки
  - `test_score_access_does_not_contain_level_methods` — нет методов уровней
  - `test_score_access_does_not_contain_lines_methods` — нет методов линий
  - `test_score_access_does_not_contain_combo_methods` — нет методов комбо
  - `test_level_access_contains_only_level_methods` — LevelAccess: только уровни
  - `test_level_access_does_not_contain_score_methods` — нет методов очков
  - `test_lines_access_contains_only_lines_methods` — LinesAccess: только линии
  - `test_lines_access_does_not_contain_score_methods` — нет методов очков
  - `test_combo_access_contains_only_combo_methods` — ComboAccess: только комбо
  - `test_combo_access_does_not_contain_score_methods` — нет методов очков
  - `test_scoring_state_inherits_narrow_traits` — ScoringState наследует трейты
  - `test_scoring_state_can_be_used_through_narrow_traits` — использование через трейты
  - `test_traits_follow_isp_principle` — следуют ISP принципу

**Тесты снижения связанности (11 тестов):**
- `test_architecture_coupling.rs` — низкая связанность:
  - `test_scoring_points_no_direct_access_to_gamestate_fields` — нет прямого доступа
  - `test_scoring_points_uses_encapsulation` — использует инкапсуляцию
  - `test_scoring_lines_uses_public_methods_only` — только публичные методы
  - `test_scoring_lines_no_direct_field_access` — нет прямого доступа к полям
  - `test_score_logic_encapsulated_in_scoreboard` — инкапсуляция в ScoreBoard
  - `test_scoreboard_has_clear_public_api` — чёткий публичный API
  - `test_coupling_reduced_through_traits` — снижение через трейты
  - `test_gamestate_not_dependent_on_concrete_implementations` — нет зависимости
  - `test_internal_logic_encapsulated` — внутренняя логика инкапсулирована
  - `test_gamestate_fields_are_private` — поля приватные
  - `test_coupling_architecture_test` — архитектурный тест

**Тесты целостности архитектуры (дополнено 4 тестами):**
- `test_architecture_integrity.rs` — целостность архитектуры:
  - `test_component_separation` — разделение компонентов
  - `test_module_boundaries` — границы модулей
  - `test_encapsulation` — инкапсуляция
  - `test_dependency_inversion` — инверсия зависимостей
  - `test_error_handling` — обработка ошибок
  - `test_no_circular_dependencies` — отсутствие циклов
  - `test_solid_principles` — принципы SOLID
  - `test_backward_compatibility` — обратная совместимость
  - ... (ещё 15 тестов)

#### Итоговая статистика:
- **Всего тестов:** 1227+
- **Удалено тестов:** 19 (избыточные и тривиальные)
- **Переименовано тестов:** 8 (устранение дубликатов)
- **Обновлено тестов:** 5 (добавлены assert)
- **Добавлено тестов:** 129 (новые тесты безопасности и архитектурные тесты)
- **Все тесты компилируются:** ✅
- **Все тесты проходят:** ✅ (100%)

---

## 📊 ОБЩАЯ СТАТИСТИКА

### Общее количество тестов: 1227+

**Unit тесты:** 946
**Integration тесты:** 281 (architecture + fixes verification + edge cases)

**Процент прохождения:** 100% (0 failed)

### Структура тестовых файлов:

**tests/** (интеграционные тесты):
- `test_architecture_improvements.rs` — 9 тестов
- `test_architecture_integrity.rs` — 17 тестов
- `test_architecture_refactoring.rs` — 37 тестов
- `test_fixes_verification.rs` — 14 тестов
- `test_architecture_components.rs` — 29 тестов

**src/tests/** (unit тесты):
- `io_drop.rs` — 6 тестов
- `leaderboard_toctou.rs` — 10 тестов
- `scoring_state.rs` — 15 тестов
- `macros.rs` — 11 тестов (после очистки)
- `test_all_fixes_integration.rs` — 10 тестов
- `test_animation.rs` — 14 тестов (после очистки)
- `test_application_error_handling.rs` — 15 тестов (обработка ошибок Application)
- `test_architecture.rs` — 19 тестов
- `test_architecture_components.rs` — 7 тестов (архитектурные компоненты)
- `test_architecture_traits.rs` — 8 тестов (архитектурные трейты)
- `test_architecture_validation.rs` — 12 тестов (архитектурная валидация)
- `test_architecture_separation.rs` — 9 тестов (разделение ответственности)
- `test_architecture_isp.rs` — 13 тестов (Interface Segregation)
- `test_architecture_coupling.rs` — 11 тестов (снижение связанности)
- `test_architecture_integrity.rs` — 23 теста (целостность архитектуры)
- `test_bag_system.rs` — 27 тестов
- `test_benchmarks.rs` — 4 теста (после очистки)
- `test_bounds_check_optimization.rs` — 7 тестов
- `test_canvas_initialization.rs` — 15 тестов (инициализация Canvas)
- `test_cast_safety.rs` — 11 тестов
- `test_cfg_attr_dead_code.rs` — 7 тестов
- `test_clippy_fixes.rs` — 7 тестов
- `test_collision.rs` — 10 тестов
- `test_constant_imports.rs` — 7 тестов
- `test_controls.rs` — 20 тестов
- `test_controls_error_handling.rs` — 3 теста
- `test_controls_path_traversal.rs` — 5 тестов
- `test_controls_path_validation.rs` — 7 тестов
- `test_controls_toctou.rs` — 10 тестов
- `test_deprecated_methods.rs` — 4 теста
- `test_direction_down.rs` — 5 тестов
- `test_edge_cases.rs` — 12 тестов
- `test_edge_cases_stress.rs` — 10 тестов
- `test_error_propagation.rs` — 5 тестов
- `test_fixes.rs` — 15 тестов
- `test_game_bitmask_check_rows.rs` — 6 тестов
- `test_game_bounds_check.rs` — 5 тестов
- `test_game_box_array.rs` — 4 теста
- `test_game_logic.rs` — 25 тестов
- `test_game_modes_detailed.rs` — 12 тестов
- `test_game_movement.rs` — 8 тестов
- `test_game_negative_coords.rs` — 6 тестов
- `test_game_rotation.rs` — 10 тестов
- `test_game_rotation_bounds.rs` — 5 тестов
- `test_game_score_overflow.rs` — 3 теста
- `test_game_stats_export.rs` — 4 теста
- `test_hard_drop_flag.rs` — 3 теста
- `test_hard_drop_overflow.rs` — 3 теста
- `test_highscore.rs` — 15 тестов
- `test_highscore_config_path.rs` — 3 теста
- `test_highscore_error_handling.rs` — 5 тестов
- `test_highscore_integrity.rs` — 10 тестов
- `test_highscore_random_hash.rs` — 4 теста
- `test_highscore_verify_integrity.rs` — 5 тестов
- `test_hmac_keys.rs` — 7 тестов
- `test_hmac_safety.rs` — 18 тестов (безопасность HMAC-SHA256)
- `test_integration.rs` — 20 тестов
- `test_integration_extended.rs` — 50 тестов
- `test_io.rs` — 10 тестов
- `test_io_canvas_result.rs` — 3 теста
- `test_io_errors.rs` — 8 тестов
- `test_io_resource_leak.rs` — 2 теста
- `test_io_utf8_handling.rs` — 5 тестов
- `test_modes_integration.rs` — 22 теста
- `test_must_use_attributes.rs` — 5 тестов
- `test_physics.rs` — 8 тестов
- `test_row_check_optimization.rs` — 6 тестов
- `test_safe_cast.rs` — 14 тестов (безопасная конвертация f32 → u32)
- `test_safety_architecture.rs` — 10 тестов
- `test_sanitize_optimization.rs` — 6 тестов
- `test_scoring_encapsulation.rs` — 5 тестов
- `test_score_overflow_protection.rs` — 12 тестов
- `test_security_fixes.rs` — 20 тестов
- `test_state_validation.rs` — 15 тестов
- `test_statistics.rs` — 5 тестов
- `test_string_caching.rs` — 6 тестов
- `test_task13_coverage.rs` — 10 тестов
- `test_tetromino.rs` — 25 тестов
- `test_tetromino_dir_down.rs` — 3 теста
- `test_tetromino_shapes.rs` — 4 теста
- `test_time_safety.rs` — 5 тестов
- `test_track_caller.rs` — 3 теста
- `test_unicode_validation.rs` — 6 тестов
- `test_unwrap_to_expect.rs` — 2 теста
- `test_utf8_limitation.rs` — 3 теста
- `test_validation_name.rs` — 6 тестов
- `test_wall_kick_refactor.rs` — 5 тестов
