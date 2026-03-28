//! Тесты обработки Direction::Down в rotate() (tetromino.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка что Direction::Down не меняет координаты
//! - Проверка что Direction::Left работает корректно
//! - Проверка что Direction::Right работает корректно
//!
//! Исправление: добавлен ранний возврат для Direction::Down в rotate()

use crate::types::Direction;
use crate::types::RotationDirection;
use crate::tetromino::{Tetromino, ShapeType, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ: Direction::Down в rotate()
// ============================================================================

/// Тест 1: Проверка что Direction::Down не меняет координаты
///
/// Проверяет, что rotate(Direction::Down) не изменяет координаты фигуры.
#[test]
fn test_dir_down_does_not_change_coords() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Сохраняем оригинальные координаты
    let original_coords = tetromino.coords;

    // Вызываем rotate с Direction::Down
    tetromino.rotate_old(Direction::Down);

    // Проверяем что координаты не изменились
    assert_eq!(
        tetromino.coords, original_coords,
        "Direction::Down не должен менять координаты фигуры"
    );

    // Проверяем с другими типами фигур
    for (shape_idx, &shape_type) in [
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::I,
    ]
    .iter()
    .enumerate()
    {
        let mut t = Tetromino {
            pos: (4.0, 0.0),
            shape: shape_type,
            coords: SHAPE_COORDS[shape_idx + 1],
            fg: shape_idx + 1,
        };

        let original = t.coords;
        t.rotate_old(Direction::Down);

        assert_eq!(
            t.coords, original,
            "Direction::Down не должен менять координаты фигуры {:?}",
            shape_type
        );
    }

    // Проверяем что квадрат (O) тоже не меняется
    let mut o_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let o_original = o_tetromino.coords;
    o_tetromino.rotate_old(Direction::Down);

    assert_eq!(
        o_tetromino.coords, o_original,
        "Direction::Down не должен менять координаты квадрата"
    );
}

/// Тест 2: Проверка что Direction::Left работает корректно
///
/// Проверяет, что rotate(Direction::Left) вращает фигуру против часовой стрелки.
#[test]
fn test_dir_left_rotates_correctly() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Исходные координаты T: (-1,0), (0,0), (1,0), (0,1)
    let original_coords = tetromino.coords;

    // Вращаем против часовой стрелки (Direction::Left)
    tetromino.rotate_old(Direction::Left);

    // После вращения против часовой: (0,-1), (0,0), (0,1), (-1,0)
    // Формула: (x,y) -> (y,-x)
    assert_eq!(
        tetromino.coords[0].0, original_coords[0].1,
        "X координата должна измениться по формуле (x,y) -> (y,-x)"
    );
    assert_eq!(
        tetromino.coords[0].1, -original_coords[0].0,
        "Y координата должна измениться по формуле (x,y) -> (y,-x)"
    );

    // Проверяем что вращение на 4 раза возвращает к исходному состоянию
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    for _ in 0..4 {
        t.rotate_old(Direction::Left);
    }

    assert_eq!(
        t.coords, SHAPE_COORDS[0],
        "4 вращения против часовой должны вернуть к исходному состоянию"
    );

    // Проверяем что квадрат (O) не вращается
    let mut o_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let o_original = o_tetromino.coords;
    o_tetromino.rotate_old(Direction::Left);

    assert_eq!(
        o_tetromino.coords, o_original,
        "Квадрат не должен вращаться"
    );
}

/// Тест 3: Проверка что Direction::Right работает корректно
///
/// Проверяет, что rotate(Direction::Right) вращает фигуру по часовой стрелке.
#[test]
fn test_dir_right_rotates_correctly() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Исходные координаты T: (-1,0), (0,0), (1,0), (0,1)
    let original_coords = tetromino.coords;

    // Вращаем по часовой стрелке (Direction::Right)
    tetromino.rotate_old(Direction::Right);

    // После вращения по часовой: (0,-1), (0,0), (0,1), (-1,0)
    // Формула: (x,y) -> (-y,x)
    assert_eq!(
        tetromino.coords[0].0, -original_coords[0].1,
        "X координата должна измениться по формуле (x,y) -> (-y,x)"
    );
    assert_eq!(
        tetromino.coords[0].1, original_coords[0].0,
        "Y координата должна измениться по формуле (x,y) -> (-y,x)"
    );

    // Проверяем что вращение на 4 раза возвращает к исходному состоянию
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    for _ in 0..4 {
        t.rotate_old(Direction::Right);
    }

    assert_eq!(
        t.coords, SHAPE_COORDS[0],
        "4 вращения по часовой должны вернуть к исходному состоянию"
    );

    // Проверяем что квадрат (O) не вращается
    let mut o_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };

    let o_original = o_tetromino.coords;
    o_tetromino.rotate_old(Direction::Right);

    assert_eq!(
        o_tetromino.coords, o_original,
        "Квадрат не должен вращаться"
    );

    // Проверяем I-фигуру
    let mut i_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    let i_original = i_tetromino.coords;
    i_tetromino.rotate_old(Direction::Right);

    // I-фигура должна измениться (вертикальная -> горизонтальная)
    assert_ne!(
        i_tetromino.coords, i_original,
        "I-фигура должна вращаться"
    );

    // После 4 вращений должна вернуться к исходному состоянию
    let mut i_tetromino2 = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };

    for _ in 0..4 {
        i_tetromino2.rotate_old(Direction::Right);
    }

    assert_eq!(
        i_tetromino2.coords, i_original,
        "4 вращения I-фигуры должны вернуть к исходному состоянию"
    );
}

/// Тест 4: Проверка что все направления работают корректно
///
/// Интеграционный тест для всех направлений вращения.
#[test]
fn test_all_directions_work_correctly() {
    // Проверяем все типы фигур
    for (shape_idx, &shape_type) in [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ]
    .iter()
    .enumerate()
    {
        let mut t = Tetromino {
            pos: (4.0, 0.0),
            shape: shape_type,
            coords: SHAPE_COORDS[shape_idx],
            fg: shape_idx,
        };

        // Direction::Down не должен менять координаты
        let original = t.coords;
        t.rotate_old(Direction::Down);
        assert_eq!(
            t.coords, original,
            "Direction::Down не должен менять координаты для {:?}",
            shape_type
        );

        // Direction::Left и Direction::Right должны вращать
        if shape_type != ShapeType::O {
            t.rotate_old(Direction::Left);
            assert_ne!(
                t.coords, original,
                "Direction::Left должен вращать фигуру {:?}",
                shape_type
            );

            t.rotate_old(Direction::Right);
            // После Left + Right должна вернуться к исходному
            assert_eq!(
                t.coords, original,
                "Left + Right должны вернуть к исходному для {:?}",
                shape_type
            );
        }
    }
}

/// Тест 5: Проверка последовательных вращений
///
/// Проверяет корректность множественных вращений.
#[test]
fn test_sequential_rotations() {
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Последовательность: Left, Left, Left, Left (должно вернуться к исходному)
    let original = tetromino.coords;
    for _ in 0..4 {
        tetromino.rotate_old(Direction::Left);
    }
    assert_eq!(
        tetromino.coords, original,
        "4 вращения Left должны вернуть к исходному"
    );

    // Последовательность: Right, Right, Right, Right
    for _ in 0..4 {
        tetromino.rotate_old(Direction::Right);
    }
    assert_eq!(
        tetromino.coords, original,
        "4 вращения Right должны вернуть к исходному"
    );

    // Последовательность: Down, Down, Down, Down (не должно менять)
    for _ in 0..4 {
        tetromino.rotate_old(Direction::Down);
    }
    assert_eq!(
        tetromino.coords, original,
        "4 вращения Down не должны менять координаты"
    );

    // Смешанная последовательность: Left, Right, Left, Right
    tetromino.rotate_old(Direction::Left);
    let after_left = tetromino.coords;
    tetromino.rotate_old(Direction::Right);
    assert_eq!(
        tetromino.coords, original,
        "Left + Right должны вернуть к исходному"
    );
    tetromino.rotate_old(Direction::Left);
    assert_eq!(
        tetromino.coords, after_left,
        "После Left должно быть то же состояние"
    );
}
