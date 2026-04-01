//! Тесты на архитектурную целостность для проекта tetris-cli.
//!
//! Этот файл содержит тесты для проверки:
//! 1. Отсутствия циклических зависимостей между модулями
//! 2. Соблюдения границ модулей
//! 3. Корректности работы выделенных компонентов
//! 4. Отсутствия недопустимых импортов
//!
//! ## Количество тестов: 18

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

#[cfg(test)]
mod tests {
    use std::fs;

    // ========================================================================
    // РАЗДЕЛ 1: ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
    // ========================================================================

    /// Проверка что core/ не импортирует из game/.
    /// Core должен быть независимым базовым модулем.
    #[test]
    fn test_no_cycle_core_game() {
        let core_path = "src/core/mod.rs";
        let core_content = fs::read_to_string(core_path).expect("Failed to read src/core/mod.rs");

        // Core не должен импортировать из game
        assert!(
            !core_content.contains("use crate::game::"),
            "core/mod.rs не должен импортировать из crate::game::\n\
             Это создаст циклическую зависимость"
        );
    }

    /// Проверка что game/components/ не импортирует из game/logic/.
    /// Компоненты должны быть независимы от логики.
    #[test]
    fn test_no_cycle_components_logic() {
        let components = [
            "src/game/components/figure_state.rs",
            "src/game/components/board_state.rs",
            "src/game/components/animation_state.rs",
        ];

        for component_path in components {
            let content = fs::read_to_string(component_path)
                .unwrap_or_else(|_| panic!("Failed to read {component_path}"));

            // Компоненты не должны импортировать из logic напрямую
            assert!(
                !content.contains("use crate::game::logic::"),
                "{component_path} не должен импортировать из crate::game::logic::\n\
                 Компоненты должны быть независимы от логики"
            );
        }
    }

    /// Проверка что io/mod.rs не создаёт циклов.
    /// IO должен зависеть только от core и констант.
    #[test]
    fn test_no_cycle_io_module() {
        let io_path = "src/io/mod.rs";
        let io_content = fs::read_to_string(io_path).expect("Failed to read src/io/mod.rs");

        // IO не должен импортировать из game (кроме констант через crate::constants)
        let lines: Vec<&str> = io_content.lines().collect();
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("use crate::game::") && !trimmed.contains("//") {
                panic!(
                    "io/mod.rs не должен импортировать из crate::game::\n\
                     Нарушение: {trimmed}"
                );
            }
        }
    }

    /// Проверка что tetromino/ не импортирует из game/.
    #[test]
    fn test_no_cycle_tetromino_game() {
        let tetromino_modules = [
            "src/tetromino/mod.rs",
            "src/tetromino/bag_generator.rs",
            "src/tetromino/shape_type.rs",
            "src/tetromino/tetromino_struct.rs",
            "src/tetromino/constants.rs",
        ];

        for module_path in tetromino_modules {
            let content = fs::read_to_string(module_path).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            assert!(
                !content.contains("use crate::game::"),
                "{module_path} не должен импортировать из crate::game::"
            );
        }
    }

    // ========================================================================
    // РАЗДЕЛ 2: ТЕСТЫ НА СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
    // ========================================================================

    /// Проверка что figure_state.rs импортирует только из core/ и tetromino/.
    #[test]
    fn test_figure_state_module_boundaries() {
        let path = "src/game/components/figure_state.rs";
        let content = fs::read_to_string(path).expect("Failed to read figure_state.rs");

        // Разрешённые импорты
        let allowed_imports = ["crate::tetromino", "crate::core", "std::", "super::"];

        // Проверяем что все use соответствуют разрешённым
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ") && !trimmed.contains("//") {
                let is_allowed = allowed_imports.iter().any(|&allowed| {
                    trimmed.contains(allowed) || trimmed.contains("crate::types::")
                });

                // Исключаем тестовые импорты
                let is_test = trimmed.contains("#[cfg(test)]");

                if !is_allowed && !is_test && trimmed.contains("use crate::") {
                    // Проверяем что это не ре-экспорт
                    assert!(
                        trimmed.contains("crate::tetromino::")
                            || trimmed.contains("crate::core::")
                            || trimmed.contains("crate::types::"),
                        "figure_state.rs содержит запрещённый импорт: {trimmed}\n\
                         Разрешены только: core/, tetromino/, types/"
                    );
                }
            }
        }
    }

    /// Проверка что board_state.rs импортирует только из core/ и game/board.rs.
    #[test]
    fn test_board_state_module_boundaries() {
        let path = "src/game/components/board_state.rs";
        let content = fs::read_to_string(path).expect("Failed to read board_state.rs");

        // board_state должен импортировать GameBoard из game/board
        assert!(
            content.contains("use crate::game::board::GameBoard")
                || content.contains("use super::board::GameBoard"),
            "board_state.rs должен импортировать GameBoard из game/board"
        );

        // Не должен импортировать из logic напрямую
        assert!(
            !content.contains("use crate::game::logic::"),
            "board_state.rs не должен импортировать из crate::game::logic::"
        );
    }

    /// Проверка что animation_state.rs минималистичен.
    #[test]
    fn test_animation_state_minimalistic() {
        let path = "src/game/components/animation_state.rs";
        let content = fs::read_to_string(path).expect("Failed to read animation_state.rs");

        // AnimationState не должен импортировать сложные зависимости
        assert!(
            !content.contains("use crate::game::logic::"),
            "animation_state.rs не должен импортировать из logic"
        );

        assert!(
            !content.contains("use crate::game::render::"),
            "animation_state.rs не должен импортировать из render"
        );

        // Должен содержать только простые структуры данных
        let use_count = content
            .lines()
            .filter(|line| line.trim().starts_with("use crate::"))
            .count();

        assert!(
            use_count <= 2,
            "animation_state.rs должен быть минималистичным (найдено {use_count} импортов из crate::)"
        );
    }

    /// Проверка что render/cache.rs импортирует только из game/state.
    #[test]
    fn test_render_cache_module_boundaries() {
        let path = "src/game/render/cache.rs";
        let content = fs::read_to_string(path).expect("Failed to read render/cache.rs");

        // cache.rs должен импортировать только из state
        assert!(
            content.contains("use super::super::state::GameState"),
            "render/cache.rs должен импортировать GameState из game/state"
        );

        // Не должен импортировать из logic напрямую
        assert!(
            !content.contains("use crate::game::logic::"),
            "render/cache.rs не должен импортировать из logic"
        );
    }

    // ========================================================================
    // РАЗДЕЛ 3: ТЕСТЫ НА ЦЕЛОСТНОСТЬ КОМПОНЕНТОВ
    // ========================================================================

    /// FigureState: создание, доступ к фигурам, hold механика.
    #[test]
    fn test_figure_state_integrity() {
        // Тест создания
        let figure_state = tetris_cli::game::components::FigureState::new();

        // Тест доступа к фигурам
        let curr = figure_state.curr_shape();
        let next = figure_state.next_shape();
        // shape() возвращает ShapeType — просто проверяем что фигуры существуют
        let _ = curr.shape();
        let _ = next.shape();

        // Тест hold механики
        assert!(
            figure_state.held_shape().is_none(),
            "Начальная held_shape должна быть None"
        );
        assert!(
            figure_state.can_hold(),
            "Начальный can_hold должен быть true"
        );

        // Тест setters
        let mut state = tetris_cli::game::components::FigureState::new();
        state.set_can_hold(false);
        assert!(!state.can_hold());
    }

    /// BoardState: создание, доступ к полю, filled_lines.
    #[test]
    fn test_board_state_integrity() {
        // Тест создания
        let board_state = tetris_cli::game::components::BoardState::new();

        // Тест доступа к полю
        let inner = board_state.inner();
        assert_eq!(inner.get_filled_lines_count(), 0, "Начальное поле пустое");

        // Тест filled_lines_mask
        assert_eq!(
            board_state.filled_lines_mask(),
            0,
            "Начальная маска должна быть 0"
        );

        // Тест установки маски
        let mut state = tetris_cli::game::components::BoardState::new();
        state.set_filled_lines_mask(0b1010);
        assert_eq!(state.filled_lines_mask(), 0b1010);
        assert_eq!(state.filled_lines_count(), 2);
    }

    /// AnimationState: создание, управление анимациями.
    #[test]
    fn test_animation_state_integrity() {
        // Тест создания
        let anim_state = tetris_cli::game::components::AnimationState::new();

        // Тест начального состояния
        assert_eq!(anim_state.animating_rows_mask(), 0);
        assert!(!anim_state.is_hard_dropping());
        assert!(!anim_state.has_active_animations());

        // Тест управления анимациями
        let mut state = tetris_cli::game::components::AnimationState::new();
        state.add_row_to_animation(5);
        assert_eq!(state.animating_rows_mask(), 1 << 5);

        state.remove_row_from_animation(5);
        assert_eq!(state.animating_rows_mask(), 0);

        // Тест Hard Drop флага
        state.set_is_hard_dropping(true);
        assert!(state.is_hard_dropping());
        assert!(state.has_active_animations());

        state.set_is_hard_dropping(false);
        assert!(!state.is_hard_dropping());
    }

    /// Тест на независимость FigureState от BoardState.
    #[test]
    fn test_figure_state_independence() {
        let figure_state = tetris_cli::game::components::FigureState::new();
        let board_state = tetris_cli::game::components::BoardState::new();

        // FigureState не должен зависеть от BoardState
        let _ = figure_state.curr_shape();
        let _ = board_state.inner();

        // Оба компонента должны работать независимо
        assert!(figure_state.can_hold());
        assert_eq!(board_state.filled_lines_count(), 0);
    }

    /// Тест на независимость AnimationState от других компонентов.
    #[test]
    fn test_animation_state_independence() {
        let anim_state = tetris_cli::game::components::AnimationState::new();
        let figure_state = tetris_cli::game::components::FigureState::new();

        // AnimationState не должен влиять на FigureState
        assert!(!anim_state.has_active_animations());
        assert!(figure_state.can_hold());
    }

    /// Тест на корректную работу spawn_new_piece.
    #[test]
    fn test_figure_state_spawn_new_piece() {
        let mut state = tetris_cli::game::components::FigureState::new();
        let old_next_shape = state.next_shape().shape();

        state.spawn_new_piece();

        // next_shape должна перейти в curr_shape
        assert_eq!(state.curr_shape().shape(), old_next_shape);
        // can_hold должен сброситься
        assert!(state.can_hold());
    }

    /// Тест на корректную работу clear_filled_lines.
    #[test]
    fn test_board_state_clear_filled_lines() {
        let mut state = tetris_cli::game::components::BoardState::new();

        // Устанавливаем заполненные линии
        state.set_filled_lines_mask(0b1111);
        assert_eq!(state.filled_lines_count(), 4);

        // Очищаем линии
        let cleared = state.clear_filled_lines();
        assert_eq!(cleared, 4);
        assert_eq!(state.filled_lines_mask(), 0);
        assert_eq!(state.filled_lines_count(), 0);
    }

    // ========================================================================
    // РАЗДЕЛ 4: ТЕСТЫ НА ОТСУТСТВИЕ НЕДОПУСТИМЫХ ИМПОРТОВ
    // ========================================================================

    /// Проверка что render/cache.rs не импортирует из logic/.
    #[test]
    fn test_no_forbidden_imports_render_cache() {
        let path = "src/game/render/cache.rs";
        let content = fs::read_to_string(path).expect("Failed to read render/cache.rs");

        // cache.rs не должен импортировать из logic
        assert!(
            !content.contains("use crate::game::logic::"),
            "render/cache.rs не должен импортировать из game/logic/"
        );

        assert!(
            !content.contains("use super::logic::"),
            "render/cache.rs не должен импортировать из super::logic::"
        );
    }

    /// Проверка что menu/ не импортирует напрямую из game/state.rs.
    #[test]
    fn test_no_forbidden_imports_menu_game_state() {
        let menu_modules = [
            "src/menu/mod.rs",
            "src/menu/constants.rs",
            "src/menu/draw.rs",
            "src/menu/input.rs",
        ];

        for module_path in menu_modules {
            let content = fs::read_to_string(module_path).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            // menu не должен импортировать GameState напрямую из game/state
            // Допустим импорт через game::GameState (ре-экспорт)
            if content.contains("use crate::game::state::GameState") {
                panic!(
                    "{module_path} не должен импортировать напрямую из game/state.rs\n\
                     Используйте ре-экспорт через game::GameState"
                );
            }
        }
    }

    /// Проверка что game/logic/ не импортирует из render/.
    #[test]
    fn test_no_forbidden_imports_logic_render() {
        let logic_modules = [
            "src/game/logic/mod.rs",
            "src/game/logic/collision.rs",
            "src/game/logic/input.rs",
            "src/game/logic/physics.rs",
            "src/game/logic/rotation.rs",
            "src/game/logic/update.rs",
            "src/game/logic/wall_kick.rs",
        ];

        for module_path in logic_modules {
            let content = fs::read_to_string(module_path).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            assert!(
                !content.contains("use crate::game::render::"),
                "{module_path} не должен импортировать из game/render/"
            );
        }
    }

    /// Проверка что game/scoring/ не импортирует из render/.
    #[test]
    fn test_no_forbidden_imports_scoring_render() {
        let scoring_modules = [
            "src/game/scoring/mod.rs",
            "src/game/scoring/lines.rs",
            "src/game/scoring/points.rs",
            "src/game/scoring/combo.rs",
        ];

        for module_path in scoring_modules {
            let content = fs::read_to_string(module_path).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            assert!(
                !content.contains("use crate::game::render::"),
                "{module_path} не должен импортировать из game/render/"
            );
        }
    }

    /// Проверка что highscore/ не импортирует из game/.
    #[test]
    fn test_no_forbidden_imports_highscore_game() {
        let highscore_modules = [
            "src/highscore/mod.rs",
            "src/highscore/leaderboard.rs",
            "src/highscore/sanitize.rs",
            "src/highscore/save_data.rs",
        ];

        for module_path in highscore_modules {
            let content = fs::read_to_string(module_path).unwrap_or_default();
            if content.is_empty() {
                continue;
            }

            // highscore может импортировать GameState для отображения статистики
            // но не должен импортировать из game/logic или game/render
            assert!(
                !content.contains("use crate::game::logic::"),
                "{module_path} не должен импортировать из game/logic/"
            );

            assert!(
                !content.contains("use crate::game::render::"),
                "{module_path} не должен импортировать из game/render/"
            );
        }
    }
}
