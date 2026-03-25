//! Модуль игрового цикла.
//!
//! Этот модуль содержит методы для управления игровым циклом:
//! - Поддержание стабильного FPS
//! - Обработка ввода
//! - Обновление состояния
//! - Отрисовка
//!
//! ## Архитектурные заметки
//! Выделено из mod.rs для улучшения читаемости и тестируемости.
//!
//! ## Разделение фаз игрового цикла
//! Игровой цикл разделён на три независимые фазы:
//! 1. **FPS Control** (`run_game_loop`) - поддержание стабильного FPS
//! 2. **Input Handling** (`handle_input`) - обработка ввода пользователя
//! 3. **Rendering** (`render`) - отрисовка текущего кадра
//!
//! TODO (#архитектура): Рассмотреть возможность выделения каждой фазы в отдельный трейт
//! для улучшения тестируемости и возможности мокирования.

use std::{thread::sleep, time::Duration};

use super::state::{GameState, UpdateEndState, FPS, GAME_OVER, GAME_OVER_DELAY_MS};
use super::{logic::update, render::update_cached_strings_extended, view::GameView};
use crate::io::Canvas;
use termion::color::Reset;

/// Результат обработки ввода.
pub enum InputResult {
    /// Продолжить игру.
    Continue,
    /// Выход в меню.
    Quit,
    /// Пауза (ожидание снятия).
    #[allow(dead_code)]
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
/// * `inp` - читатель нажатий клавиш
/// * `delta_time_ms` - время с последнего кадра (мс)
///
/// # Возвращает
/// Результат обработки ввода
#[track_caller]
pub fn handle_input(
    state: &mut GameState,
    inp: &mut crate::io::KeyReader,
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
                    Some(b'p') => break,
                    Some(127) => return InputResult::Quit, // Backspace
                    Some(_) | None => {}
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
/// * `cnv` - канвас для отрисовки
/// * `high_score_display` - строка рекорда
#[track_caller]
pub fn render(state: &mut GameState, cnv: &mut Canvas, high_score_display: &str) {
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
/// * `cnv` - канвас для отрисовки
#[track_caller]
pub fn handle_game_over(cnv: &mut Canvas) {
    cnv.draw_strs(&GAME_OVER, (10, 12), super::state::BORDER_COLOR, &Reset);
    cnv.flush();
    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
}

/// Запустить игровой цикл и вернуть финальный счёт.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `cnv` - канвас для отрисовки
/// * `inp` - читатель нажатий клавиш
/// * `high_score_display` - строка для отображения рекорда
///
/// # Возвращает
/// Финальный счёт игрока
#[track_caller]
pub fn run_game_loop(
    state: &mut GameState,
    cnv: &mut Canvas,
    inp: &mut crate::io::KeyReader,
    high_score_display: &str,
) -> u128 {
    use std::time::Instant;

    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;

    loop {
        // Поддержание стабильного FPS
        let now = Instant::now();
        let delta_time_ms = now.duration_since(last_time).as_millis() as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        // Обработка ввода и обновления состояния
        match handle_input(state, inp, delta_time_ms) {
            InputResult::Continue => {}
            InputResult::Quit => return 0,
            InputResult::GameOver => {
                handle_game_over(cnv);
                break;
            }
            InputResult::Won => break,
            InputResult::Pause => {} // Уже обработано в handle_input
        }

        // Отрисовка кадра
        render(state, cnv, high_score_display);
    }

    state.score
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
