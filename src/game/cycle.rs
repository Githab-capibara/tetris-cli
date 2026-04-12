//! Модуль игрового цикла.
//!
//! Управление игровым циклом:
//! - Поддержание стабильного FPS
//! - Обработка ввода
//! - Обновление состояния
//! - Отрисовка

use std::{thread::sleep, time::Duration};

use super::{logic::update_state, render::update_cached_strings_extended, view::GameView};
use crate::constants::{
    BORDER_COLOR, FRAME_DELAY_MS, GAME_OVER, GAME_OVER_DELAY_MS, KEY_BACKSPACE,
};
use crate::game::state::GameState;
use crate::io_traits::{InputReader, Renderer};
use crate::types::UpdateEndState;
use crate::util::frame_timing::maintain_fps;
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
    /// Игра окончена.
    GameOver,
    /// Победа (режим спринт/марафон завершён).
    Won,
}

/// Обработать ввод пользователя.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `inp` - читатель нажатий клавиш (реализует трейт `InputReader`)
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
    match update_state(state, inp, delta_time_ms) {
        UpdateEndState::Continue => InputResult::Continue,
        UpdateEndState::Quit => InputResult::Quit,
        UpdateEndState::Lost => InputResult::GameOver,
        UpdateEndState::Won => InputResult::Won,
        UpdateEndState::Pause => {
            // Обработка паузы
            let mut consecutive_errors = 0u32;
            loop {
                let key = inp.get_key();
                match key {
                    Ok(Some(b'p')) => break,
                    Ok(Some(KEY_BACKSPACE)) => return InputResult::Quit, // Backspace
                    Ok(Some(_) | None) => {
                        // Клавиша не нажата или не распознана — не считаем ошибкой
                    }
                    // Переменная `e` используется в макросе log_error!
                    #[allow(unused_variables)]
                    Err(e) => {
                        // Счётчик ошибок — первые 5 логируем всегда, затем каждую 10-ю
                        // чтобы не затоплять stderr но сохранять диагностику
                        // Исправление аудита #48: saturating_add для защиты от overflow
                        consecutive_errors = consecutive_errors.saturating_add(1);
                        if consecutive_errors <= 5 || consecutive_errors % 10 == 0 {
                            crate::log_error!(
                                "Ошибка чтения ввода во время паузы (#{consecutive_errors}): {e}"
                            );
                        }
                    }
                }
                sleep(Duration::from_millis(FRAME_DELAY_MS));
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

/// Обработать результат ввода.
///
/// # Аргументы
/// * `input_result` - результат обработки ввода
/// * `state` - состояние игры для получения финального счёта
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
///
/// # Возвращает
/// - `Some(final_score)` - игра завершена, вернуть счёт
/// - `None` - продолжить игру
#[inline]
fn handle_input_result<R: Renderer>(
    input_result: &InputResult,
    state: &GameState,
    cnv: &mut R,
) -> Option<u128> {
    match input_result {
        InputResult::Continue => None,
        InputResult::Quit | InputResult::Won => Some(state.score()),
        InputResult::GameOver => {
            handle_game_over(cnv);
            Some(state.score())
        }
    }
}

/// Запустить игровой цикл и вернуть финальный счёт.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `cnv` - канвас для отрисовки (реализует трейт Renderer)
/// * `inp` - читатель нажатий клавиш (реализует трейт `InputReader`)
/// * `high_score_display` - строка для отображения рекорда
///
/// # Возвращает
/// - `u128` - финальный счёт игрока
///
/// # Архитектурные заметки (Исправление E11)
/// Функция возвращает `u128` напрямую — обёртка `Result` была удалена,
/// так как функция всегда возвращает `Ok(final_score)`.
///
/// # Архитектурные заметки (A8: Обработка ошибок, H1: DIP)
/// Использует трейты `InputReader` и `Renderer` для зависимости от абстракций.
///
/// # Архитектурные заметки (Исправление аудита 2026-03-31)
/// Функция разбита на подфункции для улучшения читаемости:
/// - `maintain_fps()` - поддержание стабильного FPS
/// - `handle_input()` - обработка ввода пользователя
/// - `handle_input_result()` - обработка результата ввода
/// - `render()` - отрисовка текущего кадра
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::game::{GameState, cycle::run_game_loop};
/// use tetris_cli::io::{Canvas, KeyReader};
///
/// let mut state = GameState::new();
/// let mut canvas = Canvas::new().expect("canvas creation");
/// let mut input = KeyReader::new();
/// let high_score = "1000";
///
/// let result = run_game_loop(&mut state, &mut canvas, &mut input, high_score);
/// ```
#[track_caller]
pub fn run_game_loop<T: InputReader, R: Renderer>(
    state: &mut GameState,
    cnv: &mut R,
    inp: &mut T,
    high_score_display: &str,
) -> u128 {
    let mut last_time = std::time::Instant::now();
    let interval_ms = FRAME_DELAY_MS;

    loop {
        // Поддержание стабильного FPS (вынесено в отдельную функцию)
        if let Some(delta_time_ms) = maintain_fps(&mut last_time, interval_ms) {
            // Обработка ввода и обновления состояния
            let input_result = handle_input(state, inp, delta_time_ms);

            // Обработка результата ввода (вынесено в отдельную функцию)
            if let Some(final_score) = handle_input_result(&input_result, state, cnv) {
                return final_score;
            }

            // Отрисовка кадра
            render(state, cnv, high_score_display);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::frame_timing::maintain_fps;

    // ========================================================================
    // Исправление аудита 2026-04-11 (Пакет 6, #101-#105):
    // MockRenderer вынесен в общий для всех тестов — устранено 5-кратное дублирование
    // ========================================================================
    struct MockRenderer;
    impl crate::io_traits::Renderer for MockRenderer {
        fn draw_strs(
            &mut self,
            _lines: &[&str],
            _pos: (u16, u16),
            _fg: &dyn termion::color::Color,
            _bg: &dyn termion::color::Color,
        ) {
        }
        fn draw_string(
            &mut self,
            _string: &str,
            _pos: (u16, u16),
            _fg: &dyn termion::color::Color,
            _bg: &dyn termion::color::Color,
        ) {
        }
        fn flush(&mut self) {}
        fn reset(&mut self) {}
    }

    #[test]
    #[allow(clippy::no_effect_underscore_binding)]
    fn test_input_result_variants() {
        let _continue = InputResult::Continue;
        let _quit = InputResult::Quit;
        let _game_over = InputResult::GameOver;
        let _won = InputResult::Won;
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ИСПРАВЛЕНИЯ АУДИТА 2026-03-31: maintain_fps()
    // =========================================================================

    /// Тест: `maintain_fps()` корректно регулирует FPS
    ///
    /// #[ignore] — flaky: depends on OS timing, нестабилен на разных машинах.
    #[test]
    #[ignore = "flaky: depends on OS timing"]
    fn test_maintain_fps_regulates_fps() {
        let mut last_time = std::time::Instant::now();
        let interval_ms = FRAME_DELAY_MS;

        // Ждём немного меньше интервала
        std::thread::sleep(std::time::Duration::from_millis(interval_ms / 2));

        // maintain_fps() должен вернуть None так как прошло меньше интервала
        let result = maintain_fps(&mut last_time, interval_ms);
        assert!(
            result.is_none(),
            "maintain_fps() должен вернуть None когда прошло меньше интервала"
        );
    }

    /// Тест: `maintain_fps()` возвращает Some когда интервал прошёл
    ///
    /// #[ignore] — flaky: depends on OS timing, нестабилен на разных машинах.
    #[test]
    #[ignore = "flaky: depends on OS timing"]
    fn test_maintain_fps_returns_some_after_interval() {
        let mut last_time = std::time::Instant::now();
        let interval_ms = FRAME_DELAY_MS;

        // Ждём больше интервала
        std::thread::sleep(std::time::Duration::from_millis(interval_ms + 10));

        // maintain_fps() должен вернуть Some(delta_time_ms)
        let result = maintain_fps(&mut last_time, interval_ms);
        assert!(
            result.is_some(),
            "maintain_fps() должен вернуть Some когда прошло больше интервала"
        );
        assert!(
            result.unwrap() >= interval_ms,
            "delta_time_ms должен быть >= interval_ms"
        );
    }

    /// Тест: `maintain_fps()` обновляет `last_time`
    ///
    /// #[ignore] — flaky: depends on OS timing, нестабилен на разных машинах.
    #[test]
    #[ignore = "flaky: depends on OS timing"]
    fn test_maintain_fps_updates_last_time() {
        let mut last_time = std::time::Instant::now();
        let interval_ms = FRAME_DELAY_MS;

        // Сохраняем старое значение
        let old_last_time = last_time;

        // Ждём больше интервала
        std::thread::sleep(std::time::Duration::from_millis(interval_ms + 10));

        // Вызываем maintain_fps
        let _ = maintain_fps(&mut last_time, interval_ms);

        // last_time должен обновиться
        assert!(
            last_time > old_last_time,
            "last_time должен обновиться после maintain_fps"
        );
    }

    /// Тест: `maintain_fps()` обрабатывает переполнение u128 -> u64
    #[test]
    fn test_maintain_fps_handles_overflow() {
        let mut last_time = std::time::Instant::now();
        let interval_ms = FRAME_DELAY_MS;

        // Вызываем maintain_fps немедленно (без ожидания)
        let result = maintain_fps(&mut last_time, interval_ms);

        // Должен вернуть None так как прошло очень мало времени
        assert!(
            result.is_none(),
            "maintain_fps() должен обработать малый delta_time"
        );
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ИСПРАВЛЕНИЯ АУДИТА 2026-03-31: handle_input_result()
    // =========================================================================

    /// Тест: `handle_input_result()` правильно обрабатывает `InputResult::Continue`
    #[test]
    fn test_handle_input_result_continue() {
        let mut renderer = MockRenderer;
        let state = GameState::new();

        // InputResult::Continue должен вернуть None
        let result = handle_input_result(&InputResult::Continue, &state, &mut renderer);
        assert!(result.is_none(), "Continue должен вернуть None");
    }

    /// Тест: `handle_input_result()` правильно обрабатывает `InputResult::Quit`
    #[test]
    fn test_handle_input_result_quit() {
        let mut renderer = MockRenderer;
        let state = GameState::new();

        // InputResult::Quit должен вернуть Some(state.score())
        let result = handle_input_result(&InputResult::Quit, &state, &mut renderer);
        assert_eq!(
            result,
            Some(state.score()),
            "Quit должен вернуть Some(state.score())"
        );
    }

    /// Тест: `handle_input_result()` правильно обрабатывает `InputResult::GameOver`
    #[test]
    fn test_handle_input_result_game_over() {
        let mut renderer = MockRenderer;
        let state = GameState::new();

        // InputResult::GameOver должен вернуть Some(state.score())
        let result = handle_input_result(&InputResult::GameOver, &state, &mut renderer);
        assert_eq!(
            result,
            Some(0),
            "GameOver должен вернуть Some(state.score())"
        );
    }

    /// Тест: `handle_input_result()` правильно обрабатывает `InputResult::Won`
    #[test]
    fn test_handle_input_result_won() {
        let mut renderer = MockRenderer;
        let state = GameState::new();

        // InputResult::Won должен вернуть Some(state.score())
        let result = handle_input_result(&InputResult::Won, &state, &mut renderer);
        assert_eq!(result, Some(0), "Won должен вернуть Some(state.score())");
    }

    // =========================================================================
    // ИНТЕГРАЦИОННЫЕ ТЕСТЫ
    // =========================================================================

    /// Тест: все `InputResult` варианты обрабатываются корректно
    #[test]
    fn test_all_input_result_variants_handled() {
        let mut renderer = MockRenderer;
        let state = GameState::new();

        // Проверяем все варианты
        assert!(handle_input_result(&InputResult::Continue, &state, &mut renderer).is_none());
        assert_eq!(
            handle_input_result(&InputResult::Quit, &state, &mut renderer),
            Some(state.score())
        );
        assert_eq!(
            handle_input_result(&InputResult::GameOver, &state, &mut renderer),
            Some(0)
        );
        assert_eq!(
            handle_input_result(&InputResult::Won, &state, &mut renderer),
            Some(0)
        );
    }
}
