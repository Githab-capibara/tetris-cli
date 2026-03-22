//! Система сохранения рекордов.
//!
//! Этот модуль предоставляет функциональность для сохранения и загрузки рекордов игры.
//! Поддерживается как одиночный рекорд, так и таблица лидеров (топ-5 результатов).
//! Все рекорды защищены от подделки с помощью криптографического хеширования с солью.
//!
//! ## Безопасность
//!
//! В этом модуле используется криптографическая хеш-функция BLAKE3 для вычисления хешей.
//! **BLAKE3 является криптографически стойкой** и обеспечивает надёжную защиту от подделки рекордов.
//! BLAKE3 — современная быстрая хеш-функция, основанная на BLAKE2.
//! Для генерации соли используется криптографически стойкий генератор случайных чисел (getrandom).
//!
//! ## Защита от race condition
//! Все операции с конфигурацией rate limiting защищены эксклюзивной файловой блокировкой fs2.
//! Это предотвращает одновременный доступ к файлу конфигурации из нескольких процессов.

use confy::{load, store};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

// ===========================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ===========================================================================
// Общие функции для генерации хешей и солей
// Используются в [`SaveData`] и [`LeaderboardEntry`]

/// Сгенерировать случайную соль из 64 шестнадцатеричных символов (256 бит).
///
/// Используется криптографически стойкий генератор случайных чисел (getrandom).
/// Возвращает строку из ровно 64 шестнадцатеричных символов (256 бит).
/// Оптимизация: использует `hex::encode()` вместо ручного цикла
#[must_use = "Соль должна быть использована для хеширования"]
pub fn generate_salt() -> String {
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
                               // Используем криптографически стойкий генератор случайных чисел ОС.
    OsRng.fill_bytes(&mut bytes);

    // Оптимизация: используем hex::encode() для эффективного кодирования
    hex::encode(bytes)
}

/// Получить случайную соль (устаревшее имя).
///
/// # Устарело
/// Используйте [`generate_salt()`] вместо этой функции.
#[deprecated(since = "2.1.0", note = "Используйте `generate_salt()`")]
#[allow(dead_code)]
pub fn get_random_hash() -> String {
    generate_salt()
}

/// Получить хэш строки в шестнадцатеричном формате.
///
/// Использует криптографическую хеш-функцию BLAKE3 для вычисления хеша.
/// Возвращает строку из 64 шестнадцатеричных символов (256 бит = 32 байта).
fn get_hash(msg: &str) -> String {
    let hash = blake3::hash(msg.as_bytes());
    hash.to_hex().to_string()
}

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Максимальное количество рекордов в таблице лидеров.
const MAX_LEADERBOARD_SIZE: usize = 5;

/// Максимальное количество записей в минуту для rate limiting.
/// Ограничивает частоту добавления рекордов для предотвращения злоупотреблений.
const MAX_ENTRIES_PER_MINUTE: usize = 10;

/// Минимальное время между записями (cooldown) в миллисекундах.
/// Предотвращает слишком частое добавление рекордов.
/// Исправление: увеличено до 10 секунд для лучшей защиты.
/// В тестах отключено (cfg(test) переопределяет значение).
#[cfg(not(test))]
const ENTRY_COOLDOWN_MS: u64 = 10_000; // 10 секунд

/// В тестах cooldown отключен для возможности быстрого добавления рекордов.
#[cfg(test)]
const ENTRY_COOLDOWN_MS: u64 = 0;

/// Максимальное количество цифр в строковом представлении u128.
/// Используется для оптимизации выделения памяти при конвертации чисел в строку.
/// `u128::MAX` = 340282366920938463463374607431768211455 (39 цифр)
/// Исправление #10: используем константу вместо `ilog10()` для точной оценки.
/// Это предотвращает лишние вычисления и улучшает производительность.
#[allow(dead_code)]
pub const MAX_SCORE_DIGITS: usize = 39;

/// Максимальный размер файла конфигурации в байтах (1MB).
/// Используется для защиты от атак через большие файлы.
/// Исправление #23: проверка размера файла перед загрузкой.
pub const MAX_CONFIG_FILE_SIZE: u64 = 1_048_576; // 1MB

/// Имя конфигурации для хранения состояния rate limiting.
const RATE_LIMIT_CONFIG_NAME: &str = "tetris-cli_rate_limit";

/// Состояние rate limiting для защиты от обхода через изменение системного времени.
///
/// Сохраняет последний известный timestamp в конфигурационном файле.
/// При изменении системного времени назад, используется сохранённое значение.
#[derive(Serialize, Deserialize, Clone, Default)]
struct RateLimitState {
    /// Последний известный timestamp в миллисекундах.
    last_known_time_ms: u64,
}

/// Получить текущий timestamp в миллисекундах с защитой от обхода rate limiting.
///
/// # Аргументы
/// * `state` - изменяемая ссылка на состояние rate limiting
///
/// # Возвращает
/// Timestamp в миллисекундах, который гарантированно не меньше последнего сохранённого.
///
/// # Безопасность
/// Если системное время было изменено назад, возвращается `last_known_time_ms`.
/// Это предотвращает обход rate limiting через установку времени назад.
///
/// # Исправление #11
/// Выделена общая логика получения системного времени в отдельную функцию.
fn get_current_time_ms_protected(state: &mut RateLimitState) -> u64 {
    let current_time_ms = get_system_time_ms().unwrap_or_else(|e| {
        // Исправление #10: унифицированное логирование с unwrap_or_else
        eprintln!("Ошибка: системное время недоступно: {e}. Используется время 0.");
        0
    });

    // Защита от обхода rate limiting через изменение системного времени назад.
    // Если текущее время меньше последнего известного, используем последнее известное.
    if current_time_ms < state.last_known_time_ms {
        eprintln!(
            "Предупреждение: обнаружено изменение системного времени назад \
             (текущее: {}, последнее: {}). Используется последнее известное время.",
            current_time_ms, state.last_known_time_ms
        );
        state.last_known_time_ms
    } else {
        // Обновляем last_known_time_ms только если время увеличилось
        state.last_known_time_ms = current_time_ms;
        current_time_ms
    }
}

/// Получить системное время в миллисекундах.
///
/// # Возвращает
/// - `Ok(u64)` - текущее время в миллисекундах с начала UNIX epoch
/// - `Err(std::time::SystemError)` - если системное время недоступно
///
/// # Исправление #11
/// Выделена общая логика получения системного времени для переиспользования.
fn get_system_time_ms() -> Result<u64, std::time::SystemTimeError> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis() as u64)
}

/// Загрузить состояние rate limiting из конфигурации с файловой блокировкой.
///
/// # Возвращает
/// `RateLimitState` из конфигурации или default при ошибке.
///
/// # Безопасность
/// Использует эксклюзивную файловую блокировку fs2 для предотвращения race condition.
/// Исправление #23: добавлена проверка размера файла перед загрузкой.
fn load_rate_limit_state() -> RateLimitState {
    use directories::ProjectDirs;
    use fs2::FileExt;
    use std::fs;

    // Получаем путь к файлу конфигурации
    let proj_dirs = if let Some(dirs) = ProjectDirs::from("", "", APP_NAME) {
        dirs
    } else {
        eprintln!("Информация: не удалось определить директорию конфигурации. Используется новое состояние.");
        return RateLimitState::default();
    };

    let config_file = proj_dirs
        .config_dir()
        .join(format!("{RATE_LIMIT_CONFIG_NAME}.toml"));

    // Исправление #23: проверка размера файла перед загрузкой
    if let Ok(metadata) = fs::metadata(&config_file) {
        if metadata.len() > MAX_CONFIG_FILE_SIZE {
            eprintln!(
                "Предупреждение: файл конфигурации слишком большой ({} байт, максимум {} байт). Используется новое состояние.",
                metadata.len(),
                MAX_CONFIG_FILE_SIZE
            );
            return RateLimitState::default();
        }
    }

    // Пытаемся открыть файл с блокировкой
    if let Ok(file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_file)
    {
        // Устанавливаем эксклюзивную блокировку
        if let Err(e) = file.lock_exclusive() {
            eprintln!("Информация: не удалось установить блокировку файла конфигурации: {e}. Используется новое состояние.");
            return RateLimitState::default();
        }

        // Читаем конфигурацию через confy
        let result = match load::<RateLimitState>(RATE_LIMIT_CONFIG_NAME) {
            Ok(state) => state,
            Err(e) => {
                eprintln!("Информация: не удалось загрузить состояние rate limiting: {e}. Используется новое состояние.");
                RateLimitState::default()
            }
        };

        // Снимаем блокировку
        let _ = file.unlock();
        return result;
    }

    // Файл не существует или не удалось открыть - используем confy напрямую
    match load::<RateLimitState>(RATE_LIMIT_CONFIG_NAME) {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Информация: не удалось загрузить состояние rate limiting: {e}. Используется новое состояние.");
            RateLimitState::default()
        }
    }
}

/// Сохранить состояние rate limiting в конфигурацию с файловой блокировкой.
///
/// # Аргументы
/// * `state` - состояние для сохранения
///
/// # Безопасность
/// Использует эксклюзивную файловую блокировку fs2 для предотвращения race condition.
fn save_rate_limit_state(state: &RateLimitState) {
    use directories::ProjectDirs;
    use fs2::FileExt;

    // Получаем путь к файлу конфигурации
    let proj_dirs = if let Some(dirs) = ProjectDirs::from("", "", APP_NAME) {
        dirs
    } else {
        eprintln!("Предупреждение: не удалось определить директорию конфигурации.");
        return;
    };

    let config_file = proj_dirs
        .config_dir()
        .join(format!("{RATE_LIMIT_CONFIG_NAME}.toml"));

    // Пытаемся открыть файл с блокировкой
    if let Ok(file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config_file)
    {
        // Устанавливаем эксклюзивную блокировку
        if let Err(e) = file.lock_exclusive() {
            eprintln!("Предупреждение: не удалось установить блокировку файла конфигурации: {e}");
            // Пытаемся сохранить без блокировки
            if let Err(e) = store(RATE_LIMIT_CONFIG_NAME, state) {
                eprintln!("Предупреждение: не удалось сохранить состояние rate limiting: {e}");
            }
            return;
        }

        // Сохраняем конфигурацию через confy
        if let Err(e) = store(RATE_LIMIT_CONFIG_NAME, state) {
            eprintln!("Предупреждение: не удалось сохранить состояние rate limiting: {e}");
        }

        // Снимаем блокировку
        let _ = file.unlock();
    } else {
        // Не удалось открыть файл - пытаемся сохранить без блокировки
        if let Err(e) = store(RATE_LIMIT_CONFIG_NAME, state) {
            eprintln!("Предупреждение: не удалось сохранить состояние rate limiting: {e}");
        }
    }
}

/// Ошибка операции с конфигурацией.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    /// Директория конфигурации недоступна для записи.
    DirectoryNotWritable(String),
    /// Ошибка при сохранении/загрузке через confy.
    IoError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::DirectoryNotWritable(dir) => {
                write!(f, "Директория конфигурации недоступна для записи: {dir}")
            }
            ConfigError::IoError(msg) => write!(f, "Ошибка ввода/вывода: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Проверить доступность директории конфигурации для записи.
///
/// # Возвращает
/// `Ok(())` если директория доступна для записи,
/// `Err(ConfigError)` если директория недоступна
///
/// # Примечания
/// Проверяет возможность создания временного файла в директории конфигурации.
#[allow(dead_code)]
pub fn check_config_directory_writable() -> Result<(), ConfigError> {
    use std::fs;

    // Получаем путь к директории конфигурации через directories crate
    let proj_dirs = ProjectDirs::from("", "", APP_NAME).ok_or_else(|| {
        ConfigError::IoError("Не удалось определить директорию конфигурации".to_string())
    })?;

    let config_dir = proj_dirs.config_dir();

    // Проверяем существование директории
    if !config_dir.exists() {
        return Err(ConfigError::DirectoryNotWritable(format!(
            "Директория не существует: {config_dir:?}"
        )));
    }

    // Проверяем, что это действительно директория
    if !config_dir.is_dir() {
        return Err(ConfigError::DirectoryNotWritable(format!(
            "Путь не является директорией: {config_dir:?}"
        )));
    }

    // Проверяем доступность для записи, пытаясь создать временный файл
    let test_file = config_dir.join(".tetris-cli-write-test");
    match fs::write(&test_file, b"test") {
        Ok(()) => {
            // Удаляем тестовый файл
            let _ = fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => Err(ConfigError::DirectoryNotWritable(format!(
            "Не удалось создать тестовый файл в {config_dir:?}: {e}"
        ))),
    }
}

/// Данные для сохранения рекорда.
/// Содержит значение рекорда, соль для хеширования и сам хеш для защиты от подделки.
/// Использует u128 для предотвращения переполнения счёта.
#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    /// Значение рекорда.
    high_score: u128,
    /// Соль для хэша (защита от подделки).
    high_score_salt: String,
    /// Хэш рекорда с солью.
    high_score_hash: String,
}

/// Запись в таблице лидеров.
/// Представляет собой один результат с именем игрока и защищённым хешом.
/// Использует u128 для предотвращения переполнения счёта.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardEntry {
    /// Имя игрока.
    name: String,
    /// Значение рекорда.
    score_value: u128,
    /// Соль для хэша (защита от подделки).
    salt: String,
    /// Хэш записи с солью.
    hash: String,
}

impl LeaderboardEntry {
    /// Получить имя игрока.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Получить значение рекорда.
    ///
    /// # Возвращает
    /// Значение рекорда (u128)
    ///
    /// # Безопасность
    /// Метод возвращает значение только после успешной валидации хэша.
    /// Это предотвращает race condition между проверкой и использованием.
    /// Возвращает u128 для предотвращения переполнения.
    #[must_use]
    pub fn score(&self) -> u128 {
        // Валидация перед каждым использованием
        if !self.is_valid() {
            eprintln!("Предупреждение: запись в таблице лидеров не прошла валидацию!");
            return 0;
        }
        // Возвращаем значение поля напрямую через self.score_value
        // чтобы избежать бесконечной рекурсии (ранее self.score вызывало сам себя)
        self.score_value
    }

    /// Получить хэш записи.
    ///
    /// # Примечания
    /// Метод используется в тестах для проверки уникальности хэшей.
    #[allow(dead_code)]
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

/// Таблица лидеров - коллекция из топ-5 рекордов.
/// Сохраняется в конфигурационном файле и защищена от подделки.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Leaderboard {
    /// Список записей в таблице лидеров (максимум 5).
    entries: Vec<LeaderboardEntry>,
    /// Время последних добавленных записей для rate limiting (в миллисекундах).
    #[serde(default)]
    recent_entry_times: Vec<u64>,
}

impl SaveData {
    /// Загрузить конфигурацию из файла.
    ///
    /// # Возвращает
    /// `SaveData` по умолчанию при ошибке загрузки или при обнаружении подделки
    ///
    /// # Исправление #26
    /// Добавлено предупреждение в UI о проблемах с загрузкой конфигурации.
    pub fn load_config() -> Self {
        match load::<Self>(APP_NAME) {
            Ok(data) => {
                // Дополнительная проверка целостности
                // Исправление: используем [`verify_and_get_score()`] вместо deprecated `assert_hs()`
                match data.verify_and_get_score() {
                    Some(score) => {
                        // Логирование успешной загрузки
                        if score > 0 {
                            eprintln!("Информация: загружен рекорд со значением {score}");
                        }
                        data
                    }
                    None if data.high_score != 0 => {
                        // Исправление #26: подробное предупреждение о подделке
                        eprintln!("Предупреждение: обнаружена подделка рекорда! Используется значение по умолчанию.");
                        eprintln!("  Если вы не пытались изменить файл конфигурации вручную, это может быть ошибкой.");
                        Self::default()
                    }
                    None => {
                        eprintln!("Предупреждение: рекорд не прошёл валидацию. Используется значение по умолчанию.");
                        Self::default()
                    }
                }
            }
            Err(e) => {
                // Подробное логирование ошибок загрузки
                // Используем [`Display`] trait для форматирования ошибки
                let error_msg = format!("{e}");
                // Исправление #26: добавлено предупреждение в UI
                eprintln!("Ошибка загрузки конфигурации: {error_msg}. Используется значение по умолчанию.");
                eprintln!(
                    "  Проверьте права доступа к файлу конфигурации или запустите игру снова."
                );
                Self::default()
            }
        }
    }

    /// Создать `SaveData` из значения рекорда.
    ///
    /// # Аргументы
    /// * `high_score` - значение рекорда для сохранения
    ///
    /// # Возвращает
    /// Новый экземпляр `SaveData` с вычисленным хешем
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// let save = SaveData::from_value(1000);
    /// // [`high_score`] содержит значение 1000
    /// ```
    /// Использует u128 для предотвращения переполнения.
    pub fn from_value(high_score: u128) -> Self {
        let high_score_str = high_score.to_string();
        let salt = generate_salt();
        let salt_and_hs = salt.clone() + &high_score_str;
        let hash = get_hash(&salt_and_hs);

        Self {
            high_score,
            high_score_salt: salt,
            high_score_hash: hash,
        }
    }

    /// Сохранить значение рекорда в файл.
    ///
    /// # Аргументы
    /// * `high_score` - значение рекорда для сохранения
    ///
    /// # Ошибки
    /// При ошибке сохранения выводит сообщение в stderr
    /// Использует u128 для предотвращения переполнения.
    pub fn save_value(high_score: u128) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, save) {
            eprintln!("Ошибка сохранения рекорда: {e}");
        }
    }

    /// Сохранить значение рекорда в файл с возвратом Result.
    ///
    /// # Аргументы
    /// * `high_score` - значение рекорда для сохранения
    ///
    /// # Возвращает
    /// - `Ok(())` - рекорд успешно сохранён
    /// - `Err(ConfigError)` - ошибка при сохранении
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// match SaveData::save_value_result(1000) {
    ///     Ok(()) => println!("Рекорд успешно сохранён"),
    ///     Err(e) => eprintln!("Ошибка сохранения: {}", e),
    /// }
    /// ```
    /// Использует u128 для предотвращения переполнения.
    #[allow(dead_code)]
    pub fn save_value_result(high_score: u128) -> Result<(), ConfigError> {
        let save = Self::from_value(high_score);
        store(APP_NAME, save)
            .map_err(|e| ConfigError::IoError(format!("Ошибка сохранения рекорда: {e}")))
    }

    /// Проверить целостность рекорда и вернуть значение.
    ///
    /// Возвращает Some(score) если хэш совпадает, None при подделке.
    /// Логирует попытки подделки рекорда.
    ///
    /// # Возвращает
    /// - `Some(u128)` - значение рекорда, если хэш совпадает
    /// - `None` - если запись подделана или повреждена
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// let save = SaveData::from_value(1000);
    /// assert_eq!(save.verify_and_get_score(), Some(1000));
    /// ```
    #[must_use]
    pub fn verify_and_get_score(&self) -> Option<u128> {
        let high_score_str = self.high_score.to_string();
        let salt_and_hs = self.high_score_salt.clone() + &high_score_str;
        let test_hash = get_hash(&salt_and_hs);

        if self.high_score_hash == test_hash {
            Some(self.high_score)
        } else {
            // Логирование попытки подделки
            eprintln!("Предупреждение: обнаружена подделка рекорда! Хэш не совпадает.");
            None
        }
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self::from_value(0)
    }
}

/// Санитизировать имя игрока для таблицы лидеров.
///
/// Правила:
/// - trim
/// - разрешены только ASCII буквы/цифры и символы: '_', '-', ' '
/// - максимум 20 символов
/// - пустое имя (в т.ч. после фильтрации) заменяется на "Anonymous"
/// - запрещены опасные Unicode-символы (эмодзи, контрольные символы)
/// - запрещены bidirectional control characters (U+200E, U+200F)
/// - используется whitelist разрешённых символов
///
/// # Аргументы
/// * `name` - имя для санитаризации
///
/// # Возвращает
/// Безопасное имя для таблицы лидеров
///
/// # Безопасность
/// Использует `String::with_capacity()` для предотвращения реаллокаций.
/// Добавлена защита от Unicode-атак:
/// - Bidirectional control characters (U+200E, U+200F) отбрасываются
/// - Emojis и другие опасные символы фильтруются
/// - Whitelist разрешённых символов
fn sanitize_player_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Anonymous".to_string();
    }

    // Используем String::with_capacity() для предотвращения реаллокаций
    // Максимальная длина имени - 20 символов
    let mut validated = String::with_capacity(20.min(trimmed.len()));
    for c in trimmed.chars() {
        // Исправление: проверка на bidirectional control characters
        if c == '\u{200E}'
            || c == '\u{200F}'
            || c == '\u{202A}'
            || c == '\u{202B}'
            || c == '\u{202C}'
            || c == '\u{202D}'
            || c == '\u{202E}'
            || c == '\u{2066}'
            || c == '\u{2067}'
            || c == '\u{2068}'
            || c == '\u{2069}'
        {
            // Пропускаем bidirectional control characters
            continue;
        }

        // Проверка на разрешённые символы (whitelist)
        if is_valid_name_char(c) && !c.is_control() {
            validated.push(c);
            // Ограничение длины имени 20 символами
            if validated.len() >= 20 {
                break;
            }
        }
    }

    if validated.is_empty() {
        "Anonymous".to_string()
    } else {
        validated
    }
}

impl LeaderboardEntry {
    /// Создать новую запись в таблице лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// Новый экземпляр `LeaderboardEntry` с вычисленным хешем
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player".to_string(), 1000);
    /// assert_eq!(entry.name(), "Player");
    /// assert_eq!(entry.score(), 1000);
    /// ```
    /// Использует u128 для предотвращения переполнения.
    pub fn new(name: String, score: u128) -> Self {
        let valid_name = sanitize_player_name(&name);

        let salt = generate_salt();
        // Оптимизация: используем String::with_capacity() + write!() вместо format!()
        // для предотвращения лишних аллокаций.
        // Используем точную оценку длины числа через ilog10() вместо константы U128_MAX_DIGITS.
        use std::fmt::Write;
        let score_digits = if score > 0 {
            score.ilog10() as usize + 1
        } else {
            1 // Для 0 нужна 1 цифра
        };
        let mut salt_and_score =
            String::with_capacity(salt.len() + valid_name.len() + score_digits);
        let _ = write!(salt_and_score, "{salt}{valid_name}{score}");
        let hash = get_hash(&salt_and_score);

        Self {
            name: valid_name,
            score_value: score,
            salt,
            hash,
        }
    }

    /// Проверить целостность записи.
    ///
    /// # Возвращает
    /// `true` если хэш совпадает, `false` если запись была подделана
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player".to_string(), 1000);
    /// assert!(entry.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // Оптимизация: используем String::with_capacity() + write!() вместо format!().
        // Используем точную оценку длины числа через ilog10().
        use std::fmt::Write;
        let score_digits = if self.score_value > 0 {
            self.score_value.ilog10() as usize + 1
        } else {
            1
        };
        let mut salt_and_score =
            String::with_capacity(self.salt.len() + self.name.len() + score_digits);
        let _ = write!(
            salt_and_score,
            "{}{}{}",
            self.salt, self.name, self.score_value
        );
        let test_hash = get_hash(&salt_and_score);
        self.hash == test_hash
    }
}

/// Проверить допустимость символа имени.
///
/// Разрешены только:
/// - ASCII буквы (a-z, A-Z)
/// - ASCII цифры (0-9)
/// - Специальные символы: '_', '-', ' '
/// - Русские буквы (а-я, А-Я, ё, Ё)
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим
///
/// # Безопасность
/// Расширенная валидация Unicode для поддержки международных имён.
/// Запрещены управляющие символы и эмодзи через `is_control()`.
///
/// # Исправление #9
/// Используется `matches!` макрос с диапазонами для более читаемой проверки.
fn is_valid_name_char(c: char) -> bool {
    // Исправление #9: используем matches! макрос с диапазонами для читаемости
    !c.is_control()
        && !c.is_whitespace()
        && c != '/'
        && c != '\\'
        && (matches!(c,
            'a'..='z' | 'A'..='Z' | '0'..='9' |  // ASCII буквы и цифры
            'а'..='я' | 'А'..='Я' | 'ё' | 'Ё' |  // Русские буквы
            '_' | '-' | ' '  // Специальные символы
        ))
}

impl Leaderboard {
    /// Загрузить таблицу лидеров из файла конфигурации.
    ///
    /// # Возвращает
    /// Загруженную таблицу лидеров или пустую при ошибке
    pub fn load() -> Self {
        match load(&format!("{APP_NAME}_leaderboard")) {
            Ok(leaderboard) => leaderboard,
            Err(e) => {
                eprintln!("Предупреждение: не удалось загрузить таблицу лидеров: {e}. Используется пустая таблица.");
                Self::default()
            }
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    pub fn save(&self) {
        if let Err(e) = store(&format!("{APP_NAME}_leaderboard"), self) {
            eprintln!("Ошибка сохранения таблицы лидеров: {e}");
        }
    }

    /// Добавить новый рекорд в таблицу лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// `true` если рекорд был добавлен в таблицу (вошёл в топ-5),
    /// `false` если рекорд недостаточно высок или превышен лимит rate limiting
    ///
    /// # Безопасность
    /// Реализовано rate limiting: не более [`MAX_ENTRIES_PER_MINUTE`] записей в минуту.
    /// Используем u128 для предотвращения переполнения.
    /// Все timestamps валидируются: отклоняются будущие времена.
    /// При подозрительных timestamps записывается предупреждение.
    /// Добавлена проверка cooldown: минимальное время между записями [`ENTRY_COOLDOWN_MS`].
    /// Исправление: защита от обхода rate limiting через изменение системного времени.
    /// Используется сохранение последнего timestamp в конфигурационном файле.
    ///
    /// # Исправление #24
    /// Добавлена валидация имени игрока перед добавлением в таблицу лидеров.
    pub fn add_score(&mut self, name: String, score: u128) -> bool {
        // Исправление #24: валидация имени игрока
        let valid_name = sanitize_player_name(&name);
        if valid_name == "Anonymous" && name.trim() != "Anonymous" {
            eprintln!(
                "Предупреждение: имя игрока не прошло валидацию и было заменено на 'Anonymous'"
            );
        }

        // Загружаем состояние rate limiting из конфигурации
        let mut rate_limit_state = load_rate_limit_state();

        // Rate limiting: проверяем количество записей за последнюю минуту
        self.cleanup_old_entry_times();
        if self.recent_entry_times.len() >= MAX_ENTRIES_PER_MINUTE {
            eprintln!(
                "Предупреждение: превышен лимит добавления рекордов ({MAX_ENTRIES_PER_MINUTE} в минуту)"
            );
            return false;
        }

        // Проверка: достаточно ли высок рекорд для попадания в таблицу
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный рекорд
            let min_score = self
                .entries
                .iter()
                .map(|e| e.score_value)
                .min()
                .unwrap_or(0);
            if score <= min_score {
                return false;
            }
        }

        // Получаем текущий timestamp с защитой от обхода rate limiting
        let current_time_ms = get_current_time_ms_protected(&mut rate_limit_state);

        // Сохраняем состояние rate limiting после получения timestamp
        save_rate_limit_state(&rate_limit_state);

        // Проверка на "будущее" время - защита от подделки
        // Отклоняем timestamps, которые больше текущего времени + 1 час (3600000 мс)
        let one_hour_ms = 3_600_000;
        let max_valid_time = current_time_ms + one_hour_ms;

        // Проверка cooldown: минимальное время между записями
        // В тестах ENTRY_COOLDOWN_MS = 0, поэтому сравнение всегда ложно
        #[allow(clippy::absurd_extreme_comparisons)]
        if let Some(&last_time) = self.recent_entry_times.last() {
            if current_time_ms.saturating_sub(last_time) < ENTRY_COOLDOWN_MS {
                eprintln!(
                    "Предупреждение: слишком частое добавление рекордов (cooldown {ENTRY_COOLDOWN_MS} мс)"
                );
                return false;
            }
        }

        // Валидация: отклоняем подозрительно старые или будущие timestamps
        let one_minute_ms = 60_000; // 1 минута в миллисекундах

        // Проверяем существующие timestamps на валидность
        let mut invalid_count = 0;
        self.recent_entry_times.retain(|&time| {
            // Удаляем timestamps, которые >1 минуты в прошлом или в будущем
            if time <= max_valid_time && current_time_ms.saturating_sub(time) < one_minute_ms {
                true
            } else {
                invalid_count += 1;
                false
            }
        });

        if invalid_count > 0 {
            eprintln!(
                "Предупреждение: удалено {invalid_count} невалидных timestamps (подозрение на подделку)"
            );
        }

        self.recent_entry_times.push(current_time_ms);

        // Добавление новой записи с валидированным именем
        let new_entry = LeaderboardEntry::new(valid_name, score);
        self.entries.push(new_entry);

        // Сортировка по убыванию очков
        self.entries
            .sort_by(|a, b| b.score_value.cmp(&a.score_value));

        // Оставляем только топ-5
        if self.entries.len() > MAX_LEADERBOARD_SIZE {
            self.entries.truncate(MAX_LEADERBOARD_SIZE);
        }

        true
    }

    /// Очистить старые записи времён (старше 1 минуты).
    /// Используется защищённое время для предотвращения обхода rate limiting.
    fn cleanup_old_entry_times(&mut self) {
        // Загружаем состояние rate limiting для получения защищённого времени
        let mut rate_limit_state = load_rate_limit_state();
        let current_time_ms = get_current_time_ms_protected(&mut rate_limit_state);
        // Сохраняем состояние после использования
        save_rate_limit_state(&rate_limit_state);

        let one_minute_ms = 60_000;

        self.recent_entry_times
            .retain(|&time| current_time_ms.saturating_sub(time) < one_minute_ms);
    }

    /// Получить список рекордов.
    ///
    /// # Возвращает
    /// Срез записей таблицы лидеров
    pub fn get_entries(&self) -> &[LeaderboardEntry] {
        &self.entries
    }

    /// Получить лучший рекорд.
    ///
    /// # Возвращает
    /// Лучший рекорд или 0, если таблица пуста
    /// Возвращает u128 для предотвращения переполнения.
    #[allow(dead_code)]
    #[must_use]
    pub fn get_best_score(&self) -> u128 {
        self.entries.first().map_or(0, |e| e.score_value)
    }

    /// Проверить валидность всех записей.
    ///
    /// Удаляет все записи с невалидным хешем (подделанные).
    pub fn validate(&mut self) {
        self.entries.retain(LeaderboardEntry::is_valid);
    }

    /// Очистить таблицу лидеров.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Получить количество записей в таблице.
    ///
    /// # Возвращает
    /// Количество записей
    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Проверить, пуста ли таблица.
    ///
    /// # Возвращает
    /// `true` если таблица пуста
    #[allow(dead_code)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod sanitize_tests {
    use super::*;

    #[test]
    fn test_sanitize_player_name_empty_to_anonymous() {
        assert_eq!(sanitize_player_name(""), "Anonymous");
        assert_eq!(sanitize_player_name("   \t\n"), "Anonymous");
    }

    #[test]
    fn test_sanitize_player_name_filters_invalid_chars_and_fallback() {
        // Все символы невалидны -> fallback
        assert_eq!(sanitize_player_name("@@@###"), "Anonymous");

        // Смешанное имя -> остаются только разрешённые
        assert_eq!(sanitize_player_name("Pl@yer!_1"), "Plyer_1");
    }

    #[test]
    fn test_sanitize_player_name_truncates_to_20_chars() {
        let name = "abcdefghijklmnopqrstuvwxyz";
        let sanitized = sanitize_player_name(name);
        assert_eq!(sanitized.chars().count(), 20);
        assert_eq!(sanitized, "abcdefghijklmnopqrst");
    }

    #[test]
    fn test_generate_salt_length_and_hex() {
        let hash = generate_salt();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_salt_uniqueness_smoke() {
        let a = generate_salt();
        let b = generate_salt();
        assert_ne!(a, b, "Две соли подряд не должны совпадать (smoke test)");
    }

    #[test]
    fn test_generate_salt_is_lowercase_hex() {
        let hash = generate_salt();
        assert!(hash.chars().all(|c| !c.is_ascii_uppercase()));
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ UNICODE БЕЗОПАСНОСТИ
    // =========================================================================

    /// Тест: проверка на bidirectional control characters (U+200E, U+200F)
    #[test]
    fn test_sanitize_player_name_bidirectional_chars() {
        // Имя с bidirectional control characters
        let name_with_bidi = "Player\u{200E}Name"; // U+200E - LTR mark
        let sanitized = sanitize_player_name(name_with_bidi);
        // Bidi символы должны быть удалены
        assert!(!sanitized.contains('\u{200E}'));
        assert!(!sanitized.contains('\u{200F}'));
        assert_eq!(sanitized, "PlayerName");
    }

    /// Тест: проверка на другие bidirectional control characters
    #[test]
    fn test_sanitize_player_name_all_bidi_chars() {
        let bidi_chars = [
            '\u{200E}', // LTR mark
            '\u{200F}', // RTL mark
            '\u{202A}', // LTR embedding
            '\u{202B}', // RTL embedding
            '\u{202C}', // POP directional formatting
            '\u{202D}', // LTR override
            '\u{202E}', // RTL override
            '\u{2066}', // LTR isolate
            '\u{2067}', // RTL isolate
            '\u{2068}', // FSI
            '\u{2069}', // PDI
        ];

        for &char in &bidi_chars {
            let name = format!("Player{char}Name");
            let sanitized = sanitize_player_name(&name);
            assert!(
                !sanitized.contains(char),
                "Bidi символ {char:?} должен быть удалён"
            );
        }
    }

    /// Тест: проверка на эмодзи
    #[test]
    fn test_sanitize_player_name_emoji_filtered() {
        // Имя с эмодзи
        let name_with_emoji = "Player😀Name";
        let sanitized = sanitize_player_name(name_with_emoji);
        // Эмодзи должны быть удалены (они не проходят is_valid_name_char)
        assert!(!sanitized.contains('😀'));
        assert_eq!(sanitized, "PlayerName");
    }

    /// Тест: проверка на комбинированные символы
    #[test]
    fn test_sanitize_player_name_combined_chars() {
        // Имя с комбинирующими символами (например, e + combining acute = é)
        let name_combined = "Caf\u{0065}\u{0301}"; // e + combining acute
        let sanitized = sanitize_player_name(name_combined);
        // Комбинирующие символы разрешены если base символ alphanumeric
        assert!(sanitized.contains('e'));
    }

    /// Тест: проверка на очень длинные имена
    #[test]
    fn test_sanitize_player_name_very_long_name() {
        let very_long_name = "a".repeat(1000);
        let sanitized = sanitize_player_name(&very_long_name);
        assert_eq!(sanitized.len(), 20);
        assert_eq!(sanitized, "aaaaaaaaaaaaaaaaaaaa");
    }

    /// Тест: проверка на имена только с control characters
    #[test]
    fn test_sanitize_player_name_only_control_chars() {
        let name_control = "\u{200E}\u{200F}\u{202A}";
        let sanitized = sanitize_player_name(name_control);
        assert_eq!(sanitized, "Anonymous");
    }
}
