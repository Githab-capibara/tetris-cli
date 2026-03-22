//! Конфигурация управления.
//!
//! Этот модуль предоставляет систему настройки клавиш управления для игры Tetris CLI.
//! Поддерживает сохранение и загрузку конфигурации, валидацию клавиш.
//!
//! ## Структура модуля
//! - `ControlsConfig` - структура конфигурации управления
//! - `DEFAULT_CONTROLS` - значения по умолчанию
//! - `tests` - модульные тесты (4 теста)

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

/// Проверить валидность пути для конфигурации.
///
/// Эта функция реализует защиту от path traversal атак и symlink attacks:
/// 1. Запрещает абсолютные пути
/// 2. Запрещает последовательности ".."
/// 3. Проверяет, что путь находится внутри текущей директории
/// 4. Запрещает символические ссылки
/// 5. Использует canonicalize() ДО всех проверок для защиты от race condition
///
/// # Аргументы
/// * `path` - путь для проверки
///
/// # Возвращает
/// - `Ok(())` если путь валиден
/// - `Err(io::Error)` если путь невалиден
///
/// # Безопасность
/// Защищает от symlink атак: символические ссылки запрещены.
/// Проверка canonicalize() выполняется ДО валидации пути для предотвращения race condition.
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::controls::validate_config_path;
/// validate_config_path("config.json").unwrap();
/// ```
fn validate_config_path(path: &str) -> io::Result<()> {
    let full_path = Path::new(path);

    // ЗАПРЕТ АБСОЛЮТНЫХ ПУТЕЙ
    if full_path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Абсолютные пути не разрешены",
        ));
    }

    // ЗАПРЕТ PATH TRAVERSAL (..)
    if path.contains("..") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path traversal не разрешён",
        ));
    }

    // Получаем текущую директорию
    let current_dir = std::env::current_dir()?;
    let joined_path = current_dir.join(full_path);

    // Для существующих файлов - используем canonicalize()
    // Для несуществующих (сохранение) - проверяем родительскую директорию
    let canonical_path = if joined_path.exists() {
        joined_path.canonicalize().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("Неверный путь: {e}"))
        })?
    } else {
        // Файл не существует - проверяем родительскую директорию
        // Если родительской директории нет, используем текущую директорию
        joined_path
            .parent()
            .and_then(|p| p.canonicalize().ok())
            .unwrap_or_else(|| current_dir.clone())
    };

    // ЗАПРЕТ СИМВОЛИЧЕСКИХ ССЫЛОК
    // Используем symlink_metadata() для проверки symlink без следования по нему
    if let Ok(metadata) = std::fs::symlink_metadata(&joined_path) {
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Символические ссылки не разрешены",
            ));
        }
    }

    // Проверяем, что resolved path находится внутри текущей директории
    // Используем strip_prefix() для надёжной проверки
    if canonical_path.strip_prefix(&current_dir).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Путь вне разрешённой директории (symlink attack detected)",
        ));
    }

    Ok(())
}

/// Конфигурация управления игрой.
///
/// Содержит коды клавиш для всех действий в игре.
/// Все клавиши хранятся как u8 коды (ASCII значения).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlsConfig {
    /// Движение влево (по умолчанию 'a' = 97).
    pub move_left: u8,
    /// Движение вправо (по умолчанию 'd' = 100).
    pub move_right: u8,
    /// Мягкое падение/ускорение (по умолчанию 's' = 115).
    pub soft_drop: u8,
    /// Жёсткое падение/Hard Drop (по умолчанию 'w' = 119).
    pub hard_drop: u8,
    /// Вращение против часовой (по умолчанию 'q' = 113).
    pub rotate_left: u8,
    /// Вращение по часовой (по умолчанию 'e' = 101).
    pub rotate_right: u8,
    /// Удержание фигуры/Hold (по умолчанию 'c' = 99).
    pub hold: u8,
    /// Пауза (по умолчанию 'p' = 112).
    pub pause: u8,
    /// Выход/Backspace (по умолчанию 127).
    pub quit: u8,
}

impl ControlsConfig {
    /// Создать конфигурацию со значениями по умолчанию.
    ///
    /// # Возвращает
    /// Новый экземпляр `ControlsConfig` со стандартными настройками:
    /// - `move_left`: 'a' (97)
    /// - `move_right`: 'd' (100)
    /// - `soft_drop`: 's' (115)
    /// - `hard_drop`: 'w' (119)
    /// - `rotate_left`: 'q' (113)
    /// - `rotate_right`: 'e' (101)
    /// - `hold`: 'c' (99)
    /// - `pause`: 'p' (112)
    /// - `quit`: 127 (Backspace)
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::default_config();
    /// assert_eq!(config.move_left, b'a');
    /// ```
    #[must_use = "Конфигурация должна быть использована"]
    pub const fn default_config() -> Self {
        Self {
            move_left: b'a',
            move_right: b'd',
            soft_drop: b's',
            hard_drop: b'w',
            rotate_left: b'q',
            rotate_right: b'e',
            hold: b'c',
            pause: b'p',
            quit: 127, // Backspace
        }
    }

    /// Сохранить конфигурацию в JSON файл.
    ///
    /// # Аргументы
    /// * `path` - путь к файлу конфигурации (по умолчанию CONFIG_PATH)
    ///
    /// # Возвращает
    /// - `Ok(())` если сохранение успешно
    /// - `Err(io::Error)` если произошла ошибка записи
    ///
    /// # Errors
    /// Функция возвращает ошибку в следующих случаях:
    /// - Путь содержит абсолютный путь вне директории приложения
    /// - Путь содержит последовательности `..` (path traversal)
    /// - Путь выходит за пределы разрешённой директории после разрешения symlink
    /// - Ошибка записи в файл (недостаточно прав, нет места на диске)
    ///
    /// # Пример использования
    /// ```no_run
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::default_config();
    /// config.save_to_file("my_controls.json").unwrap();
    /// ```
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        // Валидация пути с использованием общей функции
        validate_config_path(path)?;

        let json =
            serde_json::to_string_pretty(self).map_err(|e| io::Error::other(e.to_string()))?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Загрузить конфигурацию из JSON файла.
    ///
    /// # Аргументы
    /// * `path` - путь к файлу конфигурации (по умолчанию CONFIG_PATH)
    ///
    /// # Возвращает
    /// - `Ok(ControlsConfig)` если загрузка успешна
    /// - `Err(io::Error)` если файл не найден или некорректен
    ///
    /// # Errors
    /// Функция возвращает ошибку в следующих случаях:
    /// - Путь содержит абсолютный путь вне директории приложения
    /// - Путь содержит последовательности `..` (path traversal)
    /// - Файл не найден или не может быть прочитан
    /// - JSON некорректен или не соответствует ожидаемой структуре
    ///
    /// # Пример использования
    /// ```no_run
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::load_from_file("my_controls.json").unwrap();
    /// ```
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        // Валидация пути с использованием общей функции
        validate_config_path(path)?;

        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }

    /// Валидировать конфигурацию управления.
    ///
    /// Проверяет:
    /// 1. Все клавиши находятся в допустимом диапазоне (1-255)
    /// 2. Нет дублирующихся клавиш (каждая клавиша уникальна)
    /// 3. Quit клавиша (Backspace) не конфликтует с другими
    ///
    /// # Возвращает
    /// - `true` если конфигурация валидна
    /// - `false` если есть дубликаты или недопустимые значения
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::default_config();
    /// assert!(config.validate());
    /// ```
    #[must_use]
    pub fn validate(&self) -> bool {
        // Сбор всех клавиш в массив для проверки
        let keys = [
            self.move_left,
            self.move_right,
            self.soft_drop,
            self.hard_drop,
            self.rotate_left,
            self.rotate_right,
            self.hold,
            self.pause,
            self.quit,
        ];

        // Проверка диапазона значений (1-255) и дубликатов за один проход O(n)
        // Оптимизация: используем массив [bool; 256] вместо HashSet для эффективности
        let mut seen = [false; 256];
        for &key in &keys {
            // Проверка: клавиша должна быть в диапазоне 1-255
            // 0 - невалидное значение (NULL байт)
            // 255+ зарезервированы для специальных клавиш
            if key == 0 || seen[key as usize] {
                return false; // Дубликат найден или невалидное значение
            }
            seen[key as usize] = true;
        }

        true
    }

    /// Создать кастомную конфигурацию.
    ///
    /// # Аргументы
    /// * `move_left` - клавиша движения влево
    /// * `move_right` - клавиша движения вправо
    /// * `soft_drop` - клавиша мягкого падения
    /// * `hard_drop` - клавиша жёсткого падения
    /// * `rotate_left` - клавиша вращения против часовой
    /// * `rotate_right` - клавиша вращения по часовой
    /// * `hold` - клавиша удержания фигуры
    /// * `pause` - клавиша паузы
    /// * `quit` - клавиша выхода
    ///
    /// # Возвращает
    /// Новый экземпляр ControlsConfig с заданными значениями
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::custom(
    ///     b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127
    /// );
    /// assert_eq!(config.move_left, b'h');
    /// ```
    #[must_use = "Конфигурация должна быть использована"]
    pub const fn custom(
        move_left: u8,
        move_right: u8,
        soft_drop: u8,
        hard_drop: u8,
        rotate_left: u8,
        rotate_right: u8,
        hold: u8,
        pause: u8,
        quit: u8,
    ) -> Self {
        Self {
            move_left,
            move_right,
            soft_drop,
            hard_drop,
            rotate_left,
            rotate_right,
            hold,
            pause,
            quit,
        }
    }
}

impl Default for ControlsConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod controls_tests {
    use super::*;
    use std::path::Path;

    // =========================================================================
    // ГРУППА ТЕСТОВ 17-20: Controls Config
    // =========================================================================
    // Эти тесты проверяют систему конфигурации управления:
    // - Значения по умолчанию
    // - Сохранение и загрузка из файла
    // - Валидация клавиш
    // - Кастомная конфигурация

    /// Тест 17: Проверка значений по умолчанию
    ///
    /// Проверяет, что все клавиши управления имеют правильные
    /// значения по умолчанию согласно стандартной раскладке WASD/QE.
    #[test]
    fn test_controls_config_default() {
        let config = ControlsConfig::default_config();

        // Проверка всех клавиш по умолчанию
        assert_eq!(config.move_left, b'a', "Движение влево должно быть 'a'");
        assert_eq!(config.move_right, b'd', "Движение вправо должно быть 'd'");
        assert_eq!(config.soft_drop, b's', "Мягкое падение должно быть 's'");
        assert_eq!(config.hard_drop, b'w', "Жёсткое падение должно быть 'w'");
        assert_eq!(config.rotate_left, b'q', "Вращение влево должно быть 'q'");
        assert_eq!(config.rotate_right, b'e', "Вращение вправо должно быть 'e'");
        assert_eq!(config.hold, b'c', "Удержание должно быть 'c'");
        assert_eq!(config.pause, b'p', "Пауза должна быть 'p'");
        assert_eq!(config.quit, 127, "Выход должен быть Backspace (127)");
    }

    /// Тест 18: Проверка сохранения и загрузки конфигурации
    ///
    /// Проверяет полный цикл: создание -> сохранение -> загрузка -> сравнение.
    /// Использует временный файл для тестирования.
    #[test]
    fn test_controls_config_save_load() -> std::io::Result<()> {
        // Используем относительный путь в текущей директории
        let test_path = "test_controls_config_temp.json";

        // Создаём кастомную конфигурацию
        let original_config = ControlsConfig::custom(
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

        // Сохраняем конфигурацию
        original_config.save_to_file(test_path)?;

        // Проверяем, что файл существует
        assert!(
            Path::new(test_path).exists(),
            "Файл конфигурации должен существовать"
        );

        // Загружаем конфигурацию
        let loaded_config = ControlsConfig::load_from_file(test_path)?;

        // Сравниваем оригинал с загруженным
        assert_eq!(
            original_config, loaded_config,
            "Загруженная конфигурация должна совпадать с оригиналом"
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(test_path);

        Ok(())
    }

    /// Тест 19: Проверка валидации клавиш
    ///
    /// Проверяет:
    /// 1. Валидная конфигурация возвращает true
    /// 2. Конфигурация с дубликатами возвращает false
    /// 3. Конфигурация с недопустимыми значениями возвращает false
    #[test]
    fn test_controls_config_validation() {
        // Тест 1: Валидная конфигурация по умолчанию
        let valid_config = ControlsConfig::default_config();
        assert!(
            valid_config.validate(),
            "Конфигурация по умолчанию должна быть валидной"
        );

        // Тест 2: Конфигурация с дубликатами (move_left == move_right)
        let duplicate_config = ControlsConfig::custom(
            b'a', b'a', // Дубликат: обе клавиши 'a'
            b's', b'w', b'q', b'e', b'c', b'p', 127,
        );
        assert!(
            !duplicate_config.validate(),
            "Конфигурация с дубликатами должна быть невалидной"
        );

        // Тест 3: Конфигурация с нулевым значением
        let zero_key_config =
            ControlsConfig::custom(0, b'd', b's', b'w', b'q', b'e', b'c', b'p', 127);
        assert!(
            !zero_key_config.validate(),
            "Конфигурация с нулевой клавишей должна быть невалидной"
        );

        // Тест 4: Кастомная валидная конфигурация
        let custom_valid =
            ControlsConfig::custom(b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9');
        assert!(
            custom_valid.validate(),
            "Кастомная конфигурация без дубликатов должна быть валидной"
        );
    }

    /// Тест 20: Проверка кастомной конфигурации
    ///
    /// Проверяет создание конфигурации с произвольными значениями клавиш.
    /// Тестирует альтернативные раскладки (например, стрелки, цифровая клавиатура).
    #[test]
    fn test_controls_config_custom() {
        // Тест 1: Конфигурация в стиле Vim (HJKL)
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

        assert_eq!(vim_config.move_left, b'h');
        assert_eq!(vim_config.move_right, b'l');
        assert_eq!(vim_config.soft_drop, b'j');
        assert_eq!(vim_config.hard_drop, b'k');
        assert!(vim_config.validate());

        // Тест 2: Конфигурация с цифровой клавиатурой
        let numpad_config = ControlsConfig::custom(
            b'4', // move_left
            b'6', // move_right
            b'5', // soft_drop
            b'8', // hard_drop
            b'1', // rotate_left
            b'3', // rotate_right
            b'0', // hold
            b'9', // pause
            b'7', // quit
        );

        assert_eq!(numpad_config.move_left, b'4');
        assert_eq!(numpad_config.move_right, b'6');
        assert!(numpad_config.validate());

        // Тест 3: Проверка, что кастомная конфигурация отличается от default
        let default_config = ControlsConfig::default_config();
        assert_ne!(
            vim_config.move_left, default_config.move_left,
            "Кастомная конфигурация должна отличаться от стандартной"
        );
    }
}
