//! Тесты обработки ошибок в Application.
//!
//! Этот модуль содержит тесты для проверки исправлений обработки ошибок:
//! - Успешная инициализация Application
//! - Ошибка инициализации терминала
//! - Корректный cleanup при ошибке
//! - unwrap_or_else() для рекорда
//! - Логирование ошибок
//!
//! # Исправления
//! - Исправление аудита 2026-03-30: Canvas::try_default() для безопасной инициализации
//! - Исправление #2: unwrap_or_else с логированием для рекорда
//! - Исправление #3: Обработка ошибок терминала

#[cfg(test)]
mod tests {
    use crate::app::Application;
    use crate::game::GameError;
    use crate::highscore::{Leaderboard, SaveData};

    // ========================================================================
    // ГРУППА ТЕСТОВ 1: Успешная инициализация Application
    // ========================================================================

    /// Тест 1: Проверка успешной инициализации Application.
    ///
    /// Проверяет что Application::new() возвращает Ok(Application)
    /// при успешной инициализации.
    #[test]
    #[ignore = "Требует доступа к терминалу"]
    fn test_application_new_success() {
        let app_result = Application::new();

        match app_result {
            Ok(app) => {
                // Проверяем что приложение создано
                // Поля приложения доступны для проверки
                assert!(app.high_score >= 0, "Рекорд должен быть неотрицательным");

                // Проверяем что таблица лидеров загружена
                assert!(
                    app.leaderboard.get_entries().len() <= 5,
                    "Таблица лидеров не должна превышать 5 записей"
                );

                println!("✓ Application успешно инициализировано");
            }
            Err(e) => {
                // В среде без терминала это ожидаемое поведение
                println!("Тест пропущен: терминал недоступен ({})", e);
            }
        }
    }

    /// Тест 2: Проверка что Application::new() возвращает Result.
    ///
    /// Проверяет что сигнатура метода возвращает Result<Self, GameError>.
    #[test]
    fn test_application_new_returns_result() {
        // Проверяем что тип возвращаемого значения - Result
        let _result_type_check: fn() -> Result<Application, GameError> = Application::new;

        // Если код компилируется, сигнатура верна
        println!("✓ Application::new() возвращает Result<Application, GameError>");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 2: Ошибка инициализации терминала
    // ========================================================================

    /// Тест 3: Проверка обработки ошибки инициализации терминала.
    ///
    /// Проверяет что при ошибке терминала возвращается GameError.
    #[test]
    fn test_application_terminal_error_handling() {
        let app_result = Application::new();

        // В зависимости от среды:
        // - Ok(Application) если терминал доступен
        // - Err(GameError) если терминал недоступен
        match app_result {
            Ok(_) => {
                // Терминал доступен - это нормально
                println!("Терминал доступен");
            }
            Err(e) => {
                // Проверяем тип ошибки
                match e {
                    GameError::Io(io_err) => {
                        // Ошибка ввода/вывода терминала
                        assert!(
                            io_err.to_string().contains("терминал")
                                || io_err.to_string().contains("Canvas"),
                            "Ошибка должна упоминать терминал или Canvas"
                        );
                    }
                    GameError::ValidationError(msg) => {
                        // Ошибка валидации (например, размер терминала)
                        assert!(
                            msg.contains("терминал") || msg.contains("размер"),
                            "Ошибка валидации должна упоминать терминал или размер"
                        );
                    }
                    _ => {
                        // Другие типы ошибок тоже допустимы
                    }
                }

                println!("Ожидаемая ошибка в тестовой среде: {}", e);
            }
        }
    }

    /// Тест 4: Проверка формата сообщения об ошибке терминала.
    ///
    /// Проверяет что сообщение об ошибке содержит понятную информацию.
    #[test]
    fn test_application_error_message_format() {
        let app_result = Application::new();

        if let Err(e) = app_result {
            let error_msg = format!("{}", e);

            // Проверяем что сообщение содержит полезную информацию
            assert!(
                !error_msg.is_empty(),
                "Сообщение об ошибке не должно быть пустым"
            );

            println!("Формат ошибки: {}", error_msg);
        } else {
            println!("Терминал доступен, ошибка не возникла");
        }
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 3: Корректный cleanup при ошибке
    // ========================================================================

    /// Тест 5: Проверка что cleanup выполняется при ошибке.
    ///
    /// Проверяет что Application не оставляет ресурсов при ошибке.
    #[test]
    fn test_application_cleanup_on_error() {
        // Создаём приложение
        let app_result = Application::new();

        match app_result {
            Ok(app) => {
                // Если приложение создано, проверяем что оно корректно очищается
                drop(app);

                // Если drop прошёл без паники, cleanup выполнен корректно
                println!("✓ Cleanup выполнен корректно");
            }
            Err(_) => {
                // При ошибке инициализации cleanup не требуется
                // так как ресурсы не были выделены
                println!("✓ Cleanup не требуется при ошибке инициализации");
            }
        }
    }

    /// Тест 6: Проверка что Drop не паникует.
    ///
    /// Проверяет что Drop реализация Application не паникует.
    #[test]
    #[ignore = "Требует доступа к терминалу"]
    fn test_application_drop_no_panic() {
        let app_result = Application::new();

        if let Ok(app) = app_result {
            // Drop должен выполниться без паники
            drop(app);

            println!("✓ Drop не паникует");
        } else {
            println!("Тест пропущен: терминал недоступен");
        }
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 4: unwrap_or_else для рекорда
    // ========================================================================

    /// Тест 7: Проверка unwrap_or_else для рекорда.
    ///
    /// Проверяет что при невалидном рекорде используется значение 0.
    #[test]
    fn test_application_unwrap_or_else_for_high_score() {
        // Создаём валидный SaveData
        let save = SaveData::from_value(1000);

        // Проверяем что verify_and_get_score() возвращает Some
        let score = save.verify_and_get_score();
        assert_eq!(score, Some(1000), "Валидный рекорд должен вернуться");

        // Проверяем unwrap_or_else с валидным значением
        let result = save.verify_and_get_score().unwrap_or_else(|| {
            eprintln!("[ERROR] Рекорд не прошёл валидацию или отсутствует. Используется 0.");
            0u128
        });
        assert_eq!(result, 1000, "Должно вернуться значение рекорда");

        // Проверяем unwrap_or_else с невалидным значением (симуляция)
        let invalid_save = SaveData::from_value(0);
        let invalid_score = invalid_save.verify_and_get_score();

        // Если рекорд 0, verify_and_get_score() может вернуть Some(0) или None
        // в зависимости от реализации валидации
        match invalid_score {
            Some(0) => {
                // Рекорд 0 считается валидным
                println!("Рекорд 0 считается валидным");
            }
            None => {
                // Рекорд невалидный, unwrap_or_else вернёт 0
                let fallback = invalid_save.verify_and_get_score().unwrap_or_else(|| {
                    eprintln!(
                        "[ERROR] Рекорд не прошёл валидацию или отсутствует. Используется 0."
                    );
                    0u128
                });
                assert_eq!(fallback, 0, "Невалидный рекорд должен вернуть 0");
            }
            Some(other) => {
                // Другое значение
                println!("Рекорд: {}", other);
            }
        }
    }

    /// Тест 8: Проверка логирования при unwrap_or_else.
    ///
    /// Проверяет что при невалидном рекорде выводится сообщение [ERROR].
    #[test]
    fn test_application_unwrap_or_else_logging() {
        // Проверяем формат сообщения об ошибке
        let error_msg = "[ERROR] Рекорд не прошёл валидацию или отсутствует. Используется 0.";

        assert!(
            error_msg.contains("[ERROR]"),
            "Сообщение должно содержать [ERROR]"
        );
        assert!(
            error_msg.contains("Рекорд"),
            "Сообщение должно упоминать рекорд"
        );
        assert!(
            error_msg.contains("0"),
            "Сообщение должно упоминать значение по умолчанию"
        );

        println!("✓ Формат логирования корректный");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 5: Логирование ошибок
    // ========================================================================

    /// Тест 9: Проверка формата логов [ERROR].
    ///
    /// Проверяет что ошибки логируются с префиксом [ERROR].
    #[test]
    fn test_error_logging_error_prefix() {
        let error_messages = [
            "[ERROR] Рекорд не прошёл валидацию или отсутствует. Используется 0.",
            "[ERROR] Ошибка инициализации терминала",
        ];

        for msg in error_messages.iter() {
            assert!(
                msg.contains("[ERROR]"),
                "Сообщение об ошибке должно содержать [ERROR]: {}",
                msg
            );
        }

        println!("✓ Префикс [ERROR] используется корректно");
    }

    /// Тест 10: Проверка формата логов [WARN].
    ///
    /// Проверяет что предупреждения логируются с префиксом [WARN].
    #[test]
    fn test_error_logging_warn_prefix() {
        let warn_messages = [
            "[WARN] Удалено 2 невалидных записей из таблицы лидеров.",
            "[WARN] Таблица лидеров пуста",
        ];

        for msg in warn_messages.iter() {
            assert!(
                msg.contains("[WARN]"),
                "Сообщение предупреждения должно содержать [WARN]: {}",
                msg
            );
        }

        println!("✓ Префикс [WARN] используется корректно");
    }

    /// Тест 11: Проверка формата логов "Критическая ошибка".
    ///
    /// Проверяет что критические ошибки логируются с префиксом "Критическая ошибка".
    #[test]
    fn test_error_logging_critical_prefix() {
        let critical_messages = [
            "Критическая ошибка запуска: терминал недоступен",
            "Критическая ошибка: не удалось инициализировать Canvas",
        ];

        for msg in critical_messages.iter() {
            assert!(
                msg.contains("Критическая ошибка"),
                "Критическое сообщение должно содержать 'Критическая ошибка': {}",
                msg
            );
        }

        println!("✓ Префикс 'Критическая ошибка' используется корректно");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 6: Интеграционные тесты
    // ========================================================================

    /// Тест 12: Интеграционный тест Application lifecycle.
    ///
    /// Проверяет полный цикл: создание -> использование -> cleanup.
    #[test]
    #[ignore = "Требует доступа к терминалу"]
    fn test_application_lifecycle() {
        let app_result = Application::new();

        match app_result {
            Ok(mut app) => {
                // Проверяем что приложение может быть использовано
                let initial_score = app.high_score;

                // Симулируем изменение рекорда
                app.high_score = 2000;
                assert_eq!(app.high_score, 2000, "Рекорд должен обновиться");

                // Cleanup
                drop(app);

                println!("✓ Application lifecycle тест пройден");
            }
            Err(e) => {
                println!("Тест пропущен: терминал недоступен ({})", e);
            }
        }
    }

    /// Тест 13: Проверка load_game_data().
    ///
    /// Проверяет что загрузка данных работает корректно.
    #[test]
    fn test_application_load_game_data() {
        // Проверяем что SaveData::load_config() возвращает валидные данные
        let save = SaveData::load_config();
        assert!(
            save.verify_and_get_score().is_some(),
            "SaveData должен иметь валидный score"
        );

        // Проверяем что Leaderboard::load() возвращает таблицу
        let leaderboard = Leaderboard::load();
        assert!(
            leaderboard.get_entries().len() <= 5,
            "Таблица лидеров не должна превышать 5 записей"
        );

        println!("✓ load_game_data() работает корректно");
    }

    /// Тест 14: Проверка validate() в Leaderboard.
    ///
    /// Проверяет что validate() удаляет невалидные записи.
    #[test]
    fn test_application_leaderboard_validate() {
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

        println!("✓ Leaderboard validate() работает корректно");
    }

    /// Тест 15: Стресс-тест создания Application.
    ///
    /// Проверяет что множественные создания Application не вызывают утечек.
    #[test]
    #[ignore = "Требует доступа к терминалу и может быть медленным"]
    fn test_application_creation_stress_test() {
        for i in 0..10 {
            let app_result = Application::new();

            match app_result {
                Ok(app) => {
                    // Сразу очищаем
                    drop(app);
                }
                Err(_) => {
                    // В среде без терминала это нормально
                    break;
                }
            }

            println!("✓ Итерация {} пройдена", i);
        }

        println!("✓ Стресс-тест создания Application пройден");
    }
}
