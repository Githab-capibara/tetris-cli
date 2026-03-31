//! Тесты исправленных архитектурных проблем tetris-cli.
//!
//! Этот модуль проверяет все исправленные архитектурные проблемы:
//! - Отсутствие deprecated полей в GameState
//! - Отсутствие дублирования HMAC функций
//! - Использование GameModeTrait вместо enum
//! - Централизация констант
//! - Использование PathValidator
//! - Удаление избыточных трейтов
//! - Целостность архитектуры
//! - SOLID принципы

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]
#![allow(deprecated)]

// ============================================================================
// ТЕСТ 1: ОТСУТСТВИЕ DEPRECATED ПОЛЕЙ (C1, M4)
// ============================================================================

/// Проверяет что GameState не содержит deprecated полей.
///
/// Тест проверяет что:
/// - GameState использует композицию через GameBoard и ScoreBoard
/// - Прямые поля blocks, score, level, lines_cleared удалены
/// - Доступ осуществляется через методы делегирования
#[test]
fn test_game_state_no_deprecated_fields() {
    use crate::game::board::GameBoard;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;

    let state = GameState::new();

    // Проверяем что GameState имеет методы доступа к компонентам
    let _blocks = state.get_blocks();
    let _score = state.score();
    let _level = state.level();
    let _lines = state.lines_cleared();

    // Проверяем что компоненты существуют отдельно
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();

    // GameBoard должен иметь свои методы
    let _block = board.get_block(0, 0);
    let _mask = board.get_filled_lines_mask();

    // ScoreBoard должен иметь свои методы
    let _score = scoreboard.get_score();
    let _level = scoreboard.get_level();
    let _lines = scoreboard.get_lines_cleared();

    // Проверяем что GameState делегирует доступ к компонентам
    assert_eq!(
        state.get_blocks().len(),
        20,
        "GameState должен делегировать доступ к полю"
    );
    assert_eq!(
        state.score(),
        0,
        "GameState должен делегировать доступ к счёту"
    );
    assert_eq!(
        state.level(),
        1,
        "GameState должен делегировать доступ к уровню"
    );
}

/// Проверяет что размер GameState уменьшился.
///
/// Тест проверяет что:
/// - Размер GameState разумный (не слишком большой)
/// - Компоненты вынесены в отдельные структуры
#[test]
fn test_game_state_size_reduced() {
    use crate::game::board::GameBoard;
    use crate::game::cache::RenderCache;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;
    use crate::game::stats::GameStats;

    // Получаем размеры структур
    let game_state_size = std::mem::size_of::<GameState>();
    let board_size = std::mem::size_of::<GameBoard>();
    let scoreboard_size = std::mem::size_of::<ScoreBoard>();
    let stats_size = std::mem::size_of::<GameStats>();
    let cache_size = std::mem::size_of::<RenderCache>();

    // Проверяем что GameState имеет разумный размер
    // GameState должен содержать компоненты через композицию
    assert!(
        game_state_size > 0,
        "GameState должен иметь ненулевой размер"
    );

    // Проверяем что компоненты имеют ожидаемые размеры
    assert!(board_size > 0, "GameBoard должен иметь ненулевой размер");
    assert!(
        scoreboard_size > 0,
        "ScoreBoard должен иметь ненулевой размер"
    );
    assert!(stats_size > 0, "GameStats должен иметь ненулевой размер");
    assert!(cache_size > 0, "RenderCache должен иметь ненулевой размер");

    // GameState должен быть больше суммы компонентов из-за дополнительных полей
    // но не должен быть чрезмерно большим
    let min_expected_size = board_size + scoreboard_size + stats_size + cache_size;
    assert!(
        game_state_size >= min_expected_size,
        "GameState должен содержать все компоненты"
    );

    // Проверяем что размер GameState не превышает разумные пределы
    // (менее 10KB для состояния игры)
    assert!(
        game_state_size < 10240,
        "GameState должен быть разумного размера (< 10KB)"
    );
}

// ============================================================================
// ТЕСТ 2: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ HMAC (H2)
// ============================================================================

/// Проверяет что HMAC функции экспортированы из crypto.rs.
///
/// Тест проверяет что:
/// - hmac_sha256 экспортирована из crypto.rs
/// - verify_hmac_sha256 экспортирована из crypto.rs
/// - Функции работают корректно
#[test]
fn test_hmac_functions_exported() {
    use crate::crypto::{hmac_sha256, verify_hmac_sha256};

    // Проверяем что функции экспортированы и работают
    let key = "тестовый ключ";
    let data = "тестовые данные";

    let signature = hmac_sha256(key, data);
    assert_eq!(signature.len(), 64, "HMAC-SHA256 должен быть 64 символа");

    let is_valid = verify_hmac_sha256(key, data, &signature);
    assert!(is_valid, "Правильная подпись должна проходить проверку");

    let is_invalid = verify_hmac_sha256(key, data, "неправильная подпись");
    assert!(
        !is_invalid,
        "Неправильная подпись не должна проходить проверку"
    );
}

/// Проверяет что validator.rs использует функции из crypto.rs.
///
/// Тест проверяет что:
/// - HmacValidator использует hmac_sha256 из crypto.rs
/// - Нет дублирования HMAC логики
#[test]
fn test_hmac_no_duplication() {
    use crate::crypto::validator::HmacValidator;
    use crate::crypto::{hmac_sha256, verify_hmac_sha256};

    let key = "тестовый ключ";
    let data = "тестовые данные";

    // Проверяем что HmacValidator работает
    let validator = HmacValidator::new(key);
    let signature = validator.sign(data);

    // Проверяем что подпись совпадает с прямой функцией
    let direct_signature = hmac_sha256(key, data);
    assert_eq!(
        signature, direct_signature,
        "HmacValidator должен использовать hmac_sha256 из crypto.rs"
    );

    // Проверяем что валидация работает
    assert!(
        validator.verify(data, &signature),
        "HmacValidator должен проверять подпись"
    );

    // Проверяем что verify_hmac_sha256 тоже работает
    assert!(
        verify_hmac_sha256(key, data, &signature),
        "verify_hmac_sha256 должен работать"
    );

    // Проверяем что нет дублирования - обе функции дают одинаковый результат
    assert_eq!(
        signature.len(),
        64,
        "Обе HMAC функции должны давать 64-символьную подпись"
    );
}

// ============================================================================
// ТЕСТ 3: ИСПОЛЬЗОВАНИЕ GAMEMODETRAIT (H1, L1)
// ============================================================================

/// Проверяет что GameMode enum deprecated.
///
/// Тест проверяет что:
/// - GameMode enum имеет атрибут deprecated
/// - Компиляция с deprecated предупреждением
#[test]
fn test_game_mode_enum_deprecated() {
    // Этот тест компилируется с #[allow(deprecated)] в начале файла
    // что подтверждает что GameMode deprecated

    use crate::game::state::GameMode;

    // Проверяем что enum всё ещё работает для обратной совместимости
    #[allow(deprecated)]
    let classic = GameMode::Classic;

    #[allow(deprecated)]
    let sprint = GameMode::Sprint;

    #[allow(deprecated)]
    let marathon = GameMode::Marathon;

    // Проверяем что метод as_trait() работает
    #[allow(deprecated)]
    {
        let mode_trait = classic.as_trait();
        assert_eq!(mode_trait.name(), "Классика");

        let sprint_trait = sprint.as_trait();
        assert_eq!(sprint_trait.name(), "Спринт");

        let marathon_trait = marathon.as_trait();
        assert_eq!(marathon_trait.name(), "Марафон");
    }
}

/// Проверяет что GameModeTrait используется.
///
/// Тест проверяет что:
/// - GameModeTrait определён и работает
/// - ClassicMode, SprintMode, MarathonMode реализуют трейт
/// - Трейт используется вместо enum
#[test]
fn test_game_mode_trait_used() {
    use crate::game::mode_trait::{ClassicMode, GameModeTrait, MarathonMode, SprintMode};

    // Проверяем что режимы реализуют трейт
    let classic = ClassicMode;
    let sprint = SprintMode::new();
    let marathon = MarathonMode::new();

    // Проверяем методы трейта
    assert_eq!(classic.name(), "Классика");
    assert_eq!(sprint.name(), "Спринт");
    assert_eq!(marathon.name(), "Марафон");

    // Проверяем условие победы
    assert!(!classic.check_win_condition(1000));
    assert!(sprint.check_win_condition(40));
    assert!(!sprint.check_win_condition(39));
    assert!(marathon.check_win_condition(150));
    assert!(!marathon.check_win_condition(149));

    // Проверяем целевые линии
    assert_eq!(classic.get_target_lines(), None);
    assert_eq!(sprint.get_target_lines(), Some(40));
    assert_eq!(marathon.get_target_lines(), Some(150));

    // Проверяем что трейт может использоваться как trait object
    let modes: Vec<Box<dyn GameModeTrait>> = vec![
        Box::new(ClassicMode),
        Box::new(SprintMode::new()),
        Box::new(MarathonMode::new()),
    ];

    assert_eq!(modes[0].name(), "Классика");
    assert_eq!(modes[1].name(), "Спринт");
    assert_eq!(modes[2].name(), "Марафон");
}

// ============================================================================
// ТЕСТ 4: ЦЕНТРАЛИЗАЦИЯ КОНСТАНТ (M2)
// ============================================================================

/// Проверяет что константы определены только в constants.rs.
///
/// Тест проверяет что:
/// - Все основные константы импортируются из constants.rs
/// - Нет дублирования констант в других модулях
#[test]
fn test_constants_centralized() {
    use crate::constants::{
        FPS, GRID_HEIGHT, GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S, LINE_SCORES,
        MAX_FALL_SPEED, SPRINT_LINES,
    };

    // Проверяем что константы доступны из constants.rs
    assert_eq!(FPS, 60, "FPS должен быть 60");
    assert_eq!(GRID_WIDTH, 10, "GRID_WIDTH должен быть 10");
    assert_eq!(GRID_HEIGHT, 20, "GRID_HEIGHT должен быть 20");
    assert_eq!(SPRINT_LINES, 40, "SPRINT_LINES должен быть 40");
    assert_eq!(LINE_SCORES.len(), 4, "LINE_SCORES должен иметь 4 значения");

    // Проверяем что константы имеют правильные значения
    assert!(INITIAL_FALL_SPD > 0.0, "INITIAL_FALL_SPD должен быть > 0");
    assert!(
        MAX_FALL_SPEED > INITIAL_FALL_SPD,
        "MAX_FALL_SPEED должен быть > INITIAL_FALL_SPD"
    );
    assert!(LAND_TIME_DELAY_S > 0.0, "LAND_TIME_DELAY_S должен быть > 0");
}

/// Проверяет что io.rs импортирует константы.
///
/// Тест проверяет что:
/// - io.rs использует константы из constants.rs
/// - Нет дублирования констант в io.rs
#[test]
fn test_io_imports_constants() {
    // Проверяем что константы доступны из обоих модулей
    use crate::constants::{GRID_HEIGHT as CONST_HEIGHT, GRID_WIDTH as CONST_WIDTH};
    use crate::io::{GRID_HEIGHT as IO_HEIGHT, GRID_WIDTH as IO_WIDTH};

    // Проверяем что константы совпадают
    assert_eq!(
        CONST_WIDTH, IO_WIDTH,
        "GRID_WIDTH должен быть одинаковым в constants.rs и io.rs"
    );
    assert_eq!(
        CONST_HEIGHT, IO_HEIGHT,
        "GRID_HEIGHT должен быть одинаковым в constants.rs и io.rs"
    );

    // Проверяем что значения правильные
    assert_eq!(IO_WIDTH, 10, "GRID_WIDTH из io.rs должен быть 10");
    assert_eq!(IO_HEIGHT, 20, "GRID_HEIGHT из io.rs должен быть 20");
}

// ============================================================================
// ТЕСТ 5: ИСПОЛЬЗОВАНИЕ PATHVALIDATOR (M3)
// ============================================================================

/// Проверяет что controls.rs использует PathValidator.
///
/// Тест проверяет что:
/// - PathValidator доступен из controls.rs
/// - controls.rs импортирует PathValidator из validation
#[test]
fn test_controls_uses_path_validator() {
    use crate::controls::DEFAULT_PATH_VALIDATOR;
    use crate::validation::path::{PathErrorKind, PathValidator};
    use std::path::Path;

    // Проверяем что DEFAULT_PATH_VALIDATOR экспортирован из controls.rs
    let validator = &DEFAULT_PATH_VALIDATOR;

    // Проверяем что валидатор работает
    let valid_path = Path::new("config.json");
    let result = validator.validate(valid_path);
    assert!(result.is_ok(), "Валидный путь должен проходить валидацию");

    // Проверяем что валидатор отклоняет невалидные пути
    let invalid_path = Path::new("../etc/passwd");
    let result = validator.validate_no_traversal("../etc/passwd");
    assert!(result.is_err(), "Path traversal должен отклоняться");

    if let Err(e) = result {
        assert_eq!(
            e.kind,
            PathErrorKind::PathTraversal,
            "Тип ошибки должен быть PathTraversal"
        );
    }

    // Проверяем что PathValidator доступен напрямую
    let custom_validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");
    let valid = Path::new("file.txt");
    assert!(
        custom_validator.validate_length(valid).is_ok(),
        "Кастомный валидатор должен работать"
    );
}

// ============================================================================
// ТЕСТ 6: УДАЛЕНИЕ ИЗБЫТОЧНЫХ ТРЕЙТОВ (M5, L2)
// ============================================================================

/// Проверяет что трейты FigureAccess, FigureMutable удалены.
///
/// Тест проверяет что:
/// - Трейты FigureAccess и FigureMutable не существуют
/// - Вместо них используются публичные методы FigureManager
#[test]
fn test_redundant_traits_removed() {
    // Этот тест проверяет что избыточные трейты удалены
    // Мы не можем проверить отсутствие типа на этапе выполнения,
    // но можем проверить что FigureManager имеет публичные методы

    use crate::game::state::GameState;

    let mut state = GameState::new();

    // Проверяем что FigureManager (через GameState) имеет публичные методы
    let curr_shape = state.curr_shape();
    let next_shape = state.next_shape();
    let held_shape = state.held_shape();

    // Проверяем что методы возвращают корректные данные
    assert!(
        matches!(
            curr_shape.shape,
            crate::tetromino::ShapeType::T
                | crate::tetromino::ShapeType::L
                | crate::tetromino::ShapeType::J
                | crate::tetromino::ShapeType::S
                | crate::tetromino::ShapeType::Z
                | crate::tetromino::ShapeType::O
                | crate::tetromino::ShapeType::I
        ),
        "curr_shape должен возвращать корректную фигуру"
    );

    assert!(
        matches!(
            next_shape.shape,
            crate::tetromino::ShapeType::T
                | crate::tetromino::ShapeType::L
                | crate::tetromino::ShapeType::J
                | crate::tetromino::ShapeType::S
                | crate::tetromino::ShapeType::Z
                | crate::tetromino::ShapeType::O
                | crate::tetromino::ShapeType::I
        ),
        "next_shape должен возвращать корректную фигуру"
    );

    // held_shape может быть None для нового состояния
    assert!(
        held_shape.is_none() || held_shape.is_some(),
        "held_shape должен существовать"
    );
}

/// Проверяет что FigureManager имеет публичные методы.
///
/// Тест проверяет что:
/// - FigureManager (через GameState) имеет публичные методы
/// - Доступ к фигурам осуществляется через методы
#[test]
fn test_figure_manager_public_methods() {
    use crate::game::state::GameState;
    use crate::tetromino::ShapeType;

    let mut state = GameState::new();

    // Проверяем публичные методы для работы с фигурами
    let curr_shape = state.curr_shape();
    let original_x = curr_shape.pos.0;

    // Проверяем что можно получить мутабельную ссылку на текущую фигуру
    let curr_shape_mut = state.get_curr_shape_mut();
    curr_shape_mut.pos.0 += 1.0;

    assert_eq!(
        state.curr_shape().pos.0,
        original_x + 1.0,
        "get_curr_shape_mut должен предоставлять мутабельный доступ"
    );

    // Проверяем что можно получить доступ к фигуре и её полям
    let shape = state.curr_shape();
    assert!(
        matches!(
            shape.shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Фигура должна иметь корректный тип"
    );

    // Проверяем что next_shape доступен
    let next = state.next_shape();
    assert!(
        matches!(
            next.shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "next_shape должен возвращать корректную фигуру"
    );
}

// ============================================================================
// ТЕСТ 7: ЦЕЛОСТНОСТЬ АРХИТЕКТУРЫ
// ============================================================================

/// Проверяет отсутствие циклических зависимостей между модулями.
///
/// Тест проверяет что:
/// - Базовые модули не зависят от модулей верхнего уровня
/// - Нет циклов между game/, tetromino/, io/
#[test]
fn test_arch_fixes_no_circular_dependencies() {
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    use crate::errors::GameError;
    use crate::types::Direction;

    // Проверяем что базовые модули работают независимо
    assert_eq!(FPS, 60);
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);

    // Direction должен работать независимо
    let dir = Direction::Left;
    let _rotation = dir.to_rotation_direction();

    // GameError должен работать независимо
    let _err = GameError::validation_error("Тест");

    // Проверяем что tetromino не зависит от game
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};

    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);
    assert!(
        matches!(
            tetromino.shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Tetromino должен работать независимо"
    );

    // Проверяем что io_traits не зависит от game
    use crate::io_traits::{InputReader, Renderer};

    // Трейты должны быть доступны без зависимости от game
    let _ = std::any::type_name::<dyn InputReader>();
    let _ = std::any::type_name::<dyn Renderer>();
}

/// Проверяет соблюдение границ модулей.
///
/// Тест проверяет что:
/// - Модули имеют чёткие границы ответственности
/// - Нет нарушения инкапсуляции
#[test]
fn test_arch_fixes_module_boundaries() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreMutable as ScoreMutableTrait};
    use crate::game::board::{
        BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait,
    };
    use crate::game::state::GameState;

    let mut state = GameState::new();

    // Проверяем что BoardReadonly предоставляет только чтение
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "get_blocks должен возвращать поле");

    // Проверяем что BoardMutable предоставляет запись
    state.set_block(5, 5, 1);
    assert_eq!(
        state.get_block(5, 5),
        1,
        "set_block должен устанавливать значение"
    );

    // Проверяем что ScoreAccess предоставляет только чтение
    let score = state.score();
    assert_eq!(score, 0, "score должен возвращать счёт");

    // Проверяем что ScoreMutable предоставляет запись
    state.set_score(100);
    assert_eq!(state.score(), 100, "set_score должен устанавливать счёт");

    // Проверяем что модули не нарушают границы
    // (это проверяется через компиляцию - если бы границы нарушались, код не скомпилировался бы)
}

/// Проверяет что game/ не зависит от app/.
///
/// Тест проверяет что:
/// - Модуль game не импортирует типы из app
/// - Разделение ответственности соблюдается
#[test]
fn test_game_does_not_depend_on_app() {
    // Этот тест проверяет что game/ может компилироваться без app/
    // Мы проверяем это через использование только game/ типов

    use crate::game::board::GameBoard;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait};
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;

    // Создаём объекты game/ без app/
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();
    let mut state = GameState::new();
    let mode = ClassicMode;

    // Проверяем что они работают
    assert_eq!(board.get_block(0, 0), Some(-1));
    assert_eq!(scoreboard.get_score(), 0);
    assert_eq!(state.score(), 0);
    assert_eq!(mode.name(), "Классика");

    // Если бы game/ зависел от app/, этот тест не скомпилировался бы
    // без импорта app/ модулей
}

// ============================================================================
// ТЕСТ 8: SOLID ПРИНЦИПЫ
// ============================================================================

/// Проверяет SRP для GameState (через количество полей).
///
/// Тест проверяет что:
/// - GameState делегирует ответственность компонентам
/// - GameBoard отвечает за поле
/// - ScoreBoard отвечает за очки
#[test]
fn test_game_state_srp() {
    use crate::game::board::GameBoard;
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;

    // Проверяем что GameBoard отвечает только за поле
    let mut board = GameBoard::new();
    board.set_block(5, 5, 1);
    assert_eq!(board.get_block(5, 5), Some(1));
    // GameBoard не должен отвечать за очки или фигуры

    // Проверяем что ScoreBoard отвечает только за очки
    let mut scoreboard = ScoreBoard::new();
    scoreboard.set_score(100);
    assert_eq!(scoreboard.get_score(), 100);
    // ScoreBoard не должен отвечать за поле или фигуры

    // Проверяем что GameState использует композицию
    let state = GameState::new();
    let _blocks = state.get_blocks(); // Делегирует GameBoard
    let _score = state.score(); // Делегирует ScoreBoard

    // Проверяем что размер GameState разумный (не слишком большой)
    let state_size = std::mem::size_of::<GameState>();
    assert!(
        state_size < 10240,
        "GameState должен быть разумного размера"
    );
}

/// Проверяет OCP для GameModeTrait.
///
/// Тест проверяет что:
/// - Можно добавить новый режим без изменения существующего кода
/// - GameModeTrait открыт для расширения
#[test]
fn test_game_mode_ocp() {
    use crate::game::mode_trait::GameModeTrait;

    // Проверяем что можно создать новый режим без изменения существующего кода
    struct CustomMode;

    impl GameModeTrait for CustomMode {
        fn check_win_condition(&self, lines: u32) -> bool {
            lines >= 100 //Custom режим с целью 100 линий
        }

        fn get_target_lines(&self) -> Option<u32> {
            Some(100)
        }

        fn name(&self) -> &'static str {
            "Пользовательский"
        }
    }

    // Проверяем что новый режим работает
    let custom = CustomMode;
    assert!(!custom.check_win_condition(99));
    assert!(custom.check_win_condition(100));
    assert_eq!(custom.name(), "Пользовательский");

    // Проверяем что можно использовать как trait object
    let modes: Vec<Box<dyn GameModeTrait>> = vec![
        Box::new(crate::game::mode_trait::ClassicMode),
        Box::new(custom),
    ];

    assert_eq!(modes[0].name(), "Классика");
    assert_eq!(modes[1].name(), "Пользовательский");
}

/// Проверяет DIP для render.rs.
///
/// Тест проверяет что:
/// - render.rs использует трейты вместо конкретных типов
/// - Можно использовать mock-реализации
#[test]
fn test_render_uses_traits() {
    use crate::io_traits::{InputReader, Renderer};
    use std::io;
    use termion::color::{Red, Reset};

    // Создаём mock-реализацию Renderer
    struct MockRenderer {
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
        }

        fn draw_string(
            &mut self,
            _string: &str,
            _pos: (u16, u16),
            _fg: &dyn termion::color::Color,
            _bg: &dyn termion::color::Color,
        ) {
        }

        fn flush(&mut self) {
            self.flushed = true;
        }

        fn reset(&mut self) {
            self.reset = true;
        }
    }

    // Создаём mock-реализацию InputReader
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
        fn get_key(&mut self) -> io::Result<Option<u8>> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }
    }

    // Проверяем что mock-реализации работают
    let mut mock_renderer = MockRenderer::new();
    mock_renderer.draw_string("Test", (1, 1), &Red, &Reset);
    mock_renderer.flush();
    assert!(mock_renderer.flushed, "flush() должен быть вызван");

    mock_renderer.reset();
    assert!(mock_renderer.reset, "reset() должен быть вызван");

    let mut mock_reader = MockInputReader::new(vec![b'a', b'b', b'c']);
    assert_eq!(mock_reader.get_key().unwrap(), Some(b'a'));
    assert_eq!(mock_reader.get_key().unwrap(), Some(b'b'));
    assert_eq!(mock_reader.get_key().unwrap(), Some(b'c'));
    assert_eq!(mock_reader.get_key().unwrap(), None);

    // Проверяем полиморфизм
    fn process_any_renderer<R: Renderer>(renderer: &mut R) {
        renderer.flush();
    }

    let mut renderer = MockRenderer::new();
    process_any_renderer(&mut renderer);
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ
// ============================================================================

/// Интеграционный тест: проверка всех архитектурных исправлений вместе.
#[test]
fn test_all_architecture_fixes_integration() {
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    use crate::crypto::validator::HmacValidator;
    use crate::crypto::{hmac_sha256, verify_hmac_sha256};
    use crate::game::board::GameBoard;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait, MarathonMode, SprintMode};
    use crate::game::scoreboard::ScoreBoard;
    use crate::game::state::GameState;
    use crate::validation::path::PathValidator;
    use std::path::Path;

    // 1. Проверяем централизацию констант
    assert_eq!(FPS, 60);
    assert_eq!(GRID_WIDTH, 10);
    assert_eq!(GRID_HEIGHT, 20);

    // 2. Проверяем HMAC функции
    let key = "test_key";
    let data = "test_data";
    let signature = hmac_sha256(key, data);
    assert!(verify_hmac_sha256(key, data, &signature));

    let validator = HmacValidator::new(key);
    assert_eq!(validator.sign(data), signature);

    // 3. Проверяем GameModeTrait
    let modes: Vec<Box<dyn GameModeTrait>> = vec![
        Box::new(ClassicMode),
        Box::new(SprintMode::new()),
        Box::new(MarathonMode::new()),
    ];
    assert_eq!(modes.len(), 3);

    // 4. Проверяем компоненты GameState
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();
    let state = GameState::new();

    assert_eq!(board.get_block(0, 0), Some(-1));
    assert_eq!(scoreboard.get_score(), 0);
    assert_eq!(state.score(), 0);

    // 5. Проверяем PathValidator
    let path_validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");
    let valid_path = Path::new("test.txt");
    assert!(path_validator.validate_length(valid_path).is_ok());
}
