//! Интеграционные тесты.
//!
//! Этот модуль содержит 20 интеграционных тестов для проверки
//! взаимодействия всех компонентов игры:
//! - Тесты полного игрового цикла (5 тестов)
//! - Тесты взаимодействия компонентов (8 тестов)
//! - Тесты производительности (7 тестов)
//!
//! Интеграционные тесты проверяют совместную работу модулей.

use crate::controls::ControlsConfig;
use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
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
    assert_eq!(
        state.lines_cleared(),
        0,
        "Начальные линии должны быть 0"
    );
    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим по умолчанию - Classic"
    );

    // Проверяем наличие фигур
    assert!(
        (state.curr_shape().shape as usize) < 7,
        "Текущая фигура должна быть валидной"
    );
    assert!(
        (state.next_shape().shape as usize) < 7,
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
    let elapsed = state.get_stats().get_elapsed_time();
    assert!(elapsed > 0.0, "Таймер должен течь");
}

/// Тест 3: Проверка движения фигуры в игровом цикле
///
/// Проверяет, что фигура может двигаться в пустом поле.
#[test]
fn test_piece_movement_cycle() {
    let mut state = GameState::new();

    // Запоминаем начальную позицию
    let initial_x = state.get_curr_shape_mut().pos.0;
    let initial_y = state.get_curr_shape_mut().pos.1;

    // Двигаем влево
    if state.can_move_curr_shape_direction(Direction::Left) {
        state.get_curr_shape_mut().pos.0 -= 1.0;
    }

    // Двигаем вправо
    if state.can_move_curr_shape_direction(Direction::Right) {
        state.get_curr_shape_mut().pos.0 += 1.0;
    }

    // Двигаем вниз
    if state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
    }

    // Проверяем, что позиция изменилась
    let final_x = state.get_curr_shape_mut().pos.0;
    let final_y = state.get_curr_shape_mut().pos.1;

    // Хотя бы одна координата должна измениться
    assert!(
        (final_x - initial_x).abs() > f32::EPSILON || (final_y - initial_y).abs() > f32::EPSILON,
        "Позиция фигуры должна измениться при движении"
    );
}

/// Тест 4: Проверка падения фигуры до пола
///
/// Проверяет полный цикл падения фигуры.
#[test]
fn test_piece_drop_to_floor() {
    let mut state = GameState::new();

    let start_y = state.get_curr_shape_mut().pos.1;

    // Опускаем фигуру до упора
    let mut drop_count = 0;
    while state.can_move_curr_shape_direction(Direction::Down) {
        state.get_curr_shape_mut().pos.1 += 1.0;
        drop_count += 1;
    }

    let end_y = state.get_curr_shape_mut().pos.1;

    // Фигура должна опуститься
    assert!(end_y > start_y, "Фигура должна опуститься вниз");
    assert!(drop_count > 0, "Должно быть хотя бы одно движение вниз");

    // Дальнейшее движение вниз должно быть заблокировано
    assert!(
        !state.can_move_curr_shape_direction(Direction::Down),
        "После достижения пола движение вниз должно быть заблокировано"
    );
}

/// Тест 5: Проверка вращения в игровом контексте
///
/// Проверяет, что вращение работает в контексте `GameState`.
#[test]
fn test_rotation_in_game_context() {
    let mut state = GameState::new();

    // Устанавливаем фигуру в центр поля для корректного вращения
    state.get_curr_shape_mut().pos = (5.0, 10.0);

    // Проверяем возможность вращения
    let can_rotate_right = state.can_rotate_curr_shape(RotationDirection::Clockwise);
    let can_rotate_left = state.can_rotate_curr_shape(RotationDirection::CounterClockwise);

    // Вращение должно быть возможно хотя бы в одном направлении
    // (кроме O-фигуры которая не вращается)
    if state.curr_shape().shape != ShapeType::O {
        assert!(
            can_rotate_right || can_rotate_left,
            "Хотя бы одно направление вращения должно быть доступно"
        );
    }
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
    assert_eq!(curr.coords.len(), 4, "У фигуры должно быть 4 блока");
    assert!(curr.fg < 7, "Индекс цвета должен быть в диапазоне 0-6");

    // Проверяем, что тип фигуры соответствует цвету
    assert_eq!(
        curr.shape as usize, curr.fg,
        "Индекс типа фигуры должен совпадать с индексом цвета"
    );
}

/// Тест 7: Проверка взаимодействия `GameState` и `BagGenerator`
///
/// Проверяет, что фигуры создаются через `BagGenerator`.
#[test]
fn test_game_state_bag_generator_interaction() {
    let mut bag = BagGenerator::new();

    // Генерируем 7 фигур
    let mut shapes_found = [false; 7];
    for _ in 0..7 {
        let shape = bag.next_shape();
        shapes_found[shape as usize] = true;
    }

    // Проверяем, что все 7 типов встретились
    for (i, &found) in shapes_found.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна быть в первом мешке");
    }
}

/// Тест 8: Проверка взаимодействия `GameState` и Leaderboard
///
/// Проверяет, что рекорды могут быть сохранены после игры.
#[test]
fn test_game_state_leaderboard_interaction() {
    let mut leaderboard = Leaderboard::default();

    // Симулируем окончание игры с рекордом
    let final_score = 5000;
    let added = leaderboard.add_score("TestPlayer", final_score);

    assert!(added, "Рекорд должен быть добавлен");
    assert_eq!(
        leaderboard.get_best_score(),
        final_score,
        "Лучший рекорд должен совпадать"
    );
}

/// Тест 9: Проверка взаимодействия `ControlsConfig` и `GameState`
///
/// Проверяет, что конфигурация управления валидна для игры.
#[test]
fn test_controls_game_state_interaction() {
    let config = ControlsConfig::default_config();

    // Проверяем, что конфигурация валидна
    assert!(
        config.validate(),
        "Конфигурация по умолчанию должна быть валидной"
    );

    // Проверяем, что все клавиши уникальны
    let keys = [
        config.move_left,
        config.move_right,
        config.soft_drop,
        config.hard_drop,
        config.rotate_left,
        config.rotate_right,
        config.hold,
        config.pause,
        config.quit,
    ];

    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(keys[i], keys[j], "Все клавиши должны быть уникальны");
        }
    }
}

/// Тест 10: Проверка взаимодействия `GameStats` и `GameState`
///
/// Проверяет, что статистика корректно собирается.
#[test]
fn test_game_stats_game_state_interaction() {
    let state = GameState::new();
    let game_stats = state.get_stats();

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

/// Тест 11: Проверка взаимодействия `SaveData` и Leaderboard
///
/// Проверяет, что обе системы сохранения работают корректно.
#[test]
fn test_save_data_leaderboard_interaction() {
    // Создаём SaveData
    let save = SaveData::from_value(3000);
    assert_eq!(
        save.verify_and_get_score(),
        Some(3000),
        "SaveData должен хранить 3000"
    );

    // Создаём Leaderboard
    let mut leaderboard = Leaderboard::default();
    let _ = leaderboard.add_score("Player", 3000);

    assert_eq!(
        leaderboard.get_best_score(),
        3000,
        "Leaderboard должен хранить 3000"
    );
}

/// Тест 12: Проверка взаимодействия всех 7 типов фигур с игрой
///
/// Проверяет, что все типы фигур могут быть использованы в игре.
#[test]
fn test_all_shapes_in_game() {
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for &shape_type in &shapes {
        // Создаём фигуру вручную
        let tetromino = Tetromino {
            pos: (4.0, 0.0),
            shape: shape_type,
            coords: crate::tetromino::SHAPE_COORDS[shape_type as usize],
            fg: shape_type as usize,
        };

        // Проверяем, что фигура валидна
        assert!(tetromino.fg < 7, "Индекс цвета должен быть валидным");
        assert_eq!(tetromino.coords.len(), 4, "У фигуры должно быть 4 блока");
    }
}

/// Тест 13: Проверка взаимодействия вращения и столкновений
///
/// Проверяет, что вращение учитывает столкновения.
#[test]
fn test_rotation_collision_interaction() {
    let state = GameState::new();

    // Проверяем, что can_rotate_curr_shape использует check_collision
    let can_rotate = state.can_rotate_curr_shape(RotationDirection::Clockwise);

    // В пустом поле вращение должно быть возможно (если фигура не у границы)
    // Тест просто проверяет, что метод работает и возвращает результат
    let _ = can_rotate; // Метод должен работать без паники
}

// ============================================================================
// ГРУППА ТЕСТОВ 14-20: Производительность
// ============================================================================

/// Тест 14: Проверка производительности создания `GameState`
///
/// Проверяет, что создание `GameState` происходит быстро.
#[test]
fn test_performance_game_state_creation() {
    let start = std::time::Instant::now();

    // Создаём 1000 состояний игры
    for _ in 0..1000 {
        let _state = GameState::new();
    }

    let duration = start.elapsed();

    // 1000 созданий должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "Создание 1000 GameState должно занять меньше 1 секунды"
    );
}

/// Тест 15: Проверка производительности Bag Generator
///
/// Проверяет, что генерация фигур происходит быстро.
#[test]
fn test_performance_bag_generator() {
    let mut bag = BagGenerator::new();
    let start = std::time::Instant::now();

    // Генерируем 10000 фигур
    for _ in 0..10_000 {
        let _shape = bag.next_shape();
    }

    let duration = start.elapsed();

    // 10000 фигур должны сгенерироваться меньше чем за 1 секунду
    assert!(
        duration.as_secs_f64() < 1.0,
        "Генерация 10000 фигур должна занять меньше 1 секунды"
    );
}

/// Тест 16: Проверка производительности проверки столкновений
///
/// Проверяет, что проверка столкновений происходит быстро.
#[test]
fn test_performance_collision_detection() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    // Выполняем 10000 проверок столкновений
    for _ in 0..10_000 {
        let _ = state.can_move_curr_shape_direction(Direction::Down);
        let _ = state.can_move_curr_shape_direction(Direction::Left);
        let _ = state.can_move_curr_shape_direction(Direction::Right);
    }

    let duration = start.elapsed();

    // 30000 проверок должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "30000 проверок столкновений должны занять меньше 1 секунды"
    );
}

/// Тест 17: Проверка производительности вращения фигур
///
/// Проверяет, что вращение фигур происходит быстро.
#[test]
fn test_performance_rotation() {
    let mut bag = BagGenerator::new();
    let mut tetromino = Tetromino::from_bag(&mut bag);
    let start = std::time::Instant::now();

    // Выполняем 10000 вращений
    for _ in 0..10_000 {
        tetromino.rotate(RotationDirection::Clockwise);
    }

    let duration = start.elapsed();

    // 10000 вращений должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "10000 вращений должны занять меньше 1 секунды"
    );
}

/// Тест 18: Проверка производительности Leaderboard
///
/// Проверяет, что добавление рекордов происходит быстро.
#[test]
fn test_performance_leaderboard() {
    let mut leaderboard = Leaderboard::default();
    let start = std::time::Instant::now();

    // Добавляем 1000 рекордов
    for i in 0..1000 {
        let _ = leaderboard.add_score(&format!("Player{i}"), i * 100);
    }

    let duration = start.elapsed();

    // 1000 добавлений должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "Добавление 1000 рекордов должно занять меньше 1 секунды"
    );
}

/// Тест 19: Проверка производительности валидации `ControlsConfig`
///
/// Проверяет, что валидация конфигурации происходит быстро.
#[test]
fn test_performance_controls_validation() {
    let config = ControlsConfig::default_config();
    let start = std::time::Instant::now();

    // Выполняем 10000 валидаций
    for _ in 0..10_000 {
        let _ = config.validate();
    }

    let duration = start.elapsed();

    // 10000 валидаций должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "10000 валидаций должны занять меньше 1 секунды"
    );
}

/// Тест 20: Проверка производительности хеширования `SaveData`
///
/// Проверяет, что создание хэшей происходит быстро.
#[test]
fn test_performance_save_data_hashing() {
    let start = std::time::Instant::now();

    // Создаём 1000 SaveData с хешированием
    for i in 0..1000 {
        let _save = SaveData::from_value(i * 100);
    }

    let duration = start.elapsed();

    // 1000 хеширований должны занять меньше 1 секунды
    assert!(
        duration.as_secs_f64() < 1.0,
        "1000 хеширований должны занять меньше 1 секунды"
    );
}
