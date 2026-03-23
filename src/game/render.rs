//! Отрисовка и анимации.
//!
//! Этот модуль содержит функции для отрисовки:
//! - Игрового поля и границ
//! - Фигур (текущей, призрачной, следующей, удержанной)
//! - Интерфейса (счёт, уровень, линии, комбо, таймер)
//! - Анимаций (мигание Hard Drop, очистка линий)

use super::state::{
    GameMode, GameState, ANIMATION_FRAME_SKIP, BORDER, BORDER_COLOR, DRAW_OFFSET_X,
    HARD_DROP_ANIM_INTERVAL_MS, HIGH_SCORE_X, HIGH_SCORE_Y, HOLD_PREVIEW_X, HOLD_PREVIEW_Y,
    LEVEL_X, LEVEL_Y, LINES_X, LINES_Y, PREVIEW_X, PREVIEW_Y, SCORE_X, SCORE_Y, SHAPE_DRAW_OFFSET,
    SHAPE_OFFSET_X, SHAPE_OFFSET_Y, SPRINT_LINES,
};
use crate::io::{Canvas, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};
use crate::tetromino::{Tetromino, SHAPE_COLORS};
use termion::color::Reset;

/// Отрисовать текущее состояние игры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки
/// * `high_score_display` - строка для отображения рекорда
pub fn draw(state: &mut GameState, cnv: &mut Canvas, high_score_display: &str) {
    cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

    // Обновление кэшированных строк
    update_cached_strings_extended(state, high_score_display);

    // Отрисовка счёта и рекорда
    cnv.draw_string(
        &state.cached_score_str,
        (SCORE_X, SCORE_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(
        &state.cached_high_score_str,
        (HIGH_SCORE_X, HIGH_SCORE_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(
        &state.cached_level_str,
        (LEVEL_X, LEVEL_Y),
        BORDER_COLOR,
        &Reset,
    );
    cnv.draw_string(
        &state.cached_lines_str,
        (LINES_X, LINES_Y),
        BORDER_COLOR,
        &Reset,
    );

    // Отрисовка счётчика комбо
    if !state.cached_combo_str.is_empty() {
        cnv.draw_string(&state.cached_combo_str, (24, 6), BORDER_COLOR, &Reset);
    }

    // Отрисовка зафиксированных фигур
    let animation_time = state.stats.get_elapsed_time();
    let millis = (animation_time * 1000.0) as u16;
    let show_animation = (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP);

    for y in 0..GRID_HEIGHT {
        let is_animating = (state.animating_rows_mask & (1 << y)) != 0;

        for x in 0..GRID_WIDTH {
            if state.blocks[y][x] != -1 {
                if is_animating && !show_animation {
                    continue;
                }

                cnv.draw_strs(
                    &[SHAPE_STR],
                    (
                        (x * SHAPE_WIDTH + 2) as u16,
                        (y + SHAPE_DRAW_OFFSET as usize) as u16,
                    ),
                    SHAPE_COLORS[state.blocks[y][x] as usize],
                    &Reset,
                );
            }
        }
    }

    // Отрисовка призрачной фигуры
    draw_ghost_shape(state, cnv);

    // Отрисовка текущей падающей фигуры с анимацией Hard Drop
    let shape_display_char = if state.is_hard_dropping {
        if (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP) {
            SHAPE_STR
        } else {
            "░░"
        }
    } else {
        SHAPE_STR
    };

    let (shape_x, shape_y) = state.curr_shape.pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;
    let shape_width_i16 = i16::try_from(SHAPE_WIDTH).unwrap_or(i16::MAX);

    for coord in state.curr_shape.coords {
        let (coord_x, coord_y) = coord;
        let x = (coord_x + shape_block_x) * shape_width_i16 + SHAPE_OFFSET_X;
        let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y;

        if x >= 0 {
            cnv.draw_strs(
                &[shape_display_char],
                (x as u16, y as u16),
                SHAPE_COLORS[state.curr_shape.fg],
                &Reset,
            );
        }
    }

    // Отрисовка следующей фигуры
    draw_next_shape(state, cnv);

    // Отрисовка удержанной фигуры
    draw_held_shape(state, cnv);

    // Отрисовка таймера для режима спринт
    if state.mode == GameMode::Sprint {
        draw_sprint_timer(state, cnv);
    }

    cnv.flush();
}

/// Обновить кэшированные строки для отрисовки.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
fn update_cached_strings(state: &mut GameState) {
    if state.score != state.last_cached_score {
        state.cached_score_str = format!("{:10}", state.score);
        state.last_cached_score = state.score;
    }

    if state.level != state.last_cached_level {
        state.cached_level_str = format!("{:10}", state.level);
        state.last_cached_level = state.level;
    }

    if state.lines_cleared != state.last_cached_lines {
        state.cached_lines_str = format!("{:10}", state.lines_cleared);
        state.last_cached_lines = state.lines_cleared;
    }
}

/// Обновить кэшированные строки (расширенная версия).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `high_score_display` - строка рекорда для кэширования
fn update_cached_strings_extended(state: &mut GameState, high_score_display: &str) {
    update_cached_strings(state);

    // Кэширование строки рекорда
    if state.cached_high_score_str.len() != high_score_display.len()
        || state.cached_high_score_str != high_score_display
    {
        state.cached_high_score_str = high_score_display.to_string();
    }

    // Кэширование строки комбо
    if state.last_cached_combo != state.stats.combo_counter {
        if state.stats.combo_counter > 1 {
            state.cached_combo_str = format!("Комбо: x{}", state.stats.combo_counter);
        } else {
            state.cached_combo_str.clear();
        }
        state.last_cached_combo = state.stats.combo_counter;
    }

    // Кэширование строки таймера для режима спринт
    if state.mode == GameMode::Sprint {
        let elapsed = state.stats.get_elapsed_time();
        let timer_str = format!("Время: {elapsed:.2}с");
        if state.cached_timer_str != timer_str {
            state.cached_timer_str = timer_str;
        }
    }
}

/// Отрисовать призрачную фигуру (точку приземления).
///
/// # Аргументы
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки
fn draw_ghost_shape(state: &GameState, cnv: &mut Canvas) {
    let mut ghost_shape = state.curr_shape;

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
            if x >= 0 && x < grid_width_i16 && state.blocks[y as usize][x as usize] != -1 {
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
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки
fn draw_next_shape(state: &GameState, cnv: &mut Canvas) {
    draw_shape_preview(cnv, &state.next_shape, PREVIEW_X, PREVIEW_Y, "След:", false);
}

/// Отрисовать удержанную фигуру (слева от поля).
///
/// # Аргументы
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки
fn draw_held_shape(state: &GameState, cnv: &mut Canvas) {
    if let Some(held) = state.held_shape {
        let is_faded = !state.can_hold;
        draw_shape_preview(
            cnv,
            &held,
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
    for coord in shape.coords {
        let (coord_x, coord_y) = coord;
        let x = pos_x.cast_signed() + coord_x * shape_width_i16 + DRAW_OFFSET_X;
        let y = pos_y.cast_signed() + coord_y + 1;

        if x >= 0 && y >= 0 {
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
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки
fn draw_sprint_timer(state: &GameState, cnv: &mut Canvas) {
    cnv.draw_string(&state.cached_timer_str, (24, 20), BORDER_COLOR, &Reset);

    let progress = format!("Цель: {}/{}", state.lines_cleared, SPRINT_LINES);
    cnv.draw_string(&progress, (24, 21), BORDER_COLOR, &Reset);
}

/// Анимировать очистку заполненных линий.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `rows_mask` - битовая маска заполненных линий
/// * `remove_count` - количество заполненных линий
pub fn animate_clear(state: &mut GameState, rows_mask: u32, remove_count: u32) {
    if remove_count > 0 {
        state.animating_rows_mask = rows_mask;
        print!("{BELL}", BELL = super::state::BELL);
        state.stats.update_max_combo(remove_count);
    }
}

/// Проверить заполненные линии и удалить их.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// # Возвращает
/// Количество удалённых линий
pub fn check_rows(state: &mut GameState) -> u32 {
    // Поиск заполненных линий
    let (rows_mask, remove_count) = super::scoring::find_full_rows(&state.blocks);

    // Анимация и звук
    animate_clear(state, rows_mask, remove_count);

    // Удаление линий
    super::scoring::remove_rows(&mut state.blocks, rows_mask);

    // Обновление счёта и уровня
    super::scoring::update_score_and_level(state, remove_count);

    remove_count
}
