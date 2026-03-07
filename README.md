# Tetris CLI 🎮

Классическая игра Тетрис для терминала Linux/Unix, написанная на Rust.

![Скриншот игры](docs/img/screenshot.png)

## 📖 Описание

**Tetris CLI** — это полноценная реализация легендарной игры Тетрис, работающая прямо в вашем терминале. Игра поддерживает все классические механики: вращение фигур, мгновенное падение, подсчёт очков и сохранение рекордов.

### Особенности

- ✨ **Классический геймплей** — все 7 типов тетрамино (T, L, J, S, Z, O, I)
- 🎨 **Цветная графика** — каждая фигура имеет свой цвет
- 🏆 **Система рекордов** — лучший результат сохраняется между запусками
- ⚡ **Плавная анимация** — 60 FPS для комфортной игры
- 🎯 **Прогрессивная сложность** — скорость падения увеличивается с каждым уровнем
- ⏸️ **Пауза** — возможность приостановить игру в любой момент
- 👻 **Призрачная фигура** — показывает, куда упадёт фигура
- 👀 **Предпросмотр** — показ следующей фигуры
- 📊 **Таблица лидеров** — топ-5 лучших результатов
- 🔄 **Удержание фигуры (Hold)** — отложите фигуру для последующего использования
- 🔊 **Звуковые эффекты** — терминальный bell при удалении линий
- 📈 **Статистика игры** — подсчёт фигур, комбо, время игры
- 🏃 **Режим спринт** — очистите 40 линий как можно быстрее
- 🎮 **Настраиваемое управление** — возможность изменить все клавиши управления

## 🚀 Установка

### Через cargo (рекомендуется)

```bash
cargo install tetris-cli
```

### Из исходников

```bash
git clone https://github.com/Githab-capibara/tetris-cli.git
cd tetris-cli
cargo build --release
cargo run
```

### Пакетные менеджеры

- **Arch Linux (AUR)**: [tetris-cli-git](https://aur.archlinux.org/packages/tetris-terminal-git)
- **NetBSD**: [tetris-cli](https://pkgsrc.se/games/tetris-cli)

## 🎮 Управление

### Клавиши по умолчанию

| Клавиша | Действие | Описание |
|---------|----------|----------|
| `a` | Влево | Перемещение фигуры влево |
| `d` | Вправо | Перемещение фигуры вправо |
| `q` | Вращение ↺ | Поворот против часовой стрелки |
| `e` | Вращение ↻ | Поворот по часовой стрелке |
| `w` | Hard Drop | Мгновенное падение (2 очка/ячейку) |
| `s` | Soft Drop | Ускоренное падение (1 очко/ячейку) |
| `c` | Hold | Удержать фигуру на потом |
| `p` | Pause | Пауза игры |
| `Backspace` | Выход | Вернуться в меню |

### Настройка управления

Все клавиши можно перенастроить через `ControlsConfig`:

```rust
use tetris_cli::controls::ControlsConfig;

// Загрузка сохранённой конфигурации
let config = ControlsConfig::load_from_file("my_controls.json").unwrap();

// Или создание новой (стиль Vim - HJKL)
let vim_config = ControlsConfig::custom(
    b'h', // влево
    b'l', // вправо
    b'j', // мягкое падение
    b'k', // жёсткое падение
    b'y', // вращение влево
    b'u', // вращение вправо
    b'i', // удержание
    b'o', // пауза
    127,  // выход
);
```

## 📋 Системные требования

- **ОС**: Linux или другая Unix-подобная система
- **Терминал**: Поддерживающий raw-режим и ANSI-цвета
- **Минимальный размер окна**: 22×25 символов
- **Шрифт**: Моноширинный (рекомендуется)

> ⚠️ **Важно**: Игра не работает в Windows Terminal. Для запуска в Windows используйте WSL (Windows Subsystem for Linux).

### Рекомендуемые терминалы

- GNOME Terminal
- Konsole
- Alacritty
- Kitty
- iTerm2 (macOS)

## 🎯 Правила игры

### Начисление очков

#### Базовые очки

| Действие | Формула | Пример |
|----------|---------|--------|
| Падение фигуры | 100 + (скорость × 50) | ~145 очков |
| Soft Drop | 1 × ячейки | 5 ячеек = 5 очков |
| Hard Drop | 2 × ячейки | 10 ячеек = 20 очков |

#### Очки за линии (с экспоненциальным бонусом)

| Линий | Формула | Очки | Название |
|-------|---------|------|----------|
| 1 | 100 × 2⁰ | 100 | Одиночный |
| 2 | 100 × 2¹ | 200 | Двойной |
| 3 | 100 × 2² | 400 | Тройной |
| 4 | 100 × 2³ + 1000 | 1800 | **TETRIS!** |

#### Бонусы

| Бонус | Условие | Очки |
|-------|---------|------|
| Комбо | 50 × (комбо - 1) | до 450+ |
| Уровень | 500 × (уровень - 1) | до 5000+ |
| Tetris | 4 линии одновременно | +1000 |

### Прогрессия сложности

- За каждую заполненную линию скорость падения увеличивается на 0.05
- Начальная скорость: 0.9 блоков/секунду
- Уровень повышается каждые 10 удалённых линий

| Уровень | Скорость | Линий для перехода |
|---------|----------|-------------------|
| 1 | 0.90 | 0-9 |
| 2 | 0.95 | 10-19 |
| 3 | 1.00 | 20-29 |
| 4 | 1.05 | 30-39 |
| 5 | 1.10 | 40-49 |
| ... | ... | ... |

### Проигрыш

Игра заканчивается, когда новая фигура не может появиться на поле (заблокирована другими фигурами).

## 🆕 Новые функции (версия 23.96.3)

### 🔄 Удержание фигуры (Hold)

Нажмите **`c`**, чтобы отложить текущую фигуру. Вы можете использовать удержание один раз за ход. При повторном нажатии `c` удержанная фигура поменяется местами с текущей.

**Особенности:**
- Удержанная фигура отображается слева от игрового поля
- Можно использовать только один раз за ход
- При удержании позиция фигуры сбрасывается к центру

### 🔊 Звуковые эффекты

При удалении линий воспроизводится терминальный bell-сигнал. Это даёт звуковую обратную связь об успешном удалении линий.

### 📈 Статистика игры

После завершения игры отображается подробная статистика:
- **Время игры** — общее время с начала
- **Количество фигур** — сколько фигур каждого типа использовано
- **Максимальное комбо** — наибольшее количество одновременных линий
- **Режим игры** — классический или спринт

### 🏃 Режим спринт

Нажмите **`r`** в главном меню для запуска режима спринт.

**Цель:** Очистить 40 линий как можно быстрее.

**Особенности:**
- Отображается таймер времени
- Показывается прогресс (X/40 линий)
- Результат не сохраняется в таблицу лидеров
- После завершения показывается статистика

## 📖 Стратегии игры

### Советы для новичков

1. **Держите поле плоским** — избегайте высоких столбцов
2. **Оставляйте колодец** — оставьте одну колонку пустой для фигуры-линии (I)
3. **Планируйте наперёд** — смотрите на следующую фигуру
4. **Используйте вращение** — некоторые позиции доступны только через вращение
5. **Используйте Hold** — откладывайте ненужные фигуры для будущих комбинаций

### Продвинутые техники

#### T-Spin
Вращение T-фигуры в последнюю минуту для размещения в узких местах. Позволяет набирать больше очков.

#### Комбо
Постоянное удаление линий создаёт цепную реакцию очков. Старайтесь оставлять несколько линий для последовательного удаления.

#### Удержание центра
Старайтесь держать самые высокие блоки в центре поля для большей гибкости.

#### Использование призрачной фигуры
Призрачная фигура (полупрозрачная) показывает, куда упадет текущая фигура. Используйте это для точного позиционирования.

## 🏗️ Архитектура

### Обзор

Tetris CLI использует модульную архитектуру с чётким разделением ответственности между компонентами.

### Диаграмма архитектуры

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│  • Точка входа в приложение                                 │
│  • Отображение главного меню                                │
│  • Инициализация Canvas и KeyReader                         │
│  • Управление циклом меню и запуском игры                   │
│  • Таблица лидеров (топ-5)                                  │
│  • Выбор режима игры (Классика/Спринт)                      │
│  • Ввод имени игрока                                        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        game.rs                              │
│  • Основной игровой цикл (GameState)                        │
│  • Обработка ввода во время игры                            │
│  • Логика падения фигур                                     │
│  • Проверка и удаление заполненных линий                    │
│  • Отрисовка игрового поля                                  │
│  • Система очков и уровней                                  │
│  • Призрачная фигура (точка приземления)                    │
│  • Предпросмотр следующей фигуры                            │
│  • Удержание фигуры (Hold)                                  │
│  • Статистика игры (GameStats)                              │
│  • Режим спринт (Sprint)                                    │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
┌───────────────────┐ ┌───────────────────┐ ┌───────────────────┐
│   tetromino.rs    │ │      io.rs        │ │   highscore.rs    │
│                   │ │                   │ │                   │
│ • Типы фигур      │ │ • Canvas          │ │ • SaveData        │
│ • Координаты      │ │ • Отрисовка       │ │ • Leaderboard     │
│ • Вращение        │ │ • Ввод            │ │ • Загрузка        │
│ • Цвета           │ │ • Терминал        │ │ • Сохранение      │
│ • Bag Generator   │ │                   │ │ • Хеширование     │
│ • 25 тестов       │ │ • 10 тестов       │ │ • 15 тестов       │
└───────────────────┘ └───────────────────┘ └───────────────────┘
                              │
                              ▼
                    ┌───────────────────┐
                    │   controls.rs     │
                    │                   │
                    │ • ControlsConfig  │
                    │ • Настройки       │
                    │ • Валидация       │
                    │ • 20 тестов       │
                    └───────────────────┘
```

### Модули

#### main.rs

**Назначение**: Точка входа в приложение и управление состоянием меню.

**Основные компоненты**:
- `MENU` — массив строк для отображения главного меню
- `LEADERBOARD_MENU` — меню таблицы лидеров
- `main()` — инициализация терминала, загрузка рекорда, цикл меню
- `get_player_name()` — запрос имени для таблицы лидеров
- `show_leaderboard()` — отображение топ-5 рекордов
- `show_game_stats()` — показ статистики после игры

**Поток выполнения**:
1. Загрузка рекорда из конфигурации
2. Загрузка и валидация таблицы лидеров
3. Проверка размера терминала
4. Инициализация Canvas и KeyReader
5. Цикл отрисовки меню (60 FPS)
6. Обработка нажатий:
   - `Enter` — запуск классической игры
   - `R` — запуск режима спринт
   - `L` — отображение таблицы лидеров
   - `Backspace` — выход
7. После игры: отображение статистики, сохранение рекорда

#### game.rs

**Назначение**: Реализация игровой логики и цикла игры.

**Основные компоненты**:
- `GameState` — структура состояния игры
  - `score` — текущий счёт
  - `level` — уровень
  - `lines_cleared` — количество удалённых линий
  - `curr_shape` — текущая фигура
  - `next_shape` — следующая фигура
  - `held_shape` — удержанная фигура
  - `can_hold` — флаг возможности удержания
  - `fall_spd` — скорость падения
  - `blocks` — сетка игрового поля (10×20)
  - `land_timer` — таймер приземления фигуры
  - `stats` — статистика игры
  - `mode` — режим игры
- `GameStats` — статистика игры
- `GameMode` — режимы игры (Classic/Sprint)
- `Dir` — направления движения (Down, Left, Right)
- `UpdateEndState` — состояния завершения обновления

**Игровой цикл**:
1. Поддержание стабильного FPS (60)
2. Обработка ввода пользователя
3. Обновление состояния фигуры
4. Проверка приземления
5. Отрисовка кадра

**Методы GameState**:
- `new()` — создание нового состояния игры
- `new_sprint()` — создание состояния для режима спринт
- `play()` — запуск игрового цикла
- `update()` — обновление состояния за один кадр
- `save_tetromino()` — фиксация фигуры в сетке
- `check_rows()` — проверка и удаление заполненных линий
- `draw()` — отрисовка текущего состояния
- `can_move_curr_shape()` — проверка возможности движения
- `can_rotate_curr_shape()` — проверка возможности вращения
- `hold_shape()` — удержание фигуры

#### tetromino.rs

**Назначение**: Определение типов фигур и их поведения.

**Типы фигур (ShapeType)**:

| Тип | Название | Цвет | Координаты |
|-----|----------|------|------------|
| T | T-образная | Пурпурный (Magenta) | (-1,0), (0,0), (1,0), (0,1) |
| L | L-образная | Жёлтый (Yellow) | (-1,-1), (0,-1), (0,0), (0,1) |
| J | J-образная | Синий (Blue) | (1,-1), (0,-1), (0,0), (0,1) |
| S | S-образная | Зелёный (Green) | (0,-1), (0,0), (1,0), (1,1) |
| Z | Z-образная | Светло-красный (LightRed) | (0,-1), (0,0), (-1,0), (-1,1) |
| O | Квадрат | Светло-жёлтый (LightYellow) | (0,0), (1,0), (0,1), (1,1) |
| I | Линия | Голубой (Cyan) | (0,-1), (0,0), (0,1), (0,2) |

**Bag Generator**:
- Система 7-bag гарантирует равномерное распределение фигур
- Каждые 7 фигур содержат все 7 типов
- Использует алгоритм Fisher-Yates для перемешивания

**Методы Tetromino**:
- `select()` — случайный выбор новой фигуры
- `from_bag()` — создание фигуры из Bag Generator
- `rotate()` — вращение фигуры на 90°

#### io.rs

**Назначение**: Абстракция ввода-вывода для терминала.

**Константы**:
- `SHAPE_STR` = "██" — строковое представление блока
- `SHAPE_WIDTH` = 2 — ширина блока в символах
- `GRID_WIDTH` = 10 — ширина игрового поля
- `GRID_HEIGHT` = 20 — высота игрового поля
- `DISP_WIDTH` = 22 — полная ширина дисплея
- `DISP_HEIGHT` = 25 — полная высота дисплея

**Структуры**:
- `Canvas` — обёртка над RawTerminal для отрисовки
  - `new()` — инициализация терминала
  - `reset()` — сброс терминала
  - `draw_strs()` — отрисовка статических строк
  - `draw_string()` — отрисовка динамической строки
  - `flush()` — обновление экрана
- `KeyReader` — асинхронный читатель клавиатуры
  - `new()` — создание читателя
  - `get_key()` — получение кода клавиши

#### highscore.rs

**Назначение**: Сохранение и загрузка рекордов с защитой от подделки.

**Структура SaveData**:
- `high_score` — значение рекорда
- `high_score_salt` — случайная соль для хеширования
- `high_score_hash` — хеш рекорда с солью

**Структура LeaderboardEntry**:
- `name` — имя игрока
- `score` — значение рекорда
- `salt` — случайная соль
- `hash` — хеш записи

**Структура Leaderboard**:
- `entries` — вектор записей (максимум 5)

**Методы**:
- `load_config()` — загрузка из файла конфигурации
- `from_value()` — создание новой записи с хешем
- `save_value()` — сохранение рекорда
- `assert_hs()` — проверка целостности рекорда
- `get_random_hash()` — генерация случайной соли
- `get_hash()` — вычисление хеша строки
- `add_score()` — добавление рекорда в таблицу
- `validate()` — валидация записей
- `save()` — сохранение таблицы лидеров

**Защита от подделки**:
Рекорд сохраняется вместе с криптографической солью и хешем. При загрузке хеш пересчитывается и сравнивается с сохранённым. Если хеши не совпадают, рекорд сбрасывается в 0.

#### controls.rs

**Назначение**: Конфигурация управления игрой.

**Структура ControlsConfig**:
- `move_left` — движение влево (по умолчанию 'a')
- `move_right` — движение вправо (по умолчанию 'd')
- `soft_drop` — мягкое падение (по умолчанию 's')
- `hard_drop` — жёсткое падение (по умолчанию 'w')
- `rotate_left` — вращение влево (по умолчанию 'q')
- `rotate_right` — вращение вправо (по умолчанию 'e')
- `hold` — удержание фигуры (по умолчанию 'c')
- `pause` — пауза (по умолчанию 'p')
- `quit` — выход (по умолчанию Backspace)

**Методы**:
- `default_config()` — конфигурация по умолчанию
- `custom()` — создание кастомной конфигурации
- `validate()` — проверка на дубликаты и недопустимые значения
- `save_to_file()` — сохранение в JSON файл
- `load_from_file()` — загрузка из JSON файла

### Поток данных

```
Пользователь
    │
    ▼
KeyReader.get_key() ──► game.rs.update() ──► GameState
    │                         │
    │                         ▼
    │                  game.rs.draw()
    │                         │
    │                         ▼
    └──────────────────► Canvas.draw_*()
                              │
                              ▼
                        Терминал
```

### Игровой цикл (детали)

#### Частота обновления
- **FPS**: 60 кадров в секунду
- **Интервал кадра**: ~16.67 мс

#### Физика падения
- Начальная скорость: 0.9 блоков/секунду
- Прирост скорости: +0.05 за каждую заполненную линию
- Плавное движение с интерполяцией по времени

#### Таймер приземления
- Задержка после касания: 0.1 секунды
- Даёт время на перемещение фигуры перед фиксацией

## 🔧 Сборка

### Требования

- Rust 1.56 или новее
- Cargo

### Команды сборки

```bash
# Отладочная сборка
cargo build

# Релизная сборка
cargo build --release

# Запуск
cargo run

# Запуск тестов
cargo test

# Проверка кода
cargo clippy

# Форматирование кода
cargo fmt
```

## 📁 Структура проекта

```
tetris-cli/
├── src/
│   ├── main.rs          # Точка входа, меню, таблица лидеров
│   ├── game.rs          # Игровая логика, уровни, призрачная фигура
│   ├── tetromino.rs     # Фигуры, вращение, Bag Generator
│   ├── io.rs            # Ввод/вывод, терминал
│   ├── highscore.rs     # Рекорды, таблица лидеров
│   ├── controls.rs      # Конфигурация управления
│   └── testes/
│       ├── mod.rs
│       ├── test_controls.rs      # 20 тестов
│       ├── test_game_logic.rs    # 30 тестов
│       ├── test_tetromino.rs     # 25 тестов
│       ├── test_highscore.rs     # 15 тестов
│       ├── test_io.rs            # 10 тестов
│       └── test_integration.rs   # 20 тестов
├── docs/
│   ├── ARCHITECTURE.md  # Архитектура проекта
│   ├── USER_GUIDE.md    # Руководство пользователя
│   └── img/
│       └── screenshot.png
├── Cargo.toml           # Зависимости проекта
├── LICENSE              # Лицензия GPL-3.0
├── CHANGELOG.md         # История изменений
└── README.md            # Этот файл
```

## 🧪 Тестирование

Проект содержит **307 модульных и интеграционных тестов**, покрывающих все компоненты:

### Группы тестов

1. **Tetromino (25 тестов)**: создание фигур, вращение, координаты, Bag Generator
2. **Game Logic (30 тестов)**: движение, столкновения, вращение, система очков, уровни
3. **Highscore (15 тестов)**: SaveData, Leaderboard, хеширование, валидация
4. **IO (10 тестов)**: Canvas, KeyReader, константы размеров
5. **Controls (20 тестов)**: конфигурация управления, валидация, сохранение/загрузка
6. **Integration (20 тестов)**: взаимодействие компонентов, производительность
7. **Achievements (20 тестов)**: система достижений, проверка, получение
8. **Physics (20 тестов)**: физическая механика, гравитация, столкновения, вращение, hold, bag generator

### Запуск тестов

```bash
cargo test
```

### Статистика тестов

```
running 220 тестов в lib.rs (включая 20 тестов physics)
running 56 тестов в bin.rs
running 31 doctest
═══════════════════════
ВСЕГО: 307 тестов
ВСЕ ПРОХОДЯТ: ✅
```

### Покрытие тестов

```
running 120 tests
test testes::test_controls::test_controls_default_config_values ... ok
test testes::test_controls::test_controls_default_trait ... ok
test testes::test_controls::test_controls_default_values_in_range ... ok
test testes::test_controls::test_controls_all_unique ... ok
test testes::test_controls::test_controls_clone ... ok
test testes::test_controls::test_controls_validation_valid_config ... ok
test testes::test_controls::test_controls_validation_duplicate_keys ... ok
test testes::test_controls::test_controls_validation_zero_values ... ok
test testes::test_controls::test_controls_validation_custom_valid ... ok
test testes::test_controls::test_controls_validation_max_values ... ok
test testes::test_controls::test_controls_save_to_file ... ok
test testes::test_controls::test_controls_load_from_file ... ok
test testes::test_controls::test_controls_load_nonexistent_file ... ok
test testes::test_controls::test_controls_save_load_cycle ... ok
test testes::test_controls::test_controls_save_load_special_chars ... ok
test testes::test_controls::test_controls_custom_vim_style ... ok
test testes::test_controls::test_controls_custom_numpad_style ... ok
test testes::test_controls::test_controls_custom_differs_from_default ... ok
test testes::test_controls::test_controls_boundary_min_values ... ok
test testes::test_controls::test_controls_boundary_max_values ... ok
test testes::test_game_logic::test_game_state_creation ... ok
test testes::test_game_logic::test_game_state_initial_piece_position ... ok
test testes::test_game_logic::test_game_state_next_shape_exists ... ok
test testes::test_game_logic::test_game_state_empty_field ... ok
test testes::test_game_logic::test_game_state_initial_fall_speed ... ok
test testes::test_game_logic::test_game_state_default_mode ... ok
test testes::test_game_logic::test_collision_left_boundary ... ok
test testes::test_game_logic::test_collision_right_boundary ... ok
test testes::test_game_logic::test_collision_floor ... ok
test testes::test_game_logic::test_collision_with_fixed_blocks ... ok
test testes::test_game_logic::test_movement_in_empty_field ... ok
test testes::test_game_logic::test_ghost_piece_boundary ... ok
test testes::test_game_logic::test_tetromino_rotate_clockwise ... ok
test testes::test_game_logic::test_tetromino_rotate_counter_clockwise ... ok
test testes::test_game_logic::test_tetromino_o_no_rotate ... ok
test testes::test_game_logic::test_tetromino_full_rotation_cycle ... ok
test testes::test_game_logic::test_all_tetromino_rotate ... ok
test testes::test_game_logic::test_piece_score_constant ... ok
test testes::test_game_logic::test_soft_drop_points_constant ... ok
test testes::test_game_logic::test_hard_drop_points_constant ... ok
test testes::test_game_logic::test_line_score_calculation ... ok
test testes::test_game_logic::test_combo_bonus_constant ... ok
test testes::test_game_logic::test_lines_per_level_constant ... ok
test testes::test_game_logic::test_level_calculation_from_lines ... ok
test testes::test_game_logic::test_speed_increase_constant ... ok
test testes::test_game_logic::test_speed_calculation_from_level ... ok
test testes::test_game_logic::test_sprint_mode_creation ... ok
test testes::test_game_logic::test_sprint_lines_constant ... ok
test testes::test_game_logic::test_sprint_timer ... ok
test testes::test_game_logic::test_game_stats_in_different_modes ... ok
test testes::test_highscore::test_save_data_from_value ... ok
test testes::test_highscore::test_save_data_default ... ok
test testes::test_highscore::test_save_data_assert_hs_valid ... ok
test testes::test_highscore::test_save_data_clone ... ok
test testes::test_highscore::test_save_data_different_values ... ok
test testes::test_highscore::test_leaderboard_empty ... ok
test testes::test_highscore::test_leaderboard_add_score ... ok
test testes::test_highscore::test_leaderboard_add_multiple_scores ... ok
test testes::test_highscore::test_leaderboard_max_size ... ok
test testes::test_highscore::test_leaderboard_sorting ... ok
test testes::test_highscore::test_leaderboard_entry_hash ... ok
test testes::test_highscore::test_leaderboard_entry_salt_unique ... ok
test testes::test_highscore::test_hash_different_values ... ok
test testes::test_highscore::test_leaderboard_entry_validation ... ok
test testes::test_highscore::test_leaderboard_get_best_score ... ok
test testes::test_integration::test_full_game_initialization ... ok
test testes::test_integration::test_sprint_game_initialization ... ok
test testes::test_integration::test_piece_movement_cycle ... ok
test testes::test_integration::test_piece_drop_to_floor ... ok
test testes::test_integration::test_rotation_in_game_context ... ok
test testes::test_integration::test_game_state_tetromino_interaction ... ok
test testes::test_integration::test_game_state_bag_generator_interaction ... ok
test testes::test_integration::test_game_state_leaderboard_interaction ... ok
test testes::test_integration::test_controls_game_state_interaction ... ok
test testes::test_integration::test_game_stats_game_state_interaction ... ok
test testes::test_integration::test_save_data_leaderboard_interaction ... ok
test testes::test_integration::test_all_shapes_in_game ... ok
test testes::test_integration::test_rotation_collision_interaction ... ok
test testes::test_integration::test_performance_game_state_creation ... ok
test testes::test_integration::test_performance_bag_generator ... ok
test testes::test_integration::test_performance_collision_detection ... ok
test testes::test_integration::test_performance_rotation ... ok
test testes::test_integration::test_performance_leaderboard ... ok
test testes::test_integration::test_performance_controls_validation ... ok
test testes::test_integration::test_performance_save_data_hashing ... ok
test testes::test_io::test_canvas_creation ... ok
test testes::test_io::test_shape_str_constant ... ok
test testes::test_io::test_shape_width_constant ... ok
test testes::test_io::test_disp_width_calculation ... ok
test testes::test_io::test_key_reader_creation ... ok
test testes::test_io::test_key_reader_get_key_no_input ... ok
test testes::test_io::test_key_reader_default ... ok
test testes::test_io::test_field_dimensions ... ok
test testes::test_io::test_disp_height_calculation ... ok
test testes::test_io::test_terminal_minimum_size ... ok
test testes::test_tetromino::test_tetromino_t_creation ... ok
test testes::test_tetromino::test_tetromino_l_creation ... ok
test testes::test_tetromino::test_tetromino_j_creation ... ok
test testes::test_tetromino::test_tetromino_s_creation ... ok
test testes::test_tetromino::test_tetromino_z_creation ... ok
test testes::test_tetromino::test_tetromino_o_creation ... ok
test testes::test_tetromino::test_tetromino_i_creation ... ok
test testes::test_tetromino::test_tetromino_t_rotation ... ok
test testes::test_tetromino::test_tetromino_l_rotation ... ok
test testes::test_tetromino::test_tetromino_j_rotation ... ok
test testes::test_tetromino::test_tetromino_s_rotation ... ok
test testes::test_tetromino::test_tetromino_z_rotation ... ok
test testes::test_tetromino::test_tetromino_o_no_rotation ... ok
test testes::test_tetromino::test_tetromino_i_rotation ... ok
test testes::test_tetromino::test_shape_colors_count ... ok
test testes::test_tetromino::test_shape_color_index_match ... ok
test testes::test_tetromino::test_random_shape_distribution ... ok
test testes::test_tetromino::test_tetromino_select_creation ... ok
test testes::test_tetromino::test_bag_generator_creation ... ok
test testes::test_tetromino::test_bag_generator_next_shape ... ok
test testes::test_tetromino::test_bag_system_all_seven_types ... ok
test testes::test_tetromino::test_bag_refill ... ok
test testes::test_tetromino::test_shape_coords_bounds ... ok
test testes::test_tetromino::test_each_shape_has_four_blocks ... ok
test testes::test_tetromino::test_shape_blocks_unique ... ok

test result: ok. 120 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 15 doctests

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 🐛 Известные ограничения

1. **Только Linux/Unix**: Windows требует эмулятора терминала (WSL, Cygwin)
2. **Моноширинный шрифт**: требуется для корректного отображения
3. **Минимальный размер**: Окно терминала должно быть не менее 22×25
4. **ASCII-графика**: Ограниченная визуализация по сравнению с GUI
5. **Нет поддержки мыши**: Управление только с клавиатуры

## ❓ Часто задаваемые вопросы (FAQ)

### Можно ли изменить управление?

Да! Используйте `ControlsConfig` для создания кастомной конфигурации и сохраните её в JSON файл.

### Как сохранить рекорд?

Рекорд сохраняется автоматически при завершении игры, если вы превзошли предыдущий.

### Есть ли звуковые эффекты?

Да! При удалении линий воспроизводится терминальный bell-сигнал.

### Можно ли играть мышью?

Нет, управление только с клавиатуры.

### Сколько максимум очков можно набрать?

Теоретического лимита нет, но на практике игра становится очень быстрой после 50-100 линий.

### Что происходит, когда скорость становится очень большой?

Фигуры падают почти мгновенно, что делает игру крайне сложной.

### Как работает удержание фигуры?

Нажмите `c`, чтобы отложить текущую фигуру. Можно использовать один раз за ход. При повторном нажатии `c` фигуры меняются местами.

### Что такое режим спринт?

Режим спринт — это игра на скорость. Цель: очистить 40 линий как можно быстрее. Запускается клавишей `r` из меню.

### Как сбросить рекорд?

Рекорд хранится в конфигурационном файле. Для сброса удалите файл конфигурации:
```bash
# Linux: ~/.config/tetris-cli/config.toml
# macOS: ~/Library/Preferences/tetris-cli/config.toml
```

## 📝 Лицензия

Проект распространяется под лицензией **GPL-3.0**. См. файл [LICENSE](LICENSE) для деталей.

### Вы можете:
- ✅ Использовать код в личных целях
- ✅ Модифицировать исходный код
- ✅ Распространять модификации

### При условии:
- 📢 Сохранение лицензии в производных работах
- 🔓 Открытость модификаций
- 📄 Предоставление исходного кода

## 🤝 Вклад в проект

Приветствуются:
- Отчёты об ошибках
- Предложения по улучшению
- Pull Request'ы с исправлениями

## 📞 Контакты

- **Репозиторий**: https://github.com/Githab-capibara/tetris-cli
- **Автор**: Dylan Turner (circa 2022)
- **Текущая версия**: 23.96.3

## 🙏 Благодарности

Спасибо всем контрибьюторам за помощь в развитии проекта!

---

## История изменений (Changelog)

### [23.96.3] — 2026-03-05

#### Добавлено
- **Удержание фигуры (Hold)** — механика откладывания фигуры с клавишей `c`
  - Отображение удержанной фигуры слева от поля
  - Обмен текущей и удержанной фигуры
  - Запрет повторного удержания в одном ходу
- **Режим спринт** — игра на скорость до 40 линий
  - Запуск клавишей `r` из меню
  - Отображение таймера времени
  - Показ прогресса (X/40 линий)
  - Статистика после завершения
- **Звуковые эффекты** — терминальный bell при удалении линий
- **Статистика игры (GameStats)** — подсчёт деталей игры
  - Количество фигур каждого типа (T, L, J, S, Z, O, I)
  - Общее количество фигур
  - Максимальное комбо (одновременное удаление линий)
  - Время игры с точностью до секунды
- **Экран статистики** — отображение после завершения игры
  - Режим игры (классика/спринт)
  - Время игры
  - Количество использованных фигур
  - Максимальное комбо
- **Настраиваемое управление (ControlsConfig)** — возможность изменить все клавиши
  - Сохранение/загрузка конфигурации в JSON
  - Валидация на дубликаты и недопустимые значения
  - Поддержка кастомных раскладок (Vim, Numpad, и др.)

#### Изменено
- **README.md** — обновлён с информацией о новых функциях
  - Добавлены 6 новых групп тестов
  - Обновлена таблица управления
  - Добавлен раздел "Новые функции"
- **Меню игры** — добавлены опции выбора режима
  - Enter — классическая игра
  - R — режим спринт
  - L — таблица лидеров
- **Игровой цикл** — интеграция новых механик
  - Обработка клавиши `c` для удержания
  - Запуск таймера при начале игры
  - Отображение статистики после игры

#### Исправлено
- **Debug для GameMode** — добавлен derive(Debug) для тестирования
- **Геттеры для тестов** — добавлены публичные методы доступа
  - get_held_shape() — удержанная фигура
  - can_hold() — флаг возможности удержания
  - get_curr_shape() — текущая фигура
  - get_lines_cleared_public() — количество линий

#### Технические изменения
- **game.rs** — +310 строк кода
  - GameStats struct с методами
  - GameMode enum (Classic/Sprint)
  - hold_shape() метод
  - draw_held_shape() отрисовка
  - draw_sprint_timer() таймер спринт
- **main.rs** — +92 строки кода
  - show_game_stats() функция
  - Обновление main() для поддержки режимов
- **tetromino.rs** — +370 строк кода
  - 20 новых модульных тестов
  - 4 группы тестов (GameStats, Hold, Sprint, GameMode)
- **controls.rs** — новый модуль
  - ControlsConfig struct
  - 20 тестов для конфигурации
- **testes/** — +120 тестов
  - test_controls.rs (20 тестов)
  - test_game_logic.rs (30 тестов)
  - test_tetromino.rs (25 тестов)
  - test_highscore.rs (15 тестов)
  - test_io.rs (10 тестов)
  - test_integration.rs (20 тестов)

#### Тестирование
- **120 модульных и интеграционных тестов** — полное покрытие новых функций
- **15 doctest** — документация API
- **Все тесты проходят** — 135/135 успешно

---

### [23.96.2] — 2026-03-04

#### Добавлено
- **Система уровней** — повышение уровня каждые 10 удалённых линий с визуальным отображением
- **Предпросмотр следующей фигуры** — отображение справа от игрового поля
- **Призрачная фигура** — показывает точку приземления текущей фигуры
- **Таблица лидеров (топ-5)** — сохранение и отображение лучших результатов
- **Ввод имени игрока** — запрос имени после завершения игры
- **20 модульных тестов** — покрытие ключевой функциональности:
  - Тесты создания и вращения фигур (4)
  - Тесты состояния игры (6)
  - Тесты линий и уровней (4)
  - Тесты таблицы лидеров (4)
  - Тесты констант и границ (2)

#### Изменено
- **Объединение документации** — вся документация из README.md, ARCHITECTURE.md, USER_GUIDE.md объединена в одном файле
- **Публичные константы** — ROW_SCORE_INC, SPD_INC, INITIAL_FALL_SPD, LAND_TIME_DELAY_S, LINES_PER_LEVEL
- **Добавлены геттеры** — для GameState: get_score(), get_fall_spd(), get_blocks()

#### Исправлено
- **Исправлено вращение** — корректные ожидаемые координаты в тестах
- **Улучшена обработка границ** — проверка при сохранении фигуры в сетку

---

### [23.96.1] — 2026-03-04

#### Изменено
- **Полный перевод на русский язык**: все комментарии, интерфейс и документация переведены на русский
- **Исправлена логика мгновенного падения**: клавиша `s` теперь корректно опускает фигуру до упора и сразу фиксирует её
- **Оптимизирован алгоритм удаления линий**: эффективный сдвиг строк вместо поэлементного копирования
- **Улучшена обработка ошибок**: заменены `unwrap()` на `expect()` с понятными сообщениями и обработкой ошибок
- **Рефакторинг типов фигур**: переименование на стандартные обозначения (T, L, J, S, Z, O, I)
- **Удалена зависимость от `math::round::floor`**: используется прямой cast для производительности
- **Добавлено сообщение о проигрыше**: отображается при окончании игры
- **Добавлены реализации Default**: для `GameState`, `Canvas`, `KeyReader`
- **Упрощён код**: замена `to_vec()` на прямые ссылки, замена `clone()` на копирование для Copy-типов

#### Документация
- **README.md**: полностью переработан с подробным описанием, таблицами и примерами
- **ARCHITECTURE.md**: новая документация по архитектуре проекта с диаграммами
- **USER_GUIDE.md**: новое руководство пользователя с советами и FAQ
- **CHANGELOG.md**: новый файл с историей изменений
- **Rustdoc**: добавлена документация для всех публичных API

#### Исправления
- Исправлен размер массива `BORDER` в `game.rs` (25 элементов вместо 23)
- Исправлен размер массива `MENU` в `main.rs` (25 элементов вместо 24)
- Убрана избыточная проверка `Dir::Down` во вращении фигур
- Исправлено использование `subsec_nanos()` на `subsec_millis()` для читаемости

---

### [23.96.0] — 2023-09-08

#### Добавлено
- **Функция паузы**: нажатие клавиши `p` приостанавливает игру с отображением сообщения "PAUSED"
- **Выход из паузы**: повторное нажатие `p` продолжает игру
- **Выход во время паузы**: нажатие Backspace возвращает в меню

#### Изменено
- Обновлена игровая логика для поддержки состояния паузы

---

### [23.95.0] — 2022-06-07

#### Добавлено
- **Поддержка NetBSD**: игра доступна в официальных репозиториях pkgsrc
- **Документация по платформам**: обновлён README с информацией о поддерживаемых ОС

#### Изменено
- Улучшена совместимость с различными Unix-системами

---

### [23.94.0] — 2022-06-06

#### Добавлено
- **Публикация на AUR**: пакет [tetris-cli-git](https://aur.archlinux.org/packages/tetris-terminal-git) для Arch Linux

#### Изменено
- Обновлена документация по установке

---

### [23.93.0] — 2022-06-05

#### Добавлено
- **Система рекордов**: сохранение лучшего результата с защитой от подделки
- **Хеширование рекордов**: использование соли и хеша для проверки целостности
- **Автоматическое сохранение**: рекорд сохраняется после каждой игры

#### Изменено
- Обновлён интерфейс для отображения рекорда в меню и во время игры

---

### [23.92.0] — 2022-06-04

#### Добавлено
- **Прогрессивная сложность**: увеличение скорости падения с каждым уровнем
- **Бонусная система очков**: умножение очков за несколько удалённых линий
- **Таймер приземления**: задержка 0.1 секунды перед фиксацией фигуры

#### Изменено
- Улучшена физика падения фигур с интерполяцией по времени
- Оптимизирован игровой цикл для стабильных 60 FPS

---

### [23.91.0] — 2022-06-03

#### Добавлено
- **7 типов тетрамино**: T, L, J, S, Z, O, I с уникальными цветами
- **Вращение фигур**: поворот на 90° по часовой и против часовой стрелки
- **Мгновенное падение**: быстрое опускание фигуры клавишей `s`

#### Изменено
- Улучшена графика терминала с использованием UTF-символов

---

### [23.90.0] — 2022-06-02

#### Добавлено
- **Первая публичная версия**
- Базовая игровая механика Тетриса
- Отрисовка в терминале через termion
- Управление с клавиатуры (a/d для движения, q/e для вращения)

#### Технические детали
- Язык: Rust 2021 Edition
- Зависимости: termion, rand, confy, serde, big_num, libmath
- Лицензия: GPL-3.0

---

## Типы изменений

- **Добавлено** — новые функции
- **Изменено** — изменения в существующих функциях
- **Удалено** — удалённые функции
- **Исправлено** — исправления ошибок

---

## Ссылки

- [Репозиторий проекта](https://github.com/Githab-capibara/tetris-cli)
- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)

---

**Приятной игры! 🎮**
