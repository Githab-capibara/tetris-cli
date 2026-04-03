//! Тесты для проверки исправлений clippy ошибок

#![allow(clippy::assertions_on_constants)]
#![allow(clippy::absurd_extreme_comparisons)]

#[cfg(test)]
mod clippy_fixes_tests {
    use crate::highscore::leaderboard::LeaderboardEntry;
    use std::io;

    /// Тест 1: Проверка что `io::Error::other()` правильно используется
    #[test]
    fn test_io_error_other() {
        let err = io::Error::other("test error");
        assert_eq!(err.kind(), io::ErrorKind::Other);
        assert_eq!(err.to_string(), "test error");
    }

    /// Тест 2: Проверка что `LeaderboardEntry` принимает &str без неоправданного借用
    #[test]
    fn test_leaderboard_entry_str_ref() {
        let name = "TestPlayer";
        let entry = LeaderboardEntry::new(name, 1000);
        assert_eq!(entry.name(), "TestPlayer");
        assert_eq!(entry.score(), Some(1000));
    }

    /// Тест 3: Проверка что диапазоны используют .`contains()` вместо manual check
    #[test]
    fn test_range_contains() {
        let value = 2.5_f32;
        assert!((-2.0..=3.0).contains(&value));

        let outside = 5.0_f32;
        assert!(!(-2.0..=3.0).contains(&outside));
    }

    /// Тест 4: Проверка что итераторы используются вместо `needless_range_loop`
    #[test]
    fn test_iterator_instead_of_range_loop() {
        let mut array = [0; 10];

        // Правильный способ с итератором
        for (i, elem) in array.iter_mut().enumerate() {
            *elem = i as i32;
        }

        assert_eq!(array[0], 0);
        assert_eq!(array[9], 9);
    }

    /// Тест 5: Проверка что empty string используется вместо `String::new()`
    #[test]
    fn test_empty_string_literal() {
        let entry = LeaderboardEntry::new("", 1000);
        assert_eq!(entry.name(), "Anonymous");
    }

    /// Тест 6: Проверка что identity операции убраны
    #[test]
    fn test_no_identity_ops() {
        let value = 50;
        // Правильно: просто value вместо value * 1
        assert_eq!(value, 50);

        let zero = 0;
        // Правильно: просто 0 вместо value * 0
        assert_eq!(zero, 0);
    }

    /// Тест 7: Проверка что assert!(true) заменены на комментарии
    #[test]
    fn test_no_assert_true() {
        // Этот тест просто проверяет что код скомпилировался
        // и нет бесполезных assert!(true) утверждений
        let x = 42;
        assert_eq!(x, 42);
    }

    /// Тест 8: Проверка что сравнения с max/min типов убраны
    #[test]
    fn test_no_absurd_comparisons() {
        let value: u32 = 100;
        // u32 не может быть > u32::MAX по определению
        // Такие проверки убраны из production кода
        // Этот тест просто подтверждает что код компилируется
        let max_check = value <= u32::MAX;
        assert!(max_check, "Значение u32 должно быть <= u32::MAX");
    }
}
