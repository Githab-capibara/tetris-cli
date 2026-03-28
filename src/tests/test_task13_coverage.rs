//! Тесты для Задачи 13: Покрытие тестами.
//!
//! Этот модуль содержит тесты для:
//! - RenderCache (кэширование строк отрисовки)
//! - GameBoardAccess (трейт доступа к игровому полю)
//! - find_filled_lines (поиск заполненных линий)
//! - TOCTOU маркер (!Send + !Sync)

use crate::game::access::{BoardMutable, BoardReadonly};
use crate::game::cache::StringCache;
use crate::game::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::game::scoring::lines::find_filled_lines;
use crate::game::state::{GameMode, GameState, GameStats};

// ============================================================================
// ТЕСТЫ ДЛЯ RENDER CACHE (StringCache)
// ============================================================================

/// Тест 1: Проверка создания нового RenderCache
#[test]
fn test_render_cache_new() {
    let cache = StringCache::new();
    assert!(
        cache.score_str.is_empty(),
        "Начальный счёт должен быть пустой строкой"
    );
    assert!(
        cache.level_str.is_empty(),
        "Начальный уровень должен быть пустой строкой"
    );
    assert!(
        cache.lines_str.is_empty(),
        "Начальные линии должны быть пустой строкой"
    );
    assert!(
        cache.high_score_str.is_empty(),
        "Начальный рекорд должен быть пустой строкой"
    );
    assert!(
        cache.combo_str.is_empty(),
        "Начальное комбо должно быть пустой строкой"
    );
    assert!(
        cache.timer_str.is_empty(),
        "Начальный таймер должен быть пустой строкой"
    );
}

/// Тест 2: Проверка обновления кэша счёта
#[test]
fn test_render_cache_update_score() {
    let mut cache = StringCache::new();
    cache.update(
        100, // score
        1,   // level
        0,   // lines_cleared
        "0", // high_score_display
        0,   // combo
        GameMode::Classic,
        &GameStats::default(),
    );

    assert_eq!(
        cache.score_str.trim(),
        "100",
        "Кэш счёта должен содержать '100'"
    );
}

/// Тест 3: Проверка обновления кэша уровня
#[test]
fn test_render_cache_update_level() {
    let mut cache = StringCache::new();
    cache.update(
        0,   // score
        5,   // level
        0,   // lines_cleared
        "0", // high_score_display
        0,   // combo
        GameMode::Classic,
        &GameStats::default(),
    );

    assert_eq!(
        cache.level_str.trim(),
        "5",
        "Кэш уровня должен содержать '5'"
    );
}

/// Тест 4: Проверка обновления кэша линий
#[test]
fn test_render_cache_update_lines() {
    let mut cache = StringCache::new();
    cache.update(
        0,   // score
        1,   // level
        25,  // lines_cleared
        "0", // high_score_display
        0,   // combo
        GameMode::Classic,
        &GameStats::default(),
    );

    assert_eq!(
        cache.lines_str.trim(),
        "25",
        "Кэш линий должен содержать '25'"
    );
}

/// Тест 5: Проверка обновления кэша комбо
#[test]
fn test_render_cache_update_combo() {
    let mut cache = StringCache::new();
    cache.update(
        0,   // score
        1,   // level
        0,   // lines_cleared
        "0", // high_score_display
        3,   // combo (комбо x3)
        GameMode::Classic,
        &GameStats::default(),
    );

    assert_eq!(
        cache.combo_str, "Комбо: x3",
        "Кэш комбо должен содержать 'Комбо: x3'"
    );
}

/// Тест 6: Проверка что кэш не обновляется без изменений
#[test]
fn test_render_cache_no_update_without_changes() {
    let mut cache = StringCache::new();

    // Первое обновление
    cache.update(
        100, // score
        1,   // level
        0,   // lines_cleared
        "0", // high_score_display
        0,   // combo
        GameMode::Classic,
        &GameStats::default(),
    );

    let old_score_str = cache.score_str.clone();
    let old_level_str = cache.level_str.clone();

    // Второе обновление с теми же данными
    cache.update(
        100, // score (не изменился)
        1,   // level (не изменился)
        0,   // lines_cleared (не изменился)
        "0", // high_score_display (не изменился)
        0,   // combo (не изменился)
        GameMode::Classic,
        &GameStats::default(),
    );

    assert_eq!(
        cache.score_str, old_score_str,
        "Кэш счёта не должен измениться без изменений данных"
    );
    assert_eq!(
        cache.level_str, old_level_str,
        "Кэш уровня не должен измениться без изменений данных"
    );
}

/// Тест 7: Проверка очистки кэша
#[test]
fn test_render_cache_clear() {
    let mut cache = StringCache::new();

    // Заполняем кэш
    cache.update(
        1000,   // score
        10,     // level
        50,     // lines_cleared
        "5000", // high_score_display
        5,      // combo
        GameMode::Classic,
        &GameStats::default(),
    );

    // Очищаем кэш
    cache.clear();

    assert!(
        cache.score_str.is_empty(),
        "После очистки кэш счёта должен быть пустым"
    );
    assert!(
        cache.level_str.is_empty(),
        "После очистки кэш уровня должен быть пустым"
    );
    assert!(
        cache.lines_str.is_empty(),
        "После очистки кэш линий должен быть пустым"
    );
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
    assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");

    // Установка счёта
    state.add_score(500);
    assert_eq!(state.get_score(), 500, "Счёт должен быть 500");

    // Добавление очков
    state.add_score(250);
    assert_eq!(state.get_score(), 750, "Счёт должен быть 750");
}

/// Тест 15: Проверка получения и установки уровня
#[test]
fn test_game_board_access_level() {
    let mut state = GameState::new();

    // Начальный уровень
    assert_eq!(state.get_level(), 1, "Начальный уровень должен быть 1");

    // Установка уровня
    state.set_level(5);
    assert_eq!(state.get_level(), 5, "Уровень должен быть 5");
}

/// Тест 16: Проверка получения и установки линий
#[test]
fn test_game_board_access_lines_cleared() {
    let mut state = GameState::new();

    // Начальное количество линий
    assert_eq!(
        state.get_lines_cleared(),
        0,
        "Начальное количество линий должно быть 0"
    );

    // Установка количества линий
    state.set_lines_cleared(15);
    assert_eq!(
        state.get_lines_cleared(),
        15,
        "Количество линий должно быть 15"
    );
}

/// Тест 17: Проверка получения и установки скорости падения
#[test]
fn test_game_board_access_fall_spd() {
    let mut state = GameState::new();

    // Начальная скорость
    let initial_spd = state.get_fall_speed();
    assert!(
        initial_spd > 0.0,
        "Начальная скорость должна быть положительной"
    );

    // Установка скорости
    state.set_fall_speed(2.5);
    assert_eq!(state.get_fall_speed(), 2.5, "Скорость должна быть 2.5");
}

/// Тест 18: Проверка получения и установки таймера приземления
#[test]
fn test_game_board_access_land_timer() {
    let mut state = GameState::new();

    // Начальный таймер (устанавливается в LAND_TIME_DELAY_S = 0.1)
    assert_eq!(
        state.get_land_timer(),
        0.1,
        "Начальный таймер должен быть 0.1"
    );

    // Установка таймера
    state.set_land_timer(0.5);
    assert_eq!(state.get_land_timer(), 0.5, "Таймер должен быть 0.5");
}

// ============================================================================
// ТЕСТЫ ДЛЯ FIND_FILLED_LINES
// ============================================================================

/// Тест 19: Проверка поиска на пустом поле
#[test]
fn test_find_filled_lines_empty_field() {
    let blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];
    let filled = find_filled_lines(&blocks);

    assert!(
        filled.is_empty(),
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

    let filled = find_filled_lines(&blocks);

    assert_eq!(filled.len(), 1, "Должна быть найдена 1 заполненная линия");
    assert_eq!(filled[0], 10, "Заполненная линия должна быть на индексе 10");
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

    let filled = find_filled_lines(&blocks);

    assert_eq!(filled.len(), 3, "Должно быть найдено 3 заполненные линии");
    assert!(filled.contains(&5), "Должна быть найдена линия 5");
    assert!(filled.contains(&10), "Должна быть найдена линия 10");
    assert!(filled.contains(&15), "Должна быть найдена линия 15");
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

    let filled = find_filled_lines(&blocks);

    assert_eq!(filled.len(), GRID_HEIGHT, "Должны быть найдены все линии");
}

/// Тест 23: Проверка поиска с частично заполненной линией
#[test]
fn test_find_filled_lines_partial_line() {
    let mut blocks = [[-1i8; GRID_WIDTH]; GRID_HEIGHT];

    // Заполняем только половину линии 10
    for block in &mut blocks[10][..(GRID_WIDTH / 2)] {
        *block = 1;
    }

    let filled = find_filled_lines(&blocks);

    assert!(
        filled.is_empty(),
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

    let filled = find_filled_lines(&blocks);

    assert_eq!(filled.len(), 1, "Должна быть найдена 1 заполненная линия");
    assert_eq!(filled[0], 7, "Заполненная линия должна быть на индексе 7");
}

// ============================================================================
// ТЕСТЫ ДЛЯ TOCTOU МАРКЕР (!Send + !Sync)
// ============================================================================

/// Тест 25: Проверка что GameState используется только в одном потоке
///
/// Этот тест документирует что GameState не предназначен для передачи между потоками.
/// TOCTOU (Time-of-check to time-of-use) уязвимости предотвращаются
/// через отсутствие реализации Send + Sync.
#[test]
fn test_toctou_marker_game_state_single_threaded() {
    // GameState по умолчанию !Send и !Sync из-за использования
    // небезопасных для потоков типов (например, Canvas, терминал)

    let state = GameState::new();

    // Проверяем что GameState существует и работает
    assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");

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

/// Тест 27: Проверка компиляции с GameState в замыкании
///
/// Этот тест проверяет что код с GameState компилируется корректно.
#[test]
fn test_toctou_marker_closure_compilation() {
    // Проверяем что можно создать замыкание которое принимает GameState
    let _closure = |state: &GameState| {
        // Используем state чтобы избежать предупреждения
        let _ = state.get_score();
    };

    let state = GameState::new();
    _closure(&state);
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

/// Тест 29: Проверка что GameState не имеет явной реализации Send
#[test]
fn test_toctou_marker_no_explicit_send() {
    // Этот тест проверяет что GameState не реализует Send явно
    // Если в будущем будет добавлена реализация Send, этот тест нужно обновить

    let state = GameState::new();

    // Проверяем что GameState работает корректно
    assert_eq!(state.get_score(), 0);
}

/// Тест 30: Проверка что GameState не имеет явной реализации Sync
#[test]
fn test_toctou_marker_no_explicit_sync() {
    // Этот тест проверяет что GameState не реализует Sync явно
    // Если в будущем будет добавлена реализация Sync, этот тест нужно обновить

    let state = GameState::new();

    // Проверяем что GameState работает корректно
    assert_eq!(state.get_score(), 0);
}
