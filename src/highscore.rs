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
use serde::{Deserialize, Serialize};

// ===========================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ===========================================================================
// Общие функции для генерации хешей и солей
// Используются в SaveData и LeaderboardEntry

/// Сгенерировать случайную соль из 64 шестнадцатеричных символов (256 бит).
///
/// Используется криптографически стойкий генератор случайных чисел (getrandom).
/// Возвращает строку из ровно 64 шестнадцатеричных символов (256 бит).
/// Оптимизация: использует String::with_capacity(64) + write!() вместо format!()
pub fn get_random_hash() -> String {
    use rand::rngs::OsRng;
    use rand::RngCore;
    use std::fmt::Write;

    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
                               // Используем криптографически стойкий генератор случайных чисел ОС.
    OsRng.fill_bytes(&mut bytes);

    // Оптимизация: предварительно выделяем память на 64 символа
    let mut result = String::with_capacity(64);
    for byte in &bytes {
        write!(result, "{:02x}", byte).unwrap();
    }
    result
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

/// Данные для сохранения рекорда.
/// Содержит значение рекорда, соль для хеширования и сам хеш для защиты от подделки.
#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    /// Значение рекорда.
    high_score: u64,
    /// Соль для хэша (защита от подделки).
    high_score_salt: String,
    /// Хэш рекорда с солью.
    high_score_hash: String,
}

/// Запись в таблице лидеров.
/// Представляет собой один результат с именем игрока и защищённым хешем.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardEntry {
    /// Имя игрока.
    name: String,
    /// Значение рекорда.
    score: u64,
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
    pub fn score(&self) -> u64 {
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
                if data.assert_hs() == 0 && data.high_score != 0 {
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
    pub fn from_value(high_score: u64) -> Self {
        let high_score_str = high_score.to_string();
        let salt = get_random_hash();
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
    pub fn save_value(high_score: u64) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, save) {
            eprintln!("Ошибка сохранения рекорда: {}", e);
        }
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
    pub fn assert_hs(&self) -> u64 {
        let high_score_str = self.high_score.to_string();
        let salt_and_hs = self.high_score_salt.clone() + &high_score_str;
        let test_hash = get_hash(&salt_and_hs);

        if self.high_score_hash == test_hash {
            self.high_score
        } else {
            0
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
/// - разрешены только буквы/цифры и символы: '_', '-', ' ', '.'
/// - максимум 20 символов
/// - пустое имя (в т.ч. после фильтрации) заменяется на "Anonymous"
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
        .filter(|&c| is_valid_name_char(c))
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
    pub fn new(name: String, score: u64) -> Self {
        let valid_name = sanitize_player_name(&name);

        let salt = get_random_hash();
        let salt_and_score = format!("{}{}{}", salt, valid_name, score);
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
    pub fn is_valid(&self) -> bool {
        let salt_and_score = format!("{}{}{}", self.salt, self.name, self.score);
        let test_hash = get_hash(&salt_and_score);
        self.hash == test_hash
    }
}

/// Проверить допустимость символа имени.
///
/// Разрешены только:
/// - Буквы (a-z, A-Z, включая кириллицу и другие Unicode буквы)
/// - Цифры (0-9)
/// - Специальные символы: '_', '-', ' ', '.'
///
/// # Аргументы
/// * `c` - символ для проверки
///
/// # Возвращает
/// `true` если символ допустим
fn is_valid_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == ' ' || c == '.'
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
    /// `false` если рекорд недостаточно высок
    pub fn add_score(&mut self, name: String, score: u64) -> bool {
        // Проверка: достаточно ли высок рекорд для попадания в таблицу
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный рекорд
            let min_score = self.entries.iter().map(|e| e.score).min().unwrap_or(0);
            if score <= min_score {
                return false;
            }
        }

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
    #[allow(dead_code)]
    pub fn get_best_score(&self) -> u64 {
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
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Проверить, пуста ли таблица.
    ///
    /// # Возвращает
    /// `true` если таблица пуста
    #[allow(dead_code)]
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
    fn test_get_random_hash_length_and_hex() {
        let hash = get_random_hash();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_get_random_hash_uniqueness_smoke() {
        let a = get_random_hash();
        let b = get_random_hash();
        assert_ne!(a, b, "Две соли подряд не должны совпадать (smoke test)");
    }

    #[test]
    fn test_get_random_hash_is_lowercase_hex() {
        let hash = get_random_hash();
        assert!(hash.chars().all(|c| !c.is_ascii_uppercase()));
    }
}
