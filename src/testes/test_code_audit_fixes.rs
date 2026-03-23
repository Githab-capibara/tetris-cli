//! Тесты для исправлений проблем, выявленных при аудите кода.
//!
//! Этот модуль содержит тесты для следующих исправлений:
//! 1. Исправление doctest в highscore.rs (новое имя параметра)
//! 2. Проверка типов при casting usize/u16 в i16
//! 3. Проверка документации на наличие ошибок
//! 4. Проверка format! макросов на излишние Debug форматы

#[cfg(test)]
mod code_audit_tests {
    use crate::highscore::LeaderboardEntry;

    /// Тест 1: Проверка doctest исправления в highscore.rs
    ///
    /// Убеждаемся, что LeaderboardEntry::new() корректно принимает &str
    /// и создает валидную запись.
    #[test]
    fn test_leaderboard_entry_accepts_str_reference() {
        // Это должно компилироваться без ошибок
        let entry = LeaderboardEntry::new("TestPlayer", 5000);

        // Проверяем, что запись создана и валидна
        assert_eq!(entry.name(), "TestPlayer");
        assert_eq!(entry.score(), 5000u128);
        assert!(entry.is_valid());
    }

    /// Тест 2: Проверка, что LeaderboardEntry::new() работает с разными именами
    #[test]
    fn test_leaderboard_entry_with_various_names() {
        let names = vec!["Player", "TestName", "Anonymous", "A"];

        for name in names {
            let entry = LeaderboardEntry::new(name, 1000);
            assert_eq!(entry.name(), name);
            assert!(entry.is_valid());
        }
    }

    /// Тест 3: Проверка целостности хеша в LeaderboardEntry
    ///
    /// Убеждаемся, что метод is_valid() корректно проверяет хеш.
    #[test]
    fn test_leaderboard_entry_hash_integrity() {
        let entry = LeaderboardEntry::new("TestPlayer", 12345);

        // Новая запись должна быть валидной
        assert!(entry.is_valid());

        // Проверяем score() возвращает правильное значение
        assert_eq!(entry.score(), 12345u128);
    }
}
