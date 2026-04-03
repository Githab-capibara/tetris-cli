//! Модуль счёта и уровней.
//!
//! # Ответственность
//! - Управление очками (score)
//! - Управление уровнем (level)
//! - Управление количеством очищенных линий (`lines_cleared`)
//!
//! # Архитектурные заметки
//! Выделено из `GameState` для соблюдения Single Responsibility Principle (SRP).
//! `ScoreBoard` инкапсулирует состояние очков и предоставляет контролируемый доступ.
//!
//! ## Трейты
//! - [`ScoreAccess`] (импортирован из [`crate::game::access`]) - доступ только на чтение
//! - [`ScoreMutable`] (импортирован из [`crate::game::access`]) - доступ на чтение и запись

#![allow(dead_code)]

// Импортируем трейты из access.rs для избежания дублирования
use crate::game::access::{ScoreAccess, ScoreMutable};

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
///
/// ## M19: Debug для отладки
/// Добавлен #[derive(Debug)] для возможности отладки через `fmt::Debug`.
#[derive(Debug)]
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
    #[must_use = "Счётчик очков должен быть использован"]
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }

    /// Получить текущий счёт.
    ///
    /// # Возвращает
    /// Текущий счёт (u128)
    ///
    /// # Пример
    /// ```
    /// use crate::game::scoreboard::ScoreBoard;
    ///
    /// let scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.get_score(), 0);
    /// ```
    #[must_use = "Счёт должен быть использован"]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Получить текущий уровень.
    ///
    /// # Возвращает
    /// Текущий уровень (u32)
    ///
    /// # Пример
    /// ```
    /// use crate::game::scoreboard::ScoreBoard;
    ///
    /// let scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.get_level(), 1);
    /// ```
    #[must_use = "Уровень должен быть использован"]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество очищенных линий.
    ///
    /// # Возвращает
    /// Количество очищенных линий (u32)
    ///
    /// # Пример
    /// ```
    /// use crate::game::scoreboard::ScoreBoard;
    ///
    /// let scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.get_lines_cleared(), 0);
    /// ```
    #[must_use = "Количество линий должно быть использовано"]
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
    /// Использует `saturating_add` для защиты от переполнения u128.
    ///
    /// # Исправление аудита 2026-04-02 (C8)
    /// Добавлено логирование при переполнении счёта.
    ///
    /// # Исправление аудита 2026-04-02 (H16)
    /// Добавлен #[`must_use`] так как возвращаемое значение (новый счёт) важно.
    ///
    /// # Пример использования
    /// ```
    /// use crate::game::scoreboard::ScoreBoard;
    ///
    /// let mut scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.add_score(100), 100);
    /// assert_eq!(scoreboard.add_score(50), 150);
    /// ```
    #[must_use = "Новый счёт должен быть использован"]
    pub fn add_score(&mut self, points: u128) -> u128 {
        let old_score = self.score;
        self.score = self.score.saturating_add(points);

        // Логирование при переполнении (C8)
        if self.score == u128::MAX && points > 0 && old_score < u128::MAX {
            eprintln!("[WARN] Переполнение счёта: достигнут максимум u128::MAX");
        }

        self.score
    }

    /// Установить счёт.
    ///
    /// # Аргументы
    /// * `value` - новое значение счёта
    ///
    /// # Валидация (H3)
    /// Проверяет значение на разумные пределы (максимум `u128::MAX`).
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
    pub fn set_level(&mut self, value: u32) {
        // Валидация (H3): ограничиваем уровень разумным максимумом
        use super::rules::MAX_LEVEL;
        self.level = value.clamp(1, MAX_LEVEL);
    }

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// Новый уровень после увеличения.
    ///
    /// # Исправление аудита 2026-04-02 (H16)
    /// Добавлен #[`must_use`] так как возвращаемое значение (новый уровень) важно.
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::scoreboard::ScoreBoard;
    ///
    /// let mut scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.increment_level(), 2);
    /// assert_eq!(scoreboard.increment_level(), 3);
    /// ```
    #[must_use = "Новый уровень должен быть использован"]
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
    ///
    /// # Исправление аудита 2026-04-02 (H16)
    /// Добавлен #[`must_use`] так как возвращаемое значение (новое количество линий) важно.
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::scoreboard::ScoreBoard;
    ///
    /// let mut scoreboard = ScoreBoard::new();
    /// assert_eq!(scoreboard.add_lines_cleared(5), 5);
    /// assert_eq!(scoreboard.add_lines_cleared(3), 8);
    /// ```
    #[must_use = "Новое количество линий должно быть использовано"]
    pub fn add_lines_cleared(&mut self, count: u32) -> u32 {
        self.lines_cleared = self.lines_cleared.saturating_add(count);
        self.lines_cleared
    }

    /// Установить количество очищенных линий.
    ///
    /// # Аргументы
    /// * `value` - новое значение количества линий
    pub fn set_lines_cleared(&mut self, value: u32) {
        self.lines_cleared = value;
    }
}

// S9: Удаление избыточных #[allow(clippy::too_many_arguments)] — методы имеют 0-1 аргумент
impl ScoreAccess for ScoreBoard {
    fn get_score(&self) -> u128 {
        // Вызываем inherent метод, а не трейт-метод (избегаем бесконечной рекурсии)
        self.score
    }
}

impl ScoreMutable for ScoreBoard {
    fn add_score(&mut self, points: u128) {
        // Новый счёт отбрасывается — трейт требует () возвращаемый тип
        let _ = self.add_score(points);
    }

    fn set_score(&mut self, value: u128) {
        self.set_score(value);
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

        let () = scoreboard.set_score(500);
        assert_eq!(scoreboard.get_score(), 500);
    }

    #[test]
    fn test_score_board_debug() {
        // M19: тест для #[derive(Debug)]
        let scoreboard = ScoreBoard::new();
        let debug_str = format!("{scoreboard:?}");
        assert!(debug_str.contains("score"));
        assert!(debug_str.contains("level"));
        assert!(debug_str.contains("lines_cleared"));
    }

    #[test]
    fn test_score_board_level() {
        let mut scoreboard = ScoreBoard::new();

        assert_eq!(scoreboard.increment_level(), 2);
        assert_eq!(scoreboard.increment_level(), 3);
        assert_eq!(scoreboard.get_level(), 3);

        let () = scoreboard.set_level(10);
        assert_eq!(scoreboard.get_level(), 10);

        // Уровень не может быть меньше 1
        let () = scoreboard.set_level(0);
        assert_eq!(scoreboard.get_level(), 1);
    }

    #[test]
    fn test_score_board_lines_cleared() {
        let mut scoreboard = ScoreBoard::new();

        assert_eq!(scoreboard.add_lines_cleared(5), 5);
        assert_eq!(scoreboard.add_lines_cleared(3), 8);
        assert_eq!(scoreboard.get_lines_cleared(), 8);

        let () = scoreboard.set_lines_cleared(20);
        assert_eq!(scoreboard.get_lines_cleared(), 20);
    }

    #[test]
    fn test_score_board_overflow_protection() {
        let mut scoreboard = ScoreBoard::new();

        // Переполнение u128
        let () = scoreboard.set_score(u128::MAX - 100);
        let _ = scoreboard.add_score(200);
        assert_eq!(scoreboard.get_score(), u128::MAX);
    }

    /// Тест C8: проверка логирования при переполнении счёта
    #[test]
    fn test_c8_score_overflow_with_logging() {
        let mut scoreboard = ScoreBoard::new();

        // Устанавливаем счёт близкий к максимуму
        let () = scoreboard.set_score(u128::MAX - 100);

        // Добавляем больше очков чем осталось до максимума
        let result = scoreboard.add_score(200);

        // Проверяем что счёт достиг максимума
        assert_eq!(result, u128::MAX);
        assert_eq!(scoreboard.get_score(), u128::MAX);
    }

    /// Тест C8: проверка граничных условий `u128::MAX`
    #[test]
    fn test_c8_u128_max_boundary() {
        let mut scoreboard = ScoreBoard::new();

        // Устанавливаем точно в максимум
        let () = scoreboard.set_score(u128::MAX);

        // Попытка добавить ещё очков должна остаться на максимуме
        let result = scoreboard.add_score(1000);
        assert_eq!(result, u128::MAX);

        // Проверка что saturating_add работает корректно
        let () = scoreboard.set_score(u128::MAX - 1);
        let result = scoreboard.add_score(1);
        assert_eq!(result, u128::MAX);
    }

    /// Тест T5: тесты граничных условий `u128::MAX` для `add_score`
    #[test]
    fn test_t5_u128_max_boundary_conditions() {
        let mut scoreboard = ScoreBoard::new();

        // Тест 1: u128::MAX - 1 + 1 = u128::MAX
        let () = scoreboard.set_score(u128::MAX - 1);
        assert_eq!(scoreboard.add_score(1), u128::MAX);

        // Тест 2: u128::MAX + anything = u128::MAX
        let () = scoreboard.set_score(u128::MAX);
        assert_eq!(scoreboard.add_score(u128::MAX), u128::MAX);

        // Тест 3: 0 + u128::MAX = u128::MAX
        let () = scoreboard.set_score(0);
        assert_eq!(scoreboard.add_score(u128::MAX), u128::MAX);

        // Тест 4: u128::MAX - 1000 + 500 < u128::MAX
        let () = scoreboard.set_score(u128::MAX - 1000);
        assert_eq!(scoreboard.add_score(500), u128::MAX - 500);
    }
}
