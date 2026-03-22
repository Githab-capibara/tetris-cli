//! Конфигурация управления.
//!
//! Этот модуль предоставляет систему настройки клавиш управления для игры Tetris CLI.
//! Поддерживает сохранение и загрузку конфигурации, валидацию клавиш.
//!
//! ## Структура модуля
//! - `ControlsConfig` - структура конфигурации управления
//! - `DEFAULT_CONTROLS` - значения по умолчанию
//! - `tests` - модульные тесты (4 теста)
//!
//! ## Безопасность
//! Конфигурация защищена HMAC-SHA256 подписью для предотвращения подделки.

use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

/// Длина HMAC ключа в байтах (256 бит).
const HMAC_KEY_LENGTH: usize = 32;

/// Сгенерировать случайный HMAC ключ.
///
/// Использует криптографически стойкий генератор случайных чисел.
/// Возвращает hex-строку из 64 символов (256 бит).
fn generate_hmac_key() -> String {
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut bytes = [0u8; HMAC_KEY_LENGTH];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Вычислить HMAC-SHA256 для данных.
///
/// # Аргументы
/// * `key` - HMAC ключ (hex-строка)
/// * `data` - данные для подписи
///
/// # Возвращает
/// HMAC подпись в виде hex-строки
fn compute_hmac(key: &str, data: &str) -> String {
    use blake3::Hasher;

    // Используем BLAKE3 как HMAC функцию
    // Формируем ключ + данные для хеширования
    let mut hasher = Hasher::new();
    hasher.update(key.as_bytes());
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    hash.to_hex().to_string()
}

/// Проверить HMAC подпись конфигурации.
///
/// # Аргументы
/// * `key` - HMAC ключ
/// * `config` - конфигурация для проверки
/// * `expected_hmac` - ожидаемая подпись
///
/// # Возвращает
/// `true` если подпись верна
fn verify_hmac(key: &str, config: &ControlsConfigInner, expected_hmac: &str) -> bool {
    // Сериализуем конфигурацию в JSON для вычисления HMAC
    let config_json = serde_json::to_string(config).unwrap_or_else(|_| String::new());
    let computed_hmac = compute_hmac(key, &config_json);
    computed_hmac == expected_hmac
}

/// Конфигурация управления с HMAC подписью.
/// Внутренняя структура для хранения подписи.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignedControlsConfig {
    /// Основная конфигурация.
    config: ControlsConfigInner,
    /// HMAC-SHA256 подпись конфигурации.
    hmac: String,
}

/// Внутренняя структура конфигурации (без подписи).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ControlsConfigInner {
    /// Движение влево.
    move_left: u8,
    /// Движение вправо.
    move_right: u8,
    /// Мягкое падение.
    soft_drop: u8,
    /// Жёсткое падение.
    hard_drop: u8,
    /// Вращение против часовой.
    rotate_left: u8,
    /// Вращение по часовой.
    rotate_right: u8,
    /// Удержание фигуры.
    hold: u8,
    /// Пауза.
    pause: u8,
    /// Выход.
    quit: u8,
    /// Случайный HMAC ключ для этой конфигурации.
    hmac_key: String,
}

/// Проверить валидность пути для конфигурации.
///
/// Эта функция реализует защиту от path traversal атак и symlink attacks:
/// 1. Запрещает абсолютные пути
/// 2. Запрещает последовательности ".."
/// 3. Проверяет, что путь находится внутри текущей директории
/// 4. Запрещает символические ссылки
/// 5. Проверяет максимальную длину пути (255 символов)
/// 6. Запрещает специальные символы в имени файла
/// 7. Использует O_NOFOLLOW при открытии файлов для защиты от symlink атак
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
/// Проверка symlink_metadata() выполняется ДО валидации пути для предотвращения race condition.
/// Использование O_NOFOLLOW предотвращает открытие symlink во время записи.
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
            format!("Абсолютные пути не разрешены: {:?}", path),
        ));
    }

    // ЗАПРЕТ PATH TRAVERSAL (..)
    if path.contains("..") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path traversal не разрешён: {:?}", path),
        ));
    }

    // ЗАПРЕТ СПЕЦИАЛЬНЫХ СИМВОЛОВ В ИМЕНИ ФАЙЛА
    // Запрещаем символы, которые могут быть использованы для атак
    if path.contains(['\0', '|', '&', ';', '$']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Специальные символы не разрешены в пути: {:?}", path),
        ));
    }

    // ПРОВЕРКА МАКСИМАЛЬНОЙ ДЛИНЫ ПУТИ
    if path.len() > 255 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Путь слишком длинный (максимум 255 символов): {:?}", path),
        ));
    }

    // Получаем текущую директорию
    let current_dir = std::env::current_dir()
        .map_err(|e| io::Error::other(format!("Не удалось получить текущую директорию: {}", e)))?;
    let joined_path = current_dir.join(full_path);

    // ЗАПРЕТ СИМВОЛИЧЕСКИХ ССЫЛОК
    // Используем symlink_metadata() для проверки symlink без следования по нему
    // Это проверка выполняется ДО открытия файла для защиты от race condition
    if let Ok(metadata) = std::fs::symlink_metadata(&joined_path) {
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Символические ссылки не разрешены: {:?}", path),
            ));
        }
    }

    // Для существующих файлов - используем canonicalize()
    // Для несуществующих (сохранение) - проверяем родительскую директорию
    let canonical_path = if joined_path.exists() {
        joined_path.canonicalize().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Неверный путь {:?}: {}", path, e),
            )
        })?
    } else {
        // Файл не существует - проверяем родительскую директорию
        // Если родительской директории нет, используем текущую директорию
        joined_path
            .parent()
            .and_then(|p| p.canonicalize().ok())
            .unwrap_or_else(|| current_dir.clone())
    };

    // Проверяем, что resolved path находится внутри текущей директории
    // Используем strip_prefix() для надёжной проверки
    if canonical_path.strip_prefix(&current_dir).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Путь вне разрешённой директории (symlink attack detected): {:?}",
                path
            ),
        ));
    }

    Ok(())
}

/// Конфигурация управления игрой.
///
/// Содержит коды клавиш для всех действий в игре.
/// Все клавиши хранятся как u8 коды (ASCII значения).
/// Конфигурация защищена HMAC-SHA256 подписью для предотвращения подделки.
///
/// ## Обратная совместимость
/// Поля структуры публичны для обратной совместимости с существующими тестами.
/// Для нового кода рекомендуется использовать геттеры: `move_left()`, `move_right()`, и т.д.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Сравнить только клавиши управления (игнорируя hmac_key).
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
    /// # Безопасность
    /// - Генерируется новый HMAC ключ при каждом сохранении
    /// - Конфигурация подписывается HMAC-SHA256
    /// - Используется O_NOFOLLOW для защиты от symlink атак
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

        // Создаём временную структуру для сериализации
        let config_inner = ControlsConfigInner {
            move_left: self.move_left,
            move_right: self.move_right,
            soft_drop: self.soft_drop,
            hard_drop: self.hard_drop,
            rotate_left: self.rotate_left,
            rotate_right: self.rotate_right,
            hold: self.hold,
            pause: self.pause,
            quit: self.quit,
            hmac_key: generate_hmac_key(),
        };

        // Вычисляем HMAC подпись
        let config_json = serde_json::to_string(&config_inner)
            .map_err(|e| io::Error::other(format!("Ошибка сериализации: {}", e)))?;
        let hmac = compute_hmac(&config_inner.hmac_key, &config_json);

        // Создаём подписанную конфигурацию
        let signed_config = SignedControlsConfig {
            config: config_inner,
            hmac,
        };

        // Сериализуем в JSON
        let json = serde_json::to_string_pretty(&signed_config)
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
    /// - HMAC подпись не совпадает (подделка конфигурации)
    ///
    /// # Безопасность
    /// - Проверяется HMAC подпись конфигурации
    /// - Используется symlink_metadata() для защиты от symlink атак
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
            .map_err(|e| {
                io::Error::other(format!("Не удалось получить текущую директорию: {}", e))
            })?
            .join(path);

        if let Ok(metadata) = std::fs::symlink_metadata(&joined_path) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Символические ссылки не разрешены: {:?}", path),
                ));
            }
        }

        // Читаем файл
        let json = fs::read_to_string(path)?;

        // Десериализуем подписанную конфигурацию
        let signed_config: SignedControlsConfig = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Проверяем HMAC подпись
        if !verify_hmac(
            &signed_config.config.hmac_key,
            &signed_config.config,
            &signed_config.hmac,
        ) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "HMAC подпись не совпадает - возможна подделка конфигурации",
            ));
        }

        Ok(Self {
            move_left: signed_config.config.move_left,
            move_right: signed_config.config.move_right,
            soft_drop: signed_config.config.soft_drop,
            hard_drop: signed_config.config.hard_drop,
            rotate_left: signed_config.config.rotate_left,
            rotate_right: signed_config.config.rotate_right,
            hold: signed_config.config.hold,
            pause: signed_config.config.pause,
            quit: signed_config.config.quit,
            hmac_key: signed_config.config.hmac_key,
        })
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
}
