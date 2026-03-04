//! Точка входа в программу игры Тетрис.
//!
//! Автор: Dylan Turner

mod game;
mod highscore;
mod io;
mod tetromino;

use crate::game::{GameState, FPS};
use crate::highscore::SaveData;
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

/// Точка входа в приложение.
fn main() {
    // Загрузка рекорда из файла конфигурации
    let save = SaveData::load_config();
    let mut high_score = save.assert_hs();

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
                if new_score > high_score {
                    high_score = new_score;
                    SaveData::save_value(high_score);
                }
            }
            127 => break, // Backspace — выход
            _ => {}
        }
    }

    cnv.reset();
}
