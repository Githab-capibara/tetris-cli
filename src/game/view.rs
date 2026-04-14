//! Представление игры для отрисовки.
//!
//! Модуль содержит структуру `GameView` для предоставления
//! неизменяемого представления состояния игры.
//! Уменьшает связанность между render.rs и `GameState`.

// Координаты ограничены GRID_WIDTH=10, GRID_HEIGHT=20, фрейм < u16::MAX,
// поэтому все приведения f32→i16, usize→i16, u16→i16, i16→u16 безопасны.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "Координаты ограничены размером поля (10x20), фрейм < u16::MAX"
)]

use super::mode_trait::GameModeTrait;
use super::state::GameState;
use crate::constants::{GRID_HEIGHT, GRID_WIDTH, SHAPE_DRAW_OFFSET, SHAPE_OFFSET_X};
use crate::tetromino::Tetromino;

/// Представление игрового состояния для отрисовки.
///
/// Используется для уменьшения связанности между render.rs и `GameState`.
/// Содержит только данные, необходимые для отрисовки.
///
/// # Время жизни
/// ## Параметр `'a`
/// Параметр времени жизни `'a` гарантирует что `GameView` не живёт дольше `GameState`.
/// Это обеспечивает безопасность ссылок на уровне компилятора.
///
/// ## Ограничения
/// - `GameView<'a>` не может пережить `GameState` из которого создан
/// - Компилятор автоматически проверяет время жизни ссылок
/// - Попытка использовать `GameView` после удаления `GameState` вызовет ошибку компиляции
///
/// ## Пример
/// ```ignore
/// fn render_frame(state: &GameState) {
///     let view = GameView::from_game_state(state);
///     // view живёт только пока существует state
///     render::draw(&view, &mut canvas);
/// } // view и state уничтожаются здесь
/// ```
///
/// ## Архитектурное назначение
/// Этот struct решает проблему сильной связанности между модулем отрисовки
/// и полным состоянием игры. Вместо передачи всего `GameState` в `render::draw()`,
/// мы создаём `GameView` с ограниченным набором данных.
///
/// ## Преимущества:
/// - **Инкапсуляция**: render.rs не имеет доступа к изменяемым данным игры
/// - **Тестируемость**: можно создать мок `GameView` для тестирования отрисовки
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
    // === UI элементы (кэшированные строки) ===
    /// Кэшированная строка счёта.
    pub(crate) score: &'a str,
    /// Кэшированная строка уровня.
    pub(crate) level: &'a str,
    /// Кэшированная строка количества линий.
    pub(crate) lines: &'a str,
    /// Кэшированная строка комбо (None если комбо нет).
    pub(crate) combo: Option<&'a str>,
    /// Кэшированная строка рекорда.
    pub(crate) high_score: &'a str,

    // === Игровое поле ===
    /// Игровое поле (двумерный массив блоков).
    pub(crate) blocks: &'a [[i8; GRID_WIDTH]; GRID_HEIGHT],

    // === Фигуры ===
    /// Текущая падающая фигура.
    pub(crate) curr_shape: &'a Tetromino,
    /// Следующая фигура (предпросмотр).
    pub(crate) next_shape: &'a Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    pub(crate) held_shape: Option<&'a Tetromino>,

    // === Анимации ===
    /// Битовая маска строк для анимации очистки.
    ///
    /// # Audit 2026-04-12, Issue 8
    /// Поле используется для передачи состояния анимации в рендерер.
    /// В будущем может быть использовано для мигания строк при очистке.
    #[allow(dead_code)]
    pub(crate) animating_rows: u32,

    // === Режим и статистика ===
    /// Режим игры (объект трейта).
    pub(crate) mode: &'a dyn GameModeTrait,
    /// Количество очищенных линий.
    pub(crate) lines_cleared: u32,
    /// Прошедшее время игры (в секундах).
    pub(crate) elapsed_time: f64,
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
    #[must_use = "Представление игры должно быть использовано"]
    pub fn from_game_state(state: &'a GameState) -> Self {
        // I127: Намеренно создаём новый GameView каждый кадр.
        // GameView содержит только ссылки и флаги (размер ~128 байт),
        // поэтому аллокация на стеке — дешёвая операция.
        // Избегание создания не требуется: это не узкое место производительности.
        GameView {
            score: state.get_cached_score_str(),
            level: state.get_cached_level_str(),
            lines: state.get_cached_lines_str(),
            combo: state.get_cached_combo_str().into(),
            high_score: state.get_cached_high_score_str(),
            blocks: state.get_blocks(),
            curr_shape: state.curr_shape(),
            next_shape: state.next_shape(),
            held_shape: state.held_shape(),
            animating_rows: state.animating_rows_mask(),
            mode: state.get_mode_trait(),
            lines_cleared: state.lines_cleared(),
            elapsed_time: state.stats().get_elapsed_time(),
        }
    }

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ (Problem 2.5 - Feature Envy)
    // ========================================================================

    // ========================================================================
    // МЕТОДЫ ОТРИСОВКИ UI (Problem 2.5 - Feature Envy)
    // ========================================================================
    // Эти методы предоставляют готовые строки для отрисовки UI.

    /// Получить строку счёта для отрисовки.
    #[must_use]
    pub const fn score_str(&self) -> &str {
        self.score
    }

    /// Получить строку уровня для отрисовки.
    #[must_use]
    pub const fn level_str(&self) -> &str {
        self.level
    }

    /// Получить строку линий для отрисовки.
    #[must_use]
    pub const fn lines_str(&self) -> &str {
        self.lines
    }

    /// Получить строку комбо для отрисовки.
    #[must_use]
    pub const fn combo_str(&self) -> Option<&str> {
        self.combo
    }

    /// Получить строку рекорда для отрисовки.
    #[must_use]
    pub const fn high_score_str(&self) -> &str {
        self.high_score
    }

    // ========================================================================
    // ГЕТТЕРЫ ДЛЯ ИНКАПСУЛИРОВАННЫХ ПОЛЕЙ (I014)
    // ========================================================================
    // Эти методы предоставляют доступ к полям GameView для внешнего кода
    // после изменения видимости с pub на pub(crate).

    /// Получить ссылку на режим игры.
    #[must_use]
    pub fn mode(&self) -> &dyn GameModeTrait {
        self.mode
    }

    /// Получить количество очищенных линий.
    #[must_use]
    pub const fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить прошедшее время игры.
    #[must_use]
    pub const fn elapsed_time(&self) -> f64 {
        self.elapsed_time
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
        use crate::constants::{SHAPE_STR, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let block_val = self.blocks[y][x];
                if block_val != -1 {
                    // i8 -> usize: проверка != -1 гарантирует значение 0..=127, безопасно
                    let color_idx = usize::try_from(block_val).unwrap_or(0);
                    if color_idx < SHAPE_COLORS.len() {
                        // usize -> u16: координаты ограничены GRID_WIDTH/HEIGHT (10x20)
                        let draw_x = u16::try_from(x * SHAPE_WIDTH + (SHAPE_OFFSET_X as usize))
                            .unwrap_or(u16::MAX);
                        let draw_y =
                            u16::try_from(y + (SHAPE_DRAW_OFFSET as usize)).unwrap_or(u16::MAX);
                        canvas.draw_strs(
                            &[SHAPE_STR],
                            (draw_x, draw_y),
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
        use crate::constants::{SHAPE_STR, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        let (shape_x, shape_y) = self.curr_shape.pos();
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_WIDTH)
        #[allow(clippy::cast_possible_wrap)]
        let shape_block_x = shape_x as i16;
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_HEIGHT)
        #[allow(clippy::cast_possible_wrap)]
        let shape_block_y = shape_y as i16;
        let shape_width_i16 = SHAPE_WIDTH as i16;

        for coord in self.curr_shape.coords() {
            let (coord_x, coord_y) = coord;
            // u16 -> i16: SHAPE_OFFSET_X = 2, coord_x <= 3, shape_block_x в пределах поля
            // coord_x и coord_y — маленькие константы (0..=3), преобразование безопасно
            #[allow(clippy::cast_possible_wrap)]
            let x = (coord_x as i16) + shape_block_x * shape_width_i16
                + (SHAPE_OFFSET_X as i16);
            #[allow(clippy::cast_possible_wrap)]
            let y = (coord_y as i16) + shape_block_y + (SHAPE_DRAW_OFFSET as i16);

            if x >= 0 {
                // i16 -> u16: проверка x >= 0 гарантирует неотрицательное значение
                let ux = u16::try_from(x).unwrap_or(u16::MAX);
                let uy = u16::try_from(y).unwrap_or(u16::MAX);
                canvas.draw_strs(
                    &[SHAPE_STR],
                    (ux, uy),
                    // u8 -> usize: fg < 7 (количество фигур)
                    SHAPE_COLORS[usize::from(self.curr_shape.fg())],
                    &Reset,
                );
            }
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
        use crate::constants::{GRID_HEIGHT, GRID_WIDTH, SHAPE_WIDTH};
        use crate::tetromino::SHAPE_COLORS;
        use termion::color::Reset;

        // I128: Намеренно создаём полную копию Tetromino через *self.curr_shape.
        // Tetromino реализует Copy и имеет небольшой размер (содержит координаты,
        // тип фигуры и позицию), поэтому копирование дешевле, чем borrowing + mutация.
        let mut ghost_shape = *self.curr_shape;

        let grid_height_i16 = GRID_HEIGHT as i16;
        let grid_width_i16 = GRID_WIDTH as i16;

        // Вычисляем расстояние до препятствия напрямую — оптимизация: вместо линейного
        // поиска для каждого блока, сначала находим самый нижний блок фигуры,
        // затем проверяем препятствия (Исправление аудита #8: производительность).
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_HEIGHT)
        #[allow(clippy::cast_possible_wrap)]
        let ghost_block_y = ghost_shape.pos().1 as i16;
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_WIDTH)
        #[allow(clippy::cast_possible_wrap)]
        let ghost_block_x = ghost_shape.pos().0 as i16;
        let mut max_drop_distance;

        // Находим самый нижний блок фигуры для ограничения расстояния до пола
        let mut max_shape_y = 0;
        for &(_coord_x, coord_y) in &ghost_shape.coords() {
            let block_y = coord_y + ghost_block_y;
            max_shape_y = max_shape_y.max(block_y);
        }
        max_drop_distance = grid_height_i16 - 1 - max_shape_y;

        for &(coord_x, coord_y) in &ghost_shape.coords() {
            let block_y = coord_y + ghost_block_y;
            let x = coord_x + ghost_block_x;

            let mut dist_to_block = grid_height_i16 - 1 - block_y;
            for y in (block_y + 1)..grid_height_i16 {
                // i16 -> usize: проверки x >= 0 и y >= 0 гарантируют безопасное преобразование
                let uy = usize::try_from(y).unwrap_or(0);
                let ux = usize::try_from(x).unwrap_or(usize::MAX);
                if x >= 0 && x < grid_width_i16 && self.blocks[uy][ux] != -1 {
                    dist_to_block = y - block_y - 1;
                    break;
                }
            }

            max_drop_distance = max_drop_distance.min(dist_to_block);
        }

        ghost_shape.pos_mut().1 += f32::from(max_drop_distance);

        // Отрисовка призрачной фигуры (полупрозрачная)
        let (shape_x, shape_y) = ghost_shape.pos();
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_WIDTH)
        #[allow(clippy::cast_possible_wrap)]
        let shape_block_x = shape_x as i16;
        // cast: f32 -> i16, потеря точности допустима: координаты фигуры в пределах поля (0..GRID_HEIGHT)
        #[allow(clippy::cast_possible_wrap)]
        let shape_block_y = shape_y as i16;
        let shape_width_i16 = SHAPE_WIDTH as i16;

        for coord in ghost_shape.coords() {
            let (coord_x, coord_y) = coord;
            // u16 -> i16: константы маленькие (SHAPE_OFFSET_X=2, SHAPE_DRAW_OFFSET=5)
            #[allow(clippy::cast_possible_wrap)]
            let x = (coord_x as i16) + shape_block_x * shape_width_i16
                + (SHAPE_OFFSET_X as i16);
            #[allow(clippy::cast_possible_wrap)]
            let y = (coord_y as i16) + shape_block_y + (SHAPE_DRAW_OFFSET as i16);

            // usize -> i16: GRID_WIDTH = 10, безопасно
            if x >= 0 && x < i16::try_from(GRID_WIDTH).unwrap_or(i16::MAX) {
                // i16 -> u16: проверка x >= 0 гарантирует неотрицательность
                let ux = u16::try_from(x).unwrap_or(u16::MAX);
                let uy = u16::try_from(y).unwrap_or(u16::MAX);
                canvas.draw_strs(
                    &["░░"],
                    (ux, uy),
                    // u8 -> usize: fg < 7
                    SHAPE_COLORS[usize::from(ghost_shape.fg())],
                    &Reset,
                );
            }
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
        use crate::constants::{PREVIEW_X, PREVIEW_Y};
        draw_shape_preview(
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
        use crate::constants::{HOLD_PREVIEW_X, HOLD_PREVIEW_Y};
        if let Some(held) = self.held_shape {
            let is_faded = false; // can_hold не доступен в GameView
            draw_shape_preview(
                canvas,
                held,
                HOLD_PREVIEW_X,
                HOLD_PREVIEW_Y,
                "Удерж:",
                is_faded,
            );
        }
    }
}

/// Отрисовать предпросмотр фигуры (свободная функция).
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
fn draw_shape_preview<R>(
    canvas: &mut R,
    shape: &crate::tetromino::Tetromino,
    pos_x: u16,
    pos_y: u16,
    title: &str,
    is_faded: bool,
) where
    R: crate::io_traits::Renderer,
{
    use crate::constants::{BORDER_COLOR, DISP_HEIGHT, DISP_WIDTH, DRAW_OFFSET_X};
    use crate::constants::{SHAPE_STR, SHAPE_WIDTH};
    use crate::tetromino::SHAPE_COLORS;
    use termion::color::Reset;

    canvas.draw_string(title, (pos_x, pos_y - 2), BORDER_COLOR, &Reset);

    let shape_width_i16 = SHAPE_WIDTH as i16;

    for coord in shape.coords() {
        let (coord_x, coord_y) = coord;
        let x = (pos_x as i16) + coord_x * shape_width_i16 + DRAW_OFFSET_X;
        let y = (pos_y as i16) + coord_y + 1;

        // Проверка всех границ — DISP_WIDTH/DISP_HEIGHT константы
        let disp_w = i16::try_from(DISP_WIDTH).unwrap_or(i16::MAX);
        let disp_h = i16::try_from(DISP_HEIGHT).unwrap_or(i16::MAX);
        if x >= 0 && y >= 0 && x < disp_w && y < disp_h {
            let display_char = if is_faded { "░░" } else { SHAPE_STR };
            // i16 -> u16: проверки >= 0 гарантируют безопасное преобразование
            let ux = u16::try_from(x).unwrap_or(u16::MAX);
            let uy = u16::try_from(y).unwrap_or(u16::MAX);
            canvas.draw_strs(
                &[display_char],
                (ux, uy),
                // u8 -> usize: fg < 7
                SHAPE_COLORS[usize::from(shape.fg())],
                &Reset,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::GameState;

    /// Базовый тест: проверяет что `GameView` создаётся из GameState без паники.
    #[test]
    fn test_game_view_from_game_state() {
        let state = GameState::new();
        // Создаём view — если метод работает без паники, тест пройден
        let _view = GameView::from_game_state(&state);
    }
}
