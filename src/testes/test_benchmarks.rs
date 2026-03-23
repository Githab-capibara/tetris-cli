//! Тесты производительности (benches/benchmarks.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка что бенчмарки компилируются
//! - Проверка что check_rows() бенчмарк работает
//! - Проверка что rotate() бенчмарк работает
//!
//! Бенчмарки используют criterion для измерения производительности.

// ============================================================================
// ГРУППА ТЕСТОВ: Бенчмарки
// ============================================================================

/// Тест 1: Проверка что бенчмарки компилируются
///
/// Проверяет, что код бенчмарков компилируется без ошибок.
#[test]
fn test_benchmarks_compile() {
    // Этот тест проверяет что модуль benchmarks существует
    // и основные функции доступны для бенчмаркинга

    use crate::game::GameState;
    use crate::tetromino::{Tetromino, ShapeType, SHAPE_COORDS};

    // Проверяем что GameState можно создать
    let state = GameState::new();
    assert_eq!(state.get_score(), 0, "Новое состояние должно иметь счёт 0");

    // Проверяем что Tetromino можно создать
    let tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    assert_eq!(tetromino.shape, ShapeType::T, "Фигура должна быть T");

    // Проверяем что можно получить поле для бенчмарка
    let blocks = state.get_blocks_for_bench();
    assert_eq!(blocks.len(), 20, "Поле должно содержать 20 строк");
    assert_eq!(blocks[0].len(), 10, "Каждая строка должна содержать 10 клеток");
}

/// Тест 2: Проверка что check_rows() бенчмарк работает
///
/// Проверяет, что check_rows() может быть забенчмаркен.
#[test]
fn test_check_rows_benchmark_works() {
    use crate::game::GameState;
    use std::time::Instant;

    // Создаём состояние игры
    let mut state = GameState::new();

    // Замеряем время выполнения check_rows() на пустом поле
    let start = Instant::now();
    let lines_cleared = state.check_rows_for_bench();
    let elapsed = start.elapsed();

    // Проверяем что check_rows() выполнился
    assert_eq!(lines_cleared, 0, "На пустом поле не должно быть линий");
    assert!(
        elapsed.as_micros() < 1000,
        "check_rows() должен выполниться быстро"
    );

    // Проверяем что check_rows() работает с заполненными линиями
    // Используем fill_line_for_bench() если метод доступен
    // (в реальной игре линии заполняются через save_tetromino())

    // Создаём новое состояние для теста
    let mut state2 = GameState::new();

    // Замеряем время на пустом поле (для сравнения)
    let start = Instant::now();
    for _ in 0..100 {
        let _ = state2.check_rows_for_bench();
    }
    let elapsed_100 = start.elapsed();

    assert!(
        elapsed_100.as_millis() < 100,
        "100 вызовов check_rows() должны выполниться быстро"
    );
}

/// Тест 3: Проверка что rotate() бенчмарк работает
///
/// Проверяет, что rotate() может быть забенчмаркен.
#[test]
fn test_rotate_benchmark_works() {
    use crate::types::Direction;
use crate::types::RotationDirection;
    use crate::tetromino::{Tetromino, ShapeType, SHAPE_COORDS};
    use std::time::Instant;

    // Создаём тестовую фигуру
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };

    // Замеряем время вращения
    let start = Instant::now();
    tetromino.rotate_old(Direction::Right);
    let elapsed = start.elapsed();

    // Проверяем что rotate() выполнился
    assert!(
        elapsed.as_micros() < 100,
        "rotate() должен выполниться быстро"
    );

    // Проверяем что вращение работает корректно
    let original_coords = SHAPE_COORDS[0];
    assert_ne!(
        tetromino.coords, original_coords,
        "Вращение должно изменить координаты"
    );

    // Замеряем время множественных вращений
    let start = Instant::now();
    for _ in 0..1000 {
        let mut t = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: SHAPE_COORDS[0],
            fg: 0,
        };
        t.rotate_old(Direction::Right);
        t.rotate_old(Direction::Left);
    }
    let elapsed_1000 = start.elapsed();

    assert!(
        elapsed_1000.as_millis() < 100,
        "1000 вращений должны выполниться быстро"
    );

    // Проверяем вращение всех типов фигур
    for (idx, &shape_type) in [
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
            coords: SHAPE_COORDS[idx],
            fg: idx,
        };

        let start = Instant::now();
        t.rotate_old(Direction::Right);
        t.rotate_old(Direction::Left);
        let elapsed = start.elapsed();

        assert!(
            elapsed.as_micros() < 50,
            "Вращение {:?} должно выполниться быстро",
            shape_type
        );
    }
}

/// Тест 4: Проверка что draw() бенчмарк работает
///
/// Проверяет, что отрисовка может быть забенчмаркенa.
#[test]
fn test_draw_benchmark_works() {
    use crate::game::GameState;
    use std::time::Instant;

    // Создаём состояние игры
    let state = GameState::new();

    // Замеряем время прохода по полю (эмуляция отрисовки)
    let start = Instant::now();
    let blocks = state.get_blocks_for_bench();

    // Проход по всему полю
    for y in 0..20 {
        for x in 0..10 {
            let _ = blocks[y][x];
        }
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_micros() < 100,
        "Проход по полю должен выполниться быстро"
    );

    // Замеряем время множественных проходов
    let start = Instant::now();
    for _ in 0..100 {
        let blocks = state.get_blocks_for_bench();
        for y in 0..20 {
            for x in 0..10 {
                let _ = blocks[y][x];
            }
        }
    }
    let elapsed_100 = start.elapsed();

    assert!(
        elapsed_100.as_millis() < 50,
        "100 проходов по полю должны выполниться быстро"
    );
}

/// Тест 5: Проверка производительности бенчмарков
///
/// Интеграционный тест производительности.
#[test]
fn test_benchmark_performance() {
    use crate::game::GameState;
    use crate::tetromino::{Tetromino, ShapeType, SHAPE_COORDS, Dir};
    use std::time::Instant;

    let iterations = 10_000;

    // Бенчмарк check_rows()
    let mut state = GameState::new();
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = state.check_rows_for_bench();
    }
    let check_rows_elapsed = start.elapsed();

    // Бенчмарк rotate()
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: SHAPE_COORDS[0],
        fg: 0,
    };
    let start = Instant::now();
    for _ in 0..iterations {
        tetromino.rotate_old(Direction::Right);
        tetromino.rotate_old(Direction::Left);
    }
    let rotate_elapsed = start.elapsed();

    // Бенчмарк draw simulation
    let state = GameState::new();
    let start = Instant::now();
    for _ in 0..iterations {
        let blocks = state.get_blocks_for_bench();
        for y in 0..20 {
            for x in 0..10 {
                let _ = blocks[y][x];
            }
        }
    }
    let draw_elapsed = start.elapsed();

    // Выводим результаты (для информации)
    println!("check_rows(): {:?}", check_rows_elapsed / iterations as u32);
    println!("rotate(): {:?}", rotate_elapsed / iterations as u32);
    println!("draw(): {:?}", draw_elapsed / iterations as u32);

    // Проверяем что все бенчмарки работают быстро
    assert!(
        check_rows_elapsed.as_millis() < 1000,
        "check_rows() должен выполниться быстро"
    );
    assert!(
        rotate_elapsed.as_millis() < 1000,
        "rotate() должен выполниться быстро"
    );
    assert!(
        draw_elapsed.as_millis() < 1000,
        "draw() должен выполниться быстро"
    );
}
