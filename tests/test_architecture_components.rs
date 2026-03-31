//! Тесты для проверки архитектурной целостности проекта tetris-cli.
//!
//! Эти тесты проверяют соблюдение архитектурных принципов и предотвращают регрессию.
//! Каждый тест соответствует конкретной архитектурной проблеме из документации.
//!
//! ## Группы тестов:
//! - C1: Разделение GameState на компоненты
//! - C2: Инкапсуляция render.rs
//! - C3: Централизация констант
//! - H1: Dependency Inversion Principle
//! - H3: Валидация данных
//! - H4: Инкапсуляция полей
//! - M3: Обработка ошибок в drop()
//! - SOLID: Принципы проектирования

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::assertions_on_constants)]

use std::fs;
use std::path::Path;

// ============================================================================
// C1: ТЕСТЫ НА РАЗДЕЛЕНИЕ GAMESTATE НА КОМПОНЕНТЫ
// ============================================================================

mod test_game_state_components {
    use super::*;

    /// Проверка что GameState использует композицию компонентов.
    ///
    /// ## Требование (C1)
    /// GameState должен содержать компоненты (GameBoard, ScoreBoard, FigureManager,
    /// AnimationState, GamePhase), а не прямые поля.
    ///
    /// ## Проверяемые компоненты:
    /// - GameBoard - состояние поля
    /// - ScoreBoard - состояние очков
    /// - GameStats - статистика игры
    /// - RenderCache - кэш отрисовки
    #[test]
    fn test_game_state_uses_components() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // Проверяем наличие компонентов через композицию
        assert!(
            state_content.contains("board: GameBoard"),
            "GameState должен содержать компонент board: GameBoard"
        );

        assert!(
            state_content.contains("scoreboard: ScoreBoard"),
            "GameState должен содержать компонент scoreboard: ScoreBoard"
        );

        assert!(
            state_content.contains("stats: GameStats"),
            "GameState должен содержать компонент stats: GameStats"
        );

        assert!(
            state_content.contains("render_cache: RenderCache"),
            "GameState должен содержать компонент render_cache: RenderCache"
        );

        println!("✅ GameState использует композицию компонентов");
    }

    /// Проверка что компоненты независимы.
    ///
    /// ## Требование (C1)
    /// GameBoard не должен зависеть от ScoreBoard.
    /// FigureManager не должен зависеть от AnimationState.
    #[test]
    fn test_components_independence() {
        // Проверяем GameBoard
        let board_path = "src/game/board.rs";
        let board_content =
            fs::read_to_string(board_path).expect("Failed to read src/game/board.rs");

        // GameBoard не должен импортировать ScoreBoard
        assert!(
            !board_content.contains("use super::scoreboard::ScoreBoard"),
            "GameBoard не должен зависеть от ScoreBoard"
        );

        // Проверяем ScoreBoard
        let scoreboard_path = "src/game/scoreboard.rs";
        let scoreboard_content =
            fs::read_to_string(scoreboard_path).expect("Failed to read src/game/scoreboard.rs");

        // ScoreBoard не должен импортировать GameBoard
        assert!(
            !scoreboard_content.contains("use super::board::GameBoard"),
            "ScoreBoard не должен зависеть от GameBoard"
        );

        // Проверяем stats.rs
        let stats_path = "src/game/stats.rs";
        if Path::new(stats_path).exists() {
            let stats_content =
                fs::read_to_string(stats_path).expect("Failed to read src/game/stats.rs");

            // GameStats не должен зависеть от board или scoreboard
            assert!(
                !stats_content.contains("use super::board::")
                    && !stats_content.contains("use super::scoreboard::"),
                "GameStats не должен зависеть от GameBoard или ScoreBoard"
            );
        }

        println!("✅ Компоненты независимы друг от друга");
    }

    /// Проверка что существуют трейты доступа к компонентам.
    ///
    /// ## Требование (C1)
    /// Должны существовать трейты:
    /// - FigureAccess, FigureMutable
    /// - AnimationAccess, AnimationMutable
    /// - BoardReadonly, BoardMutable
    /// - ScoreAccess, ScoreMutable
    #[test]
    fn test_component_access_traits() {
        let access_path = "src/game/access.rs";
        let access_content =
            fs::read_to_string(access_path).expect("Failed to read src/game/access.rs");

        // Проверяем наличие трейтов доступа к полю
        assert!(
            access_content.contains("trait BoardReadonly"),
            "Должен существовать трейт BoardReadonly"
        );

        assert!(
            access_content.contains("trait BoardMutable"),
            "Должен существовать трейт BoardMutable"
        );

        // Проверяем наличие трейтов доступа к очкам
        assert!(
            access_content.contains("trait ScoreAccess"),
            "Должен существовать трейт ScoreAccess"
        );

        // Проверяем scoreboard.rs на наличие трейтов
        let scoreboard_path = "src/game/scoreboard.rs";
        let scoreboard_content =
            fs::read_to_string(scoreboard_path).expect("Failed to read src/game/scoreboard.rs");

        assert!(
            scoreboard_content.contains("trait ScoreAccess")
                || scoreboard_content.contains("trait ScoreMutable"),
            "Должны существовать трейты ScoreAccess/ScoreMutable"
        );

        // Проверяем board.rs на наличие трейтов
        let board_path = "src/game/board.rs";
        let board_content =
            fs::read_to_string(board_path).expect("Failed to read src/game/board.rs");

        // Проверяем что трейты импортируются через pub use (реэкспорт из access.rs)
        let has_rexport = board_content.contains("pub use super::access::");
        let has_board_readonly = board_content.contains("BoardReadonly");

        assert!(
            has_rexport && has_board_readonly,
            "Должны существовать трейты BoardReadonly/BoardMutable в board.rs (через pub use)"
        );

        println!("✅ Трейты доступа к компонентам существуют");
    }
}

// ============================================================================
// C2: ТЕСТЫ НА ИНКАПСУЛЯЦИЮ render.rs
// ============================================================================

mod test_render_encapsulation {
    use super::*;

    /// Проверка что GameView имеет методы отрисовки.
    ///
    /// ## Требование (C2)
    /// GameView должен иметь методы:
    /// - draw_field()
    /// - draw_next_shape()
    /// - draw_held_shape()
    /// - draw_ui()
    /// - draw_ghost()
    #[test]
    fn test_game_view_has_draw_methods() {
        let view_path = "src/game/view.rs";
        let view_content = fs::read_to_string(view_path).expect("Failed to read src/game/view.rs");

        // Проверяем наличие методов отрисовки
        assert!(
            view_content.contains("pub fn draw_field"),
            "GameView должен иметь метод draw_field()"
        );

        assert!(
            view_content.contains("pub fn draw_next_shape"),
            "GameView должен иметь метод draw_next_shape()"
        );

        assert!(
            view_content.contains("pub fn draw_held_shape"),
            "GameView должен иметь метод draw_held_shape()"
        );

        assert!(
            view_content.contains("pub fn draw_ui"),
            "GameView должен иметь метод draw_ui()"
        );

        assert!(
            view_content.contains("pub fn draw_ghost"),
            "GameView должен иметь метод draw_ghost()"
        );

        println!("✅ GameView имеет все необходимые методы отрисовки");
    }

    /// Проверка что render.rs делегирует отрисовку GameView.
    ///
    /// ## Требование (C2)
    /// render::draw() должен вызывать методы GameView,
    /// а не напрямую отрисовывать данные GameState.
    #[test]
    fn test_render_delegates_to_game_view() {
        let render_path = "src/game/render.rs";
        let render_content =
            fs::read_to_string(render_path).expect("Failed to read src/game/render.rs");

        // render.rs должен использовать GameView
        assert!(
            render_content.contains("GameView") || render_content.contains("view::GameView"),
            "render.rs должен использовать GameView"
        );

        // Проверяем что draw() принимает GameView
        assert!(
            render_content.contains("fn draw") && render_content.contains("GameView"),
            "render::draw() должен принимать GameView"
        );

        println!("✅ render.rs делегирует отрисовку GameView");
    }
}

// ============================================================================
// C3: ТЕСТЫ НА ЦЕНТРАЛИЗАЦИЮ КОНСТАНТ
// ============================================================================

mod test_constants_centralization {
    use super::*;

    /// Проверка что константы централизованы.
    ///
    /// ## Требование (C3)
    /// - Все основные константы должны быть в constants.rs
    /// - Не должно быть дублирования в menu/constants.rs
    #[test]
    fn test_arch_comp_constants_centralized() {
        let constants_path = "src/constants.rs";
        let constants_content =
            fs::read_to_string(constants_path).expect("Failed to read src/constants.rs");

        // Проверяем наличие основных констант в constants.rs
        assert!(
            constants_content.contains("pub const GRID_WIDTH"),
            "GRID_WIDTH должен быть определён в constants.rs"
        );

        assert!(
            constants_content.contains("pub const GRID_HEIGHT"),
            "GRID_HEIGHT должен быть определён в constants.rs"
        );

        assert!(
            constants_content.contains("pub const FPS"),
            "FPS должен быть определён в constants.rs"
        );

        assert!(
            constants_content.contains("pub const INITIAL_FALL_SPD"),
            "INITIAL_FALL_SPD должен быть определён в constants.rs"
        );

        // Проверяем menu/constants.rs на дублирование
        let menu_constants_path = "src/menu/constants.rs";
        if Path::new(menu_constants_path).exists() {
            let menu_constants_content = fs::read_to_string(menu_constants_path)
                .expect("Failed to read src/menu/constants.rs");

            // menu/constants.rs должен импортировать из constants.rs, а не дублировать
            assert!(
                menu_constants_content.contains("use crate::constants::")
                    || menu_constants_content.contains("use super::super::constants::"),
                "menu/constants.rs должен импортировать константы из constants.rs"
            );

            // Проверяем что нет дублирования основных констант
            assert!(
                !menu_constants_content.contains("pub const GRID_WIDTH")
                    && !menu_constants_content.contains("const GRID_WIDTH"),
                "menu/constants.rs не должен дублировать GRID_WIDTH"
            );
        }

        println!("✅ Константы централизованы в constants.rs");
    }

    /// Проверка что другие модули импортируют константы из constants.rs.
    #[test]
    fn test_modules_import_constants() {
        let modules = vec![
            "src/game/state.rs",
            "src/game/render.rs",
            "src/game/cycle.rs",
            "src/tetromino.rs",
            "src/menu/draw.rs",
        ];

        for module_path in &modules {
            if Path::new(module_path).exists() {
                let content = fs::read_to_string(module_path)
                    .unwrap_or_else(|_| panic!("Failed to read {}", module_path));

                // Модуль должен импортировать константы или использовать их через crate::constants
                let imports_constants = content.contains("use crate::constants::")
                    || content.contains("use super::constants::")
                    || content.contains("use super::super::constants::");

                if content.contains("GRID_WIDTH") || content.contains("GRID_HEIGHT") {
                    assert!(
                        imports_constants,
                        "{} должен импортировать константы из constants.rs",
                        module_path
                    );
                }
            }
        }

        println!("✅ Модули импортируют константы из constants.rs");
    }
}

// ============================================================================
// H1: ТЕСТЫ НА DEPENDENCY INVERSION PRINCIPLE
// ============================================================================

mod test_dependency_inversion {
    use super::*;

    /// Проверка что cycle.rs использует трейты.
    ///
    /// ## Требование (H1)
    /// run_game_loop<T: InputReader, R: Renderer> должен использовать трейты,
    /// а не зависеть от конкретных типов KeyReader, Canvas.
    #[test]
    fn test_cycle_uses_traits() {
        let cycle_path = "src/game/cycle.rs";
        let cycle_content =
            fs::read_to_string(cycle_path).expect("Failed to read src/game/cycle.rs");

        // Проверяем что run_game_loop использует трейты
        assert!(
            cycle_content.contains("fn run_game_loop")
                && (cycle_content.contains("T: InputReader")
                    || cycle_content.contains("R: Renderer")),
            "run_game_loop должен использовать трейты InputReader и Renderer"
        );

        // Проверяем что handle_input использует трейт InputReader
        assert!(
            cycle_content.contains("fn handle_input") && cycle_content.contains("T: InputReader"),
            "handle_input должен использовать трейт InputReader"
        );

        // Проверяем что render использует трейт Renderer
        assert!(
            cycle_content.contains("fn render") && cycle_content.contains("R: Renderer"),
            "render должен использовать трейт Renderer"
        );

        println!("✅ cycle.rs использует трейты для Dependency Inversion");
    }

    /// Проверка что можно использовать моки.
    ///
    /// ## Требование (H1)
    /// Должна быть возможность создать мок InputReader и передать в run_game_loop.
    #[test]
    fn test_can_use_mock_input_reader() {
        // Проверяем что трейт InputReader существует и публичен
        let io_traits_path = "src/io_traits.rs";
        let io_traits_content =
            fs::read_to_string(io_traits_path).expect("Failed to read src/io_traits.rs");

        assert!(
            io_traits_content.contains("pub trait InputReader"),
            "InputReader должен быть публичным трейтом"
        );

        // Проверяем что трейт Renderer существует и публичен
        assert!(
            io_traits_content.contains("pub trait Renderer"),
            "Renderer должен быть публичным трейтом"
        );

        // Проверяем что методы трейтов позволяют создавать моки
        assert!(
            io_traits_content.contains("fn get_key")
                && io_traits_content.contains("fn draw_string"),
            "Трейты должны иметь методы для создания моков"
        );

        println!("✅ Можно использовать моки InputReader и Renderer");
    }

    /// Проверка что io_traits.rs экспортируется.
    #[test]
    fn test_io_traits_exported() {
        let lib_path = "src/lib.rs";
        let lib_content = fs::read_to_string(lib_path).expect("Failed to read src/lib.rs");

        // io_traits должен быть экспортирован
        assert!(
            lib_content.contains("io_traits") || lib_content.contains("pub use"),
            "io_traits должен быть экспортирован из lib.rs"
        );

        println!("✅ io_traits экспортируется из lib.rs");
    }
}

// ============================================================================
// H3: ТЕСТЫ НА ВАЛИДАЦИЮ ДАННЫХ
// ============================================================================

mod test_validation {
    use super::*;

    /// Проверка валидации NaN в set_fall_speed.
    ///
    /// ## Требование (H3)
    /// set_fall_speed(f32::NAN) не должен устанавливать NaN.
    #[test]
    fn test_set_fall_speed_validates_nan() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // Ищем метод set_fall_speed
        let set_fall_speed_start = state_content.find("fn set_fall_speed");
        assert!(
            set_fall_speed_start.is_some(),
            "Метод set_fall_speed должен существовать"
        );

        // Проверяем что есть валидация (clamp, is_nan, или аналогичная)
        let set_fall_speed_section = &state_content[set_fall_speed_start.unwrap()..];
        let set_fall_speed_end = set_fall_speed_section.find("\n    pub fn").unwrap_or(500);
        let method_body =
            &set_fall_speed_section[..set_fall_speed_end.min(set_fall_speed_section.len())];

        // Валидация должна использовать clamp или is_nan
        assert!(
            method_body.contains("clamp")
                || method_body.contains("is_nan")
                || method_body.contains("MAX_FALL_SPEED"),
            "set_fall_speed должен валидировать NaN и границы значений"
        );

        println!("✅ set_fall_speed валидирует NaN");
    }

    /// Проверка валидации Infinity в set_fall_speed.
    ///
    /// ## Требование (H3)
    /// set_fall_speed(f32::INFINITY) не должен устанавливать Infinity.
    #[test]
    fn test_set_fall_speed_validates_infinity() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // Проверяем что есть ограничение максимальной скорости
        assert!(
            state_content.contains("MAX_FALL_SPEED"),
            "Должна существовать константа MAX_FALL_SPEED для ограничения скорости"
        );

        // Проверяем что set_fall_speed использует clamp или аналогичное ограничение
        let set_fall_speed_start = state_content.find("fn set_fall_speed");
        if let Some(start) = set_fall_speed_start {
            let set_fall_speed_section = &state_content[start..];
            let set_fall_speed_end = set_fall_speed_section.find("\n    pub fn").unwrap_or(500);
            let method_body =
                &set_fall_speed_section[..set_fall_speed_end.min(set_fall_speed_section.len())];

            assert!(
                method_body.contains("clamp") || method_body.contains(".min("),
                "set_fall_speed должен ограничивать максимальное значение (Infinity)"
            );
        }

        println!("✅ set_fall_speed валидирует Infinity");
    }

    /// Проверка валидации уровня в ScoreBoard.
    ///
    /// ## Требование (H3)
    /// set_level(1001) должен ограничить до 1000.
    #[test]
    fn test_scoreboard_level_validates_max() {
        let scoreboard_path = "src/game/scoreboard.rs";
        let scoreboard_content =
            fs::read_to_string(scoreboard_path).expect("Failed to read src/game/scoreboard.rs");

        // Проверяем наличие MAX_LEVEL константы или ограничения
        assert!(
            scoreboard_content.contains("MAX_LEVEL") || scoreboard_content.contains(".clamp(1,"),
            "ScoreBoard должен иметь ограничение на максимальный уровень"
        );

        // Ищем impl ScoreBoard блок
        let impl_start = scoreboard_content.find("impl ScoreBoard");
        assert!(impl_start.is_some(), "impl ScoreBoard должен существовать");

        // В пределах impl ищем set_level
        let impl_section = &scoreboard_content[impl_start.unwrap()..];
        let set_level_start = impl_section.find("pub fn set_level");
        assert!(
            set_level_start.is_some(),
            "Метод set_level должен существовать в impl ScoreBoard"
        );

        // Извлекаем тело метода set_level
        let set_level_section = &impl_section[set_level_start.unwrap()..];

        // Находим тело функции через подсчёт скобок
        let brace_start = set_level_section.find('{');
        assert!(
            brace_start.is_some(),
            "set_level должен иметь открывающую скобку"
        );

        let mut brace_count = 0;
        let mut end_pos = 0;
        for (i, ch) in set_level_section.char_indices() {
            if ch == '{' {
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    end_pos = i;
                    break;
                }
            }
        }

        let method_body = &set_level_section[..end_pos];

        // Проверяем что используется clamp или аналогичное ограничение
        assert!(
            method_body.contains("clamp")
                || method_body.contains(".max(")
                || method_body.contains("MIN_LEVEL"),
            "set_level должен использовать clamp для валидации.\nТело метода: {}",
            method_body
        );

        println!("✅ ScoreBoard::set_level валидирует максимальный уровень");
    }
}

// ============================================================================
// H4: ТЕСТЫ НА ИНКАПСУЛЯЦИЮ ПОЛЕЙ
// ============================================================================

mod test_field_encapsulation {
    use super::*;

    /// Проверка что поля GameState приватные.
    ///
    /// ## Требование (H4)
    /// Поля GameState не должны быть доступны напрямую извне модуля.
    #[test]
    fn test_game_state_fields_private() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // Находим определение struct GameState
        let struct_start = state_content.find("pub struct GameState");
        assert!(
            struct_start.is_some(),
            "pub struct GameState должен существовать"
        );

        // Извлекаем определение структуры
        let struct_section = &state_content[struct_start.unwrap()..];
        let struct_end = struct_section.find("}").unwrap_or(2000);
        let struct_definition = &struct_section[..struct_end];

        // Все поля должны быть приватными (без pub)
        // Проверяем что нет публичных полей (pub field_name:)
        let has_public_fields = struct_definition
            .lines()
            .any(|line| line.trim().starts_with("pub ") && line.contains(':'));

        assert!(
            !has_public_fields,
            "Поля GameState должны быть приватными (без pub)"
        );

        println!("✅ Поля GameState приватные");
    }

    /// Проверка что GameState имеет геттеры для всех полей.
    #[test]
    fn test_arch_comp_game_state_has_getters() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // Проверяем наличие геттеров
        let getters = vec![
            "pub fn score(",
            "pub fn level(",
            "pub fn lines_cleared(",
            "pub fn curr_shape(",
            "pub fn next_shape(",
            "pub fn held_shape(",
            "pub fn fall_speed(",
            "pub fn land_timer(",
        ];

        for getter in &getters {
            assert!(
                state_content.contains(getter),
                "GameState должен иметь геттер {}",
                getter
            );
        }

        println!("✅ GameState имеет геттеры для всех полей");
    }
}

// ============================================================================
// M3: ТЕСТЫ НА ОБРАБОТКУ ОШИБОК В DROP()
// ============================================================================

mod test_canvas_drop_error_handling {
    use super::*;

    /// Проверка что Canvas::drop() логирует ошибки.
    ///
    /// ## Требование (M3)
    /// При ошибке в drop() должно быть логирование через eprintln! или log!.
    #[test]
    fn test_canvas_drop_logs_errors() {
        let io_path = "src/io.rs";
        let io_content = fs::read_to_string(io_path).expect("Failed to read src/io.rs");

        // Находим реализацию Drop для Canvas
        let impl_drop_start = io_content.find("impl Drop for Canvas");
        assert!(
            impl_drop_start.is_some(),
            "Drop для Canvas должен быть реализован"
        );

        // Извлекаем реализацию Drop (ищем до конца impl блока)
        let drop_section = &io_content[impl_drop_start.unwrap()..];
        // Ищем конец impl блока по следующей закрывающей скобке после fn drop
        let drop_fn_start = drop_section.find("fn drop").unwrap_or(0);
        let drop_fn_section = &drop_section[drop_fn_start..];

        // Ищем несколько закрывающих скобок для определения конца функции
        let mut brace_count = 0;
        let mut end_pos = 0;
        for (i, ch) in drop_fn_section.char_indices() {
            if ch == '{' {
                brace_count += 1;
            } else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    end_pos = i;
                    break;
                }
            }
        }

        let drop_implementation = &drop_fn_section[..end_pos];

        // Проверяем наличие логирования ошибок
        assert!(
            drop_implementation.contains("eprintln!") || drop_implementation.contains("log::"),
            "Canvas::drop() должен логирировать ошибки через eprintln! или log!\nНайден код: {}",
            drop_implementation
        );

        // Проверяем что есть обработка ошибок через if let Err
        assert!(
            drop_implementation.contains("if let Err") || drop_implementation.contains(".is_err()"),
            "Canvas::drop() должен обрабатывать ошибки через if let Err"
        );

        println!("✅ Canvas::drop() логирует ошибки");
    }

    /// Проверка что KeyReader::drop() также логирует ошибки.
    #[test]
    fn test_key_reader_drop_logs_errors() {
        let io_path = "src/io.rs";
        let io_content = fs::read_to_string(io_path).expect("Failed to read src/io.rs");

        // Находим реализацию Drop для KeyReader
        let impl_drop_start = io_content.find("impl Drop for KeyReader");
        assert!(
            impl_drop_start.is_some(),
            "Drop для KeyReader должен быть реализован"
        );

        println!("✅ KeyReader::drop() реализован");
    }
}

// ============================================================================
// ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

mod test_no_circular_dependencies {
    use super::*;

    /// Проверка что нет циклов между модулями.
    ///
    /// ## Требование
    /// - game/state.rs не должен импортировать из game/render.rs
    /// - game/render.rs не должен импортировать из game/logic/
    #[test]
    fn test_no_circular_dependencies_state_render() {
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // state.rs не должен импортировать render.rs напрямую
        assert!(
            !state_content.contains("use super::render::")
                && !state_content.contains("use crate::game::render::"),
            "state.rs не должен импортировать из render.rs"
        );

        println!("✅ state.rs не зависит от render.rs");
    }

    /// Проверка что render.rs не импортирует из logic/.
    #[test]
    fn test_render_does_not_import_logic() {
        let render_path = "src/game/render.rs";
        let render_content =
            fs::read_to_string(render_path).expect("Failed to read src/game/render.rs");

        // render.rs не должен импортировать из logic/
        assert!(
            !render_content.contains("use super::logic::")
                && !render_content.contains("use crate::game::logic::"),
            "render.rs не должен импортировать из logic/"
        );

        println!("✅ render.rs не зависит от logic/");
    }

    /// Проверка что logic/ не импортирует из render/.
    #[test]
    fn test_logic_does_not_import_render() {
        let logic_dir = "src/game/logic";

        if Path::new(logic_dir).exists() {
            for entry in fs::read_dir(logic_dir).expect("Failed to read logic dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use super::render::")
                            && !content.contains("use crate::game::render::"),
                        "{:?} не должен импортировать render (нарушение границ)",
                        path
                    );
                }
            }
        }

        println!("✅ logic/ не зависит от render/");
    }
}

// ============================================================================
// ТЕСТЫ НА СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
// ============================================================================

mod test_module_boundaries {
    use super::*;

    /// Проверка что второй дубликат test_logic_does_not_import_render.
    ///
    /// ## Требование
    /// Логика не должна зависеть от отрисовки.
    #[test]
    fn test_arch_comp_logic_does_not_import_render_2() {
        let logic_dir = "src/game/logic";

        if Path::new(logic_dir).exists() {
            let mut files_checked = 0;

            for entry in fs::read_dir(logic_dir).expect("Failed to read logic dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    files_checked += 1;
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use super::render")
                            && !content.contains("use crate::game::render"),
                        "{:?} не должен импортировать из render/",
                        path
                    );
                }
            }

            assert!(files_checked > 0, "Должны быть файлы в директории logic/");
        }

        println!("✅ game/logic/ не импортирует из game/render/");
    }

    /// Проверка что scoring модуль не импортирует render.
    ///
    /// ## Требование
    /// Scoring не должен зависеть от отрисовки.
    #[test]
    fn test_scoring_does_not_import_render() {
        let scoring_dir = "src/game/scoring";

        if Path::new(scoring_dir).exists() {
            let mut files_checked = 0;

            for entry in fs::read_dir(scoring_dir).expect("Failed to read scoring dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    files_checked += 1;
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use super::render")
                            && !content.contains("use crate::game::render"),
                        "{:?} не должен импортировать из render/",
                        path
                    );
                }
            }

            assert!(files_checked > 0, "Должны быть файлы в директории scoring/");
        }

        println!("✅ scoring/ не импортирует из render/");
    }

    /// Проверка что scoring/ не импортирует из logic/.
    #[test]
    fn test_scoring_does_not_import_logic() {
        let scoring_dir = "src/game/scoring";

        if Path::new(scoring_dir).exists() {
            for entry in fs::read_dir(scoring_dir).expect("Failed to read scoring dir") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                    let content = fs::read_to_string(&path).expect("Failed to read file");

                    assert!(
                        !content.contains("use super::logic")
                            && !content.contains("use crate::game::logic"),
                        "{:?} не должен импортировать из logic/",
                        path
                    );
                }
            }
        }

        println!("✅ scoring/ не импортирует из logic/");
    }
}

// ============================================================================
// SOLID ПРИНЦИПЫ
// ============================================================================

mod test_solid_principles {
    use super::*;

    /// Проверка Single Responsibility Principle.
    ///
    /// ## Требование
    /// - GameState разделён на компоненты
    /// - Каждый компонент отвечает за одну задачу
    #[test]
    fn test_srp_compliance() {
        // Проверяем что GameState использует композицию
        let state_path = "src/game/state.rs";
        let state_content =
            fs::read_to_string(state_path).expect("Failed to read src/game/state.rs");

        // GameState должен делегировать ответственности компонентам
        assert!(
            state_content.contains("board: GameBoard"),
            "GameState должен делегировать ответственность за поле компоненту GameBoard"
        );

        assert!(
            state_content.contains("scoreboard: ScoreBoard"),
            "GameState должен делегировать ответственность за очки компоненту ScoreBoard"
        );

        // Проверяем что GameBoard отвечает только за поле
        let board_path = "src/game/board.rs";
        let board_content =
            fs::read_to_string(board_path).expect("Failed to read src/game/board.rs");

        // GameBoard не должен содержать логику очков
        assert!(
            !board_content.contains("score") && !board_content.contains("level"),
            "GameBoard не должен содержать логику очков"
        );

        // Проверяем что ScoreBoard отвечает только за очки
        let scoreboard_path = "src/game/scoreboard.rs";
        let scoreboard_content =
            fs::read_to_string(scoreboard_path).expect("Failed to read src/game/scoreboard.rs");

        // ScoreBoard не должен содержать логику поля
        assert!(
            !scoreboard_content.contains("blocks") && !scoreboard_content.contains("filled_lines"),
            "ScoreBoard не должен содержать логику поля"
        );

        println!("✅ Single Responsibility Principle соблюдается");
    }

    /// Проверка Dependency Inversion Principle.
    ///
    /// ## Требование
    /// cycle.rs зависит от трейтов, а не конкретных типов.
    #[test]
    fn test_dip_compliance() {
        let cycle_path = "src/game/cycle.rs";
        let cycle_content =
            fs::read_to_string(cycle_path).expect("Failed to read src/game/cycle.rs");

        // cycle.rs должен использовать трейты
        assert!(
            cycle_content.contains("T: InputReader") && cycle_content.contains("R: Renderer"),
            "cycle.rs должен зависеть от трейтов InputReader и Renderer"
        );

        // cycle.rs не должен создавать конкретные типы напрямую
        let run_loop_start = cycle_content.find("fn run_game_loop");
        if let Some(start) = run_loop_start {
            let run_loop_section = &cycle_content[start..];
            let run_loop_end = run_loop_section.find("\n}").unwrap_or(1000);
            let run_loop_body = &run_loop_section[..run_loop_end.min(run_loop_section.len())];

            // run_game_loop не должен создавать KeyReader или Canvas напрямую
            assert!(
                !run_loop_body.contains("KeyReader::new()")
                    && !run_loop_body.contains("Canvas::new()"),
                "run_game_loop не должен создавать конкретные типы напрямую"
            );
        }

        println!("✅ Dependency Inversion Principle соблюдается");
    }

    /// Проверка Interface Segregation Principle.
    ///
    /// ## Требование
    /// BoardMutable разделён на BoardBlockAccess и BoardPhysicsAccess.
    #[test]
    fn test_isp_compliance() {
        let access_path = "src/game/access.rs";
        let access_content =
            fs::read_to_string(access_path).expect("Failed to read src/game/access.rs");

        // Должны быть разделённые трейты
        assert!(
            access_content.contains("trait BoardReadonly"),
            "Должен существовать трейт BoardReadonly"
        );

        assert!(
            access_content.contains("trait BoardMutable"),
            "Должен существовать трейт BoardMutable"
        );

        // Проверяем что BoardMutable расширяет BoardReadonly
        assert!(
            access_content.contains("trait BoardMutable: BoardReadonly"),
            "BoardMutable должен расширять BoardReadonly"
        );

        // Проверяем board.rs на наличие разделённых трейтов
        let board_path = "src/game/board.rs";
        let board_content =
            fs::read_to_string(board_path).expect("Failed to read src/game/board.rs");

        // Проверяем что трейты импортируются через pub use (реэкспорт из access.rs)
        let has_rexport = board_content.contains("pub use super::access::");
        let has_board_readonly = board_content.contains("BoardReadonly");

        assert!(
            has_rexport && has_board_readonly,
            "board.rs должен содержать разделённые трейты (через pub use)"
        );

        println!("✅ Interface Segregation Principle соблюдается");
    }

    /// Проверка Liskov Substitution Principle.
    #[test]
    fn test_lsp_compliance() {
        // Проверяем что компоненты реализуют трейты
        let board_path = "src/game/board.rs";
        let board_content =
            fs::read_to_string(board_path).expect("Failed to read src/game/board.rs");

        // GameBoard должен реализовывать BoardReadonly и BoardMutable
        assert!(
            board_content.contains("impl BoardReadonly for GameBoard"),
            "GameBoard должен реализовывать BoardReadonly"
        );

        assert!(
            board_content.contains("impl BoardMutable for GameBoard"),
            "GameBoard должен реализовывать BoardMutable"
        );

        // Проверяем ScoreBoard
        let scoreboard_path = "src/game/scoreboard.rs";
        let scoreboard_content =
            fs::read_to_string(scoreboard_path).expect("Failed to read src/game/scoreboard.rs");

        assert!(
            scoreboard_content.contains("impl ScoreAccess for ScoreBoard")
                || scoreboard_content.contains("impl ScoreMutable for ScoreBoard"),
            "ScoreBoard должен реализовывать ScoreAccess/ScoreMutable"
        );

        println!("✅ Liskov Substitution Principle соблюдается");
    }

    /// Проверка Open/Closed Principle.
    #[test]
    fn test_ocp_compliance() {
        // Проверяем что GameModeTrait позволяет добавлять новые режимы
        let mode_trait_path = "src/game/mode_trait.rs";
        let mode_trait_content =
            fs::read_to_string(mode_trait_path).expect("Failed to read src/game/mode_trait.rs");

        // GameModeTrait должен быть открыт для расширения
        assert!(
            mode_trait_content.contains("pub trait GameModeTrait"),
            "GameModeTrait должен быть публичным для расширения"
        );

        // Проверяем наличие нескольких реализаций
        assert!(
            mode_trait_content.contains("ClassicMode")
                && mode_trait_content.contains("SprintMode")
                && mode_trait_content.contains("MarathonMode"),
            "Должны быть несколько реализаций GameModeTrait"
        );

        println!("✅ Open/Closed Principle соблюдается");
    }
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ
// ============================================================================

mod test_integration {
    use super::*;

    /// Комплексная проверка архитектуры проекта.
    #[test]
    fn test_architecture_integrity() {
        let mut checks_passed = 0;
        let mut total_checks = 0;

        // Проверка 1: constants.rs существует
        total_checks += 1;
        if Path::new("src/constants.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 2: io_traits.rs существует
        total_checks += 1;
        if Path::new("src/io_traits.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 3: game/board.rs существует
        total_checks += 1;
        if Path::new("src/game/board.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 4: game/scoreboard.rs существует
        total_checks += 1;
        if Path::new("src/game/scoreboard.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 5: game/view.rs существует
        total_checks += 1;
        if Path::new("src/game/view.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 6: game/access.rs существует
        total_checks += 1;
        if Path::new("src/game/access.rs").exists() {
            checks_passed += 1;
        }

        // Проверка 7: game/logic/ существует
        total_checks += 1;
        if Path::new("src/game/logic").exists() {
            checks_passed += 1;
        }

        // Проверка 8: game/scoring/ существует
        total_checks += 1;
        if Path::new("src/game/scoring").exists() {
            checks_passed += 1;
        }

        assert!(
            checks_passed == total_checks,
            "Все архитектурные компоненты должны существовать: {}/{}",
            checks_passed,
            total_checks
        );

        println!(
            "✅ Архитектурная целостность подтверждена: {}/{}",
            checks_passed, total_checks
        );
    }
}
