//! Модуль комбо-системы.
//!
//! # Ответственность
//! - Управление комбо-счётчиком
//! - Расчёт комбо-бонусов
//! - Сброс комбо при ходе без удаления линий
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): константы комбо, `GameStats`

use crate::game::state::COMBO_BONUS;

// ============================================================================
// YAGNI КОД - ПОМЕЧЕНО ДЛЯ БУДУЩЕГО ИСПОЛЬЗОВАНИЯ
// ============================================================================
// Эти функции могут быть использованы для расширения комбо-системы в будущем.
// В настоящее время не используются в основном коде.

/// Рассчитать бонус за комбо.
///
/// # Аргументы
/// * `combo_counter` - текущий счётчик комбо
///
/// # Возвращает
/// Бонусные очки за комбо
///
/// # Формула
/// Бонус = COMBO_BONUS × (combo_counter - 1)
/// - 1-е комбо: 0 бонусных очков
/// - 2-е комбо: 50 бонусных очков
/// - 3-е комбо: 100 бонусных очков
/// - N-е комбо: 50 × (N - 1) бонусных очков
#[must_use]
#[allow(dead_code)] // Может быть использовано для расширения комбо-системы
pub fn calculate_combo_bonus(combo_counter: u32) -> u128 {
    if combo_counter > 1 {
        COMBO_BONUS.saturating_mul(u128::from(combo_counter - 1))
    } else {
        0
    }
}

/// Обновить комбо-счётчик после удаления линий.
///
/// # Аргументы
/// * `combo_counter` - текущий счётчик комбо (изменяемый)
/// * `lines_cleared` - количество удалённых линий
///
/// # Возвращает
/// Новый счётчик комбо
#[must_use]
#[allow(dead_code)] // Может быть использовано для расширения комбо-системы
pub fn update_combo_counter(combo_counter: u32, lines_cleared: u32) -> u32 {
    if lines_cleared > 0 {
        combo_counter.saturating_add(1)
    } else {
        0
    }
}

/// Сбросить комбо-счётчик.
///
/// Используется когда игрок сделал ход без удаления линий.
///
/// # Аргументы
/// * `combo_counter` - текущий счётчик комбо (изменяемый)
#[allow(dead_code)] // Может быть использовано для расширения комбо-системы
pub fn reset_combo_counter(combo_counter: &mut u32) {
    *combo_counter = 0;
}

/// Проверить, является ли комбо "Tetris" (4 линии).
///
/// # Аргументы
/// * `lines_cleared` - количество удалённых линий
///
/// # Возвращает
/// `true` если было удалено 4 линии
#[must_use]
#[allow(dead_code)] // Может быть использовано для расширения комбо-системы
pub fn is_tetris(lines_cleared: u32) -> bool {
    lines_cleared == 4
}

/// Рассчитать бонус за Tetris (4 линии).
///
/// # Аргументы
/// * `lines_cleared` - количество удалённых линий
/// * `combo_counter` - текущий счётчик комбо
///
/// # Возвращает
/// Общий бонус за Tetris (базовый + комбо)
#[must_use]
#[allow(dead_code)] // Может быть использовано для расширения комбо-системы
pub fn calculate_tetris_bonus(lines_cleared: u32, combo_counter: u32) -> u128 {
    const TETRIS_BONUS: u128 = 1000;

    if is_tetris(lines_cleared) {
        TETRIS_BONUS.saturating_add(calculate_combo_bonus(combo_counter))
    } else {
        0
    }
}

#[cfg(test)]
mod combo_tests {
    use super::*;

    #[test]
    fn test_calculate_combo_bonus_zero() {
        assert_eq!(calculate_combo_bonus(0), 0);
    }

    #[test]
    fn test_calculate_combo_bonus_first() {
        assert_eq!(calculate_combo_bonus(1), 0);
    }

    #[test]
    fn test_calculate_combo_bonus_second() {
        assert_eq!(calculate_combo_bonus(2), 50);
    }

    #[test]
    fn test_calculate_combo_bonus_fifth() {
        assert_eq!(calculate_combo_bonus(5), 200);
    }

    #[test]
    fn test_update_combo_counter_increment() {
        assert_eq!(update_combo_counter(0, 1), 1);
        assert_eq!(update_combo_counter(1, 1), 2);
        assert_eq!(update_combo_counter(2, 1), 3);
    }

    #[test]
    fn test_update_combo_counter_reset() {
        assert_eq!(update_combo_counter(3, 0), 0);
        assert_eq!(update_combo_counter(5, 0), 0);
    }

    #[test]
    fn test_is_tetris() {
        assert!(is_tetris(4));
        assert!(!is_tetris(1));
        assert!(!is_tetris(2));
        assert!(!is_tetris(3));
    }
}
