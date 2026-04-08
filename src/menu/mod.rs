//! Модуль главного меню игры.
//!
//! Этот модуль разделён на подмодули:
//! - `constants` - константы меню
//! - `draw` - функции отрисовки
//! - `input` - обработка ввода
//!
//! ## Архитектурные заметки
//! ### PROB-125: Связь menu → game
//! Модуль menu напрямую использует `GameState` и вызывает игровой цикл через
//! `run_game_mode()`. Это тесная связь — menu знает о внутренней структуре game.
//! Рефакторинг через абстракции стал бы breaking change, поэтому оставлено как есть.
//!
//! ### PROB-128: Дублирование констант
//! `menu/constants.rs` переэкспортирует `DISP_HEIGHT` и `FRAME_DELAY_MS` из
//! `crate::constants`. Это дублирование re-export — не удаляется для обратной
//! совместимости. Используйте `crate::constants::*` напрямую в новом коде.
//!
//! ## Исправление #10 (menu.rs разделение)
//! Модуль теперь разделён на подмодули для улучшения читаемости и разделения ответственности.

// Модули
pub mod constants;
pub mod draw;
pub mod input;

// Ре-экспорт для обратной совместимости
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
pub fn show_leaderboard(cnv: &mut Canvas, inp: &mut KeyReader, leaderboard: &Leaderboard) {
    draw::draw_leaderboard(cnv, leaderboard);
    input::wait_for_key(inp);
}

/// Показать статистику после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `game_state` - состояние игры для отображения статистики
pub fn show_game_stats(cnv: &mut Canvas, inp: &mut KeyReader, game_state: &GameState) {
    let stats = game_state.stats();
    let mode_trait = game_state.get_mode_trait();
    let mode_str = mode_trait.name();

    draw::draw_game_stats(
        cnv,
        mode_str,
        game_state.score(),
        game_state.level(),
        game_state.lines_cleared(),
        stats.total_pieces(),
        stats.max_combo(),
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
// Архитектурная заметка (ID 74): 6 параметров — это координационная функция,
/// которая связывает ввод/вывод, игровое состояние и таблицу лидеров.
/// Разбиение на меньшие функции потребовало бы создания контекстного объекта,
/// что увеличило бы сложность. Параметры независимы и имеют разные типы.
#[allow(clippy::too_many_arguments)]
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
                crate::log_warn!("Рекорд не сохранён в таблицу лидеров (rate limit).");
                draw::draw_record_message(cnv, "Рекорд не сохранён (rate limit)", 23);
            }
        }
    }

    new_score
}
