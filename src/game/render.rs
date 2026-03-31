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
    BORDER, BORDER_COLOR, COMBO_X, COMBO_Y, HIGH_SCORE_X, HIGH_SCORE_Y, LEVEL_X, LEVEL_Y, LINES_X,
    LINES_Y, PREVIEW_X, PROGRESS_Y, SCORE_X, SCORE_Y, SPRINT_LINES, TIMER_Y,
};
use super::state::GameState;
use super::view::GameView;
use crate::io_traits::Renderer;
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
/// ## Архитектурные заметки (ARCH-2, C2, H1)
/// Основная логика отрисовки делегирована методам `GameView`:
/// - `view.draw_field()` - отрисовка игрового поля
/// - `view.draw_shape()` - отрисовка текущей фигуры
/// - `view.draw_ghost()` - отрисовка призрачной фигуры
/// - `view.draw_next_shape()` - отрисовка следующей фигуры
/// - `view.draw_held_shape()` - отрисовка удержанной фигуры
///
/// ## Dependency Inversion (H1)
/// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
pub fn draw<R: Renderer>(view: &GameView, cnv: &mut R) {
    cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

    // Отрисовка UI (счёт, уровень, линии, комбо, рекорд)
    cnv.draw_string(view.score, (SCORE_X, SCORE_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(
        view.high_score,
        (HIGH_SCORE_X, HIGH_SCORE_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(view.level, (LEVEL_X, LEVEL_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(view.lines, (LINES_X, LINES_Y), BORDER_COLOR, &Reset);

    // Отрисовка счётчика комбо
    if let Some(combo) = view.combo {
        cnv.draw_string(combo, (COMBO_X, COMBO_Y), BORDER_COLOR, &Reset);
    }

    // Отрисовка игрового поля (C2: используется метод GameView)
    view.draw_field(cnv);

    // Отрисовка призрачной фигуры (C2: используется метод GameView)
    view.draw_ghost(cnv);

    // Отрисовка текущей фигуры (C2: используется метод GameView)
    view.draw_shape(cnv);

    // Отрисовка следующей фигуры (C2: используется метод GameView)
    view.draw_next_shape(cnv);

    // Отрисовка удержанной фигуры (C2: используется метод GameView)
    view.draw_held_shape(cnv);

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
    let combo_counter = state.stats().combo_counter();
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
        let elapsed = state.stats().get_elapsed_time();
        let timer_str = format!("Время: {elapsed:.2}с");
        let render_cache = state.get_render_cache_mut();
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_timer_str != timer_str {
            render_cache.cached_timer_str.clear();
            render_cache.cached_timer_str.push_str(&timer_str);
        }
    }
}

/// Отрисовать таймер для режима спринт.
///
/// # Аргументы
/// * `view` - представление игры
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
///
/// ## Dependency Inversion (H1)
/// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
fn draw_sprint_timer<R: Renderer>(view: &GameView, cnv: &mut R) {
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
