//! Модуль главного меню игры.
//!
//! Этот модуль содержит функции для отображения и управления главным меню:
//! - Отрисовка статического меню
//! - Обработка ввода пользователя
//! - Запрос имени игрока
//! - Отображение таблицы лидеров
//! - Показ статистики после игры
//!
//! ## Архитектурные заметки
//! Выделен из main.rs для улучшения читаемости и уменьшения размера файла.

use crate::highscore::Leaderboard;
use crate::io::{Canvas, KeyReader, DISP_HEIGHT, KEY_BACKSPACE};
use crate::{game::GameState, FRAME_DELAY_MS};
use std::{thread::sleep, time::Duration};
use termion::color::{Color, Reset, White};

// ============================================================================
// КОНСТАНТЫ МЕНЮ
// ============================================================================

/// Главное меню игры с управлением и информацией.
///
/// Содержит ровно 25 строк (равно `DISP_HEIGHT`), что обеспечивает
/// идеальное заполнение минимального размера терминала.
pub const MENU: [&str; DISP_HEIGHT as usize] = [
    "                      ", // Строка 0: пустая
    "╔════════════════════╗", // Строка 1: верхняя граница
    "║                    ║", // Строка 2
    "║   Т Е Т Р И С  CLI ║", // Строка 3: заголовок
    "║   Автор: Dylan T   ║", // Строка 4: автор
    "║    около 2022 г.   ║", // Строка 5: год
    "║                    ║", // Строка 6
    "║                    ║", // Строка 7
    "║     Управление:    ║", // Строка 8: заголовок управления
    "║ - a/d - влево/впр. ║", // Строка 9: перемещение
    "║  - q/e - поворот   ║", // Строка 10: вращение
    "║  - w - Hard Drop   ║", // Строка 11: жёсткое падение
    "║  - s - Soft Drop   ║", // Строка 12: мягкое падение
    "║  - c - удержать    ║", // Строка 13: удержание
    "║    - p - пауза     ║", // Строка 14: пауза
    "║ - back - выход     ║", // Строка 15: выход
    "║                    ║", // Строка 16
    "║ Enter - классика   ║", // Строка 17: классический режим
    "║  'r' - спринт 40   ║", // Строка 18: режим спринт
    "║  'm' - марафон 150 ║", // Строка 19: режим марафон
    "║  'l' - таблица л.  ║", // Строка 20: таблица лидеров
    "║     Рекорд:        ║", // Строка 21: заголовок рекорда
    "║                    ║", // Строка 22: место для рекорда
    "║                    ║", // Строка 23
    "╚════════════════════╝", // Строка 24: нижняя граница
];

/// Цвет отрисовки элементов меню.
pub const MENU_COLOR: &dyn Color = &White;

/// Меню таблицы лидеров.
pub const LEADERBOARD_MENU: [&str; 8] = [
    "╔════════════════════╗", // Строка 0: верхняя граница
    "║   ТАБЛИЦА ЛИДЕРОВ  ║", // Строка 1: заголовок
    "║                    ║", // Строка 2: отступ
    "║ 1.                 ║", // Строка 3: позиция 1
    "║ 2.                 ║", // Строка 4: позиция 2
    "║ 3.                 ║", // Строка 5: позиция 3
    "║ 4.                 ║", // Строка 6: позиция 4
    "║ 5.                 ║", // Строка 7: позиция 5
];

// ============================================================================
// ОТРИСОВКА МЕНЮ
// ============================================================================

/// Отрисовать главное меню.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `high_score_display` - строка рекорда для отображения
#[track_caller]
pub fn draw_menu(cnv: &mut Canvas, high_score_display: &str) {
    cnv.draw_strs(&MENU, (1, 1), MENU_COLOR, &Reset);
    cnv.draw_string(high_score_display, (11, 21), MENU_COLOR, &Reset);
    cnv.flush();
}

/// Отрисовать таблицу лидеров.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `leaderboard` - таблица лидеров для отображения
#[track_caller]
pub fn draw_leaderboard(cnv: &mut Canvas, leaderboard: &Leaderboard) {
    cnv.draw_strs(&LEADERBOARD_MENU, (1, 1), MENU_COLOR, &Reset);

    let entries = leaderboard.get_entries();
    for (i, entry) in entries.iter().take(5).enumerate() {
        let line = format!("{}. {:12} {:10}", i + 1, entry.name(), entry.score());
        cnv.draw_string(&line, (3, (3 + i) as u16), MENU_COLOR, &Reset);
    }

    cnv.flush();
}

// ============================================================================
// ВВОД ИМЕНИ ИГРОКА
// ============================================================================

/// Проверка допустимости символа имени.
///
/// Разрешены только безопасные символы:
/// - Алфавитно-цифровые (a-z, A-Z, 0-9)
/// - Подчёркивание, дефис, пробел
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим, `false` в противном случае
fn is_valid_name_char(c: char) -> bool {
    !c.is_control()
        && !c.is_whitespace()
        && c != '/'
        && c != '\\'
        && (c.is_alphanumeric() || c == '_' || c == '-' || c == ' ')
}

/// Запрос имени игрока после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
///
/// # Возвращает
/// Введённое имя игрока (может быть пустым при отмене)
#[track_caller]
pub fn get_player_name(cnv: &mut Canvas, inp: &mut KeyReader) -> String {
    const MAX_NAME_LEN: usize = 10;
    let mut name = String::new();

    // Отрисовка приглашения
    cnv.draw_string("Введите имя: ", (2, 10), MENU_COLOR, &Reset);
    cnv.draw_string(&name, (16, 10), MENU_COLOR, &Reset);
    cnv.flush();

    // Цикл ввода
    loop {
        let key = inp.get_key();

        match key {
            Some(b'\n' | b'\r') => break,
            Some(KEY_BACKSPACE) => {
                if !name.is_empty() {
                    name.pop();
                    cnv.draw_string(
                        &format!("{name:<MAX_NAME_LEN$}"),
                        (16, 10),
                        MENU_COLOR,
                        &Reset,
                    );
                    cnv.flush();
                }
            }
            Some(key) if name.len() < MAX_NAME_LEN => {
                let c = key as char;
                if is_valid_name_char(c) {
                    name.push(c);
                    cnv.draw_string(&name, (16, 10), MENU_COLOR, &Reset);
                    cnv.flush();
                }
            }
            Some(_) | None => {}
        }

        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }

    name
}

// ============================================================================
// ОТОБРАЖЕНИЕ СТАТИСТИКИ И ТАБЛИЦЫ ЛИДЕРОВ
// ============================================================================

/// Показать таблицу лидеров.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `leaderboard` - таблица лидеров для отображения
#[track_caller]
pub fn show_leaderboard(cnv: &mut Canvas, inp: &mut KeyReader, leaderboard: &Leaderboard) {
    draw_leaderboard(cnv, leaderboard);

    // Ожидание нажатия любой клавиши для возврата в меню
    loop {
        let key = inp.get_key();
        if key.is_some() {
            break;
        }
        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}

/// Показать статистику после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `state` - состояние игры для отображения статистики
#[track_caller]
pub fn show_game_stats(cnv: &mut Canvas, inp: &mut KeyReader, state: &GameState) {
    use crate::game::GameMode;

    let stats = state.get_stats();
    let mode_str = match state.get_mode() {
        GameMode::Classic => "Классика",
        GameMode::Sprint => "Спринт",
        GameMode::Marathon => "Марафон",
    };

    let stats_lines = [
        "╔════════════════════╗",
        "║   СТАТИСТИКА ИГРЫ  ║",
        "║                    ║",
        &format!("║ Режим: {mode_str:<12} ║"),
        &format!("║ Счёт: {:16} ║", state.get_score()),
        &format!("║ Уровень: {:14} ║", state.get_level()),
        &format!("║ Линии: {:16} ║", state.get_lines_cleared()),
        "║                    ║",
        &format!("║ Фигур: {:16} ║", stats.total_pieces()),
        &format!("║ Комбо: {:16} ║", stats.max_combo),
        &format!("║ Время: {:15.2} ║", stats.get_elapsed_time()),
        "║                    ║",
        "║  Любая клавиша...  ║",
        "║                    ║",
        "╚════════════════════╝",
    ];

    cnv.draw_strs(&stats_lines, (1, 1), MENU_COLOR, &Reset);
    cnv.flush();

    // Ожидание нажатия любой клавиши
    loop {
        let key = inp.get_key();
        if key.is_some() {
            break;
        }
        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}

// ============================================================================
// ЗАПУСК ИГРОВОГО РЕЖИМА
// ============================================================================

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
                cnv.draw_string(
                    "Рекорд не сохранён (rate limit)",
                    (1, 23),
                    MENU_COLOR,
                    &Reset,
                );
                cnv.flush();
            }
        }
    }

    new_score
}
