# 📋 TESTS REGISTRY - Tetris CLI

**Дата последней актуализации:** 30 марта 2026 г.
**Версия проекта:** 23.96.26
**Всего тестов:** 1186 (проходят 100%)
**Всего файлов тестов:** 80

---

## 📊 ТЕКУЩАЯ СТАТИСТИКА

### Новые тесты (30 марта 2026):

#### Добавленные тесты (37 тестов):

**Тесты защиты от переполнения очков (12 тестов):**
- `test_score_overflow_protection.rs` — защита от переполнения счёта:
  - `test_score_does_not_exceed_max` — проверка ограничения MAX_SCORE
  - `test_saturating_add_normal_values` — saturating_add с нормальными значениями
  - `test_saturating_add_overflow_protection` — защита от переполнения u128
  - `test_extreme_level_10000_plus` — экстремальный уровень 10000+
  - `test_extreme_combo_1000_plus` — экстремальное комбо 1000+
  - `test_extreme_level_and_combo_combined` — уровень и комбо одновременно
  - `test_stress_many_score_additions` — 10000+ начислений
  - `test_game_state_score_overflow` — интеграция с GameState
  - `test_update_score_for_lines_overflow_protection` — защита в функции
  - `test_max_score_constant` — проверка константы MAX_SCORE
  - `test_saturating_add_edge_cases` — краевые случаи
  - `test_no_panic_at_extreme_values` — отсутствие паник

**Тесты валидации fall_speed/land_timer (15 тестов):**
- `test_state_validation.rs` — валидация значений GameState:
  - `test_set_fall_speed_nan_returns_error` — NaN защита fall_speed
  - `test_set_fall_speed_positive_infinity_returns_error` — +Infinity защита
  - `test_set_fall_speed_negative_infinity_returns_error` — -Infinity защита
  - `test_set_fall_speed_valid_values` — валидные значения
  - `test_set_fall_speed_clamps_to_valid_range` — clamp диапазона
  - `test_set_land_timer_nan_returns_error` — NaN защита land_timer
  - `test_set_land_timer_positive_infinity_returns_error` — +Infinity защита
  - `test_set_land_timer_negative_infinity_returns_error` — -Infinity защита
  - `test_set_land_timer_valid_values` — валидные значения
  - `test_set_land_timer_negative_values_clamped_to_zero` — отрицательные значения
  - `test_set_fall_speed_boundary_values` — граничные значения
  - `test_set_land_timer_boundary_values` — граница 0.0
  - `test_validation_in_game_state_context` — интеграция
  - `test_validation_no_panic_on_invalid_values` — отсутствие паник
  - `test_validation_stress_test` — стресс-тест

**Тесты TOCTOU защиты controls (10 тестов):**
- `test_controls_toctou.rs` — защита от Time-Of-Check-Time-Of-Use атак:
  - `test_symlink_rejected_on_load` — отклонение symlink при загрузке
  - `test_symlink_rejected_on_save` — отклонение symlink при сохранении
  - `test_broken_symlink_handling` — обработка несуществующих symlink
  - `test_o_nofollow_applied_on_save` — применение O_NOFOLLOW при сохранении
  - `test_o_nofollow_applied_on_load` — применение O_NOFOLLOW при загрузке
  - `test_o_nofollow_prevents_race_condition` — предотвращение race condition
  - `test_toctou_protection_integration` — интеграционный тест
  - `test_regular_files_work_correctly` — работа с обычными файлами
  - `test_multiple_symlinks_attack` — защита от множественных symlink
  - `test_toctou_protection_no_panic` — отсутствие паник

#### Итоговая статистика:
- **Всего тестов:** 1186
- **Добавлено тестов:** 45 (HMAC, config keys, IO drop, leaderboard TOCTOU, scoring state, macros)
- **Все тесты компилируются:** ✅
- **Все тесты проходят:** ✅ (100%)

---

## 📊 ОБЩАЯ СТАТИСТИКА

### Общее количество тестов: 1186

**Unit тесты:** 898
**Integration тесты:** 288 (architecture + fixes verification + edge cases)

**Процент прохождения:** 100% (0 failed)

### Структура тестовых файлов:

**tests/** (интеграционные тесты):
- `test_architecture_improvements.rs` — 9 тестов
- `test_architecture_integrity.rs` — 17 тестов
- `test_architecture_refactoring.rs` — 37 тестов
- `test_fixes_verification.rs` — 14 тестов
- `test_architecture_components.rs` — 29 тестов

**src/tests/** (unit тесты):
- `io_drop.rs` — 6 тестов (новые)
- `leaderboard_toctou.rs` — 10 тестов (новые)
- `scoring_state.rs` — 15 тестов (новые)
- `macros.rs` — 14 тестов (новые)
- `test_all_fixes_integration.rs` — 10 тестов
- `test_animation.rs` — 15 тестов
- `test_architecture.rs` — 19 тестов
- `test_bag_system.rs` — 4 теста
- `test_benchmarks.rs` — бенчмарки
- `test_bounds_check_optimization.rs` — 5 тестов
- `test_cast_safety.rs` — 6 тестов
- `test_cfg_attr_dead_code.rs` — 2 теста
- `test_clippy_fixes.rs` — 7 тестов
- `test_collision.rs` — 10 тестов
- `test_constant_imports.rs` — 7 тестов
- `test_controls.rs` — 20 тестов
- `test_controls_error_handling.rs` — 3 теста
- `test_controls_path_traversal.rs` — 5 тестов
- `test_controls_path_validation.rs` — 5 тестов
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
- `test_safety_architecture.rs` — 10 тестов
- `test_sanitize_optimization.rs` — 6 тестов
- `test_scoring_encapsulation.rs` — 5 тестов
- `test_security_fixes.rs` — 20 тестов
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
