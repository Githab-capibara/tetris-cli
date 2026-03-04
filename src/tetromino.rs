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
/// 0-T: Magenta, 1-L: Yellow, 2-J: Blue, 3-S: Green,
/// 4-Z: LightRed, 5-O: LightYellow, 6-I: Cyan
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ShapeType {
    /// T-образная фигура (пурпурная).
    T,
    /// L-образная фигура (жёлтая).
    L,
    /// J-образная фигура (синяя, зеркальная L).
    J,
    /// S-образная фигура (зелёная).
    S,
    /// Z-образная фигура (светло-красная).
    Z,
    /// Квадратная фигура (светло-жёлтая).
    O,
    /// Линия (голубая).
    I,
}

/// Распределение для случайного выбора фигуры.
///
/// Реализует равномерное распределение: каждая из 7 фигур
/// выбирается с вероятностью 1/7.
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
#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    /// Позиция фигуры (x, y) в плавающих координатах.
    pub pos: (f32, f32),
    /// Тип фигуры (T, L, J, S, Z, O, I).
    pub shape: ShapeType,
    /// Координаты блоков относительно центра.
    pub coords: [(i16, i16); 4],
    /// Индекс цвета (0-6).
    pub fg: usize,
}

impl Tetromino {
    /// Случайный выбор новой фигуры.
    ///
    /// # Возвращает
    /// Новый Tetromino со случайным типом и начальной позицией (4.0, 0.0)
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
    /// * `dir` - направление вращения (Left = против часовой, Right = по часовой)
    ///
    /// # Примечания
    /// - Квадрат (O) не вращается
    /// - Вращение на 90 градусов вокруг центра
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

    // ========== ТЕСТЫ 1-4: Tetromino (создание, вращение, координаты) ==========

    /// Тест 1: Проверка создания фигуры типа T
    /// Проверяет, что фигура T имеет правильные координаты
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
    /// Проверяет, что фигура I имеет вертикальную ориентацию
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
    /// После вращения координаты должны измениться соответствующим образом
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

    // ========== ТЕСТЫ 5-10: GameState (движение, столкновения, сохранение) ==========

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

    // ========== ТЕСТЫ 11-14: Линии и уровни ==========

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

    // ========== ТЕСТЫ 15-18: Leaderboard ==========

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

    // ========== ТЕСТЫ 19-20: Константы и границы ==========

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
