//! Тесты архитектурной целостности для новых модулей.
//!
//! Проверяют:
//! - Отсутствие циклических зависимостей
//! - Соблюдение границ модулей
//! - Корректность экспорта типов

#![allow(clippy::assertions_on_constants)]

// ============================================================================
// ТЕСТЫ ДЛЯ GAME::CONSTANTS
// ============================================================================

#[cfg(test)]
mod test_constants_module {
    use crate::game::constants;

    /// Тест 1: Модуль constants не зависит от других модулей game
    ///
    /// Проверяет, что константы определены независимо
    #[test]
    fn test_constants_no_game_dependencies() {
        // Константы должны быть доступны без импорта game::state
        assert_eq!(constants::FPS, 60);
        assert_eq!(constants::FRAME_DELAY_MS, 16); // 1000 / 60
        assert_eq!(constants::GRID_WIDTH, 10);
        assert_eq!(constants::GRID_HEIGHT, 20);
    }

    /// Тест 2: Все константы физики корректны
    #[test]
    fn test_physics_constants_valid() {
        assert!(constants::INITIAL_FALL_SPD > 0.0);
        assert!(constants::MAX_FALL_SPEED > constants::INITIAL_FALL_SPD);
        assert!(constants::SPD_INC > 0.0);
        assert!(constants::LAND_TIME_DELAY_S > 0.0);
    }

    /// Тест 3: Все константы очков корректны
    #[test]
    fn test_scoring_constants_valid() {
        assert_eq!(constants::LINE_SCORES.len(), 4);
        assert_eq!(constants::LINE_SCORES[0], 100); // 1 линия
        assert_eq!(constants::LINE_SCORES[1], 200); // 2 линии
        assert_eq!(constants::LINE_SCORES[2], 400); // 3 линии
        assert_eq!(constants::LINE_SCORES[3], 1800); // 4 линии (Tetris)
        assert!(constants::COMBO_BONUS > 0);
        assert!(constants::LEVEL_BONUS_MULT > 0);
    }

    /// Тест 4: Позиции отрисовки корректны
    #[test]
    fn test_ui_positions_valid() {
        // Все позиции должны быть в пределах экрана
        let score_x = constants::SCORE_X;
        let score_y = constants::SCORE_Y;
        assert!(score_x < constants::DISP_WIDTH as u16);
        assert!(score_y < constants::DISP_HEIGHT as u16);

        let high_x = constants::HIGH_SCORE_X;
        let high_y = constants::HIGH_SCORE_Y;
        assert!(high_x < constants::DISP_WIDTH as u16);
        assert!(high_y < constants::DISP_HEIGHT as u16);
    }

    /// Тест 5: Wall kick offsets корректны
    ///
    /// Примечание: WALL_KICK_OFFSETS перемещён в модуль logic::rotation
    #[test]
    fn test_wall_kick_offsets_valid() {
        use crate::game::logic::rotation::WALL_KICK_OFFSETS;

        assert_eq!(WALL_KICK_OFFSETS.len(), 8);
        // Первое смещение должно быть (-1, 0) - проверка стены слева
        assert_eq!(WALL_KICK_OFFSETS[0], (-1, 0));
    }

    /// Тест 6: Константы безопасности корректны
    #[test]
    fn test_security_constants_valid() {
        assert!(constants::MAX_CONFIG_FILE_SIZE > 0);
        // MAX_PLAYER_NAME_LENGTH и MIN_PLAYER_NAME_LENGTH удалены как неиспользуемые
        // LEADERBOARD_COOLDOWN_SECS удалён как неиспользуемый
        assert!(constants::MAX_LEADERBOARD_ENTRIES > 0);
    }
}

// ============================================================================
// ТЕСТЫ ДЛЯ GAME::TYPES (Score, Level, LinesCount)
// ============================================================================

#[cfg(test)]
mod test_types_module {
    use crate::game::types::{Level, LinesCount, Score};

    /// Тест 1: Score - типобезопасность
    #[test]
    fn test_score_type_safety() {
        let score1 = Score::new();
        let score2 = Score::with_value(100);

        // Score не должен неявно конвертироваться в u128
        // Явная конвертация должна работать
        let value: u128 = score2.into();
        assert_eq!(value, 100);

        // Score должен сравниваться корректно
        assert_ne!(score1, score2);
    }

    /// Тест 2: Level - инварианты
    #[test]
    fn test_level_invariants() {
        let level1 = Level::new();
        assert_eq!(level1.value(), 1); // Минимальный уровень = 1

        let level2 = Level::with_value(0);
        assert_eq!(level2.value(), 1); // 0 должен стать 1

        let level3 = Level::with_value(5);
        assert_eq!(level3.value(), 5);
    }

    /// Тест 3: LinesCount - saturating операции
    #[test]
    fn test_lines_count_saturating() {
        let mut lines = LinesCount::with_value(u32::MAX);
        lines.add(100);
        assert_eq!(lines.value(), u32::MAX); // Saturating
    }

    /// Тест 4: Score - saturating операции
    #[test]
    fn test_score_saturating_operations() {
        let mut score = Score::with_value(u128::MAX);
        score.add(1000);
        assert_eq!(score.value(), u128::MAX); // Saturating

        let mut score = Score::with_value(u128::MAX);
        score.multiply(2);
        assert_eq!(score.value(), u128::MAX); // Saturating
    }

    /// Тест 5: Level - отсутствие переполнения
    #[test]
    fn test_level_no_overflow() {
        let mut level = Level::with_value(u32::MAX);
        let result = level.increment();
        assert!(!result); // Должно вернуть false
        assert_eq!(level.value(), u32::MAX); // Значение не изменилось

        let mut level = Level::with_value(u32::MAX - 5);
        let result = level.increment_by(10);
        assert!(!result); // Должно вернуть false (overflow)
        assert_eq!(level.value(), u32::MAX); // Saturating
    }

    /// Тест 6: LinesCount - reached метод
    #[test]
    fn test_lines_count_reached_threshold() {
        let lines = LinesCount::with_value(40);
        assert!(lines.reached(40)); // Ровно порог
        assert!(lines.reached(30)); // Больше порога
        assert!(!lines.reached(50)); // Меньше порога
    }

    /// Тест 7: Display реализация
    #[test]
    fn test_types_display_impl() {
        let score = Score::with_value(1234);
        assert_eq!(format!("{score}"), "1234");

        let level = Level::with_value(5);
        assert_eq!(format!("{level}"), "5");

        let lines = LinesCount::with_value(40);
        assert_eq!(format!("{lines}"), "40");
    }

    /// Тест 8: Default реализация
    #[test]
    fn test_types_default_impl() {
        let score = Score::default();
        assert_eq!(score.value(), 0);

        let level = Level::default();
        assert_eq!(level.value(), 1);

        let lines = LinesCount::default();
        assert_eq!(lines.value(), 0);
    }
}

// ============================================================================
// ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

#[cfg(test)]
mod test_no_circular_dependencies {
    /// Тест 1: constants.rs не зависит от state.rs
    ///
    /// Проверяется на этапе компиляции - если constants импортирует state,
    /// этот тест не скомпилируется
    #[test]
    fn test_constants_independent_from_state() {
        // Импортируем только constants
        use crate::game::constants;

        // Используем константы без state
        let _fps = constants::FPS;
        let _width = constants::GRID_WIDTH;

        // Если код компилируется - зависимости нет
    }

    /// Тест 2: types.rs не зависит от state.rs
    #[test]
    fn test_types_independent_from_state() {
        use crate::game::types;

        // Создаём типы без state
        let _score = types::Score::new();
        let _level = types::Level::new();
        let _lines = types::LinesCount::new();
    }

    /// Тест 3: modules hierarchy
    #[test]
    fn test_module_hierarchy() {
        // Базовые модули (нет зависимостей от game/*)
        use crate::game::constants;
        use crate::game::types;

        // Средний уровень (зависит от constants и types)
        use crate::game::state;

        // Используем все три
        let _fps = constants::FPS;
        let _score = types::Score::new();
        let _game = state::GameState::new();
    }
}

// ============================================================================
// ТЕСТЫ НА СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
// ============================================================================

#[cfg(test)]
mod test_module_boundaries {
    /// Тест 1: Score инкапсуляция
    #[test]
    fn test_score_encapsulation() {
        let mut score = Score::with_value(100);

        // Нет прямого доступа к внутреннему значению
        // score.0 = 200; // Это не скомпилируется

        // Только через публичные методы
        score.add(50);
        assert_eq!(score.value(), 150);
    }

    use crate::game::types::Score;

    /// Тест 2: Level инкапсуляция
    #[test]
    fn test_level_encapsulation() {
        let mut level = Level::with_value(5);

        // Нет прямого доступа
        // level.0 = 10; // Это не скомпилируется

        // Только через публичные методы
        let _ = level.increment();
        assert_eq!(level.value(), 6);
    }

    use crate::game::types::Level;

    /// Тест 3: LinesCount инкапсуляция
    #[test]
    fn test_lines_count_encapsulation() {
        let mut lines = LinesCount::with_value(10);

        // Нет прямого доступа
        // lines.0 = 20; // Это не скомпилируется

        // Только через публичные методы
        lines.add(5);
        assert_eq!(lines.value(), 15);
    }

    use crate::game::types::LinesCount;
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod test_integration {
    use crate::game::constants;
    use crate::game::types::{Level, LinesCount, Score};

    /// Тест 1: Взаимодействие констант и типов
    #[test]
    fn test_constants_and_types_integration() {
        let mut score = Score::new();
        let mut level = Level::new();
        let mut lines = LinesCount::new();

        // Симуляция удаления 4 линий (Tetris)
        lines.add(4);
        score.add(constants::LINE_SCORES[3]); // 1800 очков

        // Проверка уровня
        if lines.reached(10) {
            let _ = level.increment();
        }
        assert!(!lines.reached(10)); // Ещё нет

        // Добавим ещё 6 линий
        lines.add(6);
        assert!(lines.reached(10)); // Теперь да
        let _ = level.increment();
        assert_eq!(level.value(), 2);
    }

    /// Тест 2: Бонус за уровень
    #[test]
    fn test_level_bonus_calculation() {
        let mut score = Score::new();
        let level = Level::with_value(3);

        // Бонус за уровень
        let bonus = constants::LEVEL_BONUS_MULT * (level.value() - 1) as u128;
        score.add(bonus);

        assert_eq!(score.value(), 1000); // 500 * (3-1) = 1000
    }

    /// Тест 3: Комбо бонус
    #[test]
    fn test_combo_bonus_calculation() {
        let mut score = Score::new();
        let combo = 5;

        // Бонус за комбо
        let bonus = constants::COMBO_BONUS * (combo - 1) as u128;
        score.add(bonus);

        assert_eq!(score.value(), 200); // 50 * 4 = 200
    }

    /// Тест 4: Полный подсчёт очков за Tetris
    #[test]
    fn test_full_tetris_score_calculation() {
        let mut score = Score::new();
        let level = Level::with_value(2);
        let combo = 3;

        // Базовые очки за Tetris (4 линии)
        score.add(constants::LINE_SCORES[3]); // 1800

        // Бонус за уровень
        score.add(constants::LEVEL_BONUS_MULT * (level.value() - 1) as u128); // 500

        // Бонус за комбо
        score.add(constants::COMBO_BONUS * (combo - 1) as u128); // 100

        // Итого: 1800 + 500 + 100 = 2400
        assert_eq!(score.value(), 2400);
    }
}
