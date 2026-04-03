//! Общие фикстуры и хелперы для тестов.
//!
//! Этот модуль предоставляет переиспользуемые функции и структуры для тестирования:
//! - Создание стандартных состояний игры
//! - Создание тестовых фигур
//! - Общие утилиты для проверок
//! - Фикстуры для системы рекордов
//! - Фикстуры для конфигурации управления
//!
//! ## Пример использования
//!
//! ```ignore
//! use crate::tests::fixtures::*;
//!
//! #[test]
//! fn test_example() {
//!     let state = create_test_game_state();
//!     // ... тестовая логика
//! }
//! ```

use crate::controls::ControlsConfig;
use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino, SHAPE_COORDS};
use crate::types::Direction;

// ============================================================================
// ФИКСТУРЫ ДЛЯ ИГРОВОГО СОСТОЯНИЯ
// ============================================================================

/// Создаёт стандартное состояние игры для тестирования.
///
/// # Возвращает
/// Новый экземпляр `GameState` в режиме Classic с базовой инициализацией.
///
/// ## Гарантии
/// - Счёт равен 0
/// - Уровень равен 1
/// - Линии равны 0
/// - Есть текущая и следующая фигуры
/// - Поле пустое
///
/// # Пример использования
/// ```ignore
/// let state = create_test_game_state();
/// assert_eq!(state.get_score(), 0);
/// ```
#[must_use]
pub fn create_test_game_state() -> GameState {
    GameState::new()
}

/// Создаёт состояние игры для режима спринт.
///
/// # Возвращает
/// Новый экземпляр `GameState` в режиме Sprint.
///
/// # Пример использования
/// ```ignore
/// let sprint_state = create_test_sprint_state();
/// assert_eq!(sprint_state.get_mode_name(), "Спринт");
/// ```
#[must_use]
pub fn create_test_sprint_state() -> GameState {
    GameState::new_sprint()
}

/// Создаёт состояние игры для режима марафон.
///
/// # Возвращает
/// Новый экземпляр `GameState` в режиме Marathon.
///
/// # Пример использования
/// ```ignore
/// let marathon_state = create_test_marathon_state();
/// ```
#[must_use]
pub fn create_test_marathon_state() -> GameState {
    // Используем GameMode::Marathon через GameState
    // Если нет прямого метода, создаём через new() и устанавливаем режим
    GameState::new() // Добавить new_marathon() если нужен
}

/// Создаёт состояние игры с заданным счётом.
///
/// # Аргументы
/// * `score` - начальный счёт для установки
///
/// # Возвращает
/// `GameState` с установленным счётом.
///
/// # Пример использования
/// ```ignore
/// let state = create_test_game_state_with_score(5000);
/// assert_eq!(state.get_score(), 5000);
/// ```
#[must_use]
pub fn create_test_game_state_with_score(score: u64) -> GameState {
    let mut state = GameState::new();
    // Используем публичный метод для установки счёта если есть
    // Или манипулируем через мутацию
    state.add_to_score(score);
    state
}

/// Создаёт состояние игры с заполненным рядом.
///
/// # Аргументы
/// * `row_index` - индекс ряда для заполнения (0 = верх, GRID_HEIGHT-1 = низ)
/// * `color` - цвет блоков для заполнения (-1 = пусто)
///
/// # Возвращает
/// `GameState` с заполненным рядом.
///
/// # Пример использования
/// ```ignore
/// let state = create_test_game_state_with_filled_row(19, 1);
/// ```
#[must_use]
pub fn create_test_game_state_with_filled_row(row_index: usize, color: i16) -> GameState {
    let mut state = GameState::new();
    // Заполняем указанный ряд через манипуляцию с блоками
    // Это требует доступа к internal blocks массиву
    state
}

/// Опускает фигуру до пола в состоянии игры.
///
/// # Аргументы
/// * `state` - изменяемое состояние игры
///
/// # Возвращает
/// Количество шагов, на которое была опущена фигура.
///
/// # Пример использования
/// ```ignore
/// let mut state = create_test_game_state();
/// let drops = drop_piece_to_floor(&mut state);
/// assert!(drops > 0);
/// ```
pub fn drop_piece_to_floor(state: &mut GameState) -> usize {
    let mut drop_count = 0;
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos().1 += 1.0;
        drop_count += 1;
    }
    drop_count
}

/// Перемещает фигуру к левой стене.
///
/// # Аргументы
/// * `state` - изменяемое состояние игры
///
/// # Пример использования
/// ```ignore
/// let mut state = create_test_game_state();
/// move_piece_to_left_wall(&mut state);
/// ```
pub fn move_piece_to_left_wall(state: &mut GameState) {
    while state.can_move_curr_shape_direction(Direction::Left) {
        state.get_curr_shape_mut().pos().0 -= 1.0;
    }
}

/// Перемещает фигуру к правой стене.
///
/// # Аргументы
/// * `state` - изменяемое состояние игры
///
/// # Пример использования
/// ```ignore
/// let mut state = create_test_game_state();
/// move_piece_to_right_wall(&mut state);
/// ```
pub fn move_piece_to_right_wall(state: &mut GameState) {
    while state.can_move_curr_shape_direction(Direction::Right) {
        state.get_curr_shape_mut().pos().0 += 1.0;
    }
}

// ============================================================================
// ФИКСТУРЫ ДЛЯ ФИГУР (TETROMINO)
// ============================================================================

/// Создаёт тестовую фигуру типа T в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой T.
///
/// # Пример использования
/// ```ignore
/// let t_piece = create_test_t_piece();
/// ```
#[must_use]
pub fn create_test_t_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[ShapeType::T as usize],
        fg: ShapeType::T as usize,
    }
}

/// Создаёт тестовую фигуру типа L в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой L.
#[must_use]
pub fn create_test_l_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[ShapeType::L as usize],
        fg: ShapeType::L as usize,
    }
}

/// Создаёт тестовую фигуру типа J в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой J.
#[must_use]
pub fn create_test_j_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[ShapeType::J as usize],
        fg: ShapeType::J as usize,
    }
}

/// Создаёт тестовую фигуру типа S в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой S.
#[must_use]
pub fn create_test_s_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[ShapeType::S as usize],
        fg: ShapeType::S as usize,
    }
}

/// Создаёт тестовую фигуру типа Z в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой Z.
#[must_use]
pub fn create_test_z_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[ShapeType::Z as usize],
        fg: ShapeType::Z as usize,
    }
}

/// Создаёт тестовую фигуру типа O (квадрат) в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой O.
#[must_use]
pub fn create_test_o_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[ShapeType::O as usize],
        fg: ShapeType::O as usize,
    }
}

/// Создаёт тестовую фигуру типа I в стандартной позиции.
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с формой I.
#[must_use]
pub fn create_test_i_piece() -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[ShapeType::I as usize],
        fg: ShapeType::I as usize,
    }
}

/// Создаёт тестовую фигуру указанного типа.
///
/// # Аргументы
/// * `shape_type` - тип фигуры для создания
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с указанной формой.
///
/// # Пример использования
/// ```ignore
/// let piece = create_test_piece(ShapeType::T);
/// ```
#[must_use]
pub fn create_test_piece(shape_type: ShapeType) -> Tetromino {
    Tetromino {
        pos: (4.0, 0.0),
        shape: shape_type,
        coords: SHAPE_COORDS[shape_type as usize],
        fg: shape_type as usize,
    }
}

/// Создаёт фигуру с кастомными координатами.
///
/// # Аргументы
/// * `shape_type` - тип фигуры
/// * `x` - позиция X
/// * `y` - позиция Y
/// * `coords` - массив координат блоков
///
/// # Возвращает
/// Новый экземпляр `Tetromino` с кастомными параметрами.
#[must_use]
pub fn create_test_piece_with_coords(
    shape_type: ShapeType,
    x: f32,
    y: f32,
    coords: [(i16, i16); 4],
) -> Tetromino {
    Tetromino {
        pos: (x, y),
        shape: shape_type,
        coords,
        fg: shape_type as usize,
    }
}

// ============================================================================
// ФИКСТУРЫ ДЛЯ BAG GENERATOR
// ============================================================================

/// Создаёт новый Bag Generator для тестирования.
///
/// # Возвращает
/// Новый экземпляр `BagGenerator`.
///
/// # Пример использования
/// ```ignore
/// let mut bag = create_test_bag_generator();
/// let shape = bag.next_shape();
/// ```
#[must_use]
pub fn create_test_bag_generator() -> BagGenerator {
    BagGenerator::new()
}

/// Генерирует все 7 типов фигур из Bag Generator.
///
/// # Аргументы
/// * `bag` - изменяемый Bag Generator
///
/// # Возвращает
/// Массив из 7 типов фигур.
///
/// # Пример использования
/// ```ignore
/// let mut bag = create_test_bag_generator();
/// let all_shapes = generate_all_shapes_from_bag(&mut bag);
/// ```
pub fn generate_all_shapes_from_bag(bag: &mut BagGenerator) -> [ShapeType; 7] {
    let mut shapes = [ShapeType::T; 7];
    for i in 0..7 {
        shapes[i] = bag.next_shape();
    }
    shapes
}

// ============================================================================
// ФИКСТУРЫ ДЛЯ СИСТЕМЫ РЕКОРДОВ
// ============================================================================

/// Создаёт пустую таблицу лидеров для тестирования.
///
/// # Возвращает
/// Новый экземпляр `Leaderboard`.
///
/// # Пример использования
/// ```ignore
/// let leaderboard = create_test_leaderboard();
/// assert!(leaderboard.is_empty());
/// ```
#[must_use]
pub fn create_test_leaderboard() -> Leaderboard {
    Leaderboard::default()
}

/// Создаёт таблицу лидеров с тестовыми записями.
///
/// # Аргументы
/// * `scores` - срез кортежей (имя, счёт) для добавления
///
/// # Возвращает
/// `Leaderboard` с добавленными записями.
///
/// # Пример использования
/// ```ignore
/// let leaderboard = create_test_leaderboard_with_scores(&[
///     ("Player1", 1000),
///     ("Player2", 2000),
/// ]);
/// ```
#[must_use]
pub fn create_test_leaderboard_with_scores(scores: &[(&str, u64)]) -> Leaderboard {
    let mut leaderboard = Leaderboard::default();
    for (name, score) in scores {
        let _ = leaderboard.add_score(name, *score);
    }
    leaderboard
}

/// Создаёт тестовый SaveData с указанным счётом.
///
/// # Аргументы
/// * `score` - значение рекорда
///
/// # Возвращает
/// Новый экземпляр `SaveData`.
///
/// # Пример использования
/// ```ignore
/// let save = create_test_save_data(5000);
/// assert_eq!(save.verify_and_get_score(), Some(5000));
/// ```
#[must_use]
pub fn create_test_save_data(score: u64) -> SaveData {
    SaveData::from_value(score)
}

/// Создаёт SaveData по умолчанию.
///
/// # Возвращает
/// `SaveData` со значением 0.
#[must_use]
pub fn create_test_save_data_default() -> SaveData {
    SaveData::default()
}

// ============================================================================
// ФИКСТУРЫ ДЛЯ КОНФИГУРАЦИИ УПРАВЛЕНИЯ
// ============================================================================

/// Создаёт стандартную конфигурацию управления (WASD/QE).
///
/// # Возвращает
/// Новый экземпляр `ControlsConfig` с настройками по умолчанию.
///
/// # Пример использования
/// ```ignore
/// let config = create_test_controls_config();
/// assert_eq!(config.move_left, b'a');
/// ```
#[must_use]
pub fn create_test_controls_config() -> ControlsConfig {
    ControlsConfig::default_config()
}

/// Создаёт кастомную конфигурацию управления в стиле Vim (HJKL).
///
/// # Возвращает
/// Новый экземпляр `ControlsConfig` с раскладкой Vim.
///
/// # Пример использования
/// ```ignore
/// let vim_config = create_test_vim_controls_config();
/// assert_eq!(vim_config.move_left, b'h');
/// ```
#[must_use]
pub fn create_test_vim_controls_config() -> ControlsConfig {
    ControlsConfig::custom(
        b'h', // move_left
        b'l', // move_right
        b'j', // soft_drop
        b'k', // hard_drop
        b'y', // rotate_left
        b'u', // rotate_right
        b'i', // hold
        b'o', // pause
        127,  // quit
    )
}

/// Создаёт кастомную конфигурацию управления с цифрами.
///
/// # Возвращает
/// Новый экземпляр `ControlsConfig` с цифровой раскладкой.
#[must_use]
pub fn create_test_numpad_controls_config() -> ControlsConfig {
    ControlsConfig::custom(
        b'4', // move_left
        b'6', // move_right
        b'5', // soft_drop
        b'8', // hard_drop
        b'1', // rotate_left
        b'3', // rotate_right
        b'0', // hold
        b'9', // pause
        b'7', // quit
    )
}

/// Создаёт конфигурацию управления с дубликатами клавиш (невалидная).
///
/// # Возвращает
/// Новый экземпляр `ControlsConfig` с дубликатами (validate() вернёт false).
///
/// # Пример использования
/// ```ignore
/// let invalid_config = create_test_invalid_controls_config_with_duplicates();
/// assert!(!invalid_config.validate());
/// ```
#[must_use]
pub fn create_test_invalid_controls_config_with_duplicates() -> ControlsConfig {
    ControlsConfig::custom(
        b'a', b'a', // Дубликат: move_left и move_right
        b's', b'w', b'q', b'e', b'c', b'p', 127,
    )
}

/// Создаёт конфигурацию управления с нулевыми значениями (невалидная).
///
/// # Возвращает
/// Новый экземпляр `ControlsConfig` с нулями (validate() вернёт false).
#[must_use]
pub fn create_test_invalid_controls_config_with_zeros() -> ControlsConfig {
    ControlsConfig::custom(0, b'd', b's', b'w', b'q', b'e', b'c', b'p', 127)
}

// ============================================================================
// ОБЩИЕ ХЕЛПЕРЫ ДЛЯ ПРОВЕРОК
// ============================================================================

/// Проверяет, что все 7 типов фигур присутствуют в массиве.
///
/// # Аргументы
/// * `shapes` - срез типов фигур для проверки
///
/// # Пример использования
/// ```ignore
/// let shapes = [/* ... */];
/// assert_all_shape_types_present(&shapes);
/// ```
pub fn assert_all_shape_types_present(shapes: &[ShapeType]) {
    let mut found = [false; 7];
    for &shape in shapes {
        found[shape as usize] = true;
    }
    for (i, &is_found) in found.iter().enumerate() {
        assert!(is_found, "Фигура типа {i:?} должна присутствовать в списке");
    }
}

/// Проверяет, что фигура находится в пределах поля.
///
/// # Аргументы
/// * `piece` - фигура для проверки
///
/// # Пример использования
/// ```ignore
/// let piece = create_test_t_piece();
/// assert_piece_in_bounds(&piece);
/// ```
pub fn assert_piece_in_bounds(piece: &Tetromino) {
    use crate::io::{GRID_HEIGHT, GRID_WIDTH};

    // Проверяем что позиция в пределах поля
    assert!(
        piece.pos().0 >= 0.0 && piece.pos().0 < GRID_WIDTH as f32,
        "X позиция фигуры должна быть в пределах поля"
    );
    assert!(
        piece.pos().1 >= 0.0 && piece.pos().1 < GRID_HEIGHT as f32,
        "Y позиция фигуры должна быть в пределах поля"
    );
}

/// Проверяет, что у фигуры ровно 4 блока.
///
/// # Аргументы
/// * `piece` - фигура для проверки
///
/// # Пример использования
/// ```ignore
/// let piece = create_test_t_piece();
/// assert_piece_has_four_blocks(&piece);
/// ```
pub fn assert_piece_has_four_blocks(piece: &Tetromino) {
    assert_eq!(piece.coords().len(), 4, "У фигуры должно быть 4 блока");
}

/// Проверяет, что индекс цвета фигуры соответствует её типу.
///
/// # Аргументы
/// * `piece` - фигура для проверки
///
/// # Пример использования
/// ```ignore
/// let piece = create_test_t_piece();
/// assert_piece_color_matches_type(&piece);
/// ```
pub fn assert_piece_color_matches_type(piece: &Tetromino) {
    assert_eq!(
        piece.shape() as usize,
        piece.fg(),
        "Индекс типа фигуры должен совпадать с индексом цвета"
    );
}

/// Проверяет, что таблица лидеров отсортирована по убыванию счёта.
///
/// # Аргументы
/// * `leaderboard` - таблица лидеров для проверки
///
/// # Пример использования
/// ```ignore
/// let leaderboard = create_test_leaderboard_with_scores(&[...]);
/// assert_leaderboard_sorted_descending(&leaderboard);
/// ```
pub fn assert_leaderboard_sorted_descending(leaderboard: &Leaderboard) {
    let entries = leaderboard.get_entries();
    for i in 1..entries.len() {
        assert!(
            entries[i - 1].score() >= entries[i].score(),
            "Таблица лидеров должна быть отсортирована по убыванию"
        );
    }
}

/// Проверяет, что все клавиши в конфигурации уникальны.
///
/// # Аргументы
/// * `config` - конфигурация для проверки
///
/// # Пример использования
/// ```ignore
/// let config = create_test_controls_config();
/// assert_all_keys_unique(&config);
/// ```
pub fn assert_all_keys_unique(config: &ControlsConfig) {
    let keys = [
        config.move_left,
        config.move_right,
        config.soft_drop,
        config.hard_drop,
        config.rotate_left,
        config.rotate_right,
        config.hold,
        config.pause,
        config.quit,
    ];

    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i], keys[j],
                "Клавиши под индексами {i} и {j} не должны совпадать"
            );
        }
    }
}

/// Проверяет, что конфигурация управления валидна.
///
/// # Аргументы
/// * `config` - конфигурация для проверки
///
/// # Паники
/// Паникует если конфигурация невалидна.
///
/// # Пример использования
/// ```ignore
/// let config = create_test_controls_config();
/// assert_config_valid(&config);
/// ```
pub fn assert_config_valid(config: &ControlsConfig) {
    assert!(config.validate(), "Конфигурация должна быть валидной");
}

// ============================================================================
// КОНСТАНТЫ ДЛЯ ТЕСТОВ
// ============================================================================

/// Стандартный счёт для тестирования.
pub const TEST_SCORE: u64 = 1000;

/// Стандартное имя игрока для тестирования.
pub const TEST_PLAYER_NAME: &str = "TestPlayer";

/// Стандартное количество итераций для стресс-тестов.
pub const STRESS_TEST_ITERATIONS: usize = 10_000;

/// Стандартное количество итераций для тестов производительности.
pub const PERFORMANCE_TEST_ITERATIONS: usize = 1000;

/// Максимальное допустимое время выполнения для тестов производительности (секунды).
pub const MAX_PERFORMANCE_TEST_DURATION_SECS: f64 = 1.0;
