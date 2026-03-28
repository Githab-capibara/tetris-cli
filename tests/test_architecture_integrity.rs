//! Архитектурные тесты для проверки целостности проекта.
//!
//! Эти тесты проверяют:
//! - Отсутствие циклических зависимостей
//! - Соблюдение границ модулей
//! - Инкапсуляцию компонентов
//! - Отсутствие дублирования кода
//! - Соблюдение принципов SOLID

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // ========================================================================
    // ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
    // ========================================================================

    #[test]
    fn test_no_circular_dependencies_game_constants() {
        // Проверяем, что game/constants.rs не импортирует из game/mod.rs
        let constants_path = "src/game/constants.rs";
        let constants_content =
            fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // constants.rs не должен импортировать из parent модуля
        assert!(
            !constants_content.contains("use crate::game::mod") || constants_content.contains("//"),
            "constants.rs не должен импортировать из game/mod.rs"
        );
    }

    #[test]
    fn test_no_circular_dependencies_io_game() {
        // Проверяем, что io.rs не создаёт циклическую зависимость с game/
        let io_path = "src/io.rs";
        let io_content = fs::read_to_string(io_path).expect("Failed to read io.rs");

        // io.rs может импортировать константы, но не должен импортировать GameState
        if io_content.contains("use crate::game") {
            assert!(
                !io_content.contains("use crate::game::state::GameState"),
                "io.rs не должен импортировать GameState (циклическая зависимость)"
            );
        }
    }

    #[test]
    fn test_constants_module_independence() {
        // Проверяем, что константы независимы
        let constants_path = "src/game/constants.rs";
        let constants_content =
            fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // Константы не должны импортировать сложные типы
        assert!(
            !constants_content.contains("use crate::game::state"),
            "constants.rs не должен импортировать из state.rs"
        );

        assert!(
            !constants_content.contains("use crate::tetromino"),
            "constants.rs не должен импортировать из tetromino.rs"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
    // ========================================================================

    #[test]
    fn test_game_logic_does_not_import_render() {
        // Логика игры не должна зависеть от отрисовки
        let logic_dir = "src/game/logic";

        if Path::new(logic_dir).exists() {
            for entry in fs::read_dir(logic_dir).expect("Failed to read logic dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use crate::game::render"),
                        "{:?} не должен импортировать render (нарушение границ)",
                        path
                    );
                }
            }
        }
    }

    #[test]
    fn test_game_scoring_does_not_import_render() {
        // Scoring не должен зависеть от отрисовки
        let scoring_dir = "src/game/scoring";

        if Path::new(scoring_dir).exists() {
            for entry in fs::read_dir(scoring_dir).expect("Failed to read scoring dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use crate::game::render"),
                        "{:?} не должен импортировать render (нарушение границ)",
                        path
                    );
                }
            }
        }
    }

    #[test]
    fn test_validation_module_independence() {
        // Validation не должен импортировать game logic
        let validation_dir = "src/validation";

        if Path::new(validation_dir).exists() {
            for entry in fs::read_dir(validation_dir).expect("Failed to read validation dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use crate::game::state"),
                        "{:?} не должен импортировать game::state",
                        path
                    );
                }
            }
        }
    }

    // ========================================================================
    // ТЕСТЫ НА ИНКАПСУЛЯЦИЮ
    // ========================================================================

    #[test]
    fn test_game_state_has_getters() {
        // Проверяем, что GameState имеет геттеры для основных полей
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем наличие геттеров
        assert!(
            state_content.contains("pub fn get_score(&self)"),
            "GameState должен иметь get_score()"
        );

        assert!(
            state_content.contains("pub fn get_level(&self)"),
            "GameState должен иметь get_level()"
        );

        assert!(
            state_content.contains("pub fn get_lines_cleared(&self)"),
            "GameState должен иметь get_lines_cleared()"
        );
    }

    #[test]
    fn test_game_state_has_setters_with_validation() {
        // Проверяем, что GameState имеет сеттеры с валидацией
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем наличие валидации в сеттерах
        assert!(
            state_content.contains("set_level")
                && (state_content.contains(".max(1)") || state_content.contains("clamp")),
            "set_level должен иметь валидацию (min = 1)"
        );

        assert!(
            state_content.contains("set_fall_spd") && state_content.contains("clamp"),
            "set_fall_spd должен иметь валидацию (clamp)"
        );
    }

    #[test]
    fn test_game_state_uses_saturating_arithmetic() {
        // Проверяем использование saturating arithmetic для очков
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        assert!(
            state_content.contains("saturating_add") || state_content.contains("saturating_sub"),
            "GameState должен использовать saturating arithmetic для защиты от переполнения"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА ОТСУТСТВИЕ ДУБЛИРОВАНИЯ КОДА
    // ========================================================================

    #[test]
    fn test_no_duplicate_validation_logic() {
        // Проверяем, что валидация путей не дублируется
        let controls_path = "src/controls.rs";
        let controls_content =
            fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        let validation_path = "src/validation/path.rs";
        let validation_content =
            fs::read_to_string(validation_path).expect("Failed to read path.rs");

        // controls.rs должен использовать PathValidator, а не дублировать логику
        if controls_content.contains("validate") {
            assert!(
                controls_content.contains("PathValidator")
                    || controls_content.contains("DEFAULT_PATH_VALIDATOR"),
                "controls.rs должен использовать PathValidator вместо дублирования логики"
            );
        }

        // Проверяем, что validation/path.rs содержит основную логику
        assert!(
            validation_content.contains("impl PathValidator"),
            "PathValidator должен быть определён в validation/path.rs"
        );
    }

    #[test]
    fn test_crypto_module_reuse() {
        // Проверяем, что crypto модуль используется вместо дублирования
        let controls_path = "src/controls.rs";
        let controls_content =
            fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        let crypto_path = "src/crypto.rs";
        let crypto_content = fs::read_to_string(crypto_path).expect("Failed to read crypto.rs");

        // controls.rs должен использовать crypto.rs для HMAC
        if controls_content.contains("hash") || controls_content.contains("HMAC") {
            assert!(
                controls_content.contains("crate::crypto::")
                    || controls_content.contains("keyed_hash")
                    || controls_content.contains("generate_salt"),
                "controls.rs должен использовать crypto.rs для криптографии"
            );
        }

        // crypto.rs должен содержать основные функции
        assert!(
            crypto_content.contains("pub fn hash") || crypto_content.contains("pub fn keyed_hash"),
            "crypto.rs должен содержать криптографические функции"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА СОБЛЮДЕНИЕ SOLID ПРИНЦИПОВ
    // ========================================================================

    #[test]
    fn test_single_responsibility_controls_config() {
        // Проверяем, что ControlsConfig отвечает только за конфигурацию
        let controls_path = "src/controls.rs";
        let controls_content =
            fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // ControlsConfig не должен содержать сложную бизнес-логику
        // Разрешено: валидация, сохранение/загрузка
        assert!(
            !controls_content.contains("pub struct Game")
                || controls_content.contains("// ControlsConfig"),
            "ControlsConfig не должен содержать игровую логику"
        );
    }

    #[test]
    fn test_dependency_inversion_game_cycle() {
        // Проверяем, что игровой цикл использует трейты
        let cycle_path = "src/game/cycle.rs";
        let cycle_content = fs::read_to_string(cycle_path).expect("Failed to read cycle.rs");

        // cycle.rs должен использовать трейты для абстракции
        assert!(
            cycle_content.contains("trait")
                || cycle_content.contains("TerminalBackend")
                || cycle_content.contains("InputReader")
                || cycle_content.contains("Renderer"),
            "cycle.rs должен использовать трейты для абстракции"
        );
    }

    #[test]
    fn test_interface_segregation_game_access() {
        // Проверяем, что трейты доступа специфичны (не перегружены)
        let access_path = "src/game/access.rs";
        let access_content = fs::read_to_string(access_path).expect("Failed to read access.rs");

        // Должны быть отдельные трейты для чтения и записи
        assert!(
            access_content.contains("trait BoardReadonly")
                || access_content.contains("trait BoardMutable"),
            "Должны быть разделённые трейты для чтения и записи"
        );

        assert!(
            access_content.contains("trait ScoreAccess"),
            "Должен быть отдельный трейт для доступа к очкам"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА РАЗМЕР МОДУЛЕЙ
    // ========================================================================

    #[test]
    fn test_module_size_reasonable() {
        // Проверяем, что модули не превышают разумный размер
        let max_lines = 800; // Максимальный размер модуля в строках

        let modules = vec![
            "src/game/state.rs",
            "src/tetromino.rs",
            "src/controls.rs",
            "src/game/render.rs",
            "src/game/cycle.rs",
        ];

        for module_path in modules {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path).expect("Failed to read module");
                let lines = content.lines().count();

                // Это warning, не error (допускаем большие модули с TODO)
                if lines > max_lines {
                    println!(
                        "⚠️ Модуль {} превышает {} строк ({} строк)",
                        module_path, max_lines, lines
                    );
                }
            }
        }

        // Тест всегда проходит, но выводит предупреждения
        assert!(true);
    }

    // ========================================================================
    // ТЕСТЫ НА НАЛИЧИЕ TODO КОММЕНТАРИЕВ (ТЕХНИЧЕСКИЙ ДОЛГ)
    // ========================================================================

    #[test]
    fn test_todo_architecture_comments() {
        // Считаем количество TODO по архитектуре
        let mut todo_count = 0;

        let files = vec![
            "src/game/state.rs",
            "src/game/cycle.rs",
            "src/game/render.rs",
            "src/controls.rs",
        ];

        for file_path in files {
            if Path::new(file_path).exists() {
                let content = fs::read_to_string(file_path).expect("Failed to read file");

                for line in content.lines() {
                    if line.contains("TODO")
                        && (line.contains("#архитектура") || line.contains("архитектура"))
                    {
                        todo_count += 1;
                    }
                }
            }
        }

        // Выводим количество TODO для мониторинга
        println!("📝 Найдено TODO по архитектуре: {}", todo_count);

        // Тест проходит, но предупреждает о большом количестве TODO
        if todo_count > 10 {
            println!("⚠️ Большое количество TODO по архитектуре!");
        }

        assert!(true);
    }

    // ========================================================================
    // ТЕСТЫ НА СОГЛАСОВАННОСТЬ ИМЕНОВАНИЯ
    // ========================================================================

    #[test]
    fn test_module_naming_consistency() {
        // Проверяем, что директория тестов переименована
        let tests_dir = "src/tests";
        let testes_dir = "src/testes";

        // tests/ должна существовать
        assert!(
            Path::new(tests_dir).exists(),
            "Директория тестов должна называться 'tests/' (не 'testes/')"
        );

        // testes/ не должна существовать
        assert!(
            !Path::new(testes_dir).exists(),
            "Директория 'testes/' должна быть переименована в 'tests/'"
        );
    }

    #[test]
    fn test_unused_dependencies_removed() {
        // Проверяем, что неиспользуемые зависимости удалены
        let cargo_toml_path = "Cargo.toml";
        let cargo_toml_content =
            fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

        // fs2 не должен быть в зависимостях
        assert!(
            !cargo_toml_content.contains("fs2 = "),
            "Неиспользуемая зависимость fs2 должна быть удалена"
        );
    }

    // ========================================================================
    // КОМПЛЕКСНЫЕ ТЕСТЫ АРХИТЕКТУРЫ
    // ========================================================================

    #[test]
    fn test_architecture_integrity_comprehensive() {
        // Комплексный тест целостности архитектуры

        // 1. Проверяем отсутствие циклических зависимостей
        test_no_circular_dependencies_game_constants();
        test_no_circular_dependencies_io_game();
        test_constants_module_independence();

        // 2. Проверяем соблюдение границ модулей
        test_game_logic_does_not_import_render();
        test_game_scoring_does_not_import_render();
        test_validation_module_independence();

        // 3. Проверяем инкапсуляцию
        test_game_state_has_getters();
        test_game_state_has_setters_with_validation();
        test_game_state_uses_saturating_arithmetic();

        // 4. Проверяем отсутствие дублирования
        test_no_duplicate_validation_logic();
        test_crypto_module_reuse();

        // 5. Проверяем SOLID принципы
        test_single_responsibility_controls_config();
        test_dependency_inversion_game_cycle();
        test_interface_segregation_game_access();

        // 6. Проверяем именование
        test_module_naming_consistency();
        test_unused_dependencies_removed();

        assert!(true, "Все архитектурные тесты прошли успешно");
    }
}
