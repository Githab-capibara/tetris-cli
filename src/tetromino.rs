//! Фигуры тетромино.
//!
//! Автор: Dylan Turner

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
#[derive(Copy, Clone, PartialEq)]
pub enum ShapeType {
    /// T-образная.
    T,
    /// L-образная.
    L,
    /// J-образная (зеркальная L).
    J,
    /// S-образная.
    S,
    /// Z-образная.
    Z,
    /// Квадрат.
    O,
    /// Линия.
    I,
}

/// Распределение для случайного выбора фигуры.
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
#[derive(Clone, Copy)]
pub struct Tetromino {
    /// Позиция фигуры (x, y) в плавающих координатах.
    pub pos: (f32, f32),
    /// Тип фигуры.
    pub shape: ShapeType,
    /// Координаты блоков относительно центра.
    pub coords: [(i16, i16); 4],
    /// Индекс цвета (0-6).
    pub fg: usize,
}

impl Tetromino {
    /// Случайный выбор новой фигуры.
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
