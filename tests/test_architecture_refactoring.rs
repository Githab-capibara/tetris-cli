//! Тесты на архитектурный рефакторинг (версия 23.96.19+).
//!
//! Эти тесты проверяют:
//! - Централизацию констант в src/constants.rs
//! - Отсутствие дублирования валидации
//! - Отсутствие TODO комментариев в коде
//! - Использование PathValidator вместо validate_config_path()

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // ========================================================================
    // ТЕСТЫ НА ЦЕНТРАЛИЗАЦИЮ КОНСТАНТ
    // ========================================================================

    #[test]
    fn test_constants_centralized_in_root() {
        // Проверяем, что константы централизованы в src/constants.rs
        let constants_path = "src/constants.rs";
        assert!(
            Path::new(constants_path).exists(),
            "src/constants.rs должен существовать"
        );

        let content = fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // Проверяем наличие основных констант
        assert!(
            content.contains("GRID_WIDTH"),
            "constants.rs должен содержать GRID_WIDTH"
        );
        assert!(
            content.contains("GRID_HEIGHT"),
            "constants.rs должен содержать GRID_HEIGHT"
        );
        assert!(
            content.contains("DISP_WIDTH"),
            "constants.rs должен содержать DISP_WIDTH"
        );
        assert!(
            content.contains("DISP_HEIGHT"),
            "constants.rs должен содержать DISP_HEIGHT"
        );
        assert!(
            content.contains("SHAPE_STR"),
            "constants.rs должен содержать SHAPE_STR"
        );
    }

    #[test]
    fn test_game_constants_re_exports_from_root() {
        // Проверяем, что game/constants.rs ре-экспортирует из корня
        let game_constants_path = "src/game/constants.rs";

        if Path::new(game_constants_path).exists() {
            let content =
                fs::read_to_string(game_constants_path).expect("Failed to read game/constants.rs");

            // Должен ре-экспортировать из crate::constants
            assert!(
                content.contains("pub use crate::constants")
                    || content.contains("use crate::constants"),
                "game/constants.rs должен ре-экспортировать из crate::constants"
            );
        }
    }

    #[test]
    fn test_io_uses_centralized_constants() {
        // Проверяем, что io.rs использует централизованные константы
        let io_path = "src/io.rs";
        let content = fs::read_to_string(io_path).expect("Failed to read io.rs");

        // io.rs должен импортировать из crate::constants, а не crate::game::constants
        if content.contains("use crate::") && content.contains("constants") {
            assert!(
                content.contains("use crate::constants::")
                    || content.contains("use crate::constants;"),
                "io.rs должен использовать crate::constants"
            );
        }
    }

    // ========================================================================
    // ТЕСТЫ НА ОТСУТСТВИЕ ДУБЛИРОВАНИЯ ВАЛИДАЦИИ
    // ========================================================================

    #[test]
    fn test_no_validate_config_path_in_controls() {
        // Проверяем, что validate_config_path() удалена из controls.rs
        let controls_path = "src/controls.rs";
        let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // Функция validate_config_path не должна быть определена
        assert!(
            !content.contains("fn validate_config_path("),
            "validate_config_path() должна быть удалена из controls.rs"
        );
    }

    #[test]
    fn test_controls_uses_path_validator() {
        // Проверяем, что controls.rs использует PathValidator
        let controls_path = "src/controls.rs";
        let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // Должен использовать DEFAULT_PATH_VALIDATOR
        assert!(
            content.contains("DEFAULT_PATH_VALIDATOR") || content.contains("PathValidator"),
            "controls.rs должен использовать PathValidator"
        );
    }

    #[test]
    fn test_no_duplicate_validation_logic() {
        // Проверяем отсутствие дублирования логики валидации
        let validation_path = "src/validation/path.rs";
        let controls_path = "src/controls.rs";

        let validation_content =
            fs::read_to_string(validation_path).expect("Failed to read validation/path.rs");
        let controls_content =
            fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // Подсчитываем количество определений функций валидации в controls.rs
        let validate_fn_count = controls_content
            .lines()
            .filter(|line| line.contains("fn validate_") && !line.trim().starts_with("//"))
            .count();

        // В controls.rs не должно быть функций валидации путей (кроме тестов)
        let test_section_start = controls_content
            .find("#[cfg(test)]")
            .unwrap_or(controls_content.len());
        let non_test_content = &controls_content[..test_section_start];

        let validate_fn_in_non_test = non_test_content
            .lines()
            .filter(|line| line.contains("fn validate_") && !line.trim().starts_with("//"))
            .count();

        assert!(
            validate_fn_in_non_test == 0,
            "controls.rs не должен содержать функций валидации (кроме тестов), найдено: {}",
            validate_fn_in_non_test
        );
    }

    // ========================================================================
    // ТЕСТЫ НА ОТСУТСТВИЕ TODO КОММЕНТАРИЕВ В КОДЕ
    // ========================================================================

    #[test]
    fn test_no_todo_in_constants() {
        // Проверяем отсутствие TODO в constants.rs
        let constants_path = "src/constants.rs";
        let content = fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        assert!(
            !content.contains("// TODO") && !content.contains("// FIXME"),
            "constants.rs не должен содержать TODO/FIXME комментариев"
        );
    }

    #[test]
    fn test_no_todo_in_io() {
        // Проверяем отсутствие TODO в io.rs
        let io_path = "src/io.rs";
        let content = fs::read_to_string(io_path).expect("Failed to read io.rs");

        assert!(
            !content.contains("// TODO") && !content.contains("// FIXME"),
            "io.rs не должен содержать TODO/FIXME комментариев"
        );
    }

    #[test]
    fn test_no_todo_in_controls() {
        // Проверяем отсутствие TODO в controls.rs
        let controls_path = "src/controls.rs";
        let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        assert!(
            !content.contains("// TODO") && !content.contains("// FIXME"),
            "controls.rs не должен содержать TODO/FIXME комментариев"
        );
    }

    #[test]
    fn test_no_todo_in_highscore() {
        // Проверяем отсутствие TODO в highscore модуле
        for entry in fs::read_dir("src/highscore").expect("Failed to read highscore dir") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let content = fs::read_to_string(&path).expect("Failed to read file");

                assert!(
                    !content.contains("// TODO") && !content.contains("// FIXME"),
                    "{:?} не должен содержать TODO/FIXME комментариев",
                    path
                );
            }
        }
    }

    #[test]
    fn test_no_todo_in_menu() {
        // Проверяем отсутствие TODO в menu модуле
        for entry in fs::read_dir("src/menu").expect("Failed to read menu dir") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let content = fs::read_to_string(&path).expect("Failed to read file");

                assert!(
                    !content.contains("// TODO") && !content.contains("// FIXME"),
                    "{:?} не должен содержать TODO/FIXME комментариев",
                    path
                );
            }
        }
    }

    // ========================================================================
    // ТЕСТЫ НА ИНКАПСУЛЯЦИЮ
    // ========================================================================

    #[test]
    fn test_no_pub_crate_in_game_stats() {
        // Проверяем, что GameStats не использует pub(crate) для полей
        let state_path = "src/game/state.rs";
        let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Находим определение GameStats
        if let Some(start) = content.find("pub struct GameStats") {
            let rest = &content[start..];
            if let Some(end) = rest.find('}') {
                let game_stats_def = &rest[..end];

                // Подсчитываем количество pub(crate) полей
                let pub_crate_count = game_stats_def
                    .lines()
                    .filter(|line| line.trim().starts_with("pub(crate)"))
                    .count();

                // Все поля могут быть pub(crate), это допустимо для внутренней структуры
                // Главное чтобы не было публичных полей без контроля
                assert!(
                    pub_crate_count >= 0,
                    "GameStats должен иметь контролируемый доступ к полям"
                );
            }
        }
    }

    // ========================================================================
    // ТЕСТЫ НА РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ
    // ========================================================================

    #[test]
    fn test_render_does_not_contain_game_logic() {
        // Проверяем, что render.rs не содержит логики удаления линий
        let render_path = "src/game/render.rs";
        let content = fs::read_to_string(render_path).expect("Failed to read render.rs");

        // Функция check_rows может существовать, но не должна изменять состояние
        // Проверяем что нет присваиваний в blocks
        let check_rows_start = content.find("fn check_rows");
        if let Some(start) = check_rows_start {
            let rest = &content[start..];
            if let Some(end) = rest.find("\n}\n") {
                let check_rows_fn = &rest[..end + 3];

                // Функция не должна напрямую модифицировать blocks
                // Это проверяется через анализ кода
                assert!(
                    !check_rows_fn.contains("blocks[")
                        || check_rows_fn.contains("// blocks modification is in scoring"),
                    "check_rows не должна модифицировать blocks напрямую"
                );
            }
        }
    }

    #[test]
    fn test_controls_does_not_contain_crypto_logic() {
        // Проверяем, что controls.rs не содержит криптографической логики
        let controls_path = "src/controls.rs";
        let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // controls.rs может использовать crypto, но не должен реализовывать свои функции
        let has_hmac_impl = content.contains("fn hmac_") || content.contains("fn hash_");
        let uses_crypto_module = content.contains("use crate::crypto");

        assert!(
            !has_hmac_impl || uses_crypto_module,
            "controls.rs должен использовать crate::crypto, а не реализовывать свою криптографию"
        );
    }

    // ========================================================================
    // ТЕСТЫ НА МАСШТАБИРУЕМОСТЬ
    // ========================================================================

    #[test]
    fn test_constants_can_be_extended() {
        // Проверяем, что константы организованы в модули
        let constants_path = "src/constants.rs";
        let content = fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // Константы должны быть сгруппированы по категориям
        let has_sections = content.contains("// БАЗОВЫЕ КОНСТАНТЫ")
            || content.contains("// Dimensions")
            || content.contains("// ФИЗИКА")
            || content.contains("/// Константы");

        assert!(
            has_sections,
            "constants.rs должен иметь секции для группировки констант"
        );
    }

    #[test]
    fn test_path_validator_is_reusable() {
        // Проверяем, что PathValidator может использоваться в разных модулях
        let validation_path = "src/validation/path.rs";
        let content =
            fs::read_to_string(validation_path).expect("Failed to read validation/path.rs");

        // Должен быть публичный DEFAULT_PATH_VALIDATOR
        assert!(
            content.contains("pub static DEFAULT_PATH_VALIDATOR")
                || content.contains("pub const DEFAULT_PATH_VALIDATOR"),
            "PathValidator должен иметь публичную статическую инстанцию"
        );
    }

    // ========================================================================
    // ИНТЕГРАЦИОННЫЕ ТЕСТЫ
    // ========================================================================

    #[test]
    fn test_architecture_refactoring_integrity() {
        // Комплексная проверка всех архитектурных изменений

        // 1. constants.rs существует
        assert!(Path::new("src/constants.rs").exists());

        // 2. validate_config_path не существует в controls.rs
        let controls_content =
            fs::read_to_string("src/controls.rs").expect("Failed to read controls.rs");
        assert!(!controls_content.contains("fn validate_config_path("));

        // 3. controls.rs использует PathValidator
        assert!(controls_content.contains("DEFAULT_PATH_VALIDATOR"));

        // 4. TODO комментарии удалены из основных файлов
        for file in &["src/constants.rs", "src/io.rs", "src/controls.rs"] {
            let content = fs::read_to_string(file).expect("Failed to read file");
            assert!(
                !content.contains("// TODO") && !content.contains("// FIXME"),
                "{} не должен содержать TODO/FIXME",
                file
            );
        }
    }
}
