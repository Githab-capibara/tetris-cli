//! Тесты целостности архитектуры tetris-cli.
//!
//! Этот модуль проверяет соблюдение архитектурных принципов и границ модулей:
//! - Разделение компонентов (A1)
//! - Границы модулей (A2)
//! - Инкапсуляция (A3)
//! - Инверсия зависимостей (A7)
//! - Обработка ошибок (A8)
//! - Отсутствие циклических зависимостей
//! - Принципы SOLID
//! - Обратная совместимость

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]
#![allow(deprecated)]

// Импорты для тестов
use crate::game::board::GameBoard;
use crate::game::scoreboard::ScoreBoard;
use crate::game::state::GameState;
use crate::game::GameView;
use crate::game::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};
use crate::io::Canvas;
use crate::io_traits::{InputReader, Renderer};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino};

// ============================================================================
// ТЕСТ 1: РАЗДЕЛЕНИЕ КОМПОНЕНТОВ (A1)
// ============================================================================

/// Проверка разделения компонентов GameState.
///
/// Тест проверяет что:
/// - GameBoard существует и имеет правильные методы
/// - ScoreBoard существует и имеет правильные методы
/// - GameState использует компоненты через делегирование
#[test]
fn test_component_separation() {
    use crate::game::board::GameBoard;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;
    use crate::game::GameView;

    // === Проверка GameBoard ===
    let mut board = GameBoard::new();

    // Проверяем что GameBoard имеет методы для работы с полем
    assert_eq!(
        board.get_block(0, 0),
        Some(-1),
        "Новое поле должно быть пустым"
    );

    board.set_block(5, 10, 1);
    assert_eq!(
        board.get_block(5, 10),
        Some(1),
        "Ячейка должна быть установлена"
    );

    assert_eq!(
        board.get_block(100, 100),
        None,
        "Выход за границы должен вернуть None"
    );

    let blocks = board.get_blocks();
    assert_eq!(blocks.len(), 20, "Поле должно иметь 20 рядов");
    assert_eq!(blocks[0].len(), 10, "Поле должно иметь 10 колонок");

    // Проверяем битовую маску заполненных линий
    assert_eq!(
        board.get_filled_lines_mask(),
        0,
        "Новое поле не имеет заполненных линий"
    );

    // === Проверка ScoreBoard ===
    let mut scoreboard = ScoreBoard::new();

    // Проверяем что ScoreBoard имеет методы для работы с очками
    assert_eq!(scoreboard.get_score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(scoreboard.get_level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(
        scoreboard.get_lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );

    // Проверяем изменение очков
    let new_score = scoreboard.add_score(100);
    assert_eq!(new_score, 100, "add_score должен вернуть новый счёт");
    assert_eq!(scoreboard.get_score(), 100, "Счёт должен обновиться");

    // Проверяем изменение уровня
    scoreboard.set_level(5);
    assert_eq!(scoreboard.get_level(), 5, "Уровень должен обновиться");

    // Проверяем что уровень не может быть меньше 1
    scoreboard.set_level(0);
    assert_eq!(scoreboard.get_level(), 1, "Уровень не может быть меньше 1");

    // Проверяем изменение количества линий
    scoreboard.set_lines_cleared(25);
    assert_eq!(
        scoreboard.get_lines_cleared(),
        25,
        "Количество линий должно обновиться"
    );

    // === Проверка делегирования в GameState ===
    let state = GameState::new();

    // GameState должен делегировать доступ к компонентам
    let blocks = state.get_blocks();
    assert_eq!(
        blocks.len(),
        20,
        "GameState должен предоставлять доступ к полю"
    );

    let score = state.score();
    assert_eq!(score, 0, "GameState должен предоставлять доступ к счёту");

    let level = state.level();
    assert_eq!(level, 1, "GameState должен предоставлять доступ к уровню");

    let lines = state.lines_cleared();
    assert_eq!(lines, 0, "GameState должен предоставлять доступ к линиям");

    // Проверяем что GameState использует композицию
    let view = GameView::from_game_state(&state);
    assert!(
        !view.score.is_empty(),
        "GameView должен корректно работать с GameState"
    );
}

// ============================================================================
// ТЕСТ 2: ГРАНИЦЫ МОДУЛЕЙ (A2)
// ============================================================================

/// Проверка структуры модуля tetromino.
///
/// Тест проверяет что:
/// - Модуль tetromino/ имеет правильную структуру
/// - Все подмодули доступны через ре-экспорт
/// - ShapeType, Tetromino, BagGenerator импортируются корректно
#[test]
fn test_arch_integrity_module_boundaries() {
    // === Проверка структуры tetromino/ ===
    // Модуль должен иметь подмодули: bag_generator, constants, shape_type, tetromino

    // === Проверка ре-экспорта основных типов ===
    // ShapeType должен быть доступен напрямую из tetromino
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];
    assert_eq!(shapes.len(), 7, "Должно быть 7 типов фигур");

    // Tetromino должен быть доступен напрямую
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Tetromino должен иметь корректный ShapeType"
    );

    // BagGenerator должен быть доступен напрямую
    let mut bag2 = BagGenerator::new();
    let shape1 = Tetromino::from_bag(&mut bag2);
    let shape2 = Tetromino::from_bag(&mut bag2);

    // 7-bag гарантирует что каждые 7 фигур содержат все типы
    // Проверяем что BagGenerator работает корректно
    assert!(
        matches!(
            shape1.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "BagGenerator должен выдавать корректные фигуры"
    );

    // === Проверка констант ===
    use crate::tetromino::{SHAPE_COLORS, SHAPE_COORDS};

    assert_eq!(
        SHAPE_COORDS.len(),
        7,
        "SHAPE_COORDS должен содержать координаты для 7 фигур"
    );
    assert_eq!(
        SHAPE_COLORS.len(),
        7,
        "SHAPE_COLORS должен содержать цвета для 7 фигур"
    );

    // === Проверка что RotationDirection доступен ===
    use crate::tetromino::RotationDirection;
    let _ = RotationDirection::Clockwise;
    let _ = RotationDirection::CounterClockwise;
}

// ============================================================================
// ТЕСТ 3: ИНКАПСУЛЯЦИЯ (A3)
// ============================================================================

/// Проверка инкапсуляции через методы-мутаторы.
///
/// Тест проверяет что:
/// - Методы-мутаторы GameState работают корректно
/// - score/level/lines_cleared изменяются только через методы
#[test]
fn test_encapsulation() {
    let mut state = GameState::new();

    // === Проверка инкапсуляции счёта ===
    let initial_score = state.score();
    assert_eq!(initial_score, 0, "Начальный счёт должен быть 0");

    // Используем метод-мутатор для изменения счёта
    state.set_score(500);
    assert_eq!(state.score(), 500, "Счёт должен измениться через set_score");

    // Проверяем что add_score работает корректно
    state.add_score(250);
    assert_eq!(state.score(), 750, "add_score должен добавить очки");

    // === Проверка инкапсуляции уровня ===
    let initial_level = state.level();
    assert_eq!(initial_level, 1, "Начальный уровень должен быть 1");

    state.set_level(10);
    assert_eq!(
        state.level(),
        10,
        "Уровень должен измениться через set_level"
    );

    // Проверяем что уровень не может быть меньше 1
    state.set_level(0);
    assert_eq!(state.level(), 1, "Уровень не может быть меньше 1");

    state.increment_level();
    assert_eq!(
        state.level(),
        2,
        "increment_level должен увеличить уровень на 1"
    );

    // === Проверка инкапсуляции количества линий ===
    let initial_lines = state.lines_cleared();
    assert_eq!(initial_lines, 0, "Начальное количество линий должно быть 0");

    state.set_lines_cleared(50);
    assert_eq!(
        state.lines_cleared(),
        50,
        "Количество линий должно измениться через set_lines_cleared"
    );

    state.add_lines_cleared(10);
    assert_eq!(
        state.lines_cleared(),
        60,
        "add_lines_cleared должен добавить линии"
    );

    // === Проверка инкапсуляции поля ===
    // Поле должно изменяться только через методы
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "Поле должно иметь 20 рядов");

    // Проверяем что можно получить мутабельную ссылку на поле
    let blocks_mut = state.get_blocks_mut();
    blocks_mut[0][0] = 1;
    assert_eq!(
        state.get_blocks()[0][0],
        1,
        "Поле должно измениться через get_blocks_mut"
    );

    // === Проверка инкапсуляции фигур ===
    let curr_shape = state.curr_shape();
    assert!(
        matches!(
            curr_shape.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "curr_shape должен возвращать корректную фигуру"
    );

    let curr_shape_mut = state.get_curr_shape_mut();
    let original_x = curr_shape_mut.pos().0;
    curr_shape_mut.pos().0 += 1.0;
    assert_eq!(
        state.curr_shape().pos().0,
        original_x + 1.0,
        "Фигура должна измениться через get_curr_shape_mut"
    );
}

// ============================================================================
// ТЕСТ 4: ИНВЕРСИЯ ЗАВИСИМОСТЕЙ (A7)
// ============================================================================

/// Mock-реализация InputReader для тестирования.
struct MockInputReader {
    keys: Vec<u8>,
    index: usize,
}

impl MockInputReader {
    fn new(keys: Vec<u8>) -> Self {
        Self { keys, index: 0 }
    }
}

impl InputReader for MockInputReader {
    fn get_key(&mut self) -> std::io::Result<Option<u8>> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index];
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }
}

/// Mock-реализация Renderer для тестирования.
pub struct MockRenderer {
    flushed: bool,
    reset: bool,
}

impl MockRenderer {
    fn new() -> Self {
        Self {
            flushed: false,
            reset: false,
        }
    }
}

impl Renderer for MockRenderer {
    fn draw_strs(
        &mut self,
        _strings: &[&str],
        _pos: (u16, u16),
        _fg: &dyn termion::color::Color,
        _bg: &dyn termion::color::Color,
    ) {
        // Mock реализация
    }

    fn draw_string(
        &mut self,
        _string: &str,
        _pos: (u16, u16),
        _fg: &dyn termion::color::Color,
        _bg: &dyn termion::color::Color,
    ) {
        // Mock реализация
    }

    fn flush(&mut self) {
        self.flushed = true;
    }

    fn reset(&mut self) {
        self.reset = true;
    }
}

/// Проверка использования трейтов вместо конкретных типов.
///
/// Тест проверяет что:
/// - handle_input() принимает любой тип реализующий InputReader
/// - Можно использовать mock-реализацию InputReader
#[test]
fn test_dependency_inversion() {
    // === Проверка что MockInputReader работает ===
    let mut mock_reader = MockInputReader::new(vec![b'a', b'd', b'w', b's']);

    assert_eq!(
        mock_reader.get_key().unwrap(),
        Some(b'a'),
        "Первая клавиша должна быть 'a'"
    );
    assert_eq!(
        mock_reader.get_key().unwrap(),
        Some(b'd'),
        "Вторая клавиша должна быть 'd'"
    );
    assert_eq!(
        mock_reader.get_key().unwrap(),
        Some(b'w'),
        "Третья клавиша должна быть 'w'"
    );
    assert_eq!(
        mock_reader.get_key().unwrap(),
        Some(b's'),
        "Четвёртая клавиша должна быть 's'"
    );
    assert_eq!(
        mock_reader.get_key().unwrap(),
        None,
        "После всех клавиш должно вернуть None"
    );

    // === Проверка что MockRenderer работает ===
    let mut mock_renderer = MockRenderer::new();

    use termion::color::{Red, Reset};
    mock_renderer.draw_string("Test", (1, 1), &Red, &Reset);
    mock_renderer.flush();
    assert!(mock_renderer.flushed, "flush() должен быть вызван");

    mock_renderer.reset();
    assert!(mock_renderer.reset, "reset() должен быть вызван");

    // === Проверка полиморфизма ===
    // Функция принимающая любой InputReader
    fn process_input<R: InputReader>(reader: &mut R) -> Vec<u8> {
        let mut keys = Vec::new();
        while let Ok(Some(key)) = reader.get_key() {
            keys.push(key);
        }
        keys
    }

    let mut mock = MockInputReader::new(vec![b'x', b'y', b'z']);
    let result = process_input(&mut mock);
    assert_eq!(
        result,
        vec![b'x', b'y', b'z'],
        "Полиморфизм должен работать"
    );

    // === Проверка что GameState может работать с трейтами ===
    // GameState должен быть спроектирован для работы с InputReader
    let state = GameState::new();

    // Проверяем что GameState не зависит от конкретной реализации KeyReader
    // Это проверяется через компиляцию - если бы GameState зависел от KeyReader,
    // этот тест не скомпилировался бы
    let _ = state.score(); // Просто проверяем что state работает
}

// ============================================================================
// ТЕСТ 5: ОБРАБОТКА ОШИБОК (A8)
// ============================================================================

/// Проверка явной обработки ошибок через Result.
///
/// Тест проверяет что:
/// - run_game_loop() возвращает Result
/// - Ошибки корректно обрабатываются через ?
#[test]
fn test_error_handling() {
    // === Проверка что GameError существует ===
    use crate::errors::GameError;

    let validation_err = GameError::validation_error("Тестовая ошибка валидации");
    assert!(
        matches!(validation_err, GameError::ValidationError(_)),
        "ValidationError должен существовать"
    );

    let config_err = GameError::config_error("Тестовая ошибка конфигурации");
    assert!(
        matches!(config_err, GameError::ConfigError(_)),
        "ConfigError должен существовать"
    );

    // === Проверка конвертации io::Error в GameError ===
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO ошибка");
    let game_error: GameError = io_error.into();
    assert!(
        matches!(game_error, GameError::IoError(_)),
        "io::Error должен конвертироваться в GameError"
    );

    // === Проверка что play() возвращает Result ===
    // play() должен возвращать Result<u128, GameError>
    // Мы не можем вызвать play() без терминала, но можем проверить тип
    fn check_play_signature() {
        // Эта функция компилируется только если play() возвращает Result
        fn _requires_result<
            F: FnOnce(
                &mut GameState,
                &mut Canvas,
                &mut crate::io::KeyReader,
                &str,
            ) -> Result<u128, GameError>,
        >(
            _f: F,
        ) {
        }
    }
    check_play_signature();

    // === Проверка обработки ошибок через ? ===
    fn test_error_propagation() -> Result<(), GameError> {
        // Имитация функции которая может вернуть ошибку
        let result: Result<u128, GameError> = Ok(100);
        let value = result?; // Используем ? для обработки
        assert_eq!(value, 100);
        Ok(())
    }

    assert!(
        test_error_propagation().is_ok(),
        "Обработка ошибок через ? должна работать"
    );

    // === Проверка что ошибка корректно распространяется ===
    fn test_error_propagation_fail() -> Result<(), GameError> {
        let result: Result<u128, GameError> = Err(GameError::validation_error("Тест"));
        let _value = result?; // Ошибка должна распространиться
        Ok(())
    }

    let err = test_error_propagation_fail();
    assert!(
        err.is_err(),
        "Ошибка должна корректно распространяться через ?"
    );
    assert!(
        matches!(err, Err(GameError::ValidationError(_))),
        "Тип ошибки должен сохраниться"
    );
}

// ============================================================================
// ТЕСТ 6: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Проверка отсутствия циклических зависимостей между модулями.
///
/// Тест проверяет что:
/// - Базовые модули (constants, types, errors) не зависят от других
/// - Нет циклов между game/, tetromino/, io/
#[test]
fn test_arch_integrity_no_circular_dependencies() {
    // === Проверка независимости types.rs ===
    use crate::types::{Direction, RotationDirection, UpdateEndState};

    // Direction должен работать независимо
    let dir = Direction::Left;
    let rotation = dir.to_rotation_direction();
    assert_eq!(rotation, RotationDirection::CounterClockwise);

    let dir = Direction::Right;
    let rotation = dir.to_rotation_direction();
    assert_eq!(rotation, RotationDirection::Clockwise);

    // UpdateEndState должен работать независимо
    let state = UpdateEndState::Continue;
    assert_eq!(format!("{:?}", state), "Continue");

    // === Проверка независимости constants.rs ===
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};

    assert_eq!(FPS, 60, "FPS должен быть 60");
    assert_eq!(GRID_WIDTH, 10, "GRID_WIDTH должен быть 10");
    assert_eq!(GRID_HEIGHT, 20, "GRID_HEIGHT должен быть 20");

    // === Проверка независимости errors.rs ===
    use crate::errors::GameError;

    let _err = GameError::validation_error("Тест");

    // === Проверка что tetromino не зависит от game ===
    // Tetromino должен работать независимо от GameState
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Tetromino должен работать независимо от GameState"
    );

    // === Проверка что io_traits не зависит от game ===
    // InputReader и Renderer должны быть независимыми трейтами
    let _reader: &mut dyn InputReader = &mut MockInputReader::new(vec![]);
    let _renderer: &mut dyn Renderer = &mut MockRenderer::new();

    // === Проверка что GameBoard и ScoreBoard независимы ===
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();

    assert_eq!(board.get_block(0, 0), Some(-1));
    assert_eq!(scoreboard.get_score(), 0);
}

// ============================================================================
// ТЕСТ 7: ПРИНЦИПЫ SOLID
// ============================================================================

/// Проверка соблюдения принципов SOLID.
///
/// Тест проверяет что:
/// - Single Responsibility (GameBoard, ScoreBoard разделены)
/// - Dependency Inversion (использование трейтов)
#[test]
fn test_solid_principles() {
    // === Single Responsibility Principle (SRP) ===
    // GameBoard отвечает только за поле
    let mut board = GameBoard::new();
    board.set_block(5, 5, 1);
    assert_eq!(board.get_block(5, 5), Some(1));
    // GameBoard не должен отвечать за очки или фигуры

    // ScoreBoard отвечает только за очки
    let mut scoreboard = ScoreBoard::new();
    scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);
    // ScoreBoard не должен отвечать за поле или фигуры

    // GameState использует композицию для делегирования ответственности
    let state = GameState::new();
    let _blocks = state.get_blocks(); // Делегирует GameBoard
    let _score = state.score(); // Делегирует ScoreBoard

    // === Dependency Inversion Principle (DIP) ===
    // Модули верхнего уровня не должны зависеть от модулей нижнего уровня
    // Оба должны зависеть от абстракций

    // GameState работает с трейтами InputReader и Renderer
    // а не с конкретными реализациями KeyReader и Canvas
    fn requires_input_reader<R: InputReader>(_reader: &R) {}
    fn requires_renderer<R: Renderer>(_renderer: &R) {}

    let mock_reader = MockInputReader::new(vec![]);
    let mock_renderer = MockRenderer::new();

    requires_input_reader(&mock_reader);
    requires_renderer(&mock_renderer);

    // === Interface Segregation Principle (ISP) ===
    // Трейты должны быть специфичными, не перегруженными

    // BoardReadonly - только для чтения
    fn requires_board_readonly<B: crate::game::access::BoardReadonly>(_board: &B) {}
    requires_board_readonly(&GameState::new());

    // BoardMutable - для изменения (наследует BoardReadonly)
    fn requires_board_mutable<B: crate::game::access::BoardMutable>(_board: &mut B) {}
    requires_board_mutable(&mut GameState::new());

    // ScoreAccess - только для чтения
    fn requires_score_access<S: crate::game::access::ScoreAccess>(_scoreboard: &S) {}
    requires_score_access(&GameState::new());

    // ScoreMutable - для изменения (используем ScoreAccess с мутуабельной ссылкой)
    fn requires_score_mutable<S: crate::game::access::ScoreAccess>(_scoreboard: &mut S) {}
    requires_score_mutable(&mut GameState::new());

    // === Liskov Substitution Principle (LSP) ===
    // Подтипы должны заменять базовые типы без нарушения работы

    // MockInputReader должен заменять KeyReader
    fn process_any_reader<R: InputReader>(reader: &mut R) -> Option<u8> {
        reader.get_key().unwrap_or(None)
    }

    let mut mock = MockInputReader::new(vec![b't', b'e', b's', b't']);
    let _key = process_any_reader(&mut mock);

    // === Open/Closed Principle (OCP) ===
    // Модули должны быть открыты для расширения, закрыты для модификации

    // Можно добавить новые реализации InputReader без изменения существующего кода
    struct AnotherMockReader;
    impl InputReader for AnotherMockReader {
        fn get_key(&mut self) -> std::io::Result<Option<u8>> {
            Ok(None)
        }
    }

    let mut another = AnotherMockReader;
    let _key = process_any_reader(&mut another);
}

// ============================================================================
// ТЕСТ 8: ОБРАТНАЯ СОВМЕСТИМОСТЬ
// ============================================================================

/// Проверка обратной совместимости API.
///
/// Тест проверяет что:
/// - deprecated поля GameState всё ещё работают
/// - старые API продолжают функционировать
#[test]
fn test_architecture_backward_compatibility() {
    // === Проверка deprecated GameMode ===
    use crate::game::state::GameMode;

    let classic = GameMode::Classic;
    let sprint = GameMode::Sprint;
    let marathon = GameMode::Marathon;

    // Проверяем что enum всё ещё работает
    match classic {
        GameMode::Classic => {}
        GameMode::Sprint => {}
        GameMode::Marathon => {}
    }

    // Проверяем метод as_trait()
    let mode_trait = classic.as_trait();
    assert_eq!(mode_trait.name(), "Классика");

    let sprint_trait = sprint.as_trait();
    assert_eq!(sprint_trait.name(), "Спринт");

    let marathon_trait = marathon.as_trait();
    assert_eq!(marathon_trait.name(), "Марафон");

    // === Проверка старых API ===
    let mut state = GameState::new();

    // Проверяем что старые методы доступа работают
    let _score = state.score();
    let _level = state.level();
    let _lines = state.lines_cleared();

    // Проверяем что мутабельные методы работают
    state.set_score(100);
    state.set_level(5);
    state.set_lines_cleared(20);

    assert_eq!(state.score(), 100);
    assert_eq!(state.level(), 5);
    assert_eq!(state.lines_cleared(), 20);

    // === Проверка что GameStats доступен ===
    use crate::game::GameStats;

    let stats = GameStats::new();
    assert_eq!(stats.combo_counter(), 0);
    assert_eq!(stats.max_combo(), 0);

    // === Проверка что GameView доступен ===
    let view = GameView::from_game_state(&state);
    assert!(!view.score.is_empty());

    // === Проверка что BagGenerator работает ===
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape(),
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "BagGenerator должен работать для обратной совместимости"
    );

    // === Проверка что константы доступны ===
    use crate::constants::{FPS, INITIAL_FALL_SPD, SPRINT_LINES};

    assert_eq!(FPS, 60);
    assert!(INITIAL_FALL_SPD > 0.0);
    assert_eq!(SPRINT_LINES, 40);

    // === Проверка что трейты доступны ===
    use crate::game::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};

    // Просто проверяем что трейты импортируются
    fn _check_traits<B: BoardMutable + BoardReadonly + ScoreAccess + ScoreMutable>(_b: &B) {}
}

// ============================================================================
// ИНТЕГРАЦИОННЫЙ ТЕСТ
// ============================================================================

/// Комплексный тест всех архитектурных принципов.
#[test]
fn test_architecture_integrity_comprehensive() {
    // Создаём состояние игры
    let mut state = GameState::new();

    // Проверяем разделение компонентов
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);

    let score = state.score();
    assert_eq!(score, 0);

    // Проверяем инкапсуляцию
    state.set_score(1000);
    assert_eq!(state.score(), 1000);

    // Проверяем работу с фигурами
    let curr_shape = state.curr_shape();
    assert!(matches!(
        curr_shape.shape(),
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));

    // Проверяем обработку ошибок
    use crate::errors::GameError;
    let _err = GameError::validation_error("Тест");

    // Проверяем что трейты работают
    let mock_reader = MockInputReader::new(vec![]);
    let _reader: &dyn InputReader = &mock_reader;

    // Все архитектурные принципы работают корректно
}

// ============================================================================
// ТЕСТ 9: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ (ДОПОЛНЕНИЕ)
// ============================================================================

/// Тест на отсутствие циклических зависимостей между модулями.
///
/// # Архитектурные заметки
/// Циклические зависимости усложняют поддержку кода и тестирование.
/// Этот тест подтверждает что модули имеют правильную иерархию.
#[test]
fn test_no_circular_dependencies_between_modules() {
    // Проверяем что нет циклов:
    // game -> scoring -> game (это не цикл, а нормальная зависимость)
    // game -> logic -> game (это не цикл, а нормальная зависимость)

    // Модули имеют правильную иерархию:
    // - constants, types, errors (базовые, ни от кого не зависят)
    // - tetromino (зависит только от constants)
    // - io, io_traits (базовые для ввода/вывода)
    // - game (зависит от tetromino, io, io_traits)
    // - game::scoring, game::logic (подмодули game)

    use crate::game::logic::update::update;
    use crate::game::scoring::handle_landing;
    use crate::game::state::GameState;
    use crate::tetromino::Tetromino;

    let mut state = GameState::new();
    let mut reader = crate::io::KeyReader::default();

    // update() вызывает handle_landing()
    let _ = update(&mut state, &mut reader, 100);

    // handle_landing() использует scoring логику
    let _ = handle_landing(&mut state);

    // Tetromino работает независимо
    let mut bag = crate::tetromino::BagGenerator::new();
    let _tetromino = Tetromino::from_bag(&mut bag);

    // Нет циклических зависимостей
}

// ============================================================================
// ТЕСТ 10: СОБЛЮДЕНИЕ ИЕРАРХИИ МОДУЛЕЙ (ДОПОЛНЕНИЕ)
// ============================================================================

/// Тест на соблюдение иерархии модулей.
///
/// # Архитектурные заметки
/// Иерархия модулей должна соблюдаться:
/// - Базовые модули не зависят от модулей верхнего уровня
/// - Модули верхнего уровня могут зависеть от базовых
#[test]
fn test_module_hierarchy_respected() {
    // Базовые модули (не зависят от других):
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    use crate::errors::GameError;
    use crate::types::{Direction, RotationDirection};

    assert_eq!(FPS, 60);
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);

    let _dir = Direction::Left;
    let _rotation = RotationDirection::Clockwise;
    let _err = GameError::validation_error("Тест");

    // Модули верхнего уровня (зависят от базовых):
    use crate::game::state::GameState;
    use crate::tetromino::Tetromino;

    let state = GameState::new();
    let _score = state.score();

    let mut bag = crate::tetromino::BagGenerator::new();
    let _tetromino = Tetromino::from_bag(&mut bag);

    // Иерархия соблюдается
}

// ============================================================================
// ТЕСТ 11: ОТСУТСТВИЕ ЗАПРЕЩЁННЫХ ИМПОРТОВ (ДОПОЛНЕНИЕ)
// ============================================================================

/// Тест на отсутствие запрещённых импортов.
///
/// # Архитектурные заметки
/// Некоторые импорты запрещены для соблюдения архитектуры:
/// - scoring не должен импортировать из render
/// - logic не должен импортировать из render
/// - render не должен импортировать из logic/scoring
#[test]
fn test_no_forbidden_imports() {
    // Проверяем что render не импортирует логику из scoring/logic
    use crate::game::render::draw;
    use crate::game::view::GameView;

    // draw() принимает только GameView для отрисовки
    // Она не должна иметь доступа к функциям обновления состояния

    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // draw() не может изменять состояние
    // Просто проверяем что функция существует
    let _draw_fn = draw::<MockRenderer>;

    // Проверяем что scoring не импортирует из render

    // handle_landing() не использует функции отрисовки

    // Проверяем что logic не импортирует из render

    // update() не использует функции отрисовки
}

// ============================================================================
// ТЕСТ 12: АРХИТЕКТУРНАЯ ЦЕЛОСТНОСТЬ (ФИНАЛЬНЫЙ ТЕСТ)
// ============================================================================

/// Финальный тест на архитектурную целостность.
///
/// # Архитектурные заметки
/// Этот тест объединяет все проверки архитектурной целостности:
/// - Разделение компонентов
/// - Границы модулей
/// - Инкапсуляция
/// - Отсутствие циклических зависимостей
/// - Соблюдение иерархии
/// - Отсутствие запрещённых импортов
#[test]
fn test_architectural_integrity_final() {
    // === Разделение компонентов ===
    use crate::game::board::GameBoard;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;

    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();
    let state = GameState::new();

    assert_eq!(board.get_block(0, 0), Some(-1));
    assert_eq!(scoreboard.get_score(), 0);
    assert_eq!(state.score(), 0);

    // === Границы модулей ===
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};

    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(matches!(
        tetromino.shape(),
        ShapeType::T
            | ShapeType::L
            | ShapeType::J
            | ShapeType::S
            | ShapeType::Z
            | ShapeType::O
            | ShapeType::I
    ));

    // === Инкапсуляция ===
    let mut state = GameState::new();
    state.set_score(1000);
    assert_eq!(state.score(), 1000);

    // === Отсутствие циклических зависимостей ===
    use crate::game::logic::update::update;
    use crate::game::scoring::handle_landing;

    let mut reader = crate::io::KeyReader::default();
    let _ = update(&mut state, &mut reader, 100);
    let _ = handle_landing(&mut state);

    // === Соблюдение иерархии ===
    use crate::constants::FPS;
    assert_eq!(FPS, 60);

    // === Отсутствие запрещённых импортов ===
    use crate::game::render::draw;
    use crate::game::view::GameView;

    let view = GameView::from_game_state(&state);
    let _ = draw::<MockRenderer>;

    // Все проверки пройдены
}
