//! Бизнес-правила игры Tetris CLI.
//!
//! # Ответственность
//! - Централизация бизнес-правил игры
//! - Экспорт констант для систем очков, уровней и линий
//! - Предоставление единого источника истины для правил игры
//!
//! ## Архитектурные заметки
//! Этот модуль создан в рамках Исправления 8 для:
//! - Устранения дублирования констант в scoring/*.rs
//! - Централизации бизнес-правил в одном месте
//! - Упрощения тестирования через мокирование правил
//!
//! ## Использование
//! ```ignore
//! use crate::game::rules::GameRules;
//!
//! let rules = GameRules::default();
//! let line_score = rules.get_line_score(4); // 1800 очков за Tetris
//! ```

// ============================================================================
// GAME RULES STRUCT
// ============================================================================

use crate::constants::{
    COMBO_BONUS, HARD_DROP_POINTS, LEVEL_BONUS_MULT, LINE_SCORES, MAX_LINES_PER_CLEAR,
    SOFT_DROP_POINTS,
};

/// Максимальный уровень.
pub const MAX_LEVEL: u32 = 1000;

/// Структура бизнес-правил игры.
///
/// Предоставляет программный доступ к правилам через методы.
/// Может быть расширена для поддержки различных режимов игры.
///
/// # Примечание
/// В данный момент не используется в production-коде, но оставлена для
/// будущей расширяемости. Все константы берутся напрямую из `crate::constants`.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct GameRules {
    /// Множитель очков.
    pub score_multiplier: u128,
    /// Включён ли режим комбо.
    pub combo_enabled: bool,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            score_multiplier: 1,
            combo_enabled: true,
        }
    }
}

#[allow(dead_code)]
impl GameRules {
    /// Создать новые правила со значениями по умолчанию.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Получить очки за удаление линий.
    ///
    /// # Аргументы
    /// * `lines` - количество удалённых линий (1-4)
    ///
    /// # Возвращает
    /// Количество очков за удаление линий
    ///
    /// # Пример
    /// ```ignore
    /// let rules = GameRules::new();
    /// assert_eq!(rules.get_line_score(4), 1800); // Tetris
    /// ```
    #[must_use]
    pub const fn get_line_score(self, lines: u32) -> u128 {
        if lines == 0 || lines > MAX_LINES_PER_CLEAR {
            return 0;
        }
        LINE_SCORES[(lines - 1) as usize] * self.score_multiplier
    }

    /// Получить бонус за комбо.
    ///
    /// # Аргументы
    /// * `combo_count` - текущий счётчик комбо
    ///
    /// # Возвращает
    /// Бонус за комбо (0 если комбо отключён)
    #[must_use]
    pub fn get_combo_bonus(self, combo_count: u32) -> u128 {
        if !self.combo_enabled || combo_count <= 1 {
            return 0;
        }
        COMBO_BONUS * u128::from(combo_count - 1) * self.score_multiplier
    }

    /// Получить бонус за уровень.
    ///
    /// # Аргументы
    /// * `level` - текущий уровень
    ///
    /// # Возвращает
    /// Бонус за уровень
    #[must_use]
    pub fn get_level_bonus(self, level: u32) -> u128 {
        LEVEL_BONUS_MULT * u128::from(level) * self.score_multiplier
    }

    /// Получить очки за Soft Drop.
    ///
    /// # Аргументы
    /// * `distance` - расстояние падения
    ///
    /// # Возвращает
    /// Очки за Soft Drop
    #[must_use]
    pub fn get_soft_drop_points(self, distance: u32) -> u128 {
        u128::from(distance) * SOFT_DROP_POINTS * self.score_multiplier
    }

    /// Получить очки за Hard Drop.
    ///
    /// # Аргументы
    /// * `distance` - расстояние падения
    ///
    /// # Возвращает
    /// Очки за Hard Drop
    #[must_use]
    pub fn get_hard_drop_points(self, distance: u32) -> u128 {
        u128::from(distance) * HARD_DROP_POINTS * self.score_multiplier
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_rules_default() {
        let rules = GameRules::default();
        assert_eq!(rules.score_multiplier, 1);
        assert!(rules.combo_enabled);
    }

    #[test]
    fn test_game_rules_line_score() {
        let rules = GameRules::new();
        assert_eq!(rules.get_line_score(1), 100);
        assert_eq!(rules.get_line_score(2), 200);
        assert_eq!(rules.get_line_score(3), 400);
        assert_eq!(rules.get_line_score(4), 1800);
        assert_eq!(rules.get_line_score(0), 0);
        assert_eq!(rules.get_line_score(5), 0);
    }

    #[test]
    fn test_game_rules_combo_bonus() {
        let rules = GameRules::new();
        assert_eq!(rules.get_combo_bonus(0), 0);
        assert_eq!(rules.get_combo_bonus(1), 0);
        assert_eq!(rules.get_combo_bonus(2), 50);
        assert_eq!(rules.get_combo_bonus(3), 100);
        assert_eq!(rules.get_combo_bonus(10), 450);
    }

    #[test]
    fn test_game_rules_combo_disabled() {
        let rules = GameRules {
            score_multiplier: 1,
            combo_enabled: false,
        };
        assert_eq!(rules.get_combo_bonus(5), 0);
    }

    #[test]
    fn test_game_rules_level_bonus() {
        let rules = GameRules::new();
        assert_eq!(rules.get_level_bonus(1), 500);
        assert_eq!(rules.get_level_bonus(5), 2500);
        assert_eq!(rules.get_level_bonus(10), 5000);
    }

    #[test]
    fn test_game_rules_soft_drop() {
        let rules = GameRules::new();
        assert_eq!(rules.get_soft_drop_points(0), 0);
        assert_eq!(rules.get_soft_drop_points(1), 1);
        assert_eq!(rules.get_soft_drop_points(10), 10);
        assert_eq!(rules.get_soft_drop_points(100), 100);
    }

    #[test]
    fn test_game_rules_hard_drop() {
        let rules = GameRules::new();
        assert_eq!(rules.get_hard_drop_points(0), 0);
        assert_eq!(rules.get_hard_drop_points(1), 2);
        assert_eq!(rules.get_hard_drop_points(10), 20);
        assert_eq!(rules.get_hard_drop_points(100), 200);
    }

    #[test]
    fn test_game_rules_multiplier() {
        let rules = GameRules {
            score_multiplier: 2,
            combo_enabled: true,
        };
        assert_eq!(rules.get_line_score(4), 3600);
        assert_eq!(rules.get_combo_bonus(5), 400);
        assert_eq!(rules.get_level_bonus(10), 10_000);
    }
}
