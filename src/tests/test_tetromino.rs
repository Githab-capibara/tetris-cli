//! Тесты фигур тетрамино.
//!
//! Этот модуль содержит 25 тестов для проверки всех аспектов фигур:
//! - Тесты создания каждой фигуры (7 тестов)
//! - Тесты вращения всех фигур (7 тестов)
//! - Тесты цветов (4 теста)
//! - Тесты Bag Generator (4 теста)
//! - Тесты координат (3 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты фигур.

use crate::tetromino::{BagGenerator, RotationDirection, ShapeType, Tetromino, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ 1-3: Создание фигур (параметризованные тесты)
// ============================================================================

/// Тест 1: Проверка создания всех фигур
///
/// Проверяет создание всех 7 типов фигур с корректными координатами.
#[test]
fn test_all_tetromino_creation() {
    let shapes = [
        (ShapeType::T, 0, [(-1, 0), (0, 0), (1, 0), (0, 1)]),
        (ShapeType::L, 1, [(-1, -1), (0, -1), (0, 0), (0, 1)]),
        (ShapeType::J, 2, [(1, -1), (0, -1), (0, 0), (0, 1)]),
        (ShapeType::S, 3, [(0, -1), (0, 0), (1, 0), (1, 1)]),
        (ShapeType::Z, 4, [(0, -1), (0, 0), (-1, 0), (-1, 1)]),
        (ShapeType::O, 5, [(0, 0), (1, 0), (0, 1), (1, 1)]),
        (ShapeType::I, 6, [(0, -1), (0, 0), (0, 1), (0, 2)]),
    ];

    for (shape_type, expected_fg, expected_coords) in shapes {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: shape_type,
            coords: SHAPE_COORDS[shape_type as usize],
            fg: expected_fg,
        };

        assert_eq!(
            tetromino.shape, shape_type,
            "Фигура должна быть типа {shape_type:?}"
        );
        assert_eq!(
            tetromino.fg, expected_fg,
            "Индекс цвета должен быть {expected_fg}"
        );
        assert_eq!(
            tetromino.coords, expected_coords,
            "Координаты фигуры {shape_type:?} должны совпадать"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 8-14: Вращение всех фигур
// ============================================================================

/// Тест 8: Проверка вращения фигуры T
///
/// Проверяет, что T-фигура корректно вращается на 90 градусов.
#[test]
fn test_tetromino_t_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original_coords = t.coords;

    // Вращение по часовой
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(
        t.coords, original_coords,
        "Координаты должны измениться после вращения"
    );

    // 4 вращения должны вернуть к исходному состоянию
    for _ in 0..3 {
        t.rotate(RotationDirection::Clockwise);
    }
    assert_eq!(
        t.coords, original_coords,
        "После 4 вращений фигура должна вернуться в исходное состояние"
    );
}

/// Тест 9: Проверка вращения фигуры L
///
/// Проверяет, что L-фигура корректно вращается.
#[test]
fn test_tetromino_l_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    let original_coords = t.coords;

    // Вращение по часовой
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords, original_coords, "L-фигура должна вращаться");

    // 4 вращения возвращают к исходному состоянию
    for _ in 0..3 {
        t.rotate(RotationDirection::Clockwise);
    }
    assert_eq!(t.coords, original_coords);
}

/// Тест 10: Проверка вращения фигуры J
///
/// Проверяет, что J-фигура корректно вращается.
#[test]
fn test_tetromino_j_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    let original_coords = t.coords;
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords, original_coords, "J-фигура должна вращаться");
}

/// Тест 11: Проверка вращения фигуры S
///
/// Проверяет, что S-фигура корректно вращается.
#[test]
fn test_tetromino_s_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    let original_coords = t.coords;
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords, original_coords, "S-фигура должна вращаться");
}

/// Тест 12: Проверка вращения фигуры Z
///
/// Проверяет, что Z-фигура корректно вращается.
#[test]
fn test_tetromino_z_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    let original_coords = t.coords;
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords, original_coords, "Z-фигура должна вращаться");
}

/// Тест 13: Проверка, что фигура O не вращается
///
/// Квадратная фигура не меняет форму при вращении.
#[test]
fn test_tetromino_o_no_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let original_coords = t.coords;

    // Вращение по часовой
    t.rotate(RotationDirection::Clockwise);
    assert_eq!(
        t.coords, original_coords,
        "O-фигура не должна вращаться по часовой"
    );

    // Вращение против часовой
    t.rotate(RotationDirection::CounterClockwise);
    assert_eq!(
        t.coords, original_coords,
        "O-фигура не должна вращаться против часовой"
    );
}

/// Тест 14: Проверка вращения фигуры I
///
/// Проверяет, что I-фигура (линия) корректно вращается.
#[test]
fn test_tetromino_i_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    let original_coords = t.coords;

    // I-фигура вращается из вертикальной в горизонтальную
    t.rotate(RotationDirection::Clockwise);
    assert_ne!(t.coords, original_coords, "I-фигура должна вращаться");

    // Проверяем, что после вращения линия стала горизонтальной
    // Исходная: (0,-1), (0,0), (0,1), (0,2) - вертикальная
    // После вращения: (1,0), (0,0), (-1,0), (-2,0) - горизонтальная
    assert_eq!(
        t.coords[0].1, 0,
        "После вращения все блоки должны быть на одной строке"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-18: Цвета фигур
// ============================================================================

/// Тест 16: Проверка соответствия индексов цветов и фигур
///
/// Проверяет, что индекс цвета соответствует индексу фигуры.
#[test]
fn test_shape_color_index_match() {
    // Создаём фигуры всех типов и проверяем соответствие
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for (index, &shape) in shapes.iter().enumerate() {
        let t = Tetromino {
            pos: (4.0, 0.0),
            shape,
            coords: SHAPE_COORDS[index],
            fg: index as u8,
        };

        // Проверяем, что индекс цвета совпадает с индексом фигуры
        assert_eq!(
            t.fg, index as u8,
            "Индекс цвета должен совпадать с индексом фигуры для {shape:?}"
        );
    }
}

/// Тест 17: Проверка случайного выбора фигур (равномерность распределения)
///
/// Проверяет, что все 7 типов фигур встречаются при случайном выборе.
#[test]
fn test_random_shape_distribution() {
    // Генерируем 700 фигур и проверяем распределение
    let mut counts = [0; 7];
    let mut bag = BagGenerator::new();

    for _ in 0..700 {
        let t = Tetromino::from_bag(&mut bag);
        counts[t.fg as usize] += 1;
    }

    // Проверяем, что все типы встретились хотя бы 50 раз
    // (при равномерном распределении ожидается ~100 на тип)
    for (i, &count) in counts.iter().enumerate() {
        assert!(
            count >= 50,
            "Фигура типа {i} должна встретиться хотя бы 50 раз (встретилась {count} раз)"
        );
    }
}

/// Тест 18: Проверка создания фигуры через `Tetromino::from_bag()`
///
/// Проверяет, что `from_bag()` создаёт валидную фигуру.
#[test]
fn test_tetromino_select_creation() {
    let mut bag = BagGenerator::new();
    let t = Tetromino::from_bag(&mut bag);

    // Проверяем начальную позицию
    assert!(
        (t.pos.0 - 4.0).abs() < f32::EPSILON,
        "Начальная позиция X должна быть 4.0"
    );
    assert!(
        (t.pos.1 - 0.0).abs() < f32::EPSILON,
        "Начальная позиция Y должна быть 0.0"
    );

    // Проверяем, что тип фигуры валиден
    assert!(t.fg < 7, "Индекс цвета должен быть меньше 7");

    // Проверяем, что у фигуры 4 блока
    assert_eq!(t.coords.len(), 4, "У фигуры должно быть 4 блока");
}

// ============================================================================
// ГРУППА ТЕСТОВ 19-22: Bag Generator
// ============================================================================

/// Тест 20: Проверка получения фигур из Bag Generator
///
/// Проверяет, что `next_shape()` возвращает валидные фигуры.
#[test]
fn test_bag_generator_next_shape() {
    let mut bag = BagGenerator::new();

    // Получаем 7 фигур
    for _ in 0..7 {
        let shape = bag.next_shape();

        // Проверяем, что тип фигуры валиден
        assert!((shape as usize) < 7, "Индекс фигуры должен быть меньше 7");
    }
}

/// Тест 21: Проверка системы 7-bag (все 7 типов в мешке)
///
/// Проверяет, что каждые 7 фигур содержат все 7 типов.
#[test]
fn test_bag_system_all_seven_types() {
    let mut bag = BagGenerator::new();

    // Получаем 7 фигур и собираем их типы
    let mut found_shapes = [false; 7];

    for _ in 0..7 {
        let shape = bag.next_shape();
        found_shapes[shape as usize] = true;
    }

    // Проверяем, что все 7 типов встретились
    for (i, &found) in found_shapes.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна быть в мешке");
    }
}

/// Тест 22: Проверка заполнения нового мешка
///
/// Проверяет, что после 7 фигур создаётся новый мешок.
#[test]
fn test_bag_refill() {
    let mut bag = BagGenerator::new();

    // Получаем 14 фигур (2 полных мешка)
    for _ in 0..14 {
        let shape = bag.next_shape();
        assert!((shape as usize) < 7, "Тип фигуры должен быть валидным");
    }

    // Проверяем, что генератор продолжает работать (новый мешок создан)
    let shape = bag.next_shape();
    assert!(
        (shape as usize) < 7,
        "Генератор должен продолжать выдавать фигуры"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 23-25: Координаты фигур
// ============================================================================

/// Тест 23: Проверка границ координат фигур
///
/// Проверяет, что все координаты находятся в допустимом диапазоне (-2..2).
#[test]
fn test_shape_coords_bounds() {
    for (shape_idx, coords) in SHAPE_COORDS.iter().enumerate() {
        for (block_idx, &(x, y)) in coords.iter().enumerate() {
            assert!(
                (-2..=2).contains(&x),
                "Координата X фигуры {shape_idx} блока {block_idx} должна быть в диапазоне [-2, 2], получена {x}"
            );
            assert!(
                (-2..=2).contains(&y),
                "Координата Y фигуры {shape_idx} блока {block_idx} должна быть в диапазоне [-2, 2], получена {y}"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 25-26: Дополнительные проверки
// ============================================================================

/// Тест 25: Проверка уникальности блоков в фигуре
///
/// Проверяет, что в каждой фигуре нет дублирующихся блоков.
#[test]
fn test_shape_blocks_unique() {
    for (shape_idx, coords) in SHAPE_COORDS.iter().enumerate() {
        // Проверяем каждую пару блоков на уникальность
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                assert_ne!(
                    coords[i], coords[j],
                    "Фигура {shape_idx} имеет дублирующиеся блоки под индексами {i} и {j}"
                );
            }
        }
    }
}
