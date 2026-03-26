//! Модуль санитаризации имён игроков.
//!
//! Предоставляет функции для валидации и очистки имён игроков
//! перед сохранением в таблицу лидеров.
//!
//! ## Архитектурные заметки
//! Функции валидации имён перемещены в модуль [`crate::validation::name`]
//! для централизации кода валидации. Этот модуль переэкспортирует
//! функции для обратной совместимости.

// Переэкспорт из модуля валидации для обратной совместимости
pub use crate::validation::name::{is_valid_name_char, sanitize_player_name};

#[cfg(test)]
mod sanitize_tests {
    use super::*;
    use crate::crypto;

    #[test]
    fn test_sanitize_player_name_empty_to_anonymous() {
        assert_eq!(sanitize_player_name(""), "Anonymous");
        assert_eq!(sanitize_player_name("   \t\n"), "Anonymous");
    }

    #[test]
    fn test_sanitize_player_name_filters_invalid_chars_and_fallback() {
        // Все символы невалидны -> fallback
        assert_eq!(sanitize_player_name("@@@###"), "Anonymous");

        // Смешанное имя -> остаются только разрешённые
        assert_eq!(sanitize_player_name("Pl@yer!_1"), "Plyer_1");
    }

    #[test]
    fn test_sanitize_player_name_truncates_to_20_chars() {
        let name = "abcdefghijklmnopqrstuvwxyz";
        let sanitized = sanitize_player_name(name);
        assert_eq!(sanitized.chars().count(), 20);
        assert_eq!(sanitized, "abcdefghijklmnopqrst");
    }

    #[test]
    fn test_generate_salt_length_and_hex() {
        let hash = crypto::generate_salt();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_salt_uniqueness_smoke() {
        let a = crypto::generate_salt();
        let b = crypto::generate_salt();
        assert_ne!(a, b, "Две соли подряд не должны совпадать (smoke test)");
    }

    #[test]
    fn test_generate_salt_is_lowercase_hex() {
        let hash = crypto::generate_salt();
        assert!(hash.chars().all(|c| !c.is_ascii_uppercase()));
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ UNICODE БЕЗОПАСНОСТИ
    // =========================================================================

    /// Тест: проверка на bidirectional control characters (U+200E, U+200F)
    #[test]
    fn test_sanitize_player_name_bidirectional_chars() {
        // Имя с bidirectional control characters
        let name_with_bidi = "Player\u{200E}Name"; // U+200E - LTR mark
        let sanitized = sanitize_player_name(name_with_bidi);
        // Bidi символы должны быть удалены
        assert!(!sanitized.contains('\u{200E}'));
        assert!(!sanitized.contains('\u{200F}'));
        assert_eq!(sanitized, "PlayerName");
    }

    /// Тест: проверка на другие bidirectional control characters
    #[test]
    fn test_sanitize_player_name_all_bidi_chars() {
        let bidi_chars = [
            '\u{200E}', // LTR mark
            '\u{200F}', // RTL mark
            '\u{202A}', // LTR embedding
            '\u{202B}', // RTL embedding
            '\u{202C}', // POP directional formatting
            '\u{202D}', // LTR override
            '\u{202E}', // RTL override
            '\u{2066}', // LTR isolate
            '\u{2067}', // RTL isolate
            '\u{2068}', // FSI
            '\u{2069}', // PDI
        ];

        for &char in &bidi_chars {
            let name = format!("Player{char}Name");
            let sanitized = sanitize_player_name(&name);
            assert!(
                !sanitized.contains(char),
                "Bidi символ {char:?} должен быть удалён"
            );
        }
    }

    /// Тест: проверка на эмодзи
    #[test]
    fn test_sanitize_player_name_emoji_filtered() {
        // Имя с эмодзи
        let name_with_emoji = "Player😀Name";
        let sanitized = sanitize_player_name(name_with_emoji);
        // Эмодзи должны быть удалены (они не проходят is_valid_name_char)
        assert!(!sanitized.contains('😀'));
        assert_eq!(sanitized, "PlayerName");
    }

    /// Тест: проверка на комбинированные символы
    #[test]
    fn test_sanitize_player_name_combined_chars() {
        // Имя с комбинирующими символами (например, e + combining acute = é)
        let name_combined = "Caf\u{0065}\u{0301}"; // e + combining acute
        let sanitized = sanitize_player_name(name_combined);
        // Комбинирующие символы разрешены если base символ alphanumeric
        assert!(sanitized.contains('e'));
    }

    /// Тест: проверка на очень длинные имена
    #[test]
    fn test_sanitize_player_name_very_long_name() {
        let very_long_name = "a".repeat(1000);
        let sanitized = sanitize_player_name(&very_long_name);
        assert_eq!(sanitized.len(), 20);
        assert_eq!(sanitized, "aaaaaaaaaaaaaaaaaaaa");
    }

    /// Тест: проверка на имена только с control characters
    #[test]
    fn test_sanitize_player_name_only_control_chars() {
        let name_control = "\u{200E}\u{200F}\u{202A}";
        let sanitized = sanitize_player_name(name_control);
        assert_eq!(sanitized, "Anonymous");
    }
}
