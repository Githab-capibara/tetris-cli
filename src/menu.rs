//! Модуль главного меню игры.
//!
//! Этот модуль содержит функции для отображения и управления главным меню.
//!
//! ## Исправление #10 (menu.rs разделение)
//! Модуль теперь разделён на подмодули:
//! - `constants` - константы меню
//! - `draw` - функции отрисовки
//! - `input` - обработка ввода
//!
//! Для обратной совместимости основные функции переэкспортированы из подмодулей.

// Модули
pub mod constants;
pub mod draw;
pub mod input;

// Ре-экспорт для обратной совместимости
pub use constants::{MENU, MENU_COLOR};
pub use draw::draw_menu;
pub use input::get_player_name;

// ============================================================================
// ФУНКЦИИ ОБРАБОТКИ ИГРЫ (переэкспортированы из draw/input)
// ============================================================================

use crate::game::GameState;
use crate::highscore::Leaderboard;
use crate::io::{Canvas, KeyReader};

/// Показать таблицу лидеров.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `leaderboard` - таблица лидеров для отображения
#[track_caller]
pub fn show_leaderboard(cnv: &mut Canvas, inp: &mut KeyReader, leaderboard: &Leaderboard) {
    draw::draw_leaderboard(cnv, leaderboard);
    input::wait_for_key(inp);
}

/// Показать статистику после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `state` - состояние игры для отображения статистики
#[track_caller]
pub fn show_game_stats(cnv: &mut Canvas, inp: &mut KeyReader, state: &GameState) {
    let stats = state.get_stats();
    let mode_trait = state.get_mode_trait();
    let mode_str = mode_trait.name();

    draw::draw_game_stats(
        cnv,
        mode_str,
        state.get_score(),
        state.get_level(),
        state.get_lines_cleared(),
        stats.total_pieces(),
        stats.max_combo,
        stats.get_elapsed_time(),
    );

    input::wait_for_key(inp);
}

/// Запустить игровой режим и обработать результат.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `high_score_display` - строка рекорда для отображения
/// * `state` - состояние игры
/// * `save_to_leaderboard` - сохранять ли в таблицу лидеров
/// * `leaderboard` - таблица лидеров (изменяемая)
///
/// # Возвращает
/// Финальный счёт игрока
#[track_caller]
pub fn run_game_mode(
    cnv: &mut Canvas,
    inp: &mut KeyReader,
    high_score_display: &str,
    mut state: GameState,
    save_to_leaderboard: bool,
    leaderboard: &mut Leaderboard,
) -> u128 {
    let new_score = state.play(cnv, inp, high_score_display);

    // Отображение статистики после завершения игры
    show_game_stats(cnv, inp, &state);

    // Сохранение рекорда если игрок набрал очки
    if new_score > 0 && save_to_leaderboard {
        let name = get_player_name(cnv, inp);

        if !name.is_empty() {
            if leaderboard.add_score(&name, new_score) {
                leaderboard.save();
            } else {
                eprintln!("Предупреждение: рекорд не сохранён в таблицу лидеров (rate limit).");
                draw::draw_record_message(cnv, "Рекорд не сохранён (rate limit)", 23);
            }
        }
    }

    new_score
}
