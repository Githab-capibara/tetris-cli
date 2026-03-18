//! Тесты конфигурации управления в Tetris CLI.
//!
//! Этот модуль содержит 25 тестов для проверки системы управления:
//! - Тесты конфигурации по умолчанию (5 тестов)
//! - Тесты кастомной конфигурации (5 тестов)
//! - Тесты валидации (дубликаты, диапазон) (5 тестов)
//! - Тесты сохранения/загрузки JSON (5 тестов)
//! - Тесты граничных значений (5 тестов)
//!
//! Все тесты проверяют корректность системы конфигурации управления.

use crate::controls::ControlsConfig;
use std::fs;
use std::path::Path;

// ============================================================================
// ГРУППА ТЕСТОВ 1-5: Конфигурация по умолчанию
// ============================================================================

/// Тест 1: Конфигурация по умолчанию создаётся
#[test]
fn test_default_config_creation() {
    let config = ControlsConfig::default_config();

    assert_eq!(config.move_left, b'a', "Движение влево должно быть 'a'");
    assert_eq!(config.move_right, b'd', "Движение вправо должно быть 'd'");
}

/// Тест 2: Все клавиши по умолчанию заданы
#[test]
fn test_all_default_keys_set() {
    let config = ControlsConfig::default_config();

    assert_eq!(config.move_left, b'a');
    assert_eq!(config.move_right, b'd');
    assert_eq!(config.soft_drop, b's');
    assert_eq!(config.hard_drop, b'w');
    assert_eq!(config.rotate_left, b'q');
    assert_eq!(config.rotate_right, b'e');
    assert_eq!(config.hold, b'c');
    assert_eq!(config.pause, b'p');
    assert_eq!(config.quit, 127);
}

/// Тест 3: Конфигурация по умолчанию валидна
#[test]
fn test_default_config_valid() {
    let config = ControlsConfig::default_config();

    assert!(
        config.validate(),
        "Конфигурация по умолчанию должна быть валидной"
    );
}

/// Тест 4: Default trait для ControlsConfig
#[test]
fn test_default_trait() {
    let config = ControlsConfig::default();

    assert_eq!(config.move_left, b'a');
    assert_eq!(config.move_right, b'd');
}

/// Тест 5: Clone для конфигурации
#[test]
fn test_config_clone() {
    let config = ControlsConfig::default_config();
    let cloned = config.clone();

    assert_eq!(config.move_left, cloned.move_left);
    assert_eq!(config.move_right, cloned.move_right);
}

// ============================================================================
// ГРУППА ТЕСТОВ 6-10: Кастомная конфигурация
// ============================================================================

/// Тест 6: Кастомная конфигурация создаётся
#[test]
fn test_custom_config_creation() {
    let config = ControlsConfig::custom(b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127);

    assert_eq!(config.move_left, b'h', "Движение влево должно быть 'h'");
    assert_eq!(config.move_right, b'l', "Движение вправо должно быть 'l'");
}

/// Тест 7: Кастомная конфигурация Vim стиля
#[test]
fn test_vim_style_config() {
    let config = ControlsConfig::custom(
        b'h', // left
        b'l', // right
        b'j', // soft drop
        b'k', // hard drop
        b'y', // rotate left
        b'u', // rotate right
        b'i', // hold
        b'o', // pause
        127,  // quit
    );

    assert_eq!(config.move_left, b'h');
    assert_eq!(config.move_right, b'l');
    assert_eq!(config.soft_drop, b'j');
    assert_eq!(config.hard_drop, b'k');
}

/// Тест 8: Кастомная конфигурация с цифрами
#[test]
fn test_numpad_config() {
    let config = ControlsConfig::custom(b'4', b'6', b'5', b'8', b'1', b'3', b'0', b'9', b'7');

    assert_eq!(config.move_left, b'4');
    assert_eq!(config.move_right, b'6');
}

/// Тест 9: Кастомная конфигурация валидна
#[test]
fn test_custom_config_valid() {
    let config = ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    assert!(
        config.validate(),
        "Кастомная конфигурация без дубликатов должна быть валидной"
    );
}

/// Тест 10: Кастомная конфигурация отличается от default
#[test]
fn test_custom_config_different_from_default() {
    let custom = ControlsConfig::custom(b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127);
    let default = ControlsConfig::default_config();

    assert_ne!(
        custom.move_left, default.move_left,
        "Кастомная конфигурация должна отличаться от стандартной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 11-15: Валидация (дубликаты, диапазон)
// ============================================================================

/// Тест 11: Валидация обнаруживает дубликаты
#[test]
fn test_validation_detects_duplicates() {
    let config = ControlsConfig::custom(
        b'a', b'a', // Дубликат: обе клавиши 'a'
        b's', b'w', b'q', b'e', b'c', b'p', 127,
    );

    assert!(
        !config.validate(),
        "Конфигурация с дубликатами должна быть невалидной"
    );
}

/// Тест 12: Валидация обнаруживает нулевые значения
#[test]
fn test_validation_detects_zero_values() {
    let config = ControlsConfig::custom(0, b'd', b's', b'w', b'q', b'e', b'c', b'p', 127);

    assert!(
        !config.validate(),
        "Конфигурация с нулевой клавишей должна быть невалидной"
    );
}

/// Тест 13: Валидация пропускает валидную конфигурацию
#[test]
fn test_validation_passes_valid_config() {
    let config = ControlsConfig::default_config();

    assert!(
        config.validate(),
        "Валидная конфигурация должна проходить проверку"
    );
}

/// Тест 14: Валидация всех клавиш уникальны
#[test]
fn test_validation_all_keys_unique() {
    let config = ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    assert!(config.validate(), "Все клавиши должны быть уникальны");
}

/// Тест 15: Валидация множественные дубликаты
#[test]
fn test_validation_multiple_duplicates() {
    let config = ControlsConfig::custom(
        b'a', b'a', // Дубликат 1
        b'a', b'w', // Дубликат 2
        b'q', b'e', b'c', b'p', 127,
    );

    assert!(
        !config.validate(),
        "Конфигурация с множественными дубликатами должна быть невалидной"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 16-20: Сохранение/загрузка JSON
// ============================================================================

/// Тест 16: Сохранение конфигурации в файл
#[test]
fn test_save_config_to_file() {
    let test_path = "test_controls_save.json";
    let config = ControlsConfig::default_config();

    let result = config.save_to_file(test_path);

    assert!(result.is_ok(), "Сохранение должно быть успешным");
    assert!(
        Path::new(test_path).exists(),
        "Файл должен существовать после сохранения"
    );

    // Очищаем
    let _ = fs::remove_file(test_path);
}

/// Тест 17: Загрузка конфигурации из файла
#[test]
fn test_load_config_from_file() {
    let test_path = "test_controls_load.json";
    let original = ControlsConfig::custom(b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127);

    // Сохраняем
    let _ = original.save_to_file(test_path);

    // Загружаем
    let loaded = ControlsConfig::load_from_file(test_path);

    assert!(loaded.is_ok(), "Загрузка должна быть успешной");
    assert_eq!(
        original.move_left,
        loaded.expect("Загрузка должна быть успешной").move_left,
        "Загруженная конфигурация должна совпадать"
    );

    // Очищаем
    let _ = fs::remove_file(test_path);
}

/// Тест 18: Сохранение и загрузка полный цикл
#[test]
fn test_save_load_full_cycle() {
    let test_path = "test_controls_cycle.json";

    let original = ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    // Сохраняем
    let _ = original.save_to_file(test_path);

    // Загружаем
    let loaded = ControlsConfig::load_from_file(test_path).expect("Загрузка должна быть успешной");

    // Сравниваем
    assert_eq!(original, loaded, "Конфигурации должны совпадать");

    // Очищаем
    let _ = fs::remove_file(test_path);
}

/// Тест 19: Загрузка несуществующего файла
#[test]
fn test_load_nonexistent_file() {
    let result = ControlsConfig::load_from_file("nonexistent_file.json");

    assert!(
        result.is_err(),
        "Загрузка несуществующего файла должна вернуть ошибку"
    );
}

/// Тест 20: Сохранение разных конфигураций
#[test]
fn test_save_different_configs() {
    let test_path1 = "test_controls_1.json";
    let test_path2 = "test_controls_2.json";

    let config1 = ControlsConfig::default_config();
    let config2 = ControlsConfig::custom(b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127);

    let _ = config1.save_to_file(test_path1);
    let _ = config2.save_to_file(test_path2);

    let loaded1 = ControlsConfig::load_from_file(test_path1)
        .expect("Загрузка конфигурации 1 должна быть успешной");
    let loaded2 = ControlsConfig::load_from_file(test_path2)
        .expect("Загрузка конфигурации 2 должна быть успешной");

    assert_ne!(
        loaded1.move_left, loaded2.move_left,
        "Разные конфигурации должны загрузиться разными"
    );

    // Очищаем
    let _ = fs::remove_file(test_path1);
    let _ = fs::remove_file(test_path2);
}

// ============================================================================
// ГРУППА ТЕСТОВ 21-25: Граничные значения
// ============================================================================

/// Тест 21: Минимальное допустимое значение клавиши
#[test]
fn test_minimum_valid_key_value() {
    // Минимальное значение - 1 (0 невалидно)
    let config = ControlsConfig::custom(1, b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');

    assert!(config.validate(), "Значение 1 должно быть валидным");
}

/// Тест 22: Максимальное допустимое значение клавиши
#[test]
fn test_maximum_valid_key_value() {
    // Максимальное значение для u8 - 255
    let config = ControlsConfig::custom(255, 254, 253, 252, 251, 250, 249, 248, 247);

    assert!(
        config.validate(),
        "Высокие значения u8 должны быть валидными"
    );
}

/// Тест 23: Нулевое значение невалидно
#[test]
fn test_zero_value_invalid() {
    let config = ControlsConfig::custom(0, b'd', b's', b'w', b'q', b'e', b'c', b'p', 127);

    assert!(
        !config.validate(),
        "Нулевое значение должно быть невалидным"
    );
}

/// Тест 24: Все клавиши одинаковые невалидно
#[test]
fn test_all_same_keys_invalid() {
    let config = ControlsConfig::custom(b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a');

    assert!(
        !config.validate(),
        "Все одинаковые клавиши должны быть невалидны"
    );
}

/// Тест 25: Граничные ASCII значения
#[test]
fn test_boundary_ascii_values() {
    // Проверяем различные ASCII значения
    let config = ControlsConfig::custom(
        b'!', // 33 - минимальный печатный символ
        b'~', // 126 - максимальный печатный символ
        b'0', // 48
        b'9', // 57
        b'A', // 65
        b'Z', // 90
        b'a', // 97
        b'z', // 122
        127,  // Backspace
    );

    assert!(
        config.validate(),
        "Граничные ASCII значения должны быть валидными"
    );
}
