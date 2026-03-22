//! Тесты валидации путей (controls.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Запрет абсолютных путей
//! - Запрет path traversal (..)
//! - Разрешение относительных путей
//!
//! Валидация путей предотвращает запись файлов вне директории приложения.

use crate::controls::ControlsConfig;
use std::fs;

// ============================================================================
// ГРУППА ТЕСТОВ: Валидация путей
// ============================================================================

/// Тест 1: Проверка запрета абсолютных путей
///
/// Проверяет, что save_to_file и load_from_file возвращают ошибку
/// при попытке использовать абсолютный путь.
#[test]
fn test_запрет_абсолютных_путей() {
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
fn test_запрет_path_traversal() {
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
fn test_разрешение_относительных_путей() {
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
