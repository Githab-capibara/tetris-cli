//! Тесты валидации Unicode.
//!
//! Проверяют фильтрацию zero-width joiners, variation selectors,
//! допустимые и недопустимые символы.

use crate::highscore::LeaderboardEntry;

/// Тест 1: Проверка фильтрации zero-width joiners (U+200C)
///
/// Zero-width joiner U+200C должен отфильтровываться.
#[test]
fn test_unicode_zwnj_u200c() {
    let name_with_zwnj = "Player\u{200C}Name";
    let entry = LeaderboardEntry::new(name_with_zwnj, 1000);

    assert!(
        !entry.name().contains('\u{200C}'),
        "Zero-width joiner U+200C должен быть отфильтрован"
    );
    assert!(
        entry.name().contains("Player"),
        "Допустимые символы должны остаться"
    );
    assert!(
        entry.name().contains("Name"),
        "Допустимые символы должны остаться"
    );
}

/// Тест 2: Проверка фильтрации zero-width joiners (U+200D)
///
/// Zero-width joiner U+200D должен отфильтровываться.
#[test]
fn test_unicode_zwj_u200d() {
    let name_with_zwj = "Player\u{200D}Name";
    let entry = LeaderboardEntry::new(name_with_zwj, 1000);

    assert!(
        !entry.name().contains('\u{200D}'),
        "Zero-width joiner U+200D должен быть отфильтрован"
    );
    assert!(
        entry.name().contains("Player"),
        "Допустимые символы должны остаться"
    );
    assert!(
        entry.name().contains("Name"),
        "Допустимые символы должны остаться"
    );
}

/// Тест 3: Проверка фильтрации variation selectors (U+FE00-U+FE0F)
///
/// Все variation selectors должны отфильтровываться.
#[test]
fn test_unicode_variation_selectors_range() {
    // Проверяем весь диапазон variation selectors
    for vs in 0xFE00..=0xFE0F {
        let ch = char::from_u32(vs).unwrap();
        let name = format!("Player{}Name", ch);
        let entry = LeaderboardEntry::new(&name, 1000);

        assert!(
            !entry.name().contains(ch),
            "Variation selector U+{vs:04X} должен быть отфильтрован"
        );
    }
}

/// Тест 4: Проверка допустимых символов - ASCII буквы
///
/// ASCII буквы должны проходить фильтрацию (с учётом ограничения длины 20).
#[test]
fn test_unicode_allowed_ascii_letters() {
    // Используем имена длиной 20 символов (максимальная длина)
    let uppercase = "ABCDEFGHIJKLMNOPQRST";
    let lowercase = "abcdefghijklmnopqrst";

    let entry_upper = LeaderboardEntry::new(uppercase, 1000);
    let entry_lower = LeaderboardEntry::new(lowercase, 1000);

    assert_eq!(
        entry_upper.name(),
        uppercase,
        "Заглавные ASCII буквы должны проходить"
    );
    assert_eq!(
        entry_lower.name(),
        lowercase,
        "Строчные ASCII буквы должны проходить"
    );
}

/// Тест 5: Проверка допустимых символов - цифры
///
/// Цифры должны проходить фильтрацию.
#[test]
fn test_unicode_allowed_digits() {
    let digits = "0123456789";
    let entry = LeaderboardEntry::new(digits, 1000);

    assert_eq!(entry.name(), digits, "Цифры должны проходить");
}

/// Тест 6: Проверка допустимых символов - специальные
///
/// Символы '_', '-', ' ' должны проходить фильтрацию.
#[test]
fn test_unicode_allowed_special_chars() {
    let special = "Player_Name-Test 123";
    let entry = LeaderboardEntry::new(special, 1000);

    assert_eq!(
        entry.name(),
        special,
        "Специальные символы '_', '-', ' ' должны проходить"
    );
}

/// Тест 7: Проверка недопустимых символов - emoji
///
/// Emoji должны отфильтровываться.
#[test]
fn test_unicode_disallowed_emoji() {
    let emojis = ["😀", "😂", "🎮", "🎯", "🏆"];

    for &emoji in &emojis {
        let name = format!("Player{}Name", emoji);
        let entry = LeaderboardEntry::new(&name, 1000);

        assert!(
            !entry.name().contains(emoji),
            "Emoji {emoji} должен быть отфильтрован"
        );
    }
}

/// Тест 8: Проверка недопустимых символов - control characters
///
/// Control characters должны отфильтровываться.
#[test]
fn test_unicode_disallowed_control_chars() {
    // Проверяем control characters U+0000-U+001F
    for cc in 0x0000..=0x001F {
        let ch = char::from_u32(cc).unwrap();

        // Пропускаем некоторые разрешённые (табуляция, новая строка)
        if ch == '\t' || ch == '\n' || ch == '\r' {
            continue;
        }

        let name = format!("Player{}Name", ch);
        let entry = LeaderboardEntry::new(&name, 1000);

        assert!(
            !entry.name().contains(ch),
            "Control character U+{cc:04X} должен быть отфильтрован"
        );
    }
}

/// Тест 9: Проверка bidirectional control characters
///
/// Bidirectional control characters должны отфильтровываться.
#[test]
fn test_unicode_bidi_control_chars() {
    let bidi_chars = [
        ('\u{200E}', "LEFT-TO-RIGHT MARK"),
        ('\u{200F}', "RIGHT-TO-LEFT MARK"),
        ('\u{202A}', "LEFT-TO-RIGHT EMBEDDING"),
        ('\u{202B}', "RIGHT-TO-LEFT EMBEDDING"),
        ('\u{202C}', "POP DIRECTIONAL FORMATTING"),
        ('\u{202D}', "LEFT-TO-RIGHT OVERRIDE"),
        ('\u{202E}', "RIGHT-TO-LEFT OVERRIDE"),
        ('\u{2066}', "LEFT-TO-RIGHT ISOLATE"),
        ('\u{2067}', "RIGHT-TO-LEFT ISOLATE"),
        ('\u{2068}', "FIRST STRONG ISOLATE"),
        ('\u{2069}', "POP DIRECTIONAL ISOLATE"),
    ];

    for &(ch, name) in &bidi_chars {
        let player_name = format!("Player{}Name", ch);
        let entry = LeaderboardEntry::new(&player_name, 1000);

        assert!(
            !entry.name().contains(ch),
            "{name} (U+200E) должен быть отфильтрован"
        );
    }
}

/// Тест 10: Проверка Unicode вне Basic Multilingual Plane
///
/// Символы вне BMP должны отфильтровываться.
#[test]
fn test_unicode_outside_bmp() {
    // Символы вне BMP (U+10000 и выше)
    let outside_bmp = [
        '\u{1F600}', // 😀 Grinning Face
        '\u{1F3AE}', // 🎮 Video Game
        '\u{1F3AF}', // 🎯 Direct Hit
    ];

    for &ch in &outside_bmp {
        let name = format!("Player{}Name", ch);
        let entry = LeaderboardEntry::new(&name, 1000);

        assert!(
            !entry.name().contains(ch),
            "Символ вне BMP U+{:04X} должен быть отфильтрован",
            ch as u32
        );
    }
}

/// Тест 11: Проверка смешанных допустимых и недопустимых символов
///
/// Проверяем, что допустимые символы остаются, а недопустимые отфильтровываются.
#[test]
fn test_unicode_mixed_allowed_disallowed() {
    let mixed_name = "Play\u{200C}er\u{FE00}Name\u{200D}123";
    let entry = LeaderboardEntry::new(mixed_name, 1000);

    // Проверяем, что допустимые символы остались
    assert!(
        entry.name().contains("Player")
            || entry.name().contains("Play") && entry.name().contains("er"),
        "Допустимые символы должны остаться"
    );
    assert!(
        entry.name().contains("Name"),
        "Допустимые символы должны остаться"
    );
    assert!(
        entry.name().contains("123"),
        "Допустимые символы должны остаться"
    );

    // Проверяем, что недопустимые символы отфильтрованы
    assert!(
        !entry.name().contains('\u{200C}'),
        "U+200C должен быть отфильтрован"
    );
    assert!(
        !entry.name().contains('\u{FE00}'),
        "U+FE00 должен быть отфильтрован"
    );
    assert!(
        !entry.name().contains('\u{200D}'),
        "U+200D должен быть отфильтрован"
    );
}

/// Тест 12: Проверка кириллических символов
///
/// Кириллические символы разрешены (поддержка международных имён).
#[test]
fn test_unicode_cyrillic_filtering() {
    let cyrillic_name = "Игрок";
    let entry = LeaderboardEntry::new(cyrillic_name, 1000);

    // Кириллица разрешена для поддержки международных имён
    assert_eq!(
        entry.name(),
        "Игрок",
        "Кириллические символы должны проходить (поддержка международных имён)"
    );
}
