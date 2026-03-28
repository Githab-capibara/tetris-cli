# 🏗️ Архитектура Tetris CLI

**Версия:** 1.2
**Дата:** 28 марта 2026 (обновлено)
**Проект:** tetris-cli v23.96.18+

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
│   │   ├── state.rs         # GameState (1171 строка)
│   │   ├── mode_trait.rs    # GameModeTrait
│   │   ├── types.rs         # Типобезопасные обёртки
│   │   ├── constants.rs     # Константы
│   │   ├── view.rs          # GameView для отрисовки
│   │   ├── access.rs        # Трейты доступа
│   │   ├── cache.rs         # Кэширование
│   │   ├── cycle.rs         # Игровой цикл
│   │   ├── render.rs        # Отрисовка
│   │   ├── logic/           # Логика игры
│   │   │   ├── collision.rs
│   │   │   ├── input.rs
│   │   │   ├── physics.rs
│   │   │   ├── rotation.rs
│   │   │   ├── update.rs
│   │   │   └── wall_kick.rs
│   │   └── scoring/         # Система очков
│   │       ├── combo.rs
│   │       ├── lines.rs
│   │       └── points.rs
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
│   ├── tetromino.rs         # Фигуры (698 строк)
│   ├── controls.rs          # Конфигурация управления (757 строк)
│   ├── io.rs                # Ввод/вывод
│   ├── io_traits.rs         # Трейты InputReader, Renderer
│   ├── crypto.rs            # Криптография
│   ├── validation/          # Валидация
│   │   ├── mod.rs
│   │   ├── name.rs
│   │   └── path.rs
│   ├── terminal_backend.rs  # TerminalBackend трейт
│   ├── types.rs             # Direction, RotationDirection
│   └── tests/               # Интеграционные тесты (86 файлов)
├── tests/                   # Integration tests
│   └── test_architecture_integrity.rs
├── docs/
│   └── ARCHITECTURE.md      # Подробная документация (2607 строк)
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

#### GameState (state.rs, 1171 строка)
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
    fall_spd: f32,
    land_timer: f32,
    
    // Кэширование
    render_cache: RenderCache,
}
```

**Инкапсуляция:**
- Геттеры: `get_score()`, `get_level()`, `get_lines_cleared()`
- Сеттеры с валидацией: `set_level()`, `set_fall_spd()`
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

**Размер:** 698 строк (допустимо для Rust)

---

### 6. Controls Module (`controls.rs`)

**Ответственность:** Конфигурация управления

**Компоненты:**
- `ControlsConfig` — конфигурация клавиш
- Валидация (делегирование в `validation/`)
- Сериализация JSON

**Размер:** 757 строк

---

### 7. Validation Module (`validation/`)

**Ответственность:** Валидация данных

**Компоненты:**
- `PathValidator` — валидация путей
- `sanitize_player_name()` — валидация имён

**Принципы:**
- Централизованная валидация
- Типизированные ошибки

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
| **Количество модулей** | 15+ | ✅ |
| **Средний размер модуля** | ~400 строк | ✅ |
| **Крупные модули** | 3 (state, tetromino, controls) | ⚠️ |
| **Циклические зависимости** | 0 | ✅ |
| **Покрытие тестами** | 1085 тестов | ✅ |
| **Публичный API** | Стабильный | ✅ |
| **Меры безопасности** | 8 (constant-time HMAC, UTF-8, path traversal, saturating operations) | ✅ |

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

### Тесты верификации исправлений (`src/tests/test_fixes_verification.rs`)
- **C1**: Безопасный cast в cycle.rs (3 теста)
- **L2**: Объединённые match паттерны (3 теста)
- **L3**: if let обработка ошибок (3 теста)
- **M4**: TODO комментарии и dead_code (2 теста)
- Интеграционные тесты (3 теста)

### Тесты безопасности (`src/tests/test_audit_fixes_comprehensive.rs`)
- **Constant-time HMAC** — защита от timing-атак (1 тест)
- **Валидация UTF-8** — отбрасывание невалидных последовательностей (1 тест)
- **Безопасное вращение** — saturating операции вместо паники (1 тест)
- **Path traversal защита** — canonicalize() перед проверкой (1 тест)
- **Оптимизация sanitize_player_name** — фильтрация в один проход (1 тест)
- **Защита от переполнения времени** — безопасная конвертация u128 → u64 (1 тест)

### Запуск:
```bash
cargo test
cargo test test_architecture_integrity
```

---

## 📝 Принципы проектирования

### SOLID
- **S** — Single Responsibility (соблюдается)
- **O** — Open/Closed (GameModeTrait)
- **L** — Liskov Substitution (трейты)
- **I** — Interface Segregation (BoardReadonly/BoardMutable)
- **D** — Dependency Inversion (TerminalBackend)

### DRY
- Централизованная валидация
- Общие константы (DISP_WIDTH, DISP_HEIGHT в game/constants.rs)
- Криптография в crypto.rs

### KISS
- Простые структуры данных
- Минимум абстракций
- Явные зависимости

### YAGNI
- Удалена зависимость fs2
- Удалены неиспользуемые модули

---

## 🎯 Оценка архитектуры

**Текущая оценка: 8.0/10**

**Сильные стороны:**
- ✅ Модульная структура
- ✅ Отсутствие циклических зависимостей
- ✅ Разделение ответственности
- ✅ Трейты для абстракции
- ✅ Обширное тестирование (1037 тестов)
- ✅ Защита от переполнения (saturating операции)
- ✅ TOCTOU защита в LeaderboardEntry
- ✅ Централизованная валидация путей

**Области улучшения:**
- ⚠️ GameState — God Object (1171 строка)
- ⚠️ tetromino.rs — 698 строк
- ⚠️ controls.rs — 757 строк
- ⚠️ Нарушение инкапсуляции через pub(crate)

---

## 🔒 Безопасность

### Реализованные механизмы защиты:

#### C1: Constant-time comparison для HMAC-SHA256
- **Файл:** `src/crypto.rs`
- **Решение:** XOR-накопление вместо раннего выхода при проверке подписей
- **Защита:** Предотвращение timing-атак при проверке HMAC-SHA256
- **Тест:** `test_hmac_constant_time()`

#### C2: Защита от переполнения времени
- **Файл:** `src/io.rs`, `src/game/cycle.rs`
- **Решение:** Безопасная конвертация `u128 → u64` с проверкой границ
- **Защита:** Предотвращение переполнения при длительных интервалах между кадрами
- **Тест:** `test_time_conversion_safety()`

#### C3: Валидация UTF-8
- **Файл:** `src/io.rs`
- **Решение:** Корректное отбрасывание невалидных UTF-8 последовательностей
- **Защита:** Предотвращение паники при получении невалидного Unicode
- **Тест:** `test_utf8_validation()`

#### C4: Безопасное вращение фигур
- **Файл:** `src/tetromino.rs`
- **Решение:** `saturating_neg()` вместо `assert!` при вращении
- **Защита:** Предотвращение паники при выходе координат за границы
- **Тест:** `test_rotate_bounds_safety()`

#### C5: Path traversal защита
- **Файл:** `src/validation/path.rs`
- **Решение:** `canonicalize()` выполняется перед проверкой нахождения в директории
- **Защита:** Предотвращение обхода путей через символические ссылки
- **Тест:** `test_path_traversal_canonicalize()`

#### C6: Оптимизация sanitize_player_name
- **Файл:** `src/validation/name.rs`
- **Решение:** Объединение двух фильтров в один проход
- **Улучшение:** Снижение количества аллокаций при обработке имён
- **Тест:** `test_sanitize_single_pass()`

#### A1: Защита от переполнения очков
- **Файлы:** `src/game/scoring/points.rs`, `src/game/scoring/combo.rs`
- **Решение:** `saturating_mul()` и `saturating_add()` для всех операций с очками
- **Тест:** `test_score_overflow_protection()`

#### A2: TOCTOU защита в таблице лидеров
- **Файл:** `src/highscore/leaderboard.rs`
- **Решение:** Атомарный метод `get_valid_score()` объединяет проверку и получение
- **Тест:** `test_leaderboard_entry_atomic_validation()`

---

## 📚 Дополнительная документация

- `docs/ARCHITECTURE.md` — подробная документация (2607 строк)
- `README.md` — обзор проекта
- `TESTS_REGISTRY.md` — реестр тестов
- `SECURITY.md` — безопасность

---

**Дата последнего обновления:** 28 марта 2026
**Версия проекта:** 23.96.20
