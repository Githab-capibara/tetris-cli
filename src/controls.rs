//! Конфигурация управления.
//!
//! Модуль предоставляет систему настройки клавиш управления для игры.
//! Поддерживает сохранение/загрузку конфигурации и валидацию клавиш.

// TODO: для будущей функциональности
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// RATE LIMITING ДЛЯ save_to_file (Исправление #29)
// ============================================================================
/// Минимальный интервал между сохранениями в секундах.
const SAVE_RATE_LIMIT_SECS: u64 = 60;

/// Последняя метка времени сохранения конфигурации.
/// Используется для rate limiting чтобы предотвратить частые записи на диск.
static LAST_SAVE_TIMESTAMP: AtomicU64 = AtomicU64::new(0);

/// Сбросить rate limiting для тестов.
///
/// # Безопасность
/// Эта функция предназначена ТОЛЬКО для тестов.
/// Не вызывайте её в производственном коде.
#[cfg(test)]
pub fn reset_rate_limit_for_tests() {
    LAST_SAVE_TIMESTAMP.store(0, Ordering::Relaxed);
}

// ============================================================================
// ВАЛИДАТОР ПУТЕЙ (переэкспортирован из crate::validation)
// ============================================================================
// PathValidator, PathError и PathErrorKind теперь находятся в модуле validation
// для централизации кода валидации.

// Переэкспорт для обратной совместимости
pub use crate::validation::path::DEFAULT_PATH_VALIDATOR;

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

/// Проверить валидность пути для конфигурации.
///
/// Эта функция использует централизованный `PathValidator` для всех проверок:
/// 1. Проверка длины пути (максимум 255 символов)
/// 2. Проверка разрешённых символов
/// 3. Защита от symlink атак
/// 4. Защита от path traversal (..)
/// 5. Запрет абсолютных путей
/// 6. Проверка, что путь находится внутри текущей директории
///
/// # Аргументы
/// * `path` - путь для проверки
///
/// # Возвращает
/// - `Ok(())` если путь валиден
/// - `Err(io::Error)` если путь невалиден
///
/// # Безопасность
/// Защищает от symlink атак и path traversal.
/// Использует `O_NOFOLLOW` при открытии файлов для защиты от symlink атак.
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::controls::validate_config_path;
/// validate_config_path("config.json").unwrap();
/// ```
///
/// # Исправление #4 (DRY)
/// Функция полностью делегирует валидацию `PathValidator`, устраняя дублирование кода.
#[track_caller]
fn validate_config_path(path: &str) -> io::Result<()> {
    let full_path = Path::new(path);

    // Проверка на абсолютные пути
    DEFAULT_PATH_VALIDATOR
        .validate_not_absolute(full_path)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

    // Проверка на path traversal
    DEFAULT_PATH_VALIDATOR
        .validate_no_traversal(path)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

    // Используем PathValidator для всех остальных проверок (DRY)
    DEFAULT_PATH_VALIDATOR
        .validate(full_path)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

    // Проверка, что путь внутри директории
    let current_dir = std::env::current_dir()
        .map_err(|e| io::Error::other(format!("Не удалось получить текущую директорию: {e}")))?;
    let joined_path = current_dir.join(full_path);

    DEFAULT_PATH_VALIDATOR
        .validate_within_directory(&joined_path, &current_dir)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.message))?;

    Ok(())
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
    /// # Исправление #29
    /// Добавлено rate limiting: минимум 60 секунд между сохранениями.
    /// Это предотвращает износ диска при частых вызовах функции.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        // Исправление #29: rate limiting для предотвращения частых записей
        // Отключаем rate limiting для тестов
        #[cfg(not(test))]
        {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let last_save = LAST_SAVE_TIMESTAMP.load(Ordering::Relaxed);
            if now - last_save < SAVE_RATE_LIMIT_SECS {
                // Тихо игнорируем сохранение если прошло слишком мало времени
                return Ok(());
            }

            // Обновляем метку времени перед сохранением
            LAST_SAVE_TIMESTAMP.store(now, Ordering::Relaxed);
        }

        // Валидация пути с использованием общей функции
        validate_config_path(path)?;

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

        // Вычисляем keyed hash подпись
        let signature = crate::crypto::keyed_hash(&hmac_key, &config_json);

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
        // Валидация пути с использованием общей функции
        validate_config_path(path)?;

        // Проверяем, что файл не является symlink
        let joined_path = std::env::current_dir()
            .map_err(|e| io::Error::other(format!("Не удалось получить текущую директорию: {e}")))?
            .join(path);

        if let Ok(metadata) = std::fs::symlink_metadata(&joined_path) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Символические ссылки не разрешены: {path:?}"),
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

        let expected_signature = crate::crypto::keyed_hash(&config.hmac_key, &config_json);

        if config.signature != expected_signature {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Keyed hash подпись не совпадает - возможна подделка конфигурации",
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
        // Сбрасываем rate limiting для тестов
        reset_rate_limit_for_tests();

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
}
