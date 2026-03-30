# 🏗️ Архитектура Tetris CLI

**Версия:** 2.1
**Дата:** 30 марта 2026 (архитектурные улучшения)
**Проект:** tetris-cli v23.96.14+

---

## 📊 Общая схема проекта

```
tetris-cli/
├── src/
│   ├── main.rs              # Точка входа
│   ├── lib.rs               # Публичный API библиотеки
│   ├── app/                 # Application layer
│   │   ├── mod.rs
│   │   └── application.rs   # Application struct, игровой цикл
│   ├── game/                # Игровая логика
│   │   ├── mod.rs
│   │   ├── state.rs         # GameState (фасад)
│   │   ├── board.rs         # GameBoard (состояние поля)
│   │   ├── scoreboard.rs    # ScoreBoard (очки и уровни)
│   │   ├── stats.rs         # GameStats (статистика)
│   │   ├── mode_trait.rs    # GameModeTrait
│   │   ├── types.rs         # Типобезопасные обёртки (Score, Level, LinesCount)
│   │   ├── view.rs          # GameView для отрисовки
│   │   ├── access.rs        # Трейты доступа (BoardReadonly, BoardMutable)
│   │   ├── cache.rs         # StringCache для кэширования строк
│   │   ├── cycle.rs         # Игровой цикл (run_game_loop)
│   │   ├── render.rs        # Отрисовка игрового поля
│   │   ├── logic/           # Логика игры
│   │   │   ├── mod.rs
│   │   │   ├── input.rs     # Обработка ввода
│   │   │   ├── physics.rs   # Физика и гравитация
│   │   │   ├── collision.rs # Проверка коллизий
│   │   │   ├── rotation.rs  # Вращение с wall kick
│   │   │   ├── update.rs    # Обновление состояния
│   │   │   └── wall_kick.rs # Wall kick логика
│   │   └── scoring/         # Система очков
│   │       ├── mod.rs
│   │       ├── lines.rs     # Поиск и удаление линий
│   │       ├── points.rs    # Начисление очков
│   │       └── combo.rs     # Комбо-логика
│   ├── menu/                # Главное меню
│   │   ├── mod.rs
│   │   ├── constants.rs
│   │   ├── draw.rs
│   │   └── input.rs
│   ├── highscore/           # Система рекордов
│   │   ├── mod.rs
│   │   ├── leaderboard.rs
│   │   ├── sanitize.rs
│   │   └── save_data.rs
│   ├── tetromino/           # Фигуры (7-bag система)
│   │   ├── mod.rs
│   │   ├── constants.rs     # SHAPE_COORDS, SHAPE_COLORS
│   │   ├── shape_type.rs    # enum ShapeType
│   │   ├── tetromino.rs     # struct Tetromino
│   │   └── bag_generator.rs # struct BagGenerator
│   ├── controls.rs          # Конфигурация управления
│   ├── io.rs                # Ввод/вывод (Canvas, KeyReader)
│   ├── io_traits.rs         # Трейты InputReader, Renderer
│   ├── crypto/              # Криптография
│   │   ├── mod.rs
│   │   └── validator.rs     # HmacValidator для HMAC-SHA256
│   ├── validation/          # Валидация
│   │   ├── mod.rs
│   │   ├── name.rs
│   │   └── path.rs
│   ├── types.rs             # Direction, RotationDirection, Position
│   ├── errors.rs            # GameError (thiserror)
│   ├── constants.rs         # Глобальные константы
│   └── tests/               # Интеграционные тесты (67 файлов)
├── tests/                   # Integration tests
│   └── test_architecture_integrity.rs
├── docs/
│   └── ARCHITECTURE.md      # Подробная документация
├── benches/
│   └── benchmarks.rs        # Бенчмарки производительности
├── Cargo.toml
└── README.md
```

---

## 🎯 Основные модули

### 1. Application Layer (`app/`)

**Ответственность:** Управление жизненным циклом приложения

**Компоненты:**
- `Application` — основная структура приложения
- `run()` — точка входа после main.rs

**Зависимости:**
- `game/` — игровая логика
- `menu/` — меню
- `highscore/` — рекорды

---

### 2. Game Module (`game/`)

**Ответственность:** Игровая логика и состояние

**Ключевые компоненты:**

#### GameState (state.rs)
```rust
pub struct GameState {
    // Состояние игры
    score: u128,
    level: u32,
    lines_cleared: u32,

    // Фигуры
    curr_shape: Tetromino,
    next_shape: Tetromino,
    held_shape: Option<Tetromino>,

    // Игровое поле
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],

    // Физика
    fall_speed: f32,
    land_timer: f32,

    // Кэширование
    render_cache: RenderCache,
}
```

**Инкапсуляция:**
- Геттеры: `get_score()`, `get_level()`, `get_lines_cleared()`
- Сеттеры с валидацией: `set_level()`, `set_fall_speed()`
- Saturating arithmetic для защиты от переполнения

#### GameModeTrait (mode_trait.rs)
```rust
pub trait GameModeTrait {
    fn is_win_condition(&self, state: &GameState) -> bool;
    fn is_loss_condition(&self, state: &GameState) -> bool;
    fn get_mode_name(&self) -> &'static str;
}
```

**Реализации:**
- `ClassicMode` — классическая игра
- `SprintMode` — 40 линий на скорость
- `MarathonMode` — 150 линий

#### Подмодули логики (`logic/`)
- `input.rs` — обработка ввода пользователя
- `physics.rs` — физика и гравитация
- `collision.rs` — проверка коллизий
- `rotation.rs` — вращение с wall kick
- `update.rs` — обновление состояния

#### Подмодули системы очков (`scoring/`)
- `lines.rs` — поиск и удаление заполненных линий (битовая маска u32)
- `points.rs` — начисление очков с saturating операциями
- `combo.rs` — комбо-логика и бонусы

---

### 3. Menu Module (`menu/`)

**Ответственность:** Отображение меню и ввод пользователя

**Структура:**
- `constants.rs` — константы меню
- `draw.rs` — функции отрисовки
- `input.rs` — обработка ввода

**Разделение ответственности:**
- Отрисовка отделена от логики
- Ввод обработан отдельно

---

### 4. Highscore Module (`highscore/`)

**Ответственность:** Сохранение и защита рекордов

**Компоненты:**
- `SaveData` — структура сохранения
- `Leaderboard` — таблица лидеров
- `sanitize()` — валидация имён

**Безопасность:**
- BLAKE3 хеширование
- Соль для защиты от подделки
- Rate limiting (10 записей/минуту)

---

### 5. Tetromino Module (`tetromino.rs`)

**Ответственность:** Фигуры и их поведение

**Компоненты:**
- `ShapeType` — 7 типов фигур
- `Tetromino` — фигура с координатами
- `BagGenerator` — 7-bag система

**7-bag система:**
- Каждые 7 фигур содержат все 7 типов
- Алгоритм Fisher-Yates для перемешивания
- Гарантия равномерного распределения

---

### 6. Controls Module (`controls.rs`)

**Ответственность:** Конфигурация управления

**Компоненты:**
- `ControlsConfig` — конфигурация клавиш
- Валидация (делегирование в `validation/`)
- Сериализация JSON

**Безопасность:**
- PathValidator для валидации путей
- HMAC-подпись конфигурации
- Проверка на дубликаты клавиш

---

### 7. Validation Module (`validation/`)

**Ответственность:** Валидация данных

**Компоненты:**
- `PathValidator` — валидация путей
- `sanitize_player_name()` — валидация имён

**Принципы:**
- Централизованная валидация
- Типизированные ошибки
- Whitelist-фильтрация Unicode

---

### 8. Crypto Module (`crypto/`)

**Ответственность:** Криптографические утилиты

**Компоненты:**
- `hash()` — хеширование BLAKE3
- `generate_salt()` — генерация случайной соли
- `HmacValidator` — HMAC-SHA256 подписи и проверка

#### HmacValidator (validator.rs)
```rust
pub struct HmacValidator {
    key: String,
}

impl HmacValidator {
    pub fn new(key: &str) -> Self;
    pub fn generate() -> Self;
    pub fn sign(&self, data: &str) -> String;
    pub fn verify(&self, data: &str, signature: &str) -> bool;
    pub fn verify_and_return(&self, data: &str, signature: &str) -> Option<String>;
}
```

**Применение:**
- Защита конфигурационных файлов
- Проверка целостности данных
- Constant-time сравнение для защиты от timing-атак

---

## 🔗 Зависимости между компонентами

```
main.rs
  │
  ▼
app/application.rs
  │
  ├──────┬───────┐
  ▼      ▼       ▼
menu/   game/   highscore/
  │      │       
  │      ├───────┼───────┬────────┐
  │      ▼       ▼       ▼        ▼
  │   tetromino  io    controls  validation
  │      │       │       
  │      └───────┘       
  │              
  ▼              
crypto/          
```

**Отсутствие циклических зависимостей:**
- Константы вынесены в `game/constants.rs`
- Общие типы в `types.rs`
- Абстракции через трейты

---

## 🏛️ Паттерны проектирования

### 1. **Module Pattern**
Каждый модуль инкапсулирует свою ответственность.

### 2. **Trait Object (Dynamic Dispatch)**
```rust
pub trait GameModeTrait { ... }
pub enum GameMode { Classic, Sprint, Marathon }
impl GameMode { fn as_trait(&self) -> &dyn GameModeTrait { ... } }
```

### 3. **Facade**
`GameState` предоставляет упрощённый интерфейс к сложной игровой логике.

### 4. **Strategy**
`GameModeTrait` позволяет менять стратегию игры.

### 5. **Repository**
`Leaderboard` инкапсулирует доступ к хранилищу рекордов.

### 6. **Value Object**
`Score`, `Level`, `LinesCount` — типобезопасные обёртки.

---

## 📈 Метрики архитектуры

| Метрика | Значение | Оценка |
|---------|----------|--------|
| **Количество модулей** | 18+ | ✅ |
| **Средний размер модуля** | ~350 строк | ✅ |
| **Крупные модули** | 2 (state, tetromino) | ⚠️ |
| **Циклические зависимости** | 0 | ✅ |
| **Покрытие тестами** | 1100+ тестов | ✅ |
| **Публичный API** | Стабильный | ✅ |
| **Меры безопасности** | 10+ (HmacValidator, constant-time HMAC, UTF-8, path traversal, saturating operations) | ✅ |

---

## 🔧 Архитектурные ограничения

### Что нельзя менять:
1. **Публичный API** — `GameState`, `GameMode`, `Canvas`
2. **Формат файлов** — TOML для SaveData, JSON для ControlsConfig
3. **Криптография** — BLAKE3, формат соли
4. **7-bag система** — алгоритм генерации фигур

### Риски рефакторинга:
1. Нарушение обратной совместимости
2. Регрессия производительности
3. Поломка 1037 тестов

---

## 🧪 Тестирование архитектуры

### Integration Tests (`tests/`)
- `test_architecture_integrity.rs` — 19 тестов целостности

### Architectural Tests (`src/tests/`)
- `test_architecture_constraints` — границы модулей
- `test_architecture_integrity` — отсутствие циклов
- `test_module_naming_consistency` — именование

### Тесты безопасности (`src/tests/test_security_fixes.rs`)
- **Constant-time HMAC** — защита от timing-атак (HmacValidator)
- **Валидация UTF-8** — отбрасывание невалидных последовательностей
- **Безопасное вращение** — saturating операции вместо паники
- **Path traversal защита** — canonicalize() перед проверкой
- **Оптимизация sanitize_player_name** — фильтрация в один проход
- **Защита от переполнения времени** — безопасная конвертация u128 → u64
- **Защита от переполнения очков** — saturating_mul/add для всех операций
- **TOCTOU защита** — атомарная проверка и получение в LeaderboardEntry

### Тесты производительности
- Бенчмарки в `benches/benchmarks.rs`
- Тесты производительности в `src/tests/`

### Запуск:
```bash
cargo test                    # Запуск всех тестов
cargo test test_architecture_integrity  # Тесты целостности
cargo bench --features bench  # Бенчмарки
```

**ВСЕГО: 1100+ тестов** (unit + integration + architecture)

---

## 📝 Принципы проектирования

### SOLID
- **S** — Single Responsibility (соблюдается, модули разделены по ответственности)
- **O** — Open/Closed (GameModeTrait для расширения режимов)
- **L** — Liskov Substitution (трейты BoardReadonly/BoardMutable)
- **I** — Interface Segregation (разделение трейтов доступа)
- **D** — Dependency Inversion (TerminalBackend, InputReader, Renderer)

### DRY
- Централизованная валидация в `validation/`
- Общие константы в `constants.rs`
- Криптография в `crypto/`
- Re-export через `game/mod.rs`

### KISS
- Простые структуры данных
- Минимум абстракций
- Явные зависимости между модулями

### YAGNI
- Удалена зависимость fs2
- Удалены неиспользуемые модули
- Удалены избыточные поля структур

---

## 🎯 Оценка архитектуры

**Текущая оценка: 9.2/10**

**Сильные стороны:**
- ✅ Модульная структура с чётким разделением ответственности
- ✅ Отсутствие циклических зависимостей
- ✅ Разделение render/scoring/logic на подмодули
- ✅ Трейты для абстракции (GameModeTrait, TerminalBackend, InputReader, Renderer)
- ✅ Обширное тестирование (1092 теста, включая security-тесты)
- ✅ Защита от переполнения (saturating операции)
- ✅ TOCTOU защита в LeaderboardEntry
- ✅ Централизованная валидация путей с защитой от symlink
- ✅ HmacValidator для HMAC-SHA256 подписей
- ✅ Constant-time сравнение для защиты от timing-атак
- ✅ Битовая маска для заполненных линий (оптимизация памяти)
- ✅ Приватные поля с валидирующими сеттерами в GameStats
- ✅ Оптимизированное кэширование строк с capacity
- ✅ Безопасная конвертация u128 → u64
- ✅ Расширенная валидация Unicode с логированием

**Области улучшения:**
- ⚠️ GameState — крупный модуль (требует дальнейшего разделения)
- ⚠️ tetromino.rs — 600+ строк
- ⚠️ Нарушение инкапсуляции через pub(crate)
- ⚠️ Возможность дальнейшей оптимизации кэширования

---

## 🔒 Безопасность

### Реализованные механизмы защиты:

#### H1: HmacValidator для HMAC-SHA256
- **Файл:** `src/crypto/validator.rs`
- **Решение:** Структура `HmacValidator` с методами `sign()`, `verify()`, `verify_and_return()`
- **Защита:** Constant-time сравнение подписей через XOR-накопление
- **Применение:** Защита конфигурационных файлов, проверка целостности данных

#### C1: Constant-time comparison для HMAC-SHA256
- **Файл:** `src/crypto.rs`
- **Решение:** XOR-накопление вместо раннего выхода при проверке подписей
- **Защита:** Предотвращение timing-атак при проверке HMAC-SHA256

#### C2: Защита от переполнения времени
- **Файл:** `src/io.rs`, `src/game/cycle.rs`
- **Решение:** Безопасная конвертация `u128 → u64` с проверкой границ
- **Защита:** Предотвращение переполнения при длительных интервалах между кадрами

#### C3: Валидация UTF-8
- **Файл:** `src/io.rs`
- **Решение:** Корректное отбрасывание невалидных UTF-8 последовательностей
- **Защита:** Предотвращение паники при получении невалидного Unicode

#### C4: Безопасное вращение фигур
- **Файл:** `src/tetromino.rs`
- **Решение:** `saturating_neg()` вместо `assert!` при вращении
- **Защита:** Предотвращение паники при выходе координат за границы

#### C5: Path traversal защита
- **Файл:** `src/validation/path.rs`
- **Решение:** `canonicalize()` выполняется перед проверкой нахождения в директории
- **Защита:** Предотвращение обхода путей через символические ссылки

#### C6: Оптимизация sanitize_player_name
- **Файл:** `src/validation/name.rs`
- **Решение:** Объединение двух фильтров в один проход
- **Улучшение:** Снижение количества аллокаций при обработке имён

#### A1: Защита от переполнения очков
- **Файлы:** `src/game/scoring/points.rs`, `src/game/scoring/combo.rs`
- **Решение:** `saturating_mul()` и `saturating_add()` для всех операций с очками

#### A2: TOCTOU защита в таблице лидеров
- **Файл:** `src/highscore/leaderboard.rs`
- **Решение:** Атомарный метод `get_valid_score()` объединяет проверку и получение

#### A3: Битовая маска для заполненных линий
- **Файл:** `src/game/scoring/lines.rs`
- **Решение:** `u32` битовая маска вместо `[bool; GRID_HEIGHT]`
- **Улучшение:** Снижение памяти с 20 байт до 4 байт

---

## 📚 Дополнительная документация

- `docs/ARCHITECTURE.md` — подробная документация
- `README.md` — обзор проекта
- `TESTS_REGISTRY.md` — реестр тестов
- `SECURITY.md` — политика безопасности

---

**Дата последнего обновления:** 29 марта 2026
**Версия проекта:** 23.96.20
