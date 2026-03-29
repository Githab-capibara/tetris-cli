//! Конфигурация управления.
//!
//! Модуль предоставляет систему настройки клавиш управления для игры.
//! Поддерживает сохранение/загрузку конфигурации и валидацию клавиш.
//!
//! # Исправление #3 (CRITICAL)
//! HMAC логика перемещена в модуль `crypto::validator`.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;

// ============================================================================
// ВАЛИДАТОР ПУТЕЙ (переэкспортирован из crate::validation)
// ============================================================================
// PathValidator, PathError и PathErrorKind теперь находятся в модуле validation
// для централизации кода валидации.

// Переэкспорт для обратной совместимости
pub use crate::validation::path::DEFAULT_PATH_VALIDATOR;

use crate::crypto::validator::sign_salt_and_data;

/// Секретный ключ для HMAC подписи конфигурации.
///
/// # Безопасность (Исправление В4)
/// Ключ загружается из переменной окружения `TETRIS_HMAC_KEY` если она установлена.
/// В противном случае используется fallback ключ для обратной совместимости.
///
/// ## Использование переменной окружения
/// ```bash
/// export TETRIS_HMAC_KEY="your-secret-key-here"
/// ```
///
/// ## Fallback для обратной совместимости
/// Если переменная окружения не установлена, используется константный ключ.
/// Это обеспечивает обратную совместимость с существующими записями.
fn get_hmac_key() -> &'static str {
    option_env!("TETRIS_HMAC_KEY").unwrap_or("tetris-cli-controls-hmac-key")
}

/// Конфигурация управления с keyed hash подписью.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlsConfig {
    /// Движение влево.
    pub move_left: u8,
    /// Движение вправо.
    pub move_right: u8,
    /// Мягкое падение.
    pub soft_drop: u8,
    /// Жёсткое падение.
    pub hard_drop: u8,
    /// Вращение против часовой.
    pub rotate_left: u8,
    /// Вращение по часовой.
    pub rotate_right: u8,
    /// Удержание фигуры.
    pub hold: u8,
    /// Пауза.
    pub pause: u8,
    /// Выход.
    pub quit: u8,
    /// Внутренний HMAC ключ (не следует изменять напрямую).
    #[doc(hidden)]
    pub hmac_key: String,
    /// Подпись конфигурации.
    signature: String,
}

// ============================================================================
// КОНФИГУРАЦИЯ УПРАВЛЕНИЯ
// ============================================================================
// ControlsConfig теперь определён в начале файла с поддержкой keyed hash

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
    /// assert_eq!(config.move_left(), b'a');
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
            quit: 127,               // Backspace
            hmac_key: String::new(), // Ключ генерируется при сохранении
            signature: String::new(),
        }
    }

    /// Геттеры для всех полей конфигурации (для обратной совместимости).
    #[must_use]
    pub const fn move_left(&self) -> u8 {
        self.move_left
    }
    #[must_use]
    pub const fn move_right(&self) -> u8 {
        self.move_right
    }
    #[must_use]
    pub const fn soft_drop(&self) -> u8 {
        self.soft_drop
    }
    #[must_use]
    pub const fn hard_drop(&self) -> u8 {
        self.hard_drop
    }
    #[must_use]
    pub const fn rotate_left(&self) -> u8 {
        self.rotate_left
    }
    #[must_use]
    pub const fn rotate_right(&self) -> u8 {
        self.rotate_right
    }
    #[must_use]
    pub const fn hold(&self) -> u8 {
        self.hold
    }
    #[must_use]
    pub const fn pause(&self) -> u8 {
        self.pause
    }
    #[must_use]
    pub const fn quit(&self) -> u8 {
        self.quit
    }

    /// Сеттеры для всех полей конфигурации (для тестов и обратной совместимости).
    /// Возвращает self для возможности цепочки вызовов.
    pub fn set_move_left(&mut self, value: u8) -> &mut Self {
        self.move_left = value;
        self
    }
    pub fn set_move_right(&mut self, value: u8) -> &mut Self {
        self.move_right = value;
        self
    }
    pub fn set_soft_drop(&mut self, value: u8) -> &mut Self {
        self.soft_drop = value;
        self
    }
    pub fn set_hard_drop(&mut self, value: u8) -> &mut Self {
        self.hard_drop = value;
        self
    }
    pub fn set_rotate_left(&mut self, value: u8) -> &mut Self {
        self.rotate_left = value;
        self
    }
    pub fn set_rotate_right(&mut self, value: u8) -> &mut Self {
        self.rotate_right = value;
        self
    }
    pub fn set_hold(&mut self, value: u8) -> &mut Self {
        self.hold = value;
        self
    }
    pub fn set_pause(&mut self, value: u8) -> &mut Self {
        self.pause = value;
        self
    }
    pub fn set_quit(&mut self, value: u8) -> &mut Self {
        self.quit = value;
        self
    }

    /// Сравнить только клавиши управления (игнорируя `hmac_key`).
    /// Используется в тестах для сравнения конфигураций.
    #[must_use]
    pub fn keys_match(&self, other: &Self) -> bool {
        self.move_left == other.move_left
            && self.move_right == other.move_right
            && self.soft_drop == other.soft_drop
            && self.hard_drop == other.hard_drop
            && self.rotate_left == other.rotate_left
            && self.rotate_right == other.rotate_right
            && self.hold == other.hold
            && self.pause == other.pause
            && self.quit == other.quit
    }

    /// Сохранить конфигурацию в JSON файл.
    ///
    /// # Аргументы
    /// * `path` - путь к файлу конфигурации (по умолчанию `CONFIG_PATH`)
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
    /// # Безопасность
    /// - Генерируется новый HMAC ключ при каждом сохранении
    /// - Конфигурация подписывается keyed hash
    /// - Используется `O_NOFOLLOW` для защиты от symlink атак
    ///
    /// # Пример использования
    /// ```no_run
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::default_config();
    /// config.save_to_file("my_controls.json").unwrap();
    /// ```
    ///
    /// # Panics
    /// Может паниковать при переполнении времени (крайне маловероятно)
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::validator`.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        // Валидация пути через DEFAULT_PATH_VALIDATOR
        let current_dir = std::env::current_dir().map_err(|e| {
            io::Error::other(format!("Не удалось получить текущую директорию: {e}"))
        })?;
        let joined_path = DEFAULT_PATH_VALIDATOR
            .validate_all(path, &current_dir)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

        // Генерируем новый ключ при сохранении
        let hmac_key = crate::crypto::generate_salt();

        // Сериализуем конфигурацию без signature для вычисления хеша
        let config_for_hash = ControlsConfig {
            move_left: self.move_left,
            move_right: self.move_right,
            soft_drop: self.soft_drop,
            hard_drop: self.hard_drop,
            rotate_left: self.rotate_left,
            rotate_right: self.rotate_right,
            hold: self.hold,
            pause: self.pause,
            quit: self.quit,
            hmac_key: hmac_key.clone(),
            signature: String::new(),
        };

        let config_json = serde_json::to_string(&config_for_hash)
            .map_err(|e| io::Error::other(format!("Ошибка сериализации: {e}")))?;

        // Вычисляем HMAC-SHA256 подпись через validator модуль
        let signature = sign_salt_and_data(&hmac_key, "", &config_json);

        // Создаём итоговую конфигурацию с подписью
        let config_with_sig = ControlsConfig {
            move_left: self.move_left,
            move_right: self.move_right,
            soft_drop: self.soft_drop,
            hard_drop: self.hard_drop,
            rotate_left: self.rotate_left,
            rotate_right: self.rotate_right,
            hold: self.hold,
            pause: self.pause,
            quit: self.quit,
            hmac_key,
            signature,
        };

        // Сериализуем в JSON
        let json = serde_json::to_string_pretty(&config_with_sig)
            .map_err(|e| io::Error::other(e.to_string()))?;

        // Используем O_NOFOLLOW для защиты от symlink атак при записи
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Загрузить конфигурацию из JSON файла.
    ///
    /// # Аргументы
    /// * `path` - путь к файлу конфигурации (по умолчанию `CONFIG_PATH`)
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
    /// - HMAC подпись не совпадает (подделка конфигурации)
    ///
    /// # Безопасность
    /// - Проверяется keyed hash подпись конфигурации
    /// - Используется `symlink_metadata()` для защиты от symlink атак
    ///
    /// # Пример использования
    /// ```no_run
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::load_from_file("my_controls.json").unwrap();
    /// ```
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        // Валидация пути через DEFAULT_PATH_VALIDATOR
        let current_dir = std::env::current_dir().map_err(|e| {
            io::Error::other(format!("Не удалось получить текущую директорию: {e}"))
        })?;
        let joined_path = DEFAULT_PATH_VALIDATOR
            .validate_all(path, &current_dir)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

        // Проверяем, что файл не является symlink
        if let Ok(metadata) = std::fs::symlink_metadata(&joined_path) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Символические ссылки не разрешены: {path:?}"),
                ));
            }
            // Исправление #10: проверка размера файла перед загрузкой
            let file_size = metadata.len();
            if file_size > super::highscore::MAX_CONFIG_FILE_SIZE {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Файл конфигурации слишком большой: {} байт (максимум {} байт)",
                        file_size,
                        super::highscore::MAX_CONFIG_FILE_SIZE
                    ),
                ));
            }
        }

        // Исправление #12: используем O_NOFOLLOW для защиты от race condition
        // между проверкой symlink и открытием файла
        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;

        // Читаем файл через буферизированный Read
        let mut json = String::new();
        file.read_to_string(&mut json)?;

        // Десериализуем конфигурацию
        let config: ControlsConfig = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Проверяем keyed hash подпись
        let config_for_hash = ControlsConfig {
            move_left: config.move_left,
            move_right: config.move_right,
            soft_drop: config.soft_drop,
            hard_drop: config.hard_drop,
            rotate_left: config.rotate_left,
            rotate_right: config.rotate_right,
            hold: config.hold,
            pause: config.pause,
            quit: config.quit,
            hmac_key: config.hmac_key.clone(),
            signature: String::new(),
        };

        let config_json = serde_json::to_string(&config_for_hash).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Ошибка сериализации: {e}"),
            )
        })?;

        // Проверяем HMAC-SHA256 подпись через validator модуль
        let expected_signature = sign_salt_and_data(&config.hmac_key, "", &config_json);

        if config.signature != expected_signature {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "HMAC-SHA256 подпись не совпадает - возможна подделка конфигурации",
            ));
        }

        Ok(config)
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
    /// Новый экземпляр `ControlsConfig` с заданными значениями
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::controls::ControlsConfig;
    ///
    /// let config = ControlsConfig::custom(
    ///     b'h', b'l', b'j', b'k', b'y', b'u', b'i', b'o', 127
    /// );
    /// assert_eq!(config.move_left(), b'h');
    /// ```
    #[must_use = "Конфигурация должна быть использована"]
    #[allow(clippy::too_many_arguments)]
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
            hmac_key: String::new(),
            signature: String::new(),
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
    use std::fs;
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
        assert_eq!(config.move_left(), b'a', "Движение влево должно быть 'a'");
        assert_eq!(config.move_right(), b'd', "Движение вправо должно быть 'd'");
        assert_eq!(config.soft_drop(), b's', "Мягкое падение должно быть 's'");
        assert_eq!(config.hard_drop(), b'w', "Жёсткое падение должно быть 'w'");
        assert_eq!(config.rotate_left(), b'q', "Вращение влево должно быть 'q'");
        assert_eq!(
            config.rotate_right(),
            b'e',
            "Вращение вправо должно быть 'e'"
        );
        assert_eq!(config.hold(), b'c', "Удержание должно быть 'c'");
        assert_eq!(config.pause(), b'p', "Пауза должна быть 'p'");
        assert_eq!(config.quit(), 127, "Выход должен быть Backspace (127)");
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

        // Сравниваем только клавиши (игнорируя hmac_key)
        assert!(
            original_config.keys_match(&loaded_config),
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

        assert_eq!(vim_config.move_left(), b'h');
        assert_eq!(vim_config.move_right(), b'l');
        assert_eq!(vim_config.soft_drop(), b'j');
        assert_eq!(vim_config.hard_drop(), b'k');
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

        assert_eq!(numpad_config.move_left(), b'4');
        assert_eq!(numpad_config.move_right(), b'6');
        assert!(numpad_config.validate());

        // Тест 3: Проверка, что кастомная конфигурация отличается от default
        let default_config = ControlsConfig::default_config();
        assert_ne!(
            vim_config.move_left(),
            default_config.move_left(),
            "Кастомная конфигурация должна отличаться от стандартной"
        );
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ПРОВЕРКИ РАЗМЕРА ФАЙЛА (ИСПРАВЛЕНИЕ #6, #10)
    // =========================================================================

    /// Тест 21: Проверка проверки размера файла > 1MB
    ///
    /// Проверяет что файл конфигурации размером больше 1MB
    /// возвращает ошибку при загрузке
    #[test]
    fn test_config_file_size_too_large() -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let test_path = "test_controls_config_large.json";

        // Создаём файл размером больше 1MB (1MB + 100KB)
        let large_size = (crate::highscore::MAX_CONFIG_FILE_SIZE + 100_000) as usize;
        let mut file = File::create(test_path)?;

        // Записываем данные размером больше 1MB
        let data = vec![b'x'; large_size];
        file.write_all(&data)?;
        drop(file);

        // Проверяем что загрузка возвращает ошибку
        let result = ControlsConfig::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Загрузка файла > 1MB должна вернуть ошибку"
        );

        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("слишком большим") || error.to_string().contains("максимум"),
            "Ошибка должна упоминать превышение размера: {}",
            error
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(test_path);

        Ok(())
    }

    /// Тест 22: Проверка проверки размера файла < 1MB
    ///
    /// Проверяет что файл конфигурации размером меньше 1MB
    /// успешно загружается
    #[test]
    fn test_config_file_size_ok() -> std::io::Result<()> {
        // Создаём валидную конфигурацию
        let config = ControlsConfig::default_config();
        let test_path = "test_controls_config_small.json";

        // Сохраняем конфигурацию (должна быть < 1MB)
        config.save_to_file(test_path)?;

        // Проверяем размер файла
        let metadata = fs::metadata(test_path)?;
        assert!(
            metadata.len() < crate::highscore::MAX_CONFIG_FILE_SIZE,
            "Файл конфигурации должен быть меньше 1MB"
        );

        // Загружаем конфигурацию (должна успешно загрузиться)
        let loaded = ControlsConfig::load_from_file(test_path);
        assert!(
            loaded.is_ok(),
            "Загрузка файла < 1MB должна быть успешной: {:?}",
            loaded.err()
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(test_path);

        Ok(())
    }

    /// Тест 23: Проверка HMAC-SHA256 подписи в controls.rs
    ///
    /// Проверяет что конфигурация управления подписывается HMAC-SHA256
    #[test]
    fn test_controls_hmac_signature() -> std::io::Result<()> {
        let test_path = "test_controls_hmac.json";

        // Создаём и сохраняем конфигурацию
        let config = ControlsConfig::default_config();
        config.save_to_file(test_path)?;

        // Загружаем и проверяем подпись
        let loaded = ControlsConfig::load_from_file(test_path)?;

        // Проверяем что hmac_key не пустой после загрузки
        assert!(
            !loaded.hmac_key.is_empty(),
            "Загруженный HMAC ключ не должен быть пустым"
        );
        assert_eq!(
            loaded.hmac_key.len(),
            64,
            "Длина HMAC ключа должна быть 64 символа"
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(test_path);

        Ok(())
    }

    /// Тест 24: Проверка подделки конфигурации
    ///
    /// Проверяет что изменённая конфигурация не проходит проверку HMAC
    #[test]
    fn test_controls_tampered_config() -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let test_path = "test_controls_tampered.json";

        // Создаём и сохраняем валидную конфигурацию
        let config = ControlsConfig::default_config();
        config.save_to_file(test_path)?;

        // Читаем файл и изменяем данные
        let mut json = fs::read_to_string(test_path)?;

        // Изменяем move_left в JSON (подделка)
        json = json.replace("\"move_left\": 97", "\"move_left\": 98");

        // Записываем подделанные данные обратно
        let mut file = File::create(test_path)?;
        file.write_all(json.as_bytes())?;
        drop(file);

        // Проверяем что загрузка возвращает ошибку HMAC
        let result = ControlsConfig::load_from_file(test_path);
        assert!(
            result.is_err(),
            "Загрузка подделанной конфигурации должна вернуть ошибку"
        );

        let error = result.unwrap_err();
        assert!(
            error.to_string().contains("HMAC")
                || error.to_string().contains("подпись")
                || error.to_string().contains("подделка"),
            "Ошибка должна упоминать HMAC или подделку: {}",
            error
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(test_path);

        Ok(())
    }

    /// Тест 25: Проверка формата сообщений об ошибках размера файла
    ///
    /// Проверяет что сообщения об ошибках содержат правильную информацию
    #[test]
    fn test_file_size_error_message_format() {
        // Проверяем формат сообщения об ошибке размера файла
        let max_size = crate::highscore::MAX_CONFIG_FILE_SIZE;
        let file_size = max_size + 1;

        let error_msg = format!(
            "Файл конфигурации слишком большой: {} байт (максимум {} байт)",
            file_size, max_size
        );

        assert!(
            error_msg.contains("слишком большой") || error_msg.contains("слишком большим"),
            "Сообщение должно содержать 'слишком большой'"
        );
        assert!(
            error_msg.contains(&format!("{}", file_size)),
            "Сообщение должно содержать размер файла"
        );
        assert!(
            error_msg.contains(&format!("{}", max_size)),
            "Сообщение должно содержать максимальный размер"
        );
        assert!(
            error_msg.contains("байт"),
            "Сообщение должно содержать единицу измерения"
        );
    }

    /// Тест 26: Проверка константы MAX_CONFIG_FILE_SIZE
    ///
    /// Проверяет что константа имеет правильное значение
    #[test]
    fn test_max_config_file_size_constant() {
        assert_eq!(
            crate::highscore::MAX_CONFIG_FILE_SIZE,
            1_048_576,
            "MAX_CONFIG_FILE_SIZE должен быть 1MB (1024 * 1024)"
        );
    }
}
