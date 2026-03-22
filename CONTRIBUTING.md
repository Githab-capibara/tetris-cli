# Руководство для контрибьюторов — Tetris CLI

Благодарим за интерес к проекту Tetris CLI! Это руководство поможет вам внести свой вклад в развитие проекта.

## 📖 Содержание

- [Начало работы](#начало-работы)
- [Сборка проекта](#сборка-проекта)
- [Тестирование](#тестирование)
- [Форматирование кода](#форматирование-кода)
- [Стиль кода](#стиль-кода)
- [Процесс внесения изменений](#процесс-внесения-изменений)
- [Code Review](#code-review)
- [Сообщение об ошибках](#сообщение-об-ошибках)
- [Предложения по улучшению](#предложения-по-улучшению)

---

## 🚀 Начало работы

### Требования

- **Rust** 1.56 или новее
- **Cargo** (поставляется с Rust)
- **Git** для работы с репозиторием
- **Linux/Unix** система (или WSL для Windows)

### Установка Rust

```bash
# Установка Rust (рекомендуется через rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Проверка установки
rustc --version
cargo --version
```

### Клонирование репозитория

```bash
git clone https://github.com/Githab-capibara/tetris-cli.git
cd tetris-cli
```

---

## 🔧 Сборка проекта

### Отладочная сборка

```bash
cargo build
```

### Релизная сборка

```bash
cargo build --release
```

### Запуск игры

```bash
# Из исходников
cargo run

# Из релизной сборки
./target/release/tetris-cli
```

---

## 🧪 Тестирование

### Запуск всех тестов

```bash
cargo test
```

### Запуск тестов с выводом результатов

```bash
cargo test -- --nocapture
```

### Запуск конкретного теста

```bash
cargo test test_tetromino_t_creation
```

### Запуск тестов для конкретного модуля

```bash
cargo test --lib tetromino
```

### Игнорируемые тесты

Некоторые тесты игнорируются по умолчанию (требуют реальный терминал):

```bash
# Запуск игнорируемых тестов
cargo test -- --ignored
```

### Проверка покрытия тестов

```bash
# Установка cargo-tarpaulin
cargo install cargo-tarpaulin

# Запуск с покрытием
cargo tarpaulin --out Html
```

### Статистика тестов

Проект содержит **1548 модульных и интеграционных тестов**, покрывающих все компоненты:

```
═══════════════════════
ВСЕГО: 1548 тестов
ВСЕ ПРОХОДЯТ: ✅ (3 пропущены)
═══════════════════════
```

### Бенчмарки производительности

Проект использует фреймворк **criterion** для бенчмарков:

```bash
# Запуск бенчмарков
cargo bench
```

**Примеры бенчмарков:**
- `check_rows()` — проверка и удаление заполненных линий
- `rotate()` — вращение фигур
- `draw_simulation()` — симуляция отрисовки

---

## 📝 Форматирование кода

### Автоматическое форматирование

```bash
# Форматирование всего проекта
cargo fmt

# Проверка форматирования (без изменений)
cargo fmt -- --check
```

### Форматирование конкретного файла

```bash
rustfmt src/game.rs
```

---

## 🔍 Линтинг и статический анализ

### Запуск Clippy

```bash
# Проверка всего проекта
cargo clippy

# Проверка с тестами
cargo clippy --all-targets

# Проверка с автоисправлением
cargo clippy --fix
```

### Конфигурация Clippy

Проект использует настройки по умолчанию. Запрещены:
- `unwrap()` в production коде (используйте `expect()` или обработку ошибок)
- `panic!` в production коде
- Неиспользуемые импорты и переменные

---

## 📖 Стиль кода

### Требования к коду

1. **Идиоматичный Rust** — используйте возможности языка правильно
2. **Читаемость** — код должен быть понятным
3. **Безопасность** — избегайте `unwrap()`, обрабатывайте ошибки
4. **Производительность** — оптимизируйте критические участки
5. **Документированность** — все публичные API должны иметь документацию
6. **Тестируемость** — новые функции должны сопровождаться тестами

### Новые требования (версия 23.96.11)

- **Атрибуты компилятора**: используйте `#[must_use]` для методов, результат которых должен быть использован
- **Атрибуты компилятора**: используйте `#[allow(dead_code)]` для методов, требуемых API
- **Возвращаемые типы**: предпочитайте простые ссылки `&T` вместо `&Box<T>`
- **Derive макросы**: используйте `#[derive(Default)]` для структур с полями, имеющими Default
- **Assert проверки**: используйте `assert!` вместо `debug_assert!` для критических проверок
- **Комментарии**: добавляйте комментарии для важных ограничений (например, UTF-8)
- **Массивы на стеке**: используйте `[[i8; GRID_WIDTH]; GRID_HEIGHT]` вместо `Box<[[i8; GRID_WIDTH]; GRID_HEIGHT]>`
- **Битовые маски**: используйте `u32` для флагов вместо `[bool; N]`
- **Бенчмарки**: добавляйте бенчмарки criterion для критичных к производительности функций
- **Оптимизация строк**: используйте `String::with_capacity()` + `write!()` вместо `format!()`
- **Оптимизация векторов**: используйте `reserve()` + `extend_from_slice()` для уменьшения аллокаций
- **Именованные константы**: выносите магические числа в константы (PREVIEW_X, HOLD_PREVIEW_Y, etc.)
- **Валидация путей**: используйте `canonicalize()` для защиты от Path Traversal
- **Whitelist валидация**: используйте whitelist для валидации имён и других входных данных

### Именование

- **Функции и переменные**: `snake_case`
- **Типы и структуры**: `PascalCase`
- **Константы**: `UPPER_SNAKE_CASE`
- **Трейты**: `PascalCase`

```rust
// Пример
pub const MAX_SCORE: u64 = 1000;

pub struct GameState {
    pub score: u64,
}

pub fn calculate_score() -> u64 {
    // ...
}
```

### Документация

Все публичные API должны быть документированы:

```rust
/// Краткое описание функции.
///
/// Подробное описание с примерами использования.
///
/// # Аргументы
/// * `name` - описание аргумента
///
/// # Возвращает
/// Описание возвращаемого значения
///
/// # Пример
/// ```
/// let result = function_name(arg);
/// ```
pub fn function_name(name: &str) -> String {
    // ...
}
```

### Обработка ошибок

**Предпочтительно:**
```rust
// Использование Result
pub fn load_config() -> Result<Config, Error> {
    // ...
}

// Использование Option
pub fn find_piece(id: u32) -> Option<Piece> {
    // ...
}
```

**Избегайте:**
```rust
// unwrap() в production коде
let value = some_option.unwrap(); // ❌

// panic! для обработки ошибок
if error { panic!("Error"); } // ❌
```

**Допустимо:**
```rust
// expect() с понятным сообщением
let value = some_option.expect("Описание причины"); // ✅

// unwrap() в тестах
#[test]
fn test_example() {
    let value = get_value().unwrap(); // ✅ (в тестах допустимо)
}

// allow(dead_code) для API методов
#[allow(dead_code)]
pub fn hash(&self) -> &str { // ✅ (требуется API)
    &self.hash
}
```

### Комментарии

- Комментарии на **русском языке**
- Объясняйте **почему**, а не **что**
- Избегайте избыточных комментариев

```rust
// ✅ Хорошо: объясняет причину
// Используем Fisher-Yates для равномерного перемешивания
for i in (1..bag.len()).rev() {
    let j = rng.gen_range(0..=i);
    bag.swap(i, j);
}

// ❌ Плохо: очевидное
// Увеличиваем i на 1
i += 1;
```

### Примеры тестов для новых функций

**Тест для метода `verify_and_get_score()`:**
```rust
#[test]
fn test_verify_and_get_score_valid() {
    let save_data = SaveData::from_value(1000);
    let score = save_data.verify_and_get_score();
    assert_eq!(score, Some(1000));
}

#[test]
fn test_verify_and_get_score_invalid() {
    let mut save_data = SaveData::from_value(1000);
    save_data.high_score = 9999; // Подделка
    let score = save_data.verify_and_get_score();
    assert_eq!(score, None);
}
```

**Тест для замены assert_hs():**
```rust
#[test]
fn test_deprecated_assert_hs_replaced() {
    let save_data = SaveData::from_value(500);
    // Новый метод вместо assert_hs()
    let score = save_data.verify_and_get_score().unwrap_or(0);
    assert_eq!(score, 500);
}
```

**Тест для проверки границ вращения:**
```rust
#[test]
fn test_rotation_bounds_check() {
    let mut state = GameState::new();
    // Проверка что check_y < 0 обрабатывается корректно
    state.check_rotation_collision(Dir::Left, -1);
    // Тест должен проходить без паники
}
```

**Тест для защиты от переполнения счёта:**
```rust
#[test]
fn test_score_overflow_protection() {
    let mut state = GameState::new();
    // Проверка что infinity/NaN не ломают счёт
    state.score = u64::MAX;
    state.add_score(100); // Не должно вызвать переполнение
    assert!(state.score.is_finite());
}
```

**Тест для Path Traversal защиты:**
```rust
#[test]
fn test_path_traversal_protection() {
    let config = ControlsConfig::default();
    // Проверка что путь с ".." отклоняется
    let result = config.save_to_file("../etc/passwd");
    assert!(result.is_err());
}
```

**Тест для #[must_use] атрибутов:**
```rust
#[test]
fn test_must_use_attributes() {
    let leaderboard = Leaderboard::default();
    // Компилятор должен предупреждать, если результат не используется
    let _len = leaderboard.len(); // OK: результат используется
    leaderboard.len(); // Warning: результат не используется
}
```

**Тест для оптимизации format!() -> write!():**
```rust
#[test]
fn test_string_optimization() {
    // Проверка что используется String::with_capacity() + write!()
    let mut s = String::with_capacity(32);
    write!(&mut s, "test").unwrap();
    assert_eq!(s, "test");
}
```

**Тест для оптимизации BagGenerator::fill_bag():**
```rust
#[test]
fn test_bag_generator_optimization() {
    let mut bag = BagGenerator::new();
    bag.fill_bag();
    // Проверка что reserve() + extend_from_slice() работают
    assert_eq!(bag.get_bag().len(), 7);
}
```

**Тест для битовой маски в `check_rows()`:**
```rust
#[test]
fn test_bitmask_check_rows() {
    let mut state = GameState::new();
    state.fill_line_for_bench(10); // Заполнить линию
    let full_rows = state.clear_lines_for_bench();
    assert_eq!(full_rows, 1);
}
```

**Бенчмарк для `check_rows()`:**
```rust
fn bench_check_rows(c: &mut Criterion) {
    let mut state = GameState::new();
    state.fill_line_for_bench(10);
    c.bench_function("check_rows", |b| {
        b.iter(|| state.clear_lines_for_bench())
    });
}
```

---

## 🔄 Процесс внесения изменений

### 1. Создайте issue

Перед началом работы создайте issue с описанием изменений:
- Для новых функций: предложите описание и обоснование
- Для исправлений: опишите проблему и шаги воспроизведения

### 2. Форкните репозиторий

```bash
git fork https://github.com/Githab-capibara/tetris-cli
```

### 3. Создайте ветку

```bash
git checkout -b feature/your-feature-name
# или
git checkout -b fix/issue-number-description
```

### 4. Внесите изменения

- Следуйте стилю кода проекта
- Добавьте тесты для новых функций
- Обновите документацию

### 5. Запустите тесты

```bash
# Убедитесь, что все тесты проходят
cargo test

# Проверьте линтер
cargo clippy --all-targets

# Проверьте форматирование
cargo fmt -- --check
```

### 6. Сделайте коммит

```bash
git add .
git commit -m "feat: добавить новую функцию

- Описание изменений
- Ссылка на issue (если есть)"
```

**Формат коммита:**
- `feat:` — новая функция
- `fix:` — исправление ошибки
- `docs:` — документация
- `style:` — форматирование
- `refactor:` — рефакторинг
- `test:` — тесты
- `chore:` — вспомогательные изменения

### 7. Отправьте Pull Request

```bash
git push origin feature/your-feature-name
```

Создайте Pull Request через GitHub UI.

---

## 👀 Code Review

### Требования к PR

- [ ] Все тесты проходят (1548 тестов)
- [ ] Clippy не выдаёт предупреждений
- [ ] Код отформатирован
- [ ] Документация обновлена
- [ ] Добавлены тесты для новых функций
- [ ] Комментарии на русском языке
- [ ] Соблюдены новые требования к коду (версия 23.96.12)
- [ ] Добавлены атрибуты `#[must_use]` где необходимо
- [ ] Использованы оптимизации (`String::with_capacity()`, `reserve()`)
- [ ] Проведена валидация всех входных данных

### Процесс review

1. **Автоматическая проверка**: CI запускает тесты и линтеры
2. **Review мейнтейнера**: проверка кода и архитектуры
3. **Исправление замечаний**: внесение изменений по feedback
4. **Merge**: слияние в основную ветку

### Время review

- Обычные PR: 2-5 дней
- Критические исправления: 1-2 дня
- Большие изменения: 1-2 недели

---

## 🐛 Сообщение об ошибках

### Шаблоны issue для багов

**Заголовок:** `Bug: краткое описание`

**Содержание:**
```markdown
### Описание
Чёткое описание ошибки

### Воспроизведение
Шаги для воспроизведения:
1. Запустить команду '...'
2. Нажать клавишу '...'
3. Увидеть ошибку

### Ожидаемое поведение
Что должно было произойти

### Скриншоты/Логи
```

### Критические ошибки

Для критических ошибок (потеря данных, безопасность):
- Создайте issue с меткой `critical`
- Или напишите напрямую мейнтейнеру

---

## 💡 Предложения по улучшению

### Шаблоны issue для предложений

**Заголовок:** `Feature: описание функции`

**Содержание:**
```markdown
### Проблема
Какую проблему решает функция

### Решение
Описание предлагаемого решения

### Альтернативы
Рассмотренные альтернативные варианты

### Дополнительный контекст
Скриншоты, примеры, референсы
```

---

## 📞 Контакты

### Вопросы

- **GitHub Issues**: для вопросов по проекту
- **Discussions**: для общих обсуждений

### Мейнтейнеры

- [@Githab-capibara](https://github.com/Githab-capibara)

---

## 📜 Лицензия

Внося изменения, вы соглашаетесь с лицензией **GPL-3.0**.

---

## 🙏 Благодарности

Спасибо за ваш вклад в развитие Tetris CLI! 🎮

Каждый PR делает проект лучше для всех пользователей.
