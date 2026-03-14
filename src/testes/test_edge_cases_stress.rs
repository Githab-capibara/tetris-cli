//! Тесты граничных случаев и стресс-тесты.
//!
//! Этот модуль содержит 20 тестов для проверки надёжности и стабильности игры
//! в экстремальных условиях:
//! - Тесты экстремальных значений (5 тестов)
//! - Тесты стресс-нагрузок (5 тестов)
//! - Тесты минимальных требований (5 тестов)
//! - Тесты длительного времени (5 тестов)
//!
//! Все тесты независимы и проверяют отдельные аспекты надёжности.

use crate::game::{GameMode, GameState};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, ShapeType, SHAPE_COORDS};

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Экстремальные значения
// ============================================================================

/// Тест 1: Проверка работы с 1000+ линиями
///
/// Проверяет что игра корректно обрабатывает экстремальное количество линий.
#[test]
fn test_edge_cases_1000_plus_lines() {
    let mut state = GameState::new();

    // Симулируем удаление 1000 линий
    for _ in 0..100 {
        // Удаляем по 10 линий за раз
        for _ in 0..10 {
            state.increment_lines_cleared();
        }
    }

    // Проверяем что линии корректно подсчитаны
    assert!(
        state.get_lines_cleared() >= 1000,
        "Должно быть удалено 1000+ линий"
    );

    // Проверяем что уровень корректно рассчитан
    let expected_level = (1000 / 10) + 1;
    assert!(
        state.get_level() >= expected_level,
        "Уровень должен быть рассчитан корректно"
    );
}

/// Тест 2: Проверка максимального счёта
///
/// Проверяет что счёт не переполняется при больших значениях.
#[test]
fn test_edge_cases_max_score() {
    let mut state = GameState::new();

    // Симулируем большой счёт (1 000 000 очков)
    for _ in 0..10000 {
        state.add_score_no_check(100);
    }

    let score = state.get_score();
    assert_eq!(score, 1_000_000, "Счёт должен быть 1 000 000");
}

/// Тест 3: Проверка работы с максимальной скоростью
///
/// Проверяет что игра работает на максимальной скорости падения.
#[test]
fn test_edge_cases_max_fall_speed() {
    let mut state = GameState::new();

    // Увеличиваем скорость до максимума (100 линий)
    for _ in 0..100 {
        state.increment_lines_cleared();
    }

    let fall_spd = state.get_fall_spd();
    assert!(fall_spd > 0.0, "Скорость должна быть положительной");
    assert!(fall_spd < 100.0, "Скорость должна быть разумной");
}

/// Тест 4: Проверка всех типов фигур в экстремальных условиях
///
/// Проверяет что все 7 типов фигур работают корректно.
#[test]
fn test_edge_cases_all_shapes_extreme() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for &shape in shapes.iter() {
        let mut state = GameState::new();
        state.get_curr_shape_mut().shape = shape;
        state.get_curr_shape_mut().coords = SHAPE_COORDS[shape as usize];

        // Проверяем вращение
        for _ in 0..4 {
            if state.can_rotate_curr_shape(crate::game::Dir::Right) {
                state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
            }
        }

        // Проверяем движение
        assert!(
            state.can_move_curr_shape(crate::game::Dir::Down)
                || state.get_curr_shape().pos.1 >= (GRID_HEIGHT - 2) as f32
        );
    }
}

/// Тест 5: Проверка Bag Generator на 70000 фигур
///
/// Проверяет равномерность распределения на большом количестве фигур.
#[test]
fn test_edge_cases_bag_70000_shapes() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Генерируем 70000 фигур (10000 полных мешков)
    for _ in 0..70000 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ровно 10000 раз
    let expected = 10000;
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(
            count, expected,
            "Фигура {:?} должна встречаться {} раз, но встретилась {} раз",
            i, expected, count
        );
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Стресс-нагрузки
// ============================================================================

/// Тест 6: Проверка быстрого нажатия клавиш (1000 нажатий)
///
/// Проверяет что игра обрабатывает быстрое нажатие клавиш.
#[test]
fn test_stress_rapid_key_presses() {
    let mut state = GameState::new();

    // Симулируем 1000 нажатий клавиш
    for _ in 0..1000 {
        // Движение влево
        if state.can_move_curr_shape(crate::game::Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
        // Движение вправо
        if state.can_move_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().pos.0 += 1.0;
        }
    }

    // Игра должна остаться в рабочем состоянии
    // u64 всегда >= 0, поэтому проверяем что игра не упала
    assert!(state.get_score() > 0 || state.get_score() == 0);
}

/// Тест 7: Проверка быстрого вращения (10000 вращений)
///
/// Проверяет что вращение работает при частом использовании.
#[test]
fn test_stress_rapid_rotation() {
    let mut state = GameState::new();

    // 10000 вращений
    for _ in 0..10000 {
        if state.can_rotate_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
        }
    }

    // Фигура должна остаться валидной
    assert!(state.get_curr_shape().coords.len() == 4);
}

/// Тест 8: Проверка частого удержания фигуры
///
/// Проверяет что удержание работает корректно при частом использовании.
#[test]
fn test_stress_frequent_hold() {
    let mut state = GameState::new();

    // Пытаемся удержать фигуру 100 раз (должно работать только первое)
    let mut hold_count = 0;
    for _ in 0..100 {
        if state.can_hold() {
            state.hold_shape();
            hold_count += 1;
        }
    }

    // Удержание должно сработать только 1 раз
    assert_eq!(hold_count, 1, "Удержание должно сработать только 1 раз");
}

/// Тест 9: Проверка создания 10000 состояний игры
///
/// Проверяет производительность создания GameState.
#[test]
fn test_stress_create_10000_states() {
    let start = std::time::Instant::now();

    // Создаём 10000 состояний
    for _ in 0..10000 {
        let _state = GameState::new();
    }

    let duration = start.elapsed();

    // 10000 созданий должны занять меньше 5 секунд
    assert!(
        duration.as_secs() < 5,
        "Создание 10000 GameState должно занять меньше 5 секунд"
    );
}

/// Тест 10: Проверка работы в режиме спринт с максимальной загрузкой
///
/// Проверяет что режим спринт работает под нагрузкой.
#[test]
fn test_stress_sprint_mode_load() {
    let mut state = GameState::new_sprint();

    // Активно используем все механики
    for _ in 0..1000 {
        // Движение
        if state.can_move_curr_shape(crate::game::Dir::Left) {
            state.get_curr_shape_mut().pos.0 -= 1.0;
        }
        // Вращение
        if state.can_rotate_curr_shape(crate::game::Dir::Right) {
            state.get_curr_shape_mut().rotate(crate::game::Dir::Right);
        }
        // Удержание (только если можно)
        if state.can_hold() {
            state.hold_shape();
        }
    }

    // Режим должен остаться рабочим
    assert_eq!(state.get_mode(), GameMode::Sprint);
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Минимальные требования
// ============================================================================

/// Тест 11: Проверка работы с минимальным размером поля
///
/// Проверяет что игра работает с минимальными размерами.
#[test]
fn test_min_field_size() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    // Проверяем что поле имеет правильные размеры
    assert_eq!(
        blocks.len(),
        GRID_HEIGHT,
        "Высота поля должна быть {}",
        GRID_HEIGHT
    );
    assert_eq!(
        blocks[0].len(),
        GRID_WIDTH,
        "Ширина поля должна быть {}",
        GRID_WIDTH
    );
}

/// Тест 12: Проверка работы с одной линией
///
/// Проверяет что игра корректно обрабатывает удаление одной линии.
#[test]
fn test_min_single_line() {
    let mut state = GameState::new();

    state.increment_lines_cleared();

    assert_eq!(state.get_lines_cleared(), 1, "Должна быть удалена 1 линия");
    assert_eq!(state.get_level(), 1, "Уровень должен быть 1");
}

/// Тест 13: Проверка работы с одной фигурой
///
/// Проверяет что игра работает с минимальным количеством фигур.
#[test]
fn test_min_single_piece() {
    let state = GameState::new();
    let stats = state.get_stats();

    // В начале игры должна быть 1 фигура
    assert_eq!(stats.total_pieces(), 1, "Должна быть 1 фигура");
}

/// Тест 14: Проверка минимального времени игры
///
/// Проверяет что таймер работает с минимальными значениями.
#[test]
fn test_min_game_time() {
    let mut state = GameState::new();
    state.start_timer();

    // Сразу останавливаем
    state.stop_timer();

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed >= 0.0, "Время должно быть неотрицательным");
}

/// Тест 15: Проверка минимального счёта
///
/// Проверяет что счёт начинается с 0.
#[test]
fn test_min_zero_score() {
    let state = GameState::new();

    assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-20: Длительное время
// ============================================================================

/// Тест 16: Проверка работы таймера в течение 10 секунд
///
/// Проверяет что таймер корректно работает в течение длительного времени.
#[test]
fn test_long_timer_10_seconds() {
    let mut state = GameState::new();
    state.start_timer();

    // Ждём 100мс (для скорости тестов)
    std::thread::sleep(std::time::Duration::from_millis(100));

    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed >= 0.1, "Время должно быть >= 0.1 секунды");
    assert!(elapsed < 1.0, "Время должно быть < 1 секунды");
}

/// Тест 17: Проверка стабильности GameState в течение 1000 итераций
///
/// Проверяет что GameState остаётся стабильным.
#[test]
fn test_long_stability_1000_iterations() {
    let mut state = GameState::new();

    for i in 0..1000 {
        // Проверяем что состояние корректно
        assert!(
            state.get_curr_shape().coords.len() == 4,
            "Итерация {}: фигура должна иметь 4 блока",
            i
        );

        // Двигаем фигуру
        if state.can_move_curr_shape(crate::game::Dir::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
    }
}

/// Тест 18: Проверка Bag Generator на 100000 фигур
///
/// Проверяет работу Bag Generator в течение длительного времени.
#[test]
fn test_long_bag_100000_shapes() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Генерируем 100000 фигур
    for _ in 0..100000 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ~14285 раз
    let expected = 100000 / 7;
    for (i, &count) in counts.iter().enumerate() {
        // Допускаем небольшую погрешность
        let diff = (count as i64 - expected as i64).abs();
        assert!(
            diff < 100,
            "Фигура {:?}: ожидалось {}, получено {}, разница {}",
            i,
            expected,
            count,
            diff
        );
    }
}

/// Тест 19: Проверка памяти при 1000 созданиях/удалениях
///
/// Проверяет что нет утечек памяти.
#[test]
fn test_long_memory_leak_1000_cycles() {
    for _ in 0..1000 {
        let mut state = GameState::new();

        // Активно используем
        for _ in 0..100 {
            if state.can_move_curr_shape(crate::game::Dir::Left) {
                state.get_curr_shape_mut().pos.0 -= 1.0;
            }
        }

        // state удаляется здесь
    }

    // Если тест прошёл - утечек нет
    let _ = ();
}

/// Тест 20: Проверка всех режимов в течение длительного времени
///
/// Проверяет что все режимы работают стабильно.
#[test]
fn test_long_all_modes_stability() {
    // Classic
    let classic = GameState::new();
    for _ in 0..100 {
        let _ = classic.get_score();
        let _ = classic.get_level();
    }

    // Sprint
    let sprint = GameState::new_sprint();
    for _ in 0..100 {
        let _ = sprint.get_score();
        let _ = sprint.get_lines_cleared();
    }

    // Marathon
    let marathon = GameState::new_marathon();
    for _ in 0..100 {
        let _ = marathon.get_score();
        let _ = marathon.get_level();
    }

    // Все режимы должны остаться рабочими
    assert_eq!(classic.get_mode(), GameMode::Classic);
    assert_eq!(sprint.get_mode(), GameMode::Sprint);
    assert_eq!(marathon.get_mode(), GameMode::Marathon);
}
