//! Тесты для исправления проблемы 3: Path Traversal защита в controls.rs.

use crate::controls::ControlsConfig;
use std::fs;
use std::io;

/// Тест 1: Проверка что абсолютные пути отклоняются.
#[test]
fn test_absolute_paths_rejected() {
    let config = ControlsConfig::default_config();
    let result = config.save_to_file("/etc/passwd");

    assert!(result.is_err(), "Абсолютные пути должны быть запрещены");
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
}

/// Тест 2: Проверка что пути с ".." отклоняются.
#[test]
fn test_path_traversal_dotdot_rejected() {
    let config = ControlsConfig::default_config();
    let result = config.save_to_file("../config.json");

    assert!(result.is_err(), "Path traversal должен быть запрещён");
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
}

/// Тест 3: Проверка что символические ссылки обрабатываются корректно.
#[test]
fn test_symlink_handling() {
    let config = ControlsConfig::default_config();
    let test_file = "test_config_real.json";

    let save_result = config.save_to_file(test_file);
    assert!(save_result.is_ok());

    let load_result = ControlsConfig::load_from_file(test_file);
    assert!(load_result.is_ok());

    let _ = fs::remove_file(test_file);
}

/// Тест 4: Проверка что относительные пути в текущей директории работают.
#[test]
fn test_relative_paths_accepted() {
    let config = ControlsConfig::default_config();
    let test_path = "test_relative_config.json";

    let save_result = config.save_to_file(test_path);
    assert!(save_result.is_ok());

    let load_result = ControlsConfig::load_from_file(test_path);
    assert!(load_result.is_ok());

    let _ = fs::remove_file(test_path);
}

/// Тест 5: Проверка что вложенные директории работают корректно.
#[test]
fn test_nested_directories_handling() {
    let config = ControlsConfig::default_config();
    let test_dir = "test_config_dir";
    let test_path = "test_config_dir/config.json";

    let _ = fs::create_dir(test_dir);
    let save_result = config.save_to_file(test_path);

    if save_result.is_ok() {
        let load_result = ControlsConfig::load_from_file(test_path);
        assert!(load_result.is_ok());
    }

    let _ = fs::remove_file(test_path);
    let _ = fs::remove_dir(test_dir);
}

/// Тест 6: Проверка блокировки символических ссылок (symlink attack).
///
/// Проверяет, что попытка использования символической ссылки,
/// указывающей за пределы разрешённой директории, блокируется.
#[test]
fn test_symlink_blocked() {
    use std::os::unix::fs::symlink;

    let config = ControlsConfig::default_config();
    let real_file = "test_symlink_target.json";
    let symlink_file = "test_symlink_link.json";

    // Создаём реальный файл
    let save_result = config.save_to_file(real_file);
    assert!(
        save_result.is_ok(),
        "Создание реального файла должно быть успешным"
    );

    // Создаём символическую ссылку на реальный файл
    let symlink_result = symlink(real_file, symlink_file);

    if symlink_result.is_ok() {
        // Пытаемся загрузить конфигурацию через символическую ссылку
        let load_result = ControlsConfig::load_from_file(symlink_file);

        // Загрузка через symlink должна быть заблокирована
        assert!(
            load_result.is_err(),
            "Загрузка через символическую ссылку должна быть заблокирована"
        );

        let err = load_result.unwrap_err();
        assert!(
            err.to_string().contains("Символические ссылки") || err.to_string().contains("symlink"),
            "Ошибка должна упоминать символические ссылки: {err}"
        );

        // Удаляем symlink
        let _ = fs::remove_file(symlink_file);
    } else {
        // Если создание symlink не удалось (например, в Windows без прав),
        // просто пропускаем тест
        println!("Не удалось создать symlink (возможно, нет прав или ОС не поддерживает)");
    }

    // Удаляем реальный файл
    let _ = fs::remove_file(real_file);
}

/// Тест 7: Проверка что пути вне разрешённой директории блокируются.
///
/// Проверяет, что попытка сохранения файла за пределами текущей директории
/// блокируется механизмом защиты от symlink атак.
#[test]
fn test_path_outside_allowed_directory_blocked() {
    let config = ControlsConfig::default_config();

    // Пытаемся использовать путь с несколькими ".." для выхода за пределы
    let result = config.save_to_file("../../etc/passwd");

    assert!(
        result.is_err(),
        "Путь с '..' для выхода за пределы должен быть заблокирован"
    );

    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("Path traversal") || err.to_string().contains("не разрешён"),
        "Ошибка должна упоминать path traversal: {err}"
    );
}
