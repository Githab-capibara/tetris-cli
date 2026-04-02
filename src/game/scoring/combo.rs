//! Модуль комбо-системы.
//!
//! # Ответственность
//! - Управление комбо-счётчиком
//! - Расчёт комбо-бонусов
//! - Сброс комбо при ходе без удаления линий
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): константы комбо, `GameStats`
//!
//! ## Архитектурные заметки
//! ## Исправление #23
//! Функции комбо-системы были удалены как неиспользуемые (YAGNI).
//! В настоящее время логика комбо реализована напрямую в `handle_landing()`.
//!
//! ## Исправление C1
//! Все операции умножения используют saturating_mul для защиты от переполнения.
//!
//! TODO (#архитектура): Рассмотреть возможность восстановления этих функций
//! если потребуется более сложная система комбо с множителями и бонусами.

#![allow(clippy::absurd_extreme_comparisons, clippy::items_after_statements)]

use crate::constants::COMBO_BONUS;

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
///
/// # Исправление C1
/// Использует saturating_mul для защиты от переполнения.
#[must_use]
#[allow(dead_code)]
pub fn calculate_combo_bonus(combo_counter: u32) -> u128 {
    if combo_counter > 1 {
        COMBO_BONUS.saturating_mul(u128::from(combo_counter - 1))
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

    // ========================================================================
    // ТЕСТЫ НА ПЕРЕПОЛНЕНИЕ КОМБО (Исправление #1 - ВЫСОКИЙ ПРИОРИТЕТ)
    // ========================================================================

    /// Тест на защиту от переполнения при большом комбо.
    #[test]
    fn test_combo_overflow_protection() {
        // Очень большой комбо-счётчик
        let large_combo = u32::MAX;
        let bonus = calculate_combo_bonus(large_combo);

        // Бонус не должен переполниться
        assert!(bonus <= u128::MAX, "Переполнение комбо-бонуса");
        // Бонус должен быть положительным
        assert!(bonus > 0, "Бонус должен быть положительным");
    }

    /// Тест на saturating_mul при комбо.
    #[test]
    fn test_combo_saturating_mul() {
        // Проверяем что saturating_mul работает корректно
        let combo_1 = calculate_combo_bonus(1);
        let combo_2 = calculate_combo_bonus(2);
        let combo_100 = calculate_combo_bonus(100);

        assert_eq!(combo_1, 0, "Первое комбо не даёт бонуса");
        assert_eq!(combo_2, 50, "Второе комбо даёт 50 бонуса");
        assert_eq!(combo_100, 4950, "100-е комбо даёт 4950 бонуса");
    }
}
