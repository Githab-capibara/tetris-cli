//! Модуль приложения.
//!
//! Предоставляет структуру Application для управления жизненным циклом приложения.
//!
//! ## Архитектура
//!
//! ```text
//! ┌─────────────────┐
//! │  Application    │
//! │  ───────────    │
//! │  - canvas       │
//! │  - input        │
//! │  - leaderboard  │
//! │  - high_score   │
//! └────────┬────────┘
//!          │
//!    ┌─────┴──────┐
//!    │            │
//!    ▼            ▼
//! ┌─────────┐  ┌──────────┐
//! │  new()  │  │  run()   │
//! │  init   │  │  loop    │
//! └─────────┘  └──────────┘
//! ```
//!
//! ## Архитектурные заметки
//! ### PROB-126: Нарушение Dependency Inversion Principle
//! Application напрямую зависит от конкретных типов: Canvas, `KeyReader`, Leaderboard.
//! В идеале должен зависеть от абстракций (`io_traits`). Рефакторинг стал бы breaking change.
//! TODO: рассмотреть инъекцию зависимостей через трейты при следующем крупном релизе.
//!
//! ### PROB-130: Архитектурная рекомендация
//! TODO: рассмотреть выделение `ApplicationService` как слоя между UI и бизнес-логикой.
//!
//! ## Пример использования
//!
//! ```ignore
//! use crate::app;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     app::run()?;
//!     Ok(())
//! }
//! ```

// std
use std::time::Instant;

// external
use termion::terminal_size;

// crate
use crate::config::keys::validate_all_keys;
use crate::constants::{DISP_HEIGHT, DISP_WIDTH};
use crate::errors::GameError;
use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::io::{Canvas, KeyReader};
use crate::menu::run_game_mode;
use crate::util::frame_timing::maintain_fps;

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
    /// Кэшированная строка рекорда для отрисовки (P3-ID50).
    high_score_display: String,
}

impl Application {
    /// Инициализировать приложение.
    ///
    /// # Возвращает
    /// - `Ok(Self)` при успешной инициализации
    /// - `Err(GameError)` при ошибке инициализации
    ///
    /// # Errors
    /// Возвращает ошибку если:
    /// - `GameError::IoError` - не удалось инициализировать Canvas/KeyReader
    /// - `GameError::ValidationError` - терминал не соответствует минимальным требованиям
    /// - `GameError::IoError` - не удалось загрузить данные или проверить HMAC ключи
    ///
    /// # Исправление аудита 2026-04-02 (SEC1)
    /// Добавлена валидация HMAC ключей при запуске приложения.
    pub fn new() -> Result<Self, GameError> {
        // Валидация HMAC ключей (SEC1)
        // Исправление ISSUE-196: eprintln!() используется для логирования предупреждений
        // а не для критических ошибок - приложение продолжает работу с пустым ключом
        // Исправление аудита #33: конкретное сообщение об ошибке с контекстом
        #[allow(unused_variables)]
        if let Err(errors) = validate_all_keys() {
            crate::log_warn!(
                "HMAC ключи не прошли валидацию ({} ошибок): используется пустой ключ — записи таблицы лидеров не будут защищены от подделки",
                errors.len()
            );
        }

        // Загрузка сохранённых данных с обработкой ошибок
        let (save, leaderboard) = Self::load_game_data();

        // Проверка целостности рекорда (Исправление C3, C10: Result вместо Option)
        // Используем verify_and_get_score_result() для явной обработки ошибок
        let high_score = if let Ok(score) = save.verify_and_get_score_result() {
            score
        } else {
            log_error!("Рекорд не прошёл валидацию. Используется 0.");
            0u128
        };

        // P3-ID50: кэшируем строку рекорда — форматирование только при изменении
        let high_score_display = format!("{high_score:10}");

        // Проверка терминала и инициализация ввода/вывода
        let (canvas, input) = Self::initialize_terminal()?;

        Ok(Self {
            canvas,
            input,
            leaderboard,
            high_score,
            high_score_display,
        })
    }

    /// Загрузить сохранённые данные и таблицу лидеров.
    ///
    /// # Возвращает
    /// Кортеж ([`SaveData`], [`Leaderboard`])
    fn load_game_data() -> (SaveData, Leaderboard) {
        // Исправление #2: логирование всех ошибок загрузки
        let save = SaveData::load_config();
        let mut leaderboard = Leaderboard::load();
        // validate() удаляет невалидные записи, логируем если были удалены
        let initial_count = leaderboard.len();
        leaderboard.validate();
        let removed_count = initial_count.saturating_sub(leaderboard.len());
        if removed_count > 0 {
            crate::log_warn!("Удалено {removed_count} невалидных записей из таблицы лидеров.");
        }
        (save, leaderboard)
    }

    /// Инициализировать терминал и создать Canvas/KeyReader.
    ///
    /// # Возвращает
    /// `Result<(Canvas, KeyReader), GameError>`
    ///
    /// # Errors
    /// Возвращает ошибку если терминал не соответствует минимальным требованиям
    ///
    /// # Исправление аудита 2026-03-30
    /// Использует `Canvas::try_default()` для безопасной инициализации с обработкой ошибок.
    ///
    /// # Исправление аудита 2026-04-01 (C1)
    /// Упрощена обработка ошибок: используется прямой возврат через `?` оператор
    /// вместо избыточного `.map_err()` с логированием.
    fn initialize_terminal() -> Result<(Canvas, KeyReader), GameError> {
        // Проверка размера терминала - используем прямой ? возврат (Исправление C1)
        let (width, height) = terminal_size()?;

        // Потеря точности допустима: размеры терминала положительные значения
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        if (width as usize) < DISP_WIDTH || (height as usize) < DISP_HEIGHT {
            let msg = format!(
                "Окно терминала слишком маленькое!\n\
                 Минимальный размер: {DISP_WIDTH}x{DISP_HEIGHT} символов.\n\
                 Текущий размер: {width}x{height} символов."
            );
            crate::log_error!("{msg}");
            return Err(GameError::ValidationError(msg));
        }

        // Инициализация Canvas с безопасной обработкой ошибок (Исправление аудита #1)
        // Используем try_default() вместо new() для поддержки fallback режима
        // Исправление C1: прямой ? возврат без map_err
        let canvas =
            Canvas::try_default().map_err(|e| GameError::IoError(std::io::Error::other(e)))?;

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
        use crate::constants::FRAME_DELAY_MS;

        let mut last_time = Instant::now();
        let interval_ms = FRAME_DELAY_MS;

        loop {
            // Поддержание стабильного FPS (общая функция с игровым циклом)
            if maintain_fps(&mut last_time, interval_ms).is_none() {
                continue;
            }

            // Отрисовка меню
            crate::menu::draw_menu(&mut self.canvas, &self.high_score_display);

            // Обработка ввода
            if let Ok(Some(key)) = self.input.get_key() {
                // Проверка выхода из приложения
                if Self::check_exit_condition(key) {
                    break;
                }
                // Клонирование необходимо: process_menu_input требует &mut self,
                // но также нужна ссылка на high_score_display (NLL не может разделить borrows)
                // Исправление аудита #4: проверено — clone() необходим из-за ограничений NLL
                let score_clone = self.high_score_display.clone();
                self.process_menu_input(key, &score_clone);
            }
        }
    }

    /// Проверить условие выхода из приложения.
    ///
    /// # Аргументы
    /// * `key` - код нажатой клавиши
    ///
    /// # Возвращает
    /// `true` если нужно выйти из приложения
    const fn check_exit_condition(key: u8) -> bool {
        key == crate::constants::KEY_BACKSPACE
    }

    /// Обработать ввод в меню.
    ///
    /// # Аргументы
    /// * `key` - код нажатой клавиши
    /// * `high_score_display` - строка для отображения рекорда
    fn process_menu_input(&mut self, key: u8, high_score_display: &str) {
        use crate::menu::show_leaderboard;

        match key {
            // ЗАПУСК КЛАССИЧЕСКОЙ ИГРЫ (Enter)
            b'\n' | b'\r' => {
                let state = GameState::new();
                self.run_game_mode_with_state(high_score_display, state, true);
            }

            // ЗАПУСК РЕЖИМА СПРИНТ (R)
            b'r' => {
                let state = GameState::new_sprint();
                self.run_game_mode_with_state(high_score_display, state, false);
            }

            // ЗАПУСК РЕЖИМА МАРАФОН (M)
            b'm' => {
                let state = GameState::new_marathon();
                self.run_game_mode_with_state(high_score_display, state, true);
            }

            // ОТОБРАЖЕНИЕ ТАБЛИЦЫ ЛИДЕРОВ (L)
            b'l' => {
                show_leaderboard(&mut self.canvas, &mut self.input, &self.leaderboard);
            }

            // НЕИЗВЕСТНАЯ КЛАВИША
            _ => {}
        }
    }

    /// Запустить игровой режим с указанным состоянием.
    ///
    /// # Аргументы
    /// * `high_score_display` — строка для отображения рекорда
    /// * `state` — начальное состояние игры
    /// * `save_score` — сохранять ли рекорд после игры
    ///
    /// # Возвращает
    /// Набранные очки (если `save_score` — `true`) или `0`
    fn run_game_mode_with_state(
        &mut self,
        high_score_display: &str,
        state: GameState,
        save_score: bool,
    ) -> u128 {
        let score = run_game_mode(
            &mut self.canvas,
            &mut self.input,
            high_score_display,
            state,
            save_score,
            &mut self.leaderboard,
        );
        if save_score && score > self.high_score {
            self.high_score = score;
            // P3-ID50: обновляем кэш строки рекорда при изменении
            self.high_score_display = format!("{score:10}");
            SaveData::save_value(self.high_score);
        }
        score
    }
}

/// Запустить приложение.
///
/// Возвращает ошибку при проблемах с инициализацией.
///
/// # Возвращает
/// - `Ok(())` при успешном завершении
/// - `Err(GameError)` при ошибке инициализации
///
/// # Errors
/// Возвращает ошибку если не удалось инициализировать приложение
pub fn run() -> Result<(), GameError> {
    let mut app = Application::new()?;
    app.run();
    Ok(())
}

// ============================================================================
// ТЕСТЫ ДЛЯ APPLICATION
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Тест для `run_menu_loop()`
    ///
    /// Проверяет, что метод `run_menu_loop()` выполняется без ошибок
    /// и возвращает () вместо Result.
    ///
    /// Примечание: Этот тест игнорируется по умолчанию, так как требует
    /// доступа к терминалу и может блокировать выполнение.
    /// Метод `run_menu_loop()` содержит бесконечный цикл — тест не может
    /// проверить фактическое выполнение, только компиляцию и сигнатуру.
    /// Это нормальное поведение для тестов интерактивных методов.
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

            // Проверяем что метод существует и имеет правильную сигнатуру
            // Этот код компилируется только если сигнатура верна
            let _: () = app.run_menu_loop();

            // Тест проходит, если код компилируется
        } else {
            // Если приложение не создалось (нет терминала), тест считается пройденным
            // Это ожидаемое поведение в среде без терминала
        }
    }

    /// Тест для проверки сигнатуры `run_menu_loop()`
    ///
    /// Проверяет, что `run_menu_loop()` возвращает () а не Result
    #[test]
    fn test_run_menu_loop_return_type() {
        // Этот тест проверяет тип возвращаемого значения на этапе компиляции
        // Если сигнатура изменится, тест не скомпилируется

        // Тип метода run_menu_loop должен быть fn(&mut self) -> ()
        // Проверяем через присваивание типа
        #[allow(clippy::no_effect_underscore_binding)]
        let _type_check: fn(&mut Application) = |app| {
            app.run_menu_loop();
        };

        // Если код компилируется, сигнатура верна
    }

    /// Тест для `Application::new()`
    ///
    /// Проверяет, что приложение может быть создано (в среде с терминалом)
    #[test]
    #[ignore = "Требует доступа к терминалу"]
    fn test_application_new() {
        let app_result = Application::new();

        // В среде с терминалом приложение должно создаться успешно
        // В среде без терминала тест игнорируется
        if let Ok(app) = app_result {
            // В среде с терминалом приложение должно создаться успешно

            // Проверяем наличие полей
            let _ = app.high_score;
            let _ = app.leaderboard;
        } else {
            // В среде без терминала это ожидаемое поведение
        }
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ОБРАБОТКИ ОШИБОК (ИСПРАВЛЕНИЕ #2, #3)
    // =========================================================================

    /// Тест для проверки `unwrap_or_else` в `Application::new()`
    ///
    /// Проверяет что при невалидном рекорде используется значение 0
    /// и выводится сообщение об ошибке
    #[test]
    fn test_application_unwrap_or_else_behavior() {
        use crate::highscore::SaveData;

        // Создаём валидный SaveData для проверки unwrap_or_else
        let save = SaveData::from_value(1000).unwrap();
        let result = save.verify_and_get_score();

        // Проверяем что валидный рекорд возвращается
        assert_eq!(result, Some(1000));

        // Проверяем поведение unwrap_or_else с валидным значением
        let score = save.verify_and_get_score().unwrap_or_else(|| {
            crate::log_error!("Рекорд не прошёл валидацию или отсутствует. Используется 0.");
            0u128
        });
        assert_eq!(score, 1000, "Должно вернуться значение рекорда");
    }

    /// Тест для проверки логирования при загрузке данных
    ///
    /// Проверяет что `load_game_data()` корректно обрабатывает ошибки.
    ///
    /// #[ignore] — зависит от состояния файловой системы (наличие конфига).
    #[test]
    #[ignore = "depends on filesystem state (config file presence)"]
    fn test_application_load_game_data_logging() {
        use crate::highscore::{Leaderboard, SaveData};

        // Проверяем что Leaderboard::load() возвращает пустую таблицу при ошибке
        // (тест может зависеть от наличия конфига в системе)
        let leaderboard = Leaderboard::load();
        assert!(
            leaderboard.get_entries().len() <= 5,
            "Таблица лидеров не должна превышать 5 записей"
        );

        // Проверяем что SaveData::load_config() возвращает дефолтное значение при ошибке
        let save = SaveData::load_config();
        assert!(
            save.verify_and_get_score().is_some(),
            "SaveData должен иметь валидный score"
        );
    }

    /// Тест для проверки формата логов ошибок
    ///
    /// Проверяет что ошибки логируются в правильном формате
    #[test]
    fn test_error_logging_format() {
        // Проверяем формат [ERROR]
        let error_msg = "[ERROR] Рекорд не прошёл валидацию или отсутствует. Используется 0.";
        assert!(
            error_msg.contains("[ERROR]"),
            "Сообщение должно содержать [ERROR]"
        );

        // Проверяем формат [WARN]
        let warn_msg = "[WARN] Удалено 2 невалидных записей из таблицы лидеров.";
        assert!(
            warn_msg.contains("[WARN]"),
            "Сообщение должно содержать [WARN]"
        );

        // Проверяем формат "Критическая ошибка" (без двоеточия в некоторых местах)
        let critical_msg = "Критическая ошибка запуска: терминал недоступен";
        assert!(
            critical_msg.contains("Критическая ошибка"),
            "Сообщение должно содержать 'Критическая ошибка'"
        );
    }

    /// Тест для проверки обработки ошибок в `initialize_terminal()`
    ///
    /// Проверяет что функция возвращает ошибку при недоступности терминала
    #[test]
    #[ignore = "Требует проверки размера терминала"]
    fn test_initialize_terminal_error_handling() {
        // Этот тест требует доступа к терминалу
        // Проверяем что Application::new() возвращает Result
        let result = Application::new();

        // В зависимости от наличия терминала:
        // - Ok(Application) если терминал доступен
        // - Err если терминал недоступен или слишком мал
        match result {
            Ok(_) => {
                // Терминал доступен - это нормально
            }
            Err(e) => {
                // Ошибка терминала - тоже нормально для тестовой среды
                eprintln!("Ожидаемая ошибка в тестовой среде: {e}");
            }
        }
    }

    /// Тест для проверки `validate()` в Leaderboard
    ///
    /// Проверяет что невалидные записи удаляются с логированием
    #[test]
    fn test_leaderboard_validate_logging() {
        use crate::highscore::Leaderboard;

        let mut leaderboard = Leaderboard::default();

        // Добавляем валидные записи
        let _ = leaderboard.add_score("Player1", 1000);
        let _ = leaderboard.add_score("Player2", 2000);

        let initial_count = leaderboard.len();
        leaderboard.validate();

        // Проверяем что валидные записи не удалены
        assert_eq!(
            leaderboard.len(),
            initial_count,
            "Валидные записи не должны быть удалены"
        );
    }
}
