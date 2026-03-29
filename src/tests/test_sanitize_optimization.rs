//! Тесты оптимизации sanitize_player_name.
//!
//! Проверяют фильтрацию опасных Unicode-символов и производительность.

use crate::highscore::leaderboard::LeaderboardEntry;

/// Тест 1: Проверка фильтрации zero-width joiners (U+200C, U+200D)
///
/// Zero-width joiners должны отфильтровываться из имени.
#[test]
fn test_sanitize_zero_width_joiners() {
    // Имя с zero-width joiner U+200C
    let name_with_zwnj = "Player\u{200C}Name";
    let entry = LeaderboardEntry::new(name_with_zwnj, 1000);

    assert!(
        !entry.name().contains('\u{200C}'),
        "Zero-width joiner U+200C должен быть отфильтрован"
    );

    // Имя с zero-width joiner U+200D
    let name_with_zwj = "Player\u{200D}Name";
    let entry2 = LeaderboardEntry::new(name_with_zwj, 1000);

    assert!(
        !entry2.name().contains('\u{200D}'),
        "Zero-width joiner U+200D должен быть отфильтрован"
    );
}

/// Тест 2: Проверка фильтрации variation selectors (U+FE00-U+FE0F)
///
/// Variation selectors должны отфильтровываться из имени.
#[test]
fn test_sanitize_variation_selectors() {
    // Имя с variation selector U+FE00
    let name_with_vs1 = "Player\u{FE00}Name";
    let entry = LeaderboardEntry::new(name_with_vs1, 1000);

    assert!(
        !entry.name().contains('\u{FE00}'),
        "Variation selector U+FE00 должен быть отфильтрован"
    );

    // Имя с variation selector U+FE0F
    let name_with_vs16 = "Player\u{FE0F}Name";
    let entry2 = LeaderboardEntry::new(name_with_vs16, 1000);

    assert!(
        !entry2.name().contains('\u{FE0F}'),
        "Variation selector U+FE0F должен быть отфильтрован"
    );

    // Проверяем весь диапазон variation selectors
    for vs in 0xFE00..=0xFE0F {
        let ch =
            char::from_u32(vs).expect("Failed to convert variation selector codepoint to char");
        let name = format!("Player{}Name", ch);
        let entry = LeaderboardEntry::new(&name, 1000);

        assert!(
            !entry.name().contains(ch),
            "Variation selector U+{vs:04X} должен быть отфильтрован"
        );
    }
}

/// Тест 3: Проверка фильтрации bidirectional control characters
///
/// Bidirectional control characters должны отфильтровываться.
#[test]
fn test_sanitize_bidi_control_chars() {
    // U+200E - LEFT-TO-RIGHT MARK
    let name_with_lrm = "Player\u{200E}Name";
    let entry = LeaderboardEntry::new(name_with_lrm, 1000);
    assert!(
        !entry.name().contains('\u{200E}'),
        "LEFT-TO-RIGHT MARK U+200E должен быть отфильтрован"
    );

    // U+200F - RIGHT-TO-LEFT MARK
    let name_with_rlm = "Player\u{200F}Name";
    let entry2 = LeaderboardEntry::new(name_with_rlm, 1000);
    assert!(
        !entry2.name().contains('\u{200F}'),
        "RIGHT-TO-LEFT MARK U+200F должен быть отфильтрован"
    );
}

/// Тест 4: Проверка допустимых символов
///
/// Проверяем, что допустимые символы проходят фильтрацию.
#[test]
fn test_sanitize_allowed_chars() {
    // Допустимые символы: ASCII буквы, цифры, '_', '-', ' '
    let valid_names = [
        "Player",
        "Player123",
        "Player_Name",
        "Player-Name",
        "Player Name",
        "P1_A-B C",
        "Anonymous", // reserved name
    ];

    for &name in &valid_names {
        let entry = LeaderboardEntry::new(name, 1000);
        assert_eq!(
            entry.name(),
            name,
            "Допустимое имя '{name}' должно пройти без изменений"
        );
    }
}

/// Тест 5: Проверка производительности sanitize_player_name
///
/// Бенчмарк: фильтрация должна быть быстрой.
#[test]
fn test_sanitize_performance() {
    use std::time::Instant;

    let test_names = [
        "NormalName",
        "Player\u{200C}\u{200D}Name", // с zero-width joiners
        "Player\u{FE00}\u{FE0F}Name", // с variation selectors
        "Player\u{200E}\u{200F}Name", // с bidi controls
        "ИмяСПлохимиСимволами\u{200C}",
    ];

    let iterations = 10000;
    let start = Instant::now();

    for _ in 0..iterations {
        for &name in &test_names {
            let _entry = LeaderboardEntry::new(name, 1000);
        }
    }

    let elapsed = start.elapsed();

    // 10000 итераций × 5 имён должны выполняться < 5000ms (увеличенный таймаут для стабильности на медленных системах)
    assert!(
        elapsed.as_millis() < 5000,
        "Санитизация {iterations} итераций должна выполняться < 5000ms (прошло {:?})",
        elapsed
    );
}

/// Тест 6: Проверка замены пустого имени на "Anonymous"
///
/// Пустое имя или имя только из запрещённых символов
/// должно заменяться на "Anonymous".
#[test]
fn test_sanitize_empty_to_anonymous() {
    // Пустое имя
    let entry = LeaderboardEntry::new("", 1000);
    assert_eq!(
        entry.name(),
        "Anonymous",
        "Пустое имя должно заменяться на 'Anonymous'"
    );

    // Имя только из пробелов
    let entry2 = LeaderboardEntry::new("   ", 1000);
    assert_eq!(
        entry2.name(),
        "Anonymous",
        "Имя из пробелов должно заменяться на 'Anonymous'"
    );

    // Имя только из запрещённых символов
    let entry3 = LeaderboardEntry::new("\u{200C}\u{200D}\u{FE00}", 1000);
    assert_eq!(
        entry3.name(),
        "Anonymous",
        "Имя из запрещённых символов должно заменяться на 'Anonymous'"
    );
}

/// Тест 7: Проверка ограничения длины имени (20 символов)
///
/// Имя длиннее 20 символов должно обрезаться.
#[test]
fn test_sanitize_max_length() {
    let long_name = "ThisIsAVeryLongPlayerNameThatExceedsTwentyCharacters";
    let entry = LeaderboardEntry::new(long_name, 1000);

    assert!(
        entry.name().len() <= 20,
        "Длина имени должна быть <= 20 символов, получено {}",
        entry.name().len()
    );

    // Проверяем, что имя обрезано корректно (20 символов)
    assert_eq!(
        entry.name(),
        "ThisIsAVeryLongPlaye",
        "Имя должно быть обрезано до 20 символов"
    );
}

/// Тест 8: Проверка фильтрации emoji
///
/// Emoji и другие опасные Unicode-символы должны отфильтровываться.
#[test]
fn test_sanitize_emoji_filtering() {
    // Имя с emoji
    let name_with_emoji = "Player😀Name";
    let entry = LeaderboardEntry::new(name_with_emoji, 1000);

    // Emoji должны отфильтровываться (не входят в whitelist)
    assert!(
        !entry.name().contains('😀'),
        "Emoji должен быть отфильтрован"
    );

    // Проверяем, что допустимые символы остались
    assert!(
        entry.name().contains("Player"),
        "Допустимые символы должны остаться"
    );
    assert!(
        entry.name().contains("Name"),
        "Допустимые символы должны остаться"
    );
}
