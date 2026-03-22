//! бенчмарки для Tetris CLI.
//!
//! Этот модуль содержит бенчмарки для проверки производительности
//! ключевых функций игры:
//! - check_rows() - проверка и удаление заполненных линий
//! - find_full_rows() - поиск заполненных линий
//! - rotate() - вращение фигур
//! - save_tetromino() - сохранение фигуры в поле
//!
//! ## Примечание
//! Бенчмарки доступны только при включённой фиче `bench`.
//! Запуск: `cargo bench --features bench`

use criterion::{black_box, BenchmarkGroup, Criterion};
use tetris_cli::game::GameState;
use tetris_cli::tetromino::{RotationDirection, ShapeType, Tetromino};

/// Главная функция для запуска бенчмарков.
///
/// Регистрирует все группы бенчмарков и запускает их.
fn main() {
    let mut c = Criterion::default();

    // Запускаем все группы бенчмарков
    bench_find_full_rows(&mut c);
    bench_check_rows(&mut c);
    bench_rotate(&mut c);
    bench_save_tetromino(&mut c);
}

/// Бенчмарк для find_full_rows().
///
/// Проверяет производительность поиска заполненных линий
/// на различных состояниях поля.
fn bench_find_full_rows(c: &mut Criterion) {
    let mut group: BenchmarkGroup<'_, _> = c.benchmark_group("find_full_rows");

    // Пустое поле
    group.bench_function("empty_field", |b| {
        b.iter(|| {
            let state = GameState::new();
            black_box(state);
        });
    });

    // Поле с одной заполненной линией
    group.bench_function("one_full_line", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            state.fill_line_for_bench(10);
            black_box(state);
        });
    });

    // Поле с несколькими заполненными линиями
    group.bench_function("multiple_full_lines", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            for line in [5, 10, 15, 18] {
                state.fill_line_for_bench(line);
            }
            black_box(state);
        });
    });

    group.finish();
}

/// Бенчмарк для check_rows().
///
/// Проверяет производительность удаления заполненных линий
/// с обновлением счёта и уровня.
fn bench_check_rows(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_rows");

    // Поле с одной заполненной линией
    group.bench_function("clear_one_line", |b| {
        b.iter(|| {
            let mut game_state = GameState::new();
            game_state.fill_line_for_bench(10);
            game_state.clear_lines_for_bench();
            black_box(game_state);
        });
    });

    // Поле с несколькими заполненными линиями
    group.bench_function("clear_multiple_lines", |b| {
        b.iter(|| {
            let mut game_state = GameState::new();
            for line in [5, 10, 15] {
                game_state.fill_line_for_bench(line);
            }
            game_state.clear_lines_for_bench();
            black_box(game_state);
        });
    });

    // Tetris - 4 линии одновременно
    group.bench_function("clear_tetris", |b| {
        b.iter(|| {
            let mut game_state = GameState::new();
            for line in [16, 17, 18, 19] {
                game_state.fill_line_for_bench(line);
            }
            game_state.clear_lines_for_bench();
            black_box(game_state);
        });
    });

    group.finish();
}

/// Бенчмарк для rotate().
///
/// Проверяет производительность вращения фигур
/// для разных типов фигур и направлений.
fn bench_rotate(c: &mut Criterion) {
    let mut group = c.benchmark_group("rotate");

    // Вращение T-фигуры
    group.bench_function("rotate_t_clockwise", |b| {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::T,
            coords: tetris_cli::tetromino::SHAPE_COORDS[0],
            fg: 0,
        };
        b.iter(|| {
            let mut t = tetromino;
            t.rotate(RotationDirection::Clockwise);
            black_box(t);
        });
    });

    // Вращение I-фигуры
    group.bench_function("rotate_i_clockwise", |b| {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::I,
            coords: tetris_cli::tetromino::SHAPE_COORDS[6],
            fg: 6,
        };
        b.iter(|| {
            let mut t = tetromino;
            t.rotate(RotationDirection::Clockwise);
            black_box(t);
        });
    });

    // Вращение O-фигуры (не вращается)
    group.bench_function("rotate_o_noop", |b| {
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: ShapeType::O,
            coords: tetris_cli::tetromino::SHAPE_COORDS[5],
            fg: 5,
        };
        b.iter(|| {
            let mut t = tetromino;
            t.rotate(RotationDirection::Clockwise);
            black_box(t);
        });
    });

    group.finish();
}

/// Бенчмарк для save_tetromino().
///
/// Проверяет производительность сохранения фигуры в поле.
fn bench_save_tetromino(c: &mut Criterion) {
    let mut group = c.benchmark_group("save_tetromino");

    // Сохранение T-фигуры в центре поля
    group.bench_function("save_t_center", |b| {
        b.iter(|| {
            let mut state = GameState::new();
            state.save_tetromino_for_bench();
            black_box(state);
        });
    });

    // Сохранение I-фигуры
    group.bench_function("save_i_center", |b| {
        b.iter(|| {
            let mut game_state = GameState::new();
            // Используем set_curr_shape_for_bench для установки I-фигуры
            game_state.set_curr_shape_for_bench(Tetromino {
                pos: (4.0, 0.0),
                shape: ShapeType::I,
                coords: tetris_cli::tetromino::SHAPE_COORDS[6],
                fg: 6,
            });
            game_state.save_tetromino_for_bench();
            black_box(game_state);
        });
    });

    group.finish();
}
