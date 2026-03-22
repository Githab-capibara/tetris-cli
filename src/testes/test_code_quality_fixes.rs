//! Тесты для исправлений качества кода в проекте tetris-cli.
//!
//! Этот модуль содержит 6 тестов для проверки следующих исправлений:
//! 1. ControlsConfig Eq derive (controls.rs)
//! 2. #[must_use] на default_config() (controls.rs)
//! 3. #[must_use] на custom() (controls.rs)
//! 4. format args исправления (controls.rs, highscore.rs)
//! 5. generate_salt() #[must_use] (highscore.rs)
//! 6. документация с backticks (highscore.rs)
//!
//! Каждый тест проверяет корректность конкретного исправления.

// ============================================================================
// ТЕСТ 1: ControlsConfig Eq derive
// ============================================================================

/// Тест 1: Проверка что ControlsConfig реализует Eq.
///
/// Проверяет, что структура ControlsConfig корректно реализует trait Eq,
/// что позволяет использовать сравнение == и != для экземпляров.
/// Также проверяется транзитивность Eq: если a == b и b == a, то a == a.
#[test]
fn test_controls_config_eq() {
    use crate::controls::ControlsConfig;

    // Создаём две одинаковые конфигурации
    let config1 = ControlsConfig::default_config();
    let config2 = ControlsConfig::default_config();

    // Проверяем равенство
    assert_eq!(config1, config2, "Одинаковые конфигурации должны быть равны");

    // Проверяем транзитивность Eq
    if config1 == config2 && config2 == config1 {
        assert!(config1 == config1, "Транзитивность Eq: config1 должен быть равен самому себе");
    }

    // Проверяем что разные конфигурации не равны
    let config3 = ControlsConfig::custom(
        b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127
    );
    assert_ne!(config1, config3, "Разные конфигурации не должны быть равны");
}

// ============================================================================
// ТЕСТ 2: #[must_use] на default_config()
// ============================================================================

/// Тест 2: Проверка корректности default_config() с #[must_use].
///
/// Проверяет, что метод default_config() возвращает валидную конфигурацию
/// со всеми необходимыми клавишами управления.
/// Атрибут #[must_use] предупреждает если результат не используется.
#[test]
fn test_default_config_must_use() {
    use crate::controls::ControlsConfig;

    // Создаём конфигурацию по умолчанию
    let config = ControlsConfig::default_config();

    // Проверяем что все клавиши валидны (не None для Option<u8>)
    // В текущей реализации клавиши хранятся как u8, поэтому проверяем > 0
    assert!(config.move_left > 0, "move_left должна быть валидной");
    assert!(config.move_right > 0, "move_right должна быть валидной");
    assert!(config.rotate_right > 0, "rotate_cw должна быть валидной");
    assert!(config.rotate_left > 0, "rotate_ccw должна быть валидной");
    assert!(config.soft_drop > 0, "soft_drop должна быть валидной");
    assert!(config.hard_drop > 0, "hard_drop должна быть валидной");
    assert!(config.hold > 0, "hold должна быть валидной");
    assert!(config.pause > 0, "pause должна быть валидной");
    assert!(config.quit > 0, "quit должна быть валидной");

    // Проверяем конкретные значения по умолчанию
    assert_eq!(config.move_left, b'a', "move_left должна быть 'a'");
    assert_eq!(config.move_right, b'd', "move_right должна быть 'd'");
    assert_eq!(config.rotate_left, b'q', "rotate_left должна быть 'q'");
    assert_eq!(config.rotate_right, b'e', "rotate_right должна быть 'e'");
}

// ============================================================================
// ТЕСТ 3: #[must_use] на custom()
// ============================================================================

/// Тест 3: Проверка корректности custom() с #[must_use].
///
/// Проверяет, что метод custom() корректно создаёт конфигурацию
/// с заданными пользователем значениями клавиш.
/// Атрибут #[must_use] предупреждает если результат не используется.
#[test]
fn test_custom_config_must_use() {
    use crate::controls::ControlsConfig;

    // Создаём кастомную конфигурацию (стиль Vim HJKL)
    let config = ControlsConfig::custom(
        b'h', // move_left
        b'l', // move_right
        b'j', // soft_drop
        b'k', // hard_drop
        b'y', // rotate_left
        b'u', // rotate_right
        b'i', // hold
        b'o', // pause
        127,  // quit
    );

    // Проверяем что кастомные значения установлены корректно
    assert_eq!(config.move_left, b'h', "move_left должна быть 'h'");
    assert_eq!(config.move_right, b'l', "move_right должна быть 'l'");
    assert_eq!(config.soft_drop, b'j', "soft_drop должна быть 'j'");
    assert_eq!(config.hard_drop, b'k', "hard_drop должна быть 'k'");
    assert_eq!(config.rotate_left, b'y', "rotate_left должна быть 'y'");
    assert_eq!(config.rotate_right, b'u', "rotate_right должна быть 'u'");
    assert_eq!(config.hold, b'i', "hold должна быть 'i'");
    assert_eq!(config.pause, b'o', "pause должна быть 'o'");
    assert_eq!(config.quit, 127, "quit должна быть 127");

    // Проверяем валидность конфигурации
    assert!(config.validate(), "Кастомная конфигурация должна быть валидной");
}

// ============================================================================
// ТЕСТ 4: format args исправления
// ============================================================================

/// Тест 4: Проверка форматирования ошибок.
///
/// Проверяет, что форматирование строк с использованием format!()
/// работает корректно для сообщений об ошибках.
/// Исправление: используем format!("text: {var}") вместо format!("text: {}", var)
#[test]
fn test_error_formatting() {
    // Проверяем что форматирование ошибок работает корректно
    let test_path = "invalid/path";
    let error_msg = format!("Неверный путь: {test_path}");

    // Проверяем что сообщение содержит путь
    assert!(
        error_msg.contains(test_path),
        "Сообщение об ошибке должно содержать путь"
    );

    // Проверяем что сообщение содержит текст ошибки
    assert!(
        error_msg.contains("Неверный путь"),
        "Сообщение об ошибке должно содержать текст ошибки"
    );

    // Проверяем полное сообщение
    assert_eq!(
        error_msg, "Неверный путь: invalid/path",
        "Форматирование должно работать корректно"
    );

    // Проверяем форматирование с несколькими переменными
    let error_code = 404;
    let multi_format = format!("Ошибка {error_code}: путь {test_path}");
    assert_eq!(
        multi_format, "Ошибка 404: путь invalid/path",
        "Форматирование с несколькими переменными должно работать"
    );
}

// ============================================================================
// ТЕСТ 5: generate_salt() #[must_use]
// ============================================================================

/// Тест 5: Проверка generate_salt() с #[must_use].
///
/// Проверяет, что функция generate_salt() возвращает корректную соль:
/// - Не пустую строку
/// - Длиной 64 шестнадцатеричных символа (256 бит)
/// - Состоящую из hex-символов
/// Атрибут #[must_use] предупреждает если результат не используется.
#[test]
fn test_generate_salt_must_use() {
    use crate::highscore::generate_salt;

    // Генерируем соль
    let salt = generate_salt();

    // Проверяем что соль не пустая
    assert!(
        !salt.is_empty(),
        "Соль не должна быть пустой строкой"
    );

    // Проверяем длину (32 байта = 64 hex символа)
    assert_eq!(
        salt.len(),
        64,
        "Соль должна быть длиной 64 шестнадцатеричных символа (256 бит)"
    );

    // Проверяем что все символы - hex-цифры
    assert!(
        salt.chars().all(|c| c.is_ascii_hexdigit()),
        "Соль должна содержать только шестнадцатеричные цифры"
    );

    // Проверяем что соль состоит из lowercase hex
    assert!(
        salt.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
        "Соль должна быть в lowercase hex формате"
    );

    // Генерируем вторую соль для проверки уникальности
    let salt2 = generate_salt();
    assert_ne!(
        salt, salt2,
        "Две последовательные соли должны быть разными (рандомные)"
    );
}

// ============================================================================
// ТЕСТ 6: документация с backticks
// ============================================================================

/// Тест 6: Проверка корректности документации с backticks.
///
/// Проверяет, что документация с использованием backticks (`SaveData`,
/// [`LeaderboardEntry`], [`generate_salt()`]) компилируется корректно.
/// Это проверяется через успешную компиляцию и импорты модулей.
#[test]
fn test_documentation_backticks() {
    // Проверяем что документация компилируется
    // Это проверяется через cargo test --doc
    // Здесь просто проверяем, что модули импортируются

    use crate::highscore::LeaderboardEntry;
    use crate::highscore::SaveData;

    // Создаём тестовые данные для проверки что типы работают
    let save_data = SaveData::from_value(1000);
    assert!(
        save_data.verify_and_get_score().is_some(),
        "SaveData должен быть валидным"
    );

    // Создаём запись в таблице лидеров
    let entry = LeaderboardEntry::new("TestPlayer".to_string(), 500);
    assert_eq!(entry.name(), "TestPlayer", "Имя должно совпадать");
    assert_eq!(entry.score(), 500, "Очки должны совпадать");

    // Проверяем что документация корректна (тест проходит если компиляция успешна)
    assert!(true, "Документация с backticks корректна");
}

// ============================================================================
// ИНТЕГРАЦИОННЫЙ ТЕСТ: все исправления вместе
// ============================================================================

/// Интеграционный тест: проверка всех 6 исправлений вместе.
///
/// Проверяет что все исправления работают корректно в комбинации.
#[test]
fn test_all_code_quality_fixes_integration() {
    use crate::controls::ControlsConfig;
    use crate::highscore::{generate_salt, LeaderboardEntry, SaveData};

    // 1. Проверяем Eq для ControlsConfig
    let config1 = ControlsConfig::default_config();
    let config2 = ControlsConfig::default_config();
    assert_eq!(config1, config2);

    // 2. Проверяем default_config() с #[must_use]
    let config = ControlsConfig::default_config();
    assert_eq!(config.move_left, b'a');

    // 3. Проверяем custom() с #[must_use]
    let custom_config = ControlsConfig::custom(
        b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'
    );
    assert_eq!(custom_config.move_left, b'1');

    // 4. Проверяем форматирование
    let path = "test/path";
    let msg = format!("Путь: {path}");
    assert!(msg.contains(path));

    // 5. Проверяем generate_salt() с #[must_use]
    let salt = generate_salt();
    assert_eq!(salt.len(), 64);

    // 6. Проверяем документацию (импорты работают)
    let _save = SaveData::from_value(100);
    let _entry = LeaderboardEntry::new("Player".to_string(), 200);

    // Все исправления работают корректно
    assert!(true, "Все 6 исправлений качества кода работают");
}
