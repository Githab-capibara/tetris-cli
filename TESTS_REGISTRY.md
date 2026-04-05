# 📋 TESTS REGISTRY — Tetris CLI

**Дата последней актуализации:** 5 апреля 2026
**Версия проекта:** 0.96.14

---

## 📊 СВОДКА

| Категория | Количество | Статус |
|-----------|-----------|--------|
| Модульные тесты (src/) | ~741 | ✅ 100% pass |
| Интеграционные тесты (tests/) | 55 | ✅ 100% pass |
| Doctests (runnable) | 63 | ✅ 100% pass |
| Doctests (ignored) | 117 | — ожидаемо |
| Бенчмарки (benches/) | 26 (8 групп) | ✅ |
| **ИТОГО** | **~859** | ✅ |

---

## 📁 СТРУКТУРА ТЕСТОВ

### Интеграционные тесты (`tests/`) — 3 файла, 57 тестов

| Файл | Тестов | Описание |
|------|--------|----------|
| `test_all_fixed_issues.rs` | 11 | Критические исправления (canvas, TOCTOU, HMAC, checked rotation) |
| `test_architecture_integrity.rs` | 21 | Архитектура: циклы зависимостей, ISP, трейты, инкапсуляция |
| `test_audit_2026_04_fixes.rs` | 25 | Аудит апрель 2026: HMAC, path traversal, валидация, sanitize |

### Модульные тесты (`src/tests/`) — 17 файлов

| Файл | Тематика |
|------|----------|
| `test_bag_system.rs` | Bag Generator (7-bag система) |
| `test_boundary_values.rs` | Граничные значения Score/Level/Lines |
| `test_collision.rs` | Коллизии фигур |
| `test_game_bounds_check.rs` | Проверка границ поля |
| `test_game_box_array.rs` | Массив блоков поля |
| `test_game_logic.rs` | Игровая логика |
| `test_game_movement.rs` | Движение фигур |
| `test_game_rotation.rs` | Вращение фигур |
| `test_game_score_overflow.rs` | Переполнение счёта |
| `test_hmac_safety.rs` | HMAC безопасность (12 уникальных тестов) |
| `test_integration.rs` | Базовая интеграция |
| `test_integration_extended.rs` | Расширенная интеграция |
| `test_io.rs` | Ввод-вывод |
| `test_io_errors.rs` | Ошибки IO |
| `test_io_utf8_handling.rs` | UTF-8 в IO |
| `test_physics.rs` | Физика игры |
| `test_safe_cast.rs` | Безопасный cast |
| `test_score_overflow_protection.rs` | Защита переполнения |
| `test_state_validation.rs` | Валидация состояния |

### Встроенные тесты (`#[cfg(test)]` в модулях)

| Модуль | Что тестирует |
|--------|--------------|
| `core/mod.rs` | Direction, RotationDirection, Position (13 тестов) |
| `game/board.rs` | GameBoard (4 теста) |
| `game/cache.rs` | RenderCache, StringCache (3 теста) |
| `game/components/board_state.rs` | BoardState |
| `game/cycle.rs` | Игровой цикл, FPS (6 тестов, 3 #[ignore]) |
| `game/events.rs` | EventDispatcher (2 теста) |
| `game/logic/collision.rs` | Коллизии (8 тестов) |
| `game/mode_trait.rs` | SprintMode, MarathonMode (4 теста) |
| `game/scoring/combo.rs` | Комбо-бонус |
| `game/scoring/lines.rs` | Подсчёт линий (2 теста) |
| `game/scoring/points.rs` | Hard/Soft drop, hold (6 тестов) |
| `game/state.rs` | GameState, GameMode (8 тестов) |
| `game/types.rs` | GameAction (6 тестов) |
| `game/view.rs` | GameView (1 тест) |
| `config/keys.rs` | HMAC ключи конфигурации (8 тестов) |
| `constants.rs` | Константы игры (3 теста) |
| `controls.rs` | ControlsConfig (4 теста) |
| `crypto/hmac.rs` | HMAC-SHA256 (14 тестов) |
| `crypto.rs` | hash(), generate_salt() (5 тестов) |
| `highscore/leaderboard.rs` | LeaderboardEntry, ThreadSafeLeaderboardEntry (10 тестов) |
| `highscore/leaderboard_repository.rs` | Репозиторий (2 теста) |
| `highscore/save_data.rs` | SaveData (3 теста) |
| `highscore/storage.rs` | Хранилище (2 теста) |
| `io/backend.rs` | TermionBackend (7 тестов, 4 #[ignore]) |
| `io/canvas.rs` | Canvas (3 теста) |
| `io/key_reader.rs` | KeyReader (2 теста) |
| `menu/mod.rs` | Меню (2 теста) |
| `tetromino/bag_generator.rs` | BagGenerator (3 теста) |
| `tetromino/constants.rs` | Константы фигур (6 тестов) |
| `tetromino/shape_type.rs` | ShapeType (2 теста) |
| `tetromino/tetromino_struct.rs` | Tetromino (3 теста) |
| `types.rs` | UpdateEndState (6 тестов) |
| `validation/name.rs` | Валидация имён (8 тестов) |
| `validation/path.rs` | Валидация путей (14 тестов) |
| `validation/service.rs` | ValidationService (4 теста) |
| `app/mod.rs` | Application (3 теста, 2 #[ignore]) |

### Бенчмарки (`benches/benchmarks.rs`) — 26 бенчмарков, 8 групп

| Группа | Бенчмарки |
|--------|-----------|
| `find_full_rows` | empty_field, one_full_line, multiple_full_lines |
| `check_rows` | clear_one_line, clear_multiple_lines, clear_tetris |
| `rotate` | rotate_t_clockwise, rotate_i_clockwise, rotate_o_noop |
| `save_tetromino` | save_t_center, save_i_center |
| `collision_detection` | check_down_empty, check_left_empty, check_right_empty |
| `wall_kick` | rotate_clockwise_empty, rotate_counterclockwise_empty |
| `sanitize_player_name` | empty_name, short_valid_name, long_name_truncated, name_with_invalid_chars, cyrillic_name, mixed_name |
| `string_caching` | cache_score, cache_level, cache_lines, cache_combo |

---

## 🧹 ИСТОРИЯ ОЧИСТКИ (5 апреля 2026)

### Удалено:
- `src/tests/test_error_propagation.rs` — тестировал Rust `?` оператор, не код проекта
- `src/tests/test_unwrap_to_expect.rs` — тестировал `Option::expect()`, не код проекта
- 18 дубликатов HMAC тестов из `src/crypto.rs` (оставлены в `crypto/hmac.rs`)
- 5 дубликатов из `src/tests/test_hmac_safety.rs` (сокращён до 12 уникальных)
- 8 дубликатов Position тестов из `src/game/types.rs` (оставлены в `core/mod.rs`)
- 6 дубликатов GameAction тестов из `src/types.rs` (оставлены в `game/types.rs`)
- 1 дубликат RenderCache теста из `src/game/render/cache.rs`
- 3 дубликата Level/Lines тестов из `src/tests/test_boundary_values.rs`
- 3 бесполезных теста из `tests/test_audit_2026_04_fixes.rs` (assert_true, doc_exists, signature_exists)

### Исправлено:
- Добавлены реальные assert'ы в `test_l4_simplified_exports`
- Добавлен `#[ignore]` к confy-зависимым тестам

### Итог:
- Было ~776 lib тестов → стало ~741 (убрано 35 дубликатов и бесполезных)
- Все тесты проходят: ✅ 100%
- Clippy: ✅ 0 предупреждений
