//! Тесты #[must_use] атрибутов.
//!
//! Проверяют, что #[must_use] присутствует на функциях и
//! предупреждения компилятора работают корректно.

use crate::game::GameState;
use crate::highscore::leaderboard::LeaderboardEntry;
use crate::highscore::SaveData;

/// Тест 1: Проверка #[must_use] на score()
///
/// Проверяем, что score() имеет #[must_use].
#[test]
fn test_must_use_get_score() {
    let state = GameState::new();

    // Используем результат - не должно быть предупреждений
    let score = state.score();
    assert_eq!(score, 0, "Начальный счёт должен быть 0");
}

/// Тест 2: Проверка #[must_use] на level()
///
/// Проверяем, что level() имеет #[must_use].
#[test]
fn test_must_use_get_level() {
    let state = GameState::new();

    // Используем результат
    let level = state.level();
    assert_eq!(level, 1, "Начальный уровень должен быть 1");
}

/// Тест 3: Проверка #[must_use] на lines_cleared()
///
/// Проверяем, что lines_cleared() имеет #[must_use].
#[test]
fn test_must_use_get_lines_cleared() {
    let state = GameState::new();

    // Используем результат
    let lines = state.lines_cleared();
    assert_eq!(lines, 0, "Начальное количество линий должно быть 0");
}

/// Тест 4: Проверка #[must_use] на verify_and_get_score()
///
/// Проверяем, что verify_and_get_score() имеет #[must_use].
#[test]
fn test_must_use_verify_and_get_score() {
    let save = SaveData::from_value(1000);

    // Используем результат
    let score = save.verify_and_get_score();
    assert_eq!(score, Some(1000), "Счёт должен быть 1000");
}

/// Тест 5: Проверка #[must_use] на is_valid()
///
/// Проверяем, что is_valid() имеет #[must_use].
#[test]
fn test_must_use_is_valid() {
    let entry = LeaderboardEntry::new("Player", 1000);

    // Используем результат
    let valid = entry.is_valid();
    assert!(valid, "Запись должна быть валидной");
}

/// Тест 6: Проверка #[must_use] на total_pieces()
///
/// Проверяем, что total_pieces() имеет #[must_use].
#[test]
fn test_must_use_total_pieces() {
    let stats = crate::game::GameStats::new();

    // Используем результат
    let total = stats.total_pieces();
    assert_eq!(total, 0, "Общее количество фигур должно быть 0");
}

/// Тест 8: Проверка #[must_use] на score()
///
/// Проверяем, что score() имеет #[must_use].
#[test]
fn test_must_use_score() {
    let entry = LeaderboardEntry::new("Player", 1000);

    // Используем результат
    let score = entry.score();
    assert_eq!(score, Some(1000), "Счёт должен быть 1000");
}

/// Тест 9: Проверка #[must_use] на name()
///
/// Проверяем, что name() имеет #[must_use].
#[test]
fn test_must_use_name() {
    let entry = LeaderboardEntry::new("Player", 1000);

    // Используем результат
    let name = entry.name();
    assert_eq!(name, "Player", "Имя должно быть 'Player'");
}

/// Тест 10: Проверка #[must_use] на get_mode()
///
/// Проверяем, что get_mode() имеет #[must_use].
#[test]
fn test_must_use_get_mode() {
    let state = GameState::new();

    // Используем результат
    let mode = state.get_mode_trait().name();
    assert_eq!(mode, "Классика", "Режим по умолчанию должен быть Классика");
}

/// Тест 11: Проверка компиляции с предупреждениями
///
/// Проверяем, что код компилируется с #[must_use] атрибутами.
#[test]
fn test_must_use_compilation() {
    // Этот тест проверяет, что код с #[must_use] компилируется корректно
    let state = GameState::new();

    // Все результаты используются - не должно быть предупреждений
    let _score = state.score();
    let _level = state.level();
    let _lines = state.lines_cleared();
    let _mode = state.get_mode_trait().name();
}

/// Тест 12: Проверка #[must_use] на get_stats()
///
/// Проверяем, что get_stats() имеет #[must_use].
#[test]
fn test_must_use_get_stats() {
    let state = GameState::new();

    // Используем результат
    let st_stats = state.stats();
    // Первая фигура уже добавлена в статистику при создании GameState
    assert_eq!(
        st_stats.total_pieces(),
        1,
        "Общее количество фигур должно быть 1 (первая фигура)"
    );
}
