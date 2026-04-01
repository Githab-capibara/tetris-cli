//! Отрисовка главного меню.
//!
//! Этот модуль содержит функции для отрисовки элементов меню:
//! - Главное меню
//! - Таблица лидеров
//! - Статистика игры
//!
//! ## Исправление #10 (menu.rs разделение)
//! Выделено из `menu.rs` для улучшения читаемости и разделения ответственности.

use crate::highscore::Leaderboard;
use crate::io::Canvas;

use super::constants::{LEADERBOARD_MENU, MAX_LEADERBOARD_ENTRIES, MENU, MENU_COLOR};

/// Отрисовать главное меню.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `high_score_display` - строка рекорда для отображения
///
/// # Пример
/// ```no_run
/// use tetris_cli::io::Canvas;
/// use tetris_cli::menu::draw::draw_menu;
///
/// let mut canvas = Canvas::new().unwrap();
/// draw_menu(&mut canvas, "1000");
/// ```
#[track_caller]
pub fn draw_menu(cnv: &mut Canvas, high_score_display: &str) {
    cnv.draw_strs(&MENU, (1, 1), MENU_COLOR, &termion::color::Reset);
    cnv.draw_string(
        high_score_display,
        (11, 21),
        MENU_COLOR,
        &termion::color::Reset,
    );
    cnv.flush();
}

/// Отрисовать таблицу лидеров.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `leaderboard` - таблица лидеров для отображения
///
/// # Пример
/// ```no_run
/// use tetris_cli::io::Canvas;
/// use tetris_cli::highscore::Leaderboard;
/// use tetris_cli::menu::draw::draw_leaderboard;
///
/// let mut canvas = Canvas::new().unwrap();
/// let leaderboard = Leaderboard::load();
/// draw_leaderboard(&mut canvas, &leaderboard);
/// ```
#[track_caller]
pub fn draw_leaderboard(cnv: &mut Canvas, leaderboard: &Leaderboard) {
    cnv.draw_strs(
        &LEADERBOARD_MENU,
        (1, 1),
        MENU_COLOR,
        &termion::color::Reset,
    );

    let entries = leaderboard.get_entries();
    for (i, entry) in entries.iter().take(MAX_LEADERBOARD_ENTRIES).enumerate() {
        let line = format!(
            "{}. {:12} {:10}",
            i + 1,
            entry.name(),
            entry.score().unwrap_or(0)
        );
        cnv.draw_string(
            &line,
            (3, (3 + i) as u16),
            MENU_COLOR,
            &termion::color::Reset,
        );
    }

    cnv.flush();
}

/// Отрисовать статистику игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `mode_str` - строка режима игры
/// * `score` - финальный счёт
/// * `level` - уровень
/// * `lines_cleared` - количество очищенных линий
/// * `total_pieces` - общее количество фигур
/// * `max_combo` - максимальное комбо
/// * `elapsed_time` - время игры в секундах
///
/// # Пример
/// ```no_run
/// use tetris_cli::io::Canvas;
/// use tetris_cli::menu::draw::draw_game_stats;
///
/// let mut canvas = Canvas::new().unwrap();
/// draw_game_stats(&mut canvas, "Классика", 1000, 5, 50, 100, 3, 120.5);
/// ```
#[track_caller]
#[allow(clippy::too_many_arguments)]
pub fn draw_game_stats(
    cnv: &mut Canvas,
    mode_str: &str,
    score: u128,
    level: u32,
    lines_cleared: u32,
    total_pieces: u32,
    max_combo: u32,
    elapsed_time: f64,
) {
    let stats_lines = [
        "╔════════════════════╗",
        "║   СТАТИСТИКА ИГРЫ  ║",
        "║                    ║",
        &format!("║ Режим: {mode_str:<12} ║"),
        &format!("║ Счёт: {:16} ║", score),
        &format!("║ Уровень: {:14} ║", level),
        &format!("║ Линии: {:16} ║", lines_cleared),
        "║                    ║",
        &format!("║ Фигур: {:16} ║", total_pieces),
        &format!("║ Комбо: {:16} ║", max_combo),
        &format!("║ Время: {:15.2} ║", elapsed_time),
        "║                    ║",
        "║  Любая клавиша...  ║",
        "║                    ║",
        "╚════════════════════╝",
    ];

    cnv.draw_strs(&stats_lines, (1, 1), MENU_COLOR, &termion::color::Reset);
    cnv.flush();
}

/// Отрисовать сообщение о рекорде.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `message` - сообщение для отображения
/// * `y` - позиция Y для отрисовки
#[track_caller]
pub fn draw_record_message(cnv: &mut Canvas, message: &str, y: u16) {
    cnv.draw_string(message, (1, y), MENU_COLOR, &termion::color::Reset);
    cnv.flush();
}
