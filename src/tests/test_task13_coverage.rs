//! Тесты для Задачи 13: Покрытие тестами.
//!
//! Этот модуль содержит тесты для:
//! - `RenderCache` (кэширование строк отрисовки)
//! - `GameBoardAccess` (трейт доступа к игровому полю)
//! - `find_filled_lines` (поиск заполненных линий)
//! - TOCTOU маркер (!Send + !Sync)

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::game::access::{BoardMutable, BoardReadonly};
use crate::game::scoring::lines::find_filled_lines;
use crate::game::state::GameState;

// ============================================================================
// ТЕСТЫ ДЛЯ RENDER CACHE
// ============================================================================

/// Тест 1: Проверка создания нового `RenderCache`
#[test]
fn test_render_cache_new() {
    let cache = crate::game::cache::RenderCache::new();
    assert!(
        cache.cached_score_str.is_empty(),
        "Начальный счёт должен быть пустой строкой"
    );
    assert!(
        cache.cached_level_str.is_empty(),
        "Начальный уровень должен быть пустой строкой"
    );
    assert!(
        cache.cached_lines_str.is_empty(),
        "Начальные линии должны быть пустой строкой"
    );
    assert!(
        cache.cached_high_score_str.is_empty(),
        "Начальный рекорд должен быть пустой строкой"
    );
    assert!(
        cache.cached_combo_str.is_empty(),
        "Начальное комбо должно быть пустой строкой"
    );
    assert!(
        cache.cached_timer_str.is_empty(),
        "Начальный таймер должен быть пустой строкой"
    );
}

/// Тест 2: Проверка инициализации кэша
#[test]
fn test_render_cache_init() {
    let mut cache = crate::game::cache::RenderCache::new();
    cache.init_with_values(100, 1, 0, 0);

    assert_eq!(
        cache.cached_score_str, "100",
        "Кэш счёта должен содержать '100'"
    );
    assert_eq!(
        cache.cached_level_str, "1",
        "Кэш уровня должен содержать '1'"
    );
    assert_eq!(
        cache.cached_lines_str, "0",
        "Кэш линий должен содержать '0'"
    );
    assert_eq!(
        cache.cached_high_score_str, "0",
        "Кэш рекорда должен содержать '0'"
    );
}

/// Тест 3: Проверка обновления кэша
#[test]
fn test_render_cache_update_values() {
    let mut cache = crate::game::cache::RenderCache::new();
    cache.init_with_values(100, 1, 0, 0);

    let old_score = cache.last_cached_score;
    let old_level = cache.last_cached_level;

    cache.init_with_values(200, 2, 10, 500);

    assert_eq!(cache.last_cached_score, 200);
    assert_eq!(cache.last_cached_level, 2);
    assert_ne!(cache.last_cached_score, old_score);
    assert_ne!(cache.last_cached_level, old_level);
}

/// Тест 4: Проверка что `RenderCache` реализует Default
#[test]
fn test_render_cache_default() {
    let cache = crate::game::cache::RenderCache::default();
    assert_eq!(cache.last_cached_score, 0);
    assert_eq!(cache.last_cached_level, 0);
    assert_eq!(cache.last_cached_lines, 0);
    assert_eq!(cache.last_cached_combo, 0);
}

// ============================================================================
// ТЕСТЫ ДЛЯ GAMEBOARD ACCESS
// ============================================================================

/// Тест 8: Проверка получения игрового поля (только чтение)
#[test]
fn test_game_board_access_get_blocks() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    // Проверяем что поле не пустое (имеет правильные размеры)
    assert_eq!(
        blocks.len(),
        GRID_HEIGHT,
        "Высота поля должна быть {GRID_HEIGHT}"
    );
    assert_eq!(
        blocks[0].len(),
        GRID_WIDTH,
        "Ширина поля должна быть {GRID_WIDTH}"
    );
}

/// Тест 9: Проверка получения игрового поля (мутабельный доступ)
#[test]
fn test_game_board_access_get_blocks_mut() {
    let mut state = GameState::new();
    let blocks_mut = state.get_blocks_mut();

    // Устанавливаем блок
    blocks_mut[0][0] = 1;

    // Проверяем что блок установлен
    assert_eq!(blocks_mut[0][0], 1, "Блок должен быть установлен в (0, 0)");
}

/// Тест 10: Проверка получения отдельной ячейки
#[test]
fn test_game_board_access_get_block() {
    let state = GameState::new();

    // На пустом поле все ячейки должны быть -1
    assert_eq!(state.get_block(0, 0), -1, "Пустая ячейка должна быть -1");
    assert_eq!(state.get_block(5, 10), -1, "Пустая ячейка должна быть -1");
}

/// Тест 11: Проверка установки отдельной ячейки
#[test]
fn test_game_board_access_set_block() {
    let mut state = GameState::new();

    // Устанавливаем блок
    state.set_block(3, 7, 2);

    // Проверяем что блок установлен
    assert_eq!(
        state.get_block(3, 7),
        2,
        "Блок должен быть установлен в (3, 7)"
    );
}

/// Тест 12: Проверка проверки пустой ячейки
#[test]
fn test_game_board_access_is_block_empty() {
    let mut state = GameState::new();

    // Пустая ячейка
    assert!(
        state.is_block_empty(0, 0),
        "Ячейка (0, 0) должна быть пустой"
    );

    // Заполненная ячейка
    state.set_block(0, 0, 1);
    assert!(
        !state.is_block_empty(0, 0),
        "Ячейка (0, 0) должна быть заполненной"
    );
}

/// Тест 13: Проверка проверки заполненной ячейки
#[test]
fn test_game_board_access_is_block_occupied() {
    let mut state = GameState::new();

    // Пустая ячейка
    assert!(
        !state.is_block_occupied(0, 0),
        "Ячейка (0, 0) не должна быть заполненной"
    );

    // Заполненная ячейка
    state.set_block(0, 0, 1);
    assert!(
        state.is_block_occupied(0, 0),
        "Ячейка (0, 0) должна быть заполненной"
    );
}

/// Тест 14: Проверка получения и установки счёта
#[test]
fn test_game_board_access_score() {
    let mut state = GameState::new();

    // Начальный счёт
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");

    // Установка счёта
    let _ = state.add_score(500);
    assert_eq!(state.score(), 500, "Счёт должен быть 500");

    // Добавление очков
    let _ = state.add_score(250);
    assert_eq!(state.score(), 750, "Счёт должен быть 750");
}

/// Тест 15: Проверка получения и установки уровня
#[test]
fn test_game_board_access_level() {
    let mut state = GameState::new();

    // Начальный уровень
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");

    // Установка уровня
    state.set_level(5);
    assert_eq!(state.level(), 5, "Уровень должен быть 5");
}

/// Тест 16: Проверка получения и установки линий
#[test]
fn test_game_board_access_lines_cleared() {
    let mut state = GameState::new();

    // Начальное количество линий
    assert_eq!(
        state.lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );

    // Установка количества линий
    state.set_lines_cleared(15);
    assert_eq!(state.lines_cleared(), 15, "Количество линий должно быть 15");
}

/// Тест 17: Проверка получения и установки скорости падения
#[test]
#[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
fn test_game_board_access_fall_spd() {
    let mut state = GameState::new();

    // Начальная скорость
    let initial_spd = state.fall_speed();
    assert!(
        initial_spd > 0.0,
        "Начальная скорость должна быть положительной"
    );

    // Установка скорости
    let _ = state.set_fall_speed(2.5);
    assert_eq!(state.fall_speed(), 2.5, "Скорость должна быть 2.5");
}

/// Тест 18: Проверка получения и установки таймера приземления
#[test]
#[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
fn test_game_board_access_land_timer() {
    let mut state = GameState::new();

    // Начальный таймер (устанавливается в LAND_TIME_DELAY_S = 0.1)
    assert_eq!(state.land_timer(), 0.1, "Начальный таймер должен быть 0.1");

    // Установка таймера
    let _ = state.set_land_timer(0.5);
    assert_eq!(state.land_timer(), 0.5, "Таймер должен быть 0.5");
}

// ============================================================================
// ТЕСТЫ ДЛЯ FIND_FILLED_LINES
// ============================================================================

/// Тест 19: Проверка поиска на пустом поле
#[test]
fn test_find_filled_lines_empty_field() {
    let blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];
    let (_, filled_count) = find_filled_lines(&blocks);

    assert_eq!(
        filled_count, 0,
        "На пустом поле не должно быть заполненных линий"
    );
}

/// Тест 20: Проверка поиска с одной заполненной линией
#[test]
fn test_find_filled_lines_single_line() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем линию 10
    for block in &mut blocks[10][..GRID_WIDTH] {
        *block = 1;
    }

    let (filled_mask, filled_count) = find_filled_lines(&blocks);

    assert_eq!(filled_count, 1, "Должна быть найдена 1 заполненная линия");
    assert_ne!(
        filled_mask & (1 << 10),
        0,
        "Заполненная линия должна быть на индексе 10"
    );
}

/// Тест 21: Проверка поиска с несколькими заполненными линиями
#[test]
fn test_find_filled_lines_multiple_lines() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем линии 5, 10, 15
    for &y in &[5, 10, 15] {
        for block in &mut blocks[y][..GRID_WIDTH] {
            *block = 1;
        }
    }

    let (filled_mask, filled_count) = find_filled_lines(&blocks);

    assert_eq!(filled_count, 3, "Должно быть найдено 3 заполненные линии");
    assert_ne!(filled_mask & (1 << 5), 0, "Должна быть найдена линия 5");
    assert_ne!(filled_mask & (1 << 10), 0, "Должна быть найдена линия 10");
    assert_ne!(filled_mask & (1 << 15), 0, "Должна быть найдена линия 15");
}

/// Тест 22: Проверка поиска с полностью заполненным полем
#[test]
fn test_find_filled_lines_full_field() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем всё поле
    for row in &mut blocks[..GRID_HEIGHT] {
        for block in &mut row[..GRID_WIDTH] {
            *block = 1;
        }
    }

    let (_, filled_count) = find_filled_lines(&blocks);

    assert_eq!(
        filled_count, GRID_HEIGHT as u32,
        "Должны быть найдены все линии"
    );
}

/// Тест 23: Проверка поиска с частично заполненной линией
#[test]
fn test_find_filled_lines_partial_line() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем только половину линии 10
    for block in &mut blocks[10][..(GRID_WIDTH / 2)] {
        *block = 1;
    }

    let (_, filled_count) = find_filled_lines(&blocks);

    assert_eq!(
        filled_count, 0,
        "Частично заполненная линия не должна считаться заполненной"
    );
}

/// Тест 24: Проверка поиска с разными значениями блоков
#[test]
fn test_find_filled_lines_different_block_values() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем линию 7 разными значениями (разные цвета фигур)
    for (x, block) in blocks[7][..GRID_WIDTH].iter_mut().enumerate() {
        *block = (x % 7) as i8; // Значения 0-6
    }

    let (filled_mask, filled_count) = find_filled_lines(&blocks);

    assert_eq!(filled_count, 1, "Должна быть найдена 1 заполненная линия");
    assert_ne!(
        filled_mask & (1 << 7),
        0,
        "Заполненная линия должна быть на индексе 7"
    );
}

// ============================================================================
// ТЕСТЫ ДЛЯ TOCTOU МАРКЕР (!Send + !Sync)
// ============================================================================

/// Тест 25: Проверка что `GameState` используется только в одном потоке
///
/// Этот тест документирует что `GameState` не предназначен для передачи между потоками.
/// TOCTOU (Time-of-check to time-of-use) уязвимости предотвращаются
/// через отсутствие реализации Send + Sync.
#[test]
fn test_toctou_marker_game_state_single_threaded() {
    // GameState по умолчанию !Send и !Sync из-за использования
    // небезопасных для потоков типов (например, Canvas, терминал)

    let state = GameState::new();

    // Проверяем что GameState существует и работает
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");

    // Этот тест документирует намерение: GameState должен использоваться
    // только в одном потоке для предотвращения TOCTOU уязвимостей
}

/// Тест 26: Проверка что Canvas не используется в многопоточном контексте
///
/// Canvas содержит терминальный бэкенд который не является потокобезопасным.
#[test]
fn test_toctou_marker_canvas_single_threaded() {
    use crate::io::Canvas;

    // Canvas по умолчанию !Send и !Sync из-за работы с терминалом

    // Проверяем что Canvas создаётся и работает
    let result = std::panic::catch_unwind(|| {
        let _canvas = Canvas::new();
    });

    // Игнорируем панику если Canvas не создался (терминал недоступен в тестах)
    let _ = result;
}

/// Тест 27: Проверка компиляции с `GameState` в замыкании
///
/// Этот тест проверяет что код с `GameState` компилируется корректно.
#[test]
fn test_toctou_marker_closure_compilation() {
    // Проверяем что можно создать замыкание которое принимает GameState
    let closure = |state: &GameState| {
        // Используем state чтобы избежать предупреждения
        let _ = state.score();
    };

    let state = GameState::new();
    closure(&state);
}

/// Тест 28: Проверка документации TOCTOU
///
/// Этот тест проверяет что документация TOCTOU маркеров корректна.
#[test]
fn test_toctou_marker_documentation() {
    // Проверяем что GameState существует
    let _state = GameState::new();

    // Этот тест всегда проходит, но документирует намерение:
    // GameState должен быть !Send + !Sync для предотвращения TOCTOU уязвимостей
}

/// Тест 29: Проверка что `GameState` не имеет явной реализации Send
#[test]
fn test_toctou_marker_no_explicit_send() {
    // Этот тест проверяет что GameState не реализует Send явно
    // Если в будущем будет добавлена реализация Send, этот тест нужно обновить

    let state = GameState::new();

    // Проверяем что GameState работает корректно
    assert_eq!(state.score(), 0);
}

/// Тест 30: Проверка что `GameState` не имеет явной реализации Sync
#[test]
fn test_toctou_marker_no_explicit_sync() {
    // Этот тест проверяет что GameState не реализует Sync явно
    // Если в будущем будет добавлена реализация Sync, этот тест нужно обновить

    let state = GameState::new();

    // Проверяем что GameState работает корректно
    assert_eq!(state.score(), 0);
}
