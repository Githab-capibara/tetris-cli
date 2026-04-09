//! Интеграционные тесты.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит 20 интеграционных тестов для проверки
//! взаимодействия всех компонентов игры:
//! - Тесты полного игрового цикла (5 тестов)
//! - Тесты взаимодействия компонентов (8 тестов)
//! - Тесты производительности (7 тестов)
//!
//! Интеграционные тесты проверяют совместную работу модулей.

use crate::game::GameState;
use crate::types::{Direction, RotationDirection};

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Полный игровой цикл
// ============================================================================

/// Тест 1: Проверка создания и инициализации игры
///
/// Проверяет полный цикл создания `GameState`.
#[test]
fn test_full_game_initialization() {
    let state = GameState::new();

    // Проверяем все основные поля
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(state.lines_cleared(), 0, "Начальные линии должны быть 0");
    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим по умолчанию - Classic"
    );

    // Проверяем наличие фигур
    assert!(
        (state.curr_shape().shape() as usize) < 7,
        "Текущая фигура должна быть валидной"
    );
    assert!(
        (state.next_shape().shape() as usize) < 7,
        "Следующая фигура должна быть валидной"
    );
}

/// Тест 2: Проверка создания режима спринт
///
/// Проверяет инициализацию режима спринт.
#[test]
fn test_sprint_game_initialization() {
    let mut state = GameState::new_sprint();

    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Режим должен быть Sprint"
    );

    // Запускаем таймер
    state.start_timer();

    // Проверяем, что таймер работает
    std::thread::sleep(std::time::Duration::from_millis(50));
    let elapsed = state.stats().get_elapsed_time();
    assert!(elapsed > 0.0, "Таймер должен течь");
}

/// Тест 3: Проверка движения фигуры в игровом цикле
///
/// Проверяет, что фигура может двигаться в пустом поле.
#[test]
fn test_piece_movement_cycle() {
    let mut state = GameState::new();

    // Запоминаем начальную позицию
    let initial_x = state.get_curr_shape_mut().pos().0;
    let initial_y = state.get_curr_shape_mut().pos().1;

    // Двигаем влево
    if state.can_move_curr_shape_direction(Direction::Left) {
        state.get_curr_shape_mut().pos_mut().0 -= 1.0;
    }

    // Двигаем вправо
    if state.can_move_curr_shape_direction(Direction::Right) {
        state.get_curr_shape_mut().pos_mut().0 += 1.0;
    }

    // Двигаем вниз
    if state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos_mut().1 += 1.0;
    }

    // Проверяем, что позиция изменилась
    let final_x = state.get_curr_shape_mut().pos().0;
    let final_y = state.get_curr_shape_mut().pos().1;

    // Хотя бы одна координата должна измениться
    assert!(
        (final_x - initial_x).abs() > f32::EPSILON || (final_y - initial_y).abs() > f32::EPSILON,
        "Позиция фигуры должна измениться при движении"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-13: Взаимодействие компонентов
// ============================================================================

/// Тест 6: Проверка взаимодействия `GameState` и Tetromino
///
/// Проверяет, что `GameState` корректно работает с фигурами.
#[test]
fn test_game_state_tetromino_interaction() {
    let state = GameState::new();

    // Получаем текущую фигуру
    let curr = state.curr_shape();

    // Проверяем, что у фигуры правильная структура
    assert_eq!(curr.coords().len(), 4, "У фигуры должно быть 4 блока");
    assert!(curr.fg() < 7, "Индекс цвета должен быть в диапазоне 0-6");

    // Проверяем, что тип фигуры соответствует цвету
    assert_eq!(
        curr.shape() as u8,
        curr.fg(),
        "Индекс типа фигуры должен совпадать с индексом цвета"
    );
}

/// Тест 10: Проверка взаимодействия `GameStats` и `GameState`
///
/// Проверяет, что статистика корректно собирается.
#[test]
fn test_game_stats_game_state_interaction() {
    let state = GameState::new();
    let game_stats = state.stats();

    // В начале игры должна быть 1 фигура
    assert_eq!(
        game_stats.total_pieces(),
        1,
        "В начале игры должна быть 1 фигура"
    );

    // Проверяем, что max_combo равен 0
    assert_eq!(game_stats.max_combo(), 0, "Начальное комбо должно быть 0");

    // Проверяем, что таймер не запущен
    assert!(
        game_stats.start_time().is_none(),
        "Таймер не должен быть запущен"
    );
}

/// Тест 13: Проверка взаимодействия вращения и столкновений
///
/// Проверяет что вращение возможно в пустом поле.
#[test]
fn test_rotation_collision_interaction() {
    let state = GameState::new();
    // В пустом поле вращение должно быть возможно
    assert!(
        state.can_rotate_curr_shape(RotationDirection::Clockwise),
        "Вращение в пустом поле должно быть возможно"
    );
}
