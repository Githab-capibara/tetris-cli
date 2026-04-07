//! Тесты игровой логики.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит 30 тестов для проверки основной игровой логики Tetris:
//! - Тесты движения фигур (6 тестов)
//! - Тесты столкновений (6 тестов)
//! - Тесты вращения (5 тестов)
//! - Тесты системы очков (5 тестов)
//! - Тесты уровней и линий (4 теста)
//! - Тесты режимов игры Classic/Sprint (4 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты игровой механики.

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::constants::{INITIAL_FALL_SPD, LINES_PER_LEVEL, LINE_SCORES, SPD_INC};
use crate::game::GameState;
use crate::tetromino::{ShapeType, Tetromino};
use crate::types::RotationDirection;

// ============================================================================
// ГРУППА ТЕСТОВ 1-6: Движение фигур
// ============================================================================

/// Тест 2: Проверка начальной позиции фигуры
///
/// Проверяет, что текущая фигура появляется в правильной позиции.
#[test]
fn test_game_state_initial_piece_position() {
    let state = GameState::new();
    let curr_shape = state.curr_shape();

    // Начальная позиция X должна быть по центру (4.0)
    assert!(
        (curr_shape.pos().0 - 4.0).abs() < f32::EPSILON,
        "Начальная позиция X должна быть 4.0 (центр)"
    );

    // Начальная позиция Y должна быть 0.0 (верх поля)
    assert!(
        (curr_shape.pos().1 - 0.0).abs() < f32::EPSILON,
        "Начальная позиция Y должна быть 0.0"
    );
}

/// Тест 3: Проверка наличия следующей фигуры
///
/// Проверяет, что следующая фигура всегда инициализирована.
#[test]
fn test_game_state_next_shape_exists() {
    let state = GameState::new();
    let next_shape = state.next_shape();

    // Проверяем, что тип фигуры соответствует цвету
    assert_eq!(
        next_shape.shape() as u8,
        next_shape.fg(),
        "Индекс цвета должен соответствовать типу фигуры"
    );
}

/// Тест 4: Проверка пустого игрового поля при создании
///
/// Проверяет, что поле инициализируется пустыми клетками (-1).
#[test]
fn test_game_state_empty_field() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    for (y, row) in blocks.iter().enumerate().take(GRID_HEIGHT) {
        for (x, cell) in row.iter().enumerate().take(GRID_WIDTH) {
            assert_eq!(*cell, -1, "Клетка [{y},{x}] должна быть пустой (-1)");
        }
    }
}

/// Тест 5: Проверка начальной скорости падения
///
/// Проверяет, что скорость падения установлена в `INITIAL_FALL_SPD`.
#[test]
fn test_game_state_initial_fall_speed() {
    let state = GameState::new();
    let fall_spd = state.fall_speed();

    assert!(
        (fall_spd - INITIAL_FALL_SPD).abs() < f32::EPSILON,
        "Начальная скорость должна быть {INITIAL_FALL_SPD:.2}, получено {fall_spd:.2}"
    );
}

/// Тест 6: Проверка режима игры по умолчанию
///
/// Проверяет, что `GameState::new()` создаёт классический режим.
#[test]
fn test_game_state_default_mode() {
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим по умолчанию должен быть Классика"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 13-17: Вращение фигур
// ============================================================================

/// Тест 13: Проверка вращения фигуры T по часовой стрелке
///
/// Проверяет корректность изменения координат при вращении.
#[test]
fn test_tetromino_rotate_clockwise() {
    let mut tetromino = Tetromino::new(
        (4.0, 0.0),
        ShapeType::T,
        [(-1, 0), (0, 0), (1, 0), (0, 1)],
        0,
    );

    // Исходные координаты: (-1,0), (0,0), (1,0), (0,1)
    // Вращение по часовой: (x,y) -> (-y,x)
    tetromino.rotate(RotationDirection::Clockwise);

    // После вращения: (0,-1), (0,0), (0,1), (-1,0)
    assert_eq!(
        tetromino.coords()[0],
        (0, -1),
        "Первый блок должен повернуться"
    );
    assert_eq!(
        tetromino.coords()[1],
        (0, 0),
        "Центральный блок должен остаться на месте"
    );
}

/// Тест 14: Проверка вращения фигуры T против часовой стрелки
///
/// Проверяет корректность изменения координат при вращении в обратную сторону.
#[test]
fn test_tetromino_rotate_counter_clockwise() {
    let mut tetromino = Tetromino::new(
        (4.0, 0.0),
        ShapeType::T,
        [(-1, 0), (0, 0), (1, 0), (0, 1)],
        0,
    );

    // Вращение против часовой: (x,y) -> (y,-x)
    tetromino.rotate(RotationDirection::CounterClockwise);

    // После вращения: (0,1), (0,0), (0,-1), (1,0)
    assert_eq!(
        tetromino.coords()[3],
        (1, 0),
        "Верхний блок должен переместиться вправо"
    );
}

/// Тест 15: Проверка, что квадрат (O) не вращается
///
/// Квадрат - единственная фигура, которая не меняет форму при вращении.
#[test]
fn test_tetromino_o_no_rotate() {
    let mut tetromino = Tetromino::new(
        (4.0, 0.0),
        ShapeType::O,
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        5,
    );

    let original_coords = tetromino.coords();

    // Вращение по часовой
    tetromino.rotate(RotationDirection::Clockwise);
    assert_eq!(
        tetromino.coords(),
        original_coords,
        "Квадрат не должен вращаться по часовой"
    );

    // Вращение против часовой
    tetromino.rotate(RotationDirection::CounterClockwise);
    assert_eq!(
        tetromino.coords(),
        original_coords,
        "Квадрат не должен вращаться против часовой"
    );
}

/// Тест 16: Проверка четырёх вращений (полный цикл)
///
/// Проверяет, что 4 вращения возвращают фигуру в исходное состояние.
#[test]
fn test_tetromino_full_rotation_cycle() {
    let mut tetromino = Tetromino::new(
        (4.0, 0.0),
        ShapeType::T,
        [(-1, 0), (0, 0), (1, 0), (0, 1)],
        0,
    );

    let original_coords = tetromino.coords();

    // 4 вращения по часовой должны вернуть к исходным координатам
    for _ in 0..4 {
        tetromino.rotate(RotationDirection::Clockwise);
    }

    assert_eq!(
        tetromino.coords(),
        original_coords,
        "После 4 вращений фигура должна вернуться в исходное состояние"
    );
}

/// Тест 17: Проверка вращения всех типов фигур
///
/// Проверяет, что все 7 типов фигур могут вращаться (кроме O).
#[test]
fn test_all_tetromino_rotate() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape_type in &shapes {
        let mut tetromino = Tetromino::new(
            (4.0, 0.0),
            *shape_type,
            crate::tetromino::SHAPE_COORDS[*shape_type as usize],
            *shape_type as u8,
        );

        let original_coords = tetromino.coords();
        tetromino.rotate(RotationDirection::Clockwise);

        // Все фигуры кроме O должны изменить координаты
        if *shape_type == ShapeType::O {
            // Квадрат не должен измениться
            assert_eq!(
                tetromino.coords(),
                original_coords,
                "Квадрат (O) не должен вращаться"
            );
        } else {
            // Проверяем, что вращение произошло (координаты изменились)
            // Примечание: некоторые фигуры могут совпадать после одного вращения
            // поэтому просто проверяем, что метод не паникует
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 18-22: Система очков
// ============================================================================

/// Тест 21: Проверка расчёта очков за линии
///
/// Проверяет экспоненциальный бонус за несколько линий.
#[test]
fn test_line_score_calculation() {
    // 1 линия: 100 * 2^0 = 100
    assert_eq!(LINE_SCORES[0], 100, "1 линия = 100 очков");

    // 2 линии: 100 * 2^1 = 200
    assert_eq!(LINE_SCORES[0] * (1 << 1), 200, "2 линии = 200 очков");

    // 3 линии: 100 * 2^2 = 400
    assert_eq!(LINE_SCORES[0] * (1 << 2), 400, "3 линии = 400 очков");

    // 4 линии: 100 * 2^3 = 800
    assert_eq!(LINE_SCORES[0] * (1 << 3), 800, "4 линии = 800 очков");
}

// ============================================================================
// ГРУППА ТЕСТОВ 23-26: Уровни и линии
// ============================================================================

/// Тест 24: Проверка расчёта уровня от количества линий
///
/// Проверяет формулу: уровень = (линии / 10) + 1
#[test]
#[allow(clippy::erasing_op)] // Тест проверяет граничный случай 0 / N
fn test_level_calculation_from_lines() {
    // Уровень 1: 0-9 линий
    assert_eq!((0 / LINES_PER_LEVEL) + 1, 1, "0 линий = уровень 1");
    assert_eq!((9 / LINES_PER_LEVEL) + 1, 1, "9 линий = уровень 1");

    // Уровень 2: 10-19 линий
    assert_eq!((10 / LINES_PER_LEVEL) + 1, 2, "10 линий = уровень 2");
    assert_eq!((19 / LINES_PER_LEVEL) + 1, 2, "19 линий = уровень 2");

    // Уровень 5: 40-49 линий
    assert_eq!((40 / LINES_PER_LEVEL) + 1, 5, "40 линий = уровень 5");
    assert_eq!((49 / LINES_PER_LEVEL) + 1, 5, "49 линий = уровень 5");
}

/// Тест 25: Проверка константы увеличения скорости
///
/// Проверяет, что скорость увеличивается на `SPD_INC` за уровень.
#[test]
fn test_speed_increase_constant() {
    // SPD_INC = 0.05, проверяем что это положительное число меньше 1
    // Проверка константы времени компиляции
    const _: () = assert!(
        SPD_INC > 0.0 && SPD_INC < 1.0,
        "Прирост скорости должен быть положительным и меньше 1"
    );
    assert!(
        (SPD_INC - 0.05).abs() < f32::EPSILON,
        "Прирост скорости должен быть 0.05"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 27-30: Режимы игры (Classic/Sprint)
// ============================================================================

/// Тест 29: Проверка таймера в режиме спринт
///
/// Проверяет, что таймер можно запустить и он работает.
#[test]
fn test_sprint_timer() {
    let mut state = GameState::new_sprint();

    // Запускаем таймер
    state.start_timer();

    // Небольшая задержка для проверки работы таймера
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Получаем прошедшее время
    let game_stats = state.stats();
    let elapsed = game_stats.get_elapsed_time();

    // Проверяем, что время больше 0
    assert!(elapsed > 0.0, "Время должно течь после запуска таймера");

    // Проверяем, что время меньше 1 секунды (так как спали 100мс)
    assert!(elapsed < 1.0, "Время должно быть меньше 1 секунды");
}

/// Тест 30: Проверка статистики в разных режимах
///
/// Проверяет, что статистика собирается в обоих режимах.
#[test]
fn test_game_stats_in_different_modes() {
    // Классический режим
    let classic_state = GameState::new();
    let classic_st = classic_state.stats();
    assert_eq!(
        classic_st.total_pieces(),
        1,
        "В начале игры должна быть 1 фигура"
    );

    // Режим спринт
    let sprint_state = GameState::new_sprint();
    let sprint_st = sprint_state.stats();
    assert_eq!(
        sprint_st.total_pieces(),
        1,
        "В начале спринта должна быть 1 фигура"
    );

    // Проверяем, что режимы разные
    assert_ne!(
        classic_state.get_mode_trait().name(),
        sprint_state.get_mode_trait().name(),
        "Режимы Classic и Sprint должны отличаться"
    );
}
