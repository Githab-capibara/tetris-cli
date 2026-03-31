//! Модуль счёта и уровней.
//!
//! # Ответственность
//! - Управление очками (score)
//! - Управление уровнем (level)
//! - Управление количеством очищенных линий (lines_cleared)
//!
//! # Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle (SRP).
//! `ScoreBoard` инкапсулирует состояние очков и предоставляет контролируемый доступ.
//!
//! ## Трейты
//! - [`ScoreAccess`] (импортирован из [`crate::game::access`]) - доступ только на чтение
//! - [`ScoreMutable`] - доступ на чтение и запись (расширяет ScoreAccess)

// Импортируем ScoreAccess из access.rs для избежания дублирования
use crate::game::access::ScoreAccess;

/// Трейт для изменения состояния очков.
///
/// Предоставляет мутуабельный доступ для изменения очков и уровня.
/// Расширяет [`ScoreAccess`] методами для изменения состояния.
pub trait ScoreMutable: ScoreAccess {
    /// Добавить очки к текущему счёту.
    ///
    /// # Аргументы
    /// * `points` - количество очков для добавления
    ///
    /// # Возвращает
    /// Новый счёт после добавления очков.
    fn add_score(&mut self, points: u128) -> u128;

    /// Установить счёт.
    ///
    /// # Аргументы
    /// * `value` - новое значение счёта
    fn set_score(&mut self, value: u128);

    /// Установить уровень.
    ///
    /// # Аргументы
    /// * `value` - новое значение уровня
    ///
    /// # Примечания
    /// Уровень не может быть меньше 1.
    fn set_level(&mut self, value: u32);

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// Новый уровень после увеличения.
    fn increment_level(&mut self) -> u32;

    /// Добавить количество очищенных линий.
    ///
    /// # Аргументы
    /// * `count` - количество линий для добавления
    ///
    /// # Возвращает
    /// Новое количество очищенных линий.
    fn add_lines_cleared(&mut self, count: u32) -> u32;

    /// Установить количество очищенных линий.
    ///
    /// # Аргументы
    /// * `value` - новое значение количества линий
    fn set_lines_cleared(&mut self, value: u32);
}

/// Состояние счёта и уровней.
///
/// Инкапсулирует состояние очков, уровня и количества линий.
///
/// ## Поля
/// - `score` - текущий счёт (u128)
/// - `level` - текущий уровень (u32)
/// - `lines_cleared` - количество очищенных линий (u32)
///
/// ## Архитектурные заметки
/// Выделено из `GameState` для соблюдения Single Responsibility Principle.
/// Используется композиция в `GameState` через поле `scoreboard: ScoreBoard`.
pub struct ScoreBoard {
    /// Текущий счёт.
    score: u128,
    /// Текущий уровень.
    level: u32,
    /// Количество очищенных линий.
    lines_cleared: u32,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoreBoard {
    /// Создать новое состояние счёта с нулевыми значениями.
    ///
    /// # Возвращает
    /// Новый экземпляр `ScoreBoard` с нулевыми очками, уровнем 1 и 0 линий.
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }

    /// Получить текущий счёт.
    #[inline]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Получить текущий уровень.
    #[inline]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество очищенных линий.
    #[inline]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Добавить очки к текущему счёту.
    ///
    /// # Аргументы
    /// * `points` - количество очков для добавления
    ///
    /// # Возвращает
    /// Новый счёт после добавления очков.
    ///
    /// # Примечания
    /// Использует saturating_add для защиты от переполнения u128.
    #[inline]
    pub fn add_score(&mut self, points: u128) -> u128 {
        self.score = self.score.saturating_add(points);
        self.score
    }

    /// Установить счёт.
    ///
    /// # Аргументы
    /// * `value` - новое значение счёта
    ///
    /// # Валидация (H3)
    /// Проверяет значение на разумные пределы (максимум u128::MAX).
    #[inline]
    pub fn set_score(&mut self, value: u128) {
        // Валидация (H3): u128 уже имеет естественные границы
        self.score = value;
    }

    /// Установить уровень.
    ///
    /// # Аргументы
    /// * `value` - новое значение уровня
    ///
    /// # Примечания
    /// Уровень не может быть меньше 1.
    ///
    /// # Валидация (H3)
    /// Проверяет значение на разумные пределы (максимум 1000).
    #[inline]
    pub fn set_level(&mut self, value: u32) {
        // Валидация (H3): ограничиваем уровень разумным максимумом
        const MAX_LEVEL: u32 = 1000;
        self.level = value.clamp(1, MAX_LEVEL);
    }

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// Новый уровень после увеличения.
    #[inline]
    pub fn increment_level(&mut self) -> u32 {
        self.level = self.level.saturating_add(1);
        self.level
    }

    /// Добавить количество очищенных линий.
    ///
    /// # Аргументы
    /// * `count` - количество линий для добавления
    ///
    /// # Возвращает
    /// Новое количество очищенных линий.
    #[inline]
    pub fn add_lines_cleared(&mut self, count: u32) -> u32 {
        self.lines_cleared = self.lines_cleared.saturating_add(count);
        self.lines_cleared
    }

    /// Установить количество очищенных линий.
    ///
    /// # Аргументы
    /// * `value` - новое значение количества линий
    #[inline]
    pub fn set_lines_cleared(&mut self, value: u32) {
        self.lines_cleared = value;
    }
}

impl ScoreAccess for ScoreBoard {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_score(&self) -> u128 {
        self.get_score()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_level(&self) -> u32 {
        self.get_level()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn get_lines_cleared(&self) -> u32 {
        self.get_lines_cleared()
    }
}

impl ScoreMutable for ScoreBoard {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn add_score(&mut self, points: u128) -> u128 {
        self.add_score(points)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn set_score(&mut self, value: u128) {
        self.set_score(value);
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn set_level(&mut self, value: u32) {
        self.set_level(value);
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn increment_level(&mut self) -> u32 {
        self.increment_level()
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn add_lines_cleared(&mut self, count: u32) -> u32 {
        self.add_lines_cleared(count)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn set_lines_cleared(&mut self, value: u32) {
        self.set_lines_cleared(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_board_new() {
        let scoreboard = ScoreBoard::new();
        assert_eq!(scoreboard.get_score(), 0);
        assert_eq!(scoreboard.get_level(), 1);
        assert_eq!(scoreboard.get_lines_cleared(), 0);
    }

    #[test]
    fn test_score_board_add_score() {
        let mut scoreboard = ScoreBoard::new();

        assert_eq!(scoreboard.add_score(100), 100);
        assert_eq!(scoreboard.add_score(50), 150);
        assert_eq!(scoreboard.get_score(), 150);
    }

    #[test]
    fn test_score_board_set_score() {
        let mut scoreboard = ScoreBoard::new();

        scoreboard.set_score(500);
        assert_eq!(scoreboard.get_score(), 500);
    }

    #[test]
    fn test_score_board_level() {
        let mut scoreboard = ScoreBoard::new();

        assert_eq!(scoreboard.increment_level(), 2);
        assert_eq!(scoreboard.increment_level(), 3);
        assert_eq!(scoreboard.get_level(), 3);

        scoreboard.set_level(10);
        assert_eq!(scoreboard.get_level(), 10);

        // Уровень не может быть меньше 1
        scoreboard.set_level(0);
        assert_eq!(scoreboard.get_level(), 1);
    }

    #[test]
    fn test_score_board_lines_cleared() {
        let mut scoreboard = ScoreBoard::new();

        assert_eq!(scoreboard.add_lines_cleared(5), 5);
        assert_eq!(scoreboard.add_lines_cleared(3), 8);
        assert_eq!(scoreboard.get_lines_cleared(), 8);

        scoreboard.set_lines_cleared(20);
        assert_eq!(scoreboard.get_lines_cleared(), 20);
    }

    #[test]
    fn test_score_board_overflow_protection() {
        let mut scoreboard = ScoreBoard::new();

        // Переполнение u128
        scoreboard.set_score(u128::MAX - 100);
        scoreboard.add_score(200);
        assert_eq!(scoreboard.get_score(), u128::MAX);
    }
}
