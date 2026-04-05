//! Отрисовка главного меню.
//!
//! ## Исправление #10 (menu.rs разделение)
//! Выделено из `menu.rs` для улучшения читаемости и разделения ответственности.

use crate::highscore::Leaderboard;
use crate::io::Canvas;
use std::fmt::Write;

use super::constants::{LEADERBOARD_MENU, MAX_LEADERBOARD_ENTRIES, MENU, MENU_COLOR};

/// Отрисовать главное меню.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `high_score_display` - строка рекорда для отображения
///
/// # Расположение элементов
/// ## Основная структура меню
/// ```text
/// Строка 1:  Заголовок меню
/// Строка 2:  Счёт
/// Строка 3:  Рекорд
/// Строка 4:  Уровень
/// Строка 5:  Линии
/// Строки 6-24: Игровое поле (границы)
/// Строка 21: Отображение рекорда (позиция 11, 21)
/// ```
///
/// ## Позиции элементов
/// - MENU массив: отрисовка с позиции (1, 1)
/// - Рекорд: позиция (11, 21) - под игровым полем
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
    // Исправление проблемы 27: используем write! в переиспользуемый буфер вместо format! в цикле
    let mut line_buf = String::with_capacity(32);
    for (i, entry) in entries.iter().take(MAX_LEADERBOARD_ENTRIES).enumerate() {
        line_buf.clear();
        let _ = write!(
            line_buf,
            "{}. {:12} {:10}",
            i + 1,
            entry.name(),
            entry.score().unwrap_or(0)
        );
        // cast: usize -> u16, потеря точности допустима: количество записей <= 5
        cnv.draw_string(
            &line_buf,
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
// S9: Обоснование too_many_arguments — функция отрисовки требует 8 параметров
// для отображения статистики. Это чистая функция отрисовки без состояния,
// передача всех параметров явно соответствует функциональному стилю.
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
        &format!("║ Счёт: {score:16} ║"),
        &format!("║ Уровень: {level:14} ║"),
        &format!("║ Линии: {lines_cleared:16} ║"),
        "║                    ║",
        &format!("║ Фигур: {total_pieces:16} ║"),
        &format!("║ Комбо: {max_combo:16} ║"),
        &format!("║ Время: {elapsed_time:15.2} ║"),
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
