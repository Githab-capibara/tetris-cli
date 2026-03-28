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
/// ## Исправление #4 (Direction vs RotationDirection)
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
    /// - `None` для [`Direction::Down`] (не является направлением вращения)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::types::{Direction, RotationDirection};
    ///
    /// assert_eq!(
    ///     Direction::Left.to_rotation_direction(),
    ///     Some(RotationDirection::CounterClockwise)
    /// );
    /// assert_eq!(
    ///     Direction::Right.to_rotation_direction(),
    ///     Some(RotationDirection::Clockwise)
    /// );
    /// assert_eq!(Direction::Down.to_rotation_direction(), None);
    /// ```
    ///
    /// # Примечания
    /// Метод помечен как `#[allow(dead_code)]` так как используется
    /// только в устаревшем коде. Новый код должен явно указывать
    /// направление вращения через `RotationDirection`.
    ///
    /// # Исправление 1.3
    /// Direction::Down больше не конвертируется в RotationDirection::Clockwise.
    #[must_use]
    #[allow(dead_code)]
    pub const fn to_rotation_direction(self) -> Option<RotationDirection> {
        match self {
            Direction::Left => Some(RotationDirection::CounterClockwise),
            Direction::Right => Some(RotationDirection::Clockwise),
            Direction::Down => None,
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
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotationDirection {
    /// По часовой стрелке (90° вправо).
    Clockwise,
    /// Против часовой стрелки (90° влево).
    CounterClockwise,
}

/// Состояние завершения обновления.
///
/// Возвращается методами обновления игры для указания текущего состояния.
#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(dead_code)]
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
        assert_eq!(Direction::Down.to_rotation_direction(), None);
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
    }

    #[test]
    fn test_update_end_state_debug() {
        assert_eq!(format!("{:?}", UpdateEndState::Quit), "Quit");
        assert_eq!(format!("{:?}", UpdateEndState::Lost), "Lost");
        assert_eq!(format!("{:?}", UpdateEndState::Continue), "Continue");
        assert_eq!(format!("{:?}", UpdateEndState::Pause), "Pause");
        assert_eq!(format!("{:?}", UpdateEndState::Won), "Won");
    }
}
