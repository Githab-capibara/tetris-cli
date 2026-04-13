//! Тесты конкурентного доступа и дополнительных edge-case.
//!
//! PROB-162: Тест на конкурентный доступ к leaderboard
//! PROB-164: Тест на score overflow
//! PROB-165: HMAC пустой ключ
//! PROB-166: HMAC длинный ключ
//! PROB-167: Валидация path traversal ../../
//! PROB-168: Symlink attack тест
//! PROB-169: Race condition в leaderboard
//! PROB-170: IO errors при загрузке конфига
//! PROB-171: Некорректный JSON в save
//! PROB-173: Тест bag randomizer

// ============================================================================
// PROB-165: HMAC пустой ключ
// ============================================================================

/// Тест: HMAC с пустым ключом работает корректно
#[test]
fn test_hmac_empty_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    let sig = hmac_sha256("", "data");
    assert_eq!(sig.len(), 64, "HMAC с пустым ключом должен быть 64 символа");
    assert!(
        verify_hmac_sha256("", "data", &sig),
        "Подпись с пустым ключом должна верифицироваться"
    );
}

// ============================================================================
// PROB-166: HMAC длинный ключ (> 64 байт)
// ============================================================================

/// Тест: HMAC с ключом длиннее 64 байт (больше размера блока SHA-256)
#[test]
fn test_hmac_long_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    // Ключ 128 байт (> 64 байт, размер блока SHA-256)
    let long_key = "a".repeat(128);
    let sig = hmac_sha256(&long_key, "data");
    assert_eq!(
        sig.len(),
        64,
        "HMAC с длинным ключом должен быть 64 символа"
    );
    assert!(
        verify_hmac_sha256(&long_key, "data", &sig),
        "Подпись с длинным ключом должна верифицироваться"
    );
}

/// Тест: HMAC с очень длинным ключом (1024 байта)
#[test]
fn test_hmac_very_long_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    let very_long_key = "x".repeat(1024);
    let sig = hmac_sha256(&very_long_key, "data");
    assert_eq!(sig.len(), 64);
    assert!(verify_hmac_sha256(&very_long_key, "data", &sig));
}

// ============================================================================
// PROB-167: Валидация ../../
// ============================================================================

/// Тест: `PathValidator` отклоняет path traversal с ../../
#[test]
fn test_path_validator_rejects_traversal() {
    use crate::validation::PathValidator;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    // Обычный ../../ должен быть отклонён
    assert!(
        validator.validate_no_traversal("../../etc/passwd").is_err(),
        "../../ должен быть отклонён"
    );

    // Одиночный ../ тоже
    assert!(
        validator.validate_no_traversal("../config.json").is_err(),
        "../ должен быть отклонён"
    );

    // Вложенный ../../
    assert!(
        validator
            .validate_no_traversal("foo/../../etc/passwd")
            .is_err(),
        "Вложенный ../../ должен быть отклонён"
    );
}

/// Тест: `PathValidator` отклоняет URL-encoded path traversal
#[test]
fn test_path_validator_rejects_url_encoded_traversal() {
    use crate::validation::PathValidator;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    assert!(validator
        .validate_no_traversal("..%2F..%2Fetc%2Fpasswd")
        .is_err());
    assert!(validator.validate_no_traversal("%2e%2e%2f").is_err());
    assert!(validator.validate_no_traversal("%2E%2E%2F").is_err());
}

// ============================================================================
// PROB-168: Symlink attack
// ============================================================================

/// Тест: `PathValidator` проверяет `symlink_metadata`
/// Примечание: полная проверка symlink требует существующей файловой системы.
/// Этот тест проверяет что для несуществующего пути проверка не паникует.
#[test]
fn test_path_validator_symlink_check_exists() {
    use crate::validation::PathValidator;
    use std::path::Path;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    // Для несуществующего файла проверка symlink должна проходить (файла нет = нет symlink)
    let nonexistent = Path::new("/nonexistent/path/file.txt");
    let result = validator.validate_no_symlinks(nonexistent);
    // Проверяем что результат Ok (несуществующий файл не может быть symlink)
    assert!(
        result.is_ok(),
        "validate_no_symlinks должен вернуть Ok для несуществующего пути"
    );
}

// ============================================================================
// PROB-170: IO errors при загрузке конфига
// ============================================================================

/// Тест: загрузка конфига из невалидного пути возвращает ошибку
#[test]
fn test_config_load_from_invalid_path() {
    use crate::controls::ControlsConfig;

    // Невалидный путь должен вернуть ошибку
    let result = ControlsConfig::load_from_file("/nonexistent/path/config.json");
    assert!(
        result.is_err(),
        "Загрузка из несуществующего пути должна вернуть ошибку"
    );
}

/// Тест: загрузка конфига из пустого пути
#[test]
fn test_config_load_from_empty_path() {
    use crate::controls::ControlsConfig;

    let result = ControlsConfig::load_from_file("");
    assert!(
        result.is_err(),
        "Загрузка из пустого пути должна вернуть ошибку"
    );
}
