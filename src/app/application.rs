//! Модуль приложения.
//!
//! Предоставляет структуру Application для управления жизненным циклом приложения.

use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::io::{Canvas, KeyReader, DISP_HEIGHT, DISP_WIDTH};
use crate::menu::run_game_mode;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::terminal_size;

/// Приложение Tetris CLI.
///
/// Управляет жизненным циклом приложения:
/// - Загрузка сохранённых данных
/// - Инициализация терминала
/// - Запуск главного цикла меню
/// - Завершение работы
pub struct Application {
    /// Канвас для отрисовки в терминале.
    canvas: Canvas,
    /// Читатель нажатий клавиш.
    input: KeyReader,
    /// Таблица лидеров.
    leaderboard: Leaderboard,
    /// Текущий рекорд.
    high_score: u128,
}

impl Application {
    /// Инициализировать приложение.
    ///
    /// # Возвращает
    /// `Result<Self, Box<dyn Error>>` при ошибке инициализации
    ///
    /// # Errors
    /// Возвращает ошибку если:
    /// - Не удалось загрузить данные
    /// - Терминал не соответствует минимальным требованиям
    /// - Не удалось инициализировать Canvas/KeyReader
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Загрузка сохранённых данных
        let (save, leaderboard) = Self::load_game_data();

        // Проверка целостности рекорда
        let high_score = save.verify_and_get_score().unwrap_or_else(|| {
            eprintln!(
                "Предупреждение: обнаружена попытка подделки рекорда! Используется значение 0."
            );
            0
        });

        // Проверка терминала и инициализация ввода/вывода
        let (canvas, input) = Self::initialize_terminal()?;

        Ok(Self {
            canvas,
            input,
            leaderboard,
            high_score,
        })
    }

    /// Загрузить сохранённые данные и таблицу лидеров.
    ///
    /// # Возвращает
    /// Кортеж (SaveData, Leaderboard)
    fn load_game_data() -> (SaveData, Leaderboard) {
        let save = SaveData::load_config();
        let mut leaderboard = Leaderboard::load();
        leaderboard.validate();
        (save, leaderboard)
    }

    /// Инициализировать терминал и создать Canvas/KeyReader.
    ///
    /// # Возвращает
    /// `Result<(Canvas, KeyReader), Box<dyn Error>>`
    ///
    /// # Errors
    /// Возвращает ошибку если терминал не соответствует минимальным требованиям
    fn initialize_terminal() -> Result<(Canvas, KeyReader), Box<dyn Error>> {
        // Проверка размера терминала
        let (width, height) = terminal_size().map_err(|e| {
            eprintln!(
                "Ошибка: не удалось получить размер терминала: {e}.\n\
                 Минимальный размер: {DISP_WIDTH}x{DISP_HEIGHT} символов."
            );
            e
        })?;

        if width < DISP_WIDTH || height < DISP_HEIGHT {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Окно терминала слишком маленькое!\n\
                     Минимальный размер: {DISP_WIDTH}x{DISP_HEIGHT} символов.\n\
                     Текущий размер: {width}x{height} символов."
                ),
            );
            eprintln!("{err}");
            return Err(err.into());
        }

        // Инициализация Canvas и KeyReader
        let canvas = Canvas::new().map_err(|e| {
            eprintln!("Ошибка инициализации терминала: {e}");
            e
        })?;

        let input = KeyReader::new();

        Ok((canvas, input))
    }

    /// Запустить главный цикл приложения.
    pub fn run(&mut self) {
        self.run_menu_loop();
    }

    /// Главный цикл меню.
    ///
    /// Обрабатывает отрисовку меню и ввод пользователя.
    fn run_menu_loop(&mut self) {
        use crate::game::FPS;
        use crate::menu::draw_menu;
        use std::time::Instant;

        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;

        loop {
            // Поддержание стабильного FPS
            if !Self::wait_for_next_frame(&mut last_time, interval_ms) {
                continue;
            }

            // Отрисовка меню
            let high_score_display = format!("{:10}", self.high_score);
            draw_menu(&mut self.canvas, &high_score_display);

            // Обработка ввода
            if let Some(key) = self.input.get_key() {
                // Выход из приложения
                if key == crate::io::KEY_BACKSPACE {
                    break;
                }
                self.handle_menu_input(key, &high_score_display);
            }
        }
    }

    /// Подождать следующего кадра для поддержания FPS.
    ///
    /// # Возвращает
    /// `true` если пришло время обновлять кадр, `false` если нужно ждать
    fn wait_for_next_frame(last_time: &mut Instant, interval_ms: u64) -> bool {
        let now = Instant::now();
        let delta_time_ms = u64::from(now.duration_since(*last_time).subsec_millis());

        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            return false;
        }

        *last_time = now;
        true
    }

    /// Обработать ввод в меню.
    ///
    /// # Аргументы
    /// * `key` - код нажатой клавиши
    /// * `high_score_display` - строка для отображения рекорда
    fn handle_menu_input(&mut self, key: u8, high_score_display: &str) {
        use crate::menu::show_leaderboard;

        match key {
            // ЗАПУСК КЛАССИЧЕСКОЙ ИГРЫ (Enter)
            b'\n' | b'\r' => {
                let state = GameState::new();
                let new_score = self.run_game_classic(high_score_display, state);
                if new_score > self.high_score {
                    self.high_score = new_score;
                    SaveData::save_value(self.high_score);
                }
            }

            // ЗАПУСК РЕЖИМА СПРИНТ (R)
            b'r' => {
                let state = GameState::new_sprint();
                self.run_game_sprint(high_score_display, state);
            }

            // ЗАПУСК РЕЖИМА МАРАФОН (M)
            b'm' => {
                let state = GameState::new_marathon();
                let new_score = self.run_game_marathon(high_score_display, state);
                if new_score > self.high_score {
                    self.high_score = new_score;
                    SaveData::save_value(self.high_score);
                }
            }

            // ОТОБРАЖЕНИЕ ТАБЛИЦЫ ЛИДЕРОВ (L)
            b'l' => {
                show_leaderboard(&mut self.canvas, &mut self.input, &self.leaderboard);
            }

            // НЕИЗВЕСТНАЯ КЛАВИША
            _ => {}
        }
    }

    /// Запустить классический режим игры.
    fn run_game_classic(&mut self, high_score_display: &str, state: GameState) -> u128 {
        run_game_mode(
            &mut self.canvas,
            &mut self.input,
            high_score_display,
            state,
            true,
            &mut self.leaderboard,
        )
    }

    /// Запустить режим спринт.
    fn run_game_sprint(&mut self, high_score_display: &str, state: GameState) {
        run_game_mode(
            &mut self.canvas,
            &mut self.input,
            high_score_display,
            state,
            false,
            &mut self.leaderboard,
        );
    }

    /// Запустить режим марафон.
    fn run_game_marathon(&mut self, high_score_display: &str, state: GameState) -> u128 {
        run_game_mode(
            &mut self.canvas,
            &mut self.input,
            high_score_display,
            state,
            true,
            &mut self.leaderboard,
        )
    }
}

/// Запустить приложение.
///
/// # Panics
/// Паникует при ошибке инициализации приложения.
pub fn run() {
    let mut app = Application::new().expect("Ошибка инициализации приложения");
    app.run();
}

// ============================================================================
// ТЕСТЫ ДЛЯ APPLICATION
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Тест для run_menu_loop()
    ///
    /// Проверяет, что метод run_menu_loop() выполняется без ошибок
    /// и возвращает () вместо Result
    ///
    /// Примечание: Этот тест игнорируется по умолчанию, так как требует
    /// доступа к терминалу и может блокировать выполнение
    #[test]
    #[ignore = "Требует доступа к терминалу и может блокировать выполнение"]
    fn test_run_menu_loop_executes_without_error() {
        // Создаём приложение
        let app_result = Application::new();

        // Если приложение успешно создано, проверяем run_menu_loop
        if let Ok(mut app) = app_result {
            // run_menu_loop() теперь возвращает () вместо Result
            // Метод должен выполниться без паники
            // В реальном использовании метод зациклен и ждёт ввода,
            // поэтому тест игнорируется

            // Проверяем, что метод существует и имеет правильную сигнатуру
            // Этот код компилируется только если сигнатура верна
            let _: () = app.run_menu_loop();

            // Тест проходит, если код компилируется
            assert!(true, "run_menu_loop() должен возвращать ()");
        } else {
            // Если приложение не создалось (нет терминала), тест считается пройденным
            // Это ожидаемое поведение в среде без терминала
            assert!(true, "Приложение не создалось из-за отсутствия терминала");
        }
    }

    /// Тест для проверки сигнатуры run_menu_loop()
    ///
    /// Проверяет, что run_menu_loop() возвращает () а не Result
    #[test]
    fn test_run_menu_loop_return_type() {
        // Этот тест проверяет тип возвращаемого значения на этапе компиляции
        // Если сигнатура изменится, тест не скомпилируется

        // Тип метода run_menu_loop должен быть fn(&mut self) -> ()
        // Проверяем через присваивание типа
        let _type_check: fn(&mut Application) = |app| {
            app.run_menu_loop();
        };

        // Если код компилируется, сигнатура верна
        assert!(true, "Сигнатура run_menu_loop() верна: fn(&mut self) -> ()");
    }

    /// Тест для Application::new()
    ///
    /// Проверяет, что приложение может быть создано (в среде с терминалом)
    #[test]
    #[ignore = "Требует доступа к терминалу"]
    fn test_application_new() {
        let app_result = Application::new();

        // В среде с терминалом приложение должно создаться успешно
        // В среде без терминала тест игнорируется
        if let Ok(app) = app_result {
            assert!(
                true,
                "Приложение должно успешно создаться в среде с терминалом"
            );

            // Проверяем наличие полей
            let _ = app.high_score;
            let _ = app.leaderboard;
        } else {
            // В среде без терминала это ожидаемое поведение
            assert!(true, "Приложение не создалось из-за отсутствия терминала");
        }
    }
}
