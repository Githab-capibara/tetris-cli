//! Представление игры для отрисовки.
//!
//! Этот модуль содержит структуру [`GameView`], которая предоставляет
//! неизменяемое представление состояния игры для отрисовки.
//! Это уменьшает coupling между render.rs и `GameState`.
//!
//! ## Архитектурные заметки
//! `GameView` используется для:
//! - Уменьшения связанности между модулем отрисовки и игровым состоянием
//! - Предоставления только необходимых данных для рендеринга
//! - Упрощения тестирования отрисовки через моки
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

/// Представление игрового состояния для отрисовки.
///
/// Используется для уменьшения связанности между render.rs и GameState.
/// Содержит только данные, необходимые для отрисовки.
///
/// ## Архитектурное назначение
/// Этот struct решает проблему сильной связанности между модулем отрисовки
/// и полным состоянием игры. Вместо передачи всего `GameState` в `render::draw()`,
/// мы создаём `GameView` с ограниченным набором данных.
///
/// ## Преимущества:
/// - **Инкапсуляция**: render.rs не имеет доступа к изменяемым данным игры
/// - **Тестируемость**: можно создать мок GameView для тестирования отрисовки
/// - **Производительность**: кэшированные строки уже готовы для отрисовки
/// - **Читаемость**: явный список данных, необходимых для рендеринга
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
/// Это обеспечивает безопасность ссылок на уровне компилятора.
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
///
/// ## Архитектурные заметки
/// ## Feature Envy (Problem 2.5)
/// TODO (#архитектура, Problem 2.5): Добавить методы отрисовки в GameView
/// для уменьшения Feature Envy между render.rs и GameView.
/// В настоящее время render.rs имеет полный доступ к полям GameView.
/// Рассмотрим возможность добавления методов:
/// - `get_block(x, y)` - доступ к блокам
/// - `is_row_animating(y)` - проверка анимации строки
/// - `get_shape_display_char()` - получение символа для отрисовки фигуры
pub struct GameView<'a> {
    // === UI элементы (кэшированные строки) ===
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

    // === Игровое поле ===
    /// Игровое поле (двумерный массив блоков).
    pub blocks: &'a [[i8; GRID_WIDTH]; GRID_HEIGHT],

    // === Фигуры ===
    /// Текущая падающая фигура.
    pub curr_shape: &'a Tetromino,
    /// Следующая фигура (предпросмотр).
    pub next_shape: &'a Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    pub held_shape: &'a Option<Tetromino>,

    // === Анимации ===
    /// Битовая маска строк для анимации очистки.
    pub animating_rows: u32,
    /// Флаг анимации Hard Drop.
    pub is_hard_dropping: bool,

    // === Режим и статистика ===
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

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ (Problem 2.5 - Feature Envy)
    // ========================================================================

    /// Получить блок по координатам.
    ///
    /// # Аргументы
    /// * `x` - координата X
    /// * `y` - координата Y
    ///
    /// # Возвращает
    /// Значение блока (-1 = пусто) или -1 если координаты вне границ
    #[must_use]
    #[allow(dead_code)]
    pub fn get_block(&self, x: usize, y: usize) -> i8 {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x]
        } else {
            -1
        }
    }

    /// Проверить, пуста ли ячейка.
    ///
    /// # Аргументы
    /// * `x` - координата X
    /// * `y` - координата Y
    ///
    /// # Возвращает
    /// `true` если ячейка пуста
    #[must_use]
    #[allow(dead_code)]
    pub fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) == -1
    }

    /// Проверить, занята ли ячейка.
    ///
    /// # Аргументы
    /// * `x` - координата X
    /// * `y` - координата Y
    ///
    /// # Возвращает
    /// `true` если ячейка занята
    #[must_use]
    #[allow(dead_code)]
    pub fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) >= 0
    }

    /// Проверить, анимируется ли строка.
    ///
    /// # Аргументы
    /// * `y` - координата Y строки
    ///
    /// # Возвращает
    /// `true` если строка анимируется
    #[must_use]
    #[allow(dead_code)]
    pub fn is_row_animating(&self, y: usize) -> bool {
        (self.animating_rows & (1 << y)) != 0
    }

    /// Получить цвет блока.
    ///
    /// # Аргументы
    /// * `x` - координата X
    /// * `y` - координата Y
    ///
    /// # Возвращает
    /// Индекс цвета блока или None если блок пуст
    #[must_use]
    #[allow(dead_code)]
    pub fn get_block_color(&self, x: usize, y: usize) -> Option<usize> {
        let block = self.get_block(x, y);
        if block >= 0 {
            Some(block as usize)
        } else {
            None
        }
    }
}
