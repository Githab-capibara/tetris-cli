//! Тесты флага is_hard_dropping.
//!
//! Проверяют корректность установки и сброса флага is_hard_dropping
//! при выполнении Hard Drop.

use crate::game::GameState;

/// Тест 1: Проверка начального значения флага is_hard_dropping
///
/// Флаг должен быть false при создании нового состояния игры.
#[test]
fn test_hard_drop_flag_initial_state() {
    let state = GameState::new();

    assert!(
        !state.is_hard_dropping(),
        "Флаг is_hard_dropping должен быть false при создании игры"
    );
}

/// Тест 2: Проверка установки флага в true при Hard Drop
///
/// Флаг должен устанавливаться в true после выполнения Hard Drop.
#[test]
fn test_hard_drop_flag_set_on_drop() {
    use crate::game::scoring::handle_hard_drop;

    let mut state = GameState::new();

    // Проверяем начальное состояние
    assert!(
        !state.is_hard_dropping(),
        "До Hard Drop флаг должен быть false"
    );

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Проверяем, что флаг установлен
    assert!(
        state.is_hard_dropping(),
        "После Hard Drop флаг должен быть true для анимации"
    );
}

/// Тест 3: Проверка сброса флага после приземления
///
/// Флаг должен сбрасываться в false после обработки приземления.
#[test]
fn test_hard_drop_flag_reset_after_landing() {
    use crate::game::scoring::{handle_hard_drop, handle_landing};

    let mut state = GameState::new();

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);
    assert!(
        state.is_hard_dropping(),
        "После Hard Drop флаг должен быть true"
    );

    // Устанавливаем land_timer в 0 для немедленного приземления через сеттер
    state.set_land_timer(0.0);

    // Обрабатываем приземление
    let _result = handle_landing(&mut state);

    // Проверяем, что флаг сброшен
    assert!(
        !state.is_hard_dropping(),
        "После приземления флаг должен быть сброшен в false"
    );
}

/// Тест 4: Проверка корректности анимации Hard Drop
///
/// Проверяем, что флаг остаётся true во время анимации.
#[test]
fn test_hard_drop_flag_during_animation() {
    use crate::game::scoring::handle_hard_drop;

    let mut state = GameState::new();

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Проверяем, что флаг установлен для анимации
    assert!(
        state.is_hard_dropping(),
        "Флаг должен быть true во время анимации Hard Drop"
    );

    // Интервал анимации корректный - проверяется на уровне константы
}

/// Тест 5: Проверка множественных Hard Drop
///
/// Проверяем, что флаг корректно работает при нескольких Hard Drop подряд.
#[test]
fn test_hard_drop_flag_multiple_drops() {
    use crate::game::scoring::{handle_hard_drop, handle_landing};

    let mut state = GameState::new();

    // Первый Hard Drop
    handle_hard_drop(&mut state);
    assert!(
        state.is_hard_dropping(),
        "После 1-го Hard Drop флаг должен быть true"
    );

    // Сброс флага (симуляция окончания анимации)
    state.set_is_hard_dropping(false);

    // Переход к следующей фигуре (симуляция)
    let _ = handle_landing(&mut state);

    // Второй Hard Drop
    handle_hard_drop(&mut state);
    assert!(
        state.is_hard_dropping(),
        "После 2-го Hard Drop флаг должен быть true"
    );

    // Третий Hard Drop (после сброса)
    state.set_is_hard_dropping(false);
    handle_hard_drop(&mut state);
    assert!(
        state.is_hard_dropping(),
        "После 3-го Hard Drop флаг должен быть true"
    );
}

/// Тест 6: Проверка взаимодействия флага с начислением очков
///
/// Проверяем, что очки за Hard Drop начисляются корректно.
#[test]
fn test_hard_drop_flag_with_scoring() {
    use crate::game::scoring::handle_hard_drop;

    let mut state = GameState::new();
    let initial_score = state.score();

    // Получаем начальную позицию через геттер
    let start_y = state.curr_shape().pos.1;

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Проверяем, что флаг установлен через геттер
    assert!(
        state.is_hard_dropping(),
        "Флаг должен быть true после Hard Drop"
    );

    // Проверяем, что очки начислены через геттеры
    let drop_distance = (state.curr_shape().pos.1 - start_y) as u32;
    let expected_bonus = drop_distance as u128 * 2; // 2 очка за ячейку

    assert!(
        state.score() >= initial_score + expected_bonus,
        "Очки за Hard Drop должны быть начислены: было {initial_score}, стало {}, ожидаемый бонус {expected_bonus}",
        state.score()
    );
}

/// Тест 7: Проверка флага при разных высотах падения
///
/// Проверяем, что флаг устанавливается независимо от высоты падения.
#[test]
fn test_hard_drop_flag_different_heights() {
    use crate::game::scoring::handle_hard_drop;

    // Тест на разной высоте
    for test_num in 0..3 {
        let mut state = GameState::new();

        // Поднимаем фигуру на разную высоту через геттер
        match test_num {
            0 => {} // Начальная позиция
            1 => {
                // Поднимаем на 5 ячеек
                state.get_curr_shape_mut().pos.1 = 5.0;
            }
            2 => {
                // Поднимаем на 10 ячеек
                state.get_curr_shape_mut().pos.1 = 10.0;
            }
            _ => unreachable!(),
        }

        handle_hard_drop(&mut state);

        assert!(
            state.is_hard_dropping(),
            "Флаг должен быть true для теста {test_num}"
        );
    }
}
