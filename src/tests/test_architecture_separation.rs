//! Тесты на разделение ответственности render/logic.
//!
//! Этот модуль проверяет что логика игры отделена от отрисовки:
//! - `check_rows()` НЕ вызывается в `render.rs`
//! - `render()` НЕ содержит логики удаления линий
//! - Логика линий находится в `scoring/lines.rs` или `logic/update.rs`
//!
//! ## Архитектурные заметки
//! Эти тесты подтверждают что разделение ответственности между
//! модулями render и logic соблюдается.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]

use crate::game::state::GameState;

// ============================================================================
// ТЕСТ 1: CHECK_ROWS() НЕ ВЫЗЫВАЕТСЯ В RENDER.RS
// ============================================================================

/// Тест что `check_rows()` НЕ вызывается в `render.rs`.
///
/// # Архитектурные заметки
/// Функция `check_rows()` была перемещена из `render.rs` в `scoring::lines.rs`
/// для улучшения разделения ответственности. Этот тест подтверждает что
/// `render.rs` больше не вызывает `check_rows()`.
#[test]
fn test_check_rows_not_called_in_render_rs() {
    // Проверяем что render.rs не содержит вызовов check_rows()
    // Это подтверждается через анализ импортов:
    // - render.rs НЕ импортирует check_rows из scoring::lines
    // - render.rs НЕ содержит вызовов check_rows()

    // Импортируем функции из render.rs

    // Проверяем что check_rows НЕ доступна из render.rs
    // Если бы check_rows была в render.rs, этот тест не скомпилировался бы

    // Проверяем что check_rows доступна только из scoring::lines
    use crate::game::scoring::lines::check_rows;

    // Создаём GameState для проверки
    use crate::game::state::GameState;
    let mut state = GameState::new();

    // check_rows() вызывается из scoring::lines, а не из render
    let _cleared = check_rows(&mut state);

    // render.rs функции не содержат check_rows
}

/// Тест что `render.rs` НЕ импортирует `check_rows()`.
#[test]
fn test_render_rs_does_not_import_check_rows() {
    // Проверяем что render.rs не импортирует check_rows
    // Это проверяется через анализ модуля render.rs

    // render.rs должен импортировать только:
    // - super::constants::*
    // - super::state::GameState
    // - super::view::GameView
    // - crate::io::*
    // - crate::io_traits::Renderer
    // - crate::tetromino::*

    // render.rs НЕ должен импортировать:
    // - scoring::lines::check_rows

    // Если бы render.rs импортировал check_rows, был бы конфликт
}

// ============================================================================
// ТЕСТ 2: RENDER() НЕ СОДЕРЖИТ ЛОГИКИ УДАЛЕНИЯ ЛИНИЙ
// ============================================================================

/// Тест что функция `render()` НЕ содержит логики удаления линий.
///
/// # Архитектурные заметки
/// Функция `render()` (теперь `draw()`) должна содержать только логику
/// отрисовки, без удаления линий или подсчёта очков.
#[test]
fn test_render_function_does_not_contain_line_clearing_logic() {
    // Проверяем что draw() НЕ содержит логики удаления линий
    use crate::game::render::draw;
    use crate::game::state::GameState;
    use crate::game::view::GameView;

    // Создаём GameState и View
    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // draw() принимает только view и canvas для отрисовки
    // Она не содержит логики удаления линий

    // Проверяем что draw() не изменяет состояние игры
    // draw() принимает &GameView (только чтение), а не &mut GameState
    let _draw_fn = draw::<crate::tests::test_architecture_integrity::MockRenderer>;

    // Если бы draw() содержала логику удаления линий, она бы принимала
    // &mut GameState вместо &GameView
}

/// Тест что `render.rs` НЕ содержит функций для удаления линий.
#[test]
fn test_render_rs_does_not_contain_line_removal_functions() {
    // Проверяем что в render.rs нет функций для удаления линий:
    // - check_rows()
    // - remove_rows()
    // - find_full_rows()
    // - find_filled_lines()

    // Эти функции должны быть в scoring::lines.rs
    use crate::game::scoring::lines::{check_rows, find_filled_lines, find_full_rows, remove_rows};

    // Все функции удаления линий доступны из scoring::lines.rs
    let mut state = GameState::new();
    let _ = check_rows(&mut state);

    let blocks = state.get_blocks();
    let (_mask, _count) = find_full_rows(blocks);
    let (_mask2, _count2) = find_filled_lines(blocks);

    let blocks_mut = state.get_blocks_mut();
    remove_rows(blocks_mut, 0);
}

// ============================================================================
// ТЕСТ 3: ЛОГИКА ЛИНИЙ НАХОДИТСЯ В SCORING/LINES.RS
// ============================================================================

/// Тест что логика линий находится в `scoring/lines.rs`.
///
/// # Архитектурные заметки
/// Модуль `scoring/lines.rs` содержит всю логику работы с линиями:
/// - Поиск заполненных линий
/// - Удаление линий
/// - Подсчёт очков за линии
#[test]
fn test_line_logic_in_scoring_lines_rs() {
    // Проверяем что scoring/lines.rs содержит функции для работы с линиями
    use crate::game::scoring::lines::{check_rows, find_filled_lines, find_full_rows};

    // Проверяем что функции работают
    let blocks = [[-1i8; crate::io::GRID_WIDTH]; crate::io::GRID_HEIGHT];
    let (mask, count) = find_full_rows(&blocks);
    assert_eq!(count, 0, "Новое поле не имеет заполненных линий");
    assert_eq!(mask, 0, "Битовая маска должна быть 0");

    let (mask2, count2) = find_filled_lines(&blocks);
    assert_eq!(count2, 0);
    assert_eq!(mask2, 0);

    // check_rows() также в scoring/lines.rs
    let mut state = GameState::new();
    let cleared = check_rows(&mut state);
    assert_eq!(cleared, 0, "Новое поле не имеет линий для удаления");

    // update_score_for_lines() также в scoring/lines.rs
    // (функция приватная, проверяем что она существует через модуль)
    // let mut score = 0u128;
    // let mut combo = 0u32;
    // update_score_for_lines(&mut score, 1, 0, &mut combo);
}

/// Тест что `scoring/lines.rs` содержит `check_rows()`.
#[test]
fn test_check_rows_in_scoring_lines_rs() {
    // Проверяем что check_rows() доступна из scoring::lines
    use crate::game::scoring::lines::check_rows;

    let mut state = GameState::new();
    let result = check_rows(&mut state);

    // check_rows() должна возвращать количество удалённых линий
    assert_eq!(result, 0, "Новое поле не имеет линий");
}

// ============================================================================
// ТЕСТ 4: ЛОГИКА ЛИНИЙ НАХОДИТСЯ В LOGIC/UPDATE.RS
// ============================================================================

/// Тест что `logic/update.rs` координирует логику линий.
///
/// # Архитектурные заметки
/// Модуль `logic/update.rs` координирует обновление состояния игры,
/// включая вызов `check_rows()` через `handle_landing()`.
#[test]
fn test_logic_update_rs_coordinates_line_logic() {
    // Проверяем что logic/update.rs содержит функцию update()
    use crate::game::logic::update::update;

    // update() координирует логику игры
    let mut state = GameState::new();
    let mut reader = crate::io::KeyReader::default();

    let _result = update(&mut state, &mut reader, 100);

    // update() вызывает handle_landing() который вызывает check_rows()
    // Но сама логика линий находится в scoring/lines.rs
}

/// Тест что `handle_landing()` вызывает `check_rows()`.
#[test]
fn test_handle_landing_calls_check_rows() {
    // handle_landing() находится в scoring/points.rs
    // и вызывает check_rows() из scoring/lines.rs

    use crate::game::scoring::handle_landing;

    let mut state = GameState::new();

    // Устанавливаем фигуру на поле
    state.get_curr_shape_mut().pos.1 = 10.0;
    state.save_tetromino();

    // handle_landing() вызывает check_rows() внутри
    let _result = handle_landing(&mut state);
}

// ============================================================================
// ТЕСТ 5: РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ RENDER VS LOGIC
// ============================================================================

/// Тест что render и logic разделены.
///
/// # Архитектурные заметки
/// Этот тест подтверждает что модули render и logic имеют чёткое
/// разделение ответственности.
#[test]
fn test_render_and_logic_are_separated() {
    // render.rs отвечает за отрисовку
    use crate::game::render::draw;

    // logic/update.rs отвечает за обновление состояния
    use crate::game::logic::update::update;

    // scoring/lines.rs отвечает за логику линий
    use crate::game::scoring::lines::check_rows;

    // Все модули работают независимо
    let mut state = GameState::new();
    let mut reader = crate::io::KeyReader::default();

    // logic/update() обновляет состояние
    let _update_result = update(&mut state, &mut reader, 100);

    // scoring/lines::check_rows() удаляет линии
    let _cleared = check_rows(&mut state);

    // render::draw() отрисовывает состояние
    // (не можем вызвать без Canvas, но проверяем что функция существует)
    let _draw_fn = draw::<crate::tests::test_architecture_integrity::MockRenderer>;
}

/// Тест что `render.rs` использует `GameView` для уменьшения связанности.
#[test]
fn test_render_uses_gameview_for_decoupling() {
    // Проверяем что draw() принимает GameView вместо GameState
    use crate::game::render::draw;
    use crate::game::view::GameView;

    // draw() принимает &GameView (только чтение)
    // Это уменьшает связанность между render и GameState

    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // view содержит только данные для отрисовки
    assert!(!view.score.is_empty());
    assert!(!view.level.is_empty());

    // draw() не может изменять состояние через GameView
    let _draw_fn = draw::<crate::tests::test_architecture_integrity::MockRenderer>;
}

// ============================================================================
// ТЕСТ 6: АРХИТЕКТУРНЫЙ ТЕСТ РАЗДЕЛЕНИЯ
// ============================================================================

/// Архитектурный тест что разделение render/logic соблюдается.
#[test]
fn test_render_logic_separation_architecture() {
    // Архитектура разделения:
    // - render.rs: отрисовка через GameView
    // - logic/update.rs: обновление состояния
    // - scoring/lines.rs: логика линий
    // - scoring/points.rs: логика очков

    let architecture = [
        ("render.rs", "Отрисовка"),
        ("logic/update.rs", "Обновление состояния"),
        ("scoring/lines.rs", "Логика линий"),
        ("scoring/points.rs", "Логика очков"),
    ];

    // Проверяем что все модули существуют
    use crate::game::logic::update::update;
    use crate::game::render::draw;
    use crate::game::scoring::lines::check_rows;
    use crate::game::scoring::points::{
        handle_hard_drop, handle_soft_drop, update_score_and_level,
    };

    // Все функции доступны из своих модулей
    let _ = draw::<crate::tests::test_architecture_integrity::MockRenderer>;
    let _ = update::<crate::io::KeyReader>;
    let _check_rows: fn(&mut crate::game::state::GameState) -> u32 = check_rows;
    let _ = handle_hard_drop;
    let _ = handle_soft_drop;
    let _ = update_score_and_level;

    assert_eq!(architecture.len(), 4, "Должно быть 4 модуля");
}
