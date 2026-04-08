//! Обработка ввода в главном меню.
//!
//! ## Исправление #10 (menu.rs разделение)
//! Выделено из `menu.rs` для улучшения читаемости и разделения ответственности.

use crate::constants::KEY_BACKSPACE;
use crate::io::{Canvas, KeyReader};
use crate::validation::is_valid_name_char;
use std::{thread::sleep, time::Duration};

use super::constants::{FRAME_DELAY_MS, MAX_NAME_LEN};

/// Максимальное количество попыток чтения ввода перед возвратом ошибки.
const MAX_INPUT_ATTEMPTS: u64 = 10_000;

/// Запрос имени игрока после завершения игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
///
/// # Возвращает
/// Введённое имя игрока (может быть пустым при отмене)
///
/// # Errors
/// Возвращает ошибку после `MAX_INPUT_ATTEMPTS` неудачных попыток чтения ввода.
///
/// # Валидация
/// ## Правила ввода
/// - **Максимальная длина**: `MAX_NAME_LEN` символов
/// - **Разрешённые символы**: Только прошедшие проверку `is_valid_name_char()`
///   - ASCII буквы (a-z, A-Z)
///   - ASCII цифры (0-9)
///   - Специальные символы: '_', '-', ' ' (пробел)
///   - Русские буквы (а-я, А-Я, ё, Ё)
/// - **Запрещённые символы**:
///   - Управляющие символы (control characters)
///   - Символы пути (/, \\)
///   - Специальные символы (@, #, $, %, и т.д.)
///
/// ## Обработка ввода
/// - **Enter (\\n, \\r)**: Завершение ввода, возврат имени
/// - **Backspace**: Удаление последнего символа
/// - **Другие символы**: Добавление если проходит валидацию
///
/// ## Пустое имя
/// Если игрок не ввёл имя (только Enter), возвращается пустая строка.
/// Санитаризация выполняется позже в `sanitize_player_name()`.
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

    // M7: Счётчик попыток для защиты от бесконечного цикла
    let mut failed_attempts: u64 = 0;

    // Цикл ввода
    loop {
        let key = inp.get_key();

        match key {
            Ok(Some(b'\n' | b'\r')) => break,
            Ok(Some(KEY_BACKSPACE)) => {
                failed_attempts = 0;
                if !name.is_empty() {
                    name.pop();
                    cnv.draw_string(
                        &format!("{name:<MAX_NAME_LEN$}"),
                        (16, 10),
                        &termion::color::White,
                        &termion::color::Reset,
                    );
                    // Осознанное решение: flush на каждый символ для немедленной обратной связи (UX)
                    cnv.flush();
                }
            }
            Ok(Some(key)) if name.len() < MAX_NAME_LEN => {
                failed_attempts = 0;
                #[allow(clippy::cast_lossless)]
                let c = key as char;
                if is_valid_name_char(c) {
                    name.push(c);
                    cnv.draw_string(
                        &name,
                        (16, 10),
                        &termion::color::White,
                        &termion::color::Reset,
                    );
                    // Осознанное решение: flush на каждый символ для немедленной обратной связи (UX)
                    cnv.flush();
                }
            }
            Ok(Some(_) | None) | Err(_) => {
                // M7: Считаем только ошибки чтения (Err) и долгие простои (None)
                failed_attempts = failed_attempts.saturating_add(1);
                if failed_attempts >= MAX_INPUT_ATTEMPTS {
                    crate::log_error!(
                        "get_player_name: превышено максимальное количество попыток ввода ({MAX_INPUT_ATTEMPTS})"
                    );
                    break;
                }
            }
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
/// # Возвращает
/// `true` если клавиша была нажата, `false` при превышении лимита попыток.
pub fn wait_for_key(inp: &mut KeyReader) -> bool {
    // M7: Счётчик попыток для защиты от бесконечного цикла
    let mut failed_attempts: u64 = 0;

    loop {
        let key = inp.get_key();
        if let Ok(Some(_)) = key {
            return true;
        }
        failed_attempts = failed_attempts.saturating_add(1);
        if failed_attempts >= MAX_INPUT_ATTEMPTS {
            crate::log_error!(
                "wait_for_key: превышено максимальное количество попыток ввода ({MAX_INPUT_ATTEMPTS})"
            );
            return false;
        }
        sleep(Duration::from_millis(FRAME_DELAY_MS));
    }
}
