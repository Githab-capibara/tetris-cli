//! Модуль игрового цикла.
//!
//! Управление игровым циклом:
//! - Поддержание стабильного FPS
//! - Обработка ввода
//! - Обновление состояния
//! - Отрисовка

use std::{thread::sleep, time::Duration};

use super::constants::{BORDER_COLOR, FPS, GAME_OVER, GAME_OVER_DELAY_MS};
use super::state::{GameState, UpdateEndState};
use super::{logic::update, render::update_cached_strings_extended, view::GameView};
use crate::io::Canvas;
use termion::color::Reset;

// ============================================================================
// ТРЕЙТЫ ДЛЯ ФАЗ ИГРОВОГО ЦИКЛА
// ============================================================================
// TODO (#архитектура): Использовать эти трейты для разделения ответственности
// в игровом цикле. Каждый трейт отвечает за свою фазу.

/// Трейт для управления FPS.
///
/// Отвечает за поддержание стабильной частоты кадров.
/// TODO (#архитектура): Реализовать в отдельном модуле `fps_controller.rs`
#[allow(dead_code)] // Будет использоваться в будущей рефакторизации
pub trait FPSControl {
    /// Поддержать стабильный FPS.
    ///
    /// # Аргументы
    /// * `frame_start` - время начала кадра
    /// * `target_fps` - целевое количество кадров в секунду
    fn maintain_fps(&self, frame_start: std::time::Instant, target_fps: u64);
}

/// Трейт для обработки ввода.
///
/// Отвечает за обработку ввода пользователя.
/// TODO (#архитектура): Реализовать в отдельном модуле `input_handler.rs`
#[allow(dead_code)] // Будет использоваться в будущей рефакторизации
pub trait InputHandler {
    /// Тип результата обработки ввода.
    type InputResult;

    /// Обработать ввод пользователя.
    ///
    /// # Аргументы
    /// * `state` - состояние игры
    /// * `delta_time_ms` - время с последнего кадра (мс)
    ///
    /// # Возвращает
    /// Результат обработки ввода
    fn process_input(&mut self, state: &mut GameState, delta_time_ms: u64) -> Self::InputResult;
}

/// Трейт для обновления состояния.
///
/// Отвечает за обновление игрового состояния.
/// TODO (#архитектура): Реализовать в отдельном модуле `game_updater.rs`
#[allow(dead_code)] // Будет использоваться в будущей рефакторизации
pub trait GameUpdater {
    /// Обновить состояние игры.
    ///
    /// # Аргументы
    /// * `state` - состояние игры
    /// * `delta_time_ms` - время с последнего кадра (мс)
    fn update_state(&mut self, state: &mut GameState, delta_time_ms: u64);
}

/// Трейт для отрисовки.
///
/// Отвечает за отрисовку текущего кадра.
/// TODO (#архитектура): Реализовать в отдельном модуле `game_renderer.rs`
#[allow(dead_code)] // Будет использоваться в будущей рефакторизации
pub trait GameRenderer {
    /// Отрисовать кадр игры.
    ///
    /// # Аргументы
    /// * `state` - состояние игры
    /// * `canvas` - канвас для отрисовки
    /// * `high_score_display` - строка рекорда
    fn render(&self, state: &mut GameState, canvas: &mut Canvas, high_score_display: &str);
}

// ============================================================================
// РЕАЛИЗАЦИЯ ТРЕЙТОВ ПО УМОЛЧАНИЮ
// ============================================================================
// TODO (#архитектура): Переместить эти реализации в отдельные модули

/// Реализация FPSControl по умолчанию.
#[allow(dead_code)] // Будет использоваться в будущей рефакторизации
pub struct DefaultFPSControl;

impl FPSControl for DefaultFPSControl {
    fn maintain_fps(&self, frame_start: std::time::Instant, target_fps: u64) {
        let interval_ms = 1_000 / target_fps;
        let elapsed_ms = frame_start.elapsed().as_millis() as u64;
        if elapsed_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - elapsed_ms));
        }
    }
}

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
    cnv.draw_strs(&GAME_OVER, (10, 12), BORDER_COLOR, &Reset);
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
