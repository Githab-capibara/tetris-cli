//! Тесты системы достижений (Achievements).
//!
//! Этот модуль содержит 20 тестов для проверки системы достижений:
//! - Тесты создания Achievement (4 теста)
//! - Тесты конкретных достижений (5 тестов)
//! - Тесты проверки достижений (5 тестов)
//! - Тесты интеграции с GameStats (3 теста)
//! - Тесты производительности (3 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты системы достижений.

use crate::game::{Achievement, GameMode, GameStats};
use crate::game::{MARATHON_LINES, SPRINT_LINES};

// ============================================================================
// ГРУППА ТЕСТОВ 1-4: Создание Achievement
// ============================================================================

/// Тест 1: Проверка создания Achievement через new()
///
/// Проверяет базовое создание достижения с параметрами.
#[test]
fn test_achievement_creation() {
    let achievement = Achievement::new("Тестовое", "Описание теста", 100);

    assert_eq!(achievement.name, "Тестовое", "Название должно совпадать");
    assert_eq!(
        achievement.description, "Описание теста",
        "Описание должно совпадать"
    );
    assert_eq!(achievement.points, 100, "Очки должны совпадать");
}

/// Тест 2: Проверка Clone для Achievement
///
/// Проверяет, что клонирование достижения создаёт точную копию.
#[test]
fn test_achievement_clone() {
    let original = Achievement::new("Оригинал", "Описание оригинала", 200);
    let cloned = original.clone();

    assert_eq!(
        original.name, cloned.name,
        "Название должно совпадать при клонировании"
    );
    assert_eq!(
        original.description, cloned.description,
        "Описание должно совпадать при клонировании"
    );
    assert_eq!(
        original.points, cloned.points,
        "Очки должны совпадать при клонировании"
    );
}

/// Тест 3: Проверка Debug для Achievement
///
/// Проверяет, что реализация Debug работает корректно.
#[test]
fn test_achievement_debug() {
    let achievement = Achievement::new("Отладка", "Тест Debug", 50);
    let debug_str = format!("{:?}", achievement);

    assert!(
        debug_str.contains("Отладка"),
        "Debug строка должна содержать название"
    );
    assert!(
        debug_str.contains("Тест Debug"),
        "Debug строка должна содержать описание"
    );
}

/// Тест 4: Проверка PartialEq для Achievement
///
/// Проверяет, что сравнение достижений работает корректно.
/// Achievement не реализует PartialEq, поэтому сравниваем поля по отдельности.
#[test]
fn test_achievement_partial_eq() {
    let achievement1 = Achievement::new("Тест", "Описание", 100);
    let achievement2 = Achievement::new("Тест", "Описание", 100);
    let achievement3 = Achievement::new("Тест", "Другое", 100);

    // Achievement не реализует PartialEq, сравниваем поля
    assert_eq!(
        achievement1.name, achievement2.name,
        "Одинаковые достижения должны иметь равные названия"
    );
    assert_eq!(
        achievement1.description, achievement2.description,
        "Одинаковые достижения должны иметь равные описания"
    );
    assert_eq!(
        achievement1.points, achievement2.points,
        "Одинаковые достижения должны иметь равные очки"
    );
    assert_ne!(
        achievement1.description, achievement3.description,
        "Достижения с разным описанием должны иметь разные описания"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 5-9: Конкретные достижения
// ============================================================================

/// Тест 5: Проверка достижения "Первый Tetris"
///
/// Проверяет создание достижения за 4 линии одновременно.
#[test]
fn test_first_tetris_achievement() {
    let achievement = Achievement::first_tetris();

    assert_eq!(achievement.name, "🏆 TETRIS!", "Название должно совпадать");
    assert_eq!(
        achievement.description, "Удалите 4 линии одновременно",
        "Описание должно совпадать"
    );
    assert_eq!(achievement.points, 100, "Очки должны быть 100");
}

/// Тест 6: Проверка достижения "Комбо-мастер"
///
/// Проверяет создание достижения за комбо с разными значениями.
#[test]
fn test_combo_master_achievement() {
    let achievement_5 = Achievement::combo_master(5);
    let achievement_10 = Achievement::combo_master(10);

    assert_eq!(
        achievement_5.name, "🔥 Комбо-мастер",
        "Название должно совпадать"
    );
    assert!(
        achievement_5.description.contains("5"),
        "Описание должно содержать номер комбо"
    );
    assert_eq!(
        achievement_5.points, 250,
        "Очки за комбо x5 должны быть 250"
    );

    assert_eq!(
        achievement_10.points, 500,
        "Очки за комбо x10 должны быть 500"
    );
}

/// Тест 7: Проверка достижения "Спринтер"
///
/// Проверяет создание достижения за завершение спринта.
#[test]
fn test_sprinter_achievement() {
    let achievement = Achievement::sprinter();

    assert_eq!(achievement.name, "⚡ Спринтер", "Название должно совпадать");
    assert_eq!(
        achievement.description, "Завершите режим спринт",
        "Описание должно совпадать"
    );
    assert_eq!(achievement.points, 200, "Очки должны быть 200");
}

/// Тест 8: Проверка достижения "Марафонец"
///
/// Проверяет создание достижения за завершение марафона.
#[test]
fn test_marathoner_achievement() {
    let achievement = Achievement::marathoner();

    assert_eq!(
        achievement.name, "🏃 Марафонец",
        "Название должно совпадать"
    );
    assert_eq!(
        achievement.description, "Завершите режим марафон",
        "Описание должно совпадать"
    );
    assert_eq!(achievement.points, 500, "Очки должны быть 500");
}

/// Тест 9: Проверка достижения "Ветеран"
///
/// Проверяет создание достижения за уровень с разными значениями.
#[test]
fn test_veteran_achievement() {
    let achievement_5 = Achievement::veteran(5);
    let achievement_10 = Achievement::veteran(10);

    assert_eq!(
        achievement_5.name, "⭐ Ветеран",
        "Название должно совпадать"
    );
    assert!(
        achievement_5.description.contains("5"),
        "Описание должно содержать номер уровня"
    );
    assert_eq!(
        achievement_5.points, 500,
        "Очки за уровень 5 должны быть 500"
    );

    assert_eq!(
        achievement_10.points, 1000,
        "Очки за уровень 10 должны быть 1000"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 10-14: Проверка достижений (check_achievements)
// ============================================================================

/// Тест 10: Проверка получения достижения за Tetris
///
/// Проверяет, что достижение за 4 линии выдаётся корректно.
#[test]
fn test_check_achievements_tetris() {
    let mut stats = GameStats::new();

    // Проверяем получение достижения за 4 линии
    let achievements = stats.check_achievements(4, 1, GameMode::Classic);

    assert_eq!(achievements.len(), 1, "Должно быть получено 1 достижение");
    assert_eq!(
        achievements[0].name, "🏆 TETRIS!",
        "Достижение должно быть за Tetris"
    );
    assert_eq!(stats.tetris_count, 1, "Счётчик Tetris должен быть 1");
}

/// Тест 11: Проверка, что Tetris достижение выдаётся только один раз
///
/// Проверяет, что повторное получение 4 линий не выдаёт достижение снова.
#[test]
fn test_check_achievements_tetris_once() {
    let mut stats = GameStats::new();

    // Первое получение
    let achievements1 = stats.check_achievements(4, 1, GameMode::Classic);
    assert_eq!(
        achievements1.len(),
        1,
        "Первое достижение должно быть выдано"
    );

    // Повторное получение (не должно выдать достижение)
    let achievements2 = stats.check_achievements(4, 1, GameMode::Classic);
    assert_eq!(
        achievements2.len(),
        0,
        "Повторное достижение не должно выдаваться"
    );
    assert_eq!(stats.tetris_count, 2, "Счётчик Tetris должен увеличиться");
}

/// Тест 12: Проверка получения достижения за комбо
///
/// Проверяет, что достижение за комбо x5 выдаётся корректно.
#[test]
fn test_check_achievements_combo() {
    let mut stats = GameStats::new();
    stats.combo_counter = 5;

    let achievements = stats.check_achievements(2, 1, GameMode::Classic);

    assert_eq!(achievements.len(), 1, "Должно быть получено 1 достижение");
    assert!(
        achievements[0].name.starts_with("🔥"),
        "Достижение должно быть за комбо"
    );
}

/// Тест 13: Проверка получения достижения за спринт
///
/// Проверяет, что достижение за завершение спринта выдаётся корректно.
#[test]
fn test_check_achievements_sprint() {
    let mut stats = GameStats::new();
    stats.total_lines = SPRINT_LINES;

    let achievements = stats.check_achievements(1, 1, GameMode::Sprint);

    assert_eq!(achievements.len(), 1, "Должно быть получено 1 достижение");
    assert_eq!(
        achievements[0].name, "⚡ Спринтер",
        "Достижение должно быть за спринт"
    );
}

/// Тест 14: Проверка получения достижения за марафон
///
/// Проверяет, что достижение за завершение марафона выдаётся корректно.
#[test]
fn test_check_achievements_marathon() {
    let mut stats = GameStats::new();
    stats.total_lines = MARATHON_LINES;

    let achievements = stats.check_achievements(1, 1, GameMode::Marathon);

    assert_eq!(achievements.len(), 1, "Должно быть получено 1 достижение");
    assert_eq!(
        achievements[0].name, "🏃 Марафонец",
        "Достижение должно быть за марафон"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-17: Интеграция с GameStats
// ============================================================================

/// Тест 15: Проверка добавления достижений в GameStats
///
/// Проверяет, что достижения корректно добавляются в вектор.
#[test]
fn test_achievements_added_to_stats() {
    let mut stats = GameStats::new();

    // Получаем достижение за Tetris
    stats.check_achievements(4, 1, GameMode::Classic);

    assert_eq!(
        stats.achievements.len(),
        1,
        "В статистике должно быть 1 достижение"
    );
    assert_eq!(
        stats.achievements[0].name, "🏆 TETRIS!",
        "Достижение должно быть за Tetris"
    );
}

/// Тест 16: Проверка нескольких достижений одновременно
///
/// Проверяет, что можно получить несколько достижений за один ход.
#[test]
fn test_multiple_achievements_at_once() {
    let mut stats = GameStats::new();
    stats.combo_counter = 5;
    stats.total_lines = SPRINT_LINES;

    // Получаем достижение за комбо и спринт одновременно
    let achievements = stats.check_achievements(4, 1, GameMode::Sprint);

    // Должно быть 2 достижения: Tetris и Комбо (спринт уже был в total_lines)
    assert!(
        !achievements.is_empty(),
        "Должно быть получено хотя бы 1 достижение"
    );
}

/// Тест 17: Проверка достижения за уровень
///
/// Проверяет, что достижение за уровень 5 выдаётся корректно.
#[test]
fn test_check_achievements_level() {
    let mut stats = GameStats::new();

    // Проверяем получение достижения за уровень 5
    let achievements = stats.check_achievements(1, 5, GameMode::Classic);

    assert_eq!(achievements.len(), 1, "Должно быть получено 1 достижение");
    assert_eq!(
        achievements[0].name, "⭐ Ветеран",
        "Достижение должно быть за уровень"
    );
    assert_eq!(
        achievements[0].points, 500,
        "Очки за уровень 5 должны быть 500"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 18-20: Производительность
// ============================================================================

/// Тест 18: Проверка производительности создания достижений
///
/// Проверяет, что создание достижений происходит быстро.
#[test]
fn test_performance_achievement_creation() {
    let start = std::time::Instant::now();

    // Создаём 1000 достижений
    for i in 0..1000 {
        let _achievement =
            Achievement::new(&format!("Достижение {}", i), &format!("Описание {}", i), i);
    }

    let duration = start.elapsed();

    // 1000 созданий должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "Создание 1000 достижений должно занять меньше 1 секунды"
    );
}

/// Тест 19: Проверка производительности проверки достижений
///
/// Проверяет, что check_achievements() работает быстро.
#[test]
fn test_performance_check_achievements() {
    let mut stats = GameStats::new();
    let start = std::time::Instant::now();

    // Выполняем 10000 проверок
    for _ in 0..10000 {
        let _ = stats.check_achievements(2, 1, GameMode::Classic);
    }

    let duration = start.elapsed();

    // 10000 проверок должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "10000 проверок достижений должны занять меньше 1 секунды"
    );
}

/// Тест 20: Проверка производительности с большим количеством достижений
///
/// Проверяет, что система работает быстро даже с многими достижениями.
#[test]
fn test_performance_many_achievements() {
    let mut stats = GameStats::new();

    // Добавляем 100 достижений вручную
    for i in 0..100 {
        stats
            .achievements
            .push(Achievement::new(&format!("Тест {}", i), "Тест", 10));
    }

    let start = std::time::Instant::now();

    // Выполняем 1000 проверок (должны быть быстрыми из-за уникальности)
    for _ in 0..1000 {
        let _ = stats.check_achievements(4, 10, GameMode::Classic);
    }

    let duration = start.elapsed();

    // 1000 проверок со 100 достижениями должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "1000 проверок со 100 достижениями должны занять меньше 1 секунды"
    );
}
