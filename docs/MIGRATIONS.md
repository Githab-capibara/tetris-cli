# Руководство по миграции Tetris CLI

Этот документ описывает изменения в публичном API и предоставляет инструкции по миграции для пользователей библиотеки.

## Версия 23.96.18 (2026-03-31)

### Критические изменения

#### 1. Консолидация типов ошибок

**Что изменилось:**
- Удалён дублирующий `GameError` из `game::state`
- Все ошибки централизованы в модуле `errors`

**Как мигрировать:**
```rust
// БЫЛО:
use tetris_cli::game::state::GameError;

// СТАЛО:
use tetris_cli::errors::GameError;
// или через публичный API:
use tetris_cli::GameError;
```

**Варианты ошибок:**
- `GameError::ValidationError(String)` - ошибки валидации
- `GameError::IoError(std::io::Error)` - ошибки ввода/вывода
- `GameError::ConfigError(String)` - ошибки конфигурации

#### 2. Инкапсуляция Tetromino

**Что изменилось:**
- Все поля `Tetromino` сделаны приватными
- Добавлены публичные геттеры и мутабельные геттеры

**Как мигрировать:**
```rust
// БЫЛО:
let tetromino = Tetromino::from_bag(&mut bag);
let pos = tetromino.pos;
let shape = tetromino.shape;
tetromino.pos.1 += 1.0;

// СТАЛО:
let tetromino = Tetromino::from_bag(&mut bag);
let pos = tetromino.pos();
let shape = tetromino.shape();
tetromino.pos_mut().1 += 1.0;
```

**Доступные геттеры:**
- `pos()` / `pos_mut()` - позиция фигуры
- `shape()` - тип фигуры
- `fg()` - индекс цвета
- `coords()` / `coords_mut()` - координаты блоков

#### 3. Удаление устаревших методов

**Что удалено:**
- `Tetromino::rotate_old()` - удалён полностью

**Как мигрировать:**
```rust
// БЫЛО:
use tetris_cli::types::Direction;
tetromino.rotate_old(Direction::Right);

// СТАЛО:
use tetris_cli::types::RotationDirection;
tetromino.rotate(RotationDirection::Clockwise);
```

### Изменения в модулях

#### 4. GameError в game::access

**Что изменилось:**
- Трейты в `game::access` теперь используют `crate::errors::GameError`

**Как мигрировать:**
```rust
// БЫЛО:
use tetris_cli::game::state::GameError;

// СТАЛО:
use tetris_cli::errors::GameError;
```

#### 5. Типы возврата в game::cycle

**Что изменилось:**
- `run_game_loop()` теперь возвращает `Result<u128, crate::errors::GameError>`

**Как мигрировать:**
```rust
// БЫЛО:
use tetris_cli::game::state::GameError;

// СТАЛО:
use tetris_cli::errors::GameError;
```

## Версия 23.96.15 (2026-03-30)

### Устаревшие элементы

#### 1. Direction → RotationDirection

**Что устарело:**
- `tetris_cli::types::Direction` помечен как устаревший
- Используйте `tetris_cli::types::RotationDirection`

**Как мигрировать:**
```rust
// БЫЛО:
use tetris_cli::types::Direction;
tetromino.rotate(Direction::Left);

// СТАЛО:
use tetris_cli::types::RotationDirection;
tetromino.rotate(RotationDirection::CounterClockwise);
```

**Соответствие направлений:**
| Старое | Новое |
|--------|-------|
| `Direction::Left` | `RotationDirection::CounterClockwise` |
| `Direction::Right` | `RotationDirection::Clockwise` |
| `Direction::Down` | `RotationDirection::NoRotation` |

## Версия 23.96.14 (2026-03-29)

### Изменения в конфигурации управления

#### 1. HMAC подписи ControlsConfig

**Что изменилось:**
- Добавлена поддержка HMAC-SHA256 подписей для конфигурации управления
- Используется глобальный HMAC ключ вместо генерации нового при каждом сохранении

**Как мигрировать:**
```rust
// БЫЛО:
let config = ControlsConfig::default_config();
config.save_to_file("controls.json")?;

// СТАЛО: (без изменений в API)
let config = ControlsConfig::default_config();
config.save_to_file("controls.json")?;
```

**Внутренние изменения:**
- При загрузке проверяется HMAC подпись
- При сохранении используется глобальный ключ из переменной окружения

## Общие рекомендации

### 1. Обновление зависимостей

Убедитесь что ваша версия `tetris-cli` совместима с вашим кодом:

```toml
[dependencies]
tetris-cli = "23.96.18"
```

### 2. Проверка компиляции

После обновления запустите:

```bash
cargo check
cargo test
```

### 3. Обработка ошибок

Все функции теперь используют централизованный `GameError`:

```rust
use tetris_cli::errors::GameError;

fn my_function() -> Result<(), GameError> {
    // Ваш код
}
```

### 4. Работа с фигурами

Используйте геттеры вместо прямого доступа к полям:

```rust
// Правильно:
let pos = tetromino.pos();
let shape = tetromino.shape();

// Неправильно (не компилируется):
let pos = tetromino.pos;  // Ошибка: поле приватное
```

## Поддержка

Если вы столкнулись с проблемами при миграции:

1. Проверьте [документацию](https://docs.rs/tetris-cli)
2. Откройте [issue на GitHub](https://github.com/Githab-capibara/tetris-cli/issues)
3. Изучите [примеры использования](docs/EXAMPLES.md)

## История версий

Полный список изменений доступен в [CHANGELOG.md](CHANGELOG.md).
