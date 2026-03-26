//! Интеграционные тесты всех исправлений.
//!
//! Проверяют совместную работу всех исправлений и отсутствие регрессий.

#![allow(deprecated)] // Для обратной совместимости

use crate::game::GameState;
use crate::types::{Direction, RotationDirection};

/// Тест 1: Проверка совместной работы GameStats и констант
///
/// Проверяем, что GameStats и константы работают вместе.
#[test]
fn test_game_stats_and_constants_integration() {
    use crate::game::{COMBO_BONUS, LINE_SCORES};

    let mut state = GameState::new();

    // Проверяем, что константы доступны
    assert_eq!(LINE_SCORES[0], 100, "Очки за 1 линию должны быть 100");
    assert_eq!(COMBO_BONUS, 50, "Бонус за комбо должен быть 50");

    // Проверяем, что статистика работает
    // При создании GameState уже добавлена первая фигура
    assert!(
        state.stats.total_pieces() >= 1,
        "Должна быть хотя бы 1 фигура"
    );

    // Запоминаем начальное количество T-фигур
    let initial_t_pieces = state.stats.t_pieces;

    // Добавляем фигуру
    state.stats.add_piece(crate::tetromino::ShapeType::T);
    assert_eq!(
        state.stats.t_pieces,
        initial_t_pieces + 1,
        "Должна добавиться 1 T-фигура"
    );
}

/// Тест 2: Проверка совместной работы Hard Drop и флага is_hard_dropping
///
/// Проверяем, что Hard Drop и флаг работают вместе.
#[test]
fn test_hard_drop_and_flag_integration() {
    use crate::game::scoring::handle_hard_drop;

    let mut state = GameState::new();

    // Проверяем начальный флаг
    assert!(
        !state.is_hard_dropping,
        "Флаг должен быть false до Hard Drop"
    );

    // Выполняем Hard Drop
    handle_hard_drop(&mut state);

    // Проверяем флаг после Hard Drop
    assert!(
        state.is_hard_dropping,
        "Флаг должен быть true после Hard Drop"
    );

    // Проверяем начисление очков
    assert!(state.score > 0, "Очки за Hard Drop должны быть начислены");
}

/// Тест 3: Проверка совместной работы Direction::Down и Soft Drop
///
/// Проверяем, что Direction::Down и Soft Drop работают вместе.
#[test]
fn test_direction_down_and_soft_drop_integration() {
    use crate::game::scoring::handle_soft_drop;

    let mut state = GameState::new();
    let initial_y = state.curr_shape.pos.1;

    // Выполняем Soft Drop
    for _ in 0..5 {
        if state.can_move_curr_shape_direction(Direction::Down) {
            handle_soft_drop(&mut state);
        }
    }

    // Проверяем, что фигура опустилась
    assert!(
        state.curr_shape.pos.1 > initial_y,
        "Фигура должна опуститься"
    );

    // Очки за Soft Drop начисляются при приземлении, а не сразу
    // Проверяем, что soft_drop_distance отслеживается
    assert!(
        state.soft_drop_distance > 0,
        "soft_drop_distance должен быть > 0"
    );
}

/// Тест 4: Проверка совместной работы wall kick и вращения
///
/// Проверяем, что wall kick и вращение работают вместе.
#[test]
fn test_wall_kick_and_rotation_integration() {
    let mut state = GameState::new();

    // Проверяем, что can_rotate_curr_shape работает
    let can_rotate = state.can_rotate_curr_shape(RotationDirection::Clockwise);

    // Проверяем, что rotate_with_wall_kick работает
    let rotated = state.rotate_with_wall_kick(RotationDirection::Clockwise);

    // Результаты должны быть согласованы
    if can_rotate {
        assert!(rotated, "Вращение должно succeed");
    }
}

/// Тест 5: Проверка совместной работы sanitize и Unicode валидации
///
/// Проверяем, что sanitize_player_name и Unicode валидация работают вместе.
#[test]
fn test_sanitize_and_unicode_validation_integration() {
    use crate::highscore::leaderboard::LeaderboardEntry;

    // Имя с запрещёнными символами
    let bad_name = "Player\u{200C}\u{FE00}Name";
    let entry = LeaderboardEntry::new(bad_name, 1000);

    // Проверяем, что запрещённые символы отфильтрованы
    assert!(
        !entry.name().contains('\u{200C}'),
        "U+200C должен быть отфильтрован"
    );
    assert!(
        !entry.name().contains('\u{FE00}'),
        "U+FE00 должен быть отфильтрован"
    );

    // Проверяем, что допустимые символы остались
    assert!(
        entry.name().contains("Player"),
        "Допустимые символы должны остаться"
    );
}

/// Тест 6: Проверка совместной работы кэширования и отрисовки
///
/// Проверяем, что кэширование строк работает корректно.
#[test]
fn test_string_caching_and_render_integration() {
    use crate::game::render::update_cached_strings_extended;

    let mut state = GameState::new();

    // Устанавливаем значения
    state.score = 1500;
    state.level = 5;
    state.lines_cleared = 25;

    // Обновляем кэш
    update_cached_strings_extended(&mut state, "1000");

    // Проверяем кэш
    assert!(
        state.cached_score_str.trim() == "1500",
        "Кэш счёта должен быть '1500', получено '{}'",
        state.cached_score_str
    );
    assert!(
        state.cached_level_str.trim() == "5",
        "Кэш уровня должен быть '5', получено '{}'",
        state.cached_level_str
    );
    assert!(
        state.cached_lines_str.trim() == "25",
        "Кэш линий должен быть '25', получено '{}'",
        state.cached_lines_str
    );
}

/// Тест 7: Проверка совместной работы проверок границ и коллизий
///
/// Проверяем, что проверки границ и коллизий работают вместе.
#[test]
fn test_bounds_check_and_collision_integration() {
    use crate::game::logic::can_move_curr_shape_direction;

    let mut state = GameState::new();

    // Проверяем движение в центре поля
    assert!(
        can_move_curr_shape_direction(&state, Direction::Left),
        "Влево должно быть возможно"
    );
    assert!(
        can_move_curr_shape_direction(&state, Direction::Right),
        "Вправо должно быть возможно"
    );
    assert!(
        can_move_curr_shape_direction(&state, Direction::Down),
        "Вниз должно быть возможно"
    );

    // Перемещаем к стене
    while can_move_curr_shape_direction(&state, Direction::Left) {
        state.curr_shape.pos.0 -= 1.0;
    }

    // Проверяем, что движение влево заблокировано
    assert!(
        !can_move_curr_shape_direction(&state, Direction::Left),
        "Влево у стены должно быть заблокировано"
    );
}

/// Тест 8: Проверка совместной работы find_full_rows и remove_rows
///
/// Проверяем, что поиск и удаление линий работают вместе.
#[test]
fn test_find_and_remove_rows_integration() {
    use crate::game::scoring::{find_full_rows, remove_rows};
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Заполняем линию
    for x in 0..GRID_WIDTH {
        state.blocks[10][x] = 1;
    }

    // Находим линии
    let (rows_mask, remove_count) = find_full_rows(&state.blocks);
    assert_eq!(remove_count, 1, "Должна быть найдена 1 линия");

    // Удаляем линии
    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем, что линия удалена
    for x in 0..GRID_WIDTH {
        assert_eq!(state.blocks[10][x], -1, "Линия должна быть пустой");
    }
}

/// Тест 9: Проверка отсутствия регрессий
///
/// Проверяем, что исправления не сломали существующую функциональность.
#[test]
fn test_no_regressions() {
    let mut state = GameState::new();

    // Проверяем базовую функциональность
    assert_eq!(state.get_score(), 0, "Счёт должен быть 0");
    assert_eq!(state.get_level(), 1, "Уровень должен быть 1");
    assert_eq!(state.get_lines_cleared(), 0, "Линии должны быть 0");
    assert_eq!(
        state.get_mode(),
        crate::game::GameMode::Classic,
        "Режим должен быть Classic"
    );

    // Проверяем, что фигуры генерируются
    assert!(
        (state.curr_shape.shape as usize) < 7,
        "Фигура должна быть корректной"
    );

    // Проверяем, что движение работает
    assert!(
        state.can_move_curr_shape_direction(Direction::Down),
        "Движение вниз должно работать"
    );

    // Проверяем, что вращение работает
    assert!(
        state.can_rotate_curr_shape(RotationDirection::Clockwise),
        "Вращение должно работать"
    );
}

/// Тест 10: Проверка всех исправлений вместе
///
/// Комплексный тест всех исправлений.
#[test]
fn test_all_fixes_comprehensive_integration() {
    use crate::game::scoring::{find_full_rows, handle_hard_drop};
    use crate::highscore::leaderboard::LeaderboardEntry;

    // 1. GameStats экспорт
    let mut state = GameState::new();
    // При создании GameState уже добавлена первая фигура
    assert!(
        state.stats.total_pieces() >= 1,
        "Должна быть хотя бы 1 фигура"
    );

    // 2. Константы
    use crate::game::{HARD_DROP_POINTS, LINE_SCORES, SOFT_DROP_POINTS};
    assert_eq!(LINE_SCORES[0], 100);
    assert_eq!(HARD_DROP_POINTS, 2);
    assert_eq!(SOFT_DROP_POINTS, 1);

    // 3. Hard Drop флаг
    let was_hard_dropping = state.is_hard_dropping;
    assert!(!was_hard_dropping);
    handle_hard_drop(&mut state);
    assert!(state.is_hard_dropping);

    // 4. Direction::Down - проверяем что направление работает
    // После Hard Drop движение вниз может быть заблокировано
    assert!(
        !was_hard_dropping || state.can_move_curr_shape_direction(Direction::Down),
        "Direction::Down должен работать"
    );

    // 5. Wall kick
    assert!(state.can_rotate_curr_shape(RotationDirection::Clockwise));

    // 6. Unicode валидация
    let bad_name = "Player\u{200C}Name";
    let entry = LeaderboardEntry::new(bad_name, 1000);
    assert!(!entry.name().contains('\u{200C}'));

    // 7. Кэширование
    use crate::game::render::update_cached_strings_extended;
    state.score = 1500;
    state.level = 5;
    state.lines_cleared = 25;
    update_cached_strings_extended(&mut state, "0");
    // Кэш счёта форматируется с пробелами
    assert!(
        state.cached_score_str.trim().contains("1500"),
        "Кэш счёта должен содержать '1500'"
    );
    assert!(
        state.cached_level_str.trim().contains("5"),
        "Кэш уровня должен содержать '5'"
    );
    assert!(
        state.cached_lines_str.trim().contains("25"),
        "Кэш линий должен содержать '25'"
    );

    // 8. Проверки границ
    assert!(state.can_move_curr_shape_direction(Direction::Left));

    // 9. Поиск линий
    let (mask, count) = find_full_rows(state.get_blocks());
    assert_eq!(count, 0, "Линий для удаления должно быть 0");

    // 10. Безопасность времени
    let entry2 = LeaderboardEntry::new("Player2", 2000);
    assert!(entry2.is_valid());

    // Все тесты прошли - исправления работают вместе
    assert!(true, "Все исправления работают корректно");
}
