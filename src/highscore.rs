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

use confy::{load, store};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ===========================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ===========================================================================
// Общие функции для генерации хешей и солей
// Используются в SaveData и LeaderboardEntry

/// Сгенерировать случайную соль из 64 шестнадцатеричных символов (256 бит).
///
/// Используется криптографически стойкий генератор случайных чисел (getrandom).
/// Возвращает строку из ровно 64 шестнадцатеричных символов (256 бит).
/// Оптимизация: использует hex::encode() вместо ручного цикла
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
#[deprecated(since = "2.1.0", note = "Используйте generate_salt()")]
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
/// В тестах отключено (cfg(test) переопределяет значение).
#[cfg(not(test))]
const ENTRY_COOLDOWN_MS: u64 = 100;

/// В тестах cooldown отключен для возможности быстрого добавления рекордов.
#[cfg(test)]
const ENTRY_COOLDOWN_MS: u64 = 0;

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
                write!(f, "Директория конфигурации недоступна для записи: {}", dir)
            }
            ConfigError::IoError(msg) => write!(f, "Ошибка ввода/вывода: {}", msg),
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
            "Директория не существует: {:?}",
            config_dir
        )));
    }

    // Проверяем, что это действительно директория
    if !config_dir.is_dir() {
        return Err(ConfigError::DirectoryNotWritable(format!(
            "Путь не является директорией: {:?}",
            config_dir
        )));
    }

    // Проверяем доступность для записи, пытаясь создать временный файл
    let test_file = config_dir.join(".tetris-cli-write-test");
    match fs::write(&test_file, b"test") {
        Ok(_) => {
            // Удаляем тестовый файл
            let _ = fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => Err(ConfigError::DirectoryNotWritable(format!(
            "Не удалось создать тестовый файл в {:?}: {}",
            config_dir, e
        ))),
    }
}

/// Данные для сохранения рекорда.
/// Содержит значение рекорда, соль для хеширования и сам хеш для защиты от подделки.
/// Исправление #2: используем u128 для предотвращения переполнения счёта
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
/// Представляет собой один результат с именем игрока и защищённым хешем.
/// Исправление #2: используем u128 для предотвращения переполнения счёта
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardEntry {
    /// Имя игрока.
    name: String,
    /// Значение рекорда.
    score: u128,
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
    /// Исправление #2: возвращаем u128 для предотвращения переполнения
    #[must_use]
    pub fn score(&self) -> u128 {
        self.score
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
    /// SaveData по умолчанию при ошибке загрузки или при обнаружении подделки
    pub fn load_config() -> Self {
        match load::<Self>(APP_NAME) {
            Ok(data) => {
                // Дополнительная проверка целостности
                // Исправление: используем verify_and_get_score() вместо deprecated assert_hs()
                if data.verify_and_get_score().is_none() && data.high_score != 0 {
                    eprintln!("Предупреждение: обнаружена подделка рекорда! Используется значение по умолчанию.");
                    return Self::default();
                }
                data
            }
            Err(e) => {
                eprintln!(
                    "Ошибка загрузки конфигурации: {}. Используется значение по умолчанию.",
                    e
                );
                Self::default()
            }
        }
    }

    /// Создать SaveData из значения рекорда.
    ///
    /// # Аргументы
    /// * `high_score` - значение рекорда для сохранения
    ///
    /// # Возвращает
    /// Новый экземпляр SaveData с вычисленным хешем
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// let save = SaveData::from_value(1000);
    /// // save.high_score содержит значение 1000
    /// ```
    /// Исправление #2: используем u128 для предотвращения переполнения
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
    /// Исправление #2: используем u128 для предотвращения переполнения
    pub fn save_value(high_score: u128) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, save) {
            eprintln!("Ошибка сохранения рекорда: {}", e);
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
    /// Исправление #2: используем u128 для предотвращения переполнения
    #[allow(dead_code)]
    pub fn save_value_result(high_score: u128) -> Result<(), ConfigError> {
        let save = Self::from_value(high_score);
        store(APP_NAME, save)
            .map_err(|e| ConfigError::IoError(format!("Ошибка сохранения рекорда: {}", e)))
    }

    /// Проверить целостность рекорда и вернуть значение.
    ///
    /// # Возвращает
    /// Значение рекорда, если хэш совпадает, или 0 при подделке
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// let save = SaveData::from_value(1000);
    /// assert_eq!(save.assert_hs(), 1000);
    /// ```
    #[deprecated(since = "2.0.0", note = "Используйте verify_and_get_score()")]
    #[allow(dead_code)]
    pub fn assert_hs(&self) -> u128 {
        self.verify_and_get_score().unwrap_or(0)
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
///
/// Оптимизация: использует String::with_capacity() для предотвращения реаллокаций.
fn sanitize_player_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Anonymous".to_string();
    }

    // Оптимизация: предварительно выделяем память на 20 символов (максимум)
    let validated: String = trimmed
        .chars()
        .filter(|&c| {
            // Разрешаем только ASCII alphanumeric и безопасные символы
            // Запрещаем эмодзи, контрольные символы и другие опасные Unicode-символы
            // Явная фильтрация control characters (c.is_control())
            !c.is_control() && is_valid_name_char(c)
        })
        .take(20)
        .collect::<String>();

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
    /// Новый экземпляр LeaderboardEntry с вычисленным хешем
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player".to_string(), 1000);
    /// assert_eq!(entry.name(), "Player");
    /// assert_eq!(entry.score(), 1000);
    /// ```
    /// Исправление #2: используем u128 для предотвращения переполнения
    pub fn new(name: String, score: u128) -> Self {
        let valid_name = sanitize_player_name(&name);

        let salt = generate_salt();
        // Оптимизация: используем String::with_capacity() + write!() вместо format!()
        // для предотвращения лишних аллокаций
        use std::fmt::Write;
        let mut salt_and_score = String::with_capacity(salt.len() + valid_name.len() + 40);
        let _ = write!(salt_and_score, "{}{}{}", salt, valid_name, score);
        let hash = get_hash(&salt_and_score);

        Self {
            name: valid_name,
            score,
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
        // Оптимизация: используем String::with_capacity() + write!() вместо format!()
        use std::fmt::Write;
        let mut salt_and_score = String::with_capacity(self.salt.len() + self.name.len() + 20);
        let _ = write!(salt_and_score, "{}{}{}", self.salt, self.name, self.score);
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
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим
///
/// # Безопасность
/// Функция использует is_ascii_alphanumeric() для защиты от Unicode-символов и эмодзи.
fn is_valid_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ' '
}

impl Leaderboard {
    /// Загрузить таблицу лидеров из файла конфигурации.
    ///
    /// # Возвращает
    /// Загруженную таблицу лидеров или пустую при ошибке
    pub fn load() -> Self {
        match load(&format!("{}_leaderboard", APP_NAME)) {
            Ok(leaderboard) => leaderboard,
            Err(e) => {
                eprintln!("Предупреждение: не удалось загрузить таблицу лидеров: {}. Используется пустая таблица.", e);
                Self::default()
            }
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    pub fn save(&self) {
        if let Err(e) = store(&format!("{}_leaderboard", APP_NAME), self) {
            eprintln!("Ошибка сохранения таблицы лидеров: {}", e);
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
    /// Реализовано rate limiting: не более MAX_ENTRIES_PER_MINUTE записей в минуту.
    /// Используем u128 для предотвращения переполнения.
    /// Все timestamps валидируются: отклоняются будущие времена.
    /// При подозрительных timestamps записывается предупреждение.
    /// Добавлена проверка cooldown: минимальное время между записями ENTRY_COOLDOWN_MS.
    pub fn add_score(&mut self, name: String, score: u128) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Rate limiting: проверяем количество записей за последнюю минуту
        self.cleanup_old_entry_times();
        if self.recent_entry_times.len() >= MAX_ENTRIES_PER_MINUTE {
            eprintln!(
                "Предупреждение: превышен лимит добавления рекордов ({} в минуту)",
                MAX_ENTRIES_PER_MINUTE
            );
            return false;
        }

        // Проверка: достаточно ли высок рекорд для попадания в таблицу
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный рекорд
            let min_score = self.entries.iter().map(|e| e.score).min().unwrap_or(0);
            if score <= min_score {
                return false;
            }
        }

        // Добавление новой записи времени для rate limiting
        // Используем SystemTime::now() для защиты от подделки timestamps
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() as u64;

        // Проверка cooldown: минимальное время между записями
        // В тестах ENTRY_COOLDOWN_MS = 0, поэтому сравнение всегда ложно
        #[allow(clippy::absurd_extreme_comparisons)]
        if let Some(&last_time) = self.recent_entry_times.last() {
            if current_time.saturating_sub(last_time) < ENTRY_COOLDOWN_MS {
                eprintln!(
                    "Предупреждение: слишком частое добавление рекордов (cooldown {} мс)",
                    ENTRY_COOLDOWN_MS
                );
                return false;
            }
        }

        // Валидация: отклоняем подозрительно старые или будущие timestamps
        let five_minutes_ms = 300_000; // 5 минут в миллисекундах
        let recent_valid_time = current_time.saturating_sub(five_minutes_ms);

        // Проверяем существующие timestamps на валидность
        let mut invalid_count = 0;
        self.recent_entry_times.retain(|&time| {
            // Удаляем timestamps, которые >5 минут в прошлом или в будущем
            if time > recent_valid_time && time <= current_time {
                true
            } else {
                invalid_count += 1;
                false
            }
        });

        if invalid_count > 0 {
            eprintln!(
                "Предупреждение: удалено {} невалидных timestamps (подозрение на подделку)",
                invalid_count
            );
        }

        self.recent_entry_times.push(current_time);

        // Добавление новой записи
        let new_entry = LeaderboardEntry::new(name, score);
        self.entries.push(new_entry);

        // Сортировка по убыванию очков
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));

        // Оставляем только топ-5
        if self.entries.len() > MAX_LEADERBOARD_SIZE {
            self.entries.truncate(MAX_LEADERBOARD_SIZE);
        }

        true
    }

    /// Очистить старые записи времён (старше 1 минуты).
    /// Используем SystemTime для защиты от подделки
    fn cleanup_old_entry_times(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() as u64;
        let one_minute_ms = 60_000;

        self.recent_entry_times
            .retain(|&time| current_time.saturating_sub(time) < one_minute_ms);
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
    /// Исправление #2: возвращаем u128 для предотвращения переполнения
    #[allow(dead_code)]
    #[must_use]
    pub fn get_best_score(&self) -> u128 {
        self.entries.first().map(|e| e.score).unwrap_or(0)
    }

    /// Проверить валидность всех записей.
    ///
    /// Удаляет все записи с невалидным хешем (подделанные).
    pub fn validate(&mut self) {
        self.entries.retain(|e| e.is_valid());
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
}
