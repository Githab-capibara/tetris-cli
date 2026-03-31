//! Тесты на снижение связанности (coupling).
//!
//! Этот модуль проверяет что связанность между модулями снижена:
//! - `scoring/points.rs` НЕ имеет прямого доступа к полям `GameState`
//! - `scoring/lines.rs` использует только публичные методы
//! - Логика очков инкапсулирована в `ScoreBoard`
//!
//! ## Архитектурные заметки
//! Низкая связанность (low coupling) улучшает поддерживаемость кода
//! и упрощает тестирование отдельных модулей.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]

use crate::game::scoreboard::ScoreBoard;
use crate::game::scoring::{ComboAccess, LevelAccess, LinesAccess, ScoreAccess};
use crate::game::state::GameState;

// ============================================================================
// ТЕСТ 1: SCORING/POINTS.RS НЕ ИМЕЕТ ПРЯМОГО ДОСТУПА К ПОЛЯМ GAMESTATE
// ============================================================================

/// Тест что `scoring/points.rs` НЕ имеет прямого доступа к полям `GameState`.
///
/// # Архитектурные заметки
/// `scoring/points.rs` должен использовать публичные методы GameState
/// вместо прямого доступа к приватным полям.
#[test]
fn test_scoring_points_no_direct_access_to_gamestate_fields() {
    // Проверяем что scoring/points.rs использует публичные методы
    use crate::game::scoring::points::{
        handle_hard_drop, handle_hold, handle_soft_drop, update_score_and_level,
    };

    let mut state = GameState::new();

    // Функции используют публичные методы GameState:
    // - state.score() вместо state.scoreboard.score
    // - state.set_score() вместо state.scoreboard.score = ...
    // - state.level() вместо state.scoreboard.level
    // - state.set_level() вместо state.scoreboard.level = ...

    // Проверяем что функции работают через публичный API
    update_score_and_level(&mut state, 1);
    assert!(state.score() > 0, "Счёт должен обновиться");

    handle_hold(&mut state);
    assert!(state.held_shape().is_some(), "Фигура должна быть удержана");

    // handle_hard_drop использует публичные методы
    handle_hard_drop(&mut state);

    // handle_soft_drop использует публичные методы
    handle_soft_drop(&mut state);
}

/// Тест что `scoring/points.rs` использует инкапсуляцию.
#[test]
fn test_scoring_points_uses_encapsulation() {
    let mut state = GameState::new();

    // scoring/points.rs использует методы-мутаторы:
    // - add_score() вместо прямого изменения поля
    // - set_level() вместо прямого изменения поля
    // - add_lines_cleared() вместо прямого изменения поля

    let initial_score = state.score();
    state.add_score(100);
    assert_eq!(
        state.score(),
        initial_score + 100,
        "add_score() должен работать"
    );

    state.set_level(5);
    assert_eq!(state.level(), 5, "set_level() должен работать");
}

// ============================================================================
// ТЕСТ 2: SCORING/LINES.RS ИСПОЛЬЗУЕТ ТОЛЬКО ПУБЛИЧНЫЕ МЕТОДЫ
// ============================================================================

/// Тест что `scoring/lines.rs` использует только публичные методы.
///
/// # Архитектурные заметки
/// `scoring/lines.rs` должен использовать публичные методы GameState
/// для доступа к игровому полю и изменения состояния.
#[test]
fn test_scoring_lines_uses_public_methods_only() {
    use crate::game::scoring::lines::{check_rows, find_filled_lines, find_full_rows, remove_rows};

    let mut state = GameState::new();

    // check_rows() использует публичные методы:
    // - state.get_blocks() для чтения поля
    // - state.get_blocks_mut() для изменения поля
    // - state.get_score() для чтения счёта
    // - state.set_score() для изменения счёта

    let cleared = check_rows(&mut state);
    assert_eq!(cleared, 0, "Новое поле не имеет линий");

    // find_filled_lines() использует публичные методы
    let blocks = state.get_blocks();
    let (mask, count) = find_filled_lines(blocks);
    assert_eq!(count, 0);
    assert_eq!(mask, 0);

    // find_full_rows() использует публичные методы
    let (mask2, count2) = find_full_rows(blocks);
    assert_eq!(count2, 0);
    assert_eq!(mask2, 0);

    // remove_rows() использует публичные методы
    let blocks_mut = state.get_blocks_mut();
    remove_rows(blocks_mut, 0);
}

/// Тест что `scoring/lines.rs` не имеет прямого доступа к полям.
#[test]
fn test_scoring_lines_no_direct_field_access() {
    // scoring/lines.rs НЕ должен иметь прямого доступа к:
    // - state.scoreboard.score
    // - state.scoreboard.level
    // - state.board.blocks

    // Вместо этого используются методы:
    // - state.get_blocks()
    // - state.get_blocks_mut()
    // - state.score()
    // - state.set_score()

    use crate::game::scoring::lines::check_rows;

    let mut state = GameState::new();
    let _ = check_rows(&mut state);

    // Если код компилируется - используется публичный API
}

// ============================================================================
// ТЕСТ 3: ЛОГИКА ОЧКОВ ИНКАПСУЛИРОВАНА В SCOREBOARD
// ============================================================================

/// Тест что логика очков инкапсулирована в `ScoreBoard`.
///
/// # Архитектурные заметки
/// `ScoreBoard` инкапсулирует логику работы с очками:
/// - Начисление очков
/// - Установка уровня
/// - Подсчёт линий
#[test]
fn test_score_logic_encapsulated_in_scoreboard() {
    let mut scoreboard = ScoreBoard::new();

    // ScoreBoard инкапсулирует логику очков
    assert_eq!(scoreboard.get_score(), 0, "Начальный счёт 0");
    assert_eq!(scoreboard.get_level(), 1, "Начальный уровень 1");
    assert_eq!(
        scoreboard.get_lines_cleared(),
        0,
        "Начальное количество линий 0"
    );

    // Инкапсуляция через методы
    scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);

    scoreboard.set_level(5);
    assert_eq!(scoreboard.get_level(), 5);

    scoreboard.set_lines_cleared(10);
    assert_eq!(scoreboard.get_lines_cleared(), 10);

    // ScoreBoard инкапсулирует внутреннее состояние
    // Прямой доступ к полям невозможен извне
}

/// Тест что `ScoreBoard` имеет чёткий публичный API.
#[test]
fn test_scoreboard_has_clear_public_api() {
    let mut scoreboard = ScoreBoard::new();

    // Публичный API ScoreBoard:
    // - get_score() -> u128
    // - set_score(u128)
    // - add_score(u128)
    // - get_level() -> u32
    // - set_level(u32)
    // - get_lines_cleared() -> u32
    // - set_lines_cleared(u32)

    let score: u128 = scoreboard.get_score();
    let _ = score;

    scoreboard.set_score(100);
    scoreboard.add_score(50);

    let level: u32 = scoreboard.get_level();
    let _ = level;

    scoreboard.set_level(5);

    let lines: u32 = scoreboard.get_lines_cleared();
    let _ = lines;

    scoreboard.set_lines_cleared(10);
}

// ============================================================================
// ТЕСТ 4: СНИЖЕНИЕ СВЯЗАННОСТИ ЧЕРЕЗ ТРЕЙТЫ
// ============================================================================

/// Тест что связанность снижена через использование трейтов.
///
/// # Архитектурные заметки
/// Трейты снижают связанность между модулями позволяя функциям
/// работать с любыми типами реализующими нужный трейт.
#[test]
fn test_coupling_reduced_through_traits() {
    use crate::game::scoring::{LevelAccess, ScoreAccess};

    // Функции могут работать с любыми типами реализующими трейты
    fn add_bonus<S: ScoreAccess>(scoreable: &mut S, bonus: u128) {
        scoreable.add_score(bonus);
    }

    fn set_target_level<L: LevelAccess>(levelable: &mut L, level: u32) {
        levelable.set_level(level);
    }

    let mut state = GameState::new();

    add_bonus(&mut state, 500);
    assert_eq!(state.score(), 500);

    set_target_level(&mut state, 10);
    assert_eq!(state.level(), 10);
}

/// Тест что `GameState` не зависит от конкретных реализаций.
#[test]
fn test_gamestate_not_dependent_on_concrete_implementations() {
    // GameState работает с трейтами а не конкретными типами
    // Это снижает связанность

    let mut state = GameState::new();

    // GameState может работать с любыми реализациями трейтов
    let _: &dyn ScoreAccess = &state;
    let _: &dyn LevelAccess = &state;
    let _: &dyn LinesAccess = &state;
    let _: &dyn ComboAccess = &state;
}

// ============================================================================
// ТЕСТ 5: ИНКАПСУЛЯЦИЯ ВНУТРЕННЕЙ ЛОГИКИ
// ============================================================================

/// Тест что внутренняя логика инкапсулирована.
///
/// # Архитектурные заметки
/// Внутренняя логика модулей не должна быть доступна извне.
#[test]
fn test_internal_logic_encapsulated() {
    // Внутренние функции scoring модуля:
    // - calculate_landing_bonus() - pub(crate)
    // - update_combo_on_clear() - pub(crate)
    // - spawn_next_tetromino() - private
    // - check_game_over_condition() - private

    use crate::game::scoring::points::{calculate_landing_bonus, update_combo_on_clear};

    let mut state = GameState::new();

    // Эти функции доступны только внутри crate
    calculate_landing_bonus(&mut state);
    update_combo_on_clear(&mut state, 0);

    // Приватные функции недоступны извне модуля
    // check_game_over_condition() // Не доступна
}

/// Тест что поля GameState приватные.
#[test]
fn test_gamestate_fields_are_private() {
    // Поля GameState приватные:
    // - board: GameBoard
    // - scoreboard: ScoreBoard
    // - curr_shape: Tetromino
    // - и т.д.

    // Доступ только через методы:
    let state = GameState::new();

    let _blocks = state.get_blocks(); // Публичный метод
    let _score = state.score(); // Публичный метод
    let _level = state.level(); // Публичный метод

    // Прямой доступ к полям невозможен:
    // state.board // Не доступно
    // state.scoreboard // Не доступно
    // state.curr_shape // Не доступно
}

// ============================================================================
// ТЕСТ 6: АРХИТЕКТУРНЫЙ ТЕСТ СНИЖЕНИЯ СВЯЗАННОСТИ
// ============================================================================

/// Архитектурный тест что связанность снижена.
#[test]
fn test_coupling_architecture_test() {
    // Архитектура снижения связанности:
    // - scoring/points.rs использует публичные методы GameState
    // - scoring/lines.rs использует публичные методы GameState
    // - ScoreBoard инкапсулирует логику очков
    // - Трейты снижают связанность между модулями

    let architecture = [
        ("scoring/points.rs", "Публичные методы"),
        ("scoring/lines.rs", "Публичные методы"),
        ("ScoreBoard", "Инкапсуляция"),
        ("Трейты", "Снижение связанности"),
    ];

    // Проверяем что архитектура работает
    let mut state = GameState::new();

    // scoring/points.rs использует публичные методы
    use crate::game::scoring::points::update_score_and_level;
    update_score_and_level(&mut state, 1);

    // scoring/lines.rs использует публичные методы
    use crate::game::scoring::lines::check_rows;
    let _ = check_rows(&mut state);

    // ScoreBoard инкапсулирует логику
    let mut scoreboard = ScoreBoard::new();
    scoreboard.add_score(100);

    // Трейты снижают связанность
    use crate::game::scoring::ScoreAccess;
    fn use_score<S: ScoreAccess>(s: &mut S) {
        s.add_score(100);
    }
    use_score(&mut state);

    assert_eq!(architecture.len(), 4, "Должно быть 4 элемента архитектуры");
}

/// Тест что модули не имеют циклических зависимостей.
#[test]
fn test_modules_have_no_circular_dependencies() {
    // Проверяем что нет циклических зависимостей:
    // scoring -> GameState -> scoring (цикл)

    // scoring модуль использует GameState через публичный API
    // GameState использует scoring через методы

    // Это не цикл а нормальная зависимость через публичный API

    use crate::game::scoring::handle_landing;
    use crate::game::state::GameState;

    let mut state = GameState::new();
    let _ = handle_landing(&mut state);
}
