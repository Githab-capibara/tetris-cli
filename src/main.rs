//! Точка входа в программу игры Тетрис.
//!
//! Автор: Dylan Turner

mod io;
mod game;
mod highscore;
mod tetromino;

use std::{
    thread::sleep,
    time::{
        Instant, Duration
    }
};
use termion::{
    color::{
        Color, White, Reset
    },
    terminal_size
};
use crate::io::{
    DISP_WIDTH, DISP_HEIGHT, Canvas, KeyReader
};
use crate::game::{
    FPS, GameState
};
use crate::highscore::SaveData;

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
    "                      "
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
        let delta_time_ms =
            (now.duration_since(last_time).subsec_nanos() / 1_000_000) as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        // Преобразование рекорда в строку для отображения
        let hs_str = format!("{:020}", high_score);

        cnv.draw_strs(&MENU.to_vec(), (1, 1), &MENU_COLOR, &Reset);
        cnv.draw_string(&hs_str, (2, 22), &MENU_COLOR, &Reset);
        cnv.flush();

        let key = inp.get_key();
        match key {
            b'\n' | b'\r' => {
                // Enter — начать игру
                let mut state = GameState::new();
                let new_score = state.play(&mut cnv, &mut inp, &hs_str);
                if new_score > high_score {
                    high_score = new_score;
                    SaveData::save_value(high_score);
                }
            },
            127 => break, // Backspace — выход
            _ => {}
        }
    }

    cnv.reset();
}
