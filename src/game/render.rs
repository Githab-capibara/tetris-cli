//! Модуль отрисовки и анимаций.
//!
//! # Ответственность
//! - Отрисовка игрового поля и границ
//! - Отрисовка фигур (текущей, призрачной, следующей, удержанной)
//! - Отрисовка интерфейса (счёт, уровень, линии, комбо, таймер)
//! - Анимации (мигание Hard Drop)
//!
//! # Зависимости
//! - [`state.rs`](super::state): константы отрисовки
//! - [`view.rs`](super::view): `GameView` для отрисовки
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`, `SHAPE_COLORS`
//! - [`io.rs`](crate::io): `Canvas`, `GRID_WIDTH`, `GRID_HEIGHT`
//!
//! ## Основные функции:
//! - [`draw()`] - основная функция отрисовки кадра
//! - [`update_cached_strings_extended()`] - обновление кэшированных строк UI
//!
//! ## Архитектурные заметки
//! Этот модуль использует `GameView` для уменьшения связанности с `GameState`.
//! TODO (#архитектура): Выделить отрисовку фигур в отдельный модуль `shape_renderer.rs`
//! для улучшения тестируемости.
//!
//! ## Использование `GameView`
//!
//! Для отрисовки используется [`GameView`], который предоставляет
//! неизменяемое представление состояния игры. Это уменьшает coupling
//! между render.rs и `GameState`.
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::game::{GameState, GameView};
//! use tetris_cli::io::Canvas;
//!
//! let mut state = GameState::new();
//! let mut canvas = Canvas::new().unwrap();
//!
//! // Обновление кэшированных строк (требует mutable доступ)
//! update_cached_strings_extended(&mut state, high_score_display);
//!
//! // Создание GameView для отрисовки
//! let view = GameView::from_game_state(&state);
//!
//! // Отрисовка
//! draw(&view, &mut canvas);
//! ```
//!
//! ## Архитектурные заметки (Задача 2)
//! Функция `check_rows()` была перемещена в [`super::scoring::lines`] для:
//! - Улучшения разделения ответственности (отрисовка vs логика игры)
//! - Уменьшения связанности между модулями
//! - Улучшения тестируемости логики удаления линий

use super::constants::{
    BORDER, BORDER_COLOR, COMBO_X, COMBO_Y, DRAW_OFFSET_X, HIGH_SCORE_X, HIGH_SCORE_Y,
    HOLD_PREVIEW_X, HOLD_PREVIEW_Y, LEVEL_X, LEVEL_Y, LINES_X, LINES_Y, PREVIEW_X, PREVIEW_Y,
    PROGRESS_Y, SCORE_X, SCORE_Y, SHAPE_DRAW_OFFSET, SHAPE_OFFSET_X, SHAPE_OFFSET_Y, SPRINT_LINES,
    TIMER_Y,
};
use super::state::GameState;
use super::view::GameView;
use crate::io::{Canvas, DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};
use crate::tetromino::{Tetromino, SHAPE_COLORS};
use std::fmt::Write;
use termion::color::Reset;

/// Отрисовать текущее состояние игры.
///
/// # Аргументы
/// * `view` - представление игры для отрисовки
/// * `cnv` - канвас для отрисовки
///
/// # Пример
/// ```ignore
/// let state = GameState::new();
/// let view = GameView::from_game_state(&state);
/// draw(&view, &mut canvas);
/// ```
///
/// ## Примечания
/// - Функция принимает `GameView` вместо `GameState` для уменьшения coupling
/// - Кэшированные строки должны быть обновлены до создания `GameView`
/// - Используйте `update_cached_strings_extended()` перед созданием `GameView`
///
/// ## Архитектурные заметки (ARCH-2)
/// Основная логика отрисовки перемещена в методы `GameView`:
/// - `view.draw_field()` - отрисовка игрового поля
/// - `view.draw_shape()` - отрисовка текущей фигуры
pub fn draw(view: &GameView, cnv: &mut Canvas) {
    cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

    // Отрисовка счёта и рекорда
    cnv.draw_string(view.score, (SCORE_X, SCORE_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(
        view.high_score,
        (HIGH_SCORE_X, HIGH_SCORE_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(view.level, (LEVEL_X, LEVEL_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(view.lines, (LINES_X, LINES_Y), BORDER_COLOR, &Reset);

    // Отрисовка счётчика комбо (ИСПРАВЛЕНИЕ #9: именованные константы)
    if let Some(combo) = view.combo {
        cnv.draw_string(combo, (COMBO_X, COMBO_Y), BORDER_COLOR, &Reset);
    }

    // Отрисовка игрового поля (ARCH-2: используется метод GameView)
    view.draw_field(cnv);

    // Отрисовка призрачной фигуры
    draw_ghost_shape(view, cnv);

    // Отрисовка текущей фигуры (ARCH-2: используется метод GameView)
    view.draw_shape(cnv);

    // Отрисовка следующей фигуры
    draw_next_shape(view, cnv);

    // Отрисовка удержанной фигуры
    draw_held_shape(view, cnv);

    // Отрисовка таймера для режима спринт (цель 40 линий)
    if view.mode.get_target_lines() == Some(40) {
        draw_sprint_timer(view, cnv);
    }

    cnv.flush();
}

/// Обновить кэшированные строки для отрисовки.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
///
/// # Исправление #4 (LOW): Оптимизация кэширования строк
/// - Используется `String::with_capacity(32)` в `RenderCache::new()` для предотвращения аллокаций
/// - Используется `clear()` + `write!()` для переиспользования буфера без deallocation
fn update_cached_strings(state: &mut GameState) {
    let score = state.score();
    let level = state.level();
    let lines_cleared = state.lines_cleared();
    let render_cache = state.get_render_cache_mut();

    if score != render_cache.last_cached_score {
        render_cache.cached_score_str.clear();
        if let Err(e) = write!(render_cache.cached_score_str, "{:10}", score) {
            eprintln!("Ошибка записи кэша счёта: {e}");
        }
        render_cache.last_cached_score = score;
    }

    if level != render_cache.last_cached_level {
        render_cache.cached_level_str.clear();
        if let Err(e) = write!(render_cache.cached_level_str, "{:10}", level) {
            eprintln!("Ошибка записи кэша уровня: {e}");
        }
        render_cache.last_cached_level = level;
    }

    if lines_cleared != render_cache.last_cached_lines {
        render_cache.cached_lines_str.clear();
        if let Err(e) = write!(render_cache.cached_lines_str, "{:10}", lines_cleared) {
            eprintln!("Ошибка записи кэша линий: {e}");
        }
        render_cache.last_cached_lines = lines_cleared;
    }
}

/// Обновить кэшированные строки (расширенная версия).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `high_score_display` - строка рекорда для кэширования
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
///
/// # Исправление #4 (LOW): Оптимизация кэширования строк
/// - Используется `String::with_capacity(32)` в `RenderCache::new()` для предотвращения аллокаций
/// - Используется `clear()` + `write!()` для переиспользования буфера без deallocation
pub fn update_cached_strings_extended(state: &mut GameState, high_score_display: &str) {
    update_cached_strings(state);

    // Кэширование строки рекорда
    {
        let render_cache = state.get_render_cache_mut();
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_high_score_str != high_score_display {
            render_cache.cached_high_score_str.clear();
            render_cache
                .cached_high_score_str
                .push_str(high_score_display);
        }
    }

    // Кэширование строки комбо
    let combo_counter = state.get_stats().combo_counter();
    {
        let render_cache = state.get_render_cache_mut();
        if render_cache.last_cached_combo != combo_counter {
            // Исправление #4: используем clear() + write!() для переиспользования буфера
            render_cache.cached_combo_str.clear();
            if combo_counter > 1 {
                let _ = write!(render_cache.cached_combo_str, "Комбо: x{combo_counter}");
            }
            render_cache.last_cached_combo = combo_counter;
        }
    }

    // Кэширование строки таймера для режима спринт
    if state.get_mode_trait().get_target_lines() == Some(40) {
        let elapsed = state.get_stats().get_elapsed_time();
        let timer_str = format!("Время: {elapsed:.2}с");
        let render_cache = state.get_render_cache_mut();
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_timer_str != timer_str {
            render_cache.cached_timer_str.clear();
            render_cache.cached_timer_str.push_str(&timer_str);
        }
    }
}

/// Отрисовать призрачную фигуру (точку приземления).
///
/// # Аргументы
/// * `view` - представление игры
/// * `cnv` - канвас для отрисовки
fn draw_ghost_shape(view: &GameView, cnv: &mut Canvas) {
    let mut ghost_shape = *view.curr_shape;

    let grid_height_i16 = i16::try_from(GRID_HEIGHT).unwrap_or(i16::MAX);
    let grid_width_i16 = i16::try_from(GRID_WIDTH).unwrap_or(i16::MAX);

    // Вычисляем расстояние до препятствия напрямую
    let ghost_block_y = ghost_shape.pos.1 as i16;
    let mut max_drop_distance = grid_height_i16;

    for &(coord_x, coord_y) in &ghost_shape.coords {
        let block_y = coord_y + ghost_block_y;
        let dist_to_floor = grid_height_i16 - 1 - block_y;

        let mut dist_to_block = dist_to_floor;
        for y in (block_y + 1)..grid_height_i16 {
            let x = coord_x + ghost_shape.pos.0 as i16;
            if x >= 0 && x < grid_width_i16 && view.blocks[y as usize][x as usize] != -1 {
                dist_to_block = y - block_y - 1;
                break;
            }
        }

        max_drop_distance = max_drop_distance.min(dist_to_block);
    }

    ghost_shape.pos.1 += f32::from(max_drop_distance);

    // Отрисовка призрачной фигуры (полупрозрачная)
    let (shape_x, shape_y) = ghost_shape.pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;
    let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

    for coord in ghost_shape.coords {
        let (coord_x, coord_y) = coord;
        let x = (coord_x + shape_block_x) * shape_width_i16 + SHAPE_OFFSET_X;
        let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y;

        cnv.draw_strs(
            &["░░"],
            (x as u16, y as u16),
            SHAPE_COLORS[ghost_shape.fg as usize],
            &Reset,
        );
    }
}

/// Отрисовать следующую фигуру (предпросмотр справа от поля).
///
/// # Аргументы
/// * `view` - представление игры
/// * `cnv` - канвас для отрисовки
fn draw_next_shape(view: &GameView, cnv: &mut Canvas) {
    draw_shape_preview(cnv, view.next_shape, PREVIEW_X, PREVIEW_Y, "След:", false);
}

/// Отрисовать удержанную фигуру (слева от поля).
///
/// # Аргументы
/// * `view` - представление игры
/// * `cnv` - канвас для отрисовки
fn draw_held_shape(view: &GameView, cnv: &mut Canvas) {
    if let Some(held) = view.held_shape {
        // Примечание: can_hold не доступен в GameView, так как это внутреннее состояние
        // Для полной поддержки можно добавить can_hold в GameView если нужно
        let is_faded = false;
        draw_shape_preview(
            cnv,
            held,
            HOLD_PREVIEW_X,
            HOLD_PREVIEW_Y,
            "Удерж:",
            is_faded,
        );
    }
}

/// Отрисовать предпросмотр фигуры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `shape` - фигура для отрисовки
/// * `pos_x` - позиция по X
/// * `pos_y` - позиция по Y
/// * `title` - заголовок
/// * `is_faded` - если true, рисовать тусклым цветом
///
/// # Исправление #25
/// Добавлена проверка всех границ экрана для предотвращения выхода за пределы канваса.
fn draw_shape_preview(
    cnv: &mut Canvas,
    shape: &Tetromino,
    pos_x: u16,
    pos_y: u16,
    title: &str,
    is_faded: bool,
) {
    cnv.draw_string(title, (pos_x, pos_y - 2), BORDER_COLOR, &Reset);

    let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

    // Исправление #25: используем константы DISP_WIDTH и DISP_HEIGHT для проверки границ
    for coord in shape.coords {
        let (coord_x, coord_y) = coord;
        let x = pos_x.cast_signed() + coord_x * shape_width_i16 + DRAW_OFFSET_X;
        let y = pos_y.cast_signed() + coord_y + 1;

        // Исправление #25: полная проверка всех границ
        // x >= 0 && y >= 0 - проверка на отрицательные координаты
        // x < DISP_WIDTH && y < DISP_HEIGHT - проверка на выход за пределы экрана
        if x >= 0 && y >= 0 && x < DISP_WIDTH as i16 && y < DISP_HEIGHT as i16 {
            let display_char = if is_faded { "░░" } else { SHAPE_STR };
            cnv.draw_strs(
                &[display_char],
                (x as u16, y as u16),
                SHAPE_COLORS[shape.fg as usize],
                &Reset,
            );
        }
    }
}

/// Отрисовать таймер для режима спринт.
///
/// # Аргументы
/// * `view` - представление игры
/// * `cnv` - канвас для отрисовки
fn draw_sprint_timer(view: &GameView, cnv: &mut Canvas) {
    // Форматируем таймер на лету из elapsed_time (ИСПРАВЛЕНИЕ #9: именованные константы)
    let timer_str = format!("Время: {:.2}с", view.elapsed_time);
    cnv.draw_string(&timer_str, (PREVIEW_X, TIMER_Y), BORDER_COLOR, &Reset);

    let progress = format!("Цель: {}/{}", view.lines_cleared, SPRINT_LINES);
    cnv.draw_string(&progress, (PREVIEW_X, PROGRESS_Y), BORDER_COLOR, &Reset);
}

// ============================================================================
// АРХИТЕКТУРНЫЕ ЗАМЕТКИ (Задача 2)
// ============================================================================
//
// Функции check_rows(), animate_clear(), find_filled_lines(), animate_line_clear(),
// remove_lines() и update_score_for_lines() были перемещены в super::scoring::lines
// для улучшения разделения ответственности между модулями отрисовки и логики игры.
//
// Для проверки и удаления линий используйте:
// ```
// use tetris_cli::game::scoring::check_rows;
// check_rows(&mut state);
// ```
