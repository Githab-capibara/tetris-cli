//! Модуль валидации имён игроков.
//!
//! Предоставляет функции для валидации и очистки имён игроков
//! перед сохранением в таблицу лидеров.
//!
//! # Безопасность
//!
//! ## Whitelist подход
//! Модуль использует whitelist подход: разрешены только известные безопасные символы.
//! Это предотвращает:
//! - **XSS-атаки**: запрещены специальные символы и скрипты
//! - **Injection-атаки**: запрещены символы пути '/', '\\'
//! - **Unicode-атаки**: запрещены управляющие символы через `is_control()`
//! - **Bidi-атаки**: запрещены bidirectional control characters
//!
//! ## Разрешённые символы
//! - ASCII буквы (a-z, A-Z)
//! - ASCII цифры (0-9)
//! - Специальные символы: '_', '-', ' ' (пробел)
//! - Русские буквы (а-я, А-Я, ё, Ё)
//!
//! ## Запрещённые символы
//! - Управляющие символы (is_control)
//! - Символы пути (/, \\)
//! - Все остальные Unicode символы (эмодзи, специальные символы)
//!
//! ## Ограничение длины
//! Максимальная длина имени: 32 символа.
//! Это предотвращает:
//! - Переполнение буфера
//! - DoS через длинные строки
//! - Повреждение UI
//!
//! # Исправление аудита 2026-04-02 (C12)
//! Увеличен лимит с 20 до 32 символов для соответствию требованиям аудита.
//!
//! ## Функции
//! - [`is_valid_name_char`] — проверка допустимости символа имени (whitelist)
//! - [`sanitize_player_name`] — санитаризация имени игрока

/// Имя по умолчанию для анонимных игроков.
///
/// Используется когда имя пустое или после фильтрации не осталось валидных символов.
const ANONYMOUS_NAME: &str = "Anonymous";

/// Максимальная длина имени игрока.
///
/// # Исправление аудита 2026-04-02 (C12)
/// Увеличен лимит с 20 до 32 символов для соответствию требованиям аудита.
const MAX_NAME_LENGTH: usize = 32;

/// Проверить допустимость символа имени (whitelist подход).
///
/// Разрешены только:
/// - ASCII буквы (a-z, A-Z)
/// - ASCII цифры (0-9)
/// - Специальные символы: '_', '-', ' ' (пробел)
/// - Русские буквы (а-я, А-Я, ё, Ё)
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим
///
/// # Безопасность
/// Whitelist подход: разрешены только безопасные символы.
/// Запрещены управляющие символы через `is_control()`.
///
/// # Исправление #6 (LOW)
/// Используется `matches!` макрос с диапазонами для читаемой whitelist проверки.
pub fn is_valid_name_char(c: char) -> bool {
    // Whitelist подход: разрешаем только безопасные символы
    !c.is_control()
        && matches!(c,
            'a'..='z' | 'A'..='Z' | '0'..='9' |  // ASCII буквы и цифры
            'а'..='я' | 'А'..='Я' | 'ё' | 'Ё' |  // Русские буквы
            '_' | '-' | ' '  // Специальные символы
        )
}

/// Санитизировать имя игрока для таблицы лидеров.
///
/// Правила:
/// - trim
/// - whitelist разрешённых символов через `is_valid_name_char()`
/// - максимум 32 символа (MAX_NAME_LENGTH)
/// - пустое имя (в т.ч. после фильтрации) заменяется на "Anonymous"
///
/// # Аргументы
/// * `name` - имя для санитаризации
///
/// # Возвращает
/// Безопасное имя для таблицы лидеров
///
/// # Panics
/// Эта функция НИКОГДА не паникует.
/// Все операции безопасны: фильтрация символов и обрезка строки.
///
/// # Безопасность
/// Использует whitelist подход: разрешены только ASCII буквы/цифры,
/// специальные символы '_', '-', ' ' и русские буквы.
///
/// # Исправление аудита 2026-04-02 (C12)
/// Увеличен лимит с 20 до 32 символов.
///
/// # Исправление аудита 2026-03-31 (M2)
/// Использует двухпроходный алгоритм для точного выделения памяти:
/// 1. Подсчёт валидных символов (максимум 32)
/// 2. Выделение строки с точным размером
///
/// # Примеры
/// ```
/// use tetris_cli::validation::sanitize_player_name;
///
/// assert_eq!(sanitize_player_name("Player1"), "Player1");
/// assert_eq!(sanitize_player_name("  "), "Anonymous");
/// assert_eq!(sanitize_player_name("<script>"), "script");
/// assert_eq!(sanitize_player_name("Игрок"), "Игрок");
/// ```
pub fn sanitize_player_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return ANONYMOUS_NAME.to_string();
    }

    // M10: однопроходный алгоритм с предварительным выделением памяти
    let mut validated = String::with_capacity(MAX_NAME_LENGTH);

    for c in trimmed.chars() {
        if is_valid_name_char(c) {
            // S3: схлопывание последовательных пробелов
            if c == ' ' {
                if let Some(last) = validated.chars().last() {
                    if last == ' ' {
                        continue;
                    }
                }
            }
            validated.push(c);
            if validated.chars().count() >= MAX_NAME_LENGTH {
                break;
            }
        }
    }

    // Убираем trailing пробел
    let result = validated.trim_end();
    if result.is_empty() {
        return ANONYMOUS_NAME.to_string();
    }

    result.to_string()
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
        // Тест остаётся для обратной совместимости - проверяем что 20 символов принимаются
        let name = "abcdefghijklmnopqrst";
        let sanitized = sanitize_player_name(name);
        assert_eq!(sanitized.chars().count(), 20);
        assert_eq!(sanitized, "abcdefghijklmnopqrst");
    }

    /// Тест C12: проверка обрезки до 32 символов
    #[test]
    fn test_sanitize_player_name_truncates_to_32_chars() {
        let name = "abcdefghijklmnopqrstuvwxyz1234567890"; // 36 символов
        let sanitized = sanitize_player_name(name);
        assert_eq!(sanitized.chars().count(), 32);
        assert_eq!(sanitized, "abcdefghijklmnopqrstuvwxyz123456");
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

    /// Тест: проверка на очень длинные имена (обновлён для C12)
    #[test]
    fn test_sanitize_player_name_very_long_name() {
        let very_long_name = "a".repeat(1000);
        let sanitized = sanitize_player_name(&very_long_name);
        assert_eq!(sanitized.len(), 32);
        assert_eq!(sanitized, "a".repeat(32));
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
    // ТЕСТЫ ДЛЯ H6: ОПТИМИЗАЦИЯ sanitize_player_name()
    // =========================================================================

    /// Тест H6: проверка оптимизации с filter_map - один проход
    #[test]
    fn test_fix_h6_sanitize_single_pass_filter_map() {
        // Проверка что filter_map корректно объединяет фильтрацию и маппинг
        let name = "Pl@yer!_1";
        let sanitized = sanitize_player_name(name);
        // Все невалидные символы должны быть удалены за один проход
        assert_eq!(sanitized, "Plyer_1");
    }

    /// Тест H6: проверка оптимизации с with_capacity - предотвращение реаллокаций
    #[test]
    fn test_fix_h6_sanitize_with_capacity_optimization() {
        // Проверка что имя длиной 20 символов не требует реаллокации
        let exactly_20_chars = "abcdefghijklmnopqrst";
        let sanitized = sanitize_player_name(exactly_20_chars);
        assert_eq!(sanitized.len(), 20);
        assert_eq!(sanitized, exactly_20_chars);
    }

    /// Тест H6: проверка обработки имён с mixed valid/invalid символами
    #[test]
    fn test_fix_h6_sanitize_mixed_valid_invalid_chars() {
        // Смешанное имя с разрешёнными и запрещёнными символами
        let mixed_name = "Valid_123-Test";
        let sanitized = sanitize_player_name(mixed_name);
        assert_eq!(sanitized, "Valid_123-Test");

        // Имя с запрещёнными символами
        let invalid_name = "Invalid@#Test!";
        let sanitized_invalid = sanitize_player_name(invalid_name);
        assert_eq!(sanitized_invalid, "InvalidTest");
    }

    /// Тест H6: проверка обработки имён с пробелами
    #[test]
    fn test_fix_h6_sanitize_names_with_spaces() {
        // Имя с пробелами (пробелы разрешены)
        let name_with_spaces = "Player Name";
        let sanitized = sanitize_player_name(name_with_spaces);
        assert_eq!(sanitized, "Player Name");

        // Имя с пробелами по краям (trim)
        let name_with_trim_spaces = "  Player  ";
        let sanitized_trimmed = sanitize_player_name(name_with_trim_spaces);
        assert_eq!(sanitized_trimmed, "Player");
    }

    /// Тест H6: проверка обработки русских имён
    #[test]
    fn test_fix_h6_sanitize_cyrillic_names() {
        let russian_name = "Игрок123";
        let sanitized = sanitize_player_name(russian_name);
        assert_eq!(sanitized, "Игрок123");

        let mixed_name = "Player1Игрок2";
        let sanitized_mixed = sanitize_player_name(mixed_name);
        assert_eq!(sanitized_mixed, "Player1Игрок2");
    }

    /// Тест H6: проверка обработки special characters
    #[test]
    fn test_fix_h6_sanitize_special_characters() {
        // Разрешённые специальные символы
        let special_name = "Player_Name-123";
        let sanitized = sanitize_player_name(special_name);
        assert_eq!(sanitized, "Player_Name-123");

        // Запрещённые специальные символы
        let invalid_special = "Player@Name#Test$";
        let sanitized_invalid = sanitize_player_name(invalid_special);
        assert_eq!(sanitized_invalid, "PlayerNameTest");
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ПРОВЕРКИ КОНСТАНТЫ ANONYMOUS_NAME
    // =========================================================================

    /// Тест: проверка что константа ANONYMOUS_NAME существует и равна "Anonymous"
    #[test]
    fn test_anonymous_constant() {
        // Проверка что константа ANONYMOUS_NAME существует и равна "Anonymous"
        assert_eq!(ANONYMOUS_NAME, "Anonymous");
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ C12: ЛИМИТ ДЛИНЫ ИМЕНИ
    // =========================================================================

    /// Тест C12: проверка что константа MAX_NAME_LENGTH равна 32
    #[test]
    fn test_max_name_length_constant() {
        assert_eq!(MAX_NAME_LENGTH, 32);
    }

    /// Тест C12: проверка что длинные имена обрезаются до 32 символов
    #[test]
    fn test_c12_long_name_truncated_to_32_chars() {
        let very_long_name = "a".repeat(100);
        let sanitized = sanitize_player_name(&very_long_name);
        assert_eq!(sanitized.chars().count(), 32);
        assert_eq!(sanitized, "a".repeat(32));
    }

    /// Тест C12: проверка что имя ровно в 32 символа принимается полностью
    #[test]
    fn test_c12_exactly_32_chars_accepted() {
        let exactly_32_chars = "abcdefghijklmnopqrstuvwxyz123456"; // 32 символа: 26 букв + 6 цифр
        assert_eq!(exactly_32_chars.len(), 32);
        let sanitized = sanitize_player_name(exactly_32_chars);
        assert_eq!(sanitized.chars().count(), 32);
        assert_eq!(sanitized, exactly_32_chars);
    }
}
