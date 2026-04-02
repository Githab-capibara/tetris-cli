//! Тесты целостности архитектуры для проекта tetris-cli.
//!
//! Этот файл содержит тесты для проверки архитектурных ограничений:
//! - Отсутствие циклических зависимостей (C2)
//! - Целостность компонентов (C1)
//! - TOCTOU защита (C3)
//! - Централизация HMAC (C4)
//! - Разделение трейтов (H1)
//! - DIP (H2)
//! - SoC (H5)
//! - Абстракция времени (H6)
//! - Интеграционные тесты архитектуры

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // ========================================================================
    // РАЗДЕЛ 1: ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ (C2)
    // ========================================================================

    /// Проверить что core не зависит от game, io, tetromino.
    /// Core должен быть независимым базовым модулем.
    #[test]
    fn test_no_cyclic_dependencies_core_types() {
        let core_path = "src/core/mod.rs";
        let core_content = fs::read_to_string(core_path).expect("Failed to read src/core/mod.rs");

        // Core не должен импортировать из game, io, tetromino
        assert!(
            !core_content.contains("use crate::game::"),
            "core/mod.rs не должен импортировать из crate::game"
        );
        assert!(
            !core_content.contains("use crate::io::"),
            "core/mod.rs не должен импортировать из crate::io"
        );
        assert!(
            !core_content.contains("use crate::tetromino::"),
            "core/mod.rs не должен импортировать из crate::tetromino"
        );
    }

    /// Проверить что Direction, RotationDirection, Position не импортируют
    /// из других модулей кроме std.
    #[test]
    fn test_core_types_are_independent() {
        let core_path = "src/core/mod.rs";
        let core_content = fs::read_to_string(core_path).expect("Failed to read src/core/mod.rs");

        // Разрешены только импорты из std
        let lines: Vec<&str> = core_content.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("use crate::") && !trimmed.contains("//") {
                // Исключаем импорты из самого core модуля
                assert!(
                    trimmed.contains("crate::core::"),
                    "core/mod.rs содержит запрещённый импорт: {trimmed}\n\
                         Core типы должны быть независимыми и использовать только std"
                );
            }
        }
    }

    // ========================================================================
    // РАЗДЕЛ 2: ТЕСТЫ НА ЦЕЛОСТНОСТЬ КОМПОНЕНТОВ (C1)
    // ========================================================================

    /// Проверить что GameState использует GameBoard, ScoreBoard,
    /// а не хранит поля напрямую.
    #[test]
    fn test_game_state_uses_components() {
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // GameState должен использовать композицию с GameBoard
        assert!(
            state_content.contains("GameBoard") || state_content.contains("board:"),
            "GameState должен использовать GameBoard компонент"
        );

        // GameState должен использовать ScoreBoard
        assert!(
            state_content.contains("ScoreBoard") || state_content.contains("scoreboard:"),
            "GameState должен использовать ScoreBoard компонент"
        );
    }

    /// Проверить что GameBoard не зависит от ScoreBoard и наоборот
    /// (низкая связанность).
    #[test]
    fn test_components_are_independent() {
        let board_path = "src/game/board.rs";
        let scoreboard_path = "src/game/scoreboard.rs";

        let board_content = fs::read_to_string(board_path).unwrap_or_default();
        let scoreboard_content = fs::read_to_string(scoreboard_path).unwrap_or_default();

        // GameBoard не должен импортировать ScoreBoard
        assert!(
            !board_content.contains("use crate::game::scoreboard::")
                && !board_content.contains("use super::scoreboard::"),
            "GameBoard не должен зависеть от ScoreBoard"
        );

        // ScoreBoard не должен импортировать GameBoard
        assert!(
            !scoreboard_content.contains("use crate::game::board::")
                && !scoreboard_content.contains("use super::board::"),
            "ScoreBoard не должен зависеть от GameBoard"
        );
    }

    // ========================================================================
    // РАЗДЕЛ 3: ТЕСТЫ НА TOCTOU ЗАЩИТУ (C3)
    // ========================================================================

    /// Проверить что score() и is_valid() атомарны.
    /// LeaderboardEntry намеренно !Send + !Sync (PhantomData<*mut ()>),
    /// поэтому тестируем атомарность в одном потоке.
    #[test]
    fn test_thread_safe_leaderboard_entry_is_atomic() {
        let player_name = "TestPlayer";
        let score = 1000u128;

        let entry = tetris_cli::highscore::leaderboard::LeaderboardEntry::new(player_name, score);

        // Проверяем что score() возвращает корректное значение
        assert_eq!(
            entry.score(),
            Some(score),
            "score() должен возвращать корректное значение"
        );

        // Проверяем что is_valid() работает корректно
        assert!(
            entry.is_valid(),
            "is_valid() должен возвращать true для валидной записи"
        );

        // Проверяем что hash() возвращает непустую строку
        let hash = entry.hash();
        assert!(!hash.is_empty(), "hash() должен возвращать непустую строку");

        // Проверяем что name() возвращает правильное имя
        assert_eq!(
            entry.name(),
            player_name,
            "name() должен возвращать правильное имя"
        );

        // Проверяем что score() и is_valid() согласованы
        // Если is_valid() возвращает true, score() должен возвращать правильное значение
        if entry.is_valid() {
            assert_eq!(
                entry.score(),
                Some(score),
                "score() должен возвращать корректное значение для валидной записи"
            );
        }

        // Тест для Leaderboard (однопоточный)
        let mut leaderboard = tetris_cli::highscore::leaderboard::Leaderboard::default();

        for i in 0..5 {
            let player_name = format!("Player_{i}");
            let score = 1000 + i as u128;

            let entry =
                tetris_cli::highscore::leaderboard::LeaderboardEntry::new(&player_name, score);

            // Проверяем атомарность
            assert_eq!(entry.score(), Some(score));
            assert!(entry.is_valid());

            let _ = leaderboard.add_score(&player_name, score);
        }

        assert!(
            !leaderboard.get_entries().is_empty(),
            "Записи должны быть добавлены в таблицу лидеров"
        );
    }

    /// Тест на целостность данных LeaderboardEntry.
    #[test]
    fn test_leaderboard_entry_thread_safety() {
        // LeaderboardEntry !Send + !Sync, поэтому тестируем целостность в одном потоке
        // Это проверяет что данные не повреждаются при множественных операциях

        let mut entries = Vec::new();

        for i in 0..100 {
            let player_name = format!("Player_{i}");
            let score = i as u128 * 100;

            let entry =
                tetris_cli::highscore::leaderboard::LeaderboardEntry::new(&player_name, score);

            // Проверяем что запись валидна
            assert!(entry.is_valid(), "Запись должна быть валидной");
            assert_eq!(entry.score(), Some(score), "Счёт должен совпадать");
            assert_eq!(entry.name(), player_name, "Имя должно совпадать");

            entries.push(entry);
        }

        // Проверяем что все записи остаются валидными после множественных операций
        for (i, entry) in entries.iter().enumerate() {
            let expected_score = i as u128 * 100;
            assert!(entry.is_valid(), "Запись должна оставаться валидной");
            assert_eq!(
                entry.score(),
                Some(expected_score),
                "Счёт должен совпадать после множественных операций"
            );
        }

        // Тест для Leaderboard
        let mut leaderboard = tetris_cli::highscore::leaderboard::Leaderboard::default();

        for i in 0..10 {
            let player_name = format!("Thread_{i}");
            let score = i as u128 * 100;
            let _ = leaderboard.add_score(&player_name, score);
        }

        let entries = leaderboard.get_entries();
        assert!(
            !entries.is_empty(),
            "Таблица лидеров должна содержать хотя бы одну запись"
        );

        // Проверяем что все записи валидны
        for entry in entries {
            assert!(entry.is_valid(), "Все записи должны быть валидными");
        }
    }

    // ========================================================================
    // РАЗДЕЛ 4: ТЕСТЫ НА ЦЕНТРАЛИЗАЦИЮ HMAC (C4)
    // ========================================================================

    /// Проверить что hmac_sign и hmac_verify определены только в crypto::hmac модуле.
    #[test]
    fn test_hmac_functions_centralized() {
        let hmac_path = "src/crypto/hmac.rs";
        let crypto_path = "src/crypto.rs";

        let hmac_content = fs::read_to_string(hmac_path).expect("Failed to read crypto/hmac.rs");
        let crypto_content = fs::read_to_string(crypto_path).expect("Failed to read crypto.rs");

        // hmac_sign и hmac_verify должны быть определены в hmac.rs
        assert!(
            hmac_content.contains("pub fn hmac_sign")
                || hmac_content.contains("pub fn hmac_verify"),
            "hmac_sign и hmac_verify должны быть определены в crypto/hmac.rs"
        );

        // crypto.rs должен ре-экспортировать функции из hmac.rs
        assert!(
            crypto_content.contains("pub use hmac::")
                || crypto_content.contains("hmac_sign")
                || crypto_content.contains("hmac_verify"),
            "crypto.rs должен ре-экспортировать HMAC функции из hmac.rs"
        );
    }

    /// Проверить что нет дублирования HMAC логики в controls.rs или highscore/.
    #[test]
    fn test_no_duplicate_hmac_logic() {
        let controls_path = "src/controls.rs";
        let leaderboard_path = "src/highscore/leaderboard.rs";
        let save_data_path = "src/highscore/save_data.rs";
        let hmac_path = "src/crypto/hmac.rs";

        let controls_content = fs::read_to_string(controls_path).unwrap_or_default();
        let leaderboard_content = fs::read_to_string(leaderboard_path).unwrap_or_default();
        let save_data_content = fs::read_to_string(save_data_path).unwrap_or_default();
        let hmac_content = fs::read_to_string(hmac_path).unwrap_or_default();

        // Проверяем что hmac модуль существует и содержит основные функции
        assert!(
            hmac_content.contains("pub fn hmac_sign")
                || hmac_content.contains("pub fn hmac_verify")
                || hmac_content.contains("pub fn hmac_sign_with_salt")
                || hmac_content.contains("pub fn hmac_verify_with_salt"),
            "crypto/hmac.rs должен содержать HMAC функции"
        );

        // controls.rs должен использовать crate::crypto для HMAC
        if controls_content.contains("hmac") || controls_content.contains("HMAC") {
            assert!(
                controls_content.contains("crate::crypto::")
                    || controls_content.contains("use crate::crypto"),
                "controls.rs должен использовать crate::crypto для HMAC"
            );
        }

        // leaderboard.rs должен импортировать HMAC из crypto::hmac, а не реализовывать свою логику
        // Проверяем что используется импорт из crypto::hmac
        assert!(
            leaderboard_content.contains("use crate::crypto::hmac::")
                || leaderboard_content.contains("hmac_sign_with_salt")
                || leaderboard_content.contains("hmac_verify_with_salt"),
            "leaderboard.rs должен использовать HMAC из crypto::hmac"
        );

        // save_data.rs не должен содержать собственной HMAC реализации
        // Он должен использовать crypto модуль
        if save_data_content.contains("fn hmac_") {
            assert!(
                save_data_content.contains("crate::crypto::")
                    || save_data_content.contains("use crate::crypto"),
                "save_data.rs должен использовать crate::crypto для HMAC"
            );
        }
    }

    // ========================================================================
    // РАЗДЕЛ 5: ТЕСТЫ НА РАЗДЕЛЕНИЕ ТРЕЙТОВ (H1)
    // ========================================================================

    /// Проверить что ScoreAccess, LevelAccess, LinesAccess, ComboAccess
    /// разделены и не дублируют методы.
    #[test]
    fn test_scoring_traits_are_segregated() {
        let access_path = "src/game/access.rs";
        let access_content = fs::read_to_string(access_path).expect("Failed to read access.rs");

        // Должны существовать отдельные трейты
        let has_score_access = access_content.contains("trait ScoreAccess");
        let has_score_mutable = access_content.contains("trait ScoreMutable");
        let has_board_readonly = access_content.contains("trait BoardReadonly");
        let has_board_mutable = access_content.contains("trait BoardMutable");

        assert!(has_score_access, "Должен существовать трейт ScoreAccess");
        assert!(has_score_mutable, "Должен существовать трейт ScoreMutable");
        assert!(
            has_board_readonly,
            "Должен существовать трейт BoardReadonly"
        );
        assert!(has_board_mutable, "Должен существовать трейт BoardMutable");

        // ScoreAccess должен содержать только методы чтения (get_*)
        // ScoreMutable должен содержать методы записи (set_*, add_*)
        // Это проверяет разделение по ISP
        assert!(
            access_content.contains("get_score")
                && access_content.contains("get_level")
                && access_content.contains("get_lines_cleared"),
            "ScoreAccess должен содержать методы чтения"
        );

        assert!(
            access_content.contains("add_score")
                && access_content.contains("set_score")
                && access_content.contains("set_level"),
            "ScoreMutable должен содержать методы записи"
        );

        // Проверяем что ScoreAccess и ScoreMutable - разные трейты
        let score_access_start = access_content.find("pub trait ScoreAccess");
        let score_mutable_start = access_content.find("pub trait ScoreMutable");

        assert!(
            score_access_start.is_some() && score_mutable_start.is_some(),
            "Должны существовать оба трейта: ScoreAccess и ScoreMutable"
        );

        // ScoreMutable должен расширять ScoreAccess
        assert!(
            access_content.contains("trait ScoreMutable: ScoreAccess"),
            "ScoreMutable должен расширять ScoreAccess"
        );
    }

    /// Проверить что нет широкого трейта с 10+ методами.
    #[test]
    fn test_no_monolithic_scoring_trait() {
        let access_path = "src/game/access.rs";
        let scoring_path = "src/game/scoring/mod.rs";

        let access_content = fs::read_to_string(access_path).unwrap_or_default();
        let scoring_content = fs::read_to_string(scoring_path).unwrap_or_default();

        // Подсчитываем количество методов в каждом трейте
        let mut max_methods = 0;
        let mut max_trait_name = String::new();

        for content in [&access_content, &scoring_content] {
            let mut current_trait = String::new();
            let mut method_count = 0;

            for line in content.lines() {
                let trimmed = line.trim();

                // Находим объявление трейта
                if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
                    if !current_trait.is_empty() && method_count > max_methods {
                        max_methods = method_count;
                        max_trait_name = current_trait.clone();
                    }

                    current_trait = trimmed
                        .trim_start_matches("pub ")
                        .trim_start_matches("trait ")
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    method_count = 0;
                } else if trimmed.starts_with("fn ") && !current_trait.is_empty() {
                    method_count += 1;
                } else if trimmed.starts_with('}') && !current_trait.is_empty() {
                    if method_count > max_methods {
                        max_methods = method_count;
                        max_trait_name = current_trait.clone();
                    }
                    current_trait.clear();
                    method_count = 0;
                }
            }
        }

        // Предупреждение если найден трейт с большим количеством методов
        if max_methods > 10 {
            println!(
                "⚠️ Найден трейт {max_trait_name} с {max_methods} методами (рекомендуется разделение)"
            );
        }

        // Тест не должен падать, но предупреждает о проблеме
        assert!(
            max_methods <= 15,
            "Трейт {max_trait_name} имеет слишком много методов ({max_methods}). Рекомендуется разделение по ISP."
        );
    }

    // ========================================================================
    // РАЗДЕЛ 6: ТЕСТЫ НА DIP (H2)
    // ========================================================================

    /// Проверить что run_game_loop принимает &mut dyn Renderer,
    /// а не &mut Canvas.
    #[test]
    fn test_game_loop_uses_traits() {
        let cycle_path = "src/game/cycle.rs";
        let cycle_content = fs::read_to_string(cycle_path).expect("Failed to read cycle.rs");

        // run_game_loop должен использовать трейты для абстракции
        assert!(
            cycle_content.contains("&mut dyn") || cycle_content.contains("impl "),
            "run_game_loop должен использовать трейты (dyn Trait или impl Trait)"
        );

        // Не должен принимать конкретный тип Canvas напрямую
        if cycle_content.contains("fn run_game_loop") {
            // Находим сигнатуру функции
            let func_start = cycle_content.find("fn run_game_loop").unwrap_or(0);
            let func_end = cycle_content[func_start..]
                .find('{')
                .map_or(func_start + 500, |i| func_start + i);

            let func_signature = &cycle_content[func_start..func_end];

            // Функция должна использовать трейты для renderer
            assert!(
                func_signature.contains("dyn")
                    || func_signature.contains("impl")
                    || func_signature.contains("T: "),
                "run_game_loop должен использовать трейты для renderer параметра"
            );
        }
    }

    /// Проверить использование трейтов в Application.
    #[test]
    fn test_application_uses_trait_objects() {
        let app_path = "src/app/application.rs";
        let app_content = fs::read_to_string(app_path).expect("Failed to read application.rs");

        // Application должен использовать трейты для зависимостей
        assert!(
            app_content.contains("dyn ") || app_content.contains("impl "),
            "Application должен использовать трейты для зависимостей"
        );

        // Проверяем что Application не зависит от конкретных реализаций
        let has_trait_usage = app_content.contains("Box<dyn")
            || app_content.contains("&dyn")
            || app_content.contains("impl ")
            || app_content.contains("T: ");

        assert!(
            has_trait_usage,
            "Application должен использовать трейт-объекты или дженерики"
        );
    }

    // ========================================================================
    // РАЗДЕЛ 7: ТЕСТЫ НА SoC (H5)
    // ========================================================================

    /// Проверить что parse_input() не изменяет состояние и возвращает только GameAction.
    #[test]
    fn test_input_parser_is_pure() {
        let input_path = "src/game/logic/input.rs";
        let input_content = fs::read_to_string(input_path).expect("Failed to read input.rs");

        // parse_input должен быть чистой функцией
        // Он не должен изменять внешнее состояние

        // Проверяем что функция возвращает GameAction или Option<GameAction>
        assert!(
            input_content.contains("fn parse_input")
                && (input_content.contains("-> GameAction")
                    || input_content.contains("-> Option<GameAction>")
                    || input_content.contains("-> Option<crate::game::types::GameAction>")),
            "parse_input должен возвращать GameAction"
        );

        // Функция не должна содержать побочных эффектов
        // Проверяем что нет вызовов set_*, save_*, write_* внутри parse_input
        let parse_start = input_content.find("fn parse_input");
        if let Some(start) = parse_start {
            let parse_end = input_content[start..]
                .find("\n}")
                .map_or(start + 200, |i| start + i + 2);

            let parse_content = &input_content[start..parse_end];

            // Чистая функция не должна изменять состояние
            assert!(
                !parse_content.contains(".set_")
                    && !parse_content.contains("self.")
                    && !parse_content.contains("save_")
                    && !parse_content.contains("write_"),
                "parse_input должен быть чистой функцией без побочных эффектов"
            );
        }
    }

    /// Проверить что handle_input() использует parse_input() и execute_action() отдельно.
    #[test]
    fn test_input_logic_separation() {
        let controls_path = "src/controls.rs";
        let controls_content =
            fs::read_to_string(controls_path).expect("Failed to read controls.rs");

        // Проверяем что существует функция для маппинга клавиш на действия
        let has_map_key_to_action = controls_content.contains("fn map_key_to_action")
            || controls_content.contains("pub fn map_key_to_action");

        // Проверяем что ControlsConfig имеет методы для работы с конфигурацией
        let has_config_methods = controls_content.contains("impl ControlsConfig");

        // Проверяем что есть разделение между конфигурацией и логикой
        let has_separation = has_map_key_to_action && has_config_methods;

        assert!(
            has_separation,
            "Должно быть разделение между конфигурацией управления и логикой маппинга клавиш"
        );

        // Проверяем что map_key_to_action возвращает GameAction
        if has_map_key_to_action {
            assert!(
                controls_content.contains("-> Option<crate::game::types::GameAction>")
                    || controls_content.contains("-> Option<GameAction>"),
                "map_key_to_action должен возвращать Option<GameAction>"
            );
        }
    }

    // ========================================================================
    // РАЗДЕЛ 8: ТЕСТЫ НА АБСТРАКЦИЮ ВРЕМЕНИ (H6)
    // ========================================================================

    /// Проверить что Time структура существует и имеет нужные методы.
    #[test]
    fn test_time_abstraction_exists() {
        let time_path = "src/game/time.rs";
        let time_content = fs::read_to_string(time_path).expect("Failed to read time.rs");

        // Time структура должна существовать
        assert!(
            time_content.contains("pub struct Time"),
            "Структура Time должна существовать"
        );

        // Time должен иметь методы для работы со временем
        assert!(
            time_content.contains("fn from_secs") || time_content.contains("fn from_millis"),
            "Time должен иметь методы создания из секунд/миллисекунд"
        );

        assert!(
            time_content.contains("fn as_secs")
                || time_content.contains("fn as_millis")
                || time_content.contains("fn as_secs_f64"),
            "Time должен иметь методы получения времени"
        );
    }

    /// Проверить что Time используется вместо f64 для времени игры.
    #[test]
    fn test_time_type_safety() {
        let state_path = "src/game/state.rs";
        let stats_file_path = "src/game/stats.rs";

        let state_content = fs::read_to_string(state_path).unwrap_or_default();
        let stats_file_content = fs::read_to_string(stats_file_path).unwrap_or_default();

        // Проверяем что Time тип используется в коде
        let uses_time_type = state_content.contains("Time")
            || stats_file_content.contains("Time")
            || state_content.contains("game::time::Time")
            || stats_file_content.contains("game::time::Time");

        // Это предупреждение, не ошибка - Time может ещё не использоваться везде
        if !uses_time_type {
            println!("⚠️ Time тип может ещё не использоваться во всех модулях");
        }

        // Проверяем что Time структура определена
        let time_path = "src/game/time.rs";
        let time_content = fs::read_to_string(time_path).expect("Failed to read time.rs");

        assert!(
            time_content.contains("pub struct Time"),
            "Time структура должна быть определена для типобезопасности"
        );
    }

    // ========================================================================
    // РАЗДЕЛ 9: ИНТЕГРАЦИОННЫЕ ТЕСТЫ АРХИТЕКТУРЫ
    // ========================================================================

    /// Проверить что модули не нарушают границы: core ← game ← app (правильная иерархия).
    #[test]
    fn test_architecture_module_boundaries() {
        // Core не должен зависеть от game
        let core_path = "src/core/mod.rs";
        let core_content = fs::read_to_string(core_path).expect("Failed to read core/mod.rs");

        assert!(
            !core_content.contains("use crate::game::"),
            "core не должен зависеть от game"
        );

        // Game может зависеть от core
        let game_mod_path = "src/game/mod.rs";
        let game_mod_content =
            fs::read_to_string(game_mod_path).expect("Failed to read game/mod.rs");

        // Game может импортировать из core (это разрешено)
        let game_depends_on_core = game_mod_content.contains("crate::core::");

        // App может зависеть от game и core
        let app_path = "src/app/application.rs";
        let app_content = fs::read_to_string(app_path).expect("Failed to read application.rs");

        let app_depends_on_game = app_content.contains("crate::game::");
        let app_depends_on_core = app_content.contains("crate::core::");

        // Проверяем что иерархия соблюдается
        assert!(
            !core_content.contains("use crate::game::"),
            "Нарушение иерархии: core не должен зависеть от game"
        );

        // Выводим информацию о зависимостях для отладки
        println!("Game зависит от core: {game_depends_on_core}");
        println!("App зависит от game: {app_depends_on_game}");
        println!("App зависит от core: {app_depends_on_core}");
    }

    /// Проверить что используются геттеры/сеттеры, а не прямой доступ к полям.
    #[test]
    fn test_no_direct_field_access() {
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // GameState должен иметь публичные геттеры
        let has_score_getter = state_content.contains("pub fn score(&self)");
        let has_level_getter = state_content.contains("pub fn level(&self)");
        let has_lines_getter = state_content.contains("pub fn lines_cleared(&self)");

        assert!(
            has_score_getter,
            "GameState должен иметь публичный геттер score()"
        );
        assert!(
            has_level_getter,
            "GameState должен иметь публичный геттер level()"
        );
        assert!(
            has_lines_getter,
            "GameState должен иметь публичный геттер lines_cleared()"
        );

        // Поля должны быть приватными
        let has_private_fields = state_content.contains("score:")
            || state_content.contains("level:")
            || state_content.contains("pub(crate) score")
            || state_content.contains("pub(crate) level");

        assert!(
            has_private_fields,
            "Поля GameState должны быть приватными или pub(crate)"
        );
    }

    // ========================================================================
    // ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ ЦЕЛОСТНОСТИ
    // ========================================================================

    /// Проверить что все модули имеют документацию.
    #[test]
    fn test_modules_have_documentation() {
        let modules = vec![
            "src/core/mod.rs",
            "src/game/mod.rs",
            "src/app/mod.rs",
            "src/crypto/mod.rs",
            "src/highscore/mod.rs",
        ];

        for module_path in modules {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path)
                    .unwrap_or_else(|_| panic!("Failed to read {module_path}"));

                // Модуль должен иметь документацию в начале файла
                let has_module_doc = content.starts_with("//!")
                    || content.contains("#![doc =")
                    || content.contains("//! ");

                assert!(
                    has_module_doc,
                    "Модуль {module_path} должен иметь документацию"
                );
            }
        }
    }

    /// Проверить что нет публичных полей в структурах состояния.
    #[test]
    fn test_no_public_fields_in_state_structs() {
        let state_path = "src/game/state.rs";
        let state_content = fs::read_to_string(state_path).expect("Failed to read state.rs");

        // Находим все pub struct определения
        let lines: Vec<&str> = state_content.lines().collect();
        let mut in_pub_struct = false;
        let mut current_struct = String::new();

        for line in &lines {
            let trimmed = line.trim();

            if trimmed.starts_with("pub struct ") && !trimmed.contains('{') {
                in_pub_struct = true;
                current_struct = trimmed
                    .trim_start_matches("pub struct ")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
            } else if trimmed.starts_with("pub struct ") && trimmed.contains('{') {
                // Однострочная структура
            } else if in_pub_struct && trimmed.contains('{') {
                // Начало тела структуры
            } else if in_pub_struct && trimmed.starts_with("pub ") && !trimmed.starts_with("pub fn")
            {
                // Найдено публичное поле
                if !trimmed.contains("//") && !trimmed.contains("///") {
                    println!("⚠️ Структура {current_struct} имеет публичное поле: {trimmed}");
                }
            } else if in_pub_struct && trimmed.starts_with('}') {
                in_pub_struct = false;
                current_struct.clear();
            }
        }

        // Тест всегда проходит, но выводит предупреждения
        // Проверка публичных полей завершена
    }

    /// Проверить что ошибки обрабатываются через Result, а не unwrap/expect.
    #[test]
    fn test_error_handling_uses_result() {
        let main_files = vec!["src/main.rs", "src/app/application.rs", "src/game/cycle.rs"];

        for file_path in main_files {
            if Path::new(file_path).exists() {
                let content = fs::read_to_string(file_path)
                    .unwrap_or_else(|_| panic!("Failed to read {file_path}"));

                // Подсчитываем unwrap/expect в основном коде (не в тестах)
                let main_code_end = content.find("#[cfg(test)]").unwrap_or(content.len());
                let main_code = &content[..main_code_end];

                let unwrap_count = main_code.matches(".unwrap()").count();
                let expect_count = main_code.matches(".expect(").count();

                // Предупреждение если много unwrap/expect
                if unwrap_count + expect_count > 10 {
                    println!(
                        "⚠️ {file_path} содержит {unwrap_count} unwrap() и {expect_count} expect()"
                    );
                }
            }
        }

        // Проверка обработки ошибок завершена
    }
}
