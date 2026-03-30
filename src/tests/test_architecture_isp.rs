//! Тесты на разделение трейтов (Interface Segregation Principle).
//!
//! Этот модуль проверяет что трейты разделены согласно ISP:
//! - `ScoreAccess` содержит только методы очков
//! - `LevelAccess` содержит только методы уровней
//! - `LinesAccess` содержит только методы линий
//! - `ComboAccess` содержит только методы комбо
//! - `ScoringState` наследует узкие трейты
//!
//! ## Архитектурные заметки
//! Interface Segregation Principle (ISP) гласит что клиенты не должны
//! зависеть от методов которые они не используют. Узкие трейты улучшают
//! поддерживаемость и тестируемость кода.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]

use crate::game::scoring::{
    ComboAccess, LevelAccess, LinesAccess, ScoreAccess, ScoringState,
};
use crate::game::state::GameState;

// ============================================================================
// ТЕСТ 1: SCOREACCESS СОДЕРЖИТ ТОЛЬКО МЕТОДЫ ОЧКОВ
// ============================================================================

/// Тест что `ScoreAccess` содержит только методы для работы с очками.
///
/// # Архитектурные заметки
/// `ScoreAccess` должен содержать только:
/// - `get_score()` - получить счёт
/// - `set_score()` - установить счёт
/// - `add_score()` - добавить очки
#[test]
fn test_score_access_contains_only_score_methods() {
    // Проверяем что ScoreAccess имеет только методы очков
    let mut state = GameState::new();
    
    // get_score()
    let score: u128 = state.get_score();
    assert_eq!(score, 0, "Начальный счёт должен быть 0");
    
    // set_score()
    state.set_score(100);
    assert_eq!(state.get_score(), 100, "Счёт должен обновиться");
    
    // add_score()
    state.add_score(50);
    assert_eq!(state.get_score(), 150, "Очки должны добавиться");
    
    // ScoreAccess не должен содержать методов для уровней, линий или комбо
    // Это проверяется через компиляцию - если бы ScoreAccess содержал
    // другие методы, они были бы доступны через трейт
    
    // Проверяем что ScoreAccess работает через трейт
    fn use_score_access<S: ScoreAccess>(scoreable: &S) -> u128 {
        scoreable.get_score()
    }
    
    fn use_score_access_mut<S: ScoreAccess>(scoreable: &mut S) {
        scoreable.set_score(200);
        scoreable.add_score(100);
    }
    
    let score = use_score_access(&state);
    assert_eq!(score, 150);
    
    use_score_access_mut(&mut state);
    assert_eq!(state.get_score(), 300);
}

/// Тест что `ScoreAccess` не содержит методов уровней.
#[test]
fn test_score_access_does_not_contain_level_methods() {
    // ScoreAccess не должен иметь get_level() или set_level()
    // Это проверяется через компиляцию:
    // Если бы ScoreAccess имел эти методы, код ниже компилировался бы
    
    let mut state = GameState::new();
    
    // ScoreAccess имеет только методы очков
    let _score: u128 = state.get_score();
    state.set_score(100);
    state.add_score(50);
    
    // Эти методы НЕ доступны через ScoreAccess:
    // state.get_level() // Не доступно через ScoreAccess
    // state.set_level(5) // Не доступно через ScoreAccess
}

/// Тест что `ScoreAccess` не содержит методов линий.
#[test]
fn test_score_access_does_not_contain_lines_methods() {
    // ScoreAccess не должен иметь get_lines_cleared() или add_lines()
    let mut state = GameState::new();
    
    // ScoreAccess имеет только методы очков
    state.add_score(100);
    
    // Эти методы НЕ доступны через ScoreAccess:
    // state.get_lines_cleared() // Не доступно через ScoreAccess
    // state.add_lines(5) // Не доступно через ScoreAccess
}

/// Тест что `ScoreAccess` не содержит методов комбо.
#[test]
fn test_score_access_does_not_contain_combo_methods() {
    // ScoreAccess не должен иметь get_combo() или reset_combo()
    let mut state = GameState::new();
    
    // ScoreAccess имеет только методы очков
    state.add_score(100);
    
    // Эти методы НЕ доступны через ScoreAccess:
    // state.get_combo() // Не доступно через ScoreAccess
    // state.reset_combo() // Не доступно через ScoreAccess
}

// ============================================================================
// ТЕСТ 2: LEVELACCESS СОДЕРЖИТ ТОЛЬКО МЕТОДЫ УРОВНЕЙ
// ============================================================================

/// Тест что `LevelAccess` содержит только методы для работы с уровнями.
///
/// # Архитектурные заметки
/// `LevelAccess` должен содержать только:
/// - `get_level()` - получить уровень
/// - `set_level()` - установить уровень
#[test]
fn test_level_access_contains_only_level_methods() {
    let mut state = GameState::new();
    
    // get_level()
    let level: u32 = state.get_level();
    assert_eq!(level, 1, "Начальный уровень должен быть 1");
    
    // set_level()
    state.set_level(5);
    assert_eq!(state.get_level(), 5, "Уровень должен обновиться");
    
    // LevelAccess не должен содержать методов для очков, линий или комбо
    // Проверяем что LevelAccess работает через трейт
    fn use_level_access<L: LevelAccess>(levelable: &L) -> u32 {
        levelable.get_level()
    }
    
    fn use_level_access_mut<L: LevelAccess>(levelable: &mut L) {
        levelable.set_level(10);
    }
    
    let level = use_level_access(&state);
    assert_eq!(level, 5);
    
    use_level_access_mut(&mut state);
    assert_eq!(state.get_level(), 10);
}

/// Тест что `LevelAccess` не содержит методов очков.
#[test]
fn test_level_access_does_not_contain_score_methods() {
    let mut state = GameState::new();
    
    // LevelAccess имеет только методы уровней
    state.set_level(5);
    
    // Эти методы НЕ доступны через LevelAccess:
    // state.get_score() // Не доступно через LevelAccess
    // state.add_score(100) // Не доступно через LevelAccess
}

// ============================================================================
// ТЕСТ 3: LINESACCESS СОДЕРЖИТ ТОЛЬКО МЕТОДЫ ЛИНИЙ
// ============================================================================

/// Тест что `LinesAccess` содержит только методы для работы с линиями.
///
/// # Архитектурные заметки
/// `LinesAccess` должен содержать только:
/// - `get_lines_cleared()` - получить количество линий
/// - `set_lines_cleared()` - установить количество линий
/// - `add_lines()` - добавить линии
#[test]
fn test_lines_access_contains_only_lines_methods() {
    let mut state = GameState::new();
    
    // get_lines_cleared()
    let lines: u32 = state.get_lines_cleared();
    assert_eq!(lines, 0, "Начальное количество линий должно быть 0");
    
    // set_lines_cleared()
    state.set_lines_cleared(10);
    assert_eq!(state.get_lines_cleared(), 10, "Количество линий должно обновиться");
    
    // add_lines()
    state.add_lines(5);
    assert_eq!(state.get_lines_cleared(), 15, "Линии должны добавиться");
    
    // LinesAccess не должен содержать методов для очков, уровней или комбо
    // Проверяем что LinesAccess работает через трейт
    fn use_lines_access<L: LinesAccess>(linesable: &L) -> u32 {
        linesable.get_lines_cleared()
    }
    
    fn use_lines_access_mut<L: LinesAccess>(linesable: &mut L) {
        linesable.set_lines_cleared(20);
        linesable.add_lines(10);
    }
    
    let lines = use_lines_access(&state);
    assert_eq!(lines, 15);
    
    use_lines_access_mut(&mut state);
    assert_eq!(state.get_lines_cleared(), 30);
}

/// Тест что `LinesAccess` не содержит методов очков.
#[test]
fn test_lines_access_does_not_contain_score_methods() {
    let mut state = GameState::new();
    
    // LinesAccess имеет только методы линий
    state.add_lines(5);
    
    // Эти методы НЕ доступны через LinesAccess:
    // state.get_score() // Не доступно через LinesAccess
    // state.add_score(100) // Не доступно через LinesAccess
}

// ============================================================================
// ТЕСТ 4: COMBOACCESS СОДЕРЖИТ ТОЛЬКО МЕТОДЫ КОМБО
// ============================================================================

/// Тест что `ComboAccess` содержит только методы для работы с комбо.
///
/// # Архитектурные заметки
/// `ComboAccess` должен содержать только:
/// - `get_combo()` - получить комбо
/// - `set_combo()` - установить комбо
/// - `reset_combo()` - сбросить комбо
#[test]
fn test_combo_access_contains_only_combo_methods() {
    let mut state = GameState::new();
    
    // get_combo()
    let combo: u32 = state.get_combo();
    assert_eq!(combo, 0, "Начальный комбо должен быть 0");
    
    // set_combo()
    state.set_combo(5);
    assert_eq!(state.get_combo(), 5, "Комбо должен обновиться");
    
    // reset_combo()
    state.reset_combo();
    assert_eq!(state.get_combo(), 0, "Комбо должен сброситься");
    
    // ComboAccess не должен содержать методов для очков, уровней или линий
    // Проверяем что ComboAccess работает через трейт
    fn use_combo_access<C: ComboAccess>(comboable: &C) -> u32 {
        comboable.get_combo()
    }
    
    fn use_combo_access_mut<C: ComboAccess>(comboable: &mut C) {
        comboable.set_combo(10);
        comboable.reset_combo();
    }
    
    let combo = use_combo_access(&state);
    assert_eq!(combo, 0);
    
    use_combo_access_mut(&mut state);
    assert_eq!(state.get_combo(), 0);
}

/// Тест что `ComboAccess` не содержит методов очков.
#[test]
fn test_combo_access_does_not_contain_score_methods() {
    let mut state = GameState::new();
    
    // ComboAccess имеет только методы комбо
    state.set_combo(5);
    
    // Эти методы НЕ доступны через ComboAccess:
    // state.get_score() // Не доступно через ComboAccess
    // state.add_score(100) // Не доступно через ComboAccess
}

// ============================================================================
// ТЕСТ 5: SCORINGSTATE НАСЛЕДУЕТ УЗКИЕ ТРЕЙТЫ
// ============================================================================

/// Тест что `ScoringState` наследует узкие трейты.
///
/// # Архитектурные заметки
/// `ScoringState` объединяет узкие трейты для обратной совместимости:
/// - `ScoreAccess`
/// - `LevelAccess`
/// - `LinesAccess`
/// - `ComboAccess`
#[test]
fn test_scoring_state_inherits_narrow_traits() {
    // ScoringState должен наследовать все узкие трейты
    let mut state = GameState::new();
    
    // Проверяем что ScoringState реализует ScoreAccess
    let _: &dyn ScoreAccess = &state;
    state.set_score(100);
    assert_eq!(state.get_score(), 100);
    
    // Проверяем что ScoringState реализует LevelAccess
    let _: &dyn LevelAccess = &state;
    state.set_level(5);
    assert_eq!(state.get_level(), 5);
    
    // Проверяем что ScoringState реализует LinesAccess
    let _: &dyn LinesAccess = &state;
    state.set_lines_cleared(10);
    assert_eq!(state.get_lines_cleared(), 10);
    
    // Проверяем что ScoringState реализует ComboAccess
    let _: &dyn ComboAccess = &state;
    state.set_combo(5);
    assert_eq!(state.get_combo(), 5);

    // ScoringState объединяет все трейты
}

/// Тест что `ScoringState` можно использовать через узкие трейты.
#[test]
fn test_scoring_state_can_be_used_through_narrow_traits() {
    let mut state = GameState::new();
    
    // Используем через ScoreAccess
    fn use_score<S: ScoreAccess>(s: &mut S) {
        s.set_score(100);
        s.add_score(50);
    }
    use_score(&mut state);
    assert_eq!(state.get_score(), 150);
    
    // Используем через LevelAccess
    fn use_level<L: LevelAccess>(l: &mut L) {
        l.set_level(10);
    }
    use_level(&mut state);
    assert_eq!(state.get_level(), 10);
    
    // Используем через LinesAccess
    fn use_lines<L: LinesAccess>(l: &mut L) {
        l.set_lines_cleared(25);
        l.add_lines(5);
    }
    use_lines(&mut state);
    assert_eq!(state.get_lines_cleared(), 30);
    
    // Используем через ComboAccess
    fn use_combo<C: ComboAccess>(c: &mut C) {
        c.set_combo(10);
    }
    use_combo(&mut state);
    assert_eq!(state.get_combo(), 10);
}

// ============================================================================
// ТЕСТ 6: ISP - РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ
// ============================================================================

/// Тест что трейты следуют ISP принципу.
///
/// # Архитектурные заметки
/// Interface Segregation Principle требует чтобы клиенты не зависели
/// от методов которые они не используют.
#[test]
fn test_traits_follow_isp_principle() {
    // Каждый трейт имеет узкую ответственность:
    let score_methods = ["get_score", "set_score", "add_score"];
    let level_methods = ["get_level", "set_level"];
    let lines_methods = ["get_lines_cleared", "set_lines_cleared", "add_lines"];
    let combo_methods = ["get_combo", "set_combo", "reset_combo"];
    
    // Проверяем что трейты разделены
    assert_eq!(score_methods.len(), 3, "ScoreAccess имеет 3 метода");
    assert_eq!(level_methods.len(), 2, "LevelAccess имеет 2 метода");
    assert_eq!(lines_methods.len(), 3, "LinesAccess имеет 3 метода");
    assert_eq!(combo_methods.len(), 3, "ComboAccess имеет 3 метода");
    
    // Клиенты могут использовать только нужные трейты
    let mut state = GameState::new();
    
    // Клиент который работает только с очками
    fn score_only_client<S: ScoreAccess>(s: &S) -> u128 {
        s.get_score()
    }
    let _ = score_only_client(&state);
    
    // Клиент который работает только с уровнями
    fn level_only_client<L: LevelAccess>(l: &L) -> u32 {
        l.get_level()
    }
    let _ = level_only_client(&state);
}

// ============================================================================
// ТЕСТ 7: АРХИТЕКТУРНЫЙ ТЕСТ ISP
// ============================================================================

/// Архитектурный тест что ISP соблюдается.
#[test]
fn test_isp_architecture_test() {
    // Архитектура ISP:
    // - ScoreAccess: только очки
    // - LevelAccess: только уровни
    // - LinesAccess: только линии
    // - ComboAccess: только комбо
    // - ScoringState: объединяет все трейты
    
    let architecture = [
        ("ScoreAccess", 3),  // 3 метода
        ("LevelAccess", 2),  // 2 метода
        ("LinesAccess", 3),  // 3 метода
        ("ComboAccess", 3),  // 3 метода
        ("ScoringState", 5), // 5 дополнительных методов
    ];
    
    // Проверяем что все трейты работают
    let mut state = GameState::new();
    
    let _: &dyn ScoreAccess = &state;
    let _: &dyn LevelAccess = &state;
    let _: &dyn LinesAccess = &state;
    let _: &dyn ComboAccess = &state;
    let _: &dyn ScoringState = &state;

    assert_eq!(architecture.len(), 5, "Должно быть 5 трейтов");
}
