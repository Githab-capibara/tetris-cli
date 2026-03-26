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
//! TODO (#архитектура): Рассмотреть возможность восстановления этих функций
//! если потребуется более сложная система комбо с множителями и бонусами.

use crate::game::state::COMBO_BONUS;

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
}
