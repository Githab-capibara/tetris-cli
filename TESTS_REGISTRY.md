# Реестр тестов Tetris CLI

> Дата последней актуализации: 2026-04-13

## Модульные тесты (src/tests/)

| Файл | Описание | Тип | Статус |
|------|----------|-----|--------|
| `test_collision.rs` | Тесты столкновений фигур со стенами, полом и препятствиями | Модульный | active |
| `test_game_logic.rs` | Тесты базовой игровой логики: движение, вращение, уровни | Модульный | active |
| `test_integration.rs` | Интеграционные тесты: инициализация, движение фигур, режимы, статистика | Интеграционный | active |
| `test_physics.rs` | Тесты физики: скорость падения, hold, призрачная фигура | Модульный | active |
| `test_bag_system.rs` | Тесты генератора фигур (Bag System): распределение, случайность | Модульный | active |
| `test_boundary_values.rs` | Тесты граничных значений: уровни, очки, линии, конвертация | Модульный | active |
| `test_game_bounds_check.rs` | Тесты границ игрового поля: позиции на краях | Модульный | active |
| `test_game_movement.rs` | Тесты движения фигур: влево, вправо, вниз, hard drop | Модульный | active |
| `test_game_rotation.rs` | Тесты вращения фигур: все типы, столкновения при вращении | Модульный | active |
| `test_hmac_safety.rs` | Тесты HMAC: подпись, верификация, уникальность, tamper resistance | Модульный | active |
| `test_safe_cast.rs` | Тесты безопасной конвертации f32→u32: нормальные, специальные, граничные значения | Модульный | active |
| `test_score_overflow_protection.rs` | Тесты защиты от переполнения счёта: MAX_SCORE, saturating arithmetic | Модульный | active |
| `test_state_validation.rs` | Тесты валидации fall_speed и land_timer: NaN, Infinity, clamp | Модульный | active |
| `test_crypto_security.rs` | Тесты криптографических функций: HMAC, hash, соль | Модульный | active |
| `test_edge_cases.rs` | Тесты краевых случаев: конфигурация, лидерборд, опциональные значения | Модульный | active |
| `test_module_isolation.rs` | Тесты модульной изоляции: независимость crypto, validation, types, errors, highscore | Модульный | active |
| `test_panic_handling.rs` | Тесты отсутствия паник: GameState, BagGenerator, PathValidator | Модульный | active |

## Встроенные модульные тесты (src/)

| Модуль | Описание | Тип | Статус |
|--------|----------|-----|--------|
| `app/mod.rs` | Тесты загрузки игровых данных, логирование ошибок | Модульный | active (частично ignored) |
| `config/keys.rs` | Тесты валидации HMAC ключей | Модульный | active |
| `constants.rs` | Тесты констант границ поля | Модульный | active |
| `controls.rs` | Тесты конфигурации управления, HMAC, сохранение | Модульный | active |
| `core/mod.rs` | Тесты Position, Direction, RotationDirection | Модульный | active |
| `crypto.rs` | Тесты hash, generate_salt | Модульный | active |
| `crypto/hmac.rs` | Тесты hmac_sha256, verify_hmac_sha256 | Модульный | active |
| `game/cycle.rs` | Тесты игрового цикла, обработка ввода | Модульный | active |
| `tetromino/bag_generator.rs` | Тесты BagGenerator, Fisher-Yates | Модульный | active |
| `tetromino/constants.rs` | Тесты координат фигур, цветов | Модульный | active |
| `tetromino/shape_type.rs` | Тесты ShapeType: clone, copy, debug, hash | Модульный | active |
| `tetromino/tetromino_struct.rs` | Тесты Tetromino: new, rotate, from_bag | Модульный | active |
| `types.rs` | Тесты Score, Level, LinesCount, GameAction | Модульный | active |
| `validation/name.rs` | Тесты санитаризации имён | Модульный | active |
| `validation/path.rs` | Тесты PathValidator: traversal, symlink, URL encoding | Модульный | active |
| `validation/service.rs` | Тесты validate_f32_finite, validate_u32_range | Модульный | active |

## Интеграционные тесты (tests/)

| Файл | Описание | Тип | Статус |
|------|----------|-----|--------|
| `tests/test_architecture_integrity.rs` | Тесты потокобезопасности LeaderboardEntry | Интеграционный | active |

## Бенчмарки (benches/)

| Файл | Описание | Тип | Статус |
|------|----------|-----|--------|
| `benchmarks.rs` | Бенчмарки производительности | Бенчмарк | active |

## Документационные тесты

Документационные тесты встроены в doc-строки модулей. 64 проходят, 92 игнорируются (требуют терминала, компиляции или внешних зависимостей).

## Итого

- **Модульные тесты (src/tests/):** 18 файлов
- **Встроенные модульные тесты (src/):** ~530 тестов
- **Интеграционные тесты (tests/):** 1 файл, 2 теста
- **Документационные тесты:** 64 passed, 92 ignored
- **Бенчмарки:** 1 файл
- **Общий статус:** ✅ Все тесты проходят
