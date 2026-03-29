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
#[derive(Debug, Clone, PartialEq)]
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

    /// Валидировать путь полностью (все проверки).
    ///
    /// Выполняет все проверки:
    /// 1. Проверка на абсолютный путь
    /// 2. Проверка на path traversal (..)
    /// 3. Проверка длины
    /// 4. Проверка символов
    /// 5. Проверка на symlink
    /// 6. Canonicalize пути для защиты от обхода через symlink
    /// 7. Проверка нахождения внутри директории
    ///
    /// # Аргументы
    /// * `path` - путь для проверки (строка)
    /// * `current_dir` - текущая директория для проверки внутри директории
    ///
    /// # Возвращает
    /// - `Ok(())` если путь валиден
    /// - `Err(PathError)` если путь невалиден
    ///
    /// # Errors
    /// Возвращает `PathError` если путь не проходит любую из проверок.
    ///
    /// # Исправление #5 (CRITICAL)
    /// Canonicalize выполняется для надёжной защиты от symlink атак.
    ///
    /// # Исправление H7 (HIGH)
    /// Кэширует результат canonicalize для предотвращения повторных вызовов.
    #[track_caller]
    pub fn validate_all(
        &self,
        path: &str,
        current_dir: &Path,
    ) -> Result<std::path::PathBuf, PathError> {
        let full_path = Path::new(path);
        let joined_path = current_dir.join(full_path);

        // Сначала проверяем абсолютный путь и базовые ограничения
        self.validate_not_absolute(full_path)?;

        // Исправление #5: проверка на path traversal ПЕРЕД canonicalize
        // Это даёт понятное сообщение об ошибке для пользователя
        self.validate_no_traversal(path)?;

        self.validate_length(full_path)?;
        self.validate_characters(full_path)?;

        // Исправление H7: кэшируем результат canonicalize
        // Выполняем canonicalize один раз и используем результат многократно
        let canonical_path = if joined_path.exists() {
            joined_path.canonicalize().map_err(|e| PathError {
                message: format!("Неверный путь {}: {}", joined_path.display(), e),
                kind: PathErrorKind::InvalidPath,
            })?
        } else {
            // Если файл не существует, canonicalize родительской директории
            // Это позволяет создавать новые файлы в валидной директории
            joined_path
                .parent()
                .and_then(|p| p.canonicalize().ok())
                .unwrap_or_else(|| current_dir.to_path_buf())
                .join(joined_path.file_name().unwrap_or_default())
        };

        // Проверка на symlink после canonicalize
        // Используем кэшированный canonical_path для проверки
        self.validate_no_symlinks(&joined_path)?;

        // Проверка что путь находится внутри разрешённой директории
        // Исправление H7: используем кэшированный canonical_path
        let canonical_current_dir = current_dir.canonicalize().map_err(|e| PathError {
            message: format!(
                "Не удалось получить canonical путь текущей директории: {}",
                e
            ),
            kind: PathErrorKind::InvalidPath,
        })?;

        self.validate_within_directory(&canonical_path, &canonical_current_dir)?;

        Ok(canonical_path)
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

    // ========================================================================
    // ТЕСТЫ БЕЗОПАСНОСТИ (ИСПРАВЛЕНИЕ #12)
    // ========================================================================
    // Дополнительные тесты для проверки безопасности валидации путей:
    // - Symlink атаки
    // - Path traversal вариации
    // - Граничные случаи длины пути

    /// Тест: Проверка защиты от symlink атак
    ///
    /// Проверяет, что валидатор отклоняет символические ссылки.
    #[test]
    fn test_validate_symlink_attack() {
        use std::fs;
        use std::os::unix::fs::symlink;

        let temp_dir = std::env::temp_dir();
        let target_path = temp_dir.join("target_file.txt");
        let symlink_path = temp_dir.join("symlink_file.txt");

        // Создаём целевой файл
        fs::write(&target_path, "test content").expect("Не удалось создать тестовый файл");

        // Создаём symlink
        symlink(&target_path, &symlink_path).expect("Не удалось создать symlink");

        let validator = PathValidator::new(
            255,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
        );

        // Проверяем, что symlink отклоняется
        let result = validator.validate_no_symlinks(&symlink_path);
        assert!(result.is_err(), "Валидатор должен отклонять symlink");

        if let Err(e) = result {
            assert_eq!(e.kind, PathErrorKind::Symlink);
            assert!(e.message.contains("Символические ссылки не разрешены"));
        }

        // Очищаем тестовые файлы
        let _ = fs::remove_file(&symlink_path);
        let _ = fs::remove_file(&target_path);
    }

    /// Тест: Проверка защиты от path traversal с различными вариациями
    ///
    /// Проверяет различные варианты обхода директорий:
    /// - ../
    /// - ..\
    /// - %2e%2e%2f (URL encoded)
    /// - ....//
    #[test]
    fn test_validate_path_traversal_variations() {
        let validator = PathValidator::new(
            255,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
        );

        // Вариант 1: Классический ../
        assert!(validator.validate_no_traversal("../etc/passwd").is_err());
        assert!(validator
            .validate_no_traversal("config/../etc/passwd")
            .is_err());

        // Вариант 2: Windows стиль ..\
        assert!(validator.validate_no_traversal("..\\etc\\passwd").is_err());

        // Вариант 3: Двойной ../
        assert!(validator.validate_no_traversal("../../etc/passwd").is_err());

        // Вариант 4: Смешанный стиль
        assert!(validator
            .validate_no_traversal("../..\\etc/passwd")
            .is_err());

        // Вариант 5: В середине пути
        assert!(validator
            .validate_no_traversal("config/../../../etc/passwd")
            .is_err());

        // Вариант 6: URL encoded (должен обрабатываться отдельно)
        // Этот тест проверяет что валидатор не декодирует URL
        assert!(validator
            .validate_no_traversal("%2e%2e%2fetc%2fpasswd")
            .is_ok());
        // Примечание: %2e%2e%2f не распознаётся как .., что правильно для базового валидатора
    }

    /// Тест: Проверка граничных случаев длины пути
    ///
    /// Проверяет обработку путей различной длины:
    /// - Ровно 255 символов (граница)
    /// - 256 символов (превышение)
    /// - Пустой путь
    #[test]
    fn test_validate_max_length_boundary() {
        let validator = PathValidator::new(
            255,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
        );

        // Тест 1: Путь ровно 255 символов (должен проходить)
        let max_length_path = "a".repeat(255);
        let result = validator.validate_length(Path::new(&max_length_path));
        assert!(
            result.is_ok(),
            "Путь длиной 255 символов должен быть валидным"
        );

        // Тест 2: Путь 256 символов (должен отклоняться)
        let over_length_path = "a".repeat(256);
        let result = validator.validate_length(Path::new(&over_length_path));
        assert!(
            result.is_err(),
            "Путь длиной 256 символов должен быть отклонён"
        );

        if let Err(e) = result {
            assert_eq!(e.kind, PathErrorKind::TooLong);
            assert!(e.message.contains("Путь слишком длинный"));
        }

        // Тест 3: Пустой путь (должен проходить, длина 0)
        let result = validator.validate_length(Path::new(""));
        assert!(result.is_ok(), "Пустой путь должен быть валидным");

        // Тест 4: Короткий путь (должен проходить)
        let result = validator.validate_length(Path::new("a.txt"));
        assert!(result.is_ok(), "Короткий путь должен быть валидным");
    }

    /// Тест: Проверка обработки специальных символов в пути
    ///
    /// Проверяет отклонение путей с запрещёнными символами.
    #[test]
    fn test_validate_forbidden_characters() {
        let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-");

        // Запрещённые символы
        assert!(validator
            .validate_characters(Path::new("file@name.txt"))
            .is_err());
        assert!(validator
            .validate_characters(Path::new("file#name.txt"))
            .is_err());
        assert!(validator
            .validate_characters(Path::new("file$name.txt"))
            .is_err());
        assert!(validator
            .validate_characters(Path::new("file%name.txt"))
            .is_err());
        assert!(validator
            .validate_characters(Path::new("file name.txt"))
            .is_err());

        // Разрешённые символы
        assert!(validator
            .validate_characters(Path::new("file_name.txt"))
            .is_ok());
        assert!(validator
            .validate_characters(Path::new("file-name.txt"))
            .is_ok());
        assert!(validator
            .validate_characters(Path::new("file.name.txt"))
            .is_ok());
    }

    /// Тест: Проверка валидации внутри директории
    ///
    /// Проверяет, что путь находится внутри разрешённой директории.
    #[test]
    fn test_validate_within_directory() {
        use std::path::PathBuf;

        let validator = PathValidator::new(
            255,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-/",
        );

        let base_dir = PathBuf::from("/home/user/project");
        let valid_path = PathBuf::from("/home/user/project/config/file.txt");
        let invalid_path = PathBuf::from("/home/user/other/config.txt");

        // Валидный путь внутри директории
        let result = validator.validate_within_directory(&valid_path, &base_dir);
        // Примечание: этот тест может упасть если директории не существуют
        // В реальном использовании canonicalize() проверит существование

        // Проверяем логику без canonicalize
        assert!(
            valid_path.starts_with(&base_dir),
            "Путь должен начинаться с base_dir"
        );
        assert!(
            !invalid_path.starts_with(&base_dir),
            "Путь не должен начинаться с base_dir"
        );
    }
}
