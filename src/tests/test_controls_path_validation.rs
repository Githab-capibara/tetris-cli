//! Тесты валидации путей (controls.rs).
//!
//! Этот модуль содержит тесты для проверки исправления:
//! - Запрет абсолютных путей
//! - Запрет path traversal (..)
//! - Разрешение относительных путей
//! - Проверка null byte (\0)
//! - Проверка URL-encoding (%2e%2e)
//! - Проверка Unicode нормализации
//!
//! Валидация путей предотвращает запись файлов вне директории приложения.

use crate::controls::ControlsConfig;
use crate::validation::path::{PathErrorKind, PathValidator};
use std::fs;
use std::path::Path;

// ============================================================================
// ГРУППА ТЕСТОВ: Валидация путей
// ============================================================================

/// Тест 1: Проверка запрета абсолютных путей
///
/// Проверяет, что `save_to_file` и `load_from_file` возвращают ошибку
/// при попытке использовать абсолютный путь.
#[test]
fn test_absolute_paths_forbidden() {
    let config = ControlsConfig::default_config();

    // Тест 1: Попытка сохранить с абсолютным путём
    let result = config.save_to_file("/etc/malicious_config.json");
    assert!(
        result.is_err(),
        "Сохранение с абсолютным путём должно вернуть ошибку"
    );
    let err = result.unwrap_err();
    assert_eq!(
        err.kind(),
        std::io::ErrorKind::InvalidInput,
        "Ошибка должна быть InvalidInput"
    );
    assert!(
        err.to_string().contains("Абсолютные пути не разрешены"),
        "Сообщение об ошибке должно упоминать запрет абсолютных путей"
    );

    // Тест 2: Попытка загрузить с абсолютным путём
    let load_result = ControlsConfig::load_from_file("/etc/passwd");
    assert!(
        load_result.is_err(),
        "Загрузка с абсолютным путём должна вернуть ошибку"
    );
    let load_err = load_result.unwrap_err();
    assert_eq!(
        load_err.kind(),
        std::io::ErrorKind::InvalidInput,
        "Ошибка загрузки должна быть InvalidInput"
    );
}

/// Тест 2: Проверка запрета path traversal (..)
///
/// Проверяет, что нельзя использовать ".." для выхода за пределы директории.
#[test]
fn test_path_traversal_forbidden() {
    let config = ControlsConfig::default_config();

    // Тест 1: Попытка сохранить с path traversal
    let result = config.save_to_file("../../malicious_config.json");
    assert!(
        result.is_err(),
        "Сохранение с path traversal должно вернуть ошибку"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("Path traversal не разрешён"),
        "Сообщение об ошибке должно упоминать запрет path traversal"
    );

    // Тест 2: Попытка загрузить с path traversal
    let load_result = ControlsConfig::load_from_file("../../../etc/passwd");
    assert!(
        load_result.is_err(),
        "Загрузка с path traversal должна вернуть ошибку"
    );
    let load_err = load_result.unwrap_err();
    assert!(
        load_err.to_string().contains("Path traversal не разрешён"),
        "Сообщение об ошибке должно упоминать запрет path traversal"
    );

    // Тест 3: Path traversal в середине пути
    let result2 = config.save_to_file("config/../../malicious.json");
    assert!(
        result2.is_err(),
        "Path traversal в середине пути должен быть заблокирован"
    );
}

/// Тест 3: Проверка разрешения корректных относительных путей
///
/// Проверяет, что валидные относительные пути работают корректно.
#[test]
fn test_relative_paths_allowed() {
    let config = ControlsConfig::default_config();
    let test_path = "test_valid_path_config.json";

    // Тест 1: Сохранение с относительным путём
    let save_result = config.save_to_file(test_path);
    assert!(
        save_result.is_ok(),
        "Сохранение с относительным путём должно быть успешным"
    );

    // Тест 2: Загрузка с относительным путём
    let load_result = ControlsConfig::load_from_file(test_path);
    assert!(
        load_result.is_ok(),
        "Загрузка с относительным путём должна быть успешной"
    );

    // Проверяем что загруженная конфигурация совпадает (только клавиши)
    let loaded = load_result.expect("Загрузка должна быть успешной");
    assert!(
        loaded.keys_match(&config),
        "Загруженная конфигурация должна совпадать с оригиналом"
    );

    // Тест 3: Путь с поддиректорией (без ..)
    let nested_path = "configs/my_config.json";
    // Создаём директорию
    let _ = fs::create_dir_all("configs");
    let nested_result = config.save_to_file(nested_path);
    assert!(
        nested_result.is_ok(),
        "Сохранение в поддиректорию должно быть успешным"
    );

    // Очищаем тестовые файлы
    let _ = fs::remove_file(test_path);
    let _ = fs::remove_file(nested_path);
    let _ = fs::remove_dir("configs");
}

// ============================================================================
// ГРУППА ТЕСТОВ: Null byte проверки
// ============================================================================

/// Тест 4: Проверка пути с null byte (\0).
///
/// Проверяет что пути содержащие null байты отклоняются.
#[test]
fn test_path_with_null_byte() {
    let validator = PathValidator::new(
        255,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
    );

    // Тест 1: Null byte в конце пути
    let path_with_null = Path::new("config.json\0");
    let result = validator.validate_characters(path_with_null);
    assert!(result.is_err(), "Путь с null byte должен быть отклонён");
    if let Err(e) = result {
        assert_eq!(
            e.kind,
            PathErrorKind::NullByte,
            "Ошибка должна быть NullByte"
        );
        assert!(
            e.message.contains("null байт"),
            "Сообщение должно упоминать null байт"
        );
    }

    // Тест 2: Null byte в середине пути
    let path_with_null_mid = Path::new("con\0fig.json");
    let result2 = validator.validate_characters(path_with_null_mid);
    assert!(
        result2.is_err(),
        "Путь с null byte в середине должен быть отклонён"
    );

    // Тест 3: Multiple null bytes
    let path_with_nulls = Path::new("config\0.\0json");
    let result3 = validator.validate_characters(path_with_nulls);
    assert!(
        result3.is_err(),
        "Путь с несколькими null байтами должен быть отклонён"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ: URL-encoding проверки
// ============================================================================

/// Тест 5: Проверка пути с URL-encoding (%2e%2e).
///
/// Проверяет что URL-encoded пути не декодируются автоматически
/// и обрабатываются как обычные символы.
#[test]
fn test_path_with_url_encoding() {
    let validator = PathValidator::new(
        255,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
    );

    // Тест 1: URL-encoded path traversal (%2e%2e = ..)
    // Валидатор НЕ декодирует URL, поэтому %2e%2e не распознаётся как ..
    let url_encoded_path = "%2e%2e%2fetc%2fpasswd";
    let result = validator.validate_no_traversal(url_encoded_path);

    // Это ожидаемое поведение - валидатор не декодирует URL
    // Приложение должно декодировать пути ПЕРЕД валидацией если они приходят из URL
    assert!(
        result.is_ok(),
        "URL-encoded путь не распознаётся как path traversal (это ожидаемое поведение)"
    );

    // Тест 2: Смешанный стиль (часть URL-encoded, часть обычная)
    let mixed_path = "..%2fetc%2fpasswd";
    let result2 = validator.validate_no_traversal(mixed_path);
    // .. распознаётся и путь отклоняется
    assert!(result2.is_err(), "Смешанный путь с .. должен быть отклонён");

    // Тест 3: URL-encoded слэш (%2f = /)
    let url_encoded_slash = "config%2fconfig.json";
    let result3 = validator.validate_characters(Path::new(url_encoded_slash));
    // % не входит в разрешённые символы
    assert!(
        result3.is_err(),
        "URL-encoded символы должны быть отклонены (запрещённые символы)"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ: Unicode нормализация
// ============================================================================

/// Тест 6: Проверка пути с Unicode нормализацией.
///
/// Проверяет что Unicode пути в разной форме нормализации
/// обрабатываются корректно.
#[test]
fn test_path_with_unicode_normalization() {
    let validator = PathValidator::new(
        255,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
    );

    // Тест 1: Unicode символы вне разрешённого набора
    let unicode_path = Path::new("конфигурация.json");
    let result = validator.validate_characters(unicode_path);
    assert!(
        result.is_err(),
        "Unicode символы вне разрешённого набора должны быть отклонены"
    );
    if let Err(e) = result {
        assert_eq!(
            e.kind,
            PathErrorKind::ForbiddenCharacters,
            "Ошибка должна быть ForbiddenCharacters"
        );
    }

    // Тест 2: Unicode с разрешёнными символами (ASCII)
    let ascii_path = Path::new("config.json");
    let result2 = validator.validate_characters(ascii_path);
    assert!(result2.is_ok(), "ASCII путь должен быть валидным");

    // Тест 3: Unicode emoji (должны быть отклонены)
    let emoji_path = Path::new("config🎮.json");
    let result3 = validator.validate_characters(emoji_path);
    assert!(result3.is_err(), "Путь с emoji должен быть отклонён");

    // Тест 4: Unicode homoglyphs (похожие символы)
    // Кириллическая 'а' (U+0430) выглядит как латинская 'a' (U+0061)
    let homoglyph_path = Path::new("confiа.json"); // последняя 'а' - кириллическая
    let result4 = validator.validate_characters(homoglyph_path);
    assert!(
        result4.is_err(),
        "Путь с кириллическими homoglyphs должен быть отклонён"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ: Валидация корректных путей
// ============================================================================

/// Тест 7: Проверка валидации корректных путей.
///
/// Проверяет что различные варианты корректных путей проходят валидацию.
#[test]
fn test_valid_paths() {
    let validator = PathValidator::new(
        255,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
    );

    // Тест 1: Простой файл
    assert!(
        validator.validate(Path::new("config.json")).is_ok(),
        "config.json должен быть валидным"
    );

    // Тест 2: Путь с поддиректорией
    assert!(
        validator.validate(Path::new("configs/config.json")).is_ok(),
        "configs/config.json должен быть валидным"
    );

    // Тест 3: Путь с несколькими поддиректориями
    assert!(
        validator.validate(Path::new("a/b/c/d/config.json")).is_ok(),
        "a/b/c/d/config.json должен быть валидным"
    );

    // Тест 4: Путь с расширениями
    assert!(
        validator.validate(Path::new("config.test.json")).is_ok(),
        "config.test.json должен быть валидным"
    );

    // Тест 5: Путь с дефисами и подчёркиваниями
    assert!(
        validator
            .validate(Path::new("my-config_file.test-1.json"))
            .is_ok(),
        "my-config_file.test-1.json должен быть валидным"
    );

    // Тест 6: Пустой путь (должен проходить проверку длины)
    assert!(
        validator.validate_length(Path::new("")).is_ok(),
        "Пустой путь должен проходить проверку длины"
    );

    // Тест 7: Путь максимальной длины (255 символов)
    let max_length_path = "a".repeat(255);
    assert!(
        validator
            .validate_length(Path::new(&max_length_path))
            .is_ok(),
        "Путь длиной 255 символов должен быть валидным"
    );

    // Тест 8: Путь длиной 256 символов (должен отклоняться)
    let over_length_path = "a".repeat(256);
    let result = validator.validate_length(Path::new(&over_length_path));
    assert!(
        result.is_err(),
        "Путь длиной 256 символов должен быть отклонён"
    );
    if let Err(e) = result {
        assert_eq!(e.kind, PathErrorKind::TooLong);
    }
}
