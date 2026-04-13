//! Точка входа CLI Tetris.
//!
//! Вся логика приложения вынесена в библиотеку `tetris_cli`.
//!
use tetris_cli::app;

/// Точка входа в приложение Tetris CLI.
///
/// Делегирует всю логику модулю [`app`].
///
/// ## Возвращает
/// Ничего не возвращает. Завершает процесс с кодом 1 при ошибке.
// `err` используется в макросе log_error!, но clippy не распознаёт макрос как использование
#[allow(unused_variables)]
fn main() {
    if let Err(err) = app::run() {
        tetris_cli::log_error!("[FATAL] Критическая ошибка: {err}");
        std::process::exit(1);
    }
}
