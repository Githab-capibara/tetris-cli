//! Тесты конфигурации управления.
//!
//! Этот модуль содержит 20 тестов для проверки системы конфигурации управления:
//! - Тесты значений по умолчанию (5 тестов)
//! - Тесты валидации (5 тестов)
//! - Тесты сохранения/загрузки (5 тестов)
//! - Тесты кастомизации (3 теста)
//! - Тесты граничных значений (2 теста)
//!
//! Все тесты независимы и используют временные файлы для тестирования сохранения.

use crate::controls::ControlsConfig;
use std::fs;
use std::path::Path;

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Значения по умолчанию
// ============================================================================

/// Тест 1: Проверка создания конфигурации по умолчанию через default_config()
///
/// Проверяет, что все клавиши имеют правильные значения согласно
/// стандартной раскладке WASD/QE.
#[test]
fn test_controls_default_config_values() {
    let config = ControlsConfig::default_config();

    // Проверка всех клавиш по умолчанию
    assert_eq!(
        config.move_left, b'a',
        "Движение влево должно быть 'a' (97)"
    );
    assert_eq!(
        config.move_right, b'd',
        "Движение вправо должно быть 'd' (100)"
    );
    assert_eq!(
        config.soft_drop, b's',
        "Мягкое падение должно быть 's' (115)"
    );
    assert_eq!(
        config.hard_drop, b'w',
        "Жёсткое падение должно быть 'w' (119)"
    );
    assert_eq!(
        config.rotate_left, b'q',
        "Вращение влево должно быть 'q' (113)"
    );
    assert_eq!(
        config.rotate_right, b'e',
        "Вращение вправо должно быть 'e' (101)"
    );
    assert_eq!(config.hold, b'c', "Удержание должно быть 'c' (99)");
    assert_eq!(config.pause, b'p', "Пауза должна быть 'p' (112)");
    assert_eq!(config.quit, 127, "Выход должен быть Backspace (127)");
}

/// Тест 2: Проверка создания конфигурации через trait Default
///
/// Проверяет, что реализация Default возвращает те же значения,
/// что и default_config().
#[test]
fn test_controls_default_trait() {
    let config_default = ControlsConfig::default();
    let config_explicit = ControlsConfig::default_config();

    assert_eq!(
        config_default.move_left, config_explicit.move_left,
        "Default должен совпадать с default_config() для move_left"
    );
    assert_eq!(
        config_default.move_right, config_explicit.move_right,
        "Default должен совпадать с default_config() для move_right"
    );
    assert_eq!(
        config_default.soft_drop, config_explicit.soft_drop,
        "Default должен совпадать с default_config() для soft_drop"
    );
    assert_eq!(
        config_default.hard_drop, config_explicit.hard_drop,
        "Default должен совпадать с default_config() для hard_drop"
    );
    assert_eq!(
        config_default.rotate_left, config_explicit.rotate_left,
        "Default должен совпадать с default_config() для rotate_left"
    );
    assert_eq!(
        config_default.rotate_right, config_explicit.rotate_right,
        "Default должен совпадать с default_config() для rotate_right"
    );
    assert_eq!(
        config_default.hold, config_explicit.hold,
        "Default должен совпадать с default_config() для hold"
    );
    assert_eq!(
        config_default.pause, config_explicit.pause,
        "Default должен совпадать с default_config() для pause"
    );
    assert_eq!(
        config_default.quit, config_explicit.quit,
        "Default должен совпадать с default_config() для quit"
    );
}

/// Тест 3: Проверка, что значения по умолчанию находятся в допустимом диапазоне
///
/// Все клавиши должны быть больше 0 и меньше 256 (u8 диапазон).
#[test]
fn test_controls_default_values_in_range() {
    let config = ControlsConfig::default_config();

    let keys = [
        config.move_left,
        config.move_right,
        config.soft_drop,
        config.hard_drop,
        config.rotate_left,
        config.rotate_right,
        config.hold,
        config.pause,
        config.quit,
    ];

    for (i, &key) in keys.iter().enumerate() {
        assert!(key > 0, "Клавиша {} должна быть больше 0", i);
        // Проверка на <= 255 избыточна для типа u8, но оставляем для документации
        let _ = key; // key используется для подавления предупреждения
    }
}

/// Тест 4: Проверка, что все клавиши по умолчанию уникальны
///
/// В конфигурации по умолчанию не должно быть дубликатов клавиш.
#[test]
fn test_controls_default_all_unique() {
    let config = ControlsConfig::default_config();

    let keys = [
        config.move_left,
        config.move_right,
        config.soft_drop,
        config.hard_drop,
        config.rotate_left,
        config.rotate_right,
        config.hold,
        config.pause,
        config.quit,
    ];

    // Проверяем каждую пару на уникальность
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i], keys[j],
                "Клавиши под индексами {} и {} не должны совпадать",
                i, j
            );
        }
    }
}

/// Тест 5: Проверка Clone для конфигурации
///
/// Проверяет, что клонирование конфигурации создаёт точную копию.
#[test]
fn test_controls_clone() {
    let original = ControlsConfig::default_config();
    let cloned = original.clone();

    assert_eq!(original.move_left, cloned.move_left);
    assert_eq!(original.move_right, cloned.move_right);
    assert_eq!(original.soft_drop, cloned.soft_drop);
    assert_eq!(original.hard_drop, cloned.hard_drop);
    assert_eq!(original.rotate_left, cloned.rotate_left);
    assert_eq!(original.rotate_right, cloned.rotate_right);
    assert_eq!(original.hold, cloned.hold);
    assert_eq!(original.pause, cloned.pause);
    assert_eq!(original.quit, cloned.quit);
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Валидация конфигурации
// ============================================================================

/// Тест 6: Проверка валидации корректной конфигурации
///
/// Конфигурация по умолчанию должна проходить валидацию.
#[test]
fn test_controls_validation_valid_config() {
    let config = ControlsConfig::default_config();
    assert!(
        config.validate(),
        "Конфигурация по умолчанию должна быть валидной"
    );
}

/// Тест 7: Проверка валидации конфигурации с дубликатами
///
/// Конфигурация с одинаковыми клавишами должна быть невалидной.
#[test]
fn test_controls_validation_duplicate_keys() {
    // Тест с дубликатом move_left и move_right
    let config_dup1 = ControlsConfig::custom(
        b'a', b'a', // Дубликат
        b's', b'w', b'q', b'e', b'c', b'p', 127,
    );
    assert!(
        !config_dup1.validate(),
        "Конфигурация с дубликатом move_left/move_right должна быть невалидной"
    );

    // Тест с дубликатом rotate_left и rotate_right
    let config_dup2 = ControlsConfig::custom(b'a', b'd', b's', b'w', b'q', b'q', b'c', b'p', 127);
    assert!(
        !config_dup2.validate(),
        "Конфигурация с дубликатом rotate_left/rotate_right должна быть невалидной"
    );

    // Тест с дубликатом hold и pause
    let config_dup3 = ControlsConfig::custom(b'a', b'd', b's', b'w', b'q', b'e', b'c', b'c', 127);
    assert!(
        !config_dup3.validate(),
        "Конфигурация с дубликатом hold/pause должна быть невалидной"
    );
}

/// Тест 8: Проверка валидации конфигурации с нулевыми значениями
///
/// Клавиши со значением 0 должны делать конфигурацию невалидной.
#[test]
fn test_controls_validation_zero_values() {
    // Тест с нулевым move_left
    let config_zero1 = ControlsConfig::custom(0, b'd', b's', b'w', b'q', b'e', b'c', b'p', 127);
    assert!(
        !config_zero1.validate(),
        "Конфигурация с нулевым move_left должна быть невалидной"
    );

    // Тест с нулевым quit
    let config_zero2 = ControlsConfig::custom(b'a', b'd', b's', b'w', b'q', b'e', b'c', b'p', 0);
    assert!(
        !config_zero2.validate(),
        "Конфигурация с нулевым quit должна быть невалидной"
    );

    // Тест с несколькими нулями
    let config_zero3 = ControlsConfig::custom(0, b'd', 0, b'w', b'q', b'e', b'c', b'p', 0);
    assert!(
        !config_zero3.validate(),
        "Конфигурация с несколькими нулями должна быть невалидной"
    );
}

/// Тест 9: Проверка валидации кастомной конфигурации без дубликатов
///
/// Кастомная конфигурация с уникальными ненулевыми клавишами должна быть валидной.
#[test]
fn test_controls_validation_custom_valid() {
    // Конфигурация в стиле Vim (HJKL)
    let vim_config = ControlsConfig::custom(b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127);
    assert!(
        vim_config.validate(),
        "Vim конфигурация должна быть валидной"
    );

    // Конфигурация с цифрами
    let numpad_config =
        ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');
    assert!(
        numpad_config.validate(),
        "Конфигурация с цифрами должна быть валидной"
    );

    // Конфигурация со специальными символами
    let special_config =
        ControlsConfig::custom(b'[', b']', b';', b'\'', b',', b'.', b'/', b'\\', b'`');
    assert!(
        special_config.validate(),
        "Конфигурация со спецсимволами должна быть валидной"
    );
}

/// Тест 10: Проверка валидации с максимальными значениями u8
///
/// Проверяет, что значения близкие к 255 корректно обрабатываются.
#[test]
fn test_controls_validation_max_values() {
    // Конфигурация с максимальными значениями
    let max_config = ControlsConfig::custom(248, 249, 250, 251, 252, 253, 254, 255, 247);
    assert!(
        max_config.validate(),
        "Конфигурация с максимальными значениями должна быть валидной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Сохранение и загрузка
// ============================================================================

/// Тест 11: Проверка сохранения конфигурации в файл
///
/// Проверяет, что конфигурация корректно сохраняется в JSON файл.
#[test]
fn test_controls_save_to_file() {
    let test_path = "test_controls_save.json";
    let config = ControlsConfig::default_config();

    // Сохраняем конфигурацию
    let save_result = config.save_to_file(test_path);
    assert!(
        save_result.is_ok(),
        "Сохранение должно быть успешным: {:?}",
        save_result.err()
    );

    // Проверяем, что файл существует
    assert!(
        Path::new(test_path).exists(),
        "Файл конфигурации должен существовать после сохранения"
    );

    // Проверяем, что файл не пустой
    let file_size = fs::metadata(test_path).unwrap().len();
    assert!(file_size > 0, "Файл конфигурации не должен быть пустым");

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);
}

/// Тест 12: Проверка загрузки конфигурации из файла
///
/// Проверяет, что сохранённая конфигурация корректно загружается.
#[test]
fn test_controls_load_from_file() {
    let test_path = "test_controls_load.json";
    let original_config =
        ControlsConfig::custom(b'x', b'z', b'c', b'v', b'a', b's', b'd', b'f', 127);

    // Сохраняем и загружаем
    let _ = original_config.save_to_file(test_path);
    let loaded_result = ControlsConfig::load_from_file(test_path);

    assert!(
        loaded_result.is_ok(),
        "Загрузка должна быть успешной: {:?}",
        loaded_result.err()
    );

    let loaded_config = loaded_result.unwrap();
    assert_eq!(
        original_config, loaded_config,
        "Загруженная конфигурация должна совпадать с оригиналом"
    );

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);
}

/// Тест 13: Проверка загрузки из несуществующего файла
///
/// Попытка загрузки из несуществующего файла должна возвращать ошибку.
#[test]
fn test_controls_load_nonexistent_file() {
    let result = ControlsConfig::load_from_file("nonexistent_file_12345.json");
    assert!(
        result.is_err(),
        "Загрузка из несуществующего файла должна возвращать ошибку"
    );
}

/// Тест 14: Проверка полного цикла сохранения и загрузки
///
/// Проверяет, что после сохранения и загрузки все значения совпадают.
#[test]
fn test_controls_save_load_cycle() {
    let test_path = "test_controls_cycle.json";

    // Создаём кастомную конфигурацию
    let original = ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    // Сохраняем
    assert!(
        original.save_to_file(test_path).is_ok(),
        "Сохранение должно быть успешным"
    );

    // Загружаем
    let loaded = ControlsConfig::load_from_file(test_path).expect("Загрузка должна быть успешной");

    // Сравниваем все поля
    assert_eq!(
        original.move_left, loaded.move_left,
        "move_left должен совпадать"
    );
    assert_eq!(
        original.move_right, loaded.move_right,
        "move_right должен совпадать"
    );
    assert_eq!(
        original.soft_drop, loaded.soft_drop,
        "soft_drop должен совпадать"
    );
    assert_eq!(
        original.hard_drop, loaded.hard_drop,
        "hard_drop должен совпадать"
    );
    assert_eq!(
        original.rotate_left, loaded.rotate_left,
        "rotate_left должен совпадать"
    );
    assert_eq!(
        original.rotate_right, loaded.rotate_right,
        "rotate_right должен совпадать"
    );
    assert_eq!(original.hold, loaded.hold, "hold должен совпадать");
    assert_eq!(original.pause, loaded.pause, "pause должен совпадать");
    assert_eq!(original.quit, loaded.quit, "quit должен совпадать");

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);
}

/// Тест 15: Проверка сохранения и загрузки с специальными символами
///
/// Проверяет корректность сериализации специальных символов.
#[test]
fn test_controls_save_load_special_chars() {
    let test_path = "test_controls_special.json";

    let original = ControlsConfig::custom(b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(');

    // Сохраняем и загружаем
    let _ = original.save_to_file(test_path);
    let loaded = ControlsConfig::load_from_file(test_path).expect("Загрузка должна быть успешной");

    assert_eq!(
        original, loaded,
        "Конфигурация со спецсимволами должна сохраняться корректно"
    );

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-18: Кастомизация конфигурации
// ============================================================================

/// Тест 16: Проверка создания конфигурации в стиле Vim (HJKL)
///
/// Проверяет создание и валидацию популярной альтернативной раскладки.
#[test]
fn test_controls_custom_vim_style() {
    let vim_config = ControlsConfig::custom(
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

    assert_eq!(vim_config.move_left, b'h', "Vim move_left должен быть 'h'");
    assert_eq!(
        vim_config.move_right, b'l',
        "Vim move_right должен быть 'l'"
    );
    assert_eq!(vim_config.soft_drop, b'j', "Vim soft_drop должен быть 'j'");
    assert_eq!(vim_config.hard_drop, b'k', "Vim hard_drop должен быть 'k'");
    assert!(
        vim_config.validate(),
        "Vim конфигурация должна быть валидной"
    );
}

/// Тест 17: Проверка создания конфигурации с цифровой клавиатурой
///
/// Проверяет использование цифр для управления.
#[test]
fn test_controls_custom_numpad_style() {
    let numpad_config = ControlsConfig::custom(
        b'4', // move_left (стрелка влево на нумпаде)
        b'6', // move_right (стрелка вправо на нумпаде)
        b'5', // soft_drop (стрелка вниз на нумпаде)
        b'8', // hard_drop (стрелка вверх на нумпаде)
        b'1', // rotate_left
        b'3', // rotate_right
        b'0', // hold
        b'9', // pause
        b'7', // quit
    );

    assert_eq!(numpad_config.move_left, b'4');
    assert_eq!(numpad_config.move_right, b'6');
    assert_eq!(numpad_config.soft_drop, b'5');
    assert_eq!(numpad_config.hard_drop, b'8');
    assert!(
        numpad_config.validate(),
        "Нумпад конфигурация должна быть валидной"
    );
}

/// Тест 18: Проверка отличия кастомной конфигурации от стандартной
///
/// Проверяет, что кастомная конфигурация отличается от default.
#[test]
fn test_controls_custom_differs_from_default() {
    let default_config = ControlsConfig::default_config();
    let custom_config =
        ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    // Проверяем, что хотя бы одна клавиша отличается
    let differs = default_config.move_left != custom_config.move_left
        || default_config.move_right != custom_config.move_right
        || default_config.soft_drop != custom_config.soft_drop
        || default_config.hard_drop != custom_config.hard_drop
        || default_config.rotate_left != custom_config.rotate_left
        || default_config.rotate_right != custom_config.rotate_right
        || default_config.hold != custom_config.hold
        || default_config.pause != custom_config.pause
        || default_config.quit != custom_config.quit;

    assert!(
        differs,
        "Кастомная конфигурация должна отличаться от стандартной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 19-20: Граничные значения
// ============================================================================

/// Тест 19: Проверка минимальных допустимых значений
///
/// Проверяет, что значение 1 является минимальным допустимым.
#[test]
fn test_controls_boundary_min_values() {
    // Конфигурация с минимальными значениями (1)
    let min_config = ControlsConfig::custom(1, 2, 3, 4, 5, 6, 7, 8, 9);
    assert!(
        min_config.validate(),
        "Конфигурация с минимальными значениями должна быть валидной"
    );

    // Конфигурация с нулём должна быть невалидной
    let zero_config = ControlsConfig::custom(0, 2, 3, 4, 5, 6, 7, 8, 9);
    assert!(
        !zero_config.validate(),
        "Конфигурация с нулевым значением должна быть невалидной"
    );
}

/// Тест 20: Проверка максимальных допустимых значений
///
/// Проверяет, что значение 255 является максимальным допустимым.
#[test]
fn test_controls_boundary_max_values() {
    // Конфигурация с максимальными уникальными значениями
    let max_config = ControlsConfig::custom(255, 254, 253, 252, 251, 250, 249, 248, 247);
    assert!(
        max_config.validate(),
        "Конфигурация с максимальными значениями должна быть валидной"
    );

    // Проверяем каждое поле
    assert_eq!(max_config.move_left, 255);
    assert_eq!(max_config.move_right, 254);
    assert_eq!(max_config.soft_drop, 253);
    assert_eq!(max_config.hard_drop, 252);
    assert_eq!(max_config.rotate_left, 251);
    assert_eq!(max_config.rotate_right, 250);
    assert_eq!(max_config.hold, 249);
    assert_eq!(max_config.pause, 248);
    assert_eq!(max_config.quit, 247);
}
