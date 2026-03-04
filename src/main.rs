//! Точка входа в программу игры Тетрис.
//!
//! Автор: Dylan Turner
//!
//! Этот модуль содержит точку входа в приложение и реализует:
//! - Главное меню с отображением управления
//! - Загрузку и сохранение рекордов
//! - Проверку размера терминала
//! - Запуск игрового цикла

mod game;
mod highscore;
mod io;
mod tetromino;

use crate::game::{GameState, FPS};
use crate::highscore::{Leaderboard, SaveData};
use crate::io::{Canvas, KeyReader, DISP_HEIGHT, DISP_WIDTH};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use termion::{
    color::{Color, Reset, White},
    terminal_size,
};

/// Меню игры с управлением и информацией.
///
/// Содержит 25 строк (DISP_HEIGHT):
/// - Заголовок "ТЕТРИС CLI"
/// - Информация об авторе
/// - Управление
/// - Поле для отображения рекорда
const MENU: [&str; DISP_HEIGHT as usize] = [
    "                      ",
    "╔════════════════════╗",
    "║                    ║",
    "║   Т Е Т Р И С  CLI ║",
    "║   Автор: Dylan T   ║",
    "║    около 2022 г.   ║",
    "║                    ║",
    "║                    ║",
    "║     Управление:    ║",
    "║ - a/d - влево/впр. ║",
    "║  - q/e - поворот   ║",
    "║  - s - сброс вниз  ║",
    "║    - p - пауза     ║",
    "║ - back - выход     ║",
    "║                    ║",
    "║                    ║",
    "║  Enter для начала... ║",
    "║                    ║",
    "║                    ║",
    "║     Рекорд:        ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "╚════════════════════╝",
    "                      ",
];

/// Цвет меню.
const MENU_COLOR: &dyn Color = &White;

/// Меню таблицы лидеров.
const LEADERBOARD_MENU: [&str; 8] = [
    "╔════════════════════╗",
    "║   ТАБЛИЦА ЛИДЕРОВ  ║",
    "║                    ║",
    "║ 1.                 ║",
    "║ 2.                 ║",
    "║ 3.                 ║",
    "║ 4.                 ║",
    "║ 5.                 ║",
];

/// Точка входа в приложение.
///
/// Выполняет инициализацию терминала, загрузку рекордов
/// и запускает главный цикл меню.
fn main() {
    // Загрузка рекорда из файла конфигурации
    let save = SaveData::load_config();
    let mut high_score = save.assert_hs();

    // Загрузка таблицы лидеров
    let mut leaderboard = Leaderboard::load();
    leaderboard.validate();

    // Проверка достаточного размера терминала
    let (width, height) = terminal_size().expect("Не удалось получить размер терминала");
    if width < DISP_WIDTH || height < DISP_HEIGHT {
        println!(
            "Невозможно запустить игру! Окно терминала слишком маленькое. Минимальный размер: {}x{}",
            DISP_WIDTH, DISP_HEIGHT
        );
        return;
    }

    let mut cnv = Canvas::new();
    let mut inp = KeyReader::new();

    // Отображение меню и управления перед запуском игры
    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;
    loop {
        // Поддержание стабильного FPS
        let now = Instant::now();
        let delta_time_ms = now.duration_since(last_time).subsec_millis() as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        // Преобразование рекорда в строку для отображения
        let hs_str = format!("{:10}", high_score);

        cnv.draw_strs(&MENU, (1, 1), &MENU_COLOR, &Reset);
        cnv.draw_string(&hs_str, (11, 21), &MENU_COLOR, &Reset);
        cnv.flush();

        let key = inp.get_key();
        match key {
            b'\n' | b'\r' => {
                // Enter — начать игру
                let mut state = GameState::new();
                let new_score = state.play(&mut cnv, &mut inp, hs_str.as_str());

                // Сохранение рекорда в таблицу лидеров
                if new_score > 0 {
                    // Запрос имени для таблицы лидеров
                    let name = get_player_name(&mut cnv, &mut inp);
                    if !name.is_empty() {
                        leaderboard.add_score(name, new_score);
                        leaderboard.save();
                    }

                    // Сохранение как лучшего рекорда
                    if new_score > high_score {
                        high_score = new_score;
                        SaveData::save_value(high_score);
                    }
                }
            }
            b'l' => {
                // L — показать таблицу лидеров
                show_leaderboard(&mut cnv, &mut inp, &leaderboard);
            }
            127 => break, // Backspace — выход
            _ => {}
        }
    }

    cnv.reset();
}

/// Запрос имени игрока после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
///
/// # Возвращает
/// Введённое имя (до 10 символов) или пустую строку при отказе
fn get_player_name(cnv: &mut Canvas, inp: &mut KeyReader) -> String {
    let mut name = String::new();
    const MAX_NAME_LEN: usize = 10;

    cnv.draw_string("Введите имя: ", (2, 10), MENU_COLOR, &Reset);
    cnv.draw_string(&name, (16, 10), MENU_COLOR, &Reset);
    cnv.flush();

    loop {
        let key = inp.get_key();
        if key == b'\n' || key == b'\r' {
            // Enter — подтверждение
            break;
        } else if key == 127 {
            // Backspace — отмена
            if !name.is_empty() {
                name.pop();
                // Очистка и перерисовка
                cnv.draw_string("            ", (16, 10), MENU_COLOR, &Reset);
                cnv.draw_string(&name, (16, 10), MENU_COLOR, &Reset);
                cnv.flush();
            }
        } else if (32..=126).contains(&key) && name.len() < MAX_NAME_LEN {
            // Печатаемые символы
            name.push(key as char);
            cnv.draw_string(&name, (16, 10), MENU_COLOR, &Reset);
            cnv.flush();
        }

        // Таймаут для предотвращения зависания
        sleep(Duration::from_millis(50));
    }

    name
}

/// Показать таблицу лидеров.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `leaderboard` - таблица лидеров для отображения
fn show_leaderboard(cnv: &mut Canvas, inp: &mut KeyReader, leaderboard: &Leaderboard) {
    cnv.draw_strs(&LEADERBOARD_MENU, (1, 1), MENU_COLOR, &Reset);

    let entries = leaderboard.get_entries();
    for (i, entry) in entries.iter().enumerate() {
        let line = format!("{}. {:10} {}", i + 1, entry.name, entry.score);
        cnv.draw_string(&line, (3, 4 + i as u16), MENU_COLOR, &Reset);
    }

    cnv.draw_string("Нажмите любую клавишу", (3, 12), MENU_COLOR, &Reset);
    cnv.flush();

    // Ожидание нажатия любой клавиши
    loop {
        let key = inp.get_key();
        if key != 0 {
            break;
        }
        sleep(Duration::from_millis(50));
    }
}
