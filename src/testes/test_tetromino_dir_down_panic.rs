//! Тесты для исправления Dir::Down в rotate() (tetromino.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка паники при Dir::Down с понятным сообщением
//! - Проверка корректного вращения Dir::Left
//! - Проверка корректного вращения Dir::Right
//!
//! Исправление: rotate() теперь паникует при Dir::Down с понятным сообщением

use crate::game::Dir;
use crate::tetromino::RotationDirection;
use crate::tetromino::{Tetromino, ShapeType, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ: Dir::Down fix
// ============================================================================

/// Тест 1: Проверка паники при Dir::Down с понятным сообщением
///
/// Проверяет, что rotate(Dir::Down) вызывает панику с понятным сообщением.
#[test]
#[should_panic(expected = "Dir::Down cannot be used for rotation")]
fn test_dir_down_panic_with_message() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    
    // Вызываем rotate с Dir::Down - должно вызвать панику
    tetromino.rotate_old(Dir::Down);
}

/// Тест 2: Проверка корректного вращения Dir::Left
///
/// Проверяет, что rotate(Dir::Left) вращает фигуру против часовой стрелки.
#[test]
fn test_dir_left_rotation_correct() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    
    // Исходные координаты T: (-1,0), (0,0), (1,0), (0,1)
    let original_coords = tetromino.coords;
    
    // Вращаем против часовой стрелки (Dir::Left)
    // Формула: (x,y) -> (y,-x)
    tetromino.rotate_old(Dir::Left);
    
    // Проверяем первую координату: (-1,0) -> (0,1)
    assert_eq!(
        tetromino.coords[0], (0, 1),
        "Первая координата после вращения Left должна быть (0,1)"
    );
    
    // Проверяем вторую координату: (0,0) -> (0,0)
    assert_eq!(
        tetromino.coords[1], (0, 0),
        "Вторая координата после вращения Left должна быть (0,0)"
    );
    
    // Проверяем третью координату: (1,0) -> (0,-1)
    assert_eq!(
        tetromino.coords[2], (0, -1),
        "Третья координата после вращения Left должна быть (0,-1)"
    );
    
    // Проверяем четвёртую координату: (0,1) -> (1,0)
    assert_eq!(
        tetromino.coords[3], (1, 0),
        "Четвёртая координата после вращения Left должна быть (1,0)"
    );
    
    // Проверяем что 4 вращения Left возвращают к исходному состоянию
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    
    for _ in 0..4 {
        t.rotate_old(Dir::Left);
    }
    
    assert_eq!(
        t.coords, original_coords,
        "4 вращения Left должны вернуть к исходному состоянию"
    );
    
    // Проверяем что квадрат (O) не вращается
    let mut o_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };
    
    let o_original = o_tetromino.coords;
    o_tetromino.rotate_old(Dir::Left);
    
    assert_eq!(
        o_tetromino.coords, o_original,
        "Квадрат (O) не должен вращаться"
    );
}

/// Тест 3: Проверка корректного вращения Dir::Right
///
/// Проверяет, что rotate(Dir::Right) вращает фигуру по часовой стрелке.
#[test]
fn test_dir_right_rotation_correct() {
    // Создаём тестовую фигуру T
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    
    // Исходные координаты T: (-1,0), (0,0), (1,0), (0,1)
    let original_coords = tetromino.coords;
    
    // Вращаем по часовой стрелке (Dir::Right)
    // Формула: (x,y) -> (-y,x)
    tetromino.rotate_old(Dir::Right);
    
    // Проверяем первую координату: (-1,0) -> (0,-1)
    assert_eq!(
        tetromino.coords[0], (0, -1),
        "Первая координата после вращения Right должна быть (0,-1)"
    );
    
    // Проверяем вторую координату: (0,0) -> (0,0)
    assert_eq!(
        tetromino.coords[1], (0, 0),
        "Вторая координата после вращения Right должна быть (0,0)"
    );
    
    // Проверяем третью координату: (1,0) -> (0,1)
    assert_eq!(
        tetromino.coords[2], (0, 1),
        "Третья координата после вращения Right должна быть (0,1)"
    );
    
    // Проверяем четвёртую координату: (0,1) -> (-1,0)
    assert_eq!(
        tetromino.coords[3], (-1, 0),
        "Четвёртая координата после вращения Right должна быть (-1,0)"
    );
    
    // Проверяем что 4 вращения Right возвращают к исходному состоянию
    let mut t = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    
    for _ in 0..4 {
        t.rotate_old(Dir::Right);
    }
    
    assert_eq!(
        t.coords, original_coords,
        "4 вращения Right должны вернуть к исходному состоянию"
    );
    
    // Проверяем что квадрат (O) не вращается
    let mut o_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: SHAPE_COORDS[5],
        fg: 5,
    };
    
    let o_original = o_tetromino.coords;
    o_tetromino.rotate_old(Dir::Right);
    
    assert_eq!(
        o_tetromino.coords, o_original,
        "Квадрат (O) не должен вращаться"
    );
    
    // Проверяем I-фигуру
    let mut i_tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::I,
        coords: SHAPE_COORDS[6],
        fg: 6,
    };
    
    let i_original = i_tetromino.coords;
    i_tetromino.rotate_old(Dir::Right);
    
    // I-фигура должна измениться (вертикальная -> горизонтальная)
    assert_ne!(
        i_tetromino.coords, i_original,
        "I-фигура должна вращаться"
    );
}

/// Тест 4: Проверка что Left и Right дают противоположные результаты
///
/// Проверяет, что вращение Left затем Right возвращает к исходному состоянию.
#[test]
fn test_left_then_right_returns_to_original() {
    // Проверяем для всех типов фигур кроме O
    for (shape_idx, &shape_type) in [
        ShapeType::T,
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
            coords: SHAPE_COORDS[shape_idx],
            fg: shape_idx,
        };
        
        let original = t.coords;
        
        // Вращаем Left затем Right
        t.rotate_old(Dir::Left);
        t.rotate_old(Dir::Right);
        
        assert_eq!(
            t.coords, original,
            "Left затем Right должны вернуть к исходному состоянию для {:?}",
            shape_type
        );
        
        // Вращаем Right затем Left
        t.rotate_old(Dir::Right);
        t.rotate_old(Dir::Left);
        
        assert_eq!(
            t.coords, original,
            "Right затем Left должны вернуть к исходному состоянию для {:?}",
            shape_type
        );
    }
}

/// Тест 5: Проверка что Dir::Down паникует для всех типов фигур
///
/// Проверяет, что rotate(Dir::Down) вызывает панику для любой фигуры.
#[test]
#[should_panic(expected = "Dir::Down cannot be used for rotation")]
fn test_dir_down_panic_for_all_shapes() {
    // Проверяем для всех типов фигур
    for (shape_idx, &shape_type) in [
        ShapeType::T,
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
            coords: SHAPE_COORDS[shape_idx],
            fg: shape_idx,
        };
        
        // Должно вызвать панику для любой фигуры
        t.rotate_old(Dir::Down);
    }
}
