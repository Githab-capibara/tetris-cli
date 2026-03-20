//! Тесты обработки ошибок с ? оператором (controls.rs).
//!
//! Этот модуль содержит 5 тестов для проверки исправления:
//! - unwrap() заменён на ? оператор в тестах
//! - Корректная обработка ошибок сохранения
//! - Корректная обработка ошибок загрузки
//!
//! Исправление: использование ? оператора вместо unwrap() для лучшей обработки ошибок

use crate::controls::ControlsConfig;
use std::fs;
use std::io;
use std::path::Path;

// ============================================================================
// ГРУППА ТЕСТОВ: Обработка ошибок с ? оператором
// ============================================================================

/// Тест 1: Проверка корректной обработки ошибок сохранения
///
/// Проверяет, что ошибки сохранения корректно обрабатываются через ? оператор.
#[test]
fn test_корректная_обработка_ошибок_сохранения() -> io::Result<()> {
    // Используем относительный путь в текущей директории
    let test_path = "test_save_error_temp.json";

    // Создаём конфигурацию
    let config = ControlsConfig::default_config();

    // Сохраняем конфигурацию с использованием ? оператора
    config.save_to_file(test_path)?;

    // Проверяем что файл существует
    assert!(
        Path::new(test_path).exists(),
        "Файл должен существовать после сохранения"
    );

    // Проверяем что файл не пустой
    let metadata = fs::metadata(test_path)?;
    assert!(
        metadata.len() > 0,
        "Файл конфигурации не должен быть пустым"
    );

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);

    Ok(())
}

/// Тест 2: Проверка корректной обработки ошибок загрузки
///
/// Проверяет, что ошибки загрузки корректно обрабатываются через ? оператор.
#[test]
fn test_корректная_обработка_ошибок_загрузки() -> io::Result<()> {
    // Используем относительный путь
    let test_path = "test_load_error_temp.json";

    // Создаём и сохраняем конфигурацию
    let original_config = ControlsConfig::custom(
        b'x', b'z', b'c', b'v', b'a', b's', b'd', b'f', 127,
    );
    original_config.save_to_file(test_path)?;

    // Загружаем конфигурацию с использованием ? оператора
    let loaded_config = ControlsConfig::load_from_file(test_path)?;

    // Проверяем что загруженная конфигурация совпадает с оригиналом
    assert_eq!(
        original_config, loaded_config,
        "Загруженная конфигурация должна совпадать с оригиналом"
    );

    // Проверяем конкретные значения клавиш
    assert_eq!(loaded_config.move_left, b'x', "move_left должен быть 'x'");
    assert_eq!(loaded_config.move_right, b'z', "move_right должен быть 'z'");
    assert_eq!(loaded_config.soft_drop, b'c', "soft_drop должен быть 'c'");
    assert_eq!(loaded_config.hard_drop, b'v', "hard_drop должен быть 'v'");

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);

    Ok(())
}

/// Тест 3: Проверка работы с временными файлами
///
/// Проверяет полный цикл сохранения/загрузки с использованием временных файлов.
#[test]
fn test_работа_с_временными_файлами() -> io::Result<()> {
    // Используем относительный путь
    let test_path = "test_temp_files_temp.json";

    // Тест 1: Сохранение кастомной конфигурации
    let custom_config = ControlsConfig::custom(
        b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    );
    custom_config.save_to_file(test_path)?;

    // Проверяем что файл существует
    assert!(
        Path::new(test_path).exists(),
        "Временный файл должен существовать после сохранения"
    );

    // Тест 2: Загрузка и проверка
    let loaded = ControlsConfig::load_from_file(test_path)?;
    assert_eq!(
        custom_config, loaded,
        "Загруженная конфигурация должна совпадать"
    );

    // Тест 3: Перезапись файла с новой конфигурацией
    let new_config = ControlsConfig::custom(
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
    );
    new_config.save_to_file(test_path)?;

    let reloaded = ControlsConfig::load_from_file(test_path)?;
    assert_eq!(
        new_config, reloaded,
        "Перезаписанная конфигурация должна совпадать"
    );

    // Тест 4: Проверка валидации после загрузки
    assert!(
        reloaded.validate(),
        "Загруженная конфигурация должна быть валидной"
    );

    // Очищаем тестовый файл
    let _ = fs::remove_file(test_path);

    Ok(())
}

/// Тест 4: Проверка обработки ошибки загрузки несуществующего файла
///
/// Проверяет, что ? оператор корректно propagates ошибку.
#[test]
fn test_обработка_ошибки_несуществующего_файла() {
    // Пытаемся загрузить несуществующий файл
    let result = ControlsConfig::load_from_file("nonexistent_file_12345.json");

    // Проверяем что результат - ошибка
    assert!(
        result.is_err(),
        "Загрузка несуществующего файла должна вернуть ошибку"
    );

    // Проверяем тип ошибки
    let err = result.unwrap_err();
    assert!(
        err.kind() == io::ErrorKind::NotFound || err.kind() == io::ErrorKind::InvalidInput,
        "Ошибка должна быть NotFound или InvalidInput"
    );
}

/// Тест 5: Проверка обработки ошибки при некорректном JSON
///
/// Проверяет, что ? оператор корректно обрабатывает ошибки парсинга.
#[test]
fn test_обработка_ошибки_некорректного_json() -> io::Result<()> {
    // Создаём временную директорию
    let temp_dir = tempfile::tempdir()?;
    let test_path = temp_dir.path().join("test_invalid.json");

    // Записываем некорректный JSON
    fs::write(&test_path, "not valid json { broken")?;

    // Пытаемся загрузить - должна быть ошибка
    let result = ControlsConfig::load_from_file(test_path.to_str().expect("Путь должен быть валидным UTF-8"));

    // Проверяем что результат - ошибка
    assert!(
        result.is_err(),
        "Загрузка некорректного JSON должна вернуть ошибку"
    );

    // temp_dir автоматически очищается при выходе из области видимости
    Ok(())
}
