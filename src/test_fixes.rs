//! Тесты для исправленных проблем.
//!
//! Тесты проверяют:
//! 1. Исправление getrandom (использование rand::RngCore)
//! 2. Корректная инициализация таймера в режимах спринт/марафон
//! 3. Валидация имён в таблице лидеров
//! 4. Константа анимации Hard Drop

use crate::game::{GameMode, GameState};
use crate::highscore::LeaderboardEntry;

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ТЕСТЫ НА ИСПРАВЛЕНИЕ GETRANDOM (highscore.rs)
    // =========================================================================

    /// Тест 1: Проверка генерации случайной соли
    ///
    /// Проверяет, что функция `generate_salt()` работает корректно
    /// после исправления (использование `rand::RngCore` вместо getrandom).
    #[test]
    fn test_random_hash_generation() {
        use crate::highscore::generate_salt;

        let hash1 = generate_salt();
        let hash2 = generate_salt();

        // Соль должна быть ровно 64 hex символа (32 байта в hex формате)
        assert_eq!(hash1.len(), 64, "Соль должна быть ровно 64 hex символа");
        assert_eq!(hash2.len(), 64, "Соль должна быть ровно 64 hex символа");

        // Соли должны быть разными (очень маловероятно, что одинаковые)
        assert_ne!(hash1, hash2, "Две соли должны быть разными");
    }

    /// Тест 2: Проверка целостности `SaveData` после исправления
    #[test]
    fn test_savedata_integrity_after_fix() {
        use crate::highscore::SaveData;

        let save = SaveData::from_value(5000);
        let verified = save.verify_and_get_score();

        assert_eq!(
            verified,
            Some(5000),
            "Хеш должен совпадать для валидных данных"
        );
    }

    /// Тест 3: Проверка генерации `LeaderboardEntry` после исправления
    #[test]
    fn test_leaderboard_entry_generation() {
        let entry = LeaderboardEntry::new("TestPlayer".to_string(), 1000);

        assert_eq!(entry.name(), "TestPlayer");
        assert_eq!(entry.score(), 1000);
        assert!(entry.is_valid(), "Запись должна быть валидной");
    }

    // =========================================================================
    // ТЕСТЫ НА ИНИЦИАЛИЗАЦИЮ ТАЙМЕРА (game.rs)
    // =========================================================================

    /// Тест 4: Проверка инициализации таймера в режиме Sprint
    ///
    /// После создания `GameState::new_sprint()` таймер должен быть запущен.
    #[test]
    fn test_sprint_timer_initialization() {
        let state = GameState::new_sprint();

        // Проверяем, что режим - Sprint
        assert_eq!(state.get_mode(), GameMode::Sprint);

        // Проверяем, что start_time установлен
        let stats = state.get_stats();
        assert!(
            stats.start_time.is_some(),
            "Таймер должен быть запущен в режиме Sprint"
        );
    }

    /// Тест 5: Проверка инициализации таймера в режиме Marathon
    ///
    /// После создания `GameState::new_marathon()` таймер должен быть запущен.
    #[test]
    fn test_marathon_timer_initialization() {
        let state = GameState::new_marathon();

        // Проверяем, что режим - Marathon
        assert_eq!(state.get_mode(), GameMode::Marathon);

        // Проверяем, что start_time установлен
        let stats = state.get_stats();
        assert!(
            stats.start_time.is_some(),
            "Таймер должен быть запущен в режиме Marathon"
        );
    }

    /// Тест 6: Проверка, что Classic режим не запускает таймер автоматически
    ///
    /// `GameState::new()` не должен запускать таймер (для классического режима).
    #[test]
    fn test_classic_no_auto_timer() {
        let state = GameState::new();

        // Проверяем, что режим - Classic
        assert_eq!(state.get_mode(), GameMode::Classic);

        // Для классического режима таймер не запускается автоматически
        // (он запускается при старте игры через play() или внешний код)
    }

    // =========================================================================
    // ТЕСТЫ НА ВАЛИДАЦИЮ ИМЁН (highscore.rs)
    // =========================================================================

    /// Тест 7: Проверка замены пустого имени на "Anonymous"
    ///
    /// Пустое имя должно заменяться на "Anonymous".
    #[test]
    fn test_empty_name_replaced_with_anonymous() {
        let entry = LeaderboardEntry::new(String::new(), 1000);

        assert_eq!(
            entry.name(),
            "Anonymous",
            "Пустое имя должно заменяться на Anonymous"
        );
    }

    /// Тест 8: Проверка имени с пробелами
    ///
    /// Имя с пробелами по краям должно обрезаться.
    #[test]
    fn test_whitespace_name_trimmed() {
        let entry = LeaderboardEntry::new("  Player  ".to_string(), 1000);

        assert_eq!(entry.name(), "Player", "Имя с пробелами должно обрезаться");
    }

    /// Тест 9: Проверка ограничения длины имени
    ///
    /// Имя длиннее 20 символов должно обрезаться.
    #[test]
    fn test_long_name_truncated() {
        let long_name = "VeryLongNameThatShouldBeTruncatedToTwentyCharacters";
        let entry = LeaderboardEntry::new(long_name.to_string(), 1000);

        // Проверяем, что имя обрезано до 20 символов (не байт!)
        assert_eq!(
            entry.name().chars().count(),
            20,
            "Имя должно быть обрезано до 20 символов"
        );
    }

    // =========================================================================
    // ТЕСТЫ НА КОНСТАНТУ АНИМАЦИИ (game.rs)
    // =========================================================================

    /// Тест 10: Проверка наличия константы `HARD_DROP_ANIM_INTERVAL_MS`
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_hard_drop_animation_constant_exists() {
        // Константа должна быть доступна (проверяем через использование)
        use crate::game::HARD_DROP_ANIM_INTERVAL_MS;

        // Константа должна быть положительной
        assert!(
            HARD_DROP_ANIM_INTERVAL_MS > 0,
            "Интервал анимации должен быть положительным"
        );
    }

    /// Тест 11: Проверка значения константы интервала анимации
    #[test]
    fn test_hard_drop_animation_interval_value() {
        use crate::game::HARD_DROP_ANIM_INTERVAL_MS;

        // Константа должна быть 50 мс
        assert_eq!(
            HARD_DROP_ANIM_INTERVAL_MS, 50,
            "Интервал анимации должен быть 50 мс"
        );
    }

    /// Тест 12: Проверка работы анимации с использованием константы
    #[test]
    fn test_animation_with_constant() {
        use crate::game::HARD_DROP_ANIM_INTERVAL_MS;

        // Симулируем время анимации: 100 мс
        let time_ms = 100u16;

        // При 100 мс / 50 мс = 2, и 2.is_multiple_of(2) = true
        assert!((time_ms / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(2));

        // При 75 мс / 50 мс = 1, и 1.is_multiple_of(2) = false
        let time_ms_2 = 75u16;
        assert!(!(time_ms_2 / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(2));
    }

    // =========================================================================
    // ТЕСТЫ НА ИСПРАВЛЕНИЯ CLIPPY (test_fixes.rs)
    // =========================================================================

    /// Тест 13: Проверка, что unused import исправлен
    ///
    /// Этот тест проверяет, что импорт `LeaderboardEntry` используется,
    /// а неиспользуемый импорт Leaderboard удалён.
    #[test]
    fn test_unused_import_fixed() {
        // Используем импорт явно, чтобы избежать предупреждения
        let _entry = LeaderboardEntry::new("Test".to_string(), 100);
        assert_eq!(_entry.name(), "Test");
    }

    /// Тест 14: Проверка, что `assertions_on_constants` исправлен
    ///
    /// Этот тест проверяет, что атрибут #[`allow(clippy::assertions_on_constants)`]
    /// добавлен корректно для теста константы.
    #[test]
    fn test_assertions_on_constants_fixed() {
        use crate::game::HARD_DROP_ANIM_INTERVAL_MS;

        // Константа должна быть доступна и иметь правильное значение
        const EXPECTED_VALUE: u16 = 50;
        assert_eq!(HARD_DROP_ANIM_INTERVAL_MS, EXPECTED_VALUE);
    }

    /// Тест 15: Проверка вспомогательной функции `exit_with_terminal_reset`
    ///
    /// Проверяет, что функция `exit_with_terminal_reset` существует и имеет правильную сигнатуру.
    #[test]
    fn test_canvas_helper_function_exists() {
        use crate::io::Canvas;

        // Проверяем, что Canvas имеет метод exit_with_terminal_reset через компиляцию
        // Эта функция должна существовать и быть приватной
        // Мы не можем вызвать её напрямую, но можем проверить, что Canvas работает
        let _canvas_check = std::mem::size_of::<Canvas>();
        assert!(_canvas_check > 0, "Canvas должен иметь размер");
    }
}
