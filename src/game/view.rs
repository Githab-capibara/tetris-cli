//! Представление игры для отрисовки.
//!
//! Этот модуль содержит структуру [`GameView`], которая предоставляет
//! неизменяемое представление состояния игры для отрисовки.
//! Это уменьшает coupling между render.rs и `GameState`.
//!
//! ## Пример использования
//! ```ignore
//! use tetris_cli::game::{GameState, GameView};
//! use tetris_cli::io::Canvas;
//!
//! let state = GameState::new();
//! let view = GameView::from_game_state(&state);
//!
//! // Использование view для отрисовки
//! // render::draw(&view, &mut canvas, high_score_display);
//! ```

use super::state::{GameMode, GameState};
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::Tetromino;

/// Представление игры для отрисовки.
///
/// Уменьшает coupling между render.rs и `GameState`, предоставляя
/// только необходимые данные для отрисовки через неизменяемые ссылки.
///
/// ## Структура
/// Содержит ссылки на все данные состояния игры, необходимые для отрисовки:
/// - Кэшированные строки UI (счёт, уровень, линии, комбо, рекорд)
/// - Игровое поле (блоки)
/// - Фигуры (текущая, следующая, удержанная)
/// - Флаги анимации
/// - Режим игры и статистику
///
/// ## Время жизни
/// Параметр `'a` гарантирует, что `GameView` не живёт дольше `GameState`.
///
/// ## Пример использования
/// ```ignore
/// let state = GameState::new();
/// let view = GameView::from_game_state(&state);
///
/// // Доступ к данным
/// println!("Счёт: {}", view.score);
/// println!("Уровень: {}", view.level);
/// ```
pub struct GameView<'a> {
    /// Кэшированная строка счёта.
    pub score: &'a str,
    /// Кэшированная строка уровня.
    pub level: &'a str,
    /// Кэшированная строка количества линий.
    pub lines: &'a str,
    /// Кэшированная строка комбо (None если комбо нет).
    pub combo: Option<&'a str>,
    /// Кэшированная строка рекорда.
    pub high_score: &'a str,
    /// Игровое поле (двумерный массив блоков).
    pub blocks: &'a [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Текущая падающая фигура.
    pub curr_shape: &'a Tetromino,
    /// Следующая фигура (предпросмотр).
    pub next_shape: &'a Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    pub held_shape: &'a Option<Tetromino>,
    /// Битовая маска строк для анимации очистки.
    pub animating_rows: u32,
    /// Флаг анимации Hard Drop.
    pub is_hard_dropping: bool,
    /// Режим игры (Классика/Спринт).
    pub mode: GameMode,
    /// Количество очищенных линий.
    pub lines_cleared: u32,
    /// Прошедшее время игры (в секундах).
    pub elapsed_time: f64,
}

impl<'a> GameView<'a> {
    /// Создать `GameView` из `GameState`.
    ///
    /// # Аргументы
    /// * `state` — ссылка на состояние игры
    ///
    /// # Возвращает
    /// `GameView` с ссылками на все необходимые данные для отрисовки
    ///
    /// # Пример
    /// ```ignore
    /// let state = GameState::new();
    /// let view = GameView::from_game_state(&state);
    ///
    /// // Использование view для отрисовки
    /// // render::draw(&view, &mut canvas, high_score_display);
    /// ```
    ///
    /// ## Примечания
    /// - Все поля `GameView` являются ссылками на данные `GameState`
    /// - `GameView` не владеет данными, только предоставляет доступ
    /// - Время жизни `'a` гарантирует безопасность ссылок
    pub fn from_game_state(state: &'a GameState) -> Self {
        GameView {
            score: &state.cached_score_str,
            level: &state.cached_level_str,
            lines: &state.cached_lines_str,
            combo: state.cached_combo_str.as_str().into(),
            high_score: &state.cached_high_score_str,
            blocks: state.get_blocks(),
            curr_shape: state.get_curr_shape(),
            next_shape: state.get_next_shape(),
            held_shape: state.get_held_shape_ref(),
            animating_rows: state.animating_rows_mask,
            is_hard_dropping: state.is_hard_dropping,
            mode: state.get_mode(),
            lines_cleared: state.lines_cleared,
            elapsed_time: state.stats.get_elapsed_time(),
        }
    }
}
