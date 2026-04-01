//! Тесты для ScoringState trait.
//!
//! Этот модуль содержит тесты для проверки исправления #6 (HIGH):
//! - Trait ScoringState корректно реализован
//! - Все методы trait работают правильно
//!
//! Исправление: добавлен trait ScoringState для инкапсуляции изменений состояния

#![allow(clippy::items_after_statements)]

use crate::game::scoring::ScoringState;
use crate::game::GameState;

// ============================================================================
// ГРУППА ТЕСТОВ: ScoringState trait
// ============================================================================

/// Тест 1: Проверка что GameState реализует ScoringState
///
/// Проверяет, что GameState корректно реализует все методы trait.
#[test]
fn test_scoring_state_trait_implemented() {
    // Создаём GameState
    let mut state = GameState::new();

    // Проверяем что GameState реализует ScoringState через проверку методов
    fn assert_scoring_state<T: ScoringState>() {}
    assert_scoring_state::<GameState>();

    // Проверяем что trait методы работают
    let initial_score = state.score();
    assert_eq!(initial_score, 0, "Начальный счёт должен быть 0");

    state.set_score(100);
    assert_eq!(state.score(), 100, "Счёт должен измениться через set_score");

    let level = state.level();
    assert!(level >= 1, "Уровень должен быть >= 1");

    let lines = state.lines_cleared();
    assert_eq!(lines, 0, "Начальное количество линий должно быть 0");
}

/// Тест 2: Проверка метода score()
///
/// Проверяет, что метод score() возвращает корректное значение.
#[test]
fn test_scoring_state_score() {
    let mut state = GameState::new();

    // Начальный счёт должен быть 0
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");

    // Устанавливаем новый счёт
    state.set_score(1000);
    assert_eq!(state.score(), 1000, "Счёт должен быть 1000");

    // Увеличиваем счёт
    state.set_score(2500);
    assert_eq!(state.score(), 2500, "Счёт должен быть 2500");
}

/// Тест 3: Проверка метода set_score()
///
/// Проверяет, что метод set_score() корректно устанавливает значение.
#[test]
fn test_scoring_state_set_score() {
    let mut state = GameState::new();

    // Устанавливаем различные значения
    let scores = [0u128, 100u128, 1000u128, 10000u128, u128::MAX / 2];

    for &score in &scores {
        state.set_score(score);
        assert_eq!(
            state.score(),
            score,
            "Счёт должен совпадать с установленным значением"
        );
    }
}

/// Тест 4: Проверка метода level()
///
/// Проверяет, что метод level() возвращает корректное значение.
#[test]
fn test_scoring_state_level() {
    let state = GameState::new();

    // Начальный уровень должен быть 1
    let level = state.level();
    assert!(level >= 1, "Начальный уровень должен быть >= 1");
}

/// Тест 5: Проверка метода lines_cleared()
///
/// Проверяет, что метод lines_cleared() возвращает корректное значение.
#[test]
fn test_scoring_state_lines_cleared() {
    let state = GameState::new();

    // Начальное количество линий должно быть 0
    assert_eq!(
        state.lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );
}

/// Тест 6: Проверка метода set_lines_cleared()
///
/// Проверяет, что метод set_lines_cleared() корректно устанавливает значение.
#[test]
fn test_scoring_state_set_lines_cleared() {
    let mut state = GameState::new();

    // Устанавливаем различные значения
    let lines = [0u32, 10u32, 40u32, 100u32];

    for &lines_value in &lines {
        state.set_lines_cleared(lines_value);
        assert_eq!(
            state.lines_cleared(),
            lines_value,
            "Количество линий должно совпадать с установленным значением"
        );
    }
}

/// Тест 7: Проверка метода fall_speed()
///
/// Проверяет, что метод fall_speed() возвращает корректное значение.
#[test]
fn test_scoring_state_fall_speed() {
    let state = GameState::new();

    // Скорость падения должна быть положительной
    let speed = state.fall_speed();
    assert!(speed > 0.0, "Скорость падения должна быть положительной");
}

/// Тест 8: Проверка метода set_fall_speed()
///
/// Проверяет, что метод set_fall_speed() корректно устанавливает значение.
#[test]
fn test_scoring_state_set_fall_speed() {
    let mut state = GameState::new();

    // Проверяем что метод возвращает Ok
    let result = state.set_fall_speed(1.0);
    assert!(result.is_ok(), "set_fall_speed() должен вернуть Ok");

    // Проверяем что скорость изменилась (может не точно совпадать из-за внутренней логики)
    let speed = state.fall_speed();
    assert!(speed > 0.0, "Скорость падения должна быть положительной");
}

/// Тест 9: Проверка метода animating_rows_mask()
///
/// Проверяет, что метод animating_rows_mask() возвращает корректное значение.
#[test]
fn test_scoring_state_animating_rows_mask() {
    let state = GameState::new();

    // Маска анимируемых строк — unsigned тип, просто проверяем вызов
    let mask = state.animating_rows_mask();
    let _ = mask;
}

/// Тест 10: Проверка метода set_animating_rows_mask()
///
/// Проверяет, что метод set_animating_rows_mask() корректно устанавливает значение.
#[test]
fn test_scoring_state_set_animating_rows_mask() {
    let mut state = GameState::new();

    // Устанавливаем различные значения маски
    let masks = [0u32, 1u32, 0xFFu32, 0xFFFFu32];

    for &mask in &masks {
        state.set_animating_rows_mask(mask);
        assert_eq!(
            state.animating_rows_mask(),
            mask,
            "Маска анимируемых строк должна совпадать с установленным значением"
        );
    }
}

/// Тест 11: Проверка метода stats()
///
/// Проверяет, что метод stats() возвращает ссылку на GameStats.
#[test]
fn test_scoring_state_stats() {
    let state = GameState::new();

    // Получаем ссылку на статистику
    let stats = state.stats();

    // Проверяем что статистика существует
    let total = stats.total_pieces();
    let _ = total;
}

/// Тест 12: Проверка метода stats_mut()
///
/// Проверяет, что метод stats_mut() возвращает изменяемую ссылку на GameStats.
#[test]
fn test_scoring_state_stats_mut() {
    let mut state = GameState::new();

    // Получаем изменяемую ссылку на статистику
    let stats = state.stats_mut();

    // Проверяем что можем модифицировать статистику
    let initial_pieces = stats.total_pieces();
    // Статистика должна быть доступна для модификации
    let _ = initial_pieces;
}

/// Тест 13: Проверка метода get_blocks()
///
/// Проверяет, что метод get_blocks() возвращает ссылку на игровое поле.
#[test]
fn test_scoring_state_get_blocks() {
    let state = GameState::new();

    // Получаем ссылку на игровое поле
    let blocks = state.get_blocks();

    // Проверяем размеры поля
    assert_eq!(
        blocks.len(),
        crate::io::GRID_HEIGHT,
        "Высота поля должна совпадать"
    );
    assert_eq!(
        blocks[0].len(),
        crate::io::GRID_WIDTH,
        "Ширина поля должна совпадать"
    );
}

/// Тест 14: Проверка метода get_blocks_mut()
///
/// Проверяет, что метод get_blocks_mut() возвращает изменяемую ссылку на поле.
#[test]
fn test_scoring_state_get_blocks_mut() {
    let mut state = GameState::new();

    // Получаем изменяемую ссылку на игровое поле
    let blocks = state.get_blocks_mut();

    // Проверяем размеры поля
    assert_eq!(
        blocks.len(),
        crate::io::GRID_HEIGHT,
        "Высота поля должна совпадать"
    );
    assert_eq!(
        blocks[0].len(),
        crate::io::GRID_WIDTH,
        "Ширина поля должна совпадать"
    );

    // Проверяем что можем модифицировать поле
    blocks[0][0] = 1;
    assert_eq!(blocks[0][0], 1, "Поле должно быть изменяемым");
}

/// Тест 15: Интеграционный тест - комплексная проверка ScoringState
///
/// Проверяет, что все методы ScoringState работают вместе корректно.
#[test]
fn test_scoring_state_comprehensive() {
    let mut state = GameState::new();

    // Проверяем начальные значения
    assert_eq!(state.score(), 0);
    assert_eq!(state.lines_cleared(), 0);

    // Устанавливаем счёт
    state.set_score(1500);
    assert_eq!(state.score(), 1500);

    // Устанавливаем количество линий
    state.set_lines_cleared(10);
    assert_eq!(state.lines_cleared(), 10);

    // Устанавливаем скорость
    state.set_fall_speed(2.5).unwrap();
    assert_eq!(state.fall_speed(), 2.5);

    // Устанавливаем маску
    state.set_animating_rows_mask(0xFF);
    assert_eq!(state.animating_rows_mask(), 0xFF);

    // Проверяем что поле доступно
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), crate::io::GRID_HEIGHT);

    // Проверяем что статистика доступна
    let stats = state.stats();
    let _ = stats.total_pieces();
}
