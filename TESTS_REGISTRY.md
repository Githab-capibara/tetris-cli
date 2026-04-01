# 📋 TESTS REGISTRY - Tetris CLI

**Дата последней актуализации:** 1 апреля 2026 г. (очистка тестовой базы — удалены 8 assert!(true))
**Версия проекта:** 23.96.32+ (аудит 2026-04)
**Всего тестов:** ~1634 тестов (проходят 100%)
**Всего файлов тестов:** 130

---

## 📊 ТЕКУЩАЯ СТАТИСТИКА

### Структура тестовой базы:

**Интеграционные тесты (`/tests/`):**
- 9 файлов
- 198 тестов
- Категории: architecture, audit, fixes, verification

**Unit тесты (`/src/tests/`):**
- 86 файлов
- 1096 тестов
- Категории: game, tetromino, controls, highscore, crypto, validation, io, architecture, physics, scoring

**Встроенные тесты (`#[cfg(test)]`):**
- 33 файла
- 314 тестов
- Категории: все модули проекта

**Бенчмарки (`/benches/`):**
- 1 файл
- 26 групп бенчмарков

**Фикстуры:**
- 1 файл (test_fixtures.rs)
- 26 функций-фикстур

### Очистка тестовой базы (1 апреля 2026):

**Удалённые проблемы:**
- 8 бесполезных assert!(true, ...) из test_audit_2026_04_fixes.rs
- Удалён #![allow(deprecated)] из test_all_fixed_issues.rs

**Найденные дубликаты (требуют удаления):**
- 8 групп дублирующихся тестов (16 тестов)
- Тесты на HMAC: 15+ тестов (сократить до 5-7)
- Тесты на Position: 12 тестов (сократить до 4-5)
- Тесты на Time: 14 тестов (сократить до 6-8)

**Потенциал сокращения:** ~25-30 тестов без потери покрытия

#### Удалено файлов (2):
- `test_mod.rs` — пустой файл, 0 тестов
- `test_tetromino_dir_down.rs` — тесты с удалённым NoRotation

#### Удалено дубликатов тестов:

**Коллизии со стенами (6 тестов → 2):**
- Удалены: `test_collision_left_boundary`, `test_collision_right_boundary` (test_game_logic.rs)
- Удалены: `test_collision_with_left_wall`, `test_collision_with_right_wall` (test_physics.rs)
- Удалены: `test_move_to_left_boundary_blocked`, `test_move_to_right_boundary_blocked` (test_game_movement.rs)
- Оставлены: `test_collision_left_wall`, `test_collision_right_wall` (test_collision.rs)

**Коллизии с полом (4 теста → 1):**
- Удалены: `test_collision_reaches_floor`, `test_collision_not_through_floor` (test_collision.rs)
- Удалены: `test_piece_reaching_floor` (test_physics.rs)
- Удалены: `test_move_to_bottom_boundary_blocked` (test_game_movement.rs)
- Оставлен: `test_collision_floor` (test_game_logic.rs)

**Призрачная фигура (3 теста → 1):**
- Удалены: `test_collision_ghost_piece` (test_collision.rs)
- Удалены: `test_ghost_piece_boundary` (test_game_logic.rs)
- Удалены: `test_ghost_piece_floor_detection` (test_physics.rs)
- Оставлен: `test_ghost_piece_position` (test_physics.rs)

**Движение фигур (14 тестов → 1):**
- Удалены: `test_t_piece_move_left`, `test_t_piece_move_right`, ..., `test_i_piece_move_left`, `test_i_piece_move_right`
- Добавлен: `test_all_pieces_move_left_right` (параметризированный тест)

**Hard Drop (6 тестов → 1):**
- Удалены: `test_hard_drop_basic`, `test_hard_drop_instant_stop`, `test_hard_drop_final_position`, `test_hard_drop_different_heights`, `test_hard_drop_o_piece`
- Добавлен: `test_hard_drop_comprehensive`

#### Удалено бесполезных тестов:

**Тривиальные ассерты (>= 0):**
- `test_save_error_handling` (test_highscore_error_handling.rs)
- `test_must_use_get_elapsed_time` (test_must_use_attributes.rs)
- `test_min_game_time` (test_edge_cases_stress.rs)
- `test_sprint_mode_timer` (test_game_modes_detailed.rs)
- `test_game_has_timer` (test_integration_extended.rs)

**Тесты-документации:**
- `test_critical_canvas_graceful_degradation` (test_architecture_all.rs)
- `test_critical_controls_toctou_protection` (test_architecture_all.rs)
- `test_architecture_no_cyclic_dependencies_core` (test_architecture_all.rs)
- `test_code_checked_neg_rotation` (test_architecture_all.rs)

**Тесты без ассертов:**
- `test_get_current_time_ms_no_panic` (test_time_safety.rs)
- `test_collision_assertions_meaningful` (test_all_fixes_integration.rs)
- `test_unused_import_fixed` (test_fixes.rs)

**Сломанные тесты:**
- `test_directory_unavailable_error_handling` (test_highscore_config_path.rs) — использовал ConfigError

#### Итоговая статистика очистки:
- Удалено файлов: **2**
- Удалено тестов: **~40**
- Объединено тестов: **20 → 2**
- Изменено файлов: **12**
- Новое количество тестов: **~1368** (было ~1306)
- Новое количество файлов тестов: **94** (было 93)

---

### Аудит и исправления (1 апреля 2026 — версия 23.96.31+):

**Проведён полный аудит кода:**
- Выявлено проблем: 19
- Исправлено критических проблем: 3 (C1-C3)
- Исправлено проблем высокого приоритета: 5 (H1-H5)
- Исправлено проблем среднего приоритета: 6 (M1-M6)
- Исправлено проблем низкого приоритета: 5 (L1-L5)

**Ключевые исправления:**
- **Упрощена обработка ошибок** в `Application::initialize_terminal()` — прямой `?` возврат
- **Реализована потокобезопасность** `LeaderboardEntry` через `Arc<Mutex<>>`
- **Упрощена проверка переполнения счёта** — `saturating_add()` без избыточных проверок
- **Переименована функция** `has_collision()` → `is_position_valid()` с прямой логикой
- **Удалён `.max(0.0)`** из `set_land_timer()` — валидация гарантирует корректность
- **Заменён `[bool; 256]`** на `HashSet<u8>` в `ControlsConfig::validate()`
- **Удалён `catch_unwind`** из Drop реализаций в `io.rs`
- **Удалён мёртвый код** в `handle_movement_input()` и других функциях

**Новые тесты:**
- Добавлен тестовый файл: `test_audit_fixes_all.rs` (20 тестов для всех исправлений аудита)
- Исправлен тест `test_state_validation_integration` — проверка строгой валидации

**Статистика изменений:**
- Добавлено тестов: 20 (test_audit_fixes_all.rs)
- Исправлено тестов: 1 (test_state_validation_integration)
- Все тесты проходят: ✅ (100% pass rate)

### Очистка тестовой базы (31 марта 2026 — версия 23.96.30+):

**Удалено дубликатов:**
- test_direction_down.rs (дублировал test_tetromino_dir_down.rs)
- test_architecture_refactoring.rs (дублировал test_architecture_improvements.rs)

**Переименовано файлов (для консистентности):**
- fixtures.rs → test_fixtures.rs
- scoring_state.rs → test_scoring_state.rs
- io_drop.rs → test_io_drop.rs
- leaderboard_toctou.rs → test_leaderboard_toctou.rs
- macros.rs → test_macros.rs
- mod.rs → test_mod.rs

**Исправлено ошибок компиляции (9):**
- Доступ к приватным полям Tetromino (fg, shape, coords, pos)
- Неиспользуемые импорты
- Unused return value от Leaderboard::add_score()

**Обновлено deprecated методов (12):**
- name() → name_safe()
- score() → score_safe()
- is_valid() → is_valid_safe()

**Удалено бесполезных тестов (12):**
- assert!(true) тесты
- assert!(x || !x) тесты (всегда проходят)
- Информационные тесты без реальных проверок

**Статистика изменений:**
- Удалено строк: 1947
- Добавлено строк: 164
- Чистое изменение: -1783 строки

---

## 📊 СТРУКТУРА ТЕСТОВ

### tests/ (корневая директория): 13 файлов
- test_41_fixed_issues.rs — 42 теста на исправления аудита
- test_all_fixed_issues.rs — 13 тестов критических исправлений
- test_architecture_all.rs — 30 тестов архитектурных проблем
- test_architecture_components.rs — 7 тестов компонентов
- test_architecture_improvements.rs — 16 тестов улучшений
- test_architecture_integrity.rs — 21 тест целостности
- test_audit_2026_04_fixes.rs — 26 тестов исправлений аудита (апрель 2026)
- test_audit_fixes.rs — 25 тестов исправлений аудита
- test_audit_fixes_all.rs — 20 тестов всех исправлений
- test_audit_fixes_comprehensive.rs — комплексные тесты исправлений
- test_audit_fixes_verification.rs — 21 тест верификации
- test_fixes_verification.rs — 8 интеграционных тестов

### src/tests/: 88 файлов

#### Архитектурные тесты (13 файлов)
- test_architecture.rs — базовые архитектурные тесты
- test_architecture_boundaries.rs — границы модулей
- test_architecture_components.rs — компоненты GameState
- test_architecture_coupling.rs — связанность модулей
- test_architecture_cycles.rs — циклические зависимости
- test_architecture_fixes.rs — исправления архитектуры
- test_architecture_fixes_new.rs — новые исправления
- test_architecture_integrity.rs — целостность архитектуры
- test_architecture_integrity_new.rs — новые тесты целостности
- test_architecture_isp.rs — Interface Segregation Principle
- test_architecture_separation.rs — разделение ответственности
- test_architecture_traits.rs — архитектурные трейты
- test_architecture_validation.rs — валидация архитектуры

#### Тесты безопасности (8 файлов)
- test_application_error_handling.rs — обработка ошибок Application
- test_cfg_attr_dead_code.rs — dead_code атрибуты
- test_clippy_fixes.rs — исправления Clippy
- test_controls_path_traversal.rs — защита от path traversal
- test_controls_path_validation.rs — валидация путей
- test_controls_toctou.rs — TOCTOU защита controls
- test_hmac_safety.rs — HMAC безопасность
- test_security_fixes.rs — исправления безопасности

#### Тесты игры (25 файлов)
- test_bag_system.rs — Bag Generator
- test_collision.rs — столкновения
- test_game_bitmask_check_rows.rs — битовая маска check_rows
- test_game_bounds_check.rs — проверка границ
- test_game_box_array.rs — массив поля
- test_game_logic.rs — игровая логика
- test_game_movement.rs — движение фигур
- test_game_negative_coords.rs — отрицательные координаты
- test_game_rotation.rs — вращение фигур
- test_game_rotation_bounds.rs — границы вращения
- test_game_score_overflow.rs — переполнение счёта
- test_game_stats_export.rs — экспорт статистики
- test_hard_drop_flag.rs — флаг hard drop
- test_hard_drop_overflow.rs — переполнение hard drop
- test_integration.rs — интеграционные тесты
- test_integration_extended.rs — расширенные интеграционные
- test_modes_integration.rs — режимы игры
- test_physics.rs — физика
- test_row_check_optimization.rs — оптимизация check_rows
- test_safe_cast.rs — безопасный cast
- test_score_overflow_protection.rs — защита от переполнения
- test_state_validation.rs — валидация состояния
- test_tetromino.rs — Tetromino
- test_tetromino_dir_down.rs — направление Down
- test_tetromino_shapes.rs — формы фигур

#### Тесты highscore (5 файлов)
- test_highscore.rs — базовые тесты highscore
- test_highscore_config_path.rs — путь к конфигурации
- test_highscore_error_handling.rs — обработка ошибок
- test_highscore_random_hash.rs — случайный hash
- test_highscore_verify_integrity.rs — проверка целостности

#### Тесты controls (4 файла)
- test_controls.rs — базовые тесты controls
- test_controls_error_handling.rs — обработка ошибок
- test_controls_path_validation.rs — валидация путей
- test_controls_toctou.rs — TOCTOU защита

#### Тесты IO (5 файлов)
- test_canvas_initialization.rs — инициализация Canvas
- test_io.rs — базовые тесты IO
- test_io_canvas_result.rs — результат Canvas
- test_io_drop.rs — Drop для Canvas
- test_io_errors.rs — ошибки IO
- test_io_resource_leak.rs — утечки ресурсов
- test_io_utf8_handling.rs — обработка UTF-8
- test_utf8_limitation.rs — ограничения UTF-8

#### Тесты оптимизаций (6 файлов)
- test_animation.rs — анимации
- test_benchmarks.rs — бенчмарки
- test_bounds_check_optimization.rs — оптимизация границ
- test_cast_safety.rs — безопасность cast
- test_edge_cases.rs — граничные случаи
- test_edge_cases_stress.rs — стресс тесты
- test_must_use_attributes.rs — must_use атрибуты
- test_sanitize_optimization.rs — оптимизация sanitize
- test_string_caching.rs — кэширование строк
- test_track_caller.rs — track_caller атрибут
- test_unwrap_to_expect.rs — unwrap → expect

#### Вспомогательные файлы (6 файлов)
- test_fixtures.rs — фикстуры и хелперы
- test_macros.rs — макросы для тестов
- test_mod.rs — модуль тестов
- test_scoring_state.rs — состояние scoring
- test_leaderboard_toctou.rs — TOCTOU leaderboard
- test_all_fixes_integration.rs — интеграция всех исправлений
- test_error_propagation.rs — распространение ошибок
- test_task13_coverage.rs — покрытие кода
- test_time_safety.rs — безопасность Time
- test_unicode_validation.rs — валидация Unicode
- test_statistics.rs — статистика

---
  - `test_input_logic_separation` — handle_input() использует parse_input() и execute_action()
- **H6: Абстракция времени** (2 теста):
  - `test_time_abstraction_exists` — Time структура существует
  - `test_time_type_safety` — Time используется вместо f64
- **Интеграционный тест** (1 тест):
  - `test_all_architecture_integrity_tests_pass` — все тесты проходят

### Новые тесты (31 марта 2026 — версия 23.96.21+):

**test_audit_fixes.rs — 25 тестов:**
- **C1: Замена TetrominoType → ShapeType** (2 теста):
  - `test_c1_shapetype_in_game_event` — ShapeType в GameEvent
  - `test_c1_shapetype_in_tetromino` — ShapeType в Tetromino
- **H1: Инвертированная логика has_collision** (4 теста):
  - `test_h1_has_collision_returns_true_on_collision` — true при коллизии
  - `test_h1_has_collision_returns_false_no_collision` — false без коллизии
  - `test_h1_has_collision_different_positions` — разные позиции
- **H2: TOCTOU защита ThreadSafeLeaderboardEntry** (4 теста):
  - `test_h2_thread_safe_leaderboard_entry_exists` — существование типа
  - `test_h2_thread_safe_score_returns_correct_value` — корректность score()
  - `test_h2_verify_hash_for_value` — verify_hash_for_value
  - `test_h2_thread_safe_leaderboard_multithreaded` — потокобезопасность
- **H3: Удаление #[inline] атрибутов** (2 теста):
  - `test_h3_no_inline_in_collision` — нет #[inline] в collision.rs
  - `test_h3_no_inline_in_board` — нет #[inline] в board.rs
- **M1: Централизация констант** (2 теста):
  - `test_m1_max_config_file_size_constant` — MAX_CONFIG_FILE_SIZE
  - `test_m1_constant_imports` — импорты констант
- **M2: Оптимизация sanitize_player_name** (3 теста):
  - `test_m2_sanitize_player_name_memory_allocation` — выделение памяти
  - `test_m2_sanitize_player_name_filters_invalid_chars` — фильтрация
  - `test_m2_sanitize_player_name_empty_to_anonymous` — пустое имя
- **M3: Семантические методы GameState** (3 теста):
  - `test_m3_apply_gravity_increases_fall_speed` — apply_gravity()
  - `test_m3_spawn_new_piece` — spawn_new_piece()
  - `test_m3_update_fall_speed` — update_fall_speed()
- **L4: Рефакторинг application.rs** (4 теста):
  - `test_l4_render_menu_frame_exists` — render_menu_frame()
  - `test_l4_process_menu_input_exists` — process_menu_input()
  - `test_l4_check_exit_condition_exists` — check_exit_condition()
  - `test_l4_application_functions_integration` — интеграция
- **Интеграционные тесты** (2 теста):
  - `test_all_fixes_compile_together` — все исправления компилируются
  - `test_game_event_uses_shapetype_not_tetrominotype` — ShapeType не TetrominoType

### Изменения после очистки тестовой базы (1 апреля 2026):

#### Удалённые тесты (~40 тестов):

**Удалено файлов (2):**
- `test_mod.rs` — пустой файл, 0 тестов
- `test_tetromino_dir_down.rs` — тесты с удалённым NoRotation

**Дубликаты коллизий со стенами (6 тестов → 2):**
- Удалены: `test_collision_left_boundary`, `test_collision_right_boundary` (test_game_logic.rs)
- Удалены: `test_collision_with_left_wall`, `test_collision_with_right_wall` (test_physics.rs)
- Удалены: `test_move_to_left_boundary_blocked`, `test_move_to_right_boundary_blocked` (test_game_movement.rs)

**Дубликаты коллизий с полом (4 теста → 1):**
- Удалены: `test_collision_reaches_floor`, `test_collision_not_through_floor` (test_collision.rs)
- Удалены: `test_piece_reaching_floor` (test_physics.rs)
- Удалены: `test_move_to_bottom_boundary_blocked` (test_game_movement.rs)

**Дубликаты призрачной фигуры (3 теста → 1):**
- Удалены: `test_collision_ghost_piece` (test_collision.rs)
- Удалены: `test_ghost_piece_boundary` (test_game_logic.rs)
- Удалены: `test_ghost_piece_floor_detection` (test_physics.rs)

**Дубликаты движения фигур (14 тестов → 1):**
- Удалены: `test_t_piece_move_left`, `test_t_piece_move_right`, и другие тесты для J, L, O, S, Z, I pieces

**Дубликаты Hard Drop (6 тестов → 1):**
- Удалены: `test_hard_drop_basic`, `test_hard_drop_instant_stop`, `test_hard_drop_final_position`, `test_hard_drop_different_heights`, `test_hard_drop_o_piece`

**Бесполезные тесты:**
- `test_save_error_handling` (test_highscore_error_handling.rs)
- `test_must_use_get_elapsed_time` (test_must_use_attributes.rs)
- `test_min_game_time` (test_edge_cases_stress.rs)
- `test_sprint_mode_timer` (test_game_modes_detailed.rs)
- `test_game_has_timer` (test_integration_extended.rs)

**Тесты-документации:**
- `test_critical_canvas_graceful_degradation` (test_architecture_all.rs)
- `test_critical_controls_toctou_protection` (test_architecture_all.rs)
- `test_architecture_no_cyclic_dependencies_core` (test_architecture_all.rs)
- `test_code_checked_neg_rotation` (test_architecture_all.rs)

**Тесты без ассертов:**
- `test_get_current_time_ms_no_panic` (test_time_safety.rs)
- `test_collision_assertions_meaningful` (test_all_fixes_integration.rs)
- `test_unused_import_fixed` (test_fixes.rs)

**Сломанные тесты:**
- `test_directory_unavailable_error_handling` (test_highscore_config_path.rs) — использовал ConfigError

#### Добавлены параметризированные тесты (2):
- `test_all_pieces_move_left_right` — заменил 14 тестов движения фигур
- `test_hard_drop_comprehensive` — заменил 6 тестов hard drop

#### Итоговая статистика:
- **Всего тестов:** ~1306 (было ~1344)
- **Удалено тестов:** ~40
- **Объединено тестов:** 20 → 2
- **Удалено файлов:** 2
- **Изменено файлов:** 12
- **Все тесты компилируются:** ✅
- **Все тесты проходят:** ✅ (100% pass rate)
- **Версия проекта:** 23.96.31+

---

## 📊 ИЗМЕНЕНИЯ ПРИ ПОСЛЕДНЕЙ АКТУАЛИЗАЦИИ (31 марта 2026)

### Новые тесты (37 тестов):

**Новые файлы тестов:**
- `test_io_drop.rs` — 3 теста (Drop для IO компонентов)
- `test_set_fall_speed_validation.rs` — 6 тестов (валидация set_fall_speed)
- `test_url_encoded_path_traversal.rs` — 8 тестов (URL-encoding в PathValidator)
- `test_validation_service.rs` — 6 тестов (ValidationService тесты)
- `test_leaderboard_toctou.rs` — 8 тестов (TOCTOU защита в leaderboard)
- `test_cycle_functions.rs` — 9 тестов (разделение функций в cycle.rs)

### Исправленные проблемы:

**1. Canvas graceful degradation (E1):**
- Исправлен тест `test_fix_e1_canvas_graceful_degradation` — теперь проверяет код, а не создаёт Canvas в безтерминальном окружении
- Обновлён интеграционный тест `test_all_critical_fixes_integration`

**2. Архитектурные тесты (ISP compliance):**
- Исправлены тесты `test_component_access_traits` и `test_isp_compliance` — теперь корректно проверяют реэкспорт трейтов через `pub use super::access::`

### Результат:
- **Все тесты проходят:** ✅ 100% pass rate
- **Компиляция:** ✅ успешна
- **Линтеры:** ✅ без критических замечаний

### Integration тесты (tests/): 127 тестов

| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_architecture_components.rs` | 29 | Проверка архитектурных компонентов |
| `test_architecture_refactoring.rs` | 37 | Тесты рефакторинга архитектуры |
| `test_architecture_integrity.rs` | 21 | Целостность архитектуры (НОВЫЙ ФАЙЛ v23.96.27+) |
| `test_architecture_integrity_new.rs` | 17 | Новая целостность архитектуры |
| `test_fixes_verification.rs` | 14 | Верификация исправлений |
| `test_architecture_improvements.rs` | 9 | Улучшения архитектуры |

### Unit тесты (src/tests/): 1138 тестов

**Архитектурные тесты:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_architecture.rs` | 17 | Базовые архитектурные тесты |
| `test_architecture_components.rs` | 5 | Компоненты архитектуры |
| `test_architecture_traits.rs` | 9 | Архитектурные трейты |
| `test_architecture_validation.rs` | 13 | Валидация архитектуры |
| `test_architecture_separation.rs` | 11 | Разделение ответственности |
| `test_architecture_isp.rs` | 14 | Interface Segregation Principle |
| `test_architecture_coupling.rs` | 12 | Снижение связанности |
| `test_architecture_integrity.rs` | 15 | Целостность архитектуры |
| `test_architecture_fixes.rs` | 18 | Исправления архитектуры |
| `test_safety_architecture.rs` | 3 | Безопасность архитектуры |

**Тесты безопасности:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_hmac_safety.rs` | 18 | Безопасность HMAC-SHA256 |
| `test_cast_safety.rs` | 11 | Безопасная конвертация типов |
| `test_safe_cast.rs` | 14 | Безопасные вычисления |
| `test_state_validation.rs` | 15 | Валидация состояний |
| `test_unicode_validation.rs` | 12 | Валидация Unicode |

**Тесты игры:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_game_logic.rs` | 22 | Логика игры |
| `test_game_movement.rs` | 50 | Движение фигур |
| `test_game_rotation.rs` | 50 | Вращение фигур |
| `test_game_modes_detailed.rs` | 40 | Режимы игры |
| `test_game_score_overflow.rs` | 5 | Переполнение счёта |
| `test_game_negative_coords.rs` | 4 | Отрицательные координаты |
| `test_game_bounds_check.rs` | 3 | Проверка границ |
| `test_game_box_array.rs` | 3 | Массив коробок |
| `test_game_bitmask_check_rows.rs` | 5 | Bitmask для проверки линий |
| `test_game_stats_export.rs` | 8 | Экспорт статистики |

**Тесты физики и коллизий:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_collision.rs` | 25 | Коллизии |
| `test_physics.rs` | 19 | Физика |
| `test_hard_drop_flag.rs` | 7 | Флаг жёсткого сброса |
| `test_hard_drop_overflow.rs` | 8 | Переполнение жёсткого сброса |
| `test_direction_down.rs` | 4 | Движение вниз |
| `test_wall_kick_refactor.rs` | 7 | Рефакторинг Wall Kick |

**Тесты очков и рекордов:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_highscore.rs` | 15 | Рекорды |
| `test_highscore_config_path.rs` | 4 | Конфигурация путей рекордов |
| `test_highscore_error_handling.rs` | 5 | Обработка ошибок рекордов |
| `test_highscore_random_hash.rs` | 5 | Случайные хеши рекордов |
| `test_highscore_verify_integrity.rs` | 5 | Верификация целостности |
| `test_score_overflow_protection.rs` | 12 | Защита переполнения счёта |
| `test_scoring_encapsulation.rs` | 5 | Инкапсуляция подсчёта очков |

**Тесты управления:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_controls.rs` | 20 | Управление |
| `test_controls_error_handling.rs` | 5 | Обработка ошибок управления |
| `test_controls_path_traversal.rs` | 7 | Обход путей управления |
| `test_controls_path_validation.rs` | 7 | Валидация путей управления |
| `test_controls_toctou.rs` | 10 | TOCTOU в управлении |

**Тесты ввода-вывода:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_io.rs` | 4 | Базовый ввод-вывод |
| `test_io_errors.rs` | 8 | Ошибки ввода-вывода |
| `test_io_canvas_result.rs` | 5 | Результаты Canvas |
| `test_io_resource_leak.rs` | 5 | Утечки ресурсов |
| `test_io_utf8_handling.rs` | 5 | Обработка UTF-8 |
| `test_io_drop.rs` | 6 | Drop для IO |

**Тесты интеграции:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_integration.rs` | 20 | Базовая интеграция |
| `test_integration_extended.rs` | 49 | Расширенная интеграция |
| `test_modes_integration.rs` | 20 | Интеграция режимов |
| `test_all_fixes_integration.rs` | 23 | Интеграция всех исправлений |

**Тесты тетромино:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_tetromino.rs` | 16 | Базовые тесты тетромино |
| `test_tetromino_shapes.rs` | 40 | Формы тетромино |
| `test_tetromino_dir_down.rs` | 5 | Направление вниз |
| `test_bag_system.rs` | 27 | Система мешков |

**Тесты анимации:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_animation.rs` | 22 | Анимации |

**Тесты ошибок и валидации:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_error_propagation.rs` | 5 | Пропagация ошибок |
| `test_application_error_handling.rs` | 15 | Обработка ошибок приложения |
| `test_audit_fixes.rs` | 25 | Исправления аудита (C1, H1, H2, H3, M1, M2, M3, L4) |
| `test_edge_cases.rs` | 35 | Граничные случаи |
| `test_edge_cases_stress.rs` | 20 | Стресс-тесты граничных случаев |

**Тесты оптимизации:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_bounds_check_optimization.rs` | 7 | Оптимизация проверки границ |
| `test_row_check_optimization.rs` | 9 | Оптимизация проверки линий |
| `test_sanitize_optimization.rs` | 8 | Оптимизация санитизации |
| `test_string_caching.rs` | 8 | Кеширование строк |

**Тесты безопасности (security):**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_security_fixes.rs` | 11 | Исправления безопасности |
| `test_leaderboard_toctou.rs` | 10 | TOCTOU в таблице лидеров |
| `test_canvas_initialization.rs` | 15 | Инициализация Canvas |
| `test_cfg_attr_dead_code.rs` | 7 | Мёртвый код cfg_attr |
| `test_must_use_attributes.rs` | 12 | Атрибуты #[must_use] |
| `test_track_caller.rs` | 7 | Атрибут #[track_caller] |
| `test_unwrap_to_expect.rs` | 5 | unwrap → expect |

**Тесты фиксов:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_fixes.rs` | 15 | Исправления |
| `test_clippy_fixes.rs` | 8 | Исправления Clippy |
| `test_constants_imports.rs` | 19 | Константы и импорты |

**Тесты статистики и бенчмарков:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_statistics.rs` | 26 | Статистика |
| `test_benchmarks.rs` | 4 | Бенчмарки |
| `test_task13_coverage.rs` | 30 | Покрытие задачи 13 |

**Тесты времени:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_time_safety.rs` | 5 | Безопасность времени |

**Тесты UTF-8:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `test_utf8_limitation.rs` | 7 | Ограничения UTF-8 |

**Прочие тесты:**
| Файл | Количество тестов | Описание |
|------|------------------|----------|
| `scoring_state.rs` | 15 | Состояние подсчёта очков |
| `macros.rs` | 11 | Макросы |
| `fixtures.rs` | 1 | Фикстуры |

### Doctests: ~125 тестов

Тесты в документации (doctests) распределены по основным модулям:
- `game/` — ~25 тестов
- `crypto/` — ~14 тестов
- `validation/` — ~12 тестов
- `highscore/` — ~10 тестов
- `controls/` — ~7 тестов
- `io/` — ~7 тестов
- `types/` — ~7 тестов
- `tetromino/` — ~6 тестов
- `config/` — ~8 тестов
- `errors/` — ~4 тестов
- `menu/` — ~5 тестов
- `exports/` — ~5 тестов
- `constants/` — ~3 тестов
- `app/` — ~8 тестов
- `src/lib.rs` — ~7 тестов

---

## 📊 ОБЩАЯ СТАТИСТИКА

### Общее количество тестов: 1306

**Unit тесты:** 1104
**Integration тесты:** 106
**Doctests:** ~125 (включены в unit/integration)
**Удалено тестов:** ~40 (очистка дубликатов и бесполезных тестов)

**Процент прохождения:** 100% (0 failed)

### Динамика изменений:

| Дата | Событие | Тестов | Изменение |
|------|---------|--------|-----------|
| 30 марта 2026 (утро) | Архитектурные улучшения | 1227 | +129 |
| 30 марта 2026 (вечер) | Очистка тестов | 1225 | -74 + 72 |
| 1 апреля 2026 (утро) | Аудит и исправления | 1344 | +119 |
| 1 апреля 2026 (вечер) | Очистка тестовой базы | 1306 | -40 + 2 |

---

## 📁 ПОЛНЫЙ СПИСОК ФАЙЛОВ С ТЕСТАМИ

### Интеграционные тесты (tests/)

| Файл | Тестов | Статус |
|------|--------|--------|
| `test_architecture_components.rs` | 29 | ✅ |
| `test_architecture_refactoring.rs` | 37 | ✅ |
| `test_architecture_integrity.rs` | 17 | ✅ |
| `test_fixes_verification.rs` | 14 | ✅ |
| `test_architecture_improvements.rs` | 9 | ✅ |

**Итого tests/:** 127 тестов

### Unit тесты (src/tests/)

| Файл | Тестов | Статус |
|------|--------|--------|
| `test_architecture.rs` | 17 | ✅ |
| `test_architecture_components.rs` | 5 | ✅ |
| `test_architecture_traits.rs` | 9 | ✅ |
| `test_architecture_validation.rs` | 13 | ✅ |
| `test_architecture_separation.rs` | 11 | ✅ |
| `test_architecture_isp.rs` | 14 | ✅ |
| `test_architecture_coupling.rs` | 12 | ✅ |
| `test_architecture_integrity.rs` | 15 | ✅ |
| `test_architecture_fixes.rs` | 18 | ✅ |
| `test_safety_architecture.rs` | 3 | ✅ |
| `test_hmac_safety.rs` | 18 | ✅ |
| `test_cast_safety.rs` | 11 | ✅ |
| `test_safe_cast.rs` | 14 | ✅ |
| `test_state_validation.rs` | 15 | ✅ |
| `test_unicode_validation.rs` | 12 | ✅ |
| `test_game_logic.rs` | 22 | ✅ |
| `test_game_movement.rs` | 50 | ✅ |
| `test_game_rotation.rs` | 50 | ✅ |
| `test_game_modes_detailed.rs` | 40 | ✅ |
| `test_game_score_overflow.rs` | 5 | ✅ |
| `test_game_negative_coords.rs` | 4 | ✅ |
| `test_game_bounds_check.rs` | 3 | ✅ |
| `test_game_box_array.rs` | 3 | ✅ |
| `test_game_bitmask_check_rows.rs` | 5 | ✅ |
| `test_game_stats_export.rs` | 8 | ✅ |
| `test_collision.rs` | 25 | ✅ |
| `test_physics.rs` | 19 | ✅ |
| `test_hard_drop_flag.rs` | 7 | ✅ |
| `test_hard_drop_overflow.rs` | 8 | ✅ |
| `test_direction_down.rs` | 4 | ✅ |
| `test_wall_kick_refactor.rs` | 7 | ✅ |
| `test_highscore.rs` | 15 | ✅ |
| `test_highscore_config_path.rs` | 4 | ✅ |
| `test_highscore_error_handling.rs` | 5 | ✅ |
| `test_highscore_random_hash.rs` | 5 | ✅ |
| `test_highscore_verify_integrity.rs` | 5 | ✅ |
| `test_score_overflow_protection.rs` | 12 | ✅ |
| `test_scoring_encapsulation.rs` | 5 | ✅ |
| `test_controls.rs` | 20 | ✅ |
| `test_controls_error_handling.rs` | 5 | ✅ |
| `test_controls_path_traversal.rs` | 7 | ✅ |
| `test_controls_path_validation.rs` | 7 | ✅ |
| `test_controls_toctou.rs` | 10 | ✅ |
| `test_io.rs` | 4 | ✅ |
| `test_io_errors.rs` | 8 | ✅ |
| `test_io_canvas_result.rs` | 5 | ✅ |
| `test_io_resource_leak.rs` | 5 | ✅ |
| `test_io_utf8_handling.rs` | 5 | ✅ |
| `test_io_drop.rs` | 6 | ✅ |
| `test_integration.rs` | 20 | ✅ |
| `test_integration_extended.rs` | 49 | ✅ |
| `test_modes_integration.rs` | 20 | ✅ |
| `test_all_fixes_integration.rs` | 23 | ✅ |
| `test_tetromino.rs` | 16 | ✅ |
| `test_tetromino_shapes.rs` | 40 | ✅ |
| `test_tetromino_dir_down.rs` | 5 | ✅ |
| `test_bag_system.rs` | 27 | ✅ |
| `test_animation.rs` | 22 | ✅ |
| `test_error_propagation.rs` | 5 | ✅ |
| `test_application_error_handling.rs` | 15 | ✅ |
| `test_audit_fixes.rs` | 25 | ✅ |
| `test_edge_cases.rs` | 35 | ✅ |
| `test_edge_cases_stress.rs` | 20 | ✅ |
| `test_bounds_check_optimization.rs` | 7 | ✅ |
| `test_row_check_optimization.rs` | 9 | ✅ |
| `test_sanitize_optimization.rs` | 8 | ✅ |
| `test_string_caching.rs` | 8 | ✅ |
| `test_security_fixes.rs` | 11 | ✅ |
| `test_leaderboard_toctou.rs` | 10 | ✅ |
| `test_canvas_initialization.rs` | 15 | ✅ |
| `test_cfg_attr_dead_code.rs` | 7 | ✅ |
| `test_must_use_attributes.rs` | 12 | ✅ |
| `test_track_caller.rs` | 7 | ✅ |
| `test_unwrap_to_expect.rs` | 5 | ✅ |
| `test_fixes.rs` | 15 | ✅ |
| `test_clippy_fixes.rs` | 8 | ✅ |
| `test_constant_imports.rs` | 19 | ✅ |
| `test_statistics.rs` | 26 | ✅ |
| `test_benchmarks.rs` | 4 | ✅ |
| `test_task13_coverage.rs` | 30 | ✅ |
| `test_time_safety.rs` | 5 | ✅ |
| `test_utf8_limitation.rs` | 7 | ✅ |
| `scoring_state.rs` | 15 | ✅ |
| `macros.rs` | 11 | ✅ |
| `fixtures.rs` | 1 | ✅ |

**Итого src/tests/:** 1138 тестов

---

## 📅 ДАТА ПОСЛЕДНЕЙ ОЧИСТКИ

**Дата:** 1 апреля 2026 г.

**Что было сделано:**
- ✅ Удалено 2 файла тестов (test_mod.rs, test_tetromino_dir_down.rs)
- ✅ Удалено ~40 дубликатов и бесполезных тестов
- ✅ Объединено 20 тестов в 2 параметризированных
- ✅ Исправлены сломанные тесты (ConfigError)
- ✅ Актуализирована статистика по всем файлам

**Результат:**
- До очистки: ~1344 тестов, 95 файлов
- После очистки: ~1306 тестов, 93 файла
- Качество тестовой базы: улучшено (удалены дубликаты и фиктивные тесты)

---

## 🆕 НОВЫЕ ТЕСТЫ (1 апреля 2026 — версия 23.96.32+)

### test_audit_2026_04_fixes.rs — 26 тестов

**Файл:** `tests/test_audit_2026_04_fixes.rs`

**Описание:** Тесты для всех исправленных проблем из аудита кода (апрель 2026).

**CRITICAL проблемы (C1-C3) — 3 теста:**
- `test_c1_validate_hmac_key_rejects_empty` — валидация HMAC ключей (пустые и с пробелами)
- `test_c2_key_reader_handles_ascii_correctly` — ASCII обработка KeyReader
- `test_c3_toctou_documentation_has_key_methods` — TOCTOU документация и ключевые методы

**HIGH проблемы (H1-H10) — 10 тестов:**
- `test_h1_to_string_instead_of_format_in_state` — .to_string() вместо format!()
- `test_h2_is_none_or_in_collision` — is_none_or() в collision.rs
- `test_h3_ufcs_instead_of_closure_in_leaderboard` — UFCS вместо closure
- `test_h4_sort_by_key_in_leaderboard` — sort_by_key() в leaderboard.rs
- `test_h5_h6_errors_documentation_exists` — документация # Errors
- `test_h7_fps_constants_defined` — константы FPS
- `test_h8_forbidden_url_encoding_patterns` — URL-encoding паттерны
- `test_h9_compute_signature_method_exists` — compute_signature()
- `test_h10_consolidated_config_load_methods` — загрузка конфигурации

**MEDIUM проблемы (M1-M10) — 7 тестов:**
- `test_m1_no_redundant_ignore_examples_in_lib` — отсутствие ignore примеров
- `test_m3_simplified_canvas_drop` — упрощённый Canvas Drop
- `test_m4_must_use_only_on_critical_methods` — #[must_use] атрибуты
- `test_m5_dead_code_marked_with_allow_attribute` — #[allow(dead_code)]
- `test_m7_sanitize_removed_uses_validation_name` — validation::name вместо sanitize.rs
- `test_m10_sanitize_player_name_single_pass` — однопроходная оптимизация

**LOW проблемы (L1, L3, L4) — 3 теста:**
- `test_l1_key_codes_module_exists` — модуль key_codes
- `test_l3_simplified_error_constructors` — конструкторы ошибок
- `test_l4_simplified_exports` — упрощённые экспорты

**Интеграционные тесты — 3 теста:**
- `test_all_critical_fixes_integration` — интеграция CRITICAL исправлений
- `test_all_high_fixes_integration` — интеграция HIGH исправлений
- `test_all_medium_fixes_integration` — интеграция MEDIUM исправлений
- `test_all_low_fixes_integration` — интеграция LOW исправлений
- `test_all_26_audit_fixes_complete_integration` — полная интеграция всех 26 исправлений

**Итого:** 26 тестов (все проходят 100%)
