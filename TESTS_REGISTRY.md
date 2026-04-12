# 📋 TESTS REGISTRY — Tetris CLI

**Дата последней актуализации:** 12 апреля 2026 (очистка тестовой базы — раунд 9)
**Версия проекта:** 0.96.14

---

## 📊 СВОДКА

| Категория | Количество | Статус |
|-----------|-----------|--------|
| Модульные тесты (src/) | ~537 | ✅ 100% pass |
| Интеграционные тесты (tests/) | 1 | ✅ 100% pass |
| Doctests (runnable) | 64 | ✅ 100% pass |
| Doctests (ignored) | 94 | — ожидаемо |
| Бенчмарки (benches/) | ~25 (8 групп) | ✅ (требуют `--features bench`) |
| **ИТОГО (запускаемые)** | **~602** | ✅ |
| Ignored | 8 | — ожидаемо |

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-12, раунд 9)

### Удалённые тавтологические тесты (2 шт):
- `test_shuffle_does_not_lose_pieces` (test_bag_system.rs) — проверял счётчик цикла (total_pieces == 100), а не логику BagGenerator
- `test_saturating_add_comprehensive` (test_score_overflow_protection.rs) — тестировал стандартную библиотеку Rust (u128::saturating_add), а не код проекта

### Удалённые дубликаты между файлами (4 шт):
- `test_game_state_max_score` (test_boundary_values.rs) — дублировал test_game_state_score_overflow из test_score_overflow_protection.rs
- `test_game_state_score_overflow_protection` (test_boundary_values.rs) — дублировал test_game_state_score_overflow из test_score_overflow_protection.rs
- `test_curr_and_next_shapes_different` (test_integration_extended.rs) — дублировал проверки из test_full_game_initialization (test_integration.rs)
- `test_validation_in_game_state_context` (test_state_validation.rs) — агрегатор-дубликат, повторял проверки NaN/Infinity и валидных значений из отдельных тестов

### Удалённые бесполезные тесты (1 шт):
- `test_move_between_obstacles` (test_game_movement.rs) — проверял тривиальное условие (can_move в пустом поле всегда true), тавтология с `||`

### Исправленные тесты (5 шт):
- `test_sprint_timer` (test_game_logic.rs) — заменена тривиальная проверка `elapsed >= 0.0` на содержательную `elapsed < 1.0`
- `test_sprint_game_initialization` (test_integration.rs) — **удалён `thread::sleep(50ms)`** (flaky тест), заменён на детерминированную проверку инициализации таймера
- `test_collision_array_bounds` (test_collision.rs) — заменена тавтология `!can_left || !can_down` на конкретную проверку `!can_left`
- `test_movement_after_rotation` (test_game_movement.rs) — улучшен комментарий, добавлена явная проверка всех 3 направлений
- `test_movement_after_full_rotation_cycle` (test_game_movement.rs) — заменена слабая проверка `can_move(Left) || can_move(Right)` на проверку позиции и `can_move(Down)`

### Удалённые println из тестов (14 шт):
- test_safe_cast.rs — удалены все 14 println! с галочками, засорявшие вывод тестов

### Итого:
- Удалено: 7 тестов (тавтологии, дубликаты, бесполезные)
- Исправлено: 5 тестов (усилены assert, убран flaky sleep)
- Очищено: 14 println! из test_safe_cast.rs
- Результат: 537 тестов проходят (100% pass rate)
- Чистое сокращение: ~544 тестов → ~537 тестов (без потери покрытия!)
- `cargo clippy --tests -- -D warnings` — чисто
- `cargo fmt --check` — чисто

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-12, раунд 8)

### Удалённые дублирующиеся тесты (2 шт):
- `test_tetromino_rotate_clockwise` (test_game_logic.rs) — полный дубликат `test_all_shapes_rotate_clockwise` в test_game_rotation.rs
- `test_tetromino_rotate_counter_clockwise` (test_game_logic.rs) — полный дубликат `test_all_shapes_rotate_counter_clockwise` в test_game_rotation.rs

### Удалённые устаревшие TODO комментарии (4 файла):
- test_boundary_values.rs: удалён "TODO: рассмотреть перенос в tests/ (PROB-120)"
- test_score_overflow_protection.rs: удалён аналогичный TODO
- test_state_validation.rs: удалён аналогичный TODO
- test_hmac_safety.rs: удалён аналогичный TODO

### Очищены неиспользуемые импорты:
- test_game_logic.rs: удалены неиспользуемые импорты ShapeType, Tetromino, RotationDirection после удаления дублирующихся тестов вращения

### Итого:
- Удалено: 2 дублирующихся теста вращения
- Очищено: 4 устаревших TODO комментария
- Оптимизировано: импорты в test_game_logic.rs
- Результат: 544 теста проходят (100% pass rate)
- Чистое сокращение: ~616 тестов → ~544 тестов (без потери покрытия!)

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-09, раунд 6)

### Критические исправления:
- **2 теста #[should_panic] с debug_assert** — некорректны в release mode
  - `test_level_zero_becomes_minimum` (test_boundary_values.rs) → заменён на `#[cfg(debug_assertions)]`
  - `test_level_with_value_minimum` (types.rs) → заменён на `#[cfg(debug_assertions)]`

### Удалённые дублирующиеся тесты (3 шт):
- `test_soft_drop_different_pieces` (test_game_movement.rs) — полный дубликат test_soft_drop_basic
- `test_collision_landing_on_piece` (test_collision.rs) — полностью покрыт test_collision_all_shapes_floor
- `test_validation_no_panic_on_invalid_values` (test_state_validation.rs) — дублирует отдельные тесты NaN/Infinity

### Консолидация дублирующихся тестов (80+ тестов → ~25 параметризированных):

#### test_game_rotation.rs: 50 тестов → 8 параметризированных
- `test_all_shapes_rotate_clockwise` — заменяет 7 тестов вращения по часовой
- `test_all_shapes_rotate_counter_clockwise` — заменяет 7 тестов вращения против часовой
- `test_all_shapes_full_rotation_cycle` — заменяет 7 тестов полного цикла вращения
- `test_rotation_at_walls` — заменяет 8 тестов вращения у стен
- `test_rotation_above_piece_all_shapes` — заменяет 7 тестов вращения над фигурой
- `test_rotation_with_collision_all_shapes` — заменяет 8 тестов вращения с коллизиями
- `test_spin_rotation_all_shapes` — заменяет 4 теста spin-вращения
- `test_special_rotation_s_and_z` — заменяет 2 теста специального вращения

#### test_game_movement.rs: 7 тестов → 4 параметризированных
- Soft drop: 5 тестов → 3 (test_soft_drop_initial, test_soft_drop_to_floor, test_soft_drop_increases_y)
- Movement after rotation: 2 теста → 1 (параметризированный по Direction)

#### test_collision.rs: 6 тестов → 4 параметризированных
- Side walls: 2 теста → 1 (test_collision_side_walls)
- Down at walls: 2 теста → 1 (test_collision_down_at_side_walls)
- Away from wall: 2 теста → 1 (test_collision_away_from_wall)
- All shapes: 2 теста → 1 (test_collision_all_shapes_walls_and_floor)

#### test_boundary_values.rs: 6 тестов → 3 параметризированных
- Score saturating: 2 теста → 1 (test_score_saturating_operations)
- Lines count saturating: 2 теста → 1 (test_lines_count_saturating)
- Leaderboard queries: 2 теста → 1 (test_leaderboard_query_methods)

#### test_score_overflow_protection.rs: 6 тестов → 3 параметризированных
- Extreme scoring: 2 теста → 1 (test_extreme_scoring_parameters)
- Stress score overflow: 2 теста → 1 (test_stress_score_overflow_protection)
- Saturating add: 2 теста → 1 (test_saturating_add_comprehensive)

#### test_state_validation.rs: 2 теста → 1 параметризированный
- Boundary values: 2 теста → 1 (test_boundary_values_fall_speed_and_land_timer)

### Улучшения тестов:
- **test_sprint_timer** — удалён `thread::sleep(100ms)` (хрупкий тест)
- **test_all_medium_fixes_integration** — удалён пустой `Canvas::default(); drop(canvas)` без assert

### Итого:
- Удалено: ~120 дублирующихся тестов
- Консолидировано: 80+ тестов → ~25 параметризированных
- Исправлено: 2 критических теста с debug_assert
- Улучшено: 2 теста (убраны хрупкие/пустые проверки)
- Чистое сокращение: ~675 тестов → ~616 тестов (без потери покрытия!)

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-09, раунд 5)

### Исправленные бесполезные тесты (4 теста):
- `test_all_tetromino_rotate` (test_game_logic.rs) — добавлен `assert_ne` для проверки вращения фигур
- `test_collision_rotation_o_piece` (test_collision.rs) — добавлен `catch_unwind` для проверки отсутствия паники
- `test_path_validator_symlink_check_exists` (test_edge_cases.rs) — добавлен `assert!(result.is_ok())`
- `test_safe_f32_to_u32_no_panic` (test_safe_cast.rs) — добавлена проверка валидности возвращаемых значений

### Результат:
- Все 4 бесполезных теста исправлены (добавлены реальные проверки)
- Все 696 тестов проходят (100% pass rate)
- Дубликаты не обнаружены (критические)
- Устаревшие тесты не обнаружены

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-08, раунд 4)

### Удалённые бесполезные тесты (24 теста):
- `test_io.rs` (4 теста) — проверка констант SHAPE_STR, SHAPE_WIDTH, DISP_WIDTH, terminal size
- `test_audit_2026_04_fixes.rs` (5 тестов): H1 (проверка stdlib to_string), H7 (проверка константы), M3 (Canvas drop без assert), L1 (ASCII константы), L4 (импорты)
- `test_module_isolation.rs` (2 теста): constants, scoring — проверка констант
- `test_boundary_values.rs` (2 теста): initial_values, value_correctness — тривиальные getter/setter
- `test_edge_cases.rs` (2 теста): saturating_add (проверка stdlib), leaderboard_default_is_empty (дубликат)
- `test_game_logic.rs` (1 тест): tetromino_full_rotation_cycle — дублирует test_game_rotation.rs
- `test_panic_handling.rs` (1 тест исправлен): bag_generator_next_shape_no_panic — добавлен assert
- `test_io_errors.rs` (2 теста удалены) — compile-time проверки
- `test_io_utf8_handling.rs` (2 теста удалены) — всегда проходят

### Исправленные warnings:
- Unused variables/mut в figure_manager.rs, update.rs, wall_kick.rs, path.rs
- Float cmp warnings в test_state_validation.rs (assert_eq -> assert_f32_eq/assert_f64_eq)
- Float cmp в input.rs (assert_eq -> assert с epsilon)

### Удалены объявления модулей из lib.rs:
- mod test_io, mod test_io_errors, mod test_io_utf8_handling

### Исправленные тесты:
- `test_leaderboard_not_send_sync` (src/highscore/leaderboard.rs) — был пустым, добавлены реальные проверки
- `test_rotation_near_left_wall` (src/tests/test_physics.rs) — была тавтология `assert!(x || !x)`, заменена на реальную проверку вращения у стены

### Удалённый мёртвый код:
- `tests/common/mod.rs` — подключался в 3 файлах но ни разу не использовался
- `mod common;` удалён из test_architecture_integrity.rs, test_all_fixed_issues.rs, test_audit_2026_04_fixes.rs

### Безопасность тестов:
- 5x `.unwrap()` заменены на `.expect()` с описательными сообщениями:
  - test_state_validation.rs: 2 случая в catch_unwind
  - test_boundary_values.rs: 1 случай в saturating_add тесте
  - test_crypto_security.rs: 1 случай в HMAC tampered тесте
  - test_hmac_safety.rs: 1 случай в timing safe тесте

### Удалённые unused imports:
- `Direction` из test_game_logic.rs (стал неиспользуемым после удаления дубликатов)
- `Arc` из test_edge_cases.rs (стал неиспользуемым после удаления дубликатов)

### Итого изменено: удалено 19 тестов + мёртвый common модуль, исправлено 2 пустых/тавтологических теста, улучшено 5 unwrap → expect
- `test_movement_in_corner` — дублирует test_collision_bottom_left_corner
- `test_move_above_fixed_piece`, `test_movement_blocked_by_obstacle_left`, `test_movement_blocked_by_obstacle_right` — бесполезные тесты без реальной проверки
- `test_collision_not_beyond_left_boundary`, `test_collision_not_beyond_right_boundary` — дублируют test_collision_left/right_wall
- `test_collision_new_above_fixed`, `test_collision_triggers_correctly` — дублируют другие тесты столкновений
- `test_piece_stays_within_bounds` — дублирует test_movement_in_empty_field
- `test_leaderboard_validates_entries` — дублирует test_game_state_leaderboard_interaction
- `test_hold_piece_swap` — дублирует hold тесты из test_physics.rs

### Исправления:
- Удалена ссылка на `mod test_game_box_array` из `src/lib.rs`

### Итого удалено: ~20 тестов (676 → 656)

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-06)

### Удалённые тесты (дубликаты/бесполезные):
- `test_termion_backend_creation`, `test_termion_backend_with_raw`, `test_termion_io_backend_creation` — тавтологии `is_ok() || is_err()`
- `test_io_module_exports` — пустой тест без assert
- `test_hmac_sha256_unreachable_pattern` — без assert
- `test_performance_find_full_rows`, `test_performance_save_tetromino`, `test_performance_check_collision_direction` — дублируют бенчмарки
- `test_performance_game_state_creation`, `test_performance_bag_generator`, `test_performance_collision_detection`, `test_performance_rotation`, `test_performance_leaderboard`, `test_performance_controls_validation`, `test_performance_save_data_hashing` — дублируют бенчмарки
- `test_fast_gamestate_creation`, `test_fast_tetromino_creation`, `test_fast_piece_rotation`, `test_fast_collision_check`, `test_fast_score_save`, `test_overall_system_performance` — дублируют бенчмарки
- `test_utf8_multibyte_returns_none`, `test_invalid_utf8_no_panic`, `test_ascii_characters_read_correctly`, `test_utf8_then_ascii_sequence`, `test_utf8_range_handling` — пустые тесты без assert
- `test_safe_f32_to_u32_no_panic` — пустой тест

### Исправленные тесты (добавлены assert):
- `test_get_shape_block_coords_valid_indices` — добавлена проверка диапазона координат
- `test_get_shape_color_valid_indices` — добавлена проверка что цвет не null
- `test_rotation_collision_interaction` — добавлена assert на вращение в пустом поле
- `test_collision_array_bounds` — добавлена assert на блокировку движения
- `test_rotation_near_left_wall` — добавлена assert на bool возврат

### Оптимизация:
- Performance тесты в unit-тестах удалены — покрыты бенчмарками через criterion
- Файл `test_io_utf8_handling.rs` сокращён с 5 до 2 содержательных тестов

### Добавленные файлы:
- `test_collision.rs` (22 теста) — коллизии со стенами, блоками, границы поля
- `test_io_utf8_handling.rs` (2 теста) — UTF-8 multibyte, валидные последовательности
- `tests/common/mod.rs` — хелперы для интеграционных тестов

---

## 📁 СТРУКТУРА ТЕСТОВ

### Интеграционные тесты (`tests/`) — 4 файла, 30 тестов (включая common/mod.rs)

| Файл | Тестов | Описание |
|------|--------|----------|
| `common/mod.rs` | 0 | Хелперы для интеграционных тестов |
| `test_all_fixed_issues.rs` | 2 | Исправленные issues, стресс-тест |
| `test_architecture_integrity.rs` | 2 | Поточочная безопасность LeaderboardEntry |
| `test_audit_2026_04_fixes.rs` | 25 | Все 26 исправлений аудита |

### Модульные тесты (`src/tests/`) — 22 файла

| Файл | Тестов | Описание |
|------|--------|----------|
| `test_bag_system.rs` | 24 | 7-bag рандомизация, Fisher-Yates, распределение |
| `test_boundary_values.rs` | 32 | Границы Score, Level, LinesCount, Combo |
| `test_collision.rs` | 16 | Коллизии со стенами, блоками, границы |
| `test_edge_cases.rs` | 15 | Concurrent access, overflow, HMAC, path traversal |
| `test_game_bounds_check.rs` | 3 | f32→u64 конвертация, negative values |
| `test_game_logic.rs` | 16 | Game logic, столкновения, вращение, очки |
| `test_game_movement.rs` | 18 | Movement, soft/hard drop, rotation |
| `test_game_rotation.rs` | 50 | Вращение всех фигур, стены, wall kick |
| `test_hmac_safety.rs` | 11 | HMAC ключи, Unicode, binary data, stress |
| `test_integration.rs` | 13 | Полная инициализация, движение, вращение |
| `test_integration_extended.rs` | 10 | Tetromino из bag, game modes, leaderboard, hold |
| `test_io.rs` | 4 | Константы дисплея, SHAPE_STR, DISP_WIDTH |
| `test_io_errors.rs` | 2 | KeyReader no panic, InputReader trait |
| `test_io_utf8_handling.rs` | 2 | UTF-8 multibyte, валидные последовательности |
| `test_module_isolation.rs` | 8 | Модульная изоляция компонентов |
| `test_panic_handling.rs` | 8 | No-panic тесты |
| `test_physics.rs` | 11 | Гравитация, hold, ghost piece, коллизии |
| `test_safe_cast.rs` | 14 | f32→u32 конвертация, NaN, Infinity, overflow |
| `test_score_overflow_protection.rs` | 12 | Saturating arithmetic, экстремальные значения |
| `test_state_validation.rs` | 15 | Валидация fall_speed и land_timer |

### Встроенные тесты (`#[cfg(test)]` в модулях)

| Модуль | Тестов | Описание |
|--------|--------|----------|
| `crypto/hmac.rs` | 25 | HMAC sign/verify, determinism, key sensitivity |
| `validation/path.rs` | 30 | Sanitize, traversal, hidden files |
| `validation/name.rs` | 25 | Sanitize player names, Cyrillic, length limits |
| `game/time.rs` | 15 | FPS counter, frame timer |
| `game/stats.rs` | 4 | GameStats creation, increment |
| `core/mod.rs` | 17 | Safe cast f32→u64, edge cases |
| `errors.rs` | 7 | GameError variants, Display |
| `config/keys.rs` | 8 | HMAC key validation, min length |
| `game/state.rs` | 10 | State validation, fall_speed, set_score |
| `game/cycle.rs` | 6 (3 ignored) | FPS regulation, input result handling |
| `game/scoring/points.rs` | ~25 | Hard/soft drop, landing, combo, overflow |
| `game/scoring/lines.rs` | ~10 | Line clear, animation, combo |
| `game/scoring/combo.rs` | ~4 | Combo counter, bonus calculation |
| `game/logic/collision.rs` | ~10 | Position validation, boundaries |
| `game/logic/rotation.rs` | Встроены | Rotation logic tests |
| `game/logic/wall_kick.rs` | Встроены | Wall kick tests |
| `game/logic/update.rs` | ~5 | Update loop, land timer |
| `game/logic/input.rs` | 1 (1 should_panic) | Movement input handling |
| `game/components/figure_manager.rs` | Встроены | Figure management |
| `game/components/animation_state.rs` | Встроены | Animation state |
| `highscore/leaderboard.rs` | ~30 | Entry creation, validation, scoring, thread safety |
| `highscore/storage.rs` | 3 | Storage add, validator signature |
| `highscore/save_data.rs` | ~10 | Save/load, tamper protection |
| `highscore/leaderboard_repository.rs` | ~5 | Repository creation, save, clear |
| `tetromino/constants.rs` | 6 | Shape coords, colors |
| `tetromino/tetromino_struct.rs` | 3 | Tetromino creation, position |
| `tetromino/bag_generator.rs` | 2 | Bag creation, next shape |
| `validation/service.rs` | 4 | f32 finite validation |
| `crypto/validator.rs` | 2 | HMAC re-export tests |
| `game/types.rs` | ~15 | Score, Level, LinesCount, GameAction |
| `game/board.rs` | Встроены | Board operations |
| `game/view.rs` | Встроены | View formatting |
| `game/rules.rs` | Встроены | Rules validation |
| `game/scoreboard.rs` | Встроены | Scoreboard operations |

### Бенчмарки (`benches/benchmarks.rs`) — 8 групп, ~25 бенчмарков

| Группа | Бенчмарков | Описание |
|--------|-----------|----------|
| `find_full_rows` | 3 | empty, one line, multiple lines |
| `check_rows` | 3 | clear one, multiple, tetris |
| `rotate` | 3 | T cw, I cw, O noop |
| `save_tetromino` | 2 | T center, I center |
| `collision_detection` | 3 | down, left, right empty |
| `wall_kick` | 2 | rotate cw, ccw empty |
| `sanitize_player_name` | 7 | empty, short, long, invalid, cyrillic, mixed |
| `string_caching` | 4 | score, level, lines, combo |

---

## ⚠️ IGNORED ТЕСТЫ (16)

| Тест | Причина |
|------|---------|
| `test_maintain_fps_*` (3) | Flaky: depends on OS timing |
| `test_performance_*` (3) | Performance threshold may fail on CI |
| `test_application_*` (2) | Depends on filesystem/terminal |
| `test_initialize_terminal_error_handling` | Requires terminal size check |
| `test_run_menu_loop_executes_without_error` | Infinite blocking loop |
| `test_update_continue` | Requires terminal input |
| `test_termion_backend_creation` (3) | Requires raw terminal mode |
| `test_rotation_at_wall_and_movement` | Known wall kick edge case |
| `test_gamestate_can_save_score` | Depends on confy filesystem |
| `test_savedata_loads_score` | Depends on confy filesystem |

---

## 🗑️ УДАЛЁННЫЕ ТЕСТЫ (очистка 7 апреля 2026)

| Файл/Тест | Причина |
|-----------|---------|
| `test_game_box_array.rs` (весь файл, 3 теста) | Устаревшая документация (описывает Box но используется статический массив) |
| `test_direction_from_core` (src/types.rs) | Пустой тест, только `let _ = ...` без assert |
| `test_c2_key_reader_handles_ascii_correctly` (test_audit) | Пустой тест на non-Unix |
| `test_piece_position_at_left_boundary` | Дублирует `test_collision_left_wall` |
| `test_piece_position_at_right_boundary` | Дублирует `test_collision_right_wall` |
| `test_i_piece_at_left_boundary` | Дублирует `test_collision_left_wall` |
| `test_i_piece_at_right_boundary` | Дублирует `test_collision_right_wall` |
| `test_movement_in_corner` | Дублирует `test_collision_bottom_left_corner` |
| `test_move_above_fixed_piece` | Бесполезный тест без реальной проверки |
| `test_movement_blocked_by_obstacle_left` | Бесполезный тест без реальной проверки |
| `test_movement_blocked_by_obstacle_right` | Бесполезный тест без реальной проверки |
| `test_collision_not_beyond_left_boundary` | Дублирует `test_collision_left_wall` |
| `test_collision_not_beyond_right_boundary` | Дублирует `test_collision_right_wall` |
| `test_collision_new_above_fixed` | Дублирует другие тесты столкновений |
| `test_collision_triggers_correctly` | Дублирует `test_collision_left_wall` |
| `test_piece_stays_within_bounds` | Дублирует `test_movement_in_empty_field` |
| `test_leaderboard_validates_entries` | Дублирует `test_game_state_leaderboard_interaction` |
| `test_hold_piece_swap` | Дублирует hold тесты из `test_physics.rs` |

**Итого удалено: ~20 тестов** (676 → 656)

---

## 🗑️ УДАЛЁННЫЕ ТЕСТЫ (очистка 6 апреля 2026)

| Файл/Тест | Причина |
|-----------|---------|
| `test_game_score_overflow.rs` (весь файл, 12 тестов) | Полностью дублирует `test_score_overflow_protection.rs` |
| `test_integration_extended.rs` (22 теста) | Дублируют `test_integration.rs`, `test_game_logic.rs`, `test_game_movement.rs` |
| `test_physics.rs` (4 bag-теста) | Дублируют `test_bag_system.rs` |
| `test_io_errors.rs` (5 тестов) | Проверяли диапазоны байт вручную, не поведение KeyReader |
| `test_game_action_variants` | Без assert, только создание enum вариантов |
| `test_repository_exists` | Без assert |
| `test_repository_save_result_returns_ok` | Тавтология `is_ok() \|\| is_err()` |
| `test_save_value_unwritable_directory` | `let _ = result` без проверок |
| `test_fix_m6_handle_landing_explicit_return` | `let _ = result` без проверок |
| `assert_not_send_sync` helper | Dead code |

**Итого удалено: ~45 тестов** (730 → 685)

---

## 🧹 ОЧИСТКА ТЕСТОВОЙ БАЗЫ (2026-04-11, раунд 7)

### Исправленные тесты без ассертов (1 тест):
- `test_no_panic_at_extreme_values` (test_score_overflow_protection.rs) — добавлены явные assert для проверки что счёт и combo остаются в разумных границах после экстремальных начислений

### Обновлённая документация в заголовках файлов (6 файлов):
- `test_collision.rs` — обновлено с "50 тестов" до "9 параметризированных тестов"
- `test_game_movement.rs` — обновлено с "50 тестов" до "13 параметризированных тестов"
- `test_physics.rs` — обновлено с "20 тестов" до "4 параметризированных тестов"
- `test_game_logic.rs` — обновлено с "30 тестов" до "8 параметризированных тестов"
- `test_bag_system.rs` — обновлено с "30 тестов" до "12 параметризированных тестов"
- `test_integration.rs` — обновлено с "20 тестов" до "6 интеграционных тестов"
- Удалены устаревшие `TODO: рассмотреть перенос в tests/ (PROB-120)` комментарии

### Итого:
- Исправлено: 1 тест без ассертов
- Обновлена документация: 6 файлов
- Все 644 теста проходят (100% pass rate)
- `cargo clippy --tests -- -D warnings` — чисто
- `cargo fmt --check` — чисто

---

## ✅ КРИТЕРИИ КАЧЕСТВА

- [x] Все тесты проходят (100% pass rate)
- [x] Нет дублирующихся тестов
- [x] Нет тестов для удалённого кода
- [x] Нет пустых тестов (без assert)
- [x] `cargo fmt --check` — чисто
- [x] `cargo clippy --tests -- -D warnings` — чисто
- [x] `cargo build --release` — чисто
