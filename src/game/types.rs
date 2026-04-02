//! Модуль типов для системы очков.
//!
//! Предоставляет типобезопасные обёртки для очков, уровня и линий.
//! Решает проблему Primitive Obsession.
//!
//! ## Структуры
//! - [`Score`] - типобезопасная обёртка для очков
//! - [`Level`] - типобезопасная обёртка для уровня
//! - [`LinesCount`] - типобезопасная обёртка для количества линий
//! - [`Position`] - структура для координат (x, y)
//!
//! ## Перечисления
//! - [`GameAction`] - абстракция игровых действий

#![allow(dead_code)]

use std::fmt;

// ============================================================================
// GAMEACTION ENUM (Абстракция ввода)
// ============================================================================

/// Перечисление игровых действий.
///
/// Представляет абстракцию ввода, отделяя конкретные клавиши от игровых действий.
/// Используется для маппинга клавиш → действия в системе управления.
///
/// # Использование
/// ## Парсинг ввода
/// ```ignore
/// use tetris_cli::game::types::GameAction;
/// use tetris_cli::controls::ControlsConfig;
///
/// let config = ControlsConfig::default_config();
/// let key_code = b'a'; // Нажата клавиша 'a'
///
/// if let Some(action) = config.map_key_to_action(key_code) {
///     match action {
///         GameAction::MoveLeft => println!("Движение влево"),
///         GameAction::MoveRight => println!("Движение вправо"),
///         // ... другие действия
///         _ => {}
///     }
/// }
/// ```
///
/// ## Выполнение действий
/// ```ignore
/// use tetris_cli::game::types::GameAction;
/// use tetris_cli::game::logic::input::execute_action;
///
/// let mut state = GameState::new();
/// let action = GameAction::HardDrop;
///
/// if let Some(result) = execute_action(&mut state, action) {
///     // Обработка результата (Pause, Quit)
/// }
/// ```
///
/// ## Архитектурные заметки
/// Введение GameAction соответствует:
/// - **Dependency Inversion Principle (DIP)** - модуль ввода зависит от абстракции
/// - **Interface Segregation Principle (ISP)** - узкоспециализированный интерфейс
/// - Уменьшает связанность между controls.rs и input.rs
///
/// ## Варианты
/// - `MoveLeft` - движение фигуры влево
/// - `MoveRight` - движение фигуры вправо
/// - `SoftDrop` - ускоренное падение
/// - `HardDrop` - мгновенное падение
/// - `RotateLeft` - вращение против часовой стрелки
/// - `RotateRight` - вращение по часовой стрелке
/// - `Hold` - удержание фигуры
/// - `Pause` - пауза
/// - `Quit` - выход в меню
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    /// Движение фигуры влево.
    MoveLeft,
    /// Движение фигуры вправо.
    MoveRight,
    /// Ускоренное падение (Soft Drop).
    SoftDrop,
    /// Мгновенное падение (Hard Drop).
    HardDrop,
    /// Вращение против часовой стрелки.
    RotateLeft,
    /// Вращение по часовой стрелке.
    RotateRight,
    /// Удержание фигуры.
    Hold,
    /// Пауза.
    Pause,
    /// Выход в меню.
    Quit,
}

impl GameAction {
    /// Проверить, является ли действие движением.
    #[must_use]
    pub const fn is_movement(self) -> bool {
        matches!(self, Self::MoveLeft | Self::MoveRight)
    }

    /// Проверить, является ли действие вращением.
    #[must_use]
    pub const fn is_rotation(self) -> bool {
        matches!(self, Self::RotateLeft | Self::RotateRight)
    }

    /// Проверить, является ли действие падением.
    #[must_use]
    pub const fn is_drop(self) -> bool {
        matches!(self, Self::SoftDrop | Self::HardDrop)
    }
}

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
    #[inline]
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
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::Score;
    /// let mut score = Score::with_value(100);
    /// score.add(50);
    /// assert_eq!(score.value(), 150);
    /// ```
    #[inline]
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
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::Score;
    /// let mut score = Score::with_value(100);
    /// score.multiply(2);
    /// assert_eq!(score.value(), 200);
    /// ```
    #[inline]
    pub fn multiply(&mut self, multiplier: u128) {
        self.0 = self.0.saturating_mul(multiplier);
    }

    /// Сбросить очки в ноль.
    #[inline]
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Проверить, равны ли очки нулю.
    ///
    /// # Возвращает
    /// `true` если счёт равен нулю
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::Score;
    /// let score = Score::with_value(0);
    /// assert!(score.is_zero());
    /// ```
    #[must_use]
    #[inline]
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
#[allow(dead_code)]
pub struct Level(u32);

#[allow(dead_code)]
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
    #[inline]
    pub fn value(self) -> u32 {
        self.0
    }

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// `true` если уровень был увеличен, `false` если достигнуто максимальное значение
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::Level;
    /// let mut level = Level::with_value(1);
    /// assert!(level.increment());
    /// assert_eq!(level.value(), 2);
    /// ```
    #[must_use]
    #[inline]
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
    #[inline]
    pub fn value(self) -> u32 {
        self.0
    }

    /// Добавить количество линий.
    ///
    /// # Аргументы
    /// * `amount` - количество линий для добавления
    ///
    /// # Примечания
    /// Использует saturating_add для защиты от переполнения.
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::LinesCount;
    /// let mut lines = LinesCount::with_value(10);
    /// lines.add(5);
    /// assert_eq!(lines.value(), 15);
    /// ```
    #[inline]
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
    pub fn reached(self, threshold: u32) -> bool {
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
// СТРУКТУРА ПОЗИЦИИ (Position)
// ============================================================================

/// Структура для представления координат (x, y).
///
/// Решает проблему Data Clumps (Problem 2.4) для координат.
/// Используется для устранения дублирования пар (i16, i16) в коде.
///
/// # Примеры
/// ```
/// use tetris_cli::game::types::Position;
///
/// let pos = Position::new(5, 10);
/// assert_eq!(pos.x(), 5);
/// assert_eq!(pos.y(), 10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// Координата X.
    x: i16,
    /// Координата Y.
    y: i16,
}

impl Position {
    /// Создать новую позицию с указанными координатами.
    ///
    /// # Аргументы
    /// * `x` - координата X
    /// * `y` - координата Y
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Получить координату X.
    #[must_use]
    pub const fn x(self) -> i16 {
        self.x
    }

    /// Получить координату Y.
    #[must_use]
    pub const fn y(self) -> i16 {
        self.y
    }

    /// Установить координату X.
    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }

    /// Установить координату Y.
    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }

    /// Установить обе координаты.
    pub fn set(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    /// Сдвинуть позицию на указанные значения.
    ///
    /// # Аргументы
    /// * `dx` - смещение по X
    /// * `dy` - смещение по Y
    pub fn offset(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add(dx);
        self.y = self.y.saturating_add(dy);
    }

    /// Проверить, равна ли позиция нулю.
    ///
    /// # Возвращает
    /// `true` если обе координаты равны нулю
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::types::Position;
    /// let pos = Position::new(0, 0);
    /// assert!(pos.is_zero());
    /// ```
    #[must_use]
    #[inline]
    pub fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl From<(i16, i16)> for Position {
    fn from((x, y): (i16, i16)) -> Self {
        Self::new(x, y)
    }
}

impl From<Position> for (i16, i16) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
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

    // ==================== Тесты для Position ====================

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.x(), 5);
        assert_eq!(pos.y(), 10);
    }

    #[test]
    fn test_position_default() {
        let pos = Position::default();
        assert_eq!(pos.x(), 0);
        assert_eq!(pos.y(), 0);
    }

    #[test]
    fn test_position_set() {
        let mut pos = Position::new(0, 0);
        pos.set(15, 20);
        assert_eq!(pos.x(), 15);
        assert_eq!(pos.y(), 20);
    }

    #[test]
    fn test_position_offset() {
        let mut pos = Position::new(5, 10);
        pos.offset(3, -2);
        assert_eq!(pos.x(), 8);
        assert_eq!(pos.y(), 8);
    }

    #[test]
    fn test_position_is_zero() {
        assert!(Position::new(0, 0).is_zero());
        assert!(!Position::new(1, 0).is_zero());
        assert!(!Position::new(0, 1).is_zero());
    }

    #[test]
    fn test_position_from_tuple() {
        let pos: Position = (7, 14).into();
        assert_eq!(pos.x(), 7);
        assert_eq!(pos.y(), 14);
    }

    #[test]
    fn test_position_into_tuple() {
        let pos = Position::new(9, 11);
        let tuple: (i16, i16) = pos.into();
        assert_eq!(tuple, (9, 11));
    }

    // ==================== Тесты для GameAction ====================

    #[test]
    fn test_game_action_variants() {
        use super::GameAction;

        // Проверка всех вариантов
        let _ = GameAction::MoveLeft;
        let _ = GameAction::MoveRight;
        let _ = GameAction::SoftDrop;
        let _ = GameAction::HardDrop;
        let _ = GameAction::RotateLeft;
        let _ = GameAction::RotateRight;
        let _ = GameAction::Hold;
        let _ = GameAction::Pause;
        let _ = GameAction::Quit;
    }

    #[test]
    fn test_game_action_is_movement() {
        use super::GameAction;

        assert!(GameAction::MoveLeft.is_movement());
        assert!(GameAction::MoveRight.is_movement());
        assert!(!GameAction::SoftDrop.is_movement());
        assert!(!GameAction::HardDrop.is_movement());
        assert!(!GameAction::RotateLeft.is_movement());
        assert!(!GameAction::RotateRight.is_movement());
        assert!(!GameAction::Hold.is_movement());
        assert!(!GameAction::Pause.is_movement());
        assert!(!GameAction::Quit.is_movement());
    }

    #[test]
    fn test_game_action_is_rotation() {
        use super::GameAction;

        assert!(!GameAction::MoveLeft.is_rotation());
        assert!(!GameAction::MoveRight.is_rotation());
        assert!(!GameAction::SoftDrop.is_rotation());
        assert!(!GameAction::HardDrop.is_rotation());
        assert!(GameAction::RotateLeft.is_rotation());
        assert!(GameAction::RotateRight.is_rotation());
        assert!(!GameAction::Hold.is_rotation());
        assert!(!GameAction::Pause.is_rotation());
        assert!(!GameAction::Quit.is_rotation());
    }

    #[test]
    fn test_game_action_is_drop() {
        use super::GameAction;

        assert!(!GameAction::MoveLeft.is_drop());
        assert!(!GameAction::MoveRight.is_drop());
        assert!(GameAction::SoftDrop.is_drop());
        assert!(GameAction::HardDrop.is_drop());
        assert!(!GameAction::RotateLeft.is_drop());
        assert!(!GameAction::RotateRight.is_drop());
        assert!(!GameAction::Hold.is_drop());
        assert!(!GameAction::Pause.is_drop());
        assert!(!GameAction::Quit.is_drop());
    }

    #[test]
    fn test_game_action_debug() {
        use super::GameAction;

        assert_eq!(format!("{:?}", GameAction::MoveLeft), "MoveLeft");
        assert_eq!(format!("{:?}", GameAction::HardDrop), "HardDrop");
    }

    #[test]
    fn test_game_action_copy_clone() {
        use super::GameAction;

        let action = GameAction::MoveLeft;
        let action_copy = action; // Copy
        let action_clone = action; // Clone

        assert_eq!(action, action_copy);
        assert_eq!(action, action_clone);
    }
}
