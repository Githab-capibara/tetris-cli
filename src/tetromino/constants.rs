//! Константы для фигур тетромино.
//!
//! # Содержимое
//! - `SHAPE_COORDS` - координаты блоков для каждой фигуры
//! - `SHAPE_COLORS` - цвета для отрисовки
//! - Helper функции для доступа к координатам

use termion::color::{Blue, Color, Cyan, Green, LightRed, LightYellow, Magenta, Yellow};

/// Координаты блоков для каждого типа фигур.
///
/// Каждая фигура представлена 4 блоками с координатами относительно центра.
/// Индексы: 0=T, 1=L, 2=J, 3=S, 4=Z, 5=O, 6=I
///
/// ## Магические числа координат:
/// - Диапазон X/Y: от -2 до +2 (достаточно для всех вращений)
/// - T: (-1,0), (0,0), (1,0), (0,1) - три в ряд + один сверху по центру
/// - L: (-1,-1), (0,-1), (0,0), (0,1) - три вертикально + один слева сверху
/// - J: (1,-1), (0,-1), (0,0), (0,1) - зеркальная L
/// - S: (0,-1), (0,0), (1,0), (1,1) - S-образная форма
/// - Z: (0,-1), (0,0), (-1,0), (-1,1) - зеркальная S
/// - O: (0,0), (1,0), (0,1), (1,1) - квадрат 2x2
/// - I: (0,-1), (0,0), (0,1), (0,2) - вертикальная линия
///
/// ## ISSUE-081: Исправление
/// Для доступа используйте helper функции:
/// - `get_shape_coords(shape_index)` - получить координаты фигуры
/// - `get_shape_block_coords(shape_index, block_index)` - получить координаты конкретного блока
pub const SHAPE_COORDS: [[(i16, i16); 4]; 7] = [
    [(-1, 0), (0, 0), (1, 0), (0, 1)],   // T
    [(-1, -1), (0, -1), (0, 0), (0, 1)], // L
    [(1, -1), (0, -1), (0, 0), (0, 1)],  // J (зеркальная L)
    [(0, -1), (0, 0), (1, 0), (1, 1)],   // S
    [(0, -1), (0, 0), (-1, 0), (-1, 1)], // Z
    [(0, 0), (1, 0), (0, 1), (1, 1)],    // O (квадрат)
    [(0, -1), (0, 0), (0, 1), (0, 2)],   // I (линия)
];

/// Цвета для каждой фигуры.
///
/// Индексы соответствуют `SHAPE_COORDS`:
/// 0=T→Пурпурный, 1=L→Жёлтый, 2=J→Синий, 3=S→Зелёный,
/// 4=Z→Св.красный, 5=O→Св.жёлтый, 6=I→Голубой
///
/// Проблема 51: используется `&dyn Color` (динамическая диспетчеризация) поскольку
/// каждая фигура имеет свой конкретный цвет. Замена на enum потребовала бы
/// значительной переработки системы отрисовки без заметной выгоды для CLI.
pub const SHAPE_COLORS: [&dyn Color; 7] = [
    &Magenta,
    &Yellow,
    &Blue,
    &Green,
    &LightRed,
    &LightYellow,
    &Cyan,
];

// ============================================================================
// HELPER ФУНКЦИИ ДЛЯ ДОСТУПА (ISSUE-081, ISSUE-082)
// ============================================================================

/// Количество фигур в массивах `SHAPE_COORDS` и `SHAPE_COLORS`.
///
/// Используется для проверки валидности индексов фигур и замены магических чисел.
/// Значение 7 соответствует 7 типам тетрамино: T, L, J, S, Z, O, I.
pub const SHAPE_COUNT: usize = 7;

/// Получить координаты блоков для фигуры.
///
/// # Аргументы
/// * `shape_index` - индекс фигуры (0-6)
///
/// # Возвращает
/// Срез координат [(i16, i16); 4]. При невалидном индексе возвращает
/// координаты T-фигуры (индекс 0) как безопасное значение по умолчанию.
///
/// # Пример
/// ```ignore
/// use crate::tetromino::constants::get_shape_coords;
/// let t_coords = get_shape_coords(0); // T фигура
/// ```
#[must_use]
#[allow(dead_code)] // Публичный API для будущих расширений
pub const fn get_shape_coords(shape_index: usize) -> &'static [(i16, i16); 4] {
    if shape_index < SHAPE_COUNT {
        &SHAPE_COORDS[shape_index]
    } else {
        &SHAPE_COORDS[0]
    }
}

/// Получить координаты конкретного блока фигуры.
///
/// # Аргументы
/// * `shape_index` - индекс фигуры (0-6)
/// * `block_index` - индекс блока (0-3)
///
/// # Возвращает
/// Координаты блока (x, y). При невалидном индексе фигуры или блока
/// возвращает (0, 0) как безопасное значение по умолчанию.
///
/// # Пример
/// ```ignore
/// use crate::tetromino::constants::get_shape_block_coords;
/// let (x, y) = get_shape_block_coords(0, 0); // Первый блок T фигуры
/// ```
#[must_use]
#[allow(dead_code)] // Публичный API для будущих расширений
pub const fn get_shape_block_coords(shape_index: usize, block_index: usize) -> (i16, i16) {
    if shape_index < SHAPE_COUNT {
        let coords = SHAPE_COORDS[shape_index];
        if block_index < coords.len() {
            coords[block_index]
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    }
}

/// Получить цвет для фигуры.
///
/// # Аргументы
/// * `shape_index` - индекс фигуры (0-6)
///
/// # Возвращает
/// Ссылка на цвет. При невалидном индексе возвращает цвет T-фигуры (Magenta)
/// как безопасное значение по умолчанию.
///
/// # Пример
/// ```ignore
/// use crate::tetromino::constants::get_shape_color;
/// let color = get_shape_color(0); // Цвет T фигуры
/// ```
#[must_use]
#[inline]
#[allow(dead_code)] // Публичный API для будущих расширений
pub fn get_shape_color(shape_index: usize) -> &'static dyn Color {
    SHAPE_COLORS
        .get(shape_index)
        .copied()
        .unwrap_or(SHAPE_COLORS[0])
}

#[cfg(test)]
mod constants_tests {
    use super::*;

    #[test]
    fn test_get_shape_coords_valid_indices() {
        for i in 0..7 {
            let coords = get_shape_coords(i);
            assert_eq!(coords.len(), 4, "Каждая фигура должна иметь 4 блока");
        }
    }

    #[test]
    fn test_get_shape_coords_invalid_index_returns_default() {
        // Невалидный индекс должен вернуть координаты T-фигуры (индекс 0)
        let default_coords = get_shape_coords(0);
        assert_eq!(get_shape_coords(7), default_coords);
        assert_eq!(get_shape_coords(100), default_coords);
        assert_eq!(get_shape_coords(usize::MAX), default_coords);
    }

    #[test]
    fn test_get_shape_block_coords_valid_indices() {
        // Проверяем что все валидные индексы возвращают координаты в ожидаемом диапазоне
        for i in 0..7 {
            for b in 0..4 {
                let (x, y) = get_shape_block_coords(i, b);
                // Координаты фигур в пределах [-2, 2]
                assert!(
                    (-3..=3).contains(&x),
                    "X координата блока {b} фигуры {i} вне диапазона: {x}"
                );
                assert!(
                    (-3..=3).contains(&y),
                    "Y координата блока {b} фигуры {i} вне диапазона: {y}"
                );
            }
        }
    }

    #[test]
    fn test_get_shape_block_coords_invalid_indices_return_default() {
        assert_eq!(get_shape_block_coords(7, 0), (0, 0));
        assert_eq!(get_shape_block_coords(0, 4), (0, 0));
        assert_eq!(get_shape_block_coords(usize::MAX, usize::MAX), (0, 0));
    }

    #[test]
    fn test_get_shape_color_valid_indices() {
        // Проверяем что все валидные индексы возвращают цвет
        for i in 0..7 {
            let color = get_shape_color(i);
            // Проверяем что цвет не panic при использовании
            let _ = format!("{color:?}");
        }
    }

    #[test]
    fn test_get_shape_color_invalid_index_returns_default() {
        // Невалидный индекс должен вернуть цвет T-фигуры (индекс 0)
        let default_color = get_shape_color(0);
        // Сравниваем через адреса указателей, так как dyn Color не реализует PartialEq
        assert!(
            std::ptr::eq(
                std::ptr::from_ref(get_shape_color(7)),
                std::ptr::from_ref(default_color)
            ) || std::ptr::eq(
                std::ptr::from_ref(get_shape_color(100)),
                std::ptr::from_ref(default_color)
            ),
            "Невалидный индекс должен вернуть цвет по умолчанию"
        );
    }
}
