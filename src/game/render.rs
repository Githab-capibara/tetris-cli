//! Модуль отрисовки и анимаций.
//!
//! # Ответственность
//! - Отрисовка игрового поля и границ
//! - Отрисовка фигур (текущей, призрачной, следующей, удержанной)
//! - Отрисовка интерфейса (счёт, уровень, линии, комбо, таймер)
//! - Анимации (мигание Hard Drop, очистка линий)
//!
//! # Зависимости
//! - [`state.rs`](super::state): константы отрисовки
//! - [`view.rs`](super::view): `GameView` для отрисовки
//! - [`tetromino.rs`](crate::tetromino): `Tetromino`, `SHAPE_COLORS`
//! - [`io.rs`](crate::io): `Canvas`, `GRID_WIDTH`, `GRID_HEIGHT`
//!
//! ## Основные функции:
//! - [`draw()`] - основная функция отрисовки кадра
//! - [`check_rows()`] - проверка и анимация заполненных линий
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

use super::constants::{
    ANIMATION_FRAME_SKIP, BORDER, BORDER_COLOR, COMBO_X, COMBO_Y, DRAW_OFFSET_X,
    HARD_DROP_ANIM_INTERVAL_MS, HIGH_SCORE_X, HIGH_SCORE_Y, HOLD_PREVIEW_X, HOLD_PREVIEW_Y,
    LEVEL_X, LEVEL_Y, LINES_X, LINES_Y, PREVIEW_X, PREVIEW_Y, PROGRESS_Y, SCORE_X, SCORE_Y,
    SHAPE_DRAW_OFFSET, SHAPE_OFFSET_X, SHAPE_OFFSET_Y, SPRINT_LINES, TIMER_Y,
};
use super::state::GameState;
use super::view::GameView;
use crate::io::{Canvas, DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};
use crate::tetromino::{Tetromino, SHAPE_COLORS};
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

    // Отрисовка зафиксированных фигур
    let millis = (view.elapsed_time * 1000.0) as u16;
    let show_animation = (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP);

    for y in 0..GRID_HEIGHT {
        let is_animating = (view.animating_rows & (1 << y)) != 0;

        for x in 0..GRID_WIDTH {
            if view.blocks[y][x] != -1 {
                if is_animating && !show_animation {
                    continue;
                }

                cnv.draw_strs(
                    &[SHAPE_STR],
                    (
                        (x * SHAPE_WIDTH + 2) as u16,
                        (y + SHAPE_DRAW_OFFSET as usize) as u16,
                    ),
                    SHAPE_COLORS[view.blocks[y][x] as usize],
                    &Reset,
                );
            }
        }
    }

    // Отрисовка призрачной фигуры
    draw_ghost_shape(view, cnv);

    // Отрисовка текущей падающей фигуры с анимацией Hard Drop
    let shape_display_char = if view.is_hard_dropping {
        if (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP) {
            SHAPE_STR
        } else {
            "░░"
        }
    } else {
        SHAPE_STR
    };

    let (shape_x, shape_y) = view.curr_shape.pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;
    let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

    for coord in view.curr_shape.coords {
        let (coord_x, coord_y) = coord;
        let x = (coord_x + shape_block_x) * shape_width_i16 + SHAPE_OFFSET_X;
        let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y;

        if x >= 0 {
            cnv.draw_strs(
                &[shape_display_char],
                (x as u16, y as u16),
                SHAPE_COLORS[view.curr_shape.fg],
                &Reset,
            );
        }
    }

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
/// # Исправление #7: Оптимизация кэширования строк
/// - Используется `truncate(0)` вместо `clear()` для предотвращения deallocation
/// - Инициализация через `String::with_capacity(16)` для уменьшения аллокаций
fn update_cached_strings(state: &mut GameState) {
    use std::fmt::Write;

    let score = state.score();
    let level = state.level();
    let lines_cleared = state.lines_cleared();
    let render_cache = state.get_render_cache_mut();

    if score != render_cache.last_cached_score {
        render_cache.cached_score_str.truncate(0);
        if let Err(e) = write!(render_cache.cached_score_str, "{:10}", score) {
            eprintln!("Ошибка записи кэша счёта: {e}");
        }
        render_cache.last_cached_score = score;
    }

    if level != render_cache.last_cached_level {
        render_cache.cached_level_str.truncate(0);
        if let Err(e) = write!(render_cache.cached_level_str, "{:10}", level) {
            eprintln!("Ошибка записи кэша уровня: {e}");
        }
        render_cache.last_cached_level = level;
    }

    if lines_cleared != render_cache.last_cached_lines {
        render_cache.cached_lines_str.truncate(0);
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
pub fn update_cached_strings_extended(state: &mut GameState, high_score_display: &str) {
    update_cached_strings(state);

    // Кэширование строки рекорда
    {
        let render_cache = state.get_render_cache_mut();
        if render_cache.cached_high_score_str.len() != high_score_display.len()
            || render_cache.cached_high_score_str != high_score_display
        {
            render_cache.cached_high_score_str = high_score_display.to_string();
        }
    }

    // Кэширование строки комбо
    let combo_counter = state.get_stats().combo_counter;
    {
        let render_cache = state.get_render_cache_mut();
        if render_cache.last_cached_combo != combo_counter {
            if combo_counter > 1 {
                render_cache.cached_combo_str = format!("Комбо: x{combo_counter}");
            } else {
                render_cache.cached_combo_str.clear();
            }
            render_cache.last_cached_combo = combo_counter;
        }
    }

    // Кэширование строки таймера для режима спринт
    if state.get_mode_trait().get_target_lines() == Some(40) {
        let elapsed = state.get_stats().get_elapsed_time();
        let timer_str = format!("Время: {elapsed:.2}с");
        let render_cache = state.get_render_cache_mut();
        if render_cache.cached_timer_str != timer_str {
            render_cache.cached_timer_str = timer_str;
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
            SHAPE_COLORS[ghost_shape.fg],
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
                SHAPE_COLORS[shape.fg],
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
// ПРИВАТНЫЕ ФУНКЦИИ ДЛЯ РАЗДЕЛЕНИЯ ОТВЕТСТВЕННОСТИ (Задача 14)
// ============================================================================

/// Поиск заполненных линий.
///
/// # Аргументы
/// * `blocks` - игровое поле (только чтение)
///
/// # Возвращает
/// Вектор с индексами заполненных линий
///
/// # Пример
/// ```ignore
/// let filled_rows = find_filled_lines(&state.blocks);
/// assert!(filled_rows.is_empty()); // На пустом поле
/// ```
pub fn find_filled_lines(blocks: &[[i8; GRID_WIDTH]; GRID_HEIGHT]) -> Vec<usize> {
    let mut filled = Vec::new();

    for (y, row) in blocks.iter().enumerate() {
        // Проверка: линия заполнена если все ячейки не пустые (!= -1)
        let row_full = row.iter().all(|&cell| cell != -1);
        if row_full {
            filled.push(y);
        }
    }

    filled
}

/// Анимация удаления линии (мигание, инверсия цветов).
///
/// # Аргументы
/// * `canvas` - канвас для отрисовки (изменяемый)
/// * `row` - индекс удаляемой линии
///
/// # Примечания
/// В текущей реализации анимация упрощена до установки флага анимации.
/// Полная анимация может включать:
/// - Инверсию цветов линии
/// - Мигание (несколько кадров)
/// - Звуковой сигнал
fn animate_line_clear(_canvas: &mut Canvas, _row: usize) {
    // Примечание: полная анимация требует доступа к canvas и отрисовки нескольких кадров
    // В текущей реализации анимация выполняется через animate_clear() в check_rows()
    // Эта функция预留ена для будущей реализации полной анимации
}

/// Удаление линий и сдвиг поля.
///
/// # Аргументы
/// * `blocks` - игровое поле (изменяемое)
/// * `rows` - срез с индексами удаляемых линий
///
/// # Пример
/// ```ignore
/// let filled_rows = find_filled_lines(&blocks);
/// remove_lines(&mut blocks, &filled_rows);
/// ```
fn remove_lines(blocks: &mut [[i8; GRID_WIDTH]; GRID_HEIGHT], rows: &[usize]) {
    // Преобразуем список строк в битовую маску для совместимости с remove_rows()
    let mut rows_mask: u32 = 0;
    for &row in rows {
        rows_mask |= 1u32 << row;
    }

    // Используем существующую функцию удаления
    super::scoring::remove_rows(blocks, rows_mask);
}

/// Обновление счёта за удалённые линии.
///
/// # Аргументы
/// * `score` - счёт (изменяемый)
/// * `level` - текущий уровень
/// * `rows_cleared` - количество удалённых линий
/// * `combo_counter` - счётчик комбо (изменяемый)
///
/// # Примечания
/// Формула расчёта очков:
/// - Базовые очки за линии из LINE_SCORES[rows_cleared - 1]
/// - Бонус за комбо: COMBO_BONUS × (combo_counter - 1)
/// - Бонус за уровень: LEVEL_BONUS_MULT × (level - 1)
fn update_score_for_lines(
    score: &mut u128,
    level: u32,
    rows_cleared: usize,
    combo_counter: &mut u32,
) {
    use super::constants::{COMBO_BONUS, LEVEL_BONUS_MULT, LINE_SCORES, MAX_LINES_PER_CLEAR};

    if rows_cleared > 0 {
        // Ограничение количества линий максимум 4
        let capped_rows = rows_cleared.min(MAX_LINES_PER_CLEAR as usize);

        // Начисление очков за линии
        let line_score = LINE_SCORES[capped_rows - 1];
        *score = score.saturating_add(line_score);

        // Обновление комбо
        *combo_counter = combo_counter.saturating_add(1);

        // Бонус за комбо (если комбо > 1)
        if *combo_counter > 1 {
            let combo_bonus = COMBO_BONUS.saturating_mul(u128::from(*combo_counter - 1));
            *score = score.saturating_add(combo_bonus);
        }

        // Бонус за уровень (каждые 10 линий)
        let level_bonus = LEVEL_BONUS_MULT.saturating_mul(u128::from(level - 1));
        *score = score.saturating_add(level_bonus);
    } else {
        // Сброс комбо если линии не удалены
        *combo_counter = 0;
    }
}

// ============================================================================
// ПУБЛИЧНЫЕ ФУНКЦИИ (используют приватные функции выше)
// ============================================================================

/// Анимировать очистку заполненных линий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `rows_mask` - битовая маска заполненных линий
/// * `remove_count` - количество заполненных линий
///
/// ## Примечания
/// Эта функция модифицирует состояние игры, поэтому не может использовать `GameView`.
pub fn animate_clear(state: &mut GameState, rows_mask: u32, remove_count: u32) {
    if remove_count > 0 {
        state.set_animating_rows_mask(rows_mask);
        // Исправление #1.7: print!() не требует обработки ошибки
        // Ошибка вывода bell-символа не критична для работы игры
        print!("{BELL}", BELL = super::constants::BELL);
        state.get_stats_mut().update_max_combo(remove_count);
    }
}

/// Проверить заполненные линии и удалить их.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Возвращает
/// Количество удалённых линий
///
/// ## Примечания
/// Эта функция модифицирует состояние игры, поэтому не может использовать `GameView`.
///
/// ## Архитектурные заметки (Задача 14)
/// Функция разделена на приватные методы для улучшения тестируемости:
/// - `find_filled_lines()` - поиск заполненных линий
/// - `animate_line_clear()` - анимация удаления
/// - `remove_lines()` - удаление линий и сдвиг
/// - `update_score_for_lines()` - начисление очков
pub fn check_rows(state: &mut GameState) -> u32 {
    // Поиск заполненных линий
    let filled_rows = find_filled_lines(state.get_blocks());
    let remove_count = filled_rows.len() as u32;

    // Преобразуем в битовую маску для анимации
    let mut rows_mask: u32 = 0;
    for &row in &filled_rows {
        rows_mask |= 1u32 << row;
    }

    // Анимация и звук для каждой линии
    for &row in &filled_rows {
        animate_line_clear(
            &mut Canvas::new().unwrap_or_else(|_| Canvas::default()),
            row,
        );
    }

    // Анимация через существующий механизм
    animate_clear(state, rows_mask, remove_count);

    // Удаление линий и сдвиг поля
    remove_lines(state.get_blocks_mut(), &filled_rows);

    // Обновление счёта, уровня и комбо
    let mut score = state.score();
    let level = state.level();
    let mut combo_counter = state.get_stats_mut().combo_counter;

    update_score_for_lines(&mut score, level, filled_rows.len(), &mut combo_counter);

    state.set_score(score);
    state.get_stats_mut().combo_counter = combo_counter;

    // Обновление количества очищенных линий
    let lines_cleared = state.lines_cleared().saturating_add(remove_count);
    state.set_lines_cleared(lines_cleared);

    // Увеличение скорости игры
    use super::constants::SPD_INC;
    let fall_speed = state.fall_speed();
    state.set_fall_speed(fall_speed + SPD_INC * remove_count as f32);

    remove_count
}
