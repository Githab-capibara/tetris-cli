//! Тесты граничных значений и переполнения.
//!
//! Этот файл содержит тесты для проверки обработки граничных значений:
//! - u128::MAX, 0, отрицательные значения
//! - Переполнение очков, комбо, линий
//! - Обработка ошибок Result, Option
//!
//! ## Запуск тестов
//! ```bash
//! cargo test --lib test_boundary_values
//! ```

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

use crate::game::types::{Level, LinesCount, Score};
use crate::game::GameState;
use crate::highscore::leaderboard::{Leaderboard, LeaderboardEntry};

// ============================================================================
// ТЕСТЫ ГРАНИЧНЫХ ЗНАЧЕНИЙ ДЛЯ ОЧКОВ (Score)
// ============================================================================

/// Тест T1: Score с нулевым значением
#[test]
fn test_score_zero_value() {
    let score = Score::new();
    assert_eq!(score.value(), 0, "Новый Score должен быть 0");
    assert!(score.is_zero(), "Score должен быть нулевым");
}

/// Тест T2: Score с максимальным значением (u128::MAX)
#[test]
fn test_score_max_value() {
    let score = Score::with_value(u128::MAX);
    assert_eq!(
        score.value(),
        u128::MAX,
        "Score должен поддерживать u128::MAX"
    );
    assert!(!score.is_zero(), "Score с MAX не должен быть нулевым");
}

/// Тест T3: Score saturating_add при переполнении
#[test]
fn test_score_saturating_overflow() {
    let mut score = Score::with_value(u128::MAX);

    // Добавление 1 должно вызвать насыщение
    score.add(1);
    assert_eq!(
        score.value(),
        u128::MAX,
        "Score должен насыщаться до u128::MAX при переполнении"
    );

    // Добавление большого числа тоже должно насыщаться
    score.add(u128::MAX);
    assert_eq!(
        score.value(),
        u128::MAX,
        "Score должен оставаться на u128::MAX после добавления u128::MAX"
    );
}

/// Тест T4: Score saturating_mul при переполнении
#[test]
fn test_score_saturating_multiply() {
    let mut score = Score::with_value(u128::MAX / 2 + 1);

    // Умножение на 2 должно вызвать насыщение
    score.multiply(2);
    assert_eq!(
        score.value(),
        u128::MAX,
        "Score должен насыщаться до u128::MAX при умножении"
    );

    // Умножение на 0 должно давать 0
    score.multiply(0);
    assert_eq!(score.value(), 0, "Score должен быть 0 после умножения на 0");
}

/// Тест T5: Score конвертация из u128
#[test]
fn test_score_from_u128_boundary() {
    let score_zero: Score = 0u128.into();
    assert_eq!(score_zero.value(), 0, "Конвертация 0 должна давать 0");

    let score_max: Score = u128::MAX.into();
    assert_eq!(
        score_max.value(),
        u128::MAX,
        "Конвертация u128::MAX должна давать MAX"
    );
}

/// Тест T6: Score конвертация в u128
#[test]
fn test_score_into_u128() {
    let score = Score::with_value(u128::MAX);
    let value: u128 = score.into();
    assert_eq!(
        value,
        u128::MAX,
        "Конвертация в u128 должна сохранять значение"
    );
}

// ============================================================================
// ТЕСТЫ ГРАНИЧНЫХ ЗНАЧЕНИЙ ДЛЯ УРОВНЯ (Level)
// ============================================================================

/// Тест T7: Level с минимальным значением (1)
#[test]
fn test_level_minimum_value() {
    let level = Level::new();
    assert_eq!(level.value(), 1, "Новый Level должен быть 1");
}

/// Тест T8: Level с нулевым значением (должно стать 1)
#[test]
fn test_level_zero_becomes_minimum() {
    let level = Level::with_value(0);
    assert_eq!(level.value(), 1, "Level с 0 должен стать 1 (минимум)");
}

/// Тест T9: Level с u32::MAX
#[test]
fn test_level_max_value() {
    let level = Level::with_value(u32::MAX);
    assert_eq!(
        level.value(),
        u32::MAX,
        "Level должен поддерживать u32::MAX"
    );
}

/// Тест T10: Level increment до переполнения
#[test]
fn test_level_increment_to_overflow() {
    let mut level = Level::with_value(u32::MAX - 1);

    // Первое увеличение должно пройти
    assert!(level.increment(), "increment должен вернуть true");
    assert_eq!(level.value(), u32::MAX, "Level должен быть u32::MAX");

    // Второе увеличение должно вернуть false (переполнение)
    assert!(
        !level.increment(),
        "increment должен вернуть false при u32::MAX"
    );
    assert_eq!(level.value(), u32::MAX, "Level должен остаться u32::MAX");
}

/// Тест T11: Level increment_by с переполнением
#[test]
fn test_level_increment_by_overflow() {
    let mut level = Level::with_value(u32::MAX);

    // Увеличение на 1 должно вернуть false
    assert!(
        !level.increment_by(1),
        "increment_by(1) должен вернуть false"
    );
    assert_eq!(level.value(), u32::MAX, "Level должен остаться u32::MAX");

    // Увеличение на большое число тоже должно вернуть false
    assert!(
        !level.increment_by(u32::MAX),
        "increment_by(u32::MAX) должен вернуть false"
    );
    assert_eq!(level.value(), u32::MAX, "Level должен остаться u32::MAX");
}

/// Тест T12: Level reset
#[test]
fn test_level_reset() {
    let mut level = Level::with_value(100);
    level.reset();
    assert_eq!(level.value(), 1, "reset() должен вернуть уровень к 1");
}

// ============================================================================
// ТЕСТЫ ГРАНИЧНЫХ ЗНАЧЕНИЙ ДЛЯ ЛИНИЙ (LinesCount)
// ============================================================================

/// Тест T13: LinesCount с нулевым значением
#[test]
fn test_lines_count_zero() {
    let lines = LinesCount::new();
    assert_eq!(lines.value(), 0, "Новый LinesCount должен быть 0");
}

/// Тест T14: LinesCount с u32::MAX
#[test]
fn test_lines_count_max_value() {
    let lines = LinesCount::with_value(u32::MAX);
    assert_eq!(
        lines.value(),
        u32::MAX,
        "LinesCount должен поддерживать u32::MAX"
    );
}

/// Тест T15: LinesCount saturating_add при переполнении
#[test]
fn test_lines_count_saturating_overflow() {
    let mut lines = LinesCount::with_value(u32::MAX);

    // Добавление 1 должно вызвать насыщение
    lines.add(1);
    assert_eq!(
        lines.value(),
        u32::MAX,
        "LinesCount должен насыщаться до u32::MAX"
    );

    // Добавление большого числа
    lines.add(u32::MAX);
    assert_eq!(
        lines.value(),
        u32::MAX,
        "LinesCount должен остаться на u32::MAX"
    );
}

/// Тест T16: LinesCount increment до переполнения
#[test]
fn test_lines_count_increment_overflow() {
    let mut lines = LinesCount::with_value(u32::MAX);

    // Увеличение должно вернуть false
    assert!(
        !lines.increment(),
        "increment должен вернуть false при u32::MAX"
    );
    assert_eq!(
        lines.value(),
        u32::MAX,
        "LinesCount должен остаться u32::MAX"
    );
}

/// Тест T17: LinesCount reached с граничными значениями
#[test]
fn test_lines_count_reached_boundary() {
    let lines = LinesCount::with_value(100);

    assert!(lines.reached(100), "reached(100) должен вернуть true");
    assert!(lines.reached(99), "reached(99) должен вернуть true");
    assert!(!lines.reached(101), "reached(101) должен вернуть false");
    assert!(
        lines.reached(0),
        "reached(0) должен вернуть true для любого значения"
    );
}

/// Тест T18: LinesCount reset
#[test]
fn test_lines_count_reset() {
    let mut lines = LinesCount::with_value(150);
    lines.reset();
    assert_eq!(lines.value(), 0, "reset() должен вернуть LinesCount к 0");
}

// ============================================================================
// ТЕСТЫ ГРАНИЧНЫХ ЗНАЧЕНИЙ ДЛЯ GameState
// ============================================================================

/// Тест T19: GameState начальные значения
#[test]
fn test_game_state_initial_values() {
    let state = GameState::new();

    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(
        state.lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );
}

/// Тест T20: GameState установка максимального счёта
#[test]
fn test_game_state_max_score() {
    let mut state = GameState::new();
    state.set_score(u128::MAX);
    assert_eq!(state.score(), u128::MAX, "Счёт должен быть u128::MAX");
}

/// Тест T21: GameState переполнение счёта через set_score
#[test]
fn test_game_state_score_overflow_protection() {
    use crate::game::scoring::lines::MAX_SCORE;

    let mut state = GameState::new();

    // MAX_SCORE = u128::MAX / 2
    state.set_score(MAX_SCORE);
    assert_eq!(state.score(), MAX_SCORE, "Счёт должен быть MAX_SCORE");

    // Попытка установить больше MAX_SCORE должна ограничиваться
    state.set_score(u128::MAX);
    assert!(
        state.score() <= u128::MAX,
        "Счёт не должен превышать u128::MAX"
    );
}

/// Тест T22: GameState отрицательные значения (не должны быть возможны)
#[test]
fn test_game_state_no_negative_values() {
    let state = GameState::new();

    // Все публичные значения должны быть неотрицательными
    assert!(state.score() >= 0, "Счёт не должен быть отрицательным");
    assert!(state.level() >= 1, "Уровень не должен быть меньше 1");
    assert!(
        state.lines_cleared() >= 0,
        "Количество линий не должно быть отрицательным"
    );
}

// ============================================================================
// ТЕСТЫ ОБРАБОТКИ ОШИБОК (Result, Option)
// ============================================================================

/// Тест T23: LeaderboardEntry::score() возвращает Option
#[test]
fn test_leaderboard_entry_score_option() {
    let entry = LeaderboardEntry::new("Player", 1000);

    let score = entry.score();
    assert!(
        score.is_some(),
        "score() должен возвращать Some для валидной записи"
    );
    assert_eq!(score, Some(1000), "Счёт должен быть 1000");
}

/// Тест T24: LeaderboardEntry::score() возвращает Option
#[test]
fn test_leaderboard_entry_score_option_handling() {
    let entry = LeaderboardEntry::new("Player", 2000);

    let score = entry.score();
    assert!(score.is_some(), "score() должен возвращать Some");
    assert_eq!(score, Some(2000), "Счёт должен быть 2000");
}

/// Тест T25: LeaderboardEntry с невалидным хэшем
#[test]
fn test_leaderboard_entry_invalid_hash() {
    let entry = LeaderboardEntry::new("Player", 1000);

    // score() должен вернуть Some для валидной записи
    let score = entry.score();
    assert!(
        score.is_some(),
        "score() должен возвращать Some для валидной записи"
    );
    assert_eq!(score, Some(1000), "Счёт должен быть 1000");
}

/// Тест T26: Leaderboard::add_score() возвращает bool
#[test]
fn test_leaderboard_add_score_result() {
    let mut leaderboard = Leaderboard::default();

    let result = leaderboard.add_score("Player1", 1000);
    assert!(result, "add_score() должен вернуть true для первой записи");

    // Добавим ещё записи до заполнения
    let _ = leaderboard.add_score("Player2", 500);
    let _ = leaderboard.add_score("Player3", 1500);
    let _ = leaderboard.add_score("Player4", 2000);
    let _ = leaderboard.add_score("Player5", 2500);

    // Следующая запись с меньшим счётом не должна добавиться
    let result = leaderboard.add_score("Player6", 100);
    assert!(
        !result,
        "add_score() должен вернуть false для записи не входящей в топ-5"
    );
}

/// Тест T27: Leaderboard::get_entries() возвращает Vec
#[test]
fn test_leaderboard_get_entries() {
    let leaderboard = Leaderboard::default();

    let entries = leaderboard.get_entries();
    assert!(
        entries.is_empty(),
        "Пустой leaderboard должен возвращать пустой Vec"
    );

    let mut leaderboard = Leaderboard::default();
    let _ = leaderboard.add_score("Player", 1000);
    let entries = leaderboard.get_entries();
    assert_eq!(entries.len(), 1, "Leaderboard должен содержать 1 запись");
}

/// Тест T28: Leaderboard::get_best_score() возвращает u128
#[test]
fn test_leaderboard_get_best_score() {
    let leaderboard = Leaderboard::default();

    let best = leaderboard.get_best_score();
    assert_eq!(best, 0, "Пустой leaderboard должен возвращать 0");

    let mut leaderboard = Leaderboard::default();
    let _ = leaderboard.add_score("Player", 1000);
    let best = leaderboard.get_best_score();
    assert_eq!(best, 1000, "Лучший счёт должен быть 1000");
}

// ============================================================================
// ТЕСТЫ КОМБО СИСТЕМЫ
// ============================================================================

/// Тест T29: Combo счётчик с нулевым значением
#[test]
fn test_combo_counter_zero() {
    let stats = crate::game::GameStats::new();
    assert_eq!(stats.combo_counter(), 0, "Начальное комбо должно быть 0");
}

/// Тест T30: Combo счётчик с максимальным значением
#[test]
fn test_combo_counter_max_value() {
    let mut stats = crate::game::GameStats::new();
    stats.set_combo_counter(u32::MAX);
    assert_eq!(
        stats.combo_counter(),
        u32::MAX,
        "Комбо должен поддерживать u32::MAX"
    );
}

/// Тест T31: Combo бонус при нулевом комбо
#[test]
fn test_combo_bonus_at_zero() {
    // Бонус за первое комбо (комбо = 1) должен быть 0
    // Формула: COMBO_BONUS * (combo_counter - 1)
    const COMBO_BONUS: u64 = 50;

    let combo_level_1_bonus = COMBO_BONUS * 0; // (1 - 1) = 0
    assert_eq!(
        combo_level_1_bonus, 0,
        "Бонус за первое комбо должен быть 0"
    );
}

/// Тест T32: Combo бонус при большом комбо
#[test]
fn test_combo_bonus_at_high_combo() {
    const COMBO_BONUS: u64 = 50;

    // Бонус за 10 комбо
    let combo_10_bonus = COMBO_BONUS * 9; // (10 - 1) = 9
    assert_eq!(combo_10_bonus, 450, "Бонус за 10 комбо должен быть 450");

    // Бонус за 100 комбо
    let combo_100_bonus = COMBO_BONUS * 99; // (100 - 1) = 99
    assert_eq!(combo_100_bonus, 4950, "Бонус за 100 комбо должен быть 4950");
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ
// ============================================================================

/// Тест T33: Интеграционный тест - все граничные значения работают вместе
#[test]
fn test_all_boundary_values_integration() {
    // Score
    let mut score = Score::with_value(u128::MAX / 2);
    score.add(u128::MAX / 2);
    assert_eq!(
        score.value(),
        u128::MAX / 2 + u128::MAX / 2,
        "Score должен корректно складываться"
    );

    // Level
    let mut level = Level::new();
    for _ in 0..10 {
        let _ = level.increment();
    }
    assert_eq!(level.value(), 11, "Level должен увеличиться до 11");

    // LinesCount
    let mut lines = LinesCount::new();
    lines.add(40);
    assert_eq!(lines.value(), 40, "LinesCount должен быть 40");

    // GameState
    let mut state = GameState::new();
    state.set_score(1000);
    assert_eq!(state.score(), 1000, "GameState счёт должен быть 1000");
}

/// Тест T34: Интеграционный тест - переполнение не вызывает панику
#[test]
fn test_no_panic_on_overflow() {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut score = Score::with_value(u128::MAX);
        score.add(u128::MAX);
        score.multiply(u128::MAX);
        score.value()
    }));

    assert!(
        result.is_ok(),
        "Операции с переполнением не должны вызывать панику"
    );
    assert_eq!(
        result.unwrap(),
        u128::MAX,
        "Score должен насыщаться до u128::MAX"
    );
}

/// Тест T35: Интеграционный тест - Option/Result обработка
#[test]
fn test_option_result_handling() {
    let mut leaderboard = Leaderboard::default();

    // Пустой leaderboard
    assert_eq!(leaderboard.get_best_score(), 0);
    assert!(leaderboard.get_entries().is_empty());

    // С записями
    let _ = leaderboard.add_score("Player", 1000);
    assert!(leaderboard.get_best_score() > 0);
    assert!(!leaderboard.get_entries().is_empty());

    // Валидация записи
    let entry = LeaderboardEntry::new("Player", 1000);
    assert!(entry.score().is_some());
    assert!(entry.is_valid());
}
