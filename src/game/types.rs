//! Модуль типов для системы очков.
//!
//! Предоставляет типобезопасные обёртки для очков, уровня и линий.
//! Решает проблему Primitive Obsession.

use std::fmt;

// ============================================================================
// ТИП-ОБЁРТКА ДЛЯ ОЧКОВ (Score)
// ============================================================================

/// Тип для представления очков игрока.
///
/// Обеспечивает типобезопасность и предотвращает путаницу с другими числовыми типами.
/// Поддерживает saturating операции для защиты от переполнения.
///
/// # Примеры
/// ```
/// use tetris_cli::game::types::Score;
///
/// let mut score = Score::new();
/// score.add(100);
/// assert_eq!(score.value(), 100);
///
/// score.add(200);
/// assert_eq!(score.value(), 300);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Score(u128);

impl Score {
    /// Создать новое значение очков с нулевым значением.
    #[must_use]
    pub fn new() -> Self {
        Self(0)
    }

    /// Создать новое значение очков с указанным значением.
    ///
    /// # Аргументы
    /// * `value` - начальное значение очков
    #[must_use]
    pub fn with_value(value: u128) -> Self {
        Self(value)
    }

    /// Получить текущее значение очков.
    #[must_use]
    pub fn value(&self) -> u128 {
        self.0
    }

    /// Добавить очки к текущему значению.
    ///
    /// # Аргументы
    /// * `points` - количество очков для добавления
    ///
    /// # Примечания
    /// Использует saturating_add для защиты от переполнения.
    pub fn add(&mut self, points: u128) {
        self.0 = self.0.saturating_add(points);
    }

    /// Умножить очки на множитель.
    ///
    /// # Аргументы
    /// * `multiplier` - множитель
    ///
    /// # Примечания
    /// Использует saturating_mul для защиты от переполнения.
    pub fn multiply(&mut self, multiplier: u128) {
        self.0 = self.0.saturating_mul(multiplier);
    }

    /// Сбросить очки в ноль.
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Проверить, равны ли очки нулю.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u128> for Score {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<Score> for u128 {
    fn from(score: Score) -> Self {
        score.0
    }
}

// ============================================================================
// ТИП-ОБЁРТКА ДЛЯ УРОВНЯ (Level)
// ============================================================================

/// Тип для представления уровня игрока.
///
/// Обеспечивает типобезопасность и инварианты (уровень >= 1).
///
/// # Примеры
/// ```
/// use tetris_cli::game::types::Level;
///
/// let mut level = Level::new();
/// assert_eq!(level.value(), 1);
///
/// level.increment();
/// assert_eq!(level.value(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level(u32);

impl Level {
    /// Создать новый уровень со значением 1 (начальный уровень).
    #[must_use]
    pub fn new() -> Self {
        Self(1)
    }

    /// Создать новый уровень с указанным значением.
    ///
    /// # Аргументы
    /// * `value` - значение уровня (минимум 1)
    ///
    /// # Примечания
    /// Если value < 1, будет установлено значение 1.
    #[must_use]
    pub fn with_value(value: u32) -> Self {
        Self(value.max(1))
    }

    /// Получить текущее значение уровня.
    #[must_use]
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// `true` если уровень был увеличен, `false` если достигнуто максимальное значение
    #[must_use]
    pub fn increment(&mut self) -> bool {
        if self.0 < u32::MAX {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    /// Увеличить уровень на указанное значение.
    ///
    /// # Аргументы
    /// * `amount` - количество уровней для добавления
    ///
    /// # Возвращает
    /// `true` если уровень был увеличен, `false` если достигнуто максимальное значение
    #[must_use]
    pub fn increment_by(&mut self, amount: u32) -> bool {
        let (new_level, overflow) = self.0.overflowing_add(amount);
        if overflow {
            self.0 = u32::MAX;
            false
        } else {
            self.0 = new_level;
            true
        }
    }

    /// Сбросить уровень к начальному значению (1).
    pub fn reset(&mut self) {
        self.0 = 1;
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ТИП-ОБЁРТКА ДЛЯ ЛИНИЙ (LinesCount)
// ============================================================================

/// Тип для представления количества удалённых линий.
///
/// Обеспечивает типобезопасность и предотвращает путаницу с очками и уровнем.
///
/// # Примеры
/// ```
/// use tetris_cli::game::types::LinesCount;
///
/// let mut lines = LinesCount::new();
/// lines.add(4);
/// assert_eq!(lines.value(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinesCount(u32);

impl LinesCount {
    /// Создать новое значение с нулевым количеством линий.
    #[must_use]
    pub fn new() -> Self {
        Self(0)
    }

    /// Создать новое значение с указанным количеством линий.
    ///
    /// # Аргументы
    /// * `value` - количество линий
    #[must_use]
    pub fn with_value(value: u32) -> Self {
        Self(value)
    }

    /// Получить текущее количество линий.
    #[must_use]
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Добавить количество линий.
    ///
    /// # Аргументы
    /// * `amount` - количество линий для добавления
    ///
    /// # Примечания
    /// Использует saturating_add для защиты от переполнения.
    pub fn add(&mut self, amount: u32) {
        self.0 = self.0.saturating_add(amount);
    }

    /// Увеличить количество линий на 1.
    ///
    /// # Возвращает
    /// `true` если значение было увеличено, `false` если достигнуто максимальное значение
    #[must_use]
    pub fn increment(&mut self) -> bool {
        if self.0 < u32::MAX {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    /// Сбросить количество линий в ноль.
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Проверить, достигло ли количество линий указанного порога.
    ///
    /// # Аргументы
    /// * `threshold` - пороговое значение
    ///
    /// # Возвращает
    /// `true` если количество линий >= threshold
    #[must_use]
    pub fn reached(&self, threshold: u32) -> bool {
        self.0 >= threshold
    }
}

impl fmt::Display for LinesCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for LinesCount {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<LinesCount> for u32 {
    fn from(lines: LinesCount) -> Self {
        lines.0
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Тесты для Score ====================

    #[test]
    fn test_score_new() {
        let score = Score::new();
        assert_eq!(score.value(), 0);
    }

    #[test]
    fn test_score_with_value() {
        let score = Score::with_value(100);
        assert_eq!(score.value(), 100);
    }

    #[test]
    fn test_score_add() {
        let mut score = Score::new();
        score.add(100);
        assert_eq!(score.value(), 100);
        score.add(200);
        assert_eq!(score.value(), 300);
    }

    #[test]
    fn test_score_multiply() {
        let mut score = Score::with_value(100);
        score.multiply(2);
        assert_eq!(score.value(), 200);
    }

    #[test]
    fn test_score_reset() {
        let mut score = Score::with_value(500);
        score.reset();
        assert_eq!(score.value(), 0);
    }

    #[test]
    fn test_score_is_zero() {
        let score = Score::new();
        assert!(score.is_zero());

        let score = Score::with_value(100);
        assert!(!score.is_zero());
    }

    #[test]
    fn test_score_display() {
        let score = Score::with_value(1234);
        assert_eq!(format!("{score}"), "1234");
    }

    #[test]
    fn test_score_from_u128() {
        let score: Score = 500.into();
        assert_eq!(score.value(), 500);
    }

    #[test]
    fn test_score_into_u128() {
        let score = Score::with_value(750);
        let value: u128 = score.into();
        assert_eq!(value, 750);
    }

    #[test]
    fn test_score_saturating_add() {
        let mut score = Score::with_value(u128::MAX);
        score.add(100);
        assert_eq!(score.value(), u128::MAX); // Saturating
    }

    // ==================== Тесты для Level ====================

    #[test]
    fn test_level_new() {
        let level = Level::new();
        assert_eq!(level.value(), 1);
    }

    #[test]
    fn test_level_with_value() {
        let level = Level::with_value(5);
        assert_eq!(level.value(), 5);
    }

    #[test]
    fn test_level_with_value_minimum() {
        let level = Level::with_value(0);
        assert_eq!(level.value(), 1); // Minimum is 1
    }

    #[test]
    fn test_level_increment() {
        let mut level = Level::new();
        assert!(level.increment());
        assert_eq!(level.value(), 2);
    }

    #[test]
    fn test_level_increment_by() {
        let mut level = Level::new();
        assert!(level.increment_by(5));
        assert_eq!(level.value(), 6);
    }

    #[test]
    fn test_level_reset() {
        let mut level = Level::with_value(10);
        level.reset();
        assert_eq!(level.value(), 1);
    }

    #[test]
    fn test_level_display() {
        let level = Level::with_value(42);
        assert_eq!(format!("{level}"), "42");
    }

    // ==================== Тесты для LinesCount ====================

    #[test]
    fn test_lines_count_new() {
        let lines = LinesCount::new();
        assert_eq!(lines.value(), 0);
    }

    #[test]
    fn test_lines_count_with_value() {
        let lines = LinesCount::with_value(10);
        assert_eq!(lines.value(), 10);
    }

    #[test]
    fn test_lines_count_add() {
        let mut lines = LinesCount::new();
        lines.add(4);
        assert_eq!(lines.value(), 4);
        lines.add(6);
        assert_eq!(lines.value(), 10);
    }

    #[test]
    fn test_lines_count_increment() {
        let mut lines = LinesCount::new();
        assert!(lines.increment());
        assert_eq!(lines.value(), 1);
    }

    #[test]
    fn test_lines_count_reset() {
        let mut lines = LinesCount::with_value(40);
        lines.reset();
        assert_eq!(lines.value(), 0);
    }

    #[test]
    fn test_lines_count_reached() {
        let lines = LinesCount::with_value(40);
        assert!(lines.reached(40));
        assert!(lines.reached(30));
        assert!(!lines.reached(50));
    }

    #[test]
    fn test_lines_count_display() {
        let lines = LinesCount::with_value(150);
        assert_eq!(format!("{lines}"), "150");
    }

    #[test]
    fn test_lines_count_from_u32() {
        let lines: LinesCount = 25.into();
        assert_eq!(lines.value(), 25);
    }

    #[test]
    fn test_lines_count_into_u32() {
        let lines = LinesCount::with_value(75);
        let value: u32 = lines.into();
        assert_eq!(value, 75);
    }

    #[test]
    fn test_lines_count_saturating_add() {
        let mut lines = LinesCount::with_value(u32::MAX);
        lines.add(100);
        assert_eq!(lines.value(), u32::MAX); // Saturating
    }
}
