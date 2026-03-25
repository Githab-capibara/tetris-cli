//! Тесты экспорта GameStats из crate::game.
//!
//! Проверяют, что GameStats корректно экспортирован и все поля доступны.

use crate::game::GameStats;
use crate::tetromino::ShapeType;

/// Тест 1: Проверка доступности GameStats из crate::game
///
/// Убеждаемся, что GameStats можно импортировать из основного модуля game.
#[test]
fn test_game_stats_export_available() {
    // Создаём новый экземпляр GameStats
    let stats = GameStats::new();

    // Проверяем, что структура создана корректно
    assert_eq!(
        stats.t_pieces, 0,
        "Начальное количество T фигур должно быть 0"
    );
    assert_eq!(
        stats.total_pieces(),
        0,
        "Общее количество фигур должно быть 0"
    );
}

/// Тест 2: Проверка поля t_pieces
///
/// Проверяем, что поле t_pieces доступно и корректно работает.
#[test]
fn test_game_stats_t_pieces_field() {
    let mut stats = GameStats::new();

    // Проверяем начальное значение
    assert_eq!(stats.t_pieces, 0, "Начальное t_pieces должно быть 0");

    // Добавляем T-фигуру
    stats.add_piece(ShapeType::T);
    assert_eq!(
        stats.t_pieces, 1,
        "После добавления T t_pieces должно быть 1"
    );

    // Добавляем ещё T-фигуры
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::T);
    assert_eq!(
        stats.t_pieces, 3,
        "После добавления 3 T t_pieces должно быть 3"
    );
}

/// Тест 3: Проверка поля i_pieces
///
/// Проверяем, что поле i_pieces доступно и корректно работает.
#[test]
fn test_game_stats_i_pieces_field() {
    let mut stats = GameStats::new();

    assert_eq!(stats.i_pieces, 0, "Начальное i_pieces должно быть 0");

    stats.add_piece(ShapeType::I);
    assert_eq!(
        stats.i_pieces, 1,
        "После добавления I i_pieces должно быть 1"
    );

    // Добавляем 4 I-фигуры (для Tetris)
    for _ in 0..3 {
        stats.add_piece(ShapeType::I);
    }
    assert_eq!(
        stats.i_pieces, 4,
        "После добавления 4 I i_pieces должно быть 4"
    );
}

/// Тест 4: Проверка поля max_combo
///
/// Проверяем, что поле max_combo доступно и update_max_combo работает.
#[test]
fn test_game_stats_max_combo_field() {
    let mut stats = GameStats::new();

    assert_eq!(stats.max_combo, 0, "Начальное max_combo должно быть 0");

    // Обновляем максимальное комбо
    stats.update_max_combo(1);
    assert_eq!(
        stats.max_combo, 1,
        "После update_max_combo(1) max_combo должно быть 1"
    );

    stats.update_max_combo(4);
    assert_eq!(
        stats.max_combo, 4,
        "После update_max_combo(4) max_combo должно быть 4"
    );

    // Меньшее значение не должно обновлять max_combo
    stats.update_max_combo(2);
    assert_eq!(stats.max_combo, 4, "max_combo не должно уменьшаться");
}

/// Тест 5: Проверка поля combo_counter
///
/// Проверяем, что поле combo_counter доступно и корректно работает.
#[test]
fn test_game_stats_combo_counter_field() {
    let mut stats = GameStats::new();

    assert_eq!(
        stats.combo_counter, 0,
        "Начальное combo_counter должно быть 0"
    );

    // Увеличиваем комбо
    stats.combo_counter += 1;
    assert_eq!(
        stats.combo_counter, 1,
        "После инкремента combo_counter должно быть 1"
    );

    stats.combo_counter += 1;
    stats.combo_counter += 1;
    assert_eq!(
        stats.combo_counter, 3,
        "После 3 инкрементов combo_counter должно быть 3"
    );

    // Сброс комбо
    stats.combo_counter = 0;
    assert_eq!(
        stats.combo_counter, 0,
        "После сброса combo_counter должно быть 0"
    );
}

/// Тест 6: Проверка метода total_pieces()
///
/// Проверяем, что метод total_pieces() корректно суммирует все фигуры.
#[test]
fn test_game_stats_total_pieces_method() {
    let mut stats = GameStats::new();

    // Добавляем по одной фигуре каждого типа
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::L);
    stats.add_piece(ShapeType::J);
    stats.add_piece(ShapeType::S);
    stats.add_piece(ShapeType::Z);
    stats.add_piece(ShapeType::O);
    stats.add_piece(ShapeType::I);

    assert_eq!(
        stats.total_pieces(),
        7,
        "Общее количество фигур должно быть 7"
    );

    // Добавляем ещё 3 фигуры
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::I);

    assert_eq!(
        stats.total_pieces(),
        10,
        "Общее количество фигур должно быть 10"
    );
}

/// Тест 7: Проверка полей start_time и end_time
///
/// Проверяем, что таймер работает корректно.
#[test]
fn test_game_stats_timer_fields() {
    let mut stats = GameStats::new();

    // Проверяем начальное состояние
    assert!(
        stats.start_time.is_none(),
        "start_time должен быть None до старта"
    );
    assert!(
        stats.end_time.is_none(),
        "end_time должен быть None до остановки"
    );

    // Запускаем таймер
    stats.start_timer();
    assert!(
        stats.start_time.is_some(),
        "start_time должен быть Some после старта"
    );
    assert!(stats.end_time.is_none(), "end_time должен оставаться None");

    // Получаем elapsed time (должно работать)
    let elapsed = stats.get_elapsed_time();
    assert!(elapsed >= 0.0, "Время должно быть неотрицательным");
}

/// Тест 8: Проверка Clone trait для GameStats
///
/// Проверяем, что GameStats реализует Clone.
#[test]
fn test_game_stats_clone() {
    let mut stats = GameStats::new();
    stats.add_piece(ShapeType::T);
    stats.add_piece(ShapeType::I);
    stats.update_max_combo(4);
    stats.combo_counter = 2;

    // Клонируем статистику
    let stats_clone = stats.clone();

    // Проверяем, что клон корректен
    assert_eq!(
        stats_clone.t_pieces, stats.t_pieces,
        "t_pieces должны совпадать"
    );
    assert_eq!(
        stats_clone.i_pieces, stats.i_pieces,
        "i_pieces должны совпадать"
    );
    assert_eq!(
        stats_clone.max_combo, stats.max_combo,
        "max_combo должны совпадать"
    );
    assert_eq!(
        stats_clone.combo_counter, stats.combo_counter,
        "combo_counter должны совпадать"
    );
}
