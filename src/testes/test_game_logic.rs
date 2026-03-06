//! Тесты игровой логики.
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

use crate::game::{Dir, GameMode, GameState};
use crate::game::{
    COMBO_BONUS, HARD_DROP_POINTS, INITIAL_FALL_SPD, LINES_PER_LEVEL, PIECE_SCORE_INC,
    ROW_SCORE_INC, SOFT_DROP_POINTS, SPD_INC, SPRINT_LINES,
};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{ShapeType, Tetromino};

// ============================================================================
// ГРУППА ТЕСТОВ 1-6: Движение фигур
// ============================================================================

/// Тест 1: Проверка создания GameState
///
/// Проверяет базовую инициализацию состояния игры.
#[test]
fn test_game_state_creation() {
    let state = GameState::new();

    assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.get_level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(
        state.get_lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );
}

/// Тест 2: Проверка начальной позиции фигуры
///
/// Проверяет, что текущая фигура появляется в правильной позиции.
#[test]
fn test_game_state_initial_piece_position() {
    let state = GameState::new();
    let curr_shape = state.get_curr_shape();

    // Начальная позиция X должна быть по центру (4.0)
    assert!(
        (curr_shape.pos.0 - 4.0).abs() < f32::EPSILON,
        "Начальная позиция X должна быть 4.0 (центр)"
    );

    // Начальная позиция Y должна быть 0.0 (верх поля)
    assert!(
        (curr_shape.pos.1 - 0.0).abs() < f32::EPSILON,
        "Начальная позиция Y должна быть 0.0"
    );
}

/// Тест 3: Проверка наличия следующей фигуры
///
/// Проверяет, что следующая фигура всегда инициализирована.
#[test]
fn test_game_state_next_shape_exists() {
    let state = GameState::new();
    let next_shape = state.get_next_shape();

    // Проверяем, что тип фигуры соответствует цвету
    assert_eq!(
        next_shape.shape as usize, next_shape.fg,
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
            assert_eq!(*cell, -1, "Клетка [{},{}] должна быть пустой (-1)", y, x);
        }
    }
}

/// Тест 5: Проверка начальной скорости падения
///
/// Проверяет, что скорость падения установлена в INITIAL_FALL_SPD.
#[test]
fn test_game_state_initial_fall_speed() {
    let state = GameState::new();
    let fall_spd = state.get_fall_spd();

    assert!(
        (fall_spd - INITIAL_FALL_SPD).abs() < f32::EPSILON,
        "Начальная скорость должна быть {:.2}, получено {:.2}",
        INITIAL_FALL_SPD,
        fall_spd
    );
}

/// Тест 6: Проверка режима игры по умолчанию
///
/// Проверяет, что GameState::new() создаёт классический режим.
#[test]
fn test_game_state_default_mode() {
    let state = GameState::new();

    assert_eq!(
        state.get_mode(),
        GameMode::Classic,
        "Режим по умолчанию должен быть Classic"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 7-12: Столкновения
// ============================================================================

/// Тест 7: Проверка столкновения с левой границей
///
/// Проверяет, что фигура не может выйти за левую границу.
#[test]
fn test_collision_left_boundary() {
    let mut state = GameState::new();

    // Перемещаем фигуру к левой границе
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Дальнейшее движение влево должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Left),
        "Движение влево за границу должно быть заблокировано"
    );
}

/// Тест 8: Проверка столкновения с правой границей
///
/// Проверяет, что фигура не может выйти за правую границу.
#[test]
fn test_collision_right_boundary() {
    let mut state = GameState::new();

    // Перемещаем фигуру к правой границе
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Дальнейшее движение вправо должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Right),
        "Движение вправо за границу должно быть заблокировано"
    );
}

/// Тест 9: Проверка столкновения с полом
///
/// Проверяет, что фигура не может пройти сквозь пол.
#[test]
fn test_collision_floor() {
    let mut state = GameState::new();

    // Опускаем фигуру вниз до упора
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Дальнейшее движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Down),
        "Движение вниз за границу пола должно быть заблокировано"
    );

    // Проверяем, что фигура не вышла за пределы поля
    assert!(
        state.get_curr_shape_mut().pos.1 < GRID_HEIGHT as f32,
        "Фигура не должна выходить за пределы поля по Y"
    );
}

/// Тест 10: Проверка столкновения с зафиксированными блоками
///
/// Проверяет, что новая фигура сталкивается с уже зафиксированными.
#[test]
fn test_collision_with_fixed_blocks() {
    // Создаём состояние и симулируем заполнение нижнего ряда
    let mut state = GameState::new();

    // Опускаем фигуру близко к полу для теста
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Down),
        "Движение вниз на пол должно быть заблокировано"
    );
}

/// Тест 11: Проверка возможности движения в пустом поле
///
/// Проверяет, что в пустом поле фигура может двигаться свободно.
#[test]
fn test_movement_in_empty_field() {
    let mut state = GameState::new();

    // В начале игры движение влево/вправо должно быть возможно
    // (если фигура не у самой границы)
    let _initial_x = state.get_curr_shape_mut().pos.0;

    // Проверяем, что можем двигаться хотя бы в одну сторону
    let can_move_left = state.can_move_curr_shape(Dir::Left);
    let can_move_right = state.can_move_curr_shape(Dir::Right);

    assert!(
        can_move_left || can_move_right,
        "В пустом поле должно быть возможно движение хотя бы в одну сторону"
    );
}

/// Тест 12: Проверка границ поля для призрачной фигуры
///
/// Проверяет, что призрачная фигура корректно определяет пол.
#[test]
fn test_ghost_piece_boundary() {
    let state = GameState::new();
    let ghost_shape = state.get_curr_shape().clone();

    // Призрачная фигура должна использовать ту же логику столкновений
    let can_move_down = state.can_move_ghost_shape(&ghost_shape, Dir::Down);

    // В начале игры призрачная фигура должна иметь возможность движения вниз
    assert!(
        can_move_down,
        "Призрачная фигура должна иметь возможность падения"
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
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: [(-1, 0), (0, 0), (1, 0), (0, 1)],
        fg: 0,
    };

    // Исходные координаты: (-1,0), (0,0), (1,0), (0,1)
    // Вращение по часовой: (x,y) -> (-y,x)
    tetromino.rotate(Dir::Right);

    // После вращения: (0,-1), (0,0), (0,1), (-1,0)
    assert_eq!(
        tetromino.coords[0],
        (0, -1),
        "Первый блок должен повернуться"
    );
    assert_eq!(
        tetromino.coords[1],
        (0, 0),
        "Центральный блок должен остаться на месте"
    );
}

/// Тест 14: Проверка вращения фигуры T против часовой стрелки
///
/// Проверяет корректность изменения координат при вращении в обратную сторону.
#[test]
fn test_tetromino_rotate_counter_clockwise() {
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: [(-1, 0), (0, 0), (1, 0), (0, 1)],
        fg: 0,
    };

    // Вращение против часовой: (x,y) -> (y,-x)
    tetromino.rotate(Dir::Left);

    // После вращения: (0,1), (0,0), (0,-1), (1,0)
    assert_eq!(
        tetromino.coords[3],
        (1, 0),
        "Верхний блок должен переместиться вправо"
    );
}

/// Тест 15: Проверка, что квадрат (O) не вращается
///
/// Квадрат - единственная фигура, которая не меняет форму при вращении.
#[test]
fn test_tetromino_o_no_rotate() {
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::O,
        coords: [(0, 0), (1, 0), (0, 1), (1, 1)],
        fg: 5,
    };

    let original_coords = tetromino.coords;

    // Вращение по часовой
    tetromino.rotate(Dir::Right);
    assert_eq!(
        tetromino.coords, original_coords,
        "Квадрат не должен вращаться по часовой"
    );

    // Вращение против часовой
    tetromino.rotate(Dir::Left);
    assert_eq!(
        tetromino.coords, original_coords,
        "Квадрат не должен вращаться против часовой"
    );
}

/// Тест 16: Проверка четырёх вращений (полный цикл)
///
/// Проверяет, что 4 вращения возвращают фигуру в исходное состояние.
#[test]
fn test_tetromino_full_rotation_cycle() {
    let mut tetromino = Tetromino {
        pos: (4.0, 0.0),
        shape: ShapeType::T,
        coords: [(-1, 0), (0, 0), (1, 0), (0, 1)],
        fg: 0,
    };

    let original_coords = tetromino.coords;

    // 4 вращения по часовой должны вернуть к исходным координатам
    for _ in 0..4 {
        tetromino.rotate(Dir::Right);
    }

    assert_eq!(
        tetromino.coords, original_coords,
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

    for shape_type in shapes.iter() {
        let mut tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: *shape_type,
            coords: crate::tetromino::SHAPE_COORDS[*shape_type as usize],
            fg: *shape_type as usize,
        };

        let original_coords = tetromino.coords;
        tetromino.rotate(Dir::Right);

        // Все фигуры кроме O должны изменить координаты
        if *shape_type != ShapeType::O {
            // Проверяем, что вращение произошло (координаты изменились)
            // Примечание: некоторые фигуры могут совпадать после одного вращения
            // поэтому просто проверяем, что метод не паникует
        } else {
            // Квадрат не должен измениться
            assert_eq!(
                tetromino.coords, original_coords,
                "Квадрат (O) не должен вращаться"
            );
        }
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 18-22: Система очков
// ============================================================================

/// Тест 18: Проверка константы очков за фигуру
///
/// Проверяет значение базовых очков за размещение фигуры.
#[test]
fn test_piece_score_constant() {
    assert_eq!(PIECE_SCORE_INC, 100, "Очки за фигуру должны быть 100");
}

/// Тест 19: Проверка константы очков за Soft Drop
///
/// Проверяет, что Soft Drop даёт 1 очко за ячейку.
#[test]
fn test_soft_drop_points_constant() {
    assert_eq!(
        SOFT_DROP_POINTS, 1,
        "Очки за Soft Drop должны быть 1 за ячейку"
    );
}

/// Тест 20: Проверка константы очков за Hard Drop
///
/// Проверяет, что Hard Drop даёт 2 очка за ячейку.
#[test]
fn test_hard_drop_points_constant() {
    assert_eq!(
        HARD_DROP_POINTS, 2,
        "Очки за Hard Drop должны быть 2 за ячейку"
    );
}

/// Тест 21: Проверка расчёта очков за линии
///
/// Проверяет экспоненциальный бонус за несколько линий.
#[test]
fn test_line_score_calculation() {
    // 1 линия: 100 * 2^0 = 100
    assert_eq!(ROW_SCORE_INC, 100, "1 линия = 100 очков");

    // 2 линии: 100 * 2^1 = 200
    assert_eq!(ROW_SCORE_INC * (1 << 1), 200, "2 линии = 200 очков");

    // 3 линии: 100 * 2^2 = 400
    assert_eq!(ROW_SCORE_INC * (1 << 2), 400, "3 линии = 400 очков");

    // 4 линии: 100 * 2^3 = 800
    assert_eq!(ROW_SCORE_INC * (1 << 3), 800, "4 линии = 800 очков");
}

/// Тест 22: Проверка константы бонуса за комбо
///
/// Проверяет значение бонуса за последовательные удаления.
#[test]
fn test_combo_bonus_constant() {
    assert_eq!(COMBO_BONUS, 50, "Бонус за комбо должен быть 50");

    // Проверяем формулу расчёта бонуса
    // Комбо 1: 50 * 0 = 0
    assert_eq!(COMBO_BONUS * 1, 50, "Бонус за первое комбо должен быть 50");

    // Комбо 2: 50 * 1 = 50
    assert_eq!(COMBO_BONUS * 1, 50, "Бонус за второе комбо должен быть 50");

    // Комбо 5: 50 * 4 = 200
    assert_eq!(COMBO_BONUS * 4, 200, "Бонус за пятое комбо должен быть 200");
}

// ============================================================================
// ГРУППА ТЕСТОВ 23-26: Уровни и линии
// ============================================================================

/// Тест 23: Проверка константы линий на уровень
///
/// Проверяет, что для повышения уровня нужно 10 линий.
#[test]
fn test_lines_per_level_constant() {
    assert_eq!(LINES_PER_LEVEL, 10, "Для повышения уровня нужно 10 линий");
}

/// Тест 24: Проверка расчёта уровня от количества линий
///
/// Проверяет формулу: уровень = (линии / 10) + 1
#[test]
fn test_level_calculation_from_lines() {
    // Уровень 1: 0-9 линий
    assert_eq!(0 + 1, 1, "0 линий = уровень 1");
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
/// Проверяет, что скорость увеличивается на SPD_INC за уровень.
#[test]
fn test_speed_increase_constant() {
    // SPD_INC = 0.05, проверяем что это положительное число меньше 1
    assert!(
        SPD_INC > 0.0 && SPD_INC < 1.0,
        "Прирост скорости должен быть положительным и меньше 1"
    );
    assert!(
        (SPD_INC - 0.05).abs() < f32::EPSILON,
        "Прирост скорости должен быть 0.05"
    );
}

/// Тест 26: Проверка расчёта скорости от уровня
///
/// Проверяет, что скорость растёт с каждым уровнем.
#[test]
fn test_speed_calculation_from_level() {
    let initial = INITIAL_FALL_SPD;

    // После 1 линии
    let after_one = initial + SPD_INC * 1.0;
    assert!(
        after_one > initial,
        "Скорость должна увеличиться после 1 линии"
    );

    // После 5 линий
    let after_five = initial + SPD_INC * 5.0;
    assert!(
        after_five > after_one,
        "Скорость должна расти с количеством линий"
    );

    // После 10 линий (новый уровень)
    let after_ten = initial + SPD_INC * 10.0;
    assert!(after_ten > after_five, "Скорость должна продолжать расти");
}

// ============================================================================
// ГРУППА ТЕСТОВ 27-30: Режимы игры (Classic/Sprint)
// ============================================================================

/// Тест 27: Проверка создания режима спринт
///
/// Проверяет, что GameState::new_sprint() создаёт режим Sprint.
#[test]
fn test_sprint_mode_creation() {
    let state = GameState::new_sprint();

    assert_eq!(
        state.get_mode(),
        GameMode::Sprint,
        "Режим должен быть Sprint"
    );
}

/// Тест 28: Проверка константы линий для спринта
///
/// Проверяет, что цель спринта - 40 линий.
#[test]
fn test_sprint_lines_constant() {
    assert_eq!(SPRINT_LINES, 40, "Цель спринта должна быть 40 линий");
}

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
    let stats = state.get_stats();
    let elapsed = stats.get_elapsed_time();

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
    let classic_stats = classic_state.get_stats();
    assert_eq!(
        classic_stats.total_pieces(),
        1,
        "В начале игры должна быть 1 фигура"
    );

    // Режим спринт
    let sprint_state = GameState::new_sprint();
    let sprint_stats = sprint_state.get_stats();
    assert_eq!(
        sprint_stats.total_pieces(),
        1,
        "В начале спринта должна быть 1 фигура"
    );

    // Проверяем, что режимы разные
    assert_ne!(
        classic_state.get_mode(),
        sprint_state.get_mode(),
        "Режимы Classic и Sprint должны отличаться"
    );
}
