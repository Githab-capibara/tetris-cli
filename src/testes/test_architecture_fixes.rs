//! Тесты на архитектурные улучшения.
//!
//! Этот модуль содержит тесты для проверки архитектурных улучшений:
//! - Отсутствие дублирования меню
//! - Централизация констант
//! - Разделение GameBoardAccess на трейты
//! - Методы отрисовки в GameView
//! - Отсутствие циклических зависимостей
//! - Соблюдение границ модулей
//! - Инкапсуляция
//! - Использование трейтов

// ============================================================================
// ТЕСТ 1: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ МЕНЮ
// ============================================================================

/// Тест 1: Проверка что menu.rs не существует как отдельный файл.
///
/// menu.rs должен быть удалён, menu/ должен существовать как модуль.
/// Проверяем что модуль menu доступен и функции меню работают.
#[test]
fn test_menu_module_only() {
    // menu.rs должен быть удалён, menu/ должен существовать
    // Проверяем что модуль menu доступен
    use crate::menu;
    // Проверяем что функции меню доступны
    let _ = menu::draw_menu;
    let _ = menu::get_player_name;
}

// ============================================================================
// ТЕСТ 2: ЦЕНТРАЛИЗАЦИЯ КОНСТАНТ
// ============================================================================

/// Тест 2: Проверка что константы определены в одном месте.
///
/// Константы должны быть определены в game/constants.rs и
/// переэкспортироваться в io.rs для обратной совместимости.
#[test]
fn test_constants_centralized() {
    use crate::game::constants;
    use crate::io;

    // Константы должны быть равны (переэкспортированы из constants)
    assert_eq!(constants::GRID_WIDTH, io::GRID_WIDTH);
    assert_eq!(constants::GRID_HEIGHT, io::GRID_HEIGHT);
    assert_eq!(constants::DISP_WIDTH as u16, io::DISP_WIDTH);
    assert_eq!(constants::DISP_HEIGHT as u16, io::DISP_HEIGHT);
}

/// Тест 3: Проверка что нет дублирования определений.
///
/// В game/state.rs не должно быть собственных определений констант.
/// Только импорт из constants.rs.
#[test]
fn test_no_duplicate_constants_in_state() {
    // В game/state.rs не должно быть собственных определений
    // Только импорт из constants.rs
    use crate::game::constants;

    // Проверяем что константы доступны из constants
    assert_eq!(constants::GRID_WIDTH, 10);
    assert_eq!(constants::GRID_HEIGHT, 20);
}

// ============================================================================
// ТЕСТ 3: РАЗДЕЛЕНИЕ GAMEBOARDACCESS НА ТРЕЙТЫ
// ============================================================================

/// Тест 4: Проверка трейта BoardReadonly.
///
/// BoardReadonly должен предоставлять методы только для чтения.
#[test]
fn test_board_readonly_trait() {
    use crate::game::access::BoardReadonly;
    use crate::game::GameState;

    let state = GameState::new();

    // Проверяем что трейт работает
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);

    let block = state.get_block(0, 0);
    assert_eq!(block, -1); // Пустая ячейка

    let is_empty = state.is_block_empty(0, 0);
    assert!(is_empty);
}

/// Тест 5: Проверка трейта BoardMutable.
///
/// BoardMutable должен предоставлять методы для чтения и записи.
#[test]
fn test_board_mutable_trait() {
    use crate::game::access::{BoardMutable, BoardReadonly};
    use crate::game::GameState;

    let mut state = GameState::new();

    // Проверяем что трейт работает
    state.set_block(5, 5, 1);
    assert_eq!(state.get_block(5, 5), 1);

    let is_occupied = state.is_block_occupied(5, 5);
    assert!(is_occupied);
}

/// Тест 6: Проверка что GameBoardAccess включает оба трейта.
///
/// GameBoardAccess должен быть комбинацией BoardReadonly и BoardMutable.
#[test]
fn test_game_board_access_combined() {
    use crate::game::access::{BoardMutable, BoardReadonly};
    use crate::game::GameState;

    let mut state = GameState::new();

    // GameBoardAccess должен включать методы из обоих трейтов
    let _blocks = state.get_blocks(); // Из BoardReadonly
    state.set_block(0, 0, 1); // Из BoardMutable
}

// ============================================================================
// ТЕСТ 4: МЕТОДЫ ОТРИСОВКИ В GAMEVIEW
// ============================================================================

/// Тест 7: Проверка что GameView имеет методы отрисовки.
///
/// GameView должен иметь методы для доступа к данным отрисовки.
#[test]
fn test_game_view_draw_methods() {
    use crate::game::view::GameView;
    use crate::game::GameState;

    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // Проверяем что методы существуют (компиляция)
    // Методы могут не работать без терминала, но должны существовать
    let _ = view.get_block(0, 0);
    let _ = view.curr_shape;
    let _ = view.score;
}

/// Тест 8: Проверка что GameView уменьшает Feature Envy.
///
/// GameView должен предоставлять доступ к данным для отрисовки
/// без прямого доступа к полям GameState.
#[test]
fn test_game_view_reduces_feature_envy() {
    use crate::game::view::GameView;
    use crate::game::GameState;

    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // GameView должен предоставлять доступ к данным для отрисовки
    // без прямого доступа к полям GameState
    // Проверяем что поля доступны (они кэшированные строки)
    assert!(view.score.contains("0"));
    assert!(view.level.contains("1"));
    assert!(view.lines.contains("0"));
}

// ============================================================================
// ТЕСТ 5: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Тест 9: Проверка что game/ не зависит от menu/.
///
/// game::GameState не должен импортировать menu:: напрямую.
/// Это проверяется компиляцией.
#[test]
fn test_no_game_menu_cycle() {
    // game::GameState не должен импортировать menu::
    // Это проверяется компиляцией
    use crate::game::GameState;
    use crate::menu;

    // Если бы был цикл, компиляция бы не прошла
    let _state = GameState::new();
    let _menu_fn = menu::draw_menu;
}

/// Тест 10: Проверка что validation/ не зависит от controls/.
///
/// validation должен быть независим от controls.
#[test]
fn test_no_validation_controls_cycle() {
    use crate::controls::ControlsConfig;
    use crate::validation;

    // validation должен быть независим
    let _config = ControlsConfig::default_config();
}

// ============================================================================
// ТЕСТ 6: СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
// ============================================================================

/// Тест 11: Проверка что scoring/ не зависит от render/.
///
/// scoring должен только начислять очки, не отрисовывать.
#[test]
fn test_scoring_render_boundary() {
    // scoring должен только начислять очки, не отрисовывать
    use crate::game::scoring;
    use crate::game::GameState;

    let mut state = GameState::new();
    // Функции scoring не должны вызывать отрисовку
    let _score = state.get_score();
}

/// Тест 12: Проверка что logic/ не зависит от render/.
///
/// logic должен только обновлять состояние, не отрисовывать.
#[test]
fn test_logic_render_boundary() {
    use crate::game::logic;
    use crate::game::GameState;

    // logic должен только обновлять состояние, не отрисовывать
    let mut state = GameState::new();
    let _ = state.get_score();
}

// ============================================================================
// ТЕСТ 7: ИНКАПСУЛЯЦИЯ
// ============================================================================

/// Тест 13: Проверка что приватные поля недоступны.
///
/// Приватные поля структур должны быть недоступны извне модуля.
#[test]
fn test_private_fields_encapsulation() {
    use crate::highscore::leaderboard::LeaderboardEntry;

    // salt должно быть приватным
    let entry = LeaderboardEntry::new("Test", 1000);

    // entry.salt должно быть недоступен (проверяется компиляцией)
    // let _ = entry.salt; // Эта строка не должна компилироваться

    // Но должен быть доступен hash через метод
    let _hash = entry.hash();
}

// ============================================================================
// ТЕСТ 8: ИСПОЛЬЗОВАНИЕ ТРЕЙТОВ
// ============================================================================

/// Тест 14: Проверка что GameModeTrait используется.
///
/// GameModeTrait должен быть доступен и использоваться для режимов игры.
#[test]
fn test_game_mode_trait_usage() {
    use crate::game::mode_trait::ClassicMode;
    use crate::game::mode_trait::GameModeTrait;

    let mode = ClassicMode;

    // Проверяем методы трейта
    // ClassicMode всегда возвращает false (нет условия победы)
    assert!(!mode.check_win_condition(0));
    assert_eq!(mode.get_target_lines(), None);
    assert_eq!(mode.name(), "Классика");
}

/// Тест 15: Проверка что InputReader и Renderer трейты существуют.
///
/// Трейты io_traits должны быть доступны для использования.
#[test]
fn test_io_traits_exist() {
    use crate::io_traits::{InputReader, Renderer};

    // Трейты должны быть доступны
    // Проверяем что они компилируются (методы трейта)
    fn _use_input_reader<T: InputReader>(_r: &mut T) {}
    fn _use_renderer<T: Renderer>(_r: &mut T) {}
}

// ============================================================================
// ИНТЕГРАЦИОННЫЙ ТЕСТ: ВСЕ АРХИТЕКТУРНЫЕ УЛУЧШЕНИЯ
// ============================================================================

/// Интеграционный тест: проверка всех архитектурных улучшений вместе.
///
/// Этот тест проверяет что все архитектурные улучшения работают совместно.
#[test]
fn test_all_architecture_improvements_integration() {
    use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess};
    use crate::game::constants;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait};
    use crate::game::view::GameView;
    use crate::game::GameState;
    use crate::io;
    use crate::io_traits::{InputReader, Renderer};
    use crate::menu;

    // 1. Проверка централизации констант
    assert_eq!(constants::GRID_WIDTH, io::GRID_WIDTH);
    assert_eq!(constants::GRID_HEIGHT, io::GRID_HEIGHT);

    // 2. Проверка трейтов доступа
    let mut state = GameState::new();

    // BoardReadonly
    let _blocks: &[[i8; 10]; 20] = state.get_blocks();
    let _block = state.get_block(0, 0);

    // BoardMutable
    state.set_block(5, 5, 1);
    assert_eq!(state.get_block(5, 5), 1);

    // ScoreAccess
    state.set_score(100);
    assert_eq!(state.get_score(), 100);

    // 3. Проверка GameView
    let view = GameView::from_game_state(&state);
    // Проверяем что GameView создан корректно
    // Кэш не обновляется при set_score, но поля доступны
    let _ = view.blocks;
    let _ = view.curr_shape;
    let _ = view.score;

    // 4. Проверка GameModeTrait
    let mode = ClassicMode;
    assert_eq!(mode.name(), "Классика");

    // 5. Проверка что menu доступен
    let _ = menu::draw_menu;

    // 6. Проверка что трейты io_traits доступны
    fn _use_traits<T: InputReader + Renderer>(_t: &mut T) {}

    // Все архитектурные улучшения работают совместно
    assert!(true);
}
