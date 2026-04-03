//! Модуль отрисовки и анимаций.
//!
//! # Ответственность
//! - Отрисовка игрового поля и границ
//! - Отрисовка фигур (текущей, призрачной, следующей, удержанной)
//! - Отрисовка интерфейса (счёт, уровень, линии, комбо, таймер)
//! - Анимации (мигание Hard Drop)
//!
//! ## Архитектурные заметки
//! Архитектурное улучшение 2026-04-01 (SOC2): Разделение render.rs на подмодули:
//! - `cache.rs` - обновление кэшированных строк UI
//!
//! ## Подмодули
pub mod cache;

// Re-export функций из cache.rs
pub use cache::update_cached_strings_extended;

// ============================================================================
// ОСНОВНЫЕ ФУНКЦИИ ОТРИСОВКИ
// ============================================================================

use super::view::GameView;
use crate::constants::{
    BORDER, BORDER_COLOR, COMBO_X, COMBO_Y, HIGH_SCORE_X, HIGH_SCORE_Y, LEVEL_X, LEVEL_Y, LINES_X,
    LINES_Y, PREVIEW_X, PROGRESS_Y, SCORE_X, SCORE_Y, SPRINT_LINES, TIMER_Y,
};
use crate::io_traits::Renderer;
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
    cnv.draw_string(view.score_str(), (SCORE_X, SCORE_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(
        view.high_score_str(),
        (HIGH_SCORE_X, HIGH_SCORE_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(view.level_str(), (LEVEL_X, LEVEL_Y), BORDER_COLOR, &Reset);
    cnv.draw_string(view.lines_str(), (LINES_X, LINES_Y), BORDER_COLOR, &Reset);

    // Отрисовка счётчика комбо
    if let Some(combo) = view.combo_str() {
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
    if view.mode().get_target_lines() == Some(40) {
        draw_sprint_timer(view, cnv);
    }

    cnv.flush();
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
    let timer_str = format!("Время: {:.2}с", view.elapsed_time());
    cnv.draw_string(&timer_str, (PREVIEW_X, TIMER_Y), BORDER_COLOR, &Reset);

    let progress = format!("Цель: {}/{}", view.lines_cleared(), SPRINT_LINES);
    cnv.draw_string(&progress, (PREVIEW_X, PROGRESS_Y), BORDER_COLOR, &Reset);
}
