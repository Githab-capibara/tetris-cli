//! Модуль валидации путей к файлам.
//!
//! Предоставляет структуры и функции для безопасной валидации путей:
//! - Проверка длины пути
//! - Проверка разрешённых символов
//! - Защита от symlink атак
//! - Защита от path traversal (..)
//!
//! ## Структуры
//! - [`PathValidator`] — валидатор путей
//! - [`PathError`] — ошибка валидации
//! - [`PathErrorKind`] — тип ошибки валидации

use std::io;
use std::path::Path;

/// Ошибка валидации пути.
///
/// Содержит информацию о причине ошибки валидации.
#[derive(Debug, Clone)]
pub struct PathError {
    /// Сообщение об ошибке.
    pub message: String,
    /// Тип ошибки.
    pub kind: PathErrorKind,
}

/// Типы ошибки валидации пути.
#[derive(Debug, Clone)]
pub enum PathErrorKind {
    /// Путь слишком длинный.
    TooLong,
    /// Запрещённые символы в пути.
    ForbiddenCharacters,
    /// Символическая ссылка.
    Symlink,
    /// Выход за пределы директории.
    PathTraversal,
    /// Абсолютный путь.
    AbsolutePath,
    /// Неверный путь.
    InvalidPath,
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ошибка валидации пути: {} ({:?})",
            self.message, self.kind
        )
    }
}

impl std::error::Error for PathError {}

impl From<PathError> for io::Error {
    fn from(err: PathError) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, err.message)
    }
}

/// Валидатор путей для конфигурации.
///
/// Объединяет все проверки в одном месте (DRY).
///
/// # Архитектурные заметки
/// ## Problem 2.3 - Консолидация валидации путей
/// Этот валидатор заменяет отдельные функции:
/// - `validate_path_length`
/// - `validate_path_characters`
/// - `validate_no_symlinks`
/// - `validate_path_within_directory`
/// - `validate_config_path`
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::validation::PathValidator;
///
/// let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-");
/// let path = Path::new("config.json");
/// validator.validate(path).unwrap();
/// ```
pub struct PathValidator {
    /// Максимальная длина пути.
    max_length: usize,
    /// Разрешённые символы в пути.
    allowed_chars: &'static str,
}

impl PathValidator {
    /// Создать новый валидатор путей.
    ///
    /// # Аргументы
    /// * `max_length` - максимальная длина пути (рекомендуется 255)
    /// * `allowed_chars` - строка разрешённых символов
    ///
    /// # Возвращает
    /// Новый экземпляр `PathValidator`
    #[allow(dead_code)] // Может быть использовано для кастомных валидаторов
    pub const fn new(max_length: usize, allowed_chars: &'static str) -> Self {
        Self {
            max_length,
            allowed_chars,
        }
    }

    /// Валидировать путь.
    ///
    /// Выполняет все проверки:
    /// 1. Проверка длины
    /// 2. Проверка символов
    /// 3. Проверка на symlink
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    ///
    /// # Возвращает
    /// - `Ok(())` если путь валиден
    /// - `Err(PathError)` если путь невалиден
    ///
    /// # Errors
    /// Возвращает `PathError` если путь не проходит валидацию:
    /// - `PathErrorKind::TooLong` - путь слишком длинный
    /// - `PathErrorKind::ForbiddenCharacters` - запрещённые символы
    /// - `PathErrorKind::Symlink` - путь является символической ссылкой
    ///
    /// # Исправление #18
    /// Добавлен #[track_caller] для лучшей трассировки ошибок.
    #[track_caller]
    pub fn validate(&self, path: &Path) -> Result<(), PathError> {
        self.validate_length(path)?;
        self.validate_characters(path)?;
        self.validate_no_symlinks(path)?;
        Ok(())
    }

    /// Проверить максимальную длину пути.
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    ///
    /// # Возвращает
    /// - `Ok(())` если длина в пределах нормы
    /// - `Err(PathError)` если путь слишком длинный
    ///
    /// # Errors
    /// Возвращает `PathError` если длина пути превышает `max_length`.
    ///
    /// # Исправление #18
    /// Добавлен #[track_caller] для лучшей трассировки ошибок.
    #[track_caller]
    pub fn validate_length(&self, path: &Path) -> Result<(), PathError> {
        let path_str = path.to_str().ok_or_else(|| PathError {
            message: "Путь содержит невалидные UTF-8 символы".to_string(),
            kind: PathErrorKind::InvalidPath,
        })?;

        if path_str.len() > self.max_length {
            return Err(PathError {
                message: format!(
                    "Путь слишком длинный (максимум {} символов): {:?}",
                    self.max_length, path_str
                ),
                kind: PathErrorKind::TooLong,
            });
        }
        Ok(())
    }

    /// Проверить допустимые символы в пути.
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    ///
    /// # Возвращает
    /// - `Ok(())` если символы допустимы
    /// - `Err(PathError)` если есть запрещённые символы
    ///
    /// # Errors
    /// Возвращает `PathError` если путь содержит символы, не входящие в `allowed_chars`.
    ///
    /// # Исправление #18
    /// Добавлен #[track_caller] для лучшей трассировки ошибок.
    #[track_caller]
    pub fn validate_characters(&self, path: &Path) -> Result<(), PathError> {
        let path_str = path.to_str().ok_or_else(|| PathError {
            message: "Путь содержит невалидные UTF-8 символы".to_string(),
            kind: PathErrorKind::InvalidPath,
        })?;

        // Проверяем каждый символ
        for ch in path_str.chars() {
            if !self.allowed_chars.contains(ch) {
                return Err(PathError {
                    message: format!(
                        "Запрещённый символ в пути: {:?} (символ: '{}')",
                        path_str, ch
                    ),
                    kind: PathErrorKind::ForbiddenCharacters,
                });
            }
        }
        Ok(())
    }

    /// Проверить отсутствие символических ссылок.
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    ///
    /// # Возвращает
    /// - `Ok(())` если symlink не обнаружен
    /// - `Err(PathError)` если путь является symlink
    ///
    /// # Errors
    /// Возвращает `PathError` если путь является символической ссылкой.
    ///
    /// # Исправление #18
    /// Добавлен #[track_caller] для лучшей трассировки ошибок.
    #[allow(clippy::unused_self)]
    // Будет использоваться с конфигурируемыми параметрами
    #[track_caller]
    pub fn validate_no_symlinks(&self, path: &Path) -> Result<(), PathError> {
        if let Ok(metadata) = std::fs::symlink_metadata(path) {
            if metadata.file_type().is_symlink() {
                return Err(PathError {
                    message: format!("Символические ссылки не разрешены: {}", path.display()),
                    kind: PathErrorKind::Symlink,
                });
            }
        }
        Ok(())
    }

    /// Проверить, что путь находится внутри директории.
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    /// * `dir` - разрешённая директория
    ///
    /// # Возвращает
    /// - `Ok(())` если путь внутри директории
    /// - `Err(PathError)` если путь вне директории
    ///
    /// # Errors
    /// Возвращает `PathError` если путь находится вне разрешённой директории.
    #[allow(clippy::unused_self)] // Будет использоваться с конфигурируемыми параметрами
    pub fn validate_within_directory(&self, path: &Path, dir: &Path) -> Result<(), PathError> {
        let canonical_path = if path.exists() {
            path.canonicalize().map_err(|e| PathError {
                message: format!("Неверный путь {}: {}", path.display(), e),
                kind: PathErrorKind::InvalidPath,
            })?
        } else {
            path.parent()
                .and_then(|p| p.canonicalize().ok())
                .unwrap_or_else(|| dir.to_path_buf())
        };

        if canonical_path.strip_prefix(dir).is_err() {
            return Err(PathError {
                message: format!("Путь вне разрешённой директории: {}", path.display()),
                kind: PathErrorKind::PathTraversal,
            });
        }
        Ok(())
    }

    /// Проверить запрет абсолютных путей.
    ///
    /// # Аргументы
    /// * `path` - путь для проверки
    ///
    /// # Возвращает
    /// - `Ok(())` если путь относительный
    /// - `Err(PathError)` если путь абсолютный
    ///
    /// # Errors
    /// Возвращает `PathError` если путь является абсолютным.
    #[allow(dead_code)]
    // Может быть использовано для дополнительной валидации
    #[allow(clippy::unused_self)] // Будет использоваться с конфигурируемыми параметрами
    pub fn validate_not_absolute(&self, path: &Path) -> Result<(), PathError> {
        if path.is_absolute() {
            return Err(PathError {
                message: format!("Абсолютные пути не разрешены: {}", path.display()),
                kind: PathErrorKind::AbsolutePath,
            });
        }
        Ok(())
    }

    /// Проверить запрет path traversal (..).
    ///
    /// # Аргументы
    /// * `path` - путь для проверки (строка)
    ///
    /// # Возвращает
    /// - `Ok(())` если нет последовательностей ..
    /// - `Err(PathError)` если есть ..
    ///
    /// # Errors
    /// Возвращает `PathError` если путь содержит последовательности `..`.
    #[allow(dead_code)]
    // Может быть использовано для дополнительной валидации
    #[allow(clippy::unused_self)] // Будет использоваться с конфигурируемыми параметрами
    pub fn validate_no_traversal(&self, path: &str) -> Result<(), PathError> {
        if path.contains("..") {
            return Err(PathError {
                message: format!("Path traversal не разрешён: {:?}", path),
                kind: PathErrorKind::PathTraversal,
            });
        }
        Ok(())
    }
}

/// Валидатор путей по умолчанию для конфигурации.
///
/// Использует стандартные настройки:
/// - Максимальная длина: 255 символов
/// - Разрешённые символы: буквы, цифры, ., _, -, /
pub const DEFAULT_PATH_VALIDATOR: PathValidator = PathValidator {
    max_length: 255,
    allowed_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
};

#[cfg(test)]
mod validation_path_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_path_validator_new() {
        let validator = PathValidator::new(100, "abc");
        assert_eq!(validator.max_length, 100);
        assert_eq!(validator.allowed_chars, "abc");
    }

    #[test]
    fn test_path_validator_validate_length() {
        let validator = PathValidator::new(10, "abcdefghijklmnopqrstuvwxyz");
        let short_path = Path::new("short.txt");
        let long_path = Path::new("very_long_filename_that_exceeds_limit.txt");

        assert!(validator.validate_length(short_path).is_ok());
        assert!(validator.validate_length(long_path).is_err());
    }

    #[test]
    fn test_path_validator_validate_characters() {
        let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");
        let valid_path = Path::new("valid_file.txt");
        let invalid_path = Path::new("invalid@file.txt");

        assert!(validator.validate_characters(valid_path).is_ok());
        assert!(validator.validate_characters(invalid_path).is_err());
    }

    #[test]
    fn test_path_validator_validate_no_traversal() {
        let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

        assert!(validator.validate_no_traversal("valid/path.txt").is_ok());
        assert!(validator.validate_no_traversal("../parent.txt").is_err());
        assert!(validator
            .validate_no_traversal("path/../to/file.txt")
            .is_err());
    }

    #[test]
    fn test_path_validator_validate_not_absolute() {
        let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

        let relative = Path::new("relative/path.txt");
        let absolute = Path::new("/absolute/path.txt");

        assert!(validator.validate_not_absolute(relative).is_ok());
        assert!(validator.validate_not_absolute(absolute).is_err());
    }

    #[test]
    fn test_path_error_display() {
        let error = PathError {
            message: "Тестовая ошибка".to_string(),
            kind: PathErrorKind::TooLong,
        };
        let display = format!("{}", error);
        assert!(display.contains("Тестовая ошибка"));
        assert!(display.contains("TooLong"));
    }

    #[test]
    fn test_path_error_from_to_io_error() {
        let error = PathError {
            message: "Тест".to_string(),
            kind: PathErrorKind::InvalidPath,
        };
        let io_error: io::Error = error.into();
        assert_eq!(io_error.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_default_path_validator() {
        assert_eq!(DEFAULT_PATH_VALIDATOR.max_length, 255);
        assert!(DEFAULT_PATH_VALIDATOR.allowed_chars.contains('.'));
        assert!(DEFAULT_PATH_VALIDATOR.allowed_chars.contains('-'));
        assert!(DEFAULT_PATH_VALIDATOR.allowed_chars.contains('_'));
    }
}
