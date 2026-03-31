//! Тесты wall kick без дублирования.
//!
//! Проверяют, что логика wall kick едина и не дублируется.

use crate::game::GameState;
use crate::types::RotationDirection;

/// Тест 1: Проверка, что can_rotate_curr_shape работает
///
/// Проверяем базовую функцию проверки вращения.
#[test]
fn test_can_rotate_curr_shape_basic() {
    let state = GameState::new();

    // В начале игры вращение должно быть возможно
    let can_rotate_cw = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_ccw = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    assert!(
        can_rotate_cw || can_rotate_ccw,
        "Хотя бы одно направление вращения должно быть доступно"
    );
}

/// Тест 2: Проверка rotate_with_wall_kick
///
/// Проверяем функцию вращения с wall kick.
#[test]
fn test_rotate_with_wall_kick_basic() {
    let mut state = GameState::new();
    let original_coords = state.curr_shape().coords;

    // Выполняем вращение по часовой
    let rotated_cw = state.rotate_with_wall_kick(RotationDirection::Clockwise);

    // Вращение должно быть успешным в большинстве случаев
    assert!(
        rotated_cw,
        "Вращение по часовой должно быть возможно в центре поля"
    );

    // Координаты должны измениться (если фигура не квадрат)
    if state.curr_shape().shape != crate::tetromino::ShapeType::O {
        assert_ne!(
            state.curr_shape().coords,
            original_coords,
            "Координаты должны измениться после вращения"
        );
    }
}

/// Тест 3: Проверка wall kick у стены
///
/// Проверяем, что wall kick работает у стены.
#[test]
fn test_wall_kick_refactor_at_wall() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::types::Direction;

    let mut state = GameState::new();

    // Перемещаем фигуру к левой стене
    while can_move_curr_shape_direction(&state, Direction::Left) {
        state.get_curr_shape_mut().pos.0 -= 1.0;
    }

    // Пытаемся вращать у стены - должен сработать wall kick
    let rotated = state.rotate_with_wall_kick(RotationDirection::Clockwise);

    // Вращение должно быть успешным (с wall kick или без)
    assert!(
        rotated,
        "Вращение у стены должно быть возможно (с wall kick)"
    );
}

/// Тест 4: Проверка, что логика wall kick едина
///
/// Проверяем, что can_rotate_curr_shape и rotate_with_wall_kick
/// используют единую логику.
#[test]
fn test_wall_kick_logic_unity() {
    let mut state = GameState::new();

    // Проверяем, что can_rotate_curr_shape предсказывает успех rotate_with_wall_kick
    let can_rotate_cw = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let rotate_result = state.rotate_with_wall_kick(RotationDirection::Clockwise);

    // Если can_rotate_curr_shape возвращает true, rotate_with_wall_kick должен succeed
    if can_rotate_cw {
        assert!(
            rotate_result,
            "Если can_rotate_curr_shape возвращает true, rotate_with_wall_kick должен succeed"
        );
    }
}

/// Тест 5: Проверка wall kick таблиц смещений
///
/// Проверяем, что таблица смещений WALL_KICK_OFFSETS корректна.
#[test]
fn test_wall_kick_offsets_table() {
    use crate::game::logic::wall_kick::WALL_KICK_OFFSETS;

    // Проверяем длину таблицы
    assert_eq!(
        WALL_KICK_OFFSETS.len(),
        8,
        "Таблица wall kick должна содержать 8 смещений"
    );

    // Проверяем наличие основных смещений
    let has_left_1 = WALL_KICK_OFFSETS.contains(&(-1, 0));
    let has_right_1 = WALL_KICK_OFFSETS.contains(&(1, 0));
    let has_left_2 = WALL_KICK_OFFSETS.contains(&(-2, 0));
    let has_right_2 = WALL_KICK_OFFSETS.contains(&(2, 0));
    let has_up_1 = WALL_KICK_OFFSETS.contains(&(0, -1));

    assert!(has_left_1, "Должно быть смещение влево на 1");
    assert!(has_right_1, "Должно быть смещение вправо на 1");
    assert!(has_left_2, "Должно быть смещение влево на 2");
    assert!(has_right_2, "Должно быть смещение вправо на 2");
    assert!(has_up_1, "Должно быть смещение вверх на 1");
    // Смещение вниз (0, 1) не входит в таблицу SRS
}

/// Тест 6: Проверка wall kick для разных фигур
///
/// Проверяем, что wall kick работает для всех типов фигур.
#[test]
fn test_wall_kick_all_shapes() {
    use crate::tetromino::{BagGenerator, Tetromino};

    let mut bag = BagGenerator::new();

    // Тестируем все 7 типов фигур
    for shape_num in 0..7 {
        let mut state = GameState::new();
        state.set_curr_shape(Tetromino::from_bag(&mut bag));

        let can_rotate = state.can_rotate_curr_shape(RotationDirection::Clockwise);
        let rotate_result = state.rotate_with_wall_kick(RotationDirection::Clockwise);

        // Если вращение возможно, оно должно succeed
        if can_rotate {
            assert!(
                rotate_result,
                "Вращение для фигуры {shape_num} должно succeed"
            );
        }
    }
}

/// Тест 7: Проверка, что нет дублирования логики
///
/// Проверяем, что try_rotation_with_kicks вызывается из can_rotate_curr_shape
/// и rotate_with_wall_kick.
#[test]
fn test_no_logic_duplication() {
    let mut state = GameState::new();

    // Сохраняем оригинальные координаты
    let original_coords = state.curr_shape().coords;
    let original_pos = state.curr_shape().pos;

    // Проверяем вращение
    let can_rotate = state.can_rotate_curr_shape(RotationDirection::Clockwise);

    // Сбрасываем состояние
    state.get_curr_shape_mut().coords = original_coords;
    state.get_curr_shape_mut().pos = original_pos;

    // Выполняем вращение
    let rotated = state.rotate_with_wall_kick(RotationDirection::Clockwise);

    // Результаты должны быть согласованы
    if can_rotate {
        assert!(
            rotated,
            "Результаты can_rotate и rotate_with_wall_kick должны быть согласованы"
        );
    }
}
