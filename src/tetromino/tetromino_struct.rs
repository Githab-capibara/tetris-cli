//! Структура тетромино.
//!
//! Модуль определяет `Tetromino` - падающую фигуру в игре.

use crate::tetromino::bag_generator::BagGenerator;
use crate::tetromino::constants::SHAPE_COORDS;
use crate::tetromino::shape_type::ShapeType;
use crate::types::RotationDirection;

/// Тетромино — падающая фигура.
///
/// Содержит всю информацию о фигуре: тип, позицию, координаты блоков и цвет.
/// Использует плавающие координаты для плавного падения.
///
/// ## Структура
/// - `pos` - позиция фигуры в плавающих координатах (x, y)
/// - `shape` - тип фигуры (T, L, J, S, Z, O, I)
/// - `fg` - индекс цвета (0-6)
/// - `coords` - координаты блоков относительно центра
///
/// ## Исправление #20 (LOW)
/// Поля переупорядочены для уменьшения padding: `fg` изменён на `u8` и перемещён
/// ближе к `shape` для оптимального выравнивания.
///
/// ## Исправление M13 (MEDIUM)
/// Добавлена compile-time проверка размера структуры для контроля памяти.
/// Размер Tetromino не должен превышать 32 байт для оптимального копирования.
///
/// ## Исправление аудита 2026-03-31 (Проблема #2)
/// Поля структуры сделаны приватными. Используйте геттеры для доступа:
/// - `pos()` - позиция фигуры
/// - `shape()` - тип фигуры
/// - `fg()` - индекс цвета
/// - `coords()` - координаты блоков
///
/// ## Пример использования
/// ```
/// use tetris_cli::tetromino::{Tetromino, BagGenerator};
///
/// // Создание фигуры из мешка
/// let mut bag = BagGenerator::new();
/// let tetromino = Tetromino::from_bag(&mut bag);
/// assert_eq!(tetromino.pos(), (4.0, 0.0)); // Начальная позиция
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    /// Позиция фигуры (x, y) в плавающих координатах.
    ///
    /// Используется f32 для плавного падения с интерполяцией.
    /// Начальная позиция: (4.0, 0.0) - центр сверху.
    pos: (f32, f32),
    /// Тип фигуры (T, L, J, S, Z, O, I).
    ///
    /// Определяет форму и цвет фигуры.
    shape: ShapeType,
    /// Индекс цвета (0-6).
    ///
    /// Соответствует индексу фигуры в `SHAPE_COLORS`.
    ///
    /// # Исправление #20 (LOW)
    /// Используется `u8` вместо `usize` для экономии памяти (1 байт вместо 8).
    fg: u8,
    /// Координаты блоков относительно центра.
    ///
    /// Массив из 4 пар координат (x, y).
    /// Изменяется при вращении фигуры.
    coords: [(i16, i16); 4],
}

// Исправление #20: compile-time проверка размера Tetromino
// Размер структуры оптимизирован: pos: 8 байт + shape: 1 байт + fg: 1 байт + coords: 16 байт = 26 байт + padding = 32 байта
const _: () = assert!(
    std::mem::size_of::<Tetromino>() <= 32,
    "Размер Tetromino не должен превышать 32 байт"
);

#[allow(dead_code)]
impl Tetromino {
    // ========================================================================
    // ГЕТТЕРЫ (ИСПРАВЛЕНИЕ #2 - ИНКАПСУЛЯЦИЯ)
    // ========================================================================

    /// Получить позицию фигуры.
    ///
    /// # Возвращает
    /// Кортеж (x, y) с плавающими координатами
    #[must_use]
    pub const fn pos(&self) -> (f32, f32) {
        self.pos
    }

    /// Получить тип фигуры.
    ///
    /// # Возвращает
    /// ShapeType фигуры (T, L, J, S, Z, O, I)
    #[must_use]
    pub const fn shape(&self) -> ShapeType {
        self.shape
    }

    /// Получить индекс цвета.
    ///
    /// # Возвращает
    /// Индекс цвета (0-6)
    #[must_use]
    pub const fn fg(&self) -> u8 {
        self.fg
    }

    /// Получить координаты блоков.
    ///
    /// # Возвращает
    /// Массив из 4 пар координат (x, y) относительно центра
    #[must_use]
    pub const fn coords(&self) -> [(i16, i16); 4] {
        self.coords
    }

    /// Получить мутабельную ссылку на позицию.
    ///
    /// # Возвращает
    /// Мутабельная ссылка на кортеж (x, y)
    #[must_use]
    pub fn pos_mut(&mut self) -> &mut (f32, f32) {
        &mut self.pos
    }

    /// Получить мутабельную ссылку на координаты блоков.
    ///
    /// # Возвращает
    /// Мутабельная ссылка на массив координат
    #[must_use]
    pub fn coords_mut(&mut self) -> &mut [(i16, i16); 4] {
        &mut self.coords
    }

    /// Создать новую фигуру с заданными параметрами.
    ///
    /// # Аргументы
    /// * `pos` - позиция фигуры
    /// * `shape` - тип фигуры
    /// * `coords` - координаты блоков
    /// * `fg` - индекс цвета
    ///
    /// # Возвращает
    /// Новый Tetromino
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::tetromino::{Tetromino, ShapeType};
    ///
    /// let tetromino = Tetromino::new(
    ///     (4.0, 0.0),
    ///     ShapeType::T,
    ///     [(-1, 0), (0, 0), (1, 0), (0, 1)],
    ///     0,
    /// );
    /// ```
    #[must_use]
    pub const fn new(pos: (f32, f32), shape: ShapeType, coords: [(i16, i16); 4], fg: u8) -> Self {
        Self {
            pos,
            shape,
            fg,
            coords,
        }
    }

    /// Установить позицию фигуры.
    ///
    /// # Аргументы
    /// * `pos` - новая позиция (x, y)
    #[allow(dead_code)]
    pub fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    /// Установить тип фигуры.
    ///
    /// # Аргументы
    /// * `shape` - новый тип фигуры
    #[allow(dead_code)]
    pub fn set_shape(&mut self, shape: ShapeType) {
        self.shape = shape;
    }

    /// Установить индекс цвета.
    ///
    /// # Аргументы
    /// * `fg` - новый индекс цвета (0-6)
    #[allow(dead_code)]
    pub fn set_fg(&mut self, fg: u8) {
        self.fg = fg;
    }

    /// Установить координаты блоков.
    ///
    /// # Аргументы
    /// * `coords` - новый массив координат
    #[allow(dead_code)]
    pub fn set_coords(&mut self, coords: [(i16, i16); 4]) {
        self.coords = coords;
    }

    // ========================================================================
    // ОСНОВНЫЕ МЕТОДЫ
    // ========================================================================

    /// Создать новую фигуру из Bag Generator.
    ///
    /// Использует систему 7-bag для гарантированного появления всех 7 типов фигур.
    ///
    /// # Аргументы
    /// * `bag` - генератор фигур по системе 7-bag
    ///
    /// # Возвращает
    /// Новый Tetromino с фигурой из мешка и начальной позицией (4.0, 0.0)
    pub fn from_bag(bag: &mut BagGenerator) -> Self {
        let shape = bag.next_shape();
        Self {
            pos: (4.0, 0.0), // Начальная позиция по центру
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: shape as u8,
        }
    }

    /// Вращать фигуру в заданном направлении.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения:
    ///   - `RotationDirection::Clockwise` - по часовой стрелке (90° вправо)
    ///   - `RotationDirection::CounterClockwise` - против часовой стрелки (90° влево)
    ///
    /// # Panics
    /// Паникует, если координаты фигуры выходят за безопасные пределы
    /// во время вращения (проверяется через assert в режиме debug)
    ///
    /// # Примечания
    /// - Квадрат (O) не вращается - метод возвращает управление сразу
    /// - Вращение на 90 градусов вокруг центра фигуры
    /// - Используется формула поворота:
    ///   - По часовой: (x, y) -> (-y, x)
    ///   - Против часовой: (x, y) -> (y, -x)
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::tetromino::{Tetromino, ShapeType};
    /// use tetris_cli::types::RotationDirection;
    ///
    /// let mut t = Tetromino {
    ///     pos: (4.0, 0.0),
    ///     shape: ShapeType::T,
    ///     coords: [(-1, 0), (0, 0), (1, 0), (0, 1)],
    ///     fg: 0,
    /// };
    /// t.rotate(RotationDirection::Clockwise); // Поворот по часовой
    /// ```
    ///
    /// # Panics
    /// Никогда не паникует. При переполнении координат (i16::MIN при отрицании)
    /// метод логирует предупреждение и пропускает вращение.
    pub fn rotate(&mut self, dir: RotationDirection) {
        // Квадрат не вращается
        if self.shape == ShapeType::O {
            return;
        }

        // Исправление E3 (CRITICAL): замена saturating_neg на checked_neg с обработкой None
        // saturating_neg() может вернуть некорректное значение для i16::MIN
        // checked_neg() возвращает None при переполнении, что позволяет явно обработать ошибку
        for i in 0..4 {
            let (x, y) = self.coords[i];
            let (new_x, new_y) = match dir {
                // Против часовой: (x, y) -> (y, -x)
                RotationDirection::CounterClockwise => {
                    // Используем checked_neg() для безопасного отрицания
                    if let Some(neg_x) = x.checked_neg() {
                        (y, neg_x)
                    } else {
                        // Переполнение при отрицании x (i16::MIN)
                        eprintln!(
                            "[WARN] Вращение фигуры пропущено: переполнение при checked_neg({x})"
                        );
                        return;
                    }
                }
                // По часовой: (x, y) -> (-y, x)
                RotationDirection::Clockwise => {
                    // Используем checked_neg() для безопасного отрицания
                    if let Some(neg_y) = y.checked_neg() {
                        (neg_y, x)
                    } else {
                        // Переполнение при отрицании y (i16::MIN)
                        eprintln!(
                            "[WARN] Вращение фигуры пропущено: переполнение при checked_neg({y})"
                        );
                        return;
                    }
                }
            };

            // Явная проверка границ вместо assert - предотвращает панику в релизном режиме
            // Координаты должны оставаться в пределах безопасного диапазона
            if !(i16::MIN / 2..=i16::MAX / 2).contains(&new_x)
                || !(i16::MIN / 2..=i16::MAX / 2).contains(&new_y)
            {
                // Координаты вышли за безопасные пределы - пропускаем вращение
                // Это предотвращает панику и сохраняет фигуру в безопасном состоянии
                eprintln!(
                    "[WARN] Вращение фигуры пропущено: координаты вышли за безопасные пределы"
                );
                return;
            }

            self.coords[i] = (new_x, new_y);
        }
    }
}

#[cfg(test)]
mod tetromino_tests {
    use super::*;

    #[test]
    fn test_tetromino_from_bag() {
        let mut bag = BagGenerator::new();
        let tetromino = Tetromino::from_bag(&mut bag);

        assert_eq!(tetromino.pos, (4.0, 0.0));
        assert!((tetromino.fg as usize) < 7);
    }

    #[test]
    fn test_tetromino_rotate_o_shape() {
        let mut tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::O,
            coords: SHAPE_COORDS[5],
            fg: 5,
        };
        let original_coords = tetromino.coords;

        tetromino.rotate(RotationDirection::Clockwise);
        assert_eq!(
            tetromino.coords, original_coords,
            "O-фигура не должна вращаться"
        );
    }

    #[test]
    fn test_tetromino_rotate_t_shape() {
        let mut tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: SHAPE_COORDS[0],
            fg: 0,
        };

        tetromino.rotate(RotationDirection::Clockwise);
        // После вращения по часовой: (x,y) -> (-y,x)
        // (-1,0) -> (0,-1), (0,0) -> (0,0), (1,0) -> (0,1), (0,1) -> (-1,0)
        assert_eq!(tetromino.coords[0], (0, -1));
        assert_eq!(tetromino.coords[1], (0, 0));
    }
}
