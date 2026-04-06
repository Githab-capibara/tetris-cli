# 📋 TESTS REGISTRY — Tetris CLI

**Дата последней актуализации:** 6 апреля 2026
**Версия проекта:** 0.96.14

---

## 📊 СВОДКА

| Категория | Количество | Статус |
|-----------|-----------|--------|
| Модульные тесты (src/) | ~670 | ✅ 100% pass |
| Интеграционные тесты (tests/) | 29 | ✅ 100% pass |
| Doctests (runnable) | 62 | ✅ 100% pass |
| Doctests (ignored) | 113 | — ожидаемо |
| Бенчмарки (benches/) | ~25 (8 групп) | ✅ |
| **ИТОГО (запускаемые)** | **685** | ✅ |
| Ignored | 16 | — ожидаемо |

---

## 📁 СТРУКТУРА ТЕСТОВ

### Интеграционные тесты (`tests/`) — 3 файла, 29 тестов

| Файл | Тестов | Описание |
|------|--------|----------|
| `test_all_fixed_issues.rs` | 2 | Исправленные issues, стресс-тест |
| `test_architecture_integrity.rs` | 2 | Поточная безопасность LeaderboardEntry |
| `test_audit_2026_04_fixes.rs` | 25 | Все 26 исправлений аудита |

### Модульные тесты (`src/tests/`) — 13 файлов

| Файл | Тестов | Описание |
|------|--------|----------|
| `test_integration.rs` | 20 | Полная инициализация, движение, вращение, производительность |
| `test_integration_extended.rs` | 22 | Взаимодействие компонентов, производительность |
| `test_collision.rs` | 22 | Коллизии со стенами, блоками, границы |
| `test_game_rotation.rs` | 50 | Вращение всех фигур, стены, wall kick |
| `test_bag_system.rs` | 27 | 7-bag рандомизация, Fisher-Yates, chi-square |
| `test_physics.rs` | 16 | Гравитация, hold, ghost piece, коллизии |
| `test_safe_cast.rs` | 14 | f32→u32 конвертация, NaN, Infinity, overflow |
| `test_score_overflow_protection.rs` | 12 | Saturating arithmetic, экстремальные значения |
| `test_boundary_values.rs` | 35 | Границы Score, Level, LinesCount, Combo |
| `test_hmac_safety.rs` | 12 | HMAC ключи, Unicode, binary data, stress |
| `test_io_errors.rs` | 3 | KeyReader panic, Drop, InputReader trait |
| `test_io_utf8_handling.rs` | 5 | UTF-8 multibyte, ASCII, invalid sequences |
| `test_game_bounds_check.rs` | 3 | f32→u64 конвертация, negative values |

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

## ✅ КРИТЕРИИ КАЧЕСТВА

- [x] Все тесты проходят (100% pass rate)
- [x] Нет дублирующихся тестов
- [x] Нет тестов для удалённого кода
- [x] Нет пустых тестов (без assert)
- [x] `cargo fmt -- --check` — чисто
- [x] `cargo clippy --tests -- -D warnings` — чисто
- [x] `cargo build --release` — чисто
