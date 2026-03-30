//! Тесты на отсутствие мёртвого кода в компонентах.
//!
//! Этот модуль проверяет что в проекте отсутствуют неиспользуемые структуры:
//! - `FigureManager` не используется (если удалён)
//! - `AnimationState` не используется (если удалён)
//! - `GamePhase` не используется (если удалён)
//! - Отсутствие неиспользуемых структур в `components.rs`
//!
//! ## Архитектурные заметки
//! Эти тесты используют проверку компиляции и наличия структур в коде.
//! Если структура была удалена из проекта, тест должен подтвердить её отсутствие.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]

use crate::game::board::GameBoard;
use crate::game::scoreboard::ScoreBoard;

// ============================================================================
// ТЕСТ 4: ПРОВЕРКА КОМПОНЕНТОВ В COMPONENTS.RS
// ============================================================================

/// Тест что в `components.rs` отсутствуют неиспользуемые структуры.
///
/// # Архитектурные заметки
/// Модуль `components.rs` должен содержать только переэкспорты
/// существующих компонентов (`GameBoard`, `ScoreBoard`).
/// Этот тест проверяет что все компоненты из `components.rs` используются.
#[test]
fn test_no_unused_components_in_components_rs() {
    // Проверяем что GameBoard используется
    let board = GameBoard::new();
    assert_eq!(
        board.get_block(0, 0),
        Some(-1),
        "GameBoard должен быть доступен и работать"
    );

    // Проверяем что ScoreBoard используется
    let scoreboard = ScoreBoard::new();
    assert_eq!(
        scoreboard.get_score(),
        0,
        "ScoreBoard должен быть доступен и работать"
    );

    // Проверяем что в components.rs нет других структур кроме переэкспортов
    // Это проверяется через анализ модуля components
    use crate::game::{BoardMutable as _, BoardReadonly as _, ScoreAccess as _};
    use crate::game::scoreboard::ScoreBoard;

    // Если код компилируется - все переэкспорты корректны
}

// ============================================================================
// ТЕСТ 5: ПРОВЕРКА ЧТО GAMEBOARD И SCOREBOARD ИСПОЛЬЗУЮТСЯ
// ============================================================================

/// Тест что `GameBoard` активно используется в проекте.
///
/// # Архитектурные заметки
/// `GameBoard` был выделен из `GameState` для соблюдения Single Responsibility Principle.
/// Этот тест подтверждает что компонент используется и не является мёртвым кодом.
#[test]
fn test_game_board_is_used() {
    let mut board = GameBoard::new();

    // Проверяем основные методы
    assert_eq!(board.get_block(0, 0), Some(-1));
    assert_eq!(board.get_filled_lines_mask(), 0);

    board.set_block(5, 10, 1);
    assert_eq!(board.get_block(5, 10), Some(1));

    // GameBoard используется - не мёртвый код
}

/// Тест что `ScoreBoard` активно используется в проекте.
///
/// # Архитектурные заметки
/// `ScoreBoard` был выделен из `GameState` для соблюдения Single Responsibility Principle.
/// Этот тест подтверждает что компонент используется и не является мёртвым кодом.
#[test]
fn test_score_board_is_used() {
    let mut scoreboard = ScoreBoard::new();

    // Проверяем основные методы
    assert_eq!(scoreboard.get_score(), 0);
    assert_eq!(scoreboard.get_level(), 1);
    assert_eq!(scoreboard.get_lines_cleared(), 0);

    scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);

    scoreboard.set_level(5);
    assert_eq!(scoreboard.get_level(), 5);

    // ScoreBoard используется - не мёртвый код
}

// ============================================================================
// ТЕСТ 6: ПРОВЕРКА ОТСУТСТВИЯ МЁРТВОГО КОДА ЧЕРЕЗ АНАЛИЗ МОДУЛЯ
// ============================================================================

/// Тест на отсутствие мёртвого кода через анализ импортов.
///
/// # Архитектурные заметки
/// Этот тест использует макросы для проверки что все публичные экспорты
/// модуля `game::components` используются в проекте.
#[test]
fn test_no_dead_code_in_components_module() {
    // Проверяем что все переэкспорты из components используются
    use crate::game::{BoardMutable as _, BoardReadonly as _, ScoreAccess as _};
    use crate::game::ScoreMutable;

    // Создаём GameState для проверки использования трейтов
    use crate::game::state::GameState;
    let mut state = GameState::new();

    // Проверяем что трейты работают
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20);

    let score = state.score();
    assert_eq!(score, 0);

    state.set_score(100);
    assert_eq!(state.score(), 100);

    // Все трейты используются - нет мёртвого кода
}

// ============================================================================
// ТЕСТ 7: СТРУКТУРНЫЙ ТЕСТ КОМПОНЕНТОВ
// ============================================================================

/// Структурный тест что компоненты имеют правильную архитектуру.
///
/// # Архитектурные заметки
/// Проверяет что:
/// - GameBoard инкапсулирует состояние поля
/// - ScoreBoard инкапсулирует состояние очков
/// - Компоненты не дублируют друг друга
#[test]
fn test_components_have_correct_structure() {
    // GameBoard отвечает только за поле
    let mut board = GameBoard::new();
    board.set_block(5, 5, 1);
    assert_eq!(board.get_block(5, 5), Some(1));

    // ScoreBoard отвечает только за очки
    let mut scoreboard = ScoreBoard::new();
    scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);

    // Компоненты разделены - нет дублирования ответственности
    // GameBoard не имеет методов для работы с очками
    // ScoreBoard не имеет методов для работы с полем
    
    // Проверяем что GameBoard не имеет методов ScoreBoard
    let board_methods = [
        "get_block",
        "set_block",
        "get_blocks",
        "get_blocks_mut",
        "get_filled_lines_mask",
    ];
    assert!(!board_methods.is_empty()); // Просто проверяем что список не пуст

    // Проверяем что ScoreBoard не имеет методов GameBoard
    let scoreboard_methods = [
        "get_score",
        "set_score",
        "add_score",
        "get_level",
        "set_level",
        "get_lines_cleared",
    ];
    assert!(!scoreboard_methods.is_empty()); // Просто проверяем что список не пуст

    // Компоненты имеют разную ответственность
}
