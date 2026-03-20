//! Бенчмарки для основных функций игры Tetris CLI.
//!
//! Использует criterion для измерения производительности:
//! - check_rows() - проверка и удаление заполненных линий
//! - draw() - отрисовка игрового поля
//! - rotate() - вращение фигур
//!
//! ## Запуск бенчмарков
//! ```bash
//! cargo bench
//! ```
//!
//! ## Отчёт
//! Criterion генерирует HTML-отчёт в `target/criterion/report/index.html`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tetris_cli::game::Dir;
use tetris_cli::game::GameState;
use tetris_cli::tetromino::{ShapeType, Tetromino};

/// Бенчмарк для check_rows().
///
/// Измеряет производительность проверки и удаления заполненных линий.
/// Тестирует на различных конфигурациях поля:
/// - Пустое поле
/// - Поле с 1 заполненной линией
/// - Поле с 4 заполненными линиями (Tetris)
fn bench_check_rows(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_rows");

    // Тест 1: Пустое поле - создание GameState
    group.bench_function("empty_field", |b| {
        b.iter(|| {
            let _state = GameState::new();
        })
    });

    // Тест 2: Поле с заполненными линиями
    group.bench_function("with_lines", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            // Заполняем несколько линий для симуляции реальной игры
            for y in 10..14 {
                state.fill_line_for_bench(y);
            }
            state.clear_lines_for_bench();
        })
    });

    group.finish();
}

/// Бенчмарк для rotate().
///
/// Измеряет производительность вращения фигур.
/// Тестирует все 7 типов фигур в обоих направлениях.
fn bench_rotate(c: &mut Criterion) {
    let mut group = c.benchmark_group("rotate");

    // Тест для всех типов фигур
    for shape_type in [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ] {
        group.bench_function(format!("rotate_{:?}", shape_type), |b| {
            b.iter(|| {
                let mut tetromino = Tetromino {
                    pos: (4.0, 0.0),
                    shape: shape_type,
                    coords: tetris_cli::tetromino::SHAPE_COORDS[shape_type as usize],
                    fg: shape_type as usize,
                };
                // Вращение по часовой
                tetromino.rotate(black_box(Dir::Right));
                // Вращение против часовой
                tetromino.rotate(black_box(Dir::Left));
            })
        });
    }

    group.finish();
}

/// Бенчмарк для отрисовки (эмуляция draw()).
///
/// Измеряет производительность прохода по полю при отрисовке.
fn bench_draw_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_simulation");

    // Тест: проход по всему полю (эмуляция отрисовки)
    group.bench_function("field_iteration", |b| {
        b.iter(|| {
            let state = GameState::new();
            let blocks = state.get_blocks_for_bench();

            // Эмуляция отрисовки - проход по всему полю
            for y in 0..20 {
                for x in 0..10 {
                    black_box(blocks[y][x]);
                }
            }
        })
    });

    group.finish();
}

/// Бенчмарк для check_collision().
///
/// Измеряет производительность проверки столкновений фигуры.
/// Тестирует различные сценарии:
/// - Движение в пустом поле
/// - Движение рядом с границей
/// - Движение рядом с зафиксированными фигурами
fn bench_check_collision(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_collision");

    // Тест 1: Движение в пустом поле
    group.bench_function("empty_field", |b| {
        b.iter(|| {
            let state = GameState::new();
            // Проверка движения вниз в пустом поле
            state.can_move_curr_shape(black_box(Dir::Down));
        })
    });

    // Тест 2: Движение влево/вправо
    group.bench_function("lateral_movement", |b| {
        b.iter(|| {
            let state = GameState::new();
            // Проверка движения влево и вправо
            state.can_move_curr_shape(black_box(Dir::Left));
            state.can_move_curr_shape(black_box(Dir::Right));
        })
    });

    // Тест 3: Движение с зафиксированными фигурами
    group.bench_function("with_locked_pieces", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            // Заполняем нижние линии для симуляции зафиксированных фигур
            for y in 18..20 {
                state.fill_line_for_bench(y);
            }
            // Проверка столкновения с зафиксированными фигурами
            state.can_move_curr_shape(black_box(Dir::Down));
        })
    });

    group.finish();
}

/// Бенчмарк для save_tetromino().
///
/// Измеряет производительность сохранения фигуры в поле.
/// Тестирует различные сценарии:
/// - Сохранение фигуры в пустом поле
/// - Сохранение фигуры над зафиксированными фигурами
fn bench_save_tetromino(c: &mut Criterion) {
    let mut group = c.benchmark_group("save_tetromino");

    // Тест 1: Сохранение в пустом поле
    group.bench_function("empty_field", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            // Опускаем фигуру вниз и сохраняем
            while state.can_move_curr_shape(Dir::Down) {
                state.get_curr_shape_mut().pos.1 += 1.0;
            }
            state.save_tetromino();
        })
    });

    // Тест 2: Сохранение над зафиксированными фигурами
    group.bench_function("above_locked_pieces", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            // Заполняем нижние линии
            for y in 15..20 {
                state.fill_line_for_bench(y);
            }
            // Опускаем фигуру и сохраняем
            while state.can_move_curr_shape(Dir::Down) {
                state.get_curr_shape_mut().pos.1 += 1.0;
            }
            state.save_tetromino();
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_check_rows,
    bench_rotate,
    bench_draw_simulation,
    bench_check_collision,
    bench_save_tetromino
);
criterion_main!(benches);
