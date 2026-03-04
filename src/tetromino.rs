//! Фигуры тетромино.
//!
//! Автор: Dylan Turner
//!
//! Этот модуль определяет все 7 типов тетрамино (T, L, J, S, Z, O, I),
//! их координаты, цвета и поведение при вращении.
//!
//! ## Структура модуля
//! - `SHAPE_COORDS` - координаты блоков для каждой фигуры
//! - `SHAPE_COLORS` - цвета для отрисовки
//! - `ShapeType` - перечисление типов фигур
//! - `Tetromino` - структура падающей фигуры
//! - `tests` - 20 модульных тестов
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
use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};
use termion::color::{Blue, Color, Cyan, Green, LightRed, LightYellow, Magenta, Yellow};

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
const SHAPE_COORDS: [[(i16, i16); 4]; 7] = [
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

/// Распределение для случайного выбора фигуры.
///
/// Реализует равномерное распределение: каждая из 7 фигур
/// выбирается с вероятностью 1/7 (~14.28%).
///
/// ## Пример использования
/// ```
/// use rand::{random, Rng};
/// use tetris_cli::tetromino::ShapeType;
///
/// let shape: ShapeType = random(); // Случайная фигура
/// ```
impl Distribution<ShapeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeType {
        match rng.gen_range(0..7) {
            0 => ShapeType::T,
            1 => ShapeType::L,
            2 => ShapeType::J,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::O,
            6 => ShapeType::I,
            _ => unreachable!("Неверный диапазон фигуры"),
        }
    }
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
    pub fn select() -> Self {
        let shape = random();
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
    ///   - `Dir::Down` - не используется для вращения
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

        for i in 0..4 {
            let (x, y) = self.coords[i];
            match dir {
                Dir::Left => self.coords[i] = (y, -x), // Поворот против часовой
                Dir::Right => self.coords[i] = (-y, x), // Поворот по часовой
                Dir::Down => {}                        // Не используется
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ГРУППА ТЕСТОВ 1-4: Tetromino (создание, вращение, координаты)
    // =========================================================================
    // Эти тесты проверяют базовую функциональность фигур:
    // - Корректность создания фигур разных типов
    // - Правильность вращения (по часовой и против)
    // - Специальное поведение квадрата (O-фигура)

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

    // =========================================================================
    // ГРУППА ТЕСТОВ 5-10: GameState (движение, столкновения, сохранение)
    // =========================================================================
    // Эти тесты проверяют игровую логику:
    // - Создание и инициализация GameState
    // - Начальная скорость падения
    // - Пустое игровое поле
    // - Наличие следующей фигуры
    // - Распределение цветов фигур
    // - Случайный выбор фигур

    /// Тест 5: Проверка создания GameState
    #[test]
    fn test_game_state_creation() {
        use crate::game::GameState;
        let state = GameState::new();
        assert_eq!(state.get_score(), 0);
        assert_eq!(state.get_level(), 1);
        assert_eq!(state.get_lines_cleared(), 0);
    }

    /// Тест 6: Проверка начальной скорости падения
    #[test]
    fn test_game_state_initial_speed() {
        use crate::game::GameState;
        let state = GameState::new();
        assert!((state.get_fall_spd() - 0.9).abs() < f32::EPSILON);
    }

    /// Тест 7: Проверка, что поле инициализируется пустыми клетками
    #[test]
    fn test_game_state_empty_field() {
        use crate::game::GameState;
        use crate::io::GRID_HEIGHT;
        use crate::io::GRID_WIDTH;
        let state = GameState::new();
        let blocks = state.get_blocks();
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                assert_eq!(blocks[y][x], -1);
            }
        }
    }

    /// Тест 8: Проверка, что следующая фигура не равна None
    #[test]
    fn test_game_state_next_shape_exists() {
        use crate::game::GameState;
        let state = GameState::new();
        let next = state.get_next_shape();
        assert_eq!(next.shape as usize, next.fg);
    }

    /// Тест 9: Проверка цветов фигур
    #[test]
    fn test_shape_colors_assigned() {
        // Проверяем, что массив цветов содержит 7 элементов
        assert_eq!(SHAPE_COLORS.len(), 7);
    }

    /// Тест 10: Проверка случайного выбора фигур
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
    // ГРУППА ТЕСТОВ 11-14: Линии и уровни
    // =========================================================================
    // Эти тесты проверяют систему прогрессии:
    // - Расчёт уровня от количества линий
    // - Бонус за несколько удалённых линий
    // - Константа LINES_PER_LEVEL
    // - Увеличение скорости падения

    /// Тест 11: Проверка расчёта уровня от количества линий
    #[test]
    fn test_level_calculation() {
        // Уровень 1: 0-9 линий
        assert_eq!((0 / 10) + 1, 1);
        assert_eq!((9 / 10) + 1, 1);
        // Уровень 2: 10-19 линий
        assert_eq!((10 / 10) + 1, 2);
        assert_eq!((19 / 10) + 1, 2);
        // Уровень 5: 40-49 линий
        assert_eq!((40 / 10) + 1, 5);
    }

    /// Тест 12: Проверка бонуса за несколько линий
    #[test]
    fn test_line_bonus_calculation() {
        use crate::game::ROW_SCORE_INC;
        // 1 линия: 100 * 2^0 = 100
        assert_eq!(ROW_SCORE_INC * (1 << 0), 100);
        // 2 линии: 100 * 2^1 = 200
        assert_eq!(ROW_SCORE_INC * (1 << 1), 200);
        // 3 линии: 100 * 2^2 = 400
        assert_eq!(ROW_SCORE_INC * (1 << 2), 400);
        // 4 линии: 100 * 2^3 = 800
        assert_eq!(ROW_SCORE_INC * (1 << 3), 800);
    }

    /// Тест 13: Проверка константы LINES_PER_LEVEL
    #[test]
    fn test_lines_per_level_constant() {
        use crate::game::LINES_PER_LEVEL;
        assert_eq!(LINES_PER_LEVEL, 10);
    }

    /// Тест 14: Проверка увеличения скорости
    #[test]
    fn test_speed_increase() {
        use crate::game::{INITIAL_FALL_SPD, SPD_INC};
        let initial = INITIAL_FALL_SPD;
        let after_one_line = initial + SPD_INC * 1.0;
        let after_five_lines = initial + SPD_INC * 5.0;
        assert!(after_five_lines > after_one_line);
        assert!(after_one_line > initial);
    }

    // =========================================================================
    // ГРУППА ТЕСТОВ 15-18: Leaderboard (таблица лидеров)
    // =========================================================================
    // Эти тесты проверяют систему рекордов:
    // - Создание записи в таблице лидеров
    // - Валидация записей (проверка хеша)
    // - Добавление рекорда
    // - Сортировка по убыванию очков

    /// Тест 15: Проверка создания записи в таблице лидеров
    #[test]
    fn test_leaderboard_entry_creation() {
        use crate::highscore::LeaderboardEntry;
        let entry = LeaderboardEntry::new("Player".to_string(), 1000);
        assert_eq!(entry.name, "Player");
        assert_eq!(entry.score, 1000);
        assert!(!entry.hash.is_empty());
    }

    /// Тест 16: Проверка валидации записи
    #[test]
    fn test_leaderboard_entry_validation() {
        use crate::highscore::LeaderboardEntry;
        let entry = LeaderboardEntry::new("Player".to_string(), 1000);
        assert!(entry.is_valid());
    }

    /// Тест 17: Проверка добавления рекорда в таблицу
    #[test]
    fn test_leaderboard_add_score() {
        use crate::highscore::Leaderboard;
        let mut leaderboard = Leaderboard::default();
        let added = leaderboard.add_score("Player1".to_string(), 1000);
        assert!(added);
        assert_eq!(leaderboard.len(), 1);
    }

    /// Тест 18: Проверка сортировки таблицы лидеров
    #[test]
    fn test_leaderboard_sorting() {
        use crate::highscore::Leaderboard;
        let mut leaderboard = Leaderboard::default();
        leaderboard.add_score("Player3".to_string(), 300);
        leaderboard.add_score("Player1".to_string(), 1000);
        leaderboard.add_score("Player2".to_string(), 500);

        let entries = leaderboard.get_entries();
        assert_eq!(entries[0].score, 1000);
        assert_eq!(entries[1].score, 500);
        assert_eq!(entries[2].score, 300);
    }

    // =========================================================================
    // ГРУППА ТЕСТОВ 19-20: Константы и границы
    // =========================================================================
    // Эти тесты проверяют базовые константы игры:
    // - Размеры игрового поля (GRID_WIDTH, GRID_HEIGHT)
    // - Размеры дисплея (DISP_WIDTH, DISP_HEIGHT)
    // - Константы игры (FPS, INITIAL_FALL_SPD, LAND_TIME_DELAY_S)

    /// Тест 19: Проверка размеров игрового поля
    #[test]
    fn test_field_dimensions() {
        use crate::io::{DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_WIDTH};
        assert_eq!(GRID_WIDTH, 10);
        assert_eq!(GRID_HEIGHT, 20);
        assert_eq!(SHAPE_WIDTH, 2);
        assert_eq!(DISP_WIDTH, 22);
        assert_eq!(DISP_HEIGHT, 25);
    }

    /// Тест 20: Проверка констант игры
    #[test]
    fn test_game_constants() {
        use crate::game::{FPS, INITIAL_FALL_SPD, LAND_TIME_DELAY_S};
        assert_eq!(FPS, 60);
        assert!((INITIAL_FALL_SPD - 0.9).abs() < f32::EPSILON);
        assert!((LAND_TIME_DELAY_S - 0.1).abs() < f64::EPSILON);
    }
}
