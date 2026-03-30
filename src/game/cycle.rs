//! Модуль игрового цикла.
//!
//! Управление игровым циклом:
//! - Поддержание стабильного FPS
//! - Обработка ввода
//! - Обновление состояния
//! - Отрисовка

use std::{thread::sleep, time::Duration};

use super::constants::{BORDER_COLOR, FPS, GAME_OVER, GAME_OVER_DELAY_MS, KEY_BACKSPACE};
use super::state::{GameState, UpdateEndState};
use super::{logic::update, render::update_cached_strings_extended, view::GameView};
use crate::io::{Canvas, KeyReader};
use crate::io_traits::{InputReader, Renderer};
use termion::color::Reset;

// ============================================================================
// ИГРОВОЙ ЦИКЛ
// ============================================================================
//
// ## Архитектурные заметки
//
// **Исправление #13 (LOW):** Удалены избыточные трейты (FPSControl, InputHandler,
// GameUpdater, GameRenderer) которые не использовались полиморфно.
//
// Трейты были определены для будущего разделения игрового цикла на фазы,
// но поскольку они не используются полиморфно, они были удалены для
// уменьшения сложности кода.

/// Результат обработки ввода.
pub enum InputResult {
    /// Продолжить игру.
    Continue,
    /// Выход в меню.
    Quit,
    /// Пауза (ожидание снятия).
    Pause,
    /// Игра окончена.
    GameOver,
    /// Победа (режим спринт/марафон завершён).
    Won,
}

/// Обработать ввод пользователя.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `inp` - читатель нажатий клавиш (реализует трейт InputReader)
/// * `delta_time_ms` - время с последнего кадра (мс)
///
/// # Возвращает
/// Результат обработки ввода
///
/// # Архитектурные заметки (A7: DIP)
/// Использует трейт `InputReader` вместо конкретного типа `KeyReader`.
#[track_caller]
pub fn handle_input<T: InputReader>(
    state: &mut GameState,
    inp: &mut T,
    delta_time_ms: u64,
) -> InputResult {
    match update(state, inp, delta_time_ms) {
        UpdateEndState::Continue => InputResult::Continue,
        UpdateEndState::Quit => InputResult::Quit,
        UpdateEndState::Lost => InputResult::GameOver,
        UpdateEndState::Won => InputResult::Won,
        UpdateEndState::Pause => {
            // Обработка паузы
            loop {
                let key = inp.get_key();
                match key {
                    Ok(Some(b'p')) => break,
                    Ok(Some(KEY_BACKSPACE)) => return InputResult::Quit, // Backspace
                    Ok(Some(_) | None) | Err(_) => {}
                }
                sleep(Duration::from_millis(1_000 / FPS));
            }
            InputResult::Continue
        }
    }
}

/// Отрисовать кадр игры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
/// * `high_score_display` - строка рекорда
///
/// # Архитектурные заметки (H1: DIP)
/// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
#[track_caller]
pub fn render<R: Renderer>(state: &mut GameState, cnv: &mut R, high_score_display: &str) {
    // Обновляем кэшированные строки перед созданием GameView
    update_cached_strings_extended(state, high_score_display);
    // Создаём GameView для отрисовки
    let view = GameView::from_game_state(state);
    // Отрисовываем с использованием GameView
    super::render::draw(&view, cnv);
}

/// Обработать конец игры.
///
/// # Аргументы
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
///
/// # Архитектурные заметки (H1: DIP)
/// Использует трейт `Renderer` вместо конкретного типа `Canvas`.
#[track_caller]
pub fn handle_game_over<R: Renderer>(cnv: &mut R) {
    cnv.draw_strs(&GAME_OVER, (10, 12), BORDER_COLOR, &Reset);
    cnv.flush();
    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
}

/// Запустить игровой цикл и вернуть финальный счёт.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
/// * `inp` - читатель нажатий клавиш (реализует трейт InputReader)
/// * `high_score_display` - строка для отображения рекорда
///
/// # Возвращает
/// - `Ok(u128)` - финальный счёт игрока
/// - `Err(GameError)` - ошибка во время игрового цикла
///
/// # Errors
/// Возвращает ошибку `GameError` при сбое ввода/вывода, ошибке терминала или других критических ошибках во время игрового цикла.
///
/// # Архитектурные заметки (A8: Обработка ошибок, H1: DIP)
/// Функция возвращает `Result<u128, GameError>` для явной обработки ошибок.
/// Использует трейты `InputReader` и `Renderer` для зависимости от абстракций.
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::game::{GameState, cycle::run_game_loop};
/// use tetris_cli::io::{Canvas, KeyReader};
///
/// let mut state = GameState::new();
/// let mut canvas = Canvas::new().unwrap();
/// let mut input = KeyReader::new();
/// let high_score = "1000";
///
/// let result = run_game_loop(&mut state, &mut canvas, &mut input, high_score);
/// ```
#[allow(clippy::unnecessary_wraps)]
#[track_caller]
pub fn run_game_loop<T: InputReader, R: Renderer>(
    state: &mut GameState,
    cnv: &mut R,
    inp: &mut T,
    high_score_display: &str,
) -> Result<u128, super::state::GameError> {
    use std::time::Instant;

    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;

    loop {
        // Поддержание стабильного FPS
        let now = Instant::now();
        // Исправление C1 (CRITICAL): безопасная конвертация u128 -> u64
        // Используем try_into() с unwrap_or(u64::MAX) для предотвращения переполнения
        let delta_time_ms: u64 = now
            .duration_since(last_time)
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(
                interval_ms.saturating_sub(delta_time_ms),
            ));
            continue;
        }
        last_time = now;

        // Обработка ввода и обновления состояния
        match handle_input(state, inp, delta_time_ms) {
            InputResult::Continue | InputResult::Pause => {}
            InputResult::Quit => return Ok(0),
            InputResult::GameOver => {
                handle_game_over(cnv);
                break;
            }
            InputResult::Won => break,
        }

        // Отрисовка кадра
        render(state, cnv, high_score_display);
    }

    Ok(state.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_result_variants() {
        let _continue = InputResult::Continue;
        let _quit = InputResult::Quit;
        let _pause = InputResult::Pause;
        let _game_over = InputResult::GameOver;
        let _won = InputResult::Won;
    }
}
