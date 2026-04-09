//! Типы фигур тетромино.
//!
//! Модуль определяет enum `ShapeType` для всех 7 типов тетрамино.

/// Типы фигур тетромино.
///
/// В игре используется 7 классических фигур из тетриса.
/// Каждая фигура имеет уникальную форму и цвет.
///
/// # Варианты
/// | Вариант | Описание | Форма |
/// |---------|----------|-------|
/// | `T` | T-образная (пурпурная) | Три блока в ряд с одним блоком сверху по центру |
/// | `L` | L-образная (жёлтая) | Три блока в ряд с одним блоком снизу справа |
/// | `J` | J-образная (синяя) | Зеркальная L: три блока в ряд с одним блоком снизу слева |
/// | `S` | S-образная (зелёная) | Два блока в ряд со сдвигом вправо |
/// | `Z` | Z-образная (красная) | Зеркальная S: сдвиг влево |
/// | `O` | Квадратная (жёлтая) | Квадрат 2×2, не вращается |
/// | `I` | Линия (голубая) | Четыре блока в вертикальный ряд |
///
/// # Важно
/// Результат `as usize()` используется как индекс в массивах `SHAPE_COORDS` и `SHAPE_COLORS`.
/// Порядок вариантов определяет индекс: `T=0, L=1, J=2, S=3, Z=4, O=5, I=6`.
///
/// ## Использование
/// ```
/// use tetris_cli::tetromino::ShapeType;
///
/// let shape = ShapeType::T; // T-образная фигура
/// assert_eq!(shape as usize, 0); // Индекс для доступа к координатам и цвету
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ShapeType {
    /// T-образная фигура (пурпурная).
    /// Форма: три блока в ряд с одним блоком сверху по центру
    T,
    /// L-образная фигура (жёлтая).
    /// Форма: три блока в ряд с одним блоком снизу справа
    L,
    /// J-образная фигура (синяя, зеркальная L).
    /// Форма: три блока в ряд с одним блоком снизу слева
    J,
    /// S-образная фигура (зелёная).
    /// Форма: два блока в ряд со сдвигом вправо
    S,
    /// Z-образная фигура (светло-красная).
    /// Форма: зеркальная S - сдвиг влево
    Z,
    /// Квадратная фигура (светло-жёлтая).
    /// Форма: квадрат 2x2, не вращается
    O,
    /// Линия (голубая).
    /// Форма: четыре блока в вертикальный ряд
    I,
}

#[cfg(test)]
mod shape_type_tests {
    use super::*;

    #[test]
    fn test_shape_type_as_usize_indices() {
        // Проверяем что индексы соответствуют документации: T=0, L=1, J=2, S=3, Z=4, O=5, I=6
        assert_eq!(ShapeType::T as usize, 0);
        assert_eq!(ShapeType::L as usize, 1);
        assert_eq!(ShapeType::J as usize, 2);
        assert_eq!(ShapeType::S as usize, 3);
        assert_eq!(ShapeType::Z as usize, 4);
        assert_eq!(ShapeType::O as usize, 5);
        assert_eq!(ShapeType::I as usize, 6);
    }

    #[test]
    fn test_shape_type_clone_and_copy() {
        let original = ShapeType::T;
        let cloned = original; // Copy
        let _also = cloned; // Clone через Copy
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_shape_type_equality() {
        assert_eq!(ShapeType::T, ShapeType::T);
        assert_ne!(ShapeType::T, ShapeType::L);
    }

    #[test]
    fn test_shape_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ShapeType::T);
        set.insert(ShapeType::L);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&ShapeType::T));
        assert!(!set.contains(&ShapeType::I));
    }

    #[test]
    fn test_shape_type_debug() {
        assert_eq!(format!("{:?}", ShapeType::T), "T");
        assert_eq!(format!("{:?}", ShapeType::O), "O");
    }
}
