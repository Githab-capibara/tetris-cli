//! Общие типы и перечисления для использования во всех модулях.
//!
//! Этот модуль содержит базовые типы, которые используются несколькими модулями
//! для предотвращения циклических зависимостей.
//!
//! ## Структура модуля
//! - [`Direction`] — направление движения фигуры
//! - [`RotationDirection`] — направление вращения фигуры
//! - [`UpdateEndState`] — состояние завершения обновления

/// Направление движения фигуры.
///
/// Используется в [`crate::game`] и [`crate::tetromino`] для указания
/// направления движения или вращения фигуры.
///
/// ## Пример использования
/// ```
/// use tetris_cli::types::Direction;
///
/// let dir = Direction::Left;
/// match dir {
///     Direction::Left => println!("Движение влево"),
///     Direction::Right => println!("Движение вправо"),
///     Direction::Down => println!("Движение вниз"),
/// }
/// ```
///
/// ## Исправление #4 (Direction vs `RotationDirection`)
/// Для конвертации в направление вращения используйте метод
/// [`to_rotation_direction()`](Self::to_rotation_direction).
///
/// ### Соответствие направлений
/// | Direction | RotationDirection |
/// |-----------|-------------------|
/// | `Left` | `CounterClockwise` |
/// | `Right` | `Clockwise` |
/// | `Down` | `Clockwise` (по умолчанию) |
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    /// Движение вниз.
    Down,
    /// Движение влево.
    Left,
    /// Движение вправо.
    Right,
}

impl Direction {
    /// Конвертировать [`Direction`] в [`RotationDirection`].
    ///
    /// # Возвращает
    /// - [`RotationDirection::CounterClockwise`] для [`Direction::Left`]
    /// - [`RotationDirection::Clockwise`] для [`Direction::Right`]
    /// - [`RotationDirection::NoRotation`] для [`Direction::Down`]
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::types::{Direction, RotationDirection};
    ///
    /// assert_eq!(
    ///     Direction::Left.to_rotation_direction(),
    ///     RotationDirection::CounterClockwise
    /// );
    /// assert_eq!(
    ///     Direction::Right.to_rotation_direction(),
    ///     RotationDirection::Clockwise
    /// );
    /// assert_eq!(
    ///     Direction::Down.to_rotation_direction(),
    ///     RotationDirection::NoRotation
    /// );
    /// ```
    ///
    /// # Примечания
    /// Метод помечен как `#[allow(dead_code)]` так как используется
    /// только в устаревшем коде. Новый код должен явно указывать
    /// направление вращения через `RotationDirection`.
    ///
    /// # Исправление 1.3
    /// `Direction::Down` больше не конвертируется в `RotationDirection::Clockwise`.
    ///
    /// # Исправление аудита 2026-03-30
    /// Возвращает `RotationDirection::NoRotation` для `Direction::Down`
    /// для явного указания отсутствия вращения.
    ///
    /// # Panics
    /// Никогда не паникует — все варианты `Direction` обрабатываются корректно.
    #[must_use]
    #[allow(dead_code)]
    pub const fn to_rotation_direction(self) -> RotationDirection {
        match self {
            Direction::Left => RotationDirection::CounterClockwise,
            Direction::Right => RotationDirection::Clockwise,
            Direction::Down => RotationDirection::NoRotation,
        }
    }
}

/// Направление вращения фигуры.
///
/// Используется для вращения тетрамино по часовой или против часовой стрелки.
/// Отдельный enum предотвращает панику при передаче неправильного направления.
///
/// ## Пример использования
/// ```
/// use tetris_cli::types::RotationDirection;
///
/// let rotation = RotationDirection::Clockwise;
/// match rotation {
///     RotationDirection::Clockwise => println!("Вращение по часовой"),
///     RotationDirection::CounterClockwise => println!("Вращение против часовой"),
///     RotationDirection::NoRotation => println!("Без вращения"),
/// }
/// ```
///
/// ## Исправление аудита 2026-03-30
/// Добавлен вариант `NoRotation` для явного указания отсутствия вращения
/// при конвертации из `Direction::Down`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotationDirection {
    /// По часовой стрелке (90° вправо).
    Clockwise,
    /// Против часовой стрелки (90° влево).
    CounterClockwise,
    /// Без вращения (используется для `Direction::Down`).
    NoRotation,
}

/// Позиция в пространстве (x, y).
///
/// Используется для представления координат в игре.
/// Обеспечивает типобезопасность и удобство работы с координатами.
///
/// ## Пример использования
/// ```
/// use tetris_cli::types::Position;
///
/// let pos = Position::new(5, 10);
/// assert_eq!(pos.x, 5);
/// assert_eq!(pos.y, 10);
///
/// let tuple = pos.to_tuple();
/// assert_eq!(tuple, (5, 10));
///
/// let from_tuple = Position::from_tuple((3, 7));
/// assert_eq!(from_tuple.x, 3);
/// assert_eq!(from_tuple.y, 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// Координата X (горизонтальная позиция).
    pub x: u16,
    /// Координата Y (вертикальная позиция).
    pub y: u16,
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
    /// use tetris_cli::types::Position;
    /// let pos = Position::new(5, 10);
    /// ```
    #[must_use]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
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
    /// use tetris_cli::types::Position;
    /// let pos = Position::from_tuple((5, 10));
    /// ```
    #[must_use]
    pub fn from_tuple(tuple: (u16, u16)) -> Self {
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
    /// use tetris_cli::types::Position;
    /// let pos = Position::new(5, 10);
    /// assert_eq!(pos.to_tuple(), (5, 10));
    /// ```
    #[must_use]
    pub fn to_tuple(self) -> (u16, u16) {
        (self.x, self.y)
    }
}

/// Состояние завершения обновления.
///
/// Возвращается методами обновления игры для указания текущего состояния.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum UpdateEndState {
    /// Выход из игры.
    Quit,
    /// Проигрыш.
    Lost,
    /// Продолжить.
    Continue,
    /// Пауза.
    Pause,
    /// Победа (завершение режима спринт/марафон).
    Won,
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_direction_to_rotation_direction() {
        assert_eq!(
            Direction::Left.to_rotation_direction(),
            Some(RotationDirection::CounterClockwise)
        );
        assert_eq!(
            Direction::Right.to_rotation_direction(),
            Some(RotationDirection::Clockwise)
        );
        assert_eq!(
            Direction::Down.to_rotation_direction(),
            Some(RotationDirection::NoRotation)
        );
    }

    #[test]
    fn test_direction_debug() {
        assert_eq!(format!("{:?}", Direction::Left), "Left");
        assert_eq!(format!("{:?}", Direction::Right), "Right");
        assert_eq!(format!("{:?}", Direction::Down), "Down");
    }

    #[test]
    fn test_rotation_direction_debug() {
        assert_eq!(format!("{:?}", RotationDirection::Clockwise), "Clockwise");
        assert_eq!(
            format!("{:?}", RotationDirection::CounterClockwise),
            "CounterClockwise"
        );
        assert_eq!(format!("{:?}", RotationDirection::NoRotation), "NoRotation");
    }

    #[test]
    fn test_update_end_state_debug() {
        assert_eq!(format!("{:?}", UpdateEndState::Quit), "Quit");
        assert_eq!(format!("{:?}", UpdateEndState::Lost), "Lost");
        assert_eq!(format!("{:?}", UpdateEndState::Continue), "Continue");
        assert_eq!(format!("{:?}", UpdateEndState::Pause), "Pause");
        assert_eq!(format!("{:?}", UpdateEndState::Won), "Won");
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ПРОВЕРКИ DIRECTION::DOWN И NOROTATION (ИСПРАВЛЕНИЕ АУДИТА)
    // =========================================================================

    /// Тест: проверка что Direction::Down возвращает RotationDirection::NoRotation
    #[test]
    fn test_direction_down_returns_no_rotation() {
        // Проверка что Direction::Down возвращает RotationDirection::NoRotation
        assert_eq!(
            Direction::Down.to_rotation_direction(),
            RotationDirection::NoRotation
        );
    }

    /// Тест: проверка что NoRotation существует
    #[test]
    fn test_rotation_direction_no_rotation() {
        // Проверка что NoRotation существует
        let _ = RotationDirection::NoRotation;
    }
}
