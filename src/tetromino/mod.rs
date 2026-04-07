//! Фигуры тетромино.
//!
//! Этот модуль определяет все 7 типов тетрамино (T, L, J, S, Z, O, I),
//! их координаты, цвета и поведение при вращении.
//!
//! ## Структура модуля
//! - `constants` - константы `SHAPE_COORDS`, `SHAPE_COLORS`
//! - `shape_type` - enum `ShapeType`
//! - `bag_generator` - struct `BagGenerator`
//! - `tetromino_struct` - struct `Tetromino`
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
//! | I | Линия | Голубой | Четыре блока в вертикальный ряд

// Подмодули
pub mod bag_generator;
pub mod constants;
pub mod shape_type;
pub mod tetromino_struct;

// Ре-экспорт основных типов для удобства импорта
pub use bag_generator::BagGenerator;
pub use constants::{SHAPE_COLORS, SHAPE_COORDS, SHAPE_COUNT};
pub use shape_type::ShapeType;
pub use tetromino_struct::Tetromino;
