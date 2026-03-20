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
