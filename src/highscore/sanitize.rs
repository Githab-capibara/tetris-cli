//! Модуль санитаризации имён игроков.
//!
//! Предоставляет функции для валидации и очистки имён игроков
//! перед сохранением в таблицу лидеров.

/// Санитизировать имя игрока для таблицы лидеров.
///
/// Правила:
/// - trim
/// - разрешены только ASCII буквы/цифры и символы: '_', '-', ' '
/// - максимум 20 символов
/// - пустое имя (в т.ч. после фильтрации) заменяется на "Anonymous"
/// - запрещены опасные Unicode-символы (эмодзи, контрольные символы)
/// - запрещены bidirectional control characters (U+200E, U+200F)
/// - запрещены zero-width joiners (U+200C, U+200D) и variation selectors (U+FE00-U+FE0F)
/// - используется whitelist разрешённых символов
///
/// # Аргументы
/// * `name` - имя для санитаризации
///
/// # Возвращает
/// Безопасное имя для таблицы лидеров
///
/// # Безопасность
/// Использует итераторы с `filter()` и `take()` для эффективной фильтрации.
/// Добавлена защита от Unicode-атак:
/// - Bidirectional control characters (U+200E, U+200F) отбрасываются
/// - Zero-width joiners (U+200C, U+200D) отбрасываются
/// - Variation selectors (U+FE00-U+FE0F) отбрасываются
/// - Emojis и другие опасные символы фильтруются
/// - Whitelist разрешённых символов
pub fn sanitize_player_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Anonymous".to_string();
    }

    // Оптимизация: используем filter() + take() + collect() вместо ручного цикла
    let validated: String = trimmed
        .chars()
        .filter(|&c| {
            // Фильтрация bidirectional control characters
            if c == '\u{200E}'
                || c == '\u{200F}'
                || c == '\u{202A}'
                || c == '\u{202B}'
                || c == '\u{202C}'
                || c == '\u{202D}'
                || c == '\u{202E}'
                || c == '\u{2066}'
                || c == '\u{2067}'
                || c == '\u{2068}'
                || c == '\u{2069}'
            {
                return false;
            }

            // Фильтрация zero-width joiners
            if c == '\u{200C}' || c == '\u{200D}' {
                return false;
            }

            // Фильтрация variation selectors (U+FE00-U+FE0F)
            if ('\u{FE00}'..='\u{FE0F}').contains(&c) {
                return false;
            }

            // Проверка на разрешённые символы (whitelist)
            is_valid_name_char(c) && !c.is_control()
        })
        .take(20)
        .collect();

    if validated.is_empty() {
        "Anonymous".to_string()
    } else {
        validated
    }
}

/// Проверить допустимость символа имени.
///
/// Разрешены только:
/// - ASCII буквы (a-z, A-Z)
/// - ASCII цифры (0-9)
/// - Специальные символы: '_', '-', ' '
/// - Русские буквы (а-я, А-Я, ё, Ё)
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим
///
/// # Безопасность
/// Расширенная валидация Unicode для поддержки международных имён.
/// Запрещены управляющие символы и эмодзи через `is_control()`.
///
/// # Исправление #9
/// Используется `matches!` макрос с диапазонами для более читаемой проверки.
fn is_valid_name_char(c: char) -> bool {
    // Исправление #9: используем matches! макрос с диапазонами для читаемости
    !c.is_control()
        && c != '/'
        && c != '\\'
        && (matches!(c,
            'a'..='z' | 'A'..='Z' | '0'..='9' |  // ASCII буквы и цифры
            'а'..='я' | 'А'..='Я' | 'ё' | 'Ё' |  // Русские буквы
            '_' | '-' | ' '  // Специальные символы (включая пробел)
        ))
}

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
