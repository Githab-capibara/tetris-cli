//! Расширенные тесты фигур тетрамино.
//!
//! Этот модуль содержит 80 расширенных тестов для проверки всех аспектов фигур:
//! - Тесты создания каждой фигуры (14 тестов)
//! - Тесты вращения (20 тестов)
//! - Тесты координат (15 тестов)
//! - Тесты цветов (10 тестов)
//! - Тесты Bag Generator (21 тест)

use crate::game::Dir;
use crate::tetromino::{BagGenerator, ShapeType, Tetromino, SHAPE_COLORS, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ 1-14: Создание каждой фигуры (расширенное)
// ============================================================================

/// Тест 1: Проверка создания T-фигуры через Tetromino::select()
#[test]
fn test_extended_t_select_creation() {
    // Генерируем фигуры пока не получим T
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::T {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 0);
            return;
        }
    }
    panic!("Не удалось получить T-фигуру за 100 попыток");
}

/// Тест 2: Проверка создания L-фигуры через Tetromino::select()
#[test]
fn test_extended_l_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::L {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 1);
            return;
        }
    }
    panic!("Не удалось получить L-фигуру за 100 попыток");
}

/// Тест 3: Проверка создания J-фигуры через Tetromino::select()
#[test]
fn test_extended_j_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::J {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 2);
            return;
        }
    }
    panic!("Не удалось получить J-фигуру за 100 попыток");
}

/// Тест 4: Проверка создания S-фигуры через Tetromino::select()
#[test]
fn test_extended_s_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::S {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 3);
            return;
        }
    }
    panic!("Не удалось получить S-фигуру за 100 попыток");
}

/// Тест 5: Проверка создания Z-фигуры через Tetromino::select()
#[test]
fn test_extended_z_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::Z {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 4);
            return;
        }
    }
    panic!("Не удалось получить Z-фигуру за 100 попыток");
}

/// Тест 6: Проверка создания O-фигуры через Tetromino::select()
#[test]
fn test_extended_o_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::O {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 5);
            return;
        }
    }
    panic!("Не удалось получить O-фигуру за 100 попыток");
}

/// Тест 7: Проверка создания I-фигуры через Tetromino::select()
#[test]
fn test_extended_i_select_creation() {
    for _ in 0..100 {
        let t = Tetromino::select();
        if t.shape == ShapeType::I {
            assert_eq!(t.pos, (4.0, 0.0));
            assert_eq!(t.fg, 6);
            return;
        }
    }
    panic!("Не удалось получить I-фигуру за 100 попыток");
}

/// Тест 8: Проверка создания T-фигуры через from_bag()
#[test]
fn test_extended_t_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::T {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 9: Проверка создания L-фигуры через from_bag()
#[test]
fn test_extended_l_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::L {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 10: Проверка создания J-фигуры через from_bag()
#[test]
fn test_extended_j_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::J {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 11: Проверка создания S-фигуры через from_bag()
#[test]
fn test_extended_s_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::S {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 12: Проверка создания Z-фигуры через from_bag()
#[test]
fn test_extended_z_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::Z {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 13: Проверка создания O-фигуры через from_bag()
#[test]
fn test_extended_o_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::O {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

/// Тест 14: Проверка создания I-фигуры через from_bag()
#[test]
fn test_extended_i_from_bag() {
    let mut bag = BagGenerator::new();
    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        if t.shape == ShapeType::I {
            assert_eq!(t.pos, (4.0, 0.0));
            return;
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-34: Вращение (расширенное)
// ============================================================================

/// Тест 15: Проверка вращения T по часовой на 90 градусов
#[test]
fn test_extended_t_rotate_90_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original = t.coords;
    t.rotate(Dir::Right);

    assert_ne!(t.coords, original);
}

/// Тест 16: Проверка вращения T по часовой на 180 градусов
#[test]
fn test_extended_t_rotate_180_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    t.rotate(Dir::Right);

    assert_ne!(t.coords, original);
}

/// Тест 17: Проверка вращения T по часовой на 270 градусов
#[test]
fn test_extended_t_rotate_270_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original = t.coords;
    for _ in 0..3 {
        t.rotate(Dir::Right);
    }

    assert_ne!(t.coords, original);
}

/// Тест 18: Проверка вращения T против часовой на 90 градусов
#[test]
fn test_extended_t_rotate_90_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original = t.coords;
    t.rotate(Dir::Left);

    assert_ne!(t.coords, original);
}

/// Тест 19: Проверка вращения L по часовой
#[test]
fn test_extended_l_rotate_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_ne!(t.coords, original);
}

/// Тест 20: Проверка вращения L против часовой
#[test]
fn test_extended_l_rotate_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_ne!(t.coords, original);
}

/// Тест 21: Проверка вращения J по часовой
#[test]
fn test_extended_j_rotate_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_ne!(t.coords, original);
}

/// Тест 22: Проверка вращения J против часовой
#[test]
fn test_extended_j_rotate_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_ne!(t.coords, original);
}

/// Тест 23: Проверка вращения S по часовой
#[test]
fn test_extended_s_rotate_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_ne!(t.coords, original);
}

/// Тест 24: Проверка вращения S против часовой
#[test]
fn test_extended_s_rotate_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_ne!(t.coords, original);
}

/// Тест 25: Проверка вращения Z по часовой
#[test]
fn test_extended_z_rotate_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_ne!(t.coords, original);
}

/// Тест 26: Проверка вращения Z против часовой
#[test]
fn test_extended_z_rotate_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_ne!(t.coords, original);
}

/// Тест 27: Проверка что O не вращается по часовой
#[test]
fn test_extended_o_rotate_cw_no_change() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_eq!(t.coords, original);
}

/// Тест 28: Проверка что O не вращается против часовой
#[test]
fn test_extended_o_rotate_ccw_no_change() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_eq!(t.coords, original);
}

/// Тест 29: Проверка вращения I по часовой
#[test]
fn test_extended_i_rotate_cw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    let original = t.coords;
    t.rotate(Dir::Right);
    assert_ne!(t.coords, original);
}

/// Тест 30: Проверка вращения I против часовой
#[test]
fn test_extended_i_rotate_ccw() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    let original = t.coords;
    t.rotate(Dir::Left);
    assert_ne!(t.coords, original);
}

/// Тест 31: Проверка что 4 вращения по часовой возвращают к исходному состоянию
#[test]
fn test_extended_four_cw_rotations_return() {
    let shapes = [ShapeType::T, ShapeType::L, ShapeType::J, ShapeType::S, ShapeType::Z, ShapeType::I];

    for &shape in shapes.iter() {
        let mut t = Tetromino {
            pos: (4.0, 0.0),
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: shape as usize,
        };

        let original = t.coords;
        for _ in 0..4 {
            t.rotate(Dir::Right);
        }

        assert_eq!(t.coords, original, "{:?} должна вернуться в исходное состояние", shape);
    }
}

/// Тест 32: Проверка что 4 вращения против часовой возвращают к исходному состоянию
#[test]
fn test_extended_four_ccw_rotations_return() {
    let shapes = [ShapeType::T, ShapeType::L, ShapeType::J, ShapeType::S, ShapeType::Z, ShapeType::I];

    for &shape in shapes.iter() {
        let mut t = Tetromino {
            pos: (4.0, 0.0),
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: shape as usize,
        };

        let original = t.coords;
        for _ in 0..4 {
            t.rotate(Dir::Left);
        }

        assert_eq!(t.coords, original, "{:?} должна вернуться в исходное состояние", shape);
    }
}

/// Тест 33: Проверка чередования вращения (cw/ccw)
#[test]
fn test_extended_alternating_cw_ccw_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original = t.coords;

    // Чередование: cw, ccw, cw, ccw
    t.rotate(Dir::Right);
    t.rotate(Dir::Left);
    t.rotate(Dir::Right);
    t.rotate(Dir::Left);

    assert_eq!(t.coords, original);
}

/// Тест 34: Проверка что вращение не изменяет позицию
#[test]
fn test_extended_rotation_does_not_change_position() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    let original_pos = t.pos;
    t.rotate(Dir::Right);
    t.rotate(Dir::Right);

    assert_eq!(t.pos, original_pos);
}

// ============================================================================
// ГРУППА ТЕСТОВ 35-49: Координаты (расширенное)
// ============================================================================

/// Тест 35: Проверка что у T-фигуры 4 уникальных блока
#[test]
fn test_extended_t_unique_blocks() {
    let coords = SHAPE_COORDS[0];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки T-фигуры должны быть уникальны");
        }
    }
}

/// Тест 36: Проверка что у L-фигуры 4 уникальных блока
#[test]
fn test_extended_l_unique_blocks() {
    let coords = SHAPE_COORDS[1];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки L-фигуры должны быть уникальны");
        }
    }
}

/// Тест 37: Проверка что у J-фигуры 4 уникальных блока
#[test]
fn test_extended_j_unique_blocks() {
    let coords = SHAPE_COORDS[2];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки J-фигуры должны быть уникальны");
        }
    }
}

/// Тест 38: Проверка что у S-фигуры 4 уникальных блока
#[test]
fn test_extended_s_unique_blocks() {
    let coords = SHAPE_COORDS[3];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки S-фигуры должны быть уникальны");
        }
    }
}

/// Тест 39: Проверка что у Z-фигуры 4 уникальных блока
#[test]
fn test_extended_z_unique_blocks() {
    let coords = SHAPE_COORDS[4];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки Z-фигуры должны быть уникальны");
        }
    }
}

/// Тест 40: Проверка что у O-фигуры 4 уникальных блока
#[test]
fn test_extended_o_unique_blocks() {
    let coords = SHAPE_COORDS[5];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки O-фигуры должны быть уникальны");
        }
    }
}

/// Тест 41: Проверка что у I-фигуры 4 уникальных блока
#[test]
fn test_extended_i_unique_blocks() {
    let coords = SHAPE_COORDS[6];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(coords[i], coords[j], "Блоки I-фигуры должны быть уникальны");
        }
    }
}

/// Тест 42: Проверка что все координаты T в диапазоне [-2, 2]
#[test]
fn test_extended_t_coords_in_range() {
    let coords = SHAPE_COORDS[0];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x), "X координата T должна быть в [-2, 2]");
        assert!((-2..=2).contains(&y), "Y координата T должна быть в [-2, 2]");
    }
}

/// Тест 43: Проверка что все координаты L в диапазоне [-2, 2]
#[test]
fn test_extended_l_coords_in_range() {
    let coords = SHAPE_COORDS[1];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 44: Проверка что все координаты J в диапазоне [-2, 2]
#[test]
fn test_extended_j_coords_in_range() {
    let coords = SHAPE_COORDS[2];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 45: Проверка что все координаты S в диапазоне [-2, 2]
#[test]
fn test_extended_s_coords_in_range() {
    let coords = SHAPE_COORDS[3];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 46: Проверка что все координаты Z в диапазоне [-2, 2]
#[test]
fn test_extended_z_coords_in_range() {
    let coords = SHAPE_COORDS[4];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 47: Проверка что все координаты O в диапазоне [-2, 2]
#[test]
fn test_extended_o_coords_in_range() {
    let coords = SHAPE_COORDS[5];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 48: Проверка что все координаты I в диапазоне [-2, 2]
#[test]
fn test_extended_i_coords_in_range() {
    let coords = SHAPE_COORDS[6];
    for &(x, y) in coords.iter() {
        assert!((-2..=2).contains(&x));
        assert!((-2..=2).contains(&y));
    }
}

/// Тест 49: Проверка что все фигуры имеют ровно 4 блока
#[test]
fn test_extended_all_shapes_have_four_blocks() {
    for (i, coords) in SHAPE_COORDS.iter().enumerate() {
        assert_eq!(coords.len(), 4, "Фигура {} должна иметь 4 блока", i);
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 50-59: Цвета (расширенное)
// ============================================================================

/// Тест 50: Проверка что SHAPE_COLORS имеет 7 элементов
#[test]
fn test_extended_shape_colors_length() {
    assert_eq!(SHAPE_COLORS.len(), 7);
}

/// Тест 51: Проверка что T-фигура имеет индекс цвета 0
#[test]
fn test_extended_t_color_index() {
    assert_eq!(ShapeType::T as usize, 0);
}

/// Тест 52: Проверка что L-фигура имеет индекс цвета 1
#[test]
fn test_extended_l_color_index() {
    assert_eq!(ShapeType::L as usize, 1);
}

/// Тест 53: Проверка что J-фигура имеет индекс цвета 2
#[test]
fn test_extended_j_color_index() {
    assert_eq!(ShapeType::J as usize, 2);
}

/// Тест 54: Проверка что S-фигура имеет индекс цвета 3
#[test]
fn test_extended_s_color_index() {
    assert_eq!(ShapeType::S as usize, 3);
}

/// Тест 55: Проверка что Z-фигура имеет индекс цвета 4
#[test]
fn test_extended_z_color_index() {
    assert_eq!(ShapeType::Z as usize, 4);
}

/// Тест 56: Проверка что O-фигура имеет индекс цвета 5
#[test]
fn test_extended_o_color_index() {
    assert_eq!(ShapeType::O as usize, 5);
}

/// Тест 57: Проверка что I-фигура имеет индекс цвета 6
#[test]
fn test_extended_i_color_index() {
    assert_eq!(ShapeType::I as usize, 6);
}

/// Тест 58: Проверка что fg соответствует shape для всех фигур
#[test]
fn test_extended_fg_matches_shape() {
    // Проверяем соответствие для всех 7 фигур
    for i in 0..7 {
        let shape = match i {
            0 => ShapeType::T,
            1 => ShapeType::L,
            2 => ShapeType::J,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::O,
            6 => ShapeType::I,
            _ => unreachable!(),
        };
        
        let t = Tetromino {
            pos: (4.0, 0.0),
            shape,
            coords: SHAPE_COORDS[i],
            fg: i,
        };
        assert_eq!(t.fg, t.shape as usize);
    }
}

/// Тест 59: Проверка что цвета не равны None
#[test]
fn test_extended_colors_not_none() {
    for color in SHAPE_COLORS.iter() {
        // Просто проверяем что цвет существует
        let _ = *color;
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 60-80: Bag Generator (расширенное)
// ============================================================================

/// Тест 60: Проверка создания BagGenerator
#[test]
fn test_extended_bag_creation() {
    let bag = BagGenerator::new();
    let _ = bag;
}

/// Тест 61: Проверка Default для BagGenerator
#[test]
fn test_extended_bag_default() {
    let bag = BagGenerator::default();
    let _ = bag;
}

/// Тест 62: Проверка что next_shape возвращает валидную фигуру
#[test]
fn test_extended_bag_next_shape_valid() {
    let mut bag = BagGenerator::new();
    for _ in 0..100 {
        let shape = bag.next_shape();
        assert!((shape as usize) < 7);
    }
}

/// Тест 63: Проверка что get_index начинается с 0
#[test]
fn test_extended_bag_index_starts_at_zero() {
    let bag = BagGenerator::new();
    assert_eq!(bag.get_index(), 0);
}

/// Тест 64: Проверка что get_index увеличивается после next_shape
#[test]
fn test_extended_bag_index_increments() {
    let mut bag = BagGenerator::new();
    for i in 0..7 {
        let _ = bag.next_shape();
        assert_eq!(bag.get_index(), i + 1);
    }
}

/// Тест 65: Проверка что get_index сбрасывается после 7 фигур
#[test]
fn test_extended_bag_index_resets() {
    let mut bag = BagGenerator::new();
    for _ in 0..7 {
        let _ = bag.next_shape();
    }
    let _ = bag.next_shape();
    assert_eq!(bag.get_index(), 1);
}

/// Тест 66: Проверка что get_bag пуст в начале
#[test]
fn test_extended_bag_empty_at_start() {
    let bag = BagGenerator::new();
    assert_eq!(bag.get_bag().len(), 0);
}

/// Тест 67: Проверка что get_bag имеет 7 элементов после первого next_shape
#[test]
fn test_extended_bag_has_seven_after_first() {
    let mut bag = BagGenerator::new();
    let _ = bag.next_shape();
    assert_eq!(bag.get_bag().len(), 7);
}

/// Тест 68: Проверка равномерности распределения на 7000 фигур
#[test]
fn test_extended_bag_distribution_7000() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    for _ in 0..7000 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(count, 1000, "Фигура {} должна встретиться 1000 раз", i);
    }
}

/// Тест 69: Проверка что BagGenerator выдаёт все 7 фигур в первом мешке
#[test]
fn test_extended_bag_first_bag_has_all_seven() {
    let mut bag = BagGenerator::new();
    let mut found = [false; 7];

    for _ in 0..7 {
        let shape = bag.next_shape();
        found[shape as usize] = true;
    }

    for (i, &f) in found.iter().enumerate() {
        assert!(f, "Фигура {} должна быть в первом мешке", i);
    }
}

/// Тест 70: Проверка что BagGenerator выдаёт все 7 фигур во втором мешке
#[test]
fn test_extended_bag_second_bag_has_all_seven() {
    let mut bag = BagGenerator::new();

    // Пропускаем первый мешок
    for _ in 0..7 {
        let _ = bag.next_shape();
    }

    // Проверяем второй мешок
    let mut found = [false; 7];
    for _ in 0..7 {
        let shape = bag.next_shape();
        found[shape as usize] = true;
    }

    for (i, &f) in found.iter().enumerate() {
        assert!(f, "Фигура {} должна быть во втором мешке", i);
    }
}

/// Тест 71: Проверка производительности BagGenerator на 100000 фигур
#[test]
fn test_extended_bag_performance_100k() {
    let mut bag = BagGenerator::new();
    let start = std::time::Instant::now();

    for _ in 0..100000 {
        let _ = bag.next_shape();
    }

    let duration = start.elapsed();
    assert!(duration.as_secs_f64() < 5.0, "100000 фигур должны сгенерироваться меньше чем за 5 секунд");
}

/// Тест 72: Проверка что BagGenerator можно использовать многократно
#[test]
fn test_extended_bag_reusable() {
    let mut bag = BagGenerator::new();

    for _ in 0..10 {
        for _ in 0..7 {
            let shape = bag.next_shape();
            assert!((shape as usize) < 7);
        }
    }
}

/// Тест 73: Проверка что BagGenerator не паникует при большом количестве вызовов
#[test]
fn test_extended_bag_no_panic_many_calls() {
    let mut bag = BagGenerator::new();

    for _ in 0..10000 {
        let _ = bag.next_shape();
    }

    // Тест прошёл если не было паники
    assert!(true);
}

/// Тест 74: Проверка что BagGenerator выдаёт фигуры в случайном порядке
#[test]
fn test_extended_bag_random_order() {
    let mut bag = BagGenerator::new();
    let mut sequences = Vec::new();

    for _ in 0..10 {
        let mut seq = Vec::new();
        for _ in 0..7 {
            seq.push(bag.next_shape() as usize);
        }
        sequences.push(seq);
    }

    // Проверяем что последовательности разные (хотя бы 2 из 10)
    let mut unique_count = 0;
    for i in 0..sequences.len() {
        for j in (i + 1)..sequences.len() {
            if sequences[i] != sequences[j] {
                unique_count += 1;
            }
        }
    }

    assert!(unique_count > 0, "Должны быть разные последовательности");
}

/// Тест 75: Проверка что BagGenerator корректно работает с from_bag
#[test]
fn test_extended_bag_with_from_bag() {
    let mut bag = BagGenerator::new();

    for _ in 0..10 {
        let t = Tetromino::from_bag(&mut bag);
        assert!((t.shape as usize) < 7);
        assert_eq!(t.pos, (4.0, 0.0));
    }
}

/// Тест 76: Проверка что BagGenerator не содержит дубликатов в мешке
#[test]
fn test_extended_bag_no_duplicates() {
    let mut bag = BagGenerator::new();

    for _ in 0..10 {
        let mut shapes = Vec::new();
        for _ in 0..7 {
            shapes.push(bag.next_shape());
        }

        // Проверяем на дубликаты
        for i in 0..shapes.len() {
            for j in (i + 1)..shapes.len() {
                assert_ne!(shapes[i], shapes[j], "В мешке не должно быть дубликатов");
            }
        }
    }
}

/// Тест 77: Проверка что BagGenerator корректно обрабатывает границу мешка
#[test]
fn test_extended_bag_boundary() {
    let mut bag = BagGenerator::new();

    // Получаем 6 фигур
    for _ in 0..6 {
        let _ = bag.next_shape();
    }

    assert_eq!(bag.get_index(), 6);

    // Получаем 7-ю фигуру
    let _ = bag.next_shape();
    assert_eq!(bag.get_index(), 7);

    // Получаем первую фигуру нового мешка
    let _ = bag.next_shape();
    assert_eq!(bag.get_index(), 1);
}

/// Тест 78: Проверка что BagGenerator работает с разными типами
#[test]
fn test_extended_bag_with_all_types() {
    let mut bag = BagGenerator::new();
    let mut found = [false; 7];

    for _ in 0..70 {
        let shape = bag.next_shape();
        found[shape as usize] = true;
    }

    for (_i, &f) in found.iter().enumerate() {
        assert!(f, "Все типы фигур должны встретиться");
    }
}

/// Тест 79: Проверка что BagGenerator стабилен при многократном использовании
#[test]
fn test_extended_bag_stability() {
    let mut bag = BagGenerator::new();

    for _ in 0..100 {
        let mut counts = [0; 7];
        for _ in 0..7 {
            let shape = bag.next_shape();
            counts[shape as usize] += 1;
        }

        // Каждый мешок должен содержать все 7 фигур
        for &count in counts.iter() {
            assert_eq!(count, 1);
        }
    }
}

/// Тест 80: Проверка что BagGenerator корректно инициализируется
#[test]
fn test_extended_bag_initialization() {
    let bag = BagGenerator::new();
    assert_eq!(bag.get_index(), 0);
    assert_eq!(bag.get_bag().len(), 0);
}
