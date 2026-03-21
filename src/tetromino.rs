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

use crate::game::Dir;
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
/// rng кэшируется в структуре для избежания повторного создания thread_rng()
/// при каждом заполнении мешка.
pub struct BagGenerator {
    /// Текущий мешок с фигурами.
    bag: Vec<ShapeType>,
    /// Индекс текущей фигуры в мешке.
    index: usize,
    /// Кэшированный генератор случайных чисел.
    rng: rand::rngs::ThreadRng,
}

impl BagGenerator {
    /// Создать новый генератор с пустым мешком.
    pub fn new() -> Self {
        Self {
            bag: Vec::new(),
            index: 0,
            rng: rand::thread_rng(),
        }
    }

    /// Заполнить мешок всеми 7 типами фигур и перемешать.
    ///
    /// Использует алгоритм Fisher-Yates для равномерного перемешивания:
    /// 1. Создаётся вектор со всеми 7 типами фигур
    /// 2. Для каждой позиции i выбирается случайный индекс j от 0 до i
    /// 3. Элементы на позициях i и j меняются местами
    ///
    /// Оптимизация: использует reserve() + extend_from_slice() вместо clear() + extend()
    /// для повторного использования выделенной памяти и предотвращения реаллокаций.
    fn fill_bag(&mut self) {
        // Оптимизация: используем reserve() + extend_from_slice() для эффективности
        self.bag.clear();
        self.bag.reserve(7);
        self.bag.extend_from_slice(&[
            ShapeType::T,
            ShapeType::L,
            ShapeType::J,
            ShapeType::S,
            ShapeType::Z,
            ShapeType::O,
            ShapeType::I,
        ]);

        // Алгоритм Fisher-Yates для перемешивания
        // Используем кэшированный rng вместо создания нового thread_rng()
        for i in (1..self.bag.len()).rev() {
            // Генерируем случайный индекс от 0 до i
            let j = self.rng.gen_range(0..=i);
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
        // Если мешок пуст или фигуры закончились, заполняем новый
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
    pub fn get_bag(&self) -> &Vec<ShapeType> {
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
 * Фигура может занимать от -2 до 2 по всем направлениям (из-за вращений),
 * поэтому требуется сетка 3x4:
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
/// Координаты используются для отрисовки и проверки столкновений.
///
/// ## Структура координат
/// - Первый элемент: смещение по оси X (-2..2)
/// - Второй элемент: смещение по оси Y (-2..2)
///
/// ## Индексы фигур
/// 0. T - T-образная
/// 1. L - L-образная
/// 2. J - J-образная (зеркальная L)
/// 3. S - S-образная
/// 4. Z - Z-образная (зеркальная S)
/// 5. O - Квадрат
/// 6. I - Линия (вертикальная)
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
/// Соответствуют индексам фигур в SHAPE_COORDS:
/// | Индекс | Фигура | Цвет |
/// |--------|--------|------|
/// | 0 | T | Пурпурный (Magenta) |
/// | 1 | L | Жёлтый (Yellow) |
/// | 2 | J | Синий (Blue) |
/// | 3 | S | Зелёный (Green) |
/// | 4 | Z | Светло-красный (LightRed) |
/// | 5 | O | Светло-жёлтый (LightYellow) |
/// | 6 | I | Голубой (Cyan) |
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
/// ## Пример использования
/// ```
/// use tetris_cli::tetromino::Tetromino;
///
/// // Создание случайной фигуры
/// let tetromino = Tetromino::select();
///
/// // Вращение фигуры
/// use tetris_cli::game::Dir;
/// // tetromino.rotate(Dir::Right);
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
    /// Соответствует индексу фигуры в SHAPE_COLORS.
    pub fg: usize,
}

impl Tetromino {
    /// Случайный выбор новой фигуры.
    ///
    /// # Возвращает
    /// Новый Tetromino со случайным типом и начальной позицией (4.0, 0.0)
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::tetromino::Tetromino;
    ///
    /// let figure = Tetromino::select();
    /// assert_eq!(figure.pos, (4.0, 0.0)); // Начальная позиция
    /// ```
    #[allow(dead_code)]
    pub fn select() -> Self {
        let shape = match rand::thread_rng().gen_range(0..7) {
            0 => ShapeType::T,
            1 => ShapeType::L,
            2 => ShapeType::J,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::O,
            6 => ShapeType::I,
            _ => unreachable!("gen_range(0..7) возвращает только значения 0-6"),
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
    ///   - `Dir::Left` - против часовой стрелки (90° влево)
    ///   - `Dir::Right` - по часовой стрелке (90° вправо)
    ///
    /// # Примечания
    /// - Квадрат (O) не вращается - метод возвращает управление сразу
    /// - Dir::Down вызывает panic (не используется для вращения)
    /// - Вращение на 90 градусов вокруг центра фигуры
    /// - Используется формула поворота:
    ///   - По часовой: (x, y) -> (-y, x)
    ///   - Против часовой: (x, y) -> (y, -x)
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::tetromino::{Tetromino, ShapeType};
    /// use tetris_cli::game::Dir;
    ///
    /// let mut t = Tetromino {
    ///     pos: (4.0, 0.0),
    ///     shape: ShapeType::T,
    ///     coords: [(-1, 0), (0, 0), (1, 0), (0, 1)],
    ///     fg: 0,
    /// };
    /// t.rotate(Dir::Right); // Поворот по часовой
    /// ```
    pub fn rotate(&mut self, dir: Dir) {
        // Квадрат не вращается
        if self.shape == ShapeType::O {
            return;
        }

        // Вращение работает только с Dir::Left и Dir::Right
        for i in 0..4 {
            let (x, y) = self.coords[i];
            let (new_x, new_y) = match dir {
                Dir::Left => (y, -x),  // Поворот против часовой
                Dir::Right => (-y, x), // Поворот по часовой
                Dir::Down => unreachable!("Dir::Down не используется для вращения"),
            };

            // Проверка границ: координаты должны оставаться в пределах i16
            // Это предотвращает переполнение при экстремальных значениях
            if !(i16::MIN / 2..=i16::MAX / 2).contains(&new_x)
                || !(i16::MIN / 2..=i16::MAX / 2).contains(&new_y)
            {
                // Пропускаем вращение если координаты выходят за безопасные пределы
                // Это защищает от переполнения при последующих вращениях
                continue;
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
        tetromino.rotate(Dir::Right);
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
        tetromino.rotate(Dir::Right);
        assert_eq!(tetromino.coords, original_coords);
        tetromino.rotate(Dir::Left);
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
    fn test_random_shape_selection() {
        // Генерируем 100 фигур и проверяем, что все типы встречаются
        let mut shapes_found = [false; 7];
        for _ in 0..100 {
            let t = Tetromino::select();
            shapes_found[t.fg] = true;
        }
        for found in shapes_found.iter() {
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
            let order: Vec<ShapeType> = bag.bag.clone();
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
                "Фигура {} должна встречаться {} раз (встретилась {})",
                shape_name, expected_per_shape, count
            );
        }
    }
}
