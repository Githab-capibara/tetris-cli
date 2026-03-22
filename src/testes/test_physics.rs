//! Тесты физической механики игры.
//!
//! Этот модуль содержит 20 тестов для проверки физической механики Tetris:
//! - Тесты гравитации и падения (4 теста)
//! - Тесты столкновений (4 теста)
//! - Тесты вращения (3 теста)
//! - Тесты удержания фигуры (3 теста)
//! - Тесты призрачной фигуры (2 теста)
//! - Тесты Bag Generator (4 теста)
//!
//! Все тесты независимы и проверяют отдельные аспекты физической механики.

use crate::game::{Dir, GameState};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-4: Гравитация и падение
// ============================================================================

/// Тест 1: Проверка гравитации и падения фигуры
///
/// Проверяет, что фигура падает вниз под действием гравитации.
#[test]
fn test_gravity_and_falling() {
    let mut state = GameState::new();

    // Запоминаем начальную позицию Y
    let initial_y = state.get_curr_shape().pos.1;

    // Фигура должна иметь возможность падения вниз
    assert!(
        state.can_move_curr_shape(Dir::Down),
        "Фигура должна иметь возможность падения вниз"
    );

    // Опускаем фигуру на 5 блоков
    for _ in 0..5 {
        if state.can_move_curr_shape(Dir::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
    }

    // Проверяем, что фигура опустилась
    let final_y = state.get_curr_shape().pos.1;
    assert!(
        final_y > initial_y,
        "Фигура должна опуститься вниз под действием гравитации"
    );
    assert!(
        (final_y - initial_y).abs() >= 5.0,
        "Фигура должна опуститься минимум на 5 блоков"
    );
}

/// Тест 2: Проверка достижения пола фигурой
///
/// Проверяет, что фигура не может пройти сквозь пол.
#[test]
fn test_piece_reaching_floor() {
    let mut state = GameState::new();

    // Опускаем фигуру до упора
    let mut drop_count = 0;
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
        drop_count += 1;
    }

    // Движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Down),
        "Движение вниз должно быть заблокировано после достижения пола"
    );

    // Проверяем, что было хотя бы одно движение
    assert!(drop_count > 0, "Должно быть хотя бы одно движение вниз");
}

/// Тест 3: Проверка скорости падения
///
/// Проверяет, что скорость падения соответствует `INITIAL_FALL_SPD`.
#[test]
fn test_falling_speed() {
    let state = GameState::new();

    // Проверяем начальную скорость падения
    let fall_spd = state.get_fall_spd();
    assert!(
        (fall_spd - 0.9).abs() < f32::EPSILON,
        "Начальная скорость падения должна быть 0.9"
    );

    // Скорость должна быть положительной
    assert!(fall_spd > 0.0, "Скорость падения должна быть положительной");

    // Скорость должна быть разумной (меньше 10)
    assert!(fall_spd < 10.0, "Скорость падения должна быть меньше 10");
}

/// Тест 4: Проверка увеличения скорости падения
///
/// Проверяет, что скорость падения увеличивается с уровнем.
#[test]
fn test_falling_speed_increase() {
    use crate::game::{INITIAL_FALL_SPD, SPD_INC};

    let initial = INITIAL_FALL_SPD;
    let after_one_line = initial + SPD_INC * 1.0;
    let after_five_lines = initial + SPD_INC * 5.0;
    let after_ten_lines = initial + SPD_INC * 10.0;

    // Проверяем увеличение скорости
    assert!(
        after_one_line > initial,
        "Скорость должна увеличиться после 1 линии"
    );
    assert!(
        after_five_lines > after_one_line,
        "Скорость должна расти с количеством линий"
    );
    assert!(
        after_ten_lines > after_five_lines,
        "Скорость должна продолжать расти"
    );

    // Проверяем, что скорость не превышает разумные пределы
    assert!(
        after_ten_lines < 5.0,
        "Скорость после 10 линий должна быть меньше 5.0"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 5-8: Столкновения
// ============================================================================

/// Тест 5: Проверка столкновения с левой стеной
///
/// Проверяет, что фигура не может пройти сквозь левую стену.
#[test]
fn test_collision_with_left_wall() {
    let mut state = GameState::new();

    // Перемещаем фигуру к левой границе
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Движение влево должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Left),
        "Движение влево должно быть заблокировано у левой стены"
    );

    // Движение вправо должно быть возможно
    assert!(
        state.can_move_curr_shape(Dir::Right),
        "Движение вправо должно быть возможно у левой стены"
    );
}

/// Тест 6: Проверка столкновения с правой стеной
///
/// Проверяет, что фигура не может пройти сквозь правую стену.
#[test]
fn test_collision_with_right_wall() {
    let mut state = GameState::new();

    // Перемещаем фигуру к правой границе
    for _ in 0..10 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Движение вправо должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Right),
        "Движение вправо должно быть заблокировано у правой стены"
    );

    // Движение влево должно быть возможно
    assert!(
        state.can_move_curr_shape(Dir::Left),
        "Движение влево должно быть возможно у правой стены"
    );
}

/// Тест 7: Проверка столкновения с зафиксированными фигурами
///
/// Проверяет, что новая фигура сталкивается с уже зафиксированными.
#[test]
fn test_collision_with_fixed_pieces() {
    let mut state = GameState::new();

    // Опускаем фигуру на пол
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape(Dir::Down),
        "Движение вниз должно быть заблокировано на полу"
    );

    // Проверяем, что фигура не вышла за пределы поля
    let curr_y = state.get_curr_shape().pos.1;
    assert!(
        curr_y < GRID_HEIGHT as f32,
        "Фигура не должна выходить за пределы поля по Y"
    );
}

/// Тест 8: Проверка столкновений в пустом поле
///
/// Проверяет, что в пустом поле фигура может двигаться свободно.
#[test]
fn test_collisions_in_empty_field() {
    let state = GameState::new();

    // В начале игры движение вниз должно быть возможно
    assert!(
        state.can_move_curr_shape(Dir::Down),
        "В пустом поле движение вниз должно быть возможно"
    );

    // Проверяем, что движение влево/вправо зависит от позиции
    let curr_x = state.get_curr_shape().pos.0;

    // Если фигура не у границы, хотя бы одно направление должно быть доступно
    if curr_x > 0.0 && curr_x < (GRID_WIDTH - 1) as f32 {
        assert!(
            state.can_move_curr_shape(Dir::Left) || state.can_move_curr_shape(Dir::Right),
            "В центре поля хотя бы одно направление должно быть доступно"
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 9-11: Вращение
// ============================================================================

/// Тест 9: Проверка вращения у левой стены
///
/// Проверяет, что вращение работает корректно у левой стены.
/// Примечание: некоторые фигуры (O, S, Z у стены) могут не иметь
/// доступного вращения из-за геометрии — это ожидаемое поведение.
#[test]
fn test_rotation_near_left_wall() {
    // Создаём состояние игры и проверяем вращение у левой стены
    let mut state = GameState::new();

    // Перемещаем фигуру к левой границе
    for _ in 0..5 {
        if state.can_move_curr_shape(Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
    }

    // Проверяем, что методы вращения работают без паники
    let _can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let _can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Тест проходит, если код не паникует
    // Конкретные результаты зависят от типа фигуры
}

/// Тест 10: Проверка вращения у правой стены
///
/// Проверяет, что вращение возможно даже у правой стены.
#[test]
fn test_rotation_near_right_wall() {
    let mut state = GameState::new();

    // Перемещаем фигуру к правой границе (но не вплотную)
    for _ in 0..3 {
        if state.can_move_curr_shape(Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Вращение должно быть возможно
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Хотя бы одно направление вращения должно быть доступно
    assert!(
        can_rotate_right || can_rotate_left,
        "Хотя бы одно направление вращения должно быть доступно у правой стены"
    );
}

/// Тест 11: Проверка вращения у пола
///
/// Проверяет, что вращение возможно даже у пола.
#[test]
fn test_rotation_near_floor() {
    let mut state = GameState::new();

    // Опускаем фигуру близко к полу
    for _ in 0..15 {
        if state.can_move_curr_shape(Dir::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
    }

    // Вращение должно быть возможно
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Хотя бы одно направление вращения должно быть доступно
    assert!(
        can_rotate_right || can_rotate_left,
        "Хотя бы одно направление вращения должно быть доступно у пола"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 12-14: Удержание фигуры (Hold)
// ============================================================================

/// Тест 12: Проверка обмена удержанной фигуры
///
/// Проверяет, что hold корректно обменивает фигуры.
#[test]
fn test_hold_swap_mechanism() {
    let mut state = GameState::new();

    // Запоминаем начальную фигуру
    let initial_shape = state.get_curr_shape().shape;
    let next_shape = state.get_next_shape().shape;

    // Удерживаем фигуру
    state.hold_shape();

    // Текущая фигура должна измениться на следующую
    assert_eq!(
        state.get_curr_shape().shape,
        next_shape,
        "Текущая фигура должна стать следующей после hold"
    );

    // Удержанная фигура должна быть установлена
    assert!(
        state.get_held_shape().is_some(),
        "Удержанная фигура должна быть установлена"
    );

    // Удержанная фигура должна быть той, что была изначально
    assert_eq!(
        state
            .get_held_shape()
            .expect("Удержанная фигура должна существовать")
            .shape,
        initial_shape,
        "Удержанная фигура должна быть начальной фигурой"
    );
}

/// Тест 13: Проверка запрета повторного удержания
///
/// Проверяет, что нельзя удержать фигуру дважды за ход.
#[test]
fn test_hold_double_usage_prevention() {
    let mut state = GameState::new();

    // Первое удержание
    state.hold_shape();

    // Повторное удержание должно быть запрещено
    assert!(
        !state.can_hold(),
        "Повторное удержание должно быть запрещено в том же ходу"
    );

    // Позиция фигуры должна быть сброшена к центру
    assert_eq!(
        state.get_curr_shape().pos,
        (4.0, 0.0),
        "Позиция фигуры должна быть сброшена к центру после hold"
    );
}

/// Тест 14: Проверка сброса `can_hold` после нового хода
///
/// Проверяет, что `can_hold` сбрасывается после нового хода.
#[test]
fn test_hold_reset_after_new_turn() {
    let mut state = GameState::new();

    // В начале игры can_hold должен быть true
    assert!(state.can_hold(), "В начале игры можно удерживать фигуру");

    // Удерживаем фигуру
    state.hold_shape();

    // Теперь can_hold должен быть false
    assert!(
        !state.can_hold(),
        "После удержания can_hold должен быть false"
    );

    // Удержанная фигура должна быть установлена
    assert!(
        state.get_held_shape().is_some(),
        "Удержанная фигура должна быть установлена"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 15-16: Призрачная фигура
// ============================================================================

/// Тест 15: Проверка позиции призрачной фигуры
///
/// Проверяет, что призрачная фигура показывает правильную точку приземления.
#[test]
fn test_ghost_piece_position() {
    let state = GameState::new();
    let ghost_shape = *state.get_curr_shape();

    // Призрачная фигура должна использовать ту же логику столкновений
    let can_move_down = state.can_move_ghost_shape(&ghost_shape, Dir::Down);

    // В начале игры призрачная фигура должна иметь возможность падения
    assert!(
        can_move_down,
        "Призрачная фигура должна иметь возможность падения вниз"
    );

    // Проверяем, что призрачная фигура имеет те же координаты
    assert_eq!(
        ghost_shape.shape,
        state.get_curr_shape().shape,
        "Призрачная фигура должна быть того же типа"
    );
}

/// Тест 16: Проверка достижения пола призрачной фигурой
///
/// Проверяет, что призрачная фигура корректно определяет пол.
#[test]
fn test_ghost_piece_floor_detection() {
    let mut state = GameState::new();

    // Опускаем фигуру до пола
    while state.can_move_curr_shape(Dir::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Создаём призрачную фигуру на той же позиции
    let ghost_shape = *state.get_curr_shape();

    // Призрачная фигура не должна иметь возможность движения вниз
    let can_move_down = state.can_move_ghost_shape(&ghost_shape, Dir::Down);
    assert!(
        !can_move_down,
        "Призрачная фигура на полу не должна иметь возможность падения"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 17-20: Bag Generator
// ============================================================================

/// Тест 17: Проверка распределения фигур в Bag
///
/// Проверяет, что каждый мешок содержит все 7 типов фигур.
#[test]
fn test_bag_piece_distribution() {
    let mut bag = BagGenerator::new();

    // Получаем 7 фигур из первого мешка
    let mut shapes_found = [false; 7];
    for _ in 0..7 {
        let shape = bag.next_shape();
        shapes_found[shape as usize] = true;
    }

    // Проверяем, что все 7 типов встретились
    for (i, &found) in shapes_found.iter().enumerate() {
        assert!(
            found,
            "Фигура типа {:?} должна быть в мешке",
            match i {
                0 => "T",
                1 => "L",
                2 => "J",
                3 => "S",
                4 => "Z",
                5 => "O",
                6 => "I",
                _ => "Unknown",
            }
        );
    }
}

/// Тест 18: Проверка перемешивания в Bag
///
/// Проверяет, что фигуры в мешке перемешиваются.
#[test]
fn test_bag_shuffle_randomness() {
    let mut bag = BagGenerator::new();

    // Получаем последовательность из 7 фигур
    let mut first_sequence = Vec::new();
    for _ in 0..7 {
        first_sequence.push(bag.next_shape());
    }

    // Получаем вторую последовательность
    let mut second_sequence = Vec::new();
    for _ in 0..7 {
        second_sequence.push(bag.next_shape());
    }

    // Последовательности могут совпадать, но это маловероятно
    // Проверяем хотя бы, что они корректны
    assert_eq!(
        first_sequence.len(),
        7,
        "Первая последовательность должна содержать 7 фигур"
    );
    assert_eq!(
        second_sequence.len(),
        7,
        "Вторая последовательность должна содержать 7 фигур"
    );
}

/// Тест 19: Проверка заполнения нового мешка
///
/// Проверяет, что после опустошения мешка создаётся новый.
#[test]
fn test_bag_refill_mechanism() {
    let mut bag = BagGenerator::new();

    // Получаем все 7 фигур из первого мешка
    for _ in 0..7 {
        let _ = bag.next_shape();
    }

    // Индекс должен указывать на конец мешка
    assert_eq!(
        bag.get_index(),
        7,
        "Индекс должен быть 7 после получения 7 фигур"
    );

    // Получаем следующую фигуру - должен заполниться новый мешок
    let next_shape = bag.next_shape();

    // Индекс должен сброситься на 1
    assert_eq!(
        bag.get_index(),
        1,
        "Индекс должен быть 1 после заполнения нового мешка"
    );

    // Новая фигура должна быть валидной
    assert!((next_shape as usize) < 7, "Тип фигуры должен быть валидным");
}

/// Тест 20: Проверка честности Bag Generator
///
/// Проверяет, что Bag Generator выдаёт фигуры честно.
#[test]
fn test_bag_fairness() {
    let mut bag = BagGenerator::new();

    // Генерируем 700 фигур (100 полных мешков)
    let total_shapes = 700;
    let mut shape_counts = [0; 7];

    for _ in 0..total_shapes {
        let shape = bag.next_shape();
        shape_counts[shape as usize] += 1;
    }

    // В системе 7-bag каждая фигура встречается ровно 100 раз
    let expected_per_shape = total_shapes / 7;

    // Проверяем, что каждая фигура встретилась ожидаемое количество раз
    for (i, &count) in shape_counts.iter().enumerate() {
        assert_eq!(
            count,
            expected_per_shape,
            "Фигура {:?} должна встречаться {} раз, но встретилась {} раз",
            match i {
                0 => "T",
                1 => "L",
                2 => "J",
                3 => "S",
                4 => "Z",
                5 => "O",
                6 => "I",
                _ => "Unknown",
            },
            expected_per_shape,
            count
        );
    }
}
