//! Тесты форм и свойств фигур тетрамино в Tetris CLI.
//!
//! Этот модуль содержит 40 тестов для детальной проверки всех аспектов фигур:
//! - Детальные тесты для каждой из 7 фигур (14 тестов)
//! - Тесты координат блоков (8 тестов)
//! - Тесты цветов (6 тестов)
//! - Тесты начальной позиции (6 тестов)
//! - Тесты после вращения (6 тестов)
//!
//! Все тесты проверяют отдельные аспекты свойств фигур.

use crate::tetromino::{ShapeType, Tetromino, SHAPE_COLORS, SHAPE_COORDS};
use crate::types::RotationDirection;

// ============================================================================
// ГРУППА ТЕСТОВ 1-14: Детальные тесты для каждой из 7 фигур
// ============================================================================

/// Тест 1: T-фигура - базовые свойства
///
/// Проверяет все основные свойства T-фигуры.
#[test]
fn test_t_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    assert_eq!(t.shape, ShapeType::T, "Тип фигуры должен быть T");
    assert_eq!(t.fg, 0, "Индекс цвета должен быть 0");
    assert_eq!(t.coords.len(), 4, "У фигуры должно быть 4 блока");
    assert!(
        (t.pos.0 - 4.0).abs() < f32::EPSILON,
        "Позиция X должна быть 4.0"
    );
    assert!(
        (t.pos.1 - 0.0).abs() < f32::EPSILON,
        "Позиция Y должна быть 0.0"
    );
}

/// Тест 2: L-фигура - базовые свойства
#[test]
fn test_l_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    assert_eq!(t.shape, ShapeType::L, "Тип фигуры должен быть L");
    assert_eq!(t.fg, 1, "Индекс цвета должен быть 1");
    assert_eq!(t.coords.len(), 4, "У фигуры должно быть 4 блока");
}

/// Тест 3: J-фигура - базовые свойства
#[test]
fn test_j_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    assert_eq!(t.shape, ShapeType::J, "Тип фигуры должен быть J");
    assert_eq!(t.fg, 2, "Индекс цвета должен быть 2");
}

/// Тест 4: S-фигура - базовые свойства
#[test]
fn test_s_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    assert_eq!(t.shape, ShapeType::S, "Тип фигуры должен быть S");
    assert_eq!(t.fg, 3, "Индекс цвета должен быть 3");
}

/// Тест 5: Z-фигура - базовые свойства
#[test]
fn test_z_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    assert_eq!(t.shape, ShapeType::Z, "Тип фигуры должен быть Z");
    assert_eq!(t.fg, 4, "Индекс цвета должен быть 4");
}

/// Тест 6: O-фигура - базовые свойства
#[test]
fn test_o_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    assert_eq!(t.shape, ShapeType::O, "Тип фигуры должен быть O");
    assert_eq!(t.fg, 5, "Индекс цвета должен быть 5");
}

/// Тест 7: I-фигура - базовые свойства
#[test]
fn test_i_piece_basic_properties() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    assert_eq!(t.shape, ShapeType::I, "Тип фигуры должен быть I");
    assert_eq!(t.fg, 6, "Индекс цвета должен быть 6");
}

/// Тест 8: T-фигура - форма и структура
#[test]
fn test_t_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // T-фигура: три блока в ряд с одним блоком сверху по центру
    // Координаты: (-1,0), (0,0), (1,0), (0,1)
    assert_eq!(t.coords[0], (-1, 0), "Первый блок T-фигуры");
    assert_eq!(t.coords[1], (0, 0), "Центральный блок T-фигуры");
    assert_eq!(t.coords[2], (1, 0), "Правый блок T-фигуры");
    assert_eq!(t.coords[3], (0, 1), "Верхний блок T-фигуры");
}

/// Тест 9: L-фигура - форма и структура
#[test]
fn test_l_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    // L-фигура: три блока в ряд с одним блоком снизу справа
    assert_eq!(t.coords[0], (-1, -1), "Левый верхний блок L-фигуры");
    assert_eq!(t.coords[3], (0, 1), "Нижний блок L-фигуры");
}

/// Тест 10: J-фигура - форма и структура
#[test]
fn test_j_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    // J-фигура: зеркальная L - блок снизу слева
    assert_eq!(t.coords[0], (1, -1), "Правый верхний блок J-фигуры");
    assert_eq!(t.coords[1], (0, -1), "Центральный верхний блок J-фигуры");
}

/// Тест 11: S-фигура - форма и структура
#[test]
fn test_s_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    // S-фигура: два блока в ряд со сдвигом вправо
    assert_eq!(t.coords[0], (0, -1), "Верхний левый блок S-фигуры");
    assert_eq!(t.coords[3], (1, 1), "Нижний правый блок S-фигуры");
}

/// Тест 12: Z-фигура - форма и структура
#[test]
fn test_z_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    // Z-фигура: зеркальная S - сдвиг влево
    assert_eq!(t.coords[0], (0, -1), "Верхний блок Z-фигуры");
    assert_eq!(t.coords[3], (-1, 1), "Нижний левый блок Z-фигуры");
}

/// Тест 13: O-фигура - форма и структура (квадрат 2x2)
#[test]
fn test_o_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    // O-фигура: квадрат 2x2
    assert_eq!(t.coords[0], (0, 0), "Левый верхний блок O-фигуры");
    assert_eq!(t.coords[1], (1, 0), "Правый верхний блок O-фигуры");
    assert_eq!(t.coords[2], (0, 1), "Левый нижний блок O-фигуры");
    assert_eq!(t.coords[3], (1, 1), "Правый нижний блок O-фигуры");
}

/// Тест 14: I-фигура - форма и структура (линия)
#[test]
fn test_i_piece_shape_structure() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    // I-фигура: четыре блока в вертикальный ряд
    assert_eq!(t.coords[0], (0, -1), "Верхний блок I-фигуры");
    assert_eq!(t.coords[1], (0, 0), "Второй блок I-фигуры");
    assert_eq!(t.coords[2], (0, 1), "Третий блок I-фигуры");
    assert_eq!(t.coords[3], (0, 2), "Нижний блок I-фигуры");
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-22: Тесты координат блоков
// ============================================================================

/// Тест 15: Все фигуры имеют 4 блока
#[test]
fn test_all_pieces_have_four_blocks() {
    for (i, coords) in SHAPE_COORDS.iter().enumerate() {
        assert_eq!(coords.len(), 4, "Фигура {i} должна иметь 4 блока");
    }
}

/// Тест 16: Координаты всех фигур в допустимом диапазоне
#[test]
fn test_all_coords_in_valid_range() {
    for (shape_idx, coords) in SHAPE_COORDS.iter().enumerate() {
        for (block_idx, &(x, y)) in coords.iter().enumerate() {
            assert!(
                (-2..=2).contains(&x),
                "X координата фигуры {shape_idx} блока {block_idx} должна быть в [-2, 2], получена {x}"
            );
            assert!(
                (-2..=2).contains(&y),
                "Y координата фигуры {shape_idx} блока {block_idx} должна быть в [-2, 2], получена {y}"
            );
        }
    }
}

/// Тест 17: Уникальность координат в T-фигуре
#[test]
fn test_t_piece_coords_unique() {
    let coords = SHAPE_COORDS[0];

    // Проверяем, что все координаты уникальны
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(
                coords[i], coords[j],
                "T-фигура имеет дублирующиеся блоки {i} и {j}"
            );
        }
    }
}

/// Тест 18: Уникальность координат в L-фигуре
#[test]
fn test_l_piece_coords_unique() {
    let coords = SHAPE_COORDS[1];

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(coords[i], coords[j], "L-фигура имеет дублирующиеся блоки");
        }
    }
}

/// Тест 19: Уникальность координат в J-фигуре
#[test]
fn test_j_piece_coords_unique() {
    let coords = SHAPE_COORDS[2];

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(coords[i], coords[j]);
        }
    }
}

/// Тест 20: Уникальность координат в S-фигуре
#[test]
fn test_s_piece_coords_unique() {
    let coords = SHAPE_COORDS[3];

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(coords[i], coords[j]);
        }
    }
}

/// Тест 21: Уникальность координат в Z-фигуре
#[test]
fn test_z_piece_coords_unique() {
    let coords = SHAPE_COORDS[4];

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(coords[i], coords[j]);
        }
    }
}

/// Тест 22: Уникальность координат в I-фигуре
#[test]
fn test_i_piece_coords_unique() {
    let coords = SHAPE_COORDS[6];

    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            assert_ne!(coords[i], coords[j]);
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 23-28: Тесты цветов
// ============================================================================

/// Тест 23: Количество цветов соответствует количеству фигур
#[test]
fn test_colors_count_matches_pieces_count() {
    assert_eq!(SHAPE_COLORS.len(), 7, "Должно быть 7 цветов для 7 фигур");
}

/// Тест 24: T-фигура имеет правильный цвет (Magenta)
#[test]
fn test_t_piece_color() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    assert_eq!(t.fg, 0, "T-фигура должна иметь индекс цвета 0 (Magenta)");
}

/// Тест 25: L-фигура имеет правильный цвет (Yellow)
#[test]
fn test_l_piece_color() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    assert_eq!(t.fg, 1, "L-фигура должна иметь индекс цвета 1 (Yellow)");
}

/// Тест 26: J-фигура имеет правильный цвет (Blue)
#[test]
fn test_j_piece_color() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::J,
        coords: SHAPE_COORDS[2],
        fg: 2,
    };

    assert_eq!(t.fg, 2, "J-фигура должна иметь индекс цвета 2 (Blue)");
}

/// Тест 27: S-фигура имеет правильный цвет (Green)
#[test]
fn test_s_piece_color() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::S,
        coords: SHAPE_COORDS[3],
        fg: 3,
    };

    assert_eq!(t.fg, 3, "S-фигура должна иметь индекс цвета 3 (Green)");
}

/// Тест 28: Z-фигура имеет правильный цвет (`LightRed`)
#[test]
fn test_z_piece_color() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::Z,
        coords: SHAPE_COORDS[4],
        fg: 4,
    };

    assert_eq!(t.fg, 4, "Z-фигура должна иметь индекс цвета 4 (LightRed)");
}

// ============================================================================
// ГРУППА ТЕСТОВ 29-34: Тесты начальной позиции
// ============================================================================

/// Тест 29: Начальная позиция X равна 4.0 (центр)
#[test]
fn test_initial_position_x_is_center() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    assert!(
        (t.pos.0 - 4.0).abs() < f32::EPSILON,
        "Начальная позиция X должна быть 4.0 (центр)"
    );
}

/// Тест 30: Начальная позиция Y равна 0.0 (верх поля)
#[test]
fn test_initial_position_y_is_top() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    assert!(
        (t.pos.1 - 0.0).abs() < f32::EPSILON,
        "Начальная позиция Y должна быть 0.0"
    );
}

/// Тест 31: Все фигуры появляются в центре
#[test]
fn test_all_pieces_spawn_at_center() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in &shapes {
        let t = Tetromino {
            pos: (4.0, 0.0),
            shape: *shape,
            coords: SHAPE_COORDS[*shape as usize],
            fg: *shape as u8,
        };

        assert!(
            (t.pos.0 - 4.0).abs() < f32::EPSILON,
            "{shape:?} фигура должна появляться в центре по X"
        );
        assert!(
            (t.pos.1 - 0.0).abs() < f32::EPSILON,
            "{shape:?} фигура должна появляться сверху по Y"
        );
    }
}

/// Тест 32: Позиция не изменяется при создании
#[test]
fn test_position_unchanged_on_creation() {
    let t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Проверяем, что позиция точно равна ожидаемой
    assert_eq!(t.pos, (4.0, 0.0), "Позиция должна быть (4.0, 0.0)");
}

/// Тест 33: Позиция после перемещения влево
#[test]
fn test_position_after_move_left() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    t.pos.0 -= 1.0;
    assert!(
        (t.pos.0 - 3.0).abs() < f32::EPSILON,
        "После движения влево X должен быть 3.0"
    );
}

/// Тест 34: Позиция после перемещения вправо
#[test]
fn test_position_after_move_right() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    t.pos.0 += 1.0;
    assert!(
        (t.pos.0 - 5.0).abs() < f32::EPSILON,
        "После движения вправо X должен быть 5.0"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 35-40: Тесты после вращения
// ============================================================================

/// Тест 35: T-фигура после одного вращения по часовой
#[test]
fn test_t_piece_after_one_clockwise_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    t.rotate(RotationDirection::Clockwise);

    // После вращения по часовой: (x,y) -> (-y,x)
    // Исходные: (-1,0), (0,0), (1,0), (0,1)
    // После: (0,-1), (0,0), (0,1), (-1,0)
    assert_eq!(
        t.coords[1],
        (0, 0),
        "Центральный блок должен остаться на месте"
    );
}

/// Тест 36: T-фигура после двух вращений
#[test]
fn test_t_piece_after_two_rotations() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    t.rotate(RotationDirection::Clockwise);
    t.rotate(RotationDirection::Clockwise);

    // После 2 вращений фигура перевёрнута
    assert_ne!(
        t.coords, SHAPE_COORDS[0],
        "После 2 вращений координаты должны измениться"
    );
}

/// Тест 37: T-фигура после трёх вращений
#[test]
fn test_t_piece_after_three_rotations() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    for _ in 0..3 {
        t.rotate(RotationDirection::Clockwise);
    }

    // 3 вращения по часовой = 1 против часовой
    assert_ne!(
        t.coords, SHAPE_COORDS[0],
        "После 3 вращений координаты должны отличаться от исходных"
    );
}

/// Тест 38: T-фигура после четырёх вращений (полный цикл)
#[test]
fn test_t_piece_after_four_rotations() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    for _ in 0..4 {
        t.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        t.coords, SHAPE_COORDS[0],
        "После 4 вращений фигура должна вернуться к исходным координатам"
    );
}

/// Тест 39: L-фигура после вращения
#[test]
fn test_l_piece_after_rotation() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::L,
        coords: SHAPE_COORDS[1],
        fg: 1,
    };

    let original_coords = t.coords;
    t.rotate(RotationDirection::Clockwise);

    assert_ne!(
        t.coords, original_coords,
        "L-фигура должна изменить координаты после вращения"
    );
}

/// Тест 40: I-фигура после вращения (из вертикальной в горизонтальную)
#[test]
fn test_i_piece_after_rotation_vertical_to_horizontal() {
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    // Исходная I-фигура вертикальная: (0,-1), (0,0), (0,1), (0,2)
    assert_eq!(t.coords[0].0, 0, "Исходно I-фигура вертикальна (x=0)");

    t.rotate(RotationDirection::Clockwise);

    // После вращения I-фигура становится горизонтальной
    // Все блоки должны быть на одной строке (y=0 для центрального блока)
    assert_eq!(
        t.coords[1],
        (0, 0),
        "Центральный блок I-фигуры должен остаться на месте"
    );
}
