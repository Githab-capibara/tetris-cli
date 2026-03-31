//! Представление игры для отрисовки.
//!
//! Модуль содержит структуру `GameView` для предоставления
//! неизменяемого представления состояния игры.
//! Уменьшает связанность между render.rs и GameState.

use super::mode_trait::GameModeTrait;
use super::state::GameState;
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
    /// Режим игры (объект трейта).
    pub mode: &'a dyn GameModeTrait,
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
            score: state.get_cached_score_str(),
            level: state.get_cached_level_str(),
            lines: state.get_cached_lines_str(),
            combo: state.get_cached_combo_str().into(),
            high_score: state.get_cached_high_score_str(),
            blocks: state.get_blocks(),
            curr_shape: state.curr_shape(),
            next_shape: state.next_shape(),
            held_shape: state.get_held_shape_ref(),
            animating_rows: state.get_animating_rows_mask(),
            is_hard_dropping: state.is_hard_dropping(),
            mode: state.get_mode_trait(),
            lines_cleared: state.lines_cleared(),
            elapsed_time: state.stats().get_elapsed_time(),
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

    /// Получить символ для отрисовки фигуры.
    ///
    /// # Возвращает
    /// Символ для отрисовки блока фигуры
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// let ch = view.get_shape_display_char();
    /// ```
    #[must_use]
    #[allow(dead_code)]
    pub fn get_shape_display_char(&self) -> &str {
        use crate::io::SHAPE_STR;
        SHAPE_STR
    }

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ UI (Problem 2.5 - Feature Envy)
    // ========================================================================
    // TODO (#архитектура, Problem 2.5): Добавить методы отрисовки в GameView
    // для уменьшения Feature Envy между render.rs и GameView.
    // Эти методы предоставляют готовые строки для отрисовки UI.

    /// Получить строку счёта для отрисовки.
    ///
    /// # Возвращает
    /// Кэшированную строку счёта
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// println!("Счёт: {}", view.score_str());
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn score_str(&self) -> &str {
        self.score
    }

    /// Получить строку уровня для отрисовки.
    ///
    /// # Возвращает
    /// Кэшированную строку уровня
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// println!("Уровень: {}", view.level_str());
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn level_str(&self) -> &str {
        self.level
    }

    /// Получить строку линий для отрисовки.
    ///
    /// # Возвращает
    /// Кэшированную строку количества линий
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// println!("Линии: {}", view.lines_str());
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn lines_str(&self) -> &str {
        self.lines
    }

    /// Получить строку комбо для отрисовки.
    ///
    /// # Возвращает
    /// Кэшированную строку комбо (None если комбо нет)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// if let Some(combo) = view.combo_str() {
    ///     println!("{}", combo);
    /// }
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn combo_str(&self) -> Option<&str> {
        self.combo
    }

    /// Получить строку рекорда для отрисовки.
    ///
    /// # Возвращает
    /// Кэшированную строку рекорда
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// println!("Рекорд: {}", view.high_score_str());
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn high_score_str(&self) -> &str {
        self.high_score
    }

    /// Получить строку таймера для отрисовки.
    ///
    /// # Возвращает
    /// Отформатированную строку таймера (только для режима Sprint)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// if let Some(timer) = view.timer_str() {
    ///     println!("{}", timer);
    /// }
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации
    #[must_use]
    pub fn timer_str(&self) -> Option<String> {
        // Спринт режим имеет цель 40 линий
        if self.mode.get_target_lines() == Some(40) {
            Some(format!("Время: {:.2}с", self.elapsed_time))
        } else {
            None
        }
    }

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ (Problem 2.5 - Feature Envy)
    // ========================================================================
    // Эти методы предоставляют готовую отрисовку через Renderer,
    // уменьшая связанность между render.rs и GameView.

    /// Отрисовать игровое поле на canvas.
    ///
    /// ## Архитектурные заметки
    /// Этот метод перемещён из `render.rs` для уменьшения Feature Envy.
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - холст для отрисовки (реализует трейт Renderer)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_field(&mut canvas);
    /// ```
    pub fn draw_field<R>(&self, canvas: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::io::{SHAPE_STR, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        for y in 0..GRID_HEIGHT {
            let is_animating = (self.animating_rows & (1 << y)) != 0;
            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] != -1 {
                    let color_idx = self.blocks[y][x] as usize;
                    if color_idx < SHAPE_COLORS.len() {
                        canvas.draw_strs(
                            &[SHAPE_STR],
                            (
                                (x * SHAPE_WIDTH + 2) as u16,
                                (y + 5) as u16, // SHAPE_DRAW_OFFSET = 5
                            ),
                            SHAPE_COLORS[color_idx],
                            &Reset,
                        );
                    }
                }
            }
        }
    }

    /// Отрисовать текущую фигуру.
    ///
    /// ## Архитектурные заметки
    /// Этот метод перемещён из `render.rs` для уменьшения Feature Envy.
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - холст для отрисовки (реализует трейт Renderer)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_shape(&mut canvas);
    /// ```
    pub fn draw_shape<R>(&self, canvas: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::io::{SHAPE_STR, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = (coord_x + shape_block_x) * shape_width_i16 + 2; // SHAPE_OFFSET_X = 2
            let y = coord_y + shape_block_y + 5; // SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y = 5 + 0

            if x >= 0 {
                canvas.draw_strs(
                    &[SHAPE_STR],
                    (x as u16, y as u16),
                    SHAPE_COLORS[self.curr_shape.fg as usize],
                    &Reset,
                );
            }
        }
    }

    /// Отрисовать UI (счёт, уровень, линии, комбо, рекорд).
    ///
    /// # Аргументы
    /// * `renderer` - объект для отрисовки
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_ui(&mut canvas);
    /// ```
    #[allow(dead_code)] // Будет использоваться в будущей рефакторизации render.rs
    pub fn draw_ui<R>(&self, renderer: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::game::constants::{
            BORDER_COLOR, COMBO_X, COMBO_Y, HIGH_SCORE_X, HIGH_SCORE_Y, LEVEL_X, LEVEL_Y, LINES_X,
            LINES_Y, SCORE_X, SCORE_Y,
        };
        use termion::color::Reset;

        // Отрисовка счёта
        renderer.draw_string(
            &format!("{:10}", self.score),
            (SCORE_X, SCORE_Y),
            BORDER_COLOR,
            &Reset,
        );

        // Отрисовка рекорда
        renderer.draw_string(
            &format!("{:10}", self.high_score),
            (HIGH_SCORE_X, HIGH_SCORE_Y),
            BORDER_COLOR,
            &Reset,
        );

        // Отрисовка уровня
        renderer.draw_string(
            &format!("{:10}", self.level),
            (LEVEL_X, LEVEL_Y),
            BORDER_COLOR,
            &Reset,
        );

        // Отрисовка линий
        renderer.draw_string(
            &format!("{:10}", self.lines),
            (LINES_X, LINES_Y),
            BORDER_COLOR,
            &Reset,
        );

        // Отрисовка комбо (если есть)
        if let Some(combo) = self.combo {
            renderer.draw_string(combo, (COMBO_X, COMBO_Y), BORDER_COLOR, &Reset);
        }
    }

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ ФИГУР (C2: Улучшение инкапсуляции render.rs)
    // ========================================================================
    // Эти методы перемещены из render.rs для уменьшения связанности.

    /// Отрисовать призрачную фигуру (точку приземления).
    ///
    /// ## Архитектурные заметки
    /// Этот метод перемещён из `render.rs` для уменьшения Feature Envy.
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - холст для отрисовки (реализует трейт Renderer)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_ghost(&mut canvas);
    /// ```
    pub fn draw_ghost<R>(&self, canvas: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::io::{GRID_HEIGHT, GRID_WIDTH, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        let mut ghost_shape = *self.curr_shape;

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
                if x >= 0 && x < grid_width_i16 && self.blocks[y as usize][x as usize] != -1 {
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
            let x = (coord_x + shape_block_x) * shape_width_i16 + 2; // SHAPE_OFFSET_X
            let y = coord_y + shape_block_y + 5; // SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y

            canvas.draw_strs(
                &["░░"],
                (x as u16, y as u16),
                SHAPE_COLORS[ghost_shape.fg as usize],
                &Reset,
            );
        }
    }

    /// Отрисовать следующую фигуру (предпросмотр справа от поля).
    ///
    /// ## Архитектурные заметки
    /// Этот метод перемещён из `render.rs` для уменьшения Feature Envy.
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - холст для отрисовки (реализует трейт Renderer)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_next_shape(&mut canvas);
    /// ```
    pub fn draw_next_shape<R>(&self, canvas: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::game::constants::{PREVIEW_X, PREVIEW_Y};
        self.draw_shape_preview(
            canvas,
            self.next_shape,
            PREVIEW_X,
            PREVIEW_Y,
            "След:",
            false,
        );
    }

    /// Отрисовать удержанную фигуру (слева от поля).
    ///
    /// ## Архитектурные заметки
    /// Этот метод перемещён из `render.rs` для уменьшения Feature Envy.
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - холст для отрисовки (реализует трейт Renderer)
    ///
    /// # Пример
    /// ```ignore
    /// let view = GameView::from_game_state(&state);
    /// view.draw_held_shape(&mut canvas);
    /// ```
    pub fn draw_held_shape<R>(&self, canvas: &mut R)
    where
        R: crate::io_traits::Renderer,
    {
        use crate::game::constants::{HOLD_PREVIEW_X, HOLD_PREVIEW_Y};
        if let Some(held) = self.held_shape {
            let is_faded = false; // can_hold не доступен в GameView
            self.draw_shape_preview(
                canvas,
                held,
                HOLD_PREVIEW_X,
                HOLD_PREVIEW_Y,
                "Удерж:",
                is_faded,
            );
        }
    }

    /// Отрисовать предпросмотр фигуры (вспомогательный метод).
    ///
    /// ## Dependency Inversion (H1)
    /// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
    ///
    /// # Аргументы
    /// * `canvas` - канвас для отрисовки (реализует трейт Renderer)
    /// * `shape` - фигура для отрисовки
    /// * `pos_x` - позиция по X
    /// * `pos_y` - позиция по Y
    /// * `title` - заголовок
    /// * `is_faded` - если true, рисовать тусклым цветом
    #[allow(clippy::unused_self)]
    fn draw_shape_preview<R>(
        &self,
        canvas: &mut R,
        shape: &crate::tetromino::Tetromino,
        pos_x: u16,
        pos_y: u16,
        title: &str,
        is_faded: bool,
    ) where
        R: crate::io_traits::Renderer,
    {
        use crate::game::constants::{BORDER_COLOR, DISP_HEIGHT, DISP_WIDTH, DRAW_OFFSET_X};
        use crate::io::{SHAPE_STR, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        canvas.draw_string(title, (pos_x, pos_y - 2), BORDER_COLOR, &Reset);

        let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

        for coord in shape.coords {
            let (coord_x, coord_y) = coord;
            let x = pos_x.cast_signed() + coord_x * shape_width_i16 + DRAW_OFFSET_X;
            let y = pos_y.cast_signed() + coord_y + 1;

            // Проверка всех границ
            if x >= 0 && y >= 0 && x < DISP_WIDTH as i16 && y < DISP_HEIGHT as i16 {
                let display_char = if is_faded { "░░" } else { SHAPE_STR };
                canvas.draw_strs(
                    &[display_char],
                    (x as u16, y as u16),
                    SHAPE_COLORS[shape.fg as usize],
                    &Reset,
                );
            }
        }
    }
}
