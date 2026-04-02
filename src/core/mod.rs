//! Базовые типы для использования во всех модулях.
//!
//! Этот модуль содержит фундаментальные типы и перечисления,
//! которые используются несколькими модулями для предотвращения
//! циклических зависимостей.
//!
//! ## Структура модуля
//! - [`Direction`] — направление движения фигуры
//! - [`RotationDirection`] — направление вращения фигуры
//! - [`Position`] — позиция в пространстве (x, y)

/// Направление движения фигуры.
///
/// Используется в [`crate::game`] и [`crate::tetromino`] для указания
/// направления движения или вращения фигуры.
///
/// ## Пример использования
/// ```
/// use tetris_cli::core::Direction;
///
/// let dir = Direction::Left;
/// match dir {
///     Direction::Left => println!("Движение влево"),
///     Direction::Right => println!("Движение вправо"),
///     Direction::Down => println!("Движение вниз"),
/// }
/// ```
///
/// ## Соответствие направлений
/// | Direction | RotationDirection |
/// |-----------|-------------------|
/// | `Left` | `CounterClockwise` |
/// | `Right` | `Clockwise` |
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    /// Движение вниз.
    Down,
    /// Движение влево.
    Left,
    /// Движение вправо.
    Right,
}

/// Направление вращения фигуры.
///
/// Используется для вращения тетрамино по часовой или против часовой стрелки.
/// Отдельный enum предотвращает панику при передаче неправильного направления.
///
/// ## Пример использования
/// ```
/// use tetris_cli::core::RotationDirection;
///
/// let rotation = RotationDirection::Clockwise;
/// match rotation {
///     RotationDirection::Clockwise => println!("Вращение по часовой"),
///     RotationDirection::CounterClockwise => println!("Вращение против часовой"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RotationDirection {
    /// По часовой стрелке (90° вправо).
    Clockwise,
    /// Против часовой стрелки (90° влево).
    CounterClockwise,
}

/// Позиция в пространстве (x, y).
///
/// Используется для представления координат в игре.
/// Обеспечивает типобезопасность и удобство работы с координатами.
///
/// ## Пример использования
/// ```
/// use tetris_cli::core::Position;
///
/// let pos = Position::new(5, 10);
/// assert_eq!(pos.x(), 5);
/// assert_eq!(pos.y(), 10);
///
/// let tuple = pos.to_tuple();
/// assert_eq!(tuple, (5, 10));
///
/// let from_tuple = Position::from_tuple((3, 7));
/// assert_eq!(from_tuple.x(), 3);
/// assert_eq!(from_tuple.y(), 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    /// Координата X (горизонтальная позиция).
    x: i16,
    /// Координата Y (вертикальная позиция).
    y: i16,
}

impl Position {
    /// Создать новую позицию из координат x и y.
    ///
    /// # Аргументы
    /// * `x` - горизонтальная координата
    /// * `y` - вертикальная координата
    ///
    /// # Возвращает
    /// Новый экземпляр `Position`
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::core::Position;
    /// let pos = Position::new(5, 10);
    /// ```
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Получить координату X.
    #[must_use]
    #[inline]
    pub const fn x(self) -> i16 {
        self.x
    }

    /// Получить координату Y.
    #[must_use]
    #[inline]
    pub const fn y(self) -> i16 {
        self.y
    }

    /// Установить координату X.
    #[inline]
    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }

    /// Установить координату Y.
    #[inline]
    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }

    /// Установить обе координаты.
    #[inline]
    pub fn set(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    /// Создать позицию из кортежа (x, y).
    ///
    /// # Аргументы
    /// * `tuple` - кортеж с координатами (x, y)
    ///
    /// # Возвращает
    /// Новый экземпляр `Position`
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::core::Position;
    /// let pos = Position::from_tuple((5, 10));
    /// ```
    #[must_use]
    pub const fn from_tuple(tuple: (i16, i16)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }

    /// Преобразовать позицию в кортеж (x, y).
    ///
    /// # Возвращает
    /// Кортеж с координатами (x, y)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::core::Position;
    /// let pos = Position::new(5, 10);
    /// assert_eq!(pos.to_tuple(), (5, 10));
    /// ```
    #[must_use]
    pub const fn to_tuple(self) -> (i16, i16) {
        (self.x, self.y)
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
    #[must_use]
    pub const fn is_zero(self) -> bool {
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
        pos.to_tuple()
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod core_tests {
    use super::*;

    // ==================== Тесты для Direction ====================

    #[test]
    fn test_direction_variants() {
        let _ = Direction::Down;
        let _ = Direction::Left;
        let _ = Direction::Right;
    }

    #[test]
    fn test_direction_debug() {
        assert_eq!(format!("{:?}", Direction::Left), "Left");
        assert_eq!(format!("{:?}", Direction::Right), "Right");
        assert_eq!(format!("{:?}", Direction::Down), "Down");
    }

    #[test]
    fn test_direction_copy_clone() {
        let dir = Direction::Left;
        let dir_copy = dir; // Copy
        let dir_clone = dir; // Clone

        assert_eq!(dir, dir_copy);
        assert_eq!(dir, dir_clone);
    }

    // ==================== Тесты для RotationDirection ====================

    #[test]
    fn test_rotation_direction_variants() {
        let _ = RotationDirection::Clockwise;
        let _ = RotationDirection::CounterClockwise;
    }

    #[test]
    fn test_rotation_direction_debug() {
        assert_eq!(format!("{:?}", RotationDirection::Clockwise), "Clockwise");
        assert_eq!(
            format!("{:?}", RotationDirection::CounterClockwise),
            "CounterClockwise"
        );
    }

    #[test]
    fn test_rotation_direction_copy_clone() {
        let rot = RotationDirection::Clockwise;
        let rot_copy = rot;
        let rot_clone = rot;

        assert_eq!(rot, rot_copy);
        assert_eq!(rot, rot_clone);
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

    #[test]
    fn test_position_to_tuple() {
        let pos = Position::new(3, 7);
        assert_eq!(pos.to_tuple(), (3, 7));
    }

    #[test]
    fn test_position_copy_clone() {
        let pos = Position::new(5, 10);
        let pos_copy = pos;
        let pos_clone = pos;

        assert_eq!(pos, pos_copy);
        assert_eq!(pos, pos_clone);
    }

    #[test]
    fn test_position_saturating_offset() {
        // Проверка saturating_add при переполнении
        let mut pos = Position::new(i16::MAX, i16::MIN);
        pos.offset(100, -100);
        assert_eq!(pos.x(), i16::MAX); // Saturating
        assert_eq!(pos.y(), i16::MIN); // Saturating
    }
}
