//! Модуль приложения.
//!
//! Предоставляет структуру Application для управления жизненным циклом приложения.

use crate::highscore::{Leaderboard, SaveData};
use crate::io::{Canvas, KeyReader, DISP_HEIGHT, DISP_WIDTH};
use std::error::Error;
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
    ///
    /// # Возвращает
    /// `Result<(), Box<dyn Error>>`
    ///
    /// # Errors
    /// Возвращает ошибку при сбое в цикле меню
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.run_menu_loop()?;
        Ok(())
    }

    /// Главный цикл меню.
    ///
    /// # Аргументы
    /// * `self` - изменяемая ссылка на приложение
    ///
    /// # Возвращает
    /// `Result<(), Box<dyn Error>>`
    fn run_menu_loop(&mut self) -> Result<(), Box<dyn Error>> {
        use crate::game::GameState;
        use crate::game::FPS;
        use crate::menu::{draw_menu, run_game_mode, show_leaderboard};
        use std::{thread::sleep, time::Duration, time::Instant};

        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;

        loop {
            // Поддержание стабильного FPS
            let now = Instant::now();
            let delta_time_ms = u64::from(now.duration_since(last_time).subsec_millis());

            if delta_time_ms < interval_ms {
                sleep(Duration::from_millis(interval_ms - delta_time_ms));
                continue;
            }

            last_time = now;

            // Отрисовка меню
            let high_score_display = format!("{:10}", self.high_score);
            draw_menu(&mut self.canvas, &high_score_display);

            // Обработка ввода
            let key = self.input.get_key();

            match key {
                // ЗАПУСК КЛАССИЧЕСКОЙ ИГРЫ (Enter)
                Some(b'\n' | b'\r') => {
                    let state = GameState::new();
                    let new_score = run_game_mode(
                        &mut self.canvas,
                        &mut self.input,
                        &high_score_display,
                        state,
                        true,
                        &mut self.leaderboard,
                    );
                    if new_score > self.high_score {
                        self.high_score = new_score;
                        SaveData::save_value(self.high_score);
                    }
                }

                // ЗАПУСК РЕЖИМА СПРИНТ (R)
                Some(b'r') => {
                    let state = GameState::new_sprint();
                    run_game_mode(
                        &mut self.canvas,
                        &mut self.input,
                        &high_score_display,
                        state,
                        false,
                        &mut self.leaderboard,
                    );
                }

                // ЗАПУСК РЕЖИМА МАРАФОН (M)
                Some(b'm') => {
                    let state = GameState::new_marathon();
                    let new_score = run_game_mode(
                        &mut self.canvas,
                        &mut self.input,
                        &high_score_display,
                        state,
                        true,
                        &mut self.leaderboard,
                    );
                    if new_score > self.high_score {
                        self.high_score = new_score;
                        SaveData::save_value(self.high_score);
                    }
                }

                // ОТОБРАЖЕНИЕ ТАБЛИЦЫ ЛИДЕРОВ (L)
                Some(b'l') => {
                    show_leaderboard(&mut self.canvas, &mut self.input, &self.leaderboard);
                }

                // ВЫХОД ИЗ ПРИЛОЖЕНИЯ (Backspace)
                Some(crate::io::KEY_BACKSPACE) => break,

                // НЕИЗВЕСТНАЯ КЛАВИША
                Some(_) | None => {}
            }
        }

        Ok(())
    }
}

/// Запустить приложение.
///
/// # Возвращает
/// `Result<(), Box<dyn Error>>`
///
/// # Errors
/// Возвращает ошибку при сбое инициализации или выполнения
pub fn run() -> Result<(), Box<dyn Error>> {
    let mut app = Application::new()?;
    app.run()
}
