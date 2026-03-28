//! Модуль валидации имён игроков.
//!
//! Предоставляет функции для валидации и очистки имён игроков
//! перед сохранением в таблицу лидеров.
//!
//! ## Функции
//! - [`is_forbidden_char`] — проверка запрещённых Unicode-символов
//! - [`is_valid_name_char`] — проверка допустимости символа имени
//! - [`sanitize_player_name`] — санитаризация имени игрока

/// Проверить запрещённые Unicode-символы.
///
/// Запрещены:
/// - Bidirectional control characters (U+200E, U+200F, U+202A-U+202E, U+2066-U+2069)
/// - Zero-width joiners (U+200C, U+200D)
/// - Variation selectors (U+FE00-U+FE0F)
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ запрещён
///
/// # Исправление #18
/// Выделена из `sanitize_player_name()` для улучшения читаемости и тестируемости.
fn is_forbidden_char(c: char) -> bool {
    matches!(c,
        '\u{200E}' | '\u{200F}' |  // Bidi
        '\u{202A}'..='\u{202E}' |  // Bidi formatting
        '\u{2066}'..='\u{2069}' |  // Bidi isolate
        '\u{200C}' | '\u{200D}' |  // Zero-width joiners
        '\u{FE00}'..='\u{FE0F}'    // Variation selectors
    )
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
pub fn is_valid_name_char(c: char) -> bool {
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
/// - Bidirectional control characters отбрасываются через `is_forbidden_char()`
/// - Zero-width joiners отбрасываются через `is_forbidden_char()`
/// - Variation selectors отбрасываются через `is_forbidden_char()`
/// - Emojis и другие опасные символы фильтруются
/// - Whitelist разрешённых символов через `is_valid_name_char()`
///
/// # Исправление #18
/// Использует функцию `is_forbidden_char()` для улучшения читаемости.
///
/// # Исправление #8 (HIGH)
/// Объединяет два фильтра в один проход для оптимизации.
pub fn sanitize_player_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Anonymous".to_string();
    }

    // Исправление #8: объединяем фильтры в один проход для оптимизации
    // Вместо двух отдельных filter() используем один с комбинированным условием
    let validated: String = trimmed
        .chars()
        .filter(|&c| !is_forbidden_char(c) && is_valid_name_char(c))
        .take(20)
        .collect();

    if validated.is_empty() {
        "Anonymous".to_string()
    } else {
        validated
    }
}

#[cfg(test)]
mod validation_name_tests {
    use super::*;

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

    #[test]
    fn test_is_valid_name_char_ascii_letters() {
        assert!(is_valid_name_char('a'));
        assert!(is_valid_name_char('z'));
        assert!(is_valid_name_char('A'));
        assert!(is_valid_name_char('Z'));
    }

    #[test]
    fn test_is_valid_name_char_digits() {
        assert!(is_valid_name_char('0'));
        assert!(is_valid_name_char('9'));
    }

    #[test]
    fn test_is_valid_name_char_special() {
        assert!(is_valid_name_char('_'));
        assert!(is_valid_name_char('-'));
        assert!(is_valid_name_char(' '));
    }

    #[test]
    fn test_is_valid_name_char_cyrillic() {
        assert!(is_valid_name_char('а'));
        assert!(is_valid_name_char('я'));
        assert!(is_valid_name_char('ё'));
        assert!(is_valid_name_char('А'));
        assert!(is_valid_name_char('Я'));
        assert!(is_valid_name_char('Ё'));
    }

    #[test]
    fn test_is_valid_name_char_invalid() {
        assert!(!is_valid_name_char('/'));
        assert!(!is_valid_name_char('\\'));
        assert!(!is_valid_name_char('@'));
        assert!(!is_valid_name_char('!'));
        assert!(!is_valid_name_char('\n'));
        assert!(!is_valid_name_char('\t'));
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ is_forbidden_char()
    // =========================================================================

    /// Тест: проверка is_forbidden_char для bidi символов
    #[test]
    fn test_is_forbidden_char_bidirectional() {
        assert!(is_forbidden_char('\u{200E}'));
        assert!(is_forbidden_char('\u{200F}'));
        assert!(is_forbidden_char('\u{202A}'));
        assert!(is_forbidden_char('\u{202E}'));
        assert!(is_forbidden_char('\u{2066}'));
        assert!(is_forbidden_char('\u{2069}'));
    }

    /// Тест: проверка is_forbidden_char для zero-width joiners
    #[test]
    fn test_is_forbidden_char_zero_width() {
        assert!(is_forbidden_char('\u{200C}'));
        assert!(is_forbidden_char('\u{200D}'));
    }

    /// Тест: проверка is_forbidden_char для variation selectors
    #[test]
    fn test_is_forbidden_char_variation_selectors() {
        assert!(is_forbidden_char('\u{FE00}'));
        assert!(is_forbidden_char('\u{FE0F}'));
        assert!(is_forbidden_char('\u{FE05}'));
    }

    /// Тест: проверка is_forbidden_char для обычных символов
    #[test]
    fn test_is_forbidden_char_normal_chars() {
        assert!(!is_forbidden_char('a'));
        assert!(!is_forbidden_char('A'));
        assert!(!is_forbidden_char('0'));
        assert!(!is_forbidden_char('_'));
        assert!(!is_forbidden_char('-'));
        assert!(!is_forbidden_char(' '));
        assert!(!is_forbidden_char('а'));
        assert!(!is_forbidden_char('я'));
        assert!(!is_forbidden_char('ё'));
    }
}
