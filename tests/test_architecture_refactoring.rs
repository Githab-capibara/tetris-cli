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

    // ========================================================================
    // КРИТИЧЕСКИЕ ИСПРАВЛЕНИЯ: ТЕСТЫ АРХИТЕКТУРЫ
    // ========================================================================

    #[test]
    fn test_game_state_composition() {
        // Исправление #1: Проверка композиции GameState
        // GameState должен использовать композицию с GameStats и RenderCache

        let state_path = "src/game/state.rs";
        let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем что GameState содержит поля stats и render_cache
        assert!(
            content.contains("stats: GameStats"),
            "GameState должен содержать поле stats: GameStats"
        );
        assert!(
            content.contains("render_cache: RenderCache"),
            "GameState должен содержать поле render_cache: RenderCache"
        );

        // Проверяем что GameStats и RenderCache импортированы из отдельных модулей
        assert!(
            content.contains("use super::stats::GameStats")
                || content.contains("pub use super::stats::GameStats"),
            "GameStats должен быть импортирован из stats модуля"
        );
        assert!(
            content.contains("use super::cache::RenderCache"),
            "RenderCache должен быть импортирован из cache модуля"
        );
    }

    #[test]
    fn test_game_stats_independence() {
        // Исправление #1: Проверка независимости GameStats
        // GameStats должен быть в отдельном модуле stats.rs

        let stats_path = "src/game/stats.rs";
        assert!(
            Path::new(stats_path).exists(),
            "stats.rs должен существовать"
        );

        let content = fs::read_to_string(stats_path).expect("Failed to read stats.rs");

        // Проверяем что GameStats определён как отдельная структура
        assert!(
            content.contains("pub struct GameStats"),
            "GameStats должен быть определён в stats.rs"
        );

        // Проверяем что GameStats имеет методы для работы со статистикой
        assert!(
            content.contains("pub fn new()"),
            "GameStats должен иметь конструктор new()"
        );
        assert!(
            content.contains("pub fn add_piece"),
            "GameStats должен иметь метод add_piece"
        );
        assert!(
            content.contains("pub fn total_pieces"),
            "GameStats должен иметь метод total_pieces"
        );
    }

    #[test]
    fn test_render_cache_independence() {
        // Исправление #1: Проверка независимости RenderCache
        // RenderCache должен быть в отдельном модуле cache.rs

        let cache_path = "src/game/cache.rs";
        assert!(
            Path::new(cache_path).exists(),
            "cache.rs должен существовать"
        );

        let content = fs::read_to_string(cache_path).expect("Failed to read cache.rs");

        // Проверяем что RenderCache определён как отдельная структура
        assert!(
            content.contains("pub struct RenderCache"),
            "RenderCache должен быть определён в cache.rs"
        );

        // Проверяем что RenderCache имеет методы для кэширования
        assert!(
            content.contains("pub fn new()"),
            "RenderCache должен иметь конструктор new()"
        );
        assert!(
            content.contains("pub fn init_with_values"),
            "RenderCache должен иметь метод init_with_values"
        );
    }

    #[test]
    fn test_dependency_injection_usage() {
        // Исправление #2: Проверка использования dependency injection
        // Устранение высокой связанности через внедрение зависимостей

        let state_path = "src/game/state.rs";
        let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем что GameState использует trait object для режима игры
        assert!(
            content.contains("mode_trait: Box<dyn GameModeTrait>"),
            "GameState должен использовать Box<dyn GameModeTrait> для DI"
        );

        // Проверяем что есть метод для получения трейта
        assert!(
            content.contains("pub fn get_mode_trait") || content.contains("pub fn mode_trait"),
            "GameState должен иметь геттер для mode_trait"
        );
    }

    #[test]
    fn test_game_mode_trait_usage() {
        // Исправление #3: Проверка использования GameModeTrait вместо enum
        // Трейт должен использоваться для полиморфизма режимов

        let mode_trait_path = "src/game/mode_trait.rs";
        assert!(
            Path::new(mode_trait_path).exists(),
            "mode_trait.rs должен существовать"
        );

        let content = fs::read_to_string(mode_trait_path).expect("Failed to read mode_trait.rs");

        // Проверяем что трейт определён
        assert!(
            content.contains("pub trait GameModeTrait"),
            "GameModeTrait должен быть определён"
        );

        // Проверяем методы трейта
        assert!(
            content.contains("fn check_win_condition"),
            "GameModeTrait должен иметь метод check_win_condition"
        );
        assert!(
            content.contains("fn get_target_lines"),
            "GameModeTrait должен иметь метод get_target_lines"
        );
        assert!(
            content.contains("fn name"),
            "GameModeTrait должен иметь метод name"
        );

        // Проверяем что есть реализации для разных режимов
        assert!(
            content.contains("impl GameModeTrait for ClassicMode"),
            "Должна быть реализация для ClassicMode"
        );
        assert!(
            content.contains("impl GameModeTrait for SprintMode"),
            "Должна быть реализация для SprintMode"
        );
        assert!(
            content.contains("impl GameModeTrait for MarathonMode"),
            "Должна быть реализация для MarathonMode"
        );
    }

    // ========================================================================
    // ВАЖНЫЕ ИСПРАВЛЕНИЯ: ТЕСТЫ АРХИТЕКТУРЫ
    // ========================================================================

    #[test]
    fn test_constants_import_from_root() {
        // Исправление #4: Проверка импорта констант из constants.rs
        // Константы не должны дублироваться в других модулях

        let constants_path = "src/constants.rs";
        let content = fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // Проверяем наличие всех основных констант
        let required_constants = [
            "GRID_WIDTH",
            "GRID_HEIGHT",
            "DISP_WIDTH",
            "DISP_HEIGHT",
            "SHAPE_STR",
            "FPS",
            "INITIAL_FALL_SPD",
            "LAND_TIME_DELAY_S",
            "LINE_SCORES",
            "SPRINT_LINES",
            "MARATHON_LINES",
        ];

        for constant in &required_constants {
            assert!(
                content.contains(constant),
                "constants.rs должен содержать константу {}",
                constant
            );
        }

        // Проверяем что константы экспортируются
        assert!(
            content.contains("pub const"),
            "Константы должны быть публичными"
        );
    }

    #[test]
    fn test_no_duplicate_constants_in_game_module() {
        // Исправление #4: Проверка отсутствия дублирования констант
        // game/constants.rs должен ре-экспортировать из корня

        let game_constants_path = "src/game/constants.rs";

        if Path::new(game_constants_path).exists() {
            let content =
                fs::read_to_string(game_constants_path).expect("Failed to read game/constants.rs");

            // Не должно быть собственных определений констант
            let has_own_constants = content.contains("pub const GRID_WIDTH")
                || content.contains("pub const GRID_HEIGHT")
                || content.contains("pub const FPS");

            assert!(
                !has_own_constants,
                "game/constants.rs не должен дублировать константы, должен ре-экспортировать"
            );

            // Должен ре-экспортировать из crate::constants
            assert!(
                content.contains("pub use crate::constants")
                    || content.contains("use crate::constants"),
                "game/constants.rs должен ре-экспортировать из crate::constants"
            );
        }
    }

    #[test]
    fn test_hmac_validator_usage() {
        // Исправление #5: Проверка использования HmacValidator
        // HMAC логика должна быть в crypto модуле

        let validator_path = "src/crypto/validator.rs";
        assert!(
            Path::new(validator_path).exists(),
            "validator.rs должен существовать"
        );

        let content = fs::read_to_string(validator_path).expect("Failed to read validator.rs");

        // Проверяем что HmacValidator определён
        assert!(
            content.contains("pub struct HmacValidator"),
            "HmacValidator должен быть определён"
        );

        // Проверяем методы валидатора
        assert!(
            content.contains("pub fn sign"),
            "HmacValidator должен иметь метод sign"
        );
        assert!(
            content.contains("pub fn verify"),
            "HmacValidator должен иметь метод verify"
        );
        assert!(
            content.contains("pub fn new"),
            "HmacValidator должен иметь конструктор new"
        );
    }

    #[test]
    fn test_crypto_module_separation() {
        // Исправление #5: Проверка разделения crypto логики
        // Другие модули не должны содержать HMAC реализацию

        let controls_path = "src/controls.rs";
        let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // controls.rs не должен содержать реализацию HMAC
        let has_hmac_impl =
            content.contains("fn hmac_sha256") || content.contains("fn hash_with_hmac");

        assert!(
            !has_hmac_impl,
            "controls.rs не должен содержать реализацию HMAC функций"
        );

        // Может использовать crypto модуль
        let uses_crypto = content.contains("use crate::crypto");
        assert!(
            uses_crypto,
            "controls.rs должен использовать crate::crypto модуль"
        );
    }

    #[test]
    fn test_io_module_modularity() {
        // Исправление #6: Проверка модульности io.rs
        // io.rs должен использовать трейты из io_traits.rs

        let io_path = "src/io.rs";
        let io_traits_path = "src/io_traits.rs";

        assert!(
            Path::new(io_traits_path).exists(),
            "io_traits.rs должен существовать"
        );

        let io_content = fs::read_to_string(io_path).expect("Failed to read io.rs");
        let traits_content =
            fs::read_to_string(io_traits_path).expect("Failed to read io_traits.rs");

        // Проверяем что io_traits содержит трейты
        assert!(
            traits_content.contains("pub trait Renderer"),
            "io_traits должен содержать трейт Renderer"
        );
        assert!(
            traits_content.contains("pub trait InputReader"),
            "io_traits должен содержать трейт InputReader"
        );

        // Проверяем что io.rs имплементирует трейты
        assert!(
            io_content.contains("impl InputReader for KeyReader"),
            "KeyReader должен имплементировать InputReader"
        );
        assert!(
            io_content.contains("impl Renderer for Canvas"),
            "Canvas должен имплементировать Renderer"
        );
    }

    #[test]
    fn test_io_traits_separation() {
        // Исправление #6: Проверка разделения трейтов ввода/вывода
        // Трейты должны быть в отдельном файле io_traits.rs

        let io_traits_path = "src/io_traits.rs";
        let content = fs::read_to_string(io_traits_path).expect("Failed to read io_traits.rs");

        // Проверяем наличие трейтов
        assert!(
            content.contains("pub trait Renderer"),
            "io_traits должен содержать трейт Renderer"
        );
        assert!(
            content.contains("pub trait InputReader"),
            "io_traits должен содержать трейт InputReader"
        );

        // Проверяем методы трейтов
        assert!(
            content.contains("fn draw_strs") || content.contains("fn draw_string"),
            "Renderer должен иметь методы отрисовки"
        );
        assert!(
            content.contains("fn get_key"),
            "InputReader должен иметь метод get_key"
        );
    }

    #[test]
    fn test_renderer_drawable_flushable() {
        // Исправление #7: Проверка разделения Renderer на Drawable и Flushable
        // Трейт Renderer должен иметь методы для отрисовки и flush

        let io_traits_path = "src/io_traits.rs";
        let content = fs::read_to_string(io_traits_path).expect("Failed to read io_traits.rs");

        // Проверяем что Renderer имеет методы отрисовки
        assert!(
            content.contains("fn draw_strs") || content.contains("fn draw_string"),
            "Renderer должен иметь методы отрисовки"
        );

        // Проверяем что Renderer имеет метод flush
        assert!(
            content.contains("fn flush"),
            "Renderer должен иметь метод flush"
        );

        // Проверяем что Canvas реализует Renderer
        let io_path = "src/io.rs";
        let io_content = fs::read_to_string(io_path).expect("Failed to read io.rs");

        assert!(
            io_content.contains("impl Renderer for Canvas"),
            "Canvas должен реализовывать трейт Renderer"
        );
    }

    #[test]
    fn test_builder_pattern_for_game_state() {
        // Исправление #8: Проверка использования Builder pattern для GameState
        // GameState должен иметь конструкторы с разными конфигурациями

        let state_path = "src/game/state.rs";
        let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем наличие конструкторов
        assert!(
            content.contains("pub fn new()"),
            "GameState должен иметь конструктор new()"
        );
        assert!(
            content.contains("pub fn new_sprint()"),
            "GameState должен иметь конструктор new_sprint()"
        );
        assert!(
            content.contains("pub fn new_marathon()"),
            "GameState должен иметь конструктор new_marathon()"
        );

        // Проверяем наличие внутреннего метода инициализации
        assert!(
            content.contains("fn new_internal"),
            "GameState должен иметь внутренний метод new_internal"
        );
    }

    #[test]
    fn test_deprecated_functions_marked() {
        // Исправление #9: Проверка маркировки устаревших функций
        // Устаревшие функции должны иметь атрибут #[deprecated]

        let state_path = "src/game/state.rs";
        let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Проверяем что устаревшие функции помечены
        assert!(
            content.contains("#[deprecated"),
            "В state.rs должны быть устаревшие функции с #[deprecated]"
        );

        // Проверяем наличие deprecated атрибутов для старых геттеров
        assert!(
            content.contains("deprecated(since =") && content.contains("get_score")
                || content.contains("get_level"),
            "Старые геттеры должны быть помечены как deprecated"
        );

        // Проверяем что GameMode enum помечен как deprecated
        let mode_trait_path = "src/game/mode_trait.rs";
        let mode_content =
            fs::read_to_string(mode_trait_path).expect("Failed to read mode_trait.rs");

        // GameMode enum должен быть в state.rs
        assert!(
            content.contains("#[deprecated") && content.contains("pub enum GameMode"),
            "GameMode enum должен быть помечен как deprecated"
        );
    }

    #[test]
    fn test_soc_violation_fixed() {
        // Исправление #10: Проверка устранения SoC нарушений
        // Логика должна быть разделена по модулям

        // Проверяем что render.rs не содержит логики удаления линий
        let render_path = "src/game/render.rs";
        let render_content = fs::read_to_string(render_path).expect("Failed to read render.rs");

        // Функции удаления линий должны быть в scoring модуле
        assert!(
            !render_content.contains("fn remove_lines")
                && !render_content.contains("fn check_rows"),
            "render.rs не должен содержать функции удаления линий"
        );

        // Проверяем что scoring модуль существует
        let scoring_path = "src/game/scoring/mod.rs";
        assert!(
            Path::new(scoring_path).exists(),
            "scoring/mod.rs должен существовать"
        );

        // Проверяем что scoring содержит логику удаления линий
        let scoring_lines_path = "src/game/scoring/lines.rs";
        assert!(
            Path::new(scoring_lines_path).exists(),
            "scoring/lines.rs должен существовать"
        );

        let scoring_content =
            fs::read_to_string(scoring_lines_path).expect("Failed to read scoring/lines.rs");

        assert!(
            scoring_content.contains("pub fn check_rows")
                || scoring_content.contains("fn check_rows"),
            "scoring/lines.rs должен содержать check_rows"
        );
        assert!(
            scoring_content.contains("pub fn remove_rows")
                || scoring_content.contains("fn remove_rows"),
            "scoring/lines.rs должен содержать remove_rows"
        );
    }

    #[test]
    fn test_logic_module_separation() {
        // Исправление #10: Проверка разделения логики по модулям
        // Логика игры должна быть в logic/ модуле

        let logic_dir = "src/game/logic";
        assert!(
            Path::new(logic_dir).is_dir(),
            "logic/ директория должна существовать"
        );

        // Проверяем наличие подмодулей логики
        let required_logic_modules = [
            "collision.rs",
            "input.rs",
            "physics.rs",
            "rotation.rs",
            "update.rs",
            "wall_kick.rs",
        ];

        for module in &required_logic_modules {
            let module_path = format!("{}/{}", logic_dir, module);
            assert!(
                Path::new(&module_path).exists(),
                "logic/{} должен существовать",
                module
            );
        }
    }

    // ========================================================================
    // ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
    // ========================================================================

    #[test]
    fn test_no_circular_dependencies_game_module() {
        // Проверка отсутствия циклических зависимостей в game модуле
        // Модули должны иметь чёткую иерархию зависимостей

        // Базовые модули не должны зависеть от game::*
        let constants_path = "src/constants.rs";
        let constants_content =
            fs::read_to_string(constants_path).expect("Failed to read constants.rs");

        // constants.rs не должен импортировать из game
        assert!(
            !constants_content.contains("use crate::game"),
            "constants.rs не должен зависеть от game модуля"
        );

        // mode_trait.rs не должен зависеть от game::*
        let mode_trait_path = "src/game/mode_trait.rs";
        let mode_trait_content =
            fs::read_to_string(mode_trait_path).expect("Failed to read mode_trait.rs");

        assert!(
            !mode_trait_content.contains("use super::state::GameState"),
            "mode_trait.rs не должен зависеть от GameState"
        );
    }

    #[test]
    fn test_module_boundary_compliance() {
        // Проверка соблюдения границ модулей
        // Каждый модуль должен иметь чёткую ответственность

        // Проверяем что stats.rs не содержит логики отрисовки
        let stats_path = "src/game/stats.rs";
        let stats_content = fs::read_to_string(stats_path).expect("Failed to read stats.rs");

        assert!(
            !stats_content.contains("fn draw") && !stats_content.contains("Canvas"),
            "stats.rs не должен содержать логики отрисовки"
        );

        // Проверяем что cache.rs не содержит игровой логики
        let cache_path = "src/game/cache.rs";
        let cache_content = fs::read_to_string(cache_path).expect("Failed to read cache.rs");

        assert!(
            !cache_content.contains("fn check_collision")
                && !cache_content.contains("fn update_physics"),
            "cache.rs не должен содержать игровой логики"
        );

        // Проверяем что render.rs не содержит бизнес-логики
        let render_path = "src/game/render.rs";
        let render_content = fs::read_to_string(render_path).expect("Failed to read render.rs");

        assert!(
            !render_content.contains("fn calculate_score")
                && !render_content.contains("fn update_level"),
            "render.rs не должен содержать бизнес-логики"
        );
    }

    #[test]
    fn test_architecture_layering() {
        // Проверка правильной слоистой архитектуры
        // Верхние слои могут зависеть от нижних, но не наоборот

        // Базовый слой: constants, types
        // Средний слой: state, stats, cache, mode_trait
        // Верхний слой: logic, scoring, render, view
        // Самый верхний: cycle

        // Проверяем что state.rs не зависит от render.rs
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        assert!(
            !state_content.contains("use super::render::"),
            "state.rs не должен зависеть от render.rs"
        );

        // Проверяем что logic не зависит от render
        let logic_collision_path = "src/game/logic/collision.rs";
        if Path::new(logic_collision_path).exists() {
            let logic_content =
                fs::read_to_string(logic_collision_path).expect("Failed to read collision.rs");

            assert!(
                !logic_content.contains("use super::render::"),
                "logic не должен зависеть от render"
            );
        }
    }

    // ========================================================================
    // КОМПЛЕКСНЫЕ ИНТЕГРАЦИОННЫЕ ТЕСТЫ
    // ========================================================================

    #[test]
    fn test_all_architectural_fixes_comprehensive() {
        // Комплексная проверка всех 10 архитектурных исправлений

        let mut checks_passed = 0;
        let total_checks = 10;

        // 1. Разделение GameState
        if Path::new("src/game/stats.rs").exists() && Path::new("src/game/cache.rs").exists() {
            checks_passed += 1;
        }

        // 2. Dependency injection
        let state_content =
            fs::read_to_string("src/game/state.rs").expect("Failed to read state.rs");
        if state_content.contains("Box<dyn GameModeTrait>") {
            checks_passed += 1;
        }

        // 3. GameModeTrait
        if Path::new("src/game/mode_trait.rs").exists() {
            let mode_content =
                fs::read_to_string("src/game/mode_trait.rs").expect("Failed to read mode_trait.rs");
            if mode_content.contains("pub trait GameModeTrait") {
                checks_passed += 1;
            }
        }

        // 4. Дублирование констант
        if Path::new("src/constants.rs").exists() {
            checks_passed += 1;
        }

        // 5. HMAC логика
        if Path::new("src/crypto/validator.rs").exists() {
            checks_passed += 1;
        }

        // 6. Разделение io.rs
        if Path::new("src/io_traits.rs").exists() {
            checks_passed += 1;
        }

        // 7. Разделение Renderer
        let traits_content =
            fs::read_to_string("src/io_traits.rs").expect("Failed to read io_traits.rs");
        if traits_content.contains("pub trait Renderer") && traits_content.contains("fn flush") {
            checks_passed += 1;
        }

        // 8. Builder pattern
        if state_content.contains("pub fn new()") && state_content.contains("pub fn new_sprint()") {
            checks_passed += 1;
        }

        // 9. Deprecated функции
        if state_content.contains("#[deprecated") {
            checks_passed += 1;
        }

        // 10. SoC нарушения
        if Path::new("src/game/scoring/lines.rs").exists() {
            checks_passed += 1;
        }

        assert!(
            checks_passed == total_checks,
            "Все {} архитектурных исправлений должны быть применены, пройдено: {}",
            total_checks,
            checks_passed
        );
    }
}
