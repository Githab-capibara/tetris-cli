//! Обработка ввода в главном меню.
//!
//! Этот модуль содержит функции для обработки ввода пользователя:
//! - Запрос имени игрока
//! - Ожидание нажатия клавиши
//!
//! ## Исправление #10 (menu.rs разделение)
//! Выделено из `menu.rs` для улучшения читаемости и разделения ответственности.

use crate::io::{Canvas, KeyReader, KEY_BACKSPACE};
use crate::validation::is_valid_name_char;
use std::{thread::sleep, time::Duration};

use super::constants::{FRAME_DELAY_MS, MAX_NAME_LEN};

/// Запрос имени игрока после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
///
/// # Возвращает
/// Введённое имя игрока (может быть пустым при отмене)
///
/// # Пример
/// ```no_run
/// use tetris_cli::io::{Canvas, KeyReader};
/// use tetris_cli::menu::input::get_player_name;
///
/// let mut canvas = Canvas::new().unwrap();
/// let mut reader = KeyReader::new();
/// let name = get_player_name(&mut canvas, &mut reader);
/// ```
#[track_caller]
pub fn get_player_name(cnv: &mut Canvas, inp: &mut KeyReader) -> String {
    let mut name = String::new();

    // Отрисовка приглашения
    cnv.draw_string(
        "Введите имя: ",
        (2, 10),
        &termion::color::White,
        &termion::color::Reset,
    );
    cnv.draw_string(
        &name,
        (16, 10),
        &termion::color::White,
        &termion::color::Reset,
    );
    cnv.flush();

    // Цикл ввода
    loop {
        let key = inp.get_key();

        match key {
            Ok(Some(b'\n' | b'\r')) => break,
            Ok(Some(KEY_BACKSPACE)) => {
                if !name.is_empty() {
                    name.pop();
                    cnv.draw_string(
                        &format!("{name:<MAX_NAME_LEN$}"),
                        (16, 10),
                        &termion::color::White,
                        &termion::color::Reset,
                    );
                    cnv.flush();
                }
            }
            Ok(Some(key)) if name.len() < MAX_NAME_LEN => {
                let c = key as char;
                if is_valid_name_char(c) {
                    name.push(c);
                    cnv.draw_string(
                        &name,
                        (16, 10),
                        &termion::color::White,
                        &termion::color::Reset,
                    );
                    cnv.flush();
                }
            }
            Ok(Some(_) | None) | Err(_) => {}
        }

        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }

    name
}

/// Ожидать нажатия любой клавиши.
///
/// # Аргументы
/// * `inp` - читатель нажатий клавиш
///
/// # Пример
/// ```no_run
/// use tetris_cli::io::KeyReader;
/// use tetris_cli::menu::input::wait_for_key;
///
/// let mut reader = KeyReader::new();
/// wait_for_key(&mut reader);
/// ```
pub fn wait_for_key(inp: &mut KeyReader) {
    loop {
        let key = inp.get_key();
        if let Ok(Some(_)) = key {
            break;
        }
        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}
