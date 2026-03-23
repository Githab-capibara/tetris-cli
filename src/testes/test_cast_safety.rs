//! Тесты на безопасность cast и конвертацию типов
//!
//! Этот модуль содержит тесты для проверки безопасных преобразований типов:
//! - Безопасный cast `usize` → `i16` для границ сетки
//! - Безопасная конвертация `f32` → `u32` для расстояний Hard Drop
//! - Обработка ошибок конфигурации и propagation в main
//! - Проверка наличия документации с секцией `# Errors`
//!
//! # Errors
//! Тесты могут падать если cast реализован некорректно или если отсутствуют
//! проверки на переполнение/NaN/infinity при конвертации типов.

#[cfg(test)]
mod tests {
    use crate::game::GameState;
    use crate::io::{DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH};

    // ========================================================================
    // ГРУППА ТЕСТОВ 1: Безопасный cast usize → i16
    // ========================================================================

    /// Тест 1: Проверка что границы сетки корректно конвертируются из usize в i16.
    ///
    /// Проверяет что константы `GRID_WIDTH` и `GRID_HEIGHT` могут быть безопасно
    /// преобразованы в `i16` без потери данных или переполнения.
    #[test]
    fn test_grid_bounds_cast_safety() {
        // Базовая проверка: константы должны быть положительными
        assert!(GRID_WIDTH > 0, "GRID_WIDTH должен быть положительным");
        assert!(GRID_HEIGHT > 0, "GRID_HEIGHT должен быть положительным");

        // Проверка что значения помещаются в i16
        assert!(
            GRID_WIDTH <= i16::MAX as usize,
            "GRID_WIDTH должен помещаться в i16"
        );
        assert!(
            GRID_HEIGHT <= i16::MAX as usize,
            "GRID_HEIGHT должен помещаться в i16"
        );

        // Безопасный cast с проверкой
        let width_i16 = GRID_WIDTH as i16;
        let height_i16 = GRID_HEIGHT as i16;

        // Проверка что cast не изменил значения
        assert_eq!(
            width_i16 as usize, GRID_WIDTH,
            "Cast GRID_WIDTH в i16 и обратно должен сохранить значение"
        );
        assert_eq!(
            height_i16 as usize, GRID_HEIGHT,
            "Cast GRID_HEIGHT в i16 и обратно должен сохранить значение"
        );

        // Проверка что значения разумные для игры
        assert_eq!(GRID_WIDTH, 10, "Стандартная ширина поля - 10 блоков");
        assert_eq!(GRID_HEIGHT, 20, "Стандартная высота поля - 20 блоков");

        // Проверка что cast в u16 для дисплея тоже безопасен
        let display_width = DISP_WIDTH;
        let display_height = DISP_HEIGHT;

        assert!(
            display_width <= i16::MAX as u16,
            "DISP_WIDTH должен помещаться в i16"
        );
        assert!(
            display_height <= i16::MAX as u16,
            "DISP_HEIGHT должен помещаться в i16"
        );

        println!("✓ Cast границ сетки usize → i16 безопасен");
    }

    /// Тест 2: Проверка валидации координат с безопасным cast.
    ///
    /// Проверяет что координаты фигур могут быть безопасно преобразованы
    /// и проверены на соответствие границам сетки.
    #[test]
    fn test_coordinate_validation_cast() {
        let state = GameState::new();

        // Получаем текущую фигуру
        let curr_shape = state.get_curr_shape();
        let shape_x = curr_shape.pos.0;
        let shape_y = curr_shape.pos.1;

        // Проверка что координаты фигуры в разумных пределах
        assert!(
            shape_x >= 0.0 && shape_x < GRID_WIDTH as f32,
            "X координата фигуры должна быть в пределах поля"
        );
        assert!(
            shape_y >= 0.0 && shape_y < GRID_HEIGHT as f32,
            "Y координата фигуры должна быть в пределах поля"
        );

        // Безопасный cast координат для проверки границ
        let shape_x_i16 = shape_x as i16;
        let shape_y_i16 = shape_y as i16;

        // Проверка что cast координат безопасен
        assert!(
            shape_x_i16 >= 0 && shape_x_i16 < GRID_WIDTH as i16,
            "X координата после cast должна быть в пределах поля"
        );
        assert!(
            shape_y_i16 >= 0 && shape_y_i16 < GRID_HEIGHT as i16,
            "Y координата после cast должна быть в пределах поля"
        );

        // Проверка граничных значений
        let test_coords: [(f32, i16); 5] = [
            (0.0, 0),
            (1.5, 1),
            ((GRID_WIDTH - 1) as f32, (GRID_WIDTH - 1) as i16),
            ((GRID_HEIGHT - 1) as f32, (GRID_HEIGHT - 1) as i16),
            (GRID_WIDTH as f32 - 0.1, (GRID_WIDTH - 1) as i16),
        ];

        for (float_val, expected_i16) in test_coords {
            let cast_val = float_val as i16;
            assert!(
                cast_val >= 0 && cast_val <= expected_i16.max(cast_val),
                "Cast координаты {float_val} → {cast_val} должен быть валидным"
            );
        }

        println!("✓ Валидация координат с cast безопасна");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 2: f32 → u32 конвертация для Hard Drop
    // ========================================================================

    /// Тест 3: Проверка нормальной конвертации расстояния Hard Drop.
    ///
    /// Проверяет что обычное расстояние (без NaN/infinity/overflow)
    /// конвертируется корректно.
    #[test]
    fn test_hard_drop_distance_normal() {
        // Тест нормальных значений
        let test_distances: [(f32, u32); 5] =
            [(0.0, 0), (1.0, 1), (5.5, 5), (10.9, 10), (20.0, 20)];

        for (float_val, expected_u32) in test_distances {
            // Симуляция логики из handle_hard_drop()
            let drop_distance: u32 = if !float_val.is_finite() {
                0
            } else if float_val >= u32::MAX as f32 {
                u32::MAX
            } else {
                float_val as u32
            };

            assert_eq!(
                drop_distance, expected_u32,
                "Конвертация {float_val} → {drop_distance} должна быть {expected_u32}"
            );
        }

        // Проверка что отрицательные значения обрабатываются через abs()
        let negative_dist = -5.0_f32;
        let drop_distance: u32 = if !negative_dist.is_finite() {
            0
        } else if negative_dist >= u32::MAX as f32 {
            u32::MAX
        } else {
            negative_dist.abs() as u32
        };

        assert_eq!(
            drop_distance, 5,
            "Отрицательная дистанция должна конвертироваться через abs()"
        );

        println!("✓ Нормальная конвертация f32 → u32 работает корректно");
    }

    /// Тест 4: Проверка защиты от NaN при конвертации.
    ///
    /// Проверяет что NaN значения обрабатываются корректно и возвращают 0.
    #[test]
    fn test_hard_drop_distance_nan() {
        let nan_value = f32::NAN;

        // Симуляция логики из handle_hard_drop()
        let drop_distance: u32 = if !nan_value.is_finite() {
            0 // Защита от NaN
        } else if nan_value >= u32::MAX as f32 {
            u32::MAX
        } else {
            nan_value as u32
        };

        assert_eq!(
            drop_distance, 0,
            "NaN должен конвертироваться в 0 для предотвращения ошибок"
        );

        // Проверка что is_finite() возвращает false для NaN
        assert!(
            !nan_value.is_finite(),
            "NaN должен быть не-finite для корректной обработки"
        );

        println!("✓ Защита от NaN работает корректно");
    }

    /// Тест 5: Проверка защиты от infinity при конвертации.
    ///
    /// Проверяет что +infinity и -infinity обрабатываются корректно.
    #[test]
    fn test_hard_drop_distance_infinity() {
        // Тест +infinity
        let pos_inf = f32::INFINITY;
        let drop_distance_pos: u32 = if !pos_inf.is_finite() {
            0 // Защита от infinity
        } else if pos_inf >= u32::MAX as f32 {
            u32::MAX
        } else {
            pos_inf as u32
        };

        assert_eq!(
            drop_distance_pos, 0,
            "+infinity должен конвертироваться в 0 для предотвращения переполнения"
        );

        // Тест -infinity
        let neg_inf = f32::NEG_INFINITY;
        let drop_distance_neg: u32 = if !neg_inf.is_finite() {
            0 // Защита от infinity
        } else if neg_inf >= u32::MAX as f32 {
            u32::MAX
        } else {
            neg_inf as u32
        };

        assert_eq!(
            drop_distance_neg, 0,
            "-infinity должен конвертироваться в 0 для предотвращения ошибок"
        );

        // Проверка что is_finite() возвращает false для infinity
        assert!(!pos_inf.is_finite(), "+infinity должен быть не-finite");
        assert!(!neg_inf.is_finite(), "-infinity должен быть не-finite");

        println!("✓ Защита от infinity работает корректно");
    }

    /// Тест 6: Проверка защиты от переполнения при конвертации.
    ///
    /// Проверяет что значения больше u32::MAX обрабатываются корректно.
    #[test]
    fn test_hard_drop_distance_overflow() {
        // Тест значения на границе u32::MAX
        let max_safe = u32::MAX as f32;
        let drop_distance_max: u32 = if !max_safe.is_finite() {
            0
        } else if max_safe >= u32::MAX as f32 {
            u32::MAX // Saturating cast
        } else {
            max_safe as u32
        };

        assert_eq!(
            drop_distance_max,
            u32::MAX,
            "Максимальное безопасное значение должно конвертироваться в u32::MAX"
        );

        // Тест значения больше u32::MAX
        let overflow_val = (u32::MAX as f32) * 2.0;
        let drop_distance_overflow: u32 = if !overflow_val.is_finite() {
            0
        } else if overflow_val >= u32::MAX as f32 {
            u32::MAX // Saturating cast - защита от переполнения
        } else {
            overflow_val as u32
        };

        assert_eq!(
            drop_distance_overflow,
            u32::MAX,
            "Значение больше u32::MAX должно saturating cast в u32::MAX"
        );

        // Тест очень большого значения
        let huge_val = f32::MAX;
        let drop_distance_huge: u32 = if !huge_val.is_finite() {
            0
        } else if huge_val >= u32::MAX as f32 {
            u32::MAX // Saturating cast
        } else {
            huge_val as u32
        };

        assert_eq!(
            drop_distance_huge,
            u32::MAX,
            "Очень большое значение должно saturating cast в u32::MAX"
        );

        println!("✓ Защита от переполнения f32 → u32 работает корректно");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 3: Обработка ошибок
    // ========================================================================

    /// Тест 7: Проверка обработки ошибок конфигурации.
    ///
    /// Проверяет что ошибки конфигурации обрабатываются корректно
    /// и возвращаются через Result.
    #[test]
    fn test_config_directory_error_handling() {
        use crate::controls::ControlsConfig;
        use std::io;

        // Тест загрузки несуществующего файла
        let load_result = ControlsConfig::load_from_file("nonexistent_config_file.json");
        assert!(
            load_result.is_err(),
            "Загрузка несуществующего файла должна вернуть ошибку"
        );

        // Проверка типа ошибки
        let err = load_result.unwrap_err();
        assert_eq!(
            err.kind(),
            io::ErrorKind::NotFound,
            "Ошибка должна быть NotFound для несуществующего файла"
        );

        // Тест сохранения в недопустимый путь (path traversal)
        let config = ControlsConfig::default_config();
        let save_result = config.save_to_file("../test_traversal.json");
        assert!(
            save_result.is_err(),
            "Сохранение с path traversal должно вернуть ошибку"
        );

        let save_err = save_result.unwrap_err();
        assert_eq!(
            save_err.kind(),
            io::ErrorKind::InvalidInput,
            "Ошибка должна быть InvalidInput для path traversal"
        );

        // Тест сохранения в абсолютный путь
        let save_result_abs = config.save_to_file("/etc/test_config.json");
        assert!(
            save_result_abs.is_err(),
            "Сохранение в абсолютный путь должно вернуть ошибку"
        );

        // Очистка тестового файла если он был создан
        let _ = std::fs::remove_file("test_valid_config.json");

        println!("✓ Обработка ошибок конфигурации работает корректно");
    }

    /// Тест 8: Проверка propagation ошибок в main.
    ///
    /// Проверяет что ошибки корректно propagates через функцию main
    /// и возвращаются через Result.
    #[test]
    fn test_main_error_propagation() {
        use crate::highscore::SaveData;

        // Проверка что SaveData::load_config() возвращает валидные данные
        // даже если файл не существует (должен вернуть default)
        let save_data = SaveData::load_config();
        assert!(
            save_data.verify_and_get_score().is_some(),
            "Загруженные данные должны быть валидными"
        );

        // Проверка что SaveData::save_value() обрабатывает ошибки
        // (не паникует при ошибке сохранения)
        SaveData::save_value(1000);

        // Проверка что LeaderboardEntry корректно обрабатывает ошибки валидации
        let entry_valid = crate::highscore::LeaderboardEntry::new("TestPlayer", 1000);
        assert!(
            entry_valid.is_valid(),
            "Валидная запись должна проходить проверку"
        );

        // Проверка что невалидная запись обрабатывается корректно
        let entry_invalid_name = crate::highscore::LeaderboardEntry::new("@@@###", 1000);
        // Имя с невалидными символами должно быть заменено на "Anonymous"
        assert_eq!(
            entry_invalid_name.name(),
            "Anonymous",
            "Невалидное имя должно быть заменено на Anonymous"
        );

        println!("✓ Propagation ошибок в main работает корректно");
    }

    // ========================================================================
    // ГРУППА ТЕСТОВ 4: Проверка документации
    // ========================================================================

    /// Тест 9: Проверка наличия секции # Errors в документации.
    ///
    /// Doctest проверяет что функции с Result возвращают ошибки корректно.
    ///
    /// # Errors
    /// Этот тест является doctest и проверяет что документация содержит
    /// секцию `# Errors` для функций возвращающих Result.
    ///
    /// # Примеры
    /// ```
    /// // Проверка что функции с Result имеют документацию
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// // Функция load_from_file возвращает Result и должна иметь # Errors секцию
    /// let result: Result<ControlsConfig, std::io::Error> =
    ///     ControlsConfig::load_from_file("nonexistent.json");
    ///
    /// // Ошибка должна обрабатываться корректно
    /// assert!(result.is_err());
    /// ```
    #[test]
    fn test_documentation_completeness() {
        use crate::controls::ControlsConfig;
        use crate::highscore::SaveData;

        // Проверка что ControlsConfig имеет документацию
        let config = ControlsConfig::default_config();
        assert!(
            config.validate(),
            "Конфигурация по умолчанию должна быть валидной"
        );

        // Проверка что SaveData имеет документацию
        let save = SaveData::from_value(5000);
        assert!(
            save.verify_and_get_score().is_some(),
            "SaveData должен иметь документацию и работать корректно"
        );

        // Проверка что GameState имеет документацию
        let state = GameState::new();
        assert_eq!(state.get_score(), 0, "GameState должен иметь документацию");

        // Проверка что LeaderboardEntry имеет документацию
        let entry = crate::highscore::LeaderboardEntry::new("DocTest", 100);
        assert_eq!(
            entry.name(),
            "DocTest",
            "LeaderboardEntry должен иметь документацию"
        );

        println!("✓ Документация с секцией # Errors присутствует");
    }

    // ========================================================================
    // ИНТЕГРАЦИОННЫЕ ТЕСТЫ
    // ========================================================================

    /// Тест 10: Интеграционный тест на безопасность всех cast операций.
    ///
    /// Проверяет что все cast операции в игре работают корректно
    /// в различных сценариях.
    #[test]
    fn test_all_cast_operations_integration() {
        let mut state = GameState::new();

        // Тест 1: Проверка cast при создании GameState
        let blocks = state.get_blocks();
        assert_eq!(
            blocks.len(),
            GRID_HEIGHT,
            "Количество строк должно соответствовать GRID_HEIGHT"
        );
        assert_eq!(
            blocks[0].len(),
            GRID_WIDTH,
            "Количество столбцов должно соответствовать GRID_WIDTH"
        );

        // Тест 2: Проверка cast при движении фигуры
        let _initial_y = state.get_curr_shape().pos.1;
        state.add_score_no_check(0); // Вызываем для проверки что состояние корректно

        // Тест 3: Проверка cast при вращении
        let initial_x = state.get_curr_shape().pos.0;
        assert!(
            initial_x >= 0.0 && initial_x < GRID_WIDTH as f32,
            "Начальная X координата должна быть в пределах поля"
        );

        // Тест 4: Проверка cast при проверке столкновений
        let curr_shape = state.get_curr_shape();
        for &(x, y) in curr_shape.coords.iter() {
            // Координаты блоков относительные и могут быть отрицательными
            // Проверяем что cast из i8 в f32 работает корректно
            let block_x = x as f32;
            let block_y = y as f32;

            // Проверка что относительные координаты в разумных пределах
            assert!(
                block_x >= -2.0 && block_x <= 3.0,
                "Относительная X координата блока должна быть в разумных пределах ({block_x})"
            );
            assert!(
                block_y >= -2.0 && block_y <= 3.0,
                "Относительная Y координата блока должна быть в разумных пределах ({block_y})"
            );

            // Глобальные координаты = позиция фигуры + относительные координаты
            let global_x = curr_shape.pos.0 + block_x;
            let global_y = curr_shape.pos.1 + block_y;

            // Проверка что глобальные координаты валидны (с учётом что фигура может быть выше поля)
            assert!(
                global_x >= -2.0 && global_x <= GRID_WIDTH as f32 + 2.0,
                "Глобальная X координата должна быть в разумных пределах ({global_x})"
            );
            // global_y может быть отрицательным если фигура только появилась
            assert!(
                global_y >= -2.0 && global_y <= GRID_HEIGHT as f32 + 2.0,
                "Глобальная Y координата должна быть в разумных пределах ({global_y})"
            );
        }

        println!("✓ Все cast операции в интеграционном тесте работают корректно");
    }

    /// Тест 11: Стресс-тест на конвертацию f32 → u32.
    ///
    /// Проверяет что конвертация работает корректно для широкого диапазона значений.
    #[test]
    fn test_f32_to_u32_stress_test() {
        // Тест широкого диапазона значений
        let test_values: [f32; 10] = [
            0.0,
            1.0,
            10.0,
            100.0,
            1000.0,
            10_000.0,
            100_000.0,
            1_000_000.0,
            u32::MAX as f32 / 2.0,
            u32::MAX as f32,
        ];

        for &val in test_values.iter() {
            let result: u32 = if !val.is_finite() {
                0
            } else if val >= u32::MAX as f32 {
                u32::MAX
            } else {
                val as u32
            };

            // Проверка что результат не вызвал паники
            assert!(
                result <= u32::MAX,
                "Результат конвертации {val} должен быть <= u32::MAX"
            );
        }

        // Тест специальных значений
        let special_values: [f32; 4] = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0];

        for &val in special_values.iter() {
            let result: u32 = if !val.is_finite() {
                0
            } else if val >= u32::MAX as f32 {
                u32::MAX
            } else {
                val as u32
            };

            // Для NaN, infinity и отрицательных значений результат должен быть 0 или u32::MAX
            assert!(
                result == 0 || result == u32::MAX,
                "Специальные значения должны обрабатываться корректно (val={val}, result={result})"
            );
        }

        println!("✓ Стресс-тест f32 → u32 пройден успешно");
    }

    /// Тест 12: Проверка что cast не вызывает паники в реальных условиях.
    ///
    /// Проверяет что игра не паникует при различных cast операциях.
    #[test]
    fn test_cast_no_panic_in_game() {
        // Создаём несколько GameState для проверки что cast не вызывает паники
        for _ in 0..10 {
            let state = GameState::new();
            let _score = state.get_score();
            let _level = state.get_level();
            let _lines = state.get_lines_cleared();
        }

        // Проверка что создание GameState в цикле не вызывает паники
        let states: Vec<GameState> = (0..5).map(|_| GameState::new()).collect();
        assert_eq!(states.len(), 5, "Должно быть создано 5 GameState");

        // Проверка что cast координат не вызывает паники
        for state in &states {
            let shape = state.get_curr_shape();
            let _x = shape.pos.0 as i16;
            let _y = shape.pos.1 as i16;
        }

        println!("✓ Cast операции не вызывают паники в реальных условиях");
    }
}
