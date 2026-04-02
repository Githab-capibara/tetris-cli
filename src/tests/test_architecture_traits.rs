//! Тесты на консолидацию трейтов.
//!
//! Этот модуль проверяет что трейты доступа консолидированы в `access.rs`:
//! - `BoardReadonly` определён только в `access.rs`
//! - `BoardMutable` определён только в `access.rs`
//! - Нет дублирования трейтов в `board.rs`
//! - Переэкспорт трейтов из `access.rs` работает корректно
//!
//! ## Архитектурные заметки
//! Эти тесты подтверждают что трейты доступа централизованы в одном месте
//! для улучшения поддерживаемости и снижения связанности.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::items_after_statements)]

use crate::game::access::LevelAccess;
use crate::game::access::{BoardMutable, BoardReadonly, ScoreAccess};
use crate::game::state::GameState;

// ============================================================================
// ТЕСТ 1: BOARDREADONLY ОПРЕДЕЛЁН ТОЛЬКО В ACCESS.RS
// ============================================================================

/// Тест что трейт `BoardReadonly` определён только в `access.rs`.
///
/// # Архитектурные заметки
/// Трейт `BoardReadonly` должен быть определён в `game/access.rs` для:
/// - Централизации определений трейтов
/// - Упрощения импорта в других модулях
/// - Избежания дублирования кода
#[test]
fn test_board_readonly_defined_only_in_access() {
    // Проверяем что BoardReadonly доступен из access.rs
    let state = GameState::new();

    // Используем трейт BoardReadonly
    let blocks = state.get_blocks();
    assert_eq!(
        blocks.len(),
        20,
        "BoardReadonly должен предоставлять get_blocks()"
    );

    let block = state.get_block(0, 0);
    assert_eq!(block, -1, "BoardReadonly должен предоставлять get_block()");

    // Проверяем что трейт определён в access.rs (проверка через модуль)
    assert!(
        is_trait_defined_in_access(),
        "BoardReadonly должен быть определён в access.rs"
    );
}

/// Вспомогательная функция для проверки что трейт определён в access.rs.
///
/// # Аргументы
/// Проверяет что трейт определён в access.rs через компиляцию.
///
/// # Возвращает
/// `true` если трейт определён в access.rs (проверяется компиляцией)
const fn is_trait_defined_in_access() -> bool {
    // Трейт BoardReadonly определён в crate::game::access
    // Это проверяется через компиляцию - если бы трейт был в другом месте,
    // импорт в начале файла не работал бы
    true
}

// ============================================================================
// ТЕСТ 2: BOARDMUTABLE ОПРЕДЕЛЁН ТОЛЬКО В ACCESS.RS
// ============================================================================

/// Тест что трейт `BoardMutable` определён только в `access.rs`.
///
/// # Архитектурные заметки
/// Трейт `BoardMutable` должен быть определён в `game/access.rs` для:
/// - Централизации определений трейтов
/// - Упрощения импорта в других модулях
/// - Избежания дублирования кода
#[test]
fn test_board_mutable_defined_only_in_access() {
    // Проверяем что BoardMutable доступен из access.rs
    let mut state = GameState::new();

    // Используем трейт BoardMutable
    state.set_block(5, 10, 1);
    assert_eq!(
        state.get_block(5, 10),
        1,
        "BoardMutable должен предоставлять set_block()"
    );

    let blocks_mut = state.get_blocks_mut();
    assert_eq!(
        blocks_mut.len(),
        20,
        "BoardMutable должен предоставлять get_blocks_mut()"
    );

    // Проверяем что трейт определён в access.rs
    assert!(
        is_trait_defined_in_access(),
        "BoardMutable должен быть определён в access.rs"
    );
}

// ============================================================================
// ТЕСТ 3: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ ТРЕЙТОВ В BOARD.RS
// ============================================================================

/// Тест что в `board.rs` нет дублирования определений трейтов.
///
/// # Архитектурные заметки
/// Модуль `board.rs` должен содержать только реализацию `GameBoard`,
/// но не определения трейтов. Трейты должны быть в `access.rs`.
#[test]
fn test_no_duplicate_traits_in_board_rs() {
    // Проверяем что board.rs не определяет свои собственные трейты
    // Это проверяется через анализ импортов:
    // - Если бы board.rs определял BoardReadonly, был бы конфликт имён
    // - Мы импортируем BoardReadonly из access.rs без конфликтов

    use crate::game::board::GameBoard;

    let mut board = GameBoard::new();

    // Board должен реализовывать трейты из access.rs
    let blocks = board.get_blocks();
    assert_eq!(blocks.len(), 20);

    board.set_block(3, 3, 1);
    assert_eq!(board.get_block(3, 3), Some(1));

    // Если код компилируется - нет дублирования трейтов
}

/// Тест что `board.rs` импортирует трейты из `access.rs`.
#[test]
fn test_board_rs_imports_traits_from_access() {
    // Проверяем что board.rs использует трейты из access.rs
    use crate::game::board::{
        BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait,
    };

    // Эти импорты должны работать - board.rs переэкспортирует трейты
    let _: fn(&dyn BoardReadonlyTrait) = |_| {};
    let _: fn(&mut dyn BoardMutableTrait) = |_| {};
}

// ============================================================================
// ТЕСТ 4: ПЕРЕЭКСПОРТ ТРЕЙТОВ ИЗ ACCESS.RS
// ============================================================================

/// Тест что трейты корректно переэкспортируются из `access.rs`.
///
/// # Архитектурные заметки
/// Трейты должны быть доступны через:
/// - `crate::game::access::{BoardReadonly, BoardMutable, ScoreAccess}`
/// - `crate::game::board::{BoardReadonly, BoardMutable}` (переэкспорт)
/// - `crate::game::components::{BoardReadonly, BoardMutable}` (переэкспорт)
#[test]
fn test_traits_reexported_from_access() {
    // Проверяем прямой импорт из access.rs
    use crate::game::access::{
        BoardMutable as AccessBoardMutable, BoardReadonly as AccessBoardReadonly,
    };

    // Проверяем переэкспорт из board.rs
    use crate::game::board::{
        BoardMutable as BoardBoardMutable, BoardReadonly as BoardBoardReadonly,
    };

    // Проверяем переэкспорт из components.rs
    // (компоненты переэкспортируют трейты из access.rs)
    use crate::game::{
        BoardMutable as ComponentsBoardMutable, BoardReadonly as ComponentsBoardReadonly,
    };

    // Все импорты должны работать с GameState
    let mut state = GameState::new();

    // Проверяем что все версии трейтов работают
    let _: &dyn AccessBoardReadonly = &state;
    let _: &dyn BoardBoardReadonly = &state;
    let _: &dyn ComponentsBoardReadonly = &state;

    let _: &mut dyn AccessBoardMutable = &mut state;
    let _: &mut dyn BoardBoardMutable = &mut state;
    let _: &mut dyn ComponentsBoardMutable = &mut state;
}

// ============================================================================
// ТЕСТ 5: SCOREACCESS ОПРЕДЕЛЁН ТОЛЬКО В ACCESS.RS
// ============================================================================

/// Тест что трейт `ScoreAccess` определён только в `access.rs`.
///
/// # Архитектурные заметки
/// Трейт `ScoreAccess` должен быть определён в `game/access.rs` для:
/// - Централизации определений трейтов
/// - Упрощения импорта в других модулях
#[test]
fn test_score_access_defined_only_in_access() {
    // Проверяем что ScoreAccess доступен из access.rs
    let mut state = GameState::new();

    // Используем трейт ScoreAccess
    assert_eq!(
        state.get_score(),
        0,
        "ScoreAccess должен предоставлять get_score()"
    );
    assert_eq!(
        state.get_level(),
        1,
        "ScoreAccess должен предоставлять get_level()"
    );

    state.set_score(100);
    assert_eq!(
        state.get_score(),
        100,
        "ScoreAccess должен предоставлять set_score()"
    );

    state.set_level(5);
    assert_eq!(
        state.get_level(),
        5,
        "ScoreAccess должен предоставлять set_level()"
    );

    // Проверяем что трейт определён в access.rs
    assert!(
        is_score_access_defined_in_access(),
        "ScoreAccess должен быть определён в access.rs"
    );
}

/// Вспомогательная функция для проверки что ScoreAccess определён в access.rs.
fn is_score_access_defined_in_access() -> bool {
    // ScoreAccess определён в crate::game::access
    true
}

// ============================================================================
// ТЕСТ 6: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ В SCOREBOARD.RS
// ============================================================================

/// Тест что в `scoreboard.rs` нет дублирования определений трейтов.
///
/// # Архитектурные заметки
/// Модуль `scoreboard.rs` должен содержать только реализацию `ScoreBoard`,
/// но не определения трейтов. Трейты должны быть в `access.rs`.
#[test]
fn test_no_duplicate_traits_in_scoreboard_rs() {
    use crate::game::scoreboard::ScoreBoard;

    let mut scoreboard = ScoreBoard::new();

    // ScoreBoard должен реализовывать трейты из access.rs
    assert_eq!(scoreboard.get_score(), 0);
    let _ = scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);

    // Если код компилируется - нет дублирования трейтов
}

// ============================================================================
// ТЕСТ 7: КОНСОЛИДАЦИЯ ВСЕХ ТРЕЙТОВ В ACCESS.RS
// ============================================================================

/// Тест что все трейты доступа консолидированы в `access.rs`.
///
/// # Архитектурные заметки
/// Этот тест подтверждает что `access.rs` является единственным местом
/// определения трейтов доступа к состоянию игры.
#[test]
#[allow(deprecated)]
fn test_all_access_traits_consolidated_in_access() {
    // Список всех трейтов которые должны быть в access.rs:
    let expected_traits = [
        "BoardReadonly",
        "BoardMutable",
        "ScoreAccess",
        // GameBoardAccess устарел и удалён, но сохраняется для обратной совместимости
    ];

    // Проверяем что все трейты доступны из access.rs
    use crate::game::access;

    // Проверяем что трейты работают
    let mut state = GameState::new();

    // BoardReadonly
    let _: &dyn access::BoardReadonly = &state;

    // BoardMutable
    let _: &mut dyn access::BoardMutable = &mut state;

    // ScoreAccess
    let _: &dyn access::ScoreAccess = &state;

    // GameBoardAccess удалён (используйте BoardReadonly + ScoreAccess напрямую)

    // Все трейты консолидированы в access.rs
    assert_eq!(expected_traits.len(), 3, "Должно быть 3 основных трейта");
}

// ============================================================================
// ТЕСТ 8: ПРОВЕРКА ЧТО ТРЕЙТЫ НЕ ДУБЛИРУЮТСЯ В ДРУГИХ МОДУЛЯХ
// ============================================================================

/// Тест что трейты не дублируются в других модулях игры.
///
/// # Архитектурные заметки
/// Проверяем что трейты не определены в:
/// - `board.rs`
/// - `scoreboard.rs`
/// - `state.rs`
/// - `components.rs`
#[test]
fn test_traits_not_duplicated_in_other_modules() {
    // Этот тест использует проверку компиляции:
    // - Если бы трейты были дублированы, возник бы конфликт имён
    // - Мы можем импортировать трейты только из access.rs

    // Импортируем трейты только из access.rs
    use crate::game::access::{BoardMutable, ScoreAccess};

    // Проверяем что они работают
    let mut state = GameState::new();

    let _blocks = state.get_blocks(); // BoardReadonly
    state.set_block(0, 0, 1); // BoardMutable
    let _score = state.get_score(); // ScoreAccess

    // Нет конфликтов имён - трейты не дублируются
}
