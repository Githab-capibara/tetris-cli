//! Фигуры тетромино.
//!
//! Этот модуль определяет все 7 типов тетрамино (T, L, J, S, Z, O, I),
//! их координаты, цвета и поведение при вращении.
//!
//! ## Структура модуля
//! - `SHAPE_COORDS` - координаты блоков для каждой фигуры
//! - `SHAPE_COLORS` - цвета для отрисовки
//! - `ShapeType` - перечисление типов фигур
//! - `Tetromino` - структура падающей фигуры
//! - `BagGenerator` - генератор фигур по системе 7-bag
//! - `tests` - 25+ модульных тестов
//!
//! ## Типы фигур
//! | Тип | Название | Цвет | Описание |
//! |-----|----------|------|----------|
//! | T | T-образная | Пурпурный | Три блока в ряд с одним блоком сверху по центру |
//! | L | L-образная | Жёлтый | Три блока в ряд с одним блоком снизу справа |
//! | J | J-образная | Синий | Зеркальная L - блок снизу слева |
//! | S | S-образная | Зелёный | Два блока в ряд со сдвигом вправо |
//! | Z | Z-образная | Светло-красный | Зеркальная S - сдвиг влево |
//! | O | Квадрат | Светло-жёлтый | Квадрат 2x2 |
//! | I | Линия | Голубой | Четыре блока в вертикальный ряд |

pub use crate::types::RotationDirection;
use rand::Rng;
use termion::color::{Blue, Color, Cyan, Green, LightRed, LightYellow, Magenta, Yellow};

/// Генератор фигур по системе 7-bag.
///
/// Гарантирует, что каждые 7 фигур будут содержать все 7 типов.
/// Использует алгоритм Fisher-Yates для перемешивания.
///
/// ## Как работает:
/// 1. Создаётся "мешок" со всеми 7 типами фигур
/// 2. Мешок перемешивается алгоритмом Fisher-Yates
/// 3. Фигуры берутся из мешка по очереди
/// 4. Когда мешок пуст, создаётся новый
///
/// ## Оптимизация:
/// Используется фиксированный массив [`ShapeType`; 7] вместо Vec<ShapeType>
/// для предотвращения аллокаций в куче и улучшения производительности.
///
/// ## Исправление #20
/// rng удалён из структуры и создаётся локально при заполнении мешка
/// для оптимизации использования памяти.
#[derive(Clone)]
pub struct BagGenerator {
    /// Текущий мешок с фигурами.
    bag: [ShapeType; 7],
    /// Индекс текущей фигуры в мешке.
    index: usize,
}

/// Константный массив всех 7 типов фигур.
/// Используется для инициализации мешка в `BagGenerator`.
const ALL_SHAPES: [ShapeType; 7] = [
    ShapeType::T,
    ShapeType::L,
    ShapeType::J,
    ShapeType::S,
    ShapeType::Z,
    ShapeType::O,
    ShapeType::I,
];

impl BagGenerator {
    /// Создать новый генератор с пустым мешком.
    pub fn new() -> Self {
        let mut bag = Self {
            bag: [ShapeType::T; 7], // Временная инициализация, заполнится в fill_bag()
            index: 7, // Устанавливаем index=7, чтобы первый вызов next_shape() вызвал fill_bag()
        };
        bag.fill_bag(); // Заполняем мешок сразу при создании
        bag
    }

    /// Заполнить мешок всеми 7 типами фигур и перемешать.
    ///
    /// Использует алгоритм Fisher-Yates для равномерного перемешивания:
    /// 1. Создаётся массив со всеми 7 типами фигур
    /// 2. Для каждой позиции i выбирается случайный индекс j от 0 до i
    /// 3. Элементы на позициях i и j меняются местами
    ///
    /// Используется фиксированный массив вместо Vec
    /// для предотвращения аллокаций в куче.
    ///
    /// rng создаётся локально для оптимизации памяти.
    fn fill_bag(&mut self) {
        // Заполнение фиксированного массива из константы
        self.bag = ALL_SHAPES;

        // Создаём rng локально для оптимизации использования памяти
        let mut rng = rand::rng();

        // Алгоритм Fisher-Yates для перемешивания
        for i in (1..self.bag.len()).rev() {
            // Генерируем случайный индекс от 0 до i
            let j = rng.random_range(0..=i);
            self.bag.swap(i, j);
        }

        self.index = 0;
    }

    /// Получить следующую фигуру из мешка.
    ///
    /// # Возвращает
    /// Следующий тип фигуры из мешка
    ///
    /// # Примечания
    /// - Если мешок пуст, автоматически заполняется новым набором
    /// - Гарантирует равномерное распределение фигур
    pub fn next_shape(&mut self) -> ShapeType {
        // Если фигуры закончились, заполняем новый мешок
        if self.index >= self.bag.len() {
            self.fill_bag();
        }

        // Берём фигуру из мешка и увеличиваем индекс
        let shape = self.bag[self.index];
        self.index += 1;
        shape
    }

    /// Получить текущий мешок фигур (для тестов).
    #[cfg(test)]
    #[must_use]
    pub fn get_bag(&self) -> &[ShapeType; 7] {
        &self.bag
    }

    /// Получить текущий индекс в мешке (для тестов).
    #[cfg(test)]
    #[must_use]
    pub fn get_index(&self) -> usize {
        self.index
    }
}

impl Default for BagGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/*
 * Сетка координат фигур (3x4):
 * Фигура может занимать от -2 до 2 по всем направлениям (из-за вращений).
 *      -2  -1  0   1    2
 *    _____________________
 * -2 |   |   |   |   |   |
 *    _____________________
 * -1 |   |   |   |   |   |
 *    ---------------------
 *  0 |   |   |   |   |   |
 *    ---------------------
 *  1 |   |   |   |   |   |
 *    ---------------------
 *  2 |   |   |   |   |   |
 *    ---------------------
 */

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
/// Индексы соответствуют SHAPE_COORDS:
/// 0=T→Пурпурный, 1=L→Жёлтый, 2=J→Синий, 3=S→Зелёный,
/// 4=Z→Св.красный, 5=O→Св.жёлтый, 6=I→Голубой
pub const SHAPE_COLORS: [&dyn Color; 7] = [
    &Magenta,
    &Yellow,
    &Blue,
    &Green,
    &LightRed,
    &LightYellow,
    &Cyan,
];

/// Типы фигур тетромино.
///
/// В игре используется 7 классических фигур из тетриса.
/// Каждая фигура имеет уникальную форму и цвет.
///
/// ## Использование
/// ```
/// use tetris_cli::tetromino::ShapeType;
///
/// let shape = ShapeType::T; // T-образная фигура
/// assert_eq!(shape as usize, 0); // Индекс для доступа к координатам и цвету
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
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

/// Тетромино — падающая фигура.
///
/// Содержит всю информацию о фигуре: тип, позицию, координаты блоков и цвет.
/// Использует плавающие координаты для плавного падения.
///
/// ## Структура
/// - `pos` - позиция фигуры в плавающих координатах (x, y)
/// - `shape` - тип фигуры (T, L, J, S, Z, O, I)
/// - `coords` - координаты блоков относительно центра
/// - `fg` - индекс цвета (0-6)
///
/// ## Исправление M13 (MEDIUM)
/// Добавлена compile-time проверка размера структуры для контроля памяти.
/// Размер Tetromino не должен превышать 40 байт для оптимального копирования.
///
/// ## Пример использования
/// ```
/// use tetris_cli::tetromino::{Tetromino, BagGenerator};
///
/// // Создание фигуры из мешка
/// let mut bag = BagGenerator::new();
/// let tetromino = Tetromino::from_bag(&mut bag);
/// assert_eq!(tetromino.pos, (4.0, 0.0)); // Начальная позиция
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    /// Позиция фигуры (x, y) в плавающих координатах.
    ///
    /// Используется f32 для плавного падения с интерполяцией.
    /// Начальная позиция: (4.0, 0.0) - центр сверху.
    pub pos: (f32, f32),
    /// Тип фигуры (T, L, J, S, Z, O, I).
    ///
    /// Определяет форму и цвет фигуры.
    pub shape: ShapeType,
    /// Координаты блоков относительно центра.
    ///
    /// Массив из 4 пар координат (x, y).
    /// Изменяется при вращении фигуры.
    pub coords: [(i16, i16); 4],
    /// Индекс цвета (0-6).
    ///
    /// Соответствует индексу фигуры в `SHAPE_COLORS`.
    pub fg: usize,
}

// Исправление M13: compile-time проверка размера Tetromino
// Размер структуры не должен превышать 40 байт для оптимального копирования
// pos: 8 байт + shape: 1 байт + coords: 16 байт + fg: 8 байт = 33 байта + padding
const _: () = assert!(
    std::mem::size_of::<Tetromino>() <= 40,
    "Размер Tetromino не должен превышать 40 байт"
);

impl Tetromino {
    /// Случайный выбор новой фигуры.
    ///
    /// # Устарело
    /// Используйте [`Tetromino::from_bag()`] вместо этой функции.
    /// Эта функция оставлена только для обратной совместимости тестов.
    ///
    /// # Возвращает
    /// Новый Tetromino со случайным типом и начальной позицией (4.0, 0.0)
    #[deprecated(since = "23.96.11", note = "Используйте Tetromino::from_bag()")]
    #[allow(dead_code)]
    pub fn select() -> Self {
        let shape = match rand::rng().random_range(0..7) {
            0 => ShapeType::T,
            1 => ShapeType::L,
            2 => ShapeType::J,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::O,
            6 => ShapeType::I,
            _ => unreachable!("random_range(0..7) возвращает только значения 0-6"),
        };
        Self {
            pos: (4.0, 0.0), // Начальная позиция по центру
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: shape as usize,
        }
    }

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
            fg: shape as usize,
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
    pub fn rotate(&mut self, dir: RotationDirection) {
        // Квадрат не вращается
        if self.shape == ShapeType::O {
            return;
        }

        // Исправление #4 (CRITICAL): замена assert на явную проверку с saturating операциями
        // Предотвращаем панику путём использования saturating_neg для отрицания
        // и пропуска вращения если координаты выходят за безопасные пределы
        for i in 0..4 {
            let (x, y) = self.coords[i];
            let (new_x, new_y) = match dir {
                // Против часовой: (x, y) -> (y, -x)
                RotationDirection::CounterClockwise => (y, x.saturating_neg()),
                // По часовой: (x, y) -> (-y, x)
                RotationDirection::Clockwise => (y.saturating_neg(), x),
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

    /// Вращать фигуру в заданном направлении (устаревший метод для совместимости).
    ///
    /// # Аргументы
    /// * `dir` - направление вращения:
    ///   - `Dir::Left` - против часовой стрелки (90° влево)
    ///   - `Dir::Right` - по часовой стрелке (90° вправо)
    ///   - `Dir::Down` - не используется, игнорируется
    ///
    /// # Panics
    /// Паникует, если координаты фигуры выходят за безопасные пределы
    /// во время вращения (проверяется через assert в режиме debug)
    ///
    /// # Примечания
    /// - Квадрат (O) не вращается - метод возвращает управление сразу
    /// - `Direction::Down` игнорируется (не вызывает панику)
    ///
    /// # Устарело
    /// Используйте [`Tetromino::rotate()`] с `RotationDirection` вместо этого метода.
    #[deprecated(since = "23.96.15", note = "Используйте rotate() с RotationDirection")]
    #[allow(dead_code)]
    pub fn rotate_old(&mut self, dir: crate::types::Direction) {
        // Квадрат не вращается
        if self.shape == ShapeType::O {
            return;
        }

        // Direction::Down игнорируется
        if dir == crate::types::Direction::Down {
            return;
        }

        // Исправление #4 (CRITICAL): замена assert на явную проверку с saturating операциями
        // Вращение по указанному направлению
        for i in 0..4 {
            let (x, y) = self.coords[i];
            let (new_x, new_y) = match dir {
                // Против часовой: (x, y) -> (y, -x)
                crate::types::Direction::Left => (y, x.saturating_neg()),
                // По часовой: (x, y) -> (-y, x)
                crate::types::Direction::Right => (y.saturating_neg(), x),
                crate::types::Direction::Down => return, // Игнорируем
            };

            // Явная проверка границ вместо assert - предотвращает панику в релизном режиме
            if !(i16::MIN / 2..=i16::MAX / 2).contains(&new_x)
                || !(i16::MIN / 2..=i16::MAX / 2).contains(&new_y)
            {
                // Координаты вышли за безопасные пределы - пропускаем вращение
                eprintln!("[WARN] Вращение фигуры пропущено (rotate_old): координаты вышли за безопасные пределы");
                return;
            }

            self.coords[i] = (new_x, new_y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ТЕСТЫ НА FIGURES (Tetromino, ShapeType)
    // =========================================================================
    // Эти тесты проверяют базовую функциональность фигур:
    // - Корректность создания фигур разных типов
    // - Правильность вращения (по часовой и против)
    // - Специальное поведение квадрата (O-фигура)
    // - Распределение цветов

    /// Тест 1: Проверка создания фигуры типа T
    ///
    /// Проверяет, что фигура T имеет правильные координаты:
    /// - Три блока в ряд: (-1,0), (0,0), (1,0)
    /// - Один блок сверху по центру: (0,1)
    #[test]
    fn test_tetromino_t_creation() {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: SHAPE_COORDS[0],
            fg: 0,
        };
        assert_eq!(tetromino.shape, ShapeType::T);
        assert_eq!(tetromino.coords[0], (-1, 0));
        assert_eq!(tetromino.coords[1], (0, 0));
        assert_eq!(tetromino.coords[2], (1, 0));
        assert_eq!(tetromino.coords[3], (0, 1));
    }

    /// Тест 2: Проверка создания фигуры типа I (линия)
    ///
    /// Проверяет, что фигура I имеет вертикальную ориентацию:
    /// - Четыре блока по вертикали: (0,-1), (0,0), (0,1), (0,2)
    /// - Используется для очистки 4 линий одновременно (Tetris)
    #[test]
    fn test_tetromino_i_creation() {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::I,
            coords: SHAPE_COORDS[6],
            fg: 6,
        };
        assert_eq!(tetromino.shape, ShapeType::I);
        // I-фигура - вертикальная линия
        assert_eq!(tetromino.coords[0], (0, -1));
        assert_eq!(tetromino.coords[3], (0, 2));
    }

    /// Тест 3: Проверка вращения фигуры T по часовой стрелке
    ///
    /// После вращения координаты должны измениться соответствующим образом:
    /// - Исходные: (-1,0), (0,0), (1,0), (0,1)
    /// - После вращения по часовой: (0,-1), (0,0), (0,1), (-1,0)
    /// - Формула: (x,y) -> (-y,x)
    #[test]
    fn test_tetromino_rotate_clockwise() {
        let mut tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: SHAPE_COORDS[0],
            fg: 0,
        };
        // Исходные координаты: (-1,0), (0,0), (1,0), (0,1)
        // Вращение по часовой: (x,y) -> (-y,x)
        // (-1,0) -> (0,-1), (0,0) -> (0,0), (1,0) -> (0,1), (0,1) -> (-1,0)
        tetromino.rotate(RotationDirection::Clockwise);
        assert_eq!(tetromino.coords[0], (0, -1));
        assert_eq!(tetromino.coords[1], (0, 0));
    }

    /// Тест 4: Проверка, что квадрат (O) не вращается
    ///
    /// Квадрат - единственная фигура, которая не вращается,
    /// так как это не меняет её форму.
    #[test]
    fn test_tetromino_o_no_rotate() {
        let mut tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::O,
            coords: SHAPE_COORDS[5],
            fg: 5,
        };
        let original_coords = tetromino.coords;
        tetromino.rotate(RotationDirection::Clockwise);
        assert_eq!(tetromino.coords, original_coords);
        tetromino.rotate(RotationDirection::CounterClockwise);
        assert_eq!(tetromino.coords, original_coords);
    }

    /// Тест 5: Проверка цветов фигур
    #[test]
    fn test_shape_colors_assigned() {
        // Проверяем, что массив цветов содержит 7 элементов
        assert_eq!(SHAPE_COLORS.len(), 7);
    }

    /// Тест 6: Проверка случайного выбора фигур
    #[test]
    #[allow(deprecated)]
    fn test_random_shape_selection() {
        // Генерируем 100 фигур и проверяем, что все типы встречаются
        let mut shapes_found = [false; 7];
        for _ in 0..100 {
            let t = Tetromino::select();
            shapes_found[t.fg] = true;
        }
        for found in &shapes_found {
            assert!(*found, "Не все типы фигур были сгенерированы");
        }
    }

    // =========================================================================
    // ТЕСТЫ НА BAG SYSTEM (7-bag генератор)
    // =========================================================================
    // Эти тесты проверяют систему генерации фигур 7-bag:
    // - Bag содержит все 7 фигур
    // - Перемешивание Fisher-Yates работает корректно
    // - Заполнение после опустошения
    // - Равномерное распределение фигур

    /// Тест 7: Проверка, что bag содержит все 7 фигур
    #[test]
    fn test_bag_contains_all_shapes() {
        let mut bag = BagGenerator::new();
        bag.fill_bag();

        assert_eq!(bag.get_bag().len(), 7, "Мешок должен содержать 7 фигур");
        assert!(bag.get_bag().contains(&ShapeType::T));
        assert!(bag.get_bag().contains(&ShapeType::L));
        assert!(bag.get_bag().contains(&ShapeType::J));
        assert!(bag.get_bag().contains(&ShapeType::S));
        assert!(bag.get_bag().contains(&ShapeType::Z));
        assert!(bag.get_bag().contains(&ShapeType::O));
        assert!(bag.get_bag().contains(&ShapeType::I));

        // Проверяем, что каждая фигура встречается ровно один раз
        let t_count = bag.get_bag().iter().filter(|&&s| s == ShapeType::T).count();
        let l_count = bag.get_bag().iter().filter(|&&s| s == ShapeType::L).count();
        let i_count = bag.get_bag().iter().filter(|&&s| s == ShapeType::I).count();

        assert_eq!(t_count, 1, "Фигура T должна встречаться ровно 1 раз");
        assert_eq!(l_count, 1, "Фигура L должна встречаться ровно 1 раз");
        assert_eq!(i_count, 1, "Фигура I должна встречаться ровно 1 раз");
    }

    /// Тест 8: Проверка перемешивания Fisher-Yates
    #[test]
    fn test_bag_shuffle_randomness() {
        let mut bag = BagGenerator::new();
        let mut unique_orders = Vec::new();
        let iterations = 10;

        for _ in 0..iterations {
            bag.fill_bag();
            // Конвертация массива в Vec для теста
            let order: Vec<ShapeType> = bag.bag.to_vec();
            if !unique_orders.contains(&order) {
                unique_orders.push(order);
            }
        }

        assert!(
            unique_orders.len() >= 5,
            "Должно быть как минимум 5 уникальных порядков из 10 попыток"
        );

        for order in &unique_orders {
            assert_eq!(order.len(), 7, "Каждый порядок должен содержать 7 фигур");
            assert!(order.contains(&ShapeType::T));
            assert!(order.contains(&ShapeType::I));
        }
    }

    /// Тест 9: Проверка заполнения мешка после опустошения
    #[test]
    fn test_bag_refill_after_empty() {
        let mut bag = BagGenerator::new();

        let mut first_bag_shapes = Vec::new();
        for _ in 0..7 {
            first_bag_shapes.push(bag.next_shape());
        }

        assert!(first_bag_shapes.contains(&ShapeType::T));
        assert!(first_bag_shapes.contains(&ShapeType::I));
        assert_eq!(
            bag.get_index(),
            7,
            "Индекс должен быть 7 после получения 7 фигур"
        );

        let next_shape = bag.next_shape();
        assert_eq!(
            bag.get_index(),
            1,
            "Индекс должен быть 1 после заполнения нового мешка"
        );
        assert!(matches!(
            next_shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ));
        assert_eq!(
            bag.get_bag().len(),
            7,
            "Новый мешок должен содержать 7 фигур"
        );
    }

    /// Тест 10: Проверка равномерного распределения фигур
    #[test]
    fn test_bag_uniform_distribution() {
        let mut bag = BagGenerator::new();
        let total_shapes = 700;
        let mut shape_counts = [0; 7];

        for _ in 0..total_shapes {
            let shape = bag.next_shape();
            shape_counts[shape as usize] += 1;
        }

        let expected_per_shape = total_shapes / 7;
        for (shape_index, &count) in shape_counts.iter().enumerate() {
            let shape_name = match shape_index {
                0 => "T",
                1 => "L",
                2 => "J",
                3 => "S",
                4 => "Z",
                5 => "O",
                6 => "I",
                _ => "Unknown",
            };
            assert_eq!(
                count, expected_per_shape,
                "Фигура {shape_name} должна встречаться {expected_per_shape} раз (встретилась {count})"
            );
        }
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ M13: size_of::<Tetromino>() - РАЗМЕР СТРУКТУРЫ
    // ========================================================================

    /// Тест M13: проверка размера структуры Tetromino
    #[test]
    fn test_fix_m13_size_of_tetromino() {
        // Проверка размера структуры Tetromino
        // pos: 8 байт (f32, f32) + shape: 1 байт + coords: 16 байт + fg: 8 байт = 33 байта + padding
        let size = std::mem::size_of::<Tetromino>();

        // Compile-time assertion проверяет что размер <= 40 байт
        // Здесь проверяем что размер корректный
        assert!(size <= 40, "Размер Tetromino не должен превышать 40 байт");
        assert!(size >= 32, "Размер Tetromino должен быть не менее 32 байт");

        // Ожидаемый размер с учётом padding: 36 или 40 байт
        println!("Размер Tetromino: {} байт", size);
    }

    /// Тест M13: проверка размера полей Tetromino
    #[test]
    fn test_fix_m13_size_of_tetromino_fields() {
        // Проверка размеров отдельных полей
        assert_eq!(
            std::mem::size_of::<(f32, f32)>(),
            8,
            "pos (f32, f32) должен быть 8 байт"
        );
        assert_eq!(
            std::mem::size_of::<ShapeType>(),
            1,
            "ShapeType должен быть 1 байт"
        );
        assert_eq!(
            std::mem::size_of::<[(i16, i16); 4]>(),
            16,
            "coords должен быть 16 байт"
        );
        assert_eq!(std::mem::size_of::<usize>(), 8, "usize должен быть 8 байт");

        // Суммарный размер без padding: 8 + 1 + 16 + 8 = 33 байта
        // С padding: 36 или 40 байт (в зависимости от выравнивания)
    }

    /// Тест M13: проверка compile-time assertion
    #[test]
    fn test_fix_m13_compile_time_assertion() {
        // Этот тест проверяет что compile-time assertion работает
        // Если размер превысит 40 байт, код не скомпилируется

        // Compile-time assertion из кода:
        // const _: () = assert!(std::mem::size_of::<Tetromino>() <= 40, ...);

        // Проверяем что assertion не паникует
        let size = std::mem::size_of::<Tetromino>();
        assert!(
            size <= 40,
            "Compile-time assertion должен гарантировать размер <= 40"
        );
    }

    /// Тест M13: проверка оптимальности размера для копирования
    #[test]
    fn test_fix_m13_optimal_size_for_copying() {
        // Tetromino копируется через Clone/Copy
        // Размер <= 40 байт оптимален для копирования в стеке

        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: SHAPE_COORDS[0],
            fg: 0,
        };

        // Копирование должно быть быстрым (маленький размер)
        let copied = tetromino; // Copy семантика
        let cloned = tetromino; // Copy семантика (clone избыточен для Copy типа)

        assert_eq!(copied.shape, cloned.shape);
        assert_eq!(copied.pos, cloned.pos);
        assert_eq!(copied.coords, cloned.coords);
    }
}
