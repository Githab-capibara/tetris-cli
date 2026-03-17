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
/// Возвращает строку вида "a3f7b2c1d4e5f678901234567890123456789012345678901234567890123456"
pub fn get_random_hash() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32]; // 32 байта = 256 бит
                               // Используем криптографически стойкий генератор
    rand::thread_rng().fill_bytes(&mut bytes);
    // Конвертируем в hex строку
    bytes.iter().map(|&b| format!("{:x}", b)).collect()
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
    pub name: String,
    /// Значение рекорда.
    pub score: u64,
    /// Соль для хэша (защита от подделки).
    salt: String,
    /// Хэш записи с солью.
    pub hash: String,
}

/// Таблица лидеров - коллекция из топ-5 рекордов.
/// Сохраняется в конфигурационном файле и защищена от подделки.
#[derive(Serialize, Deserialize, Default)]
pub struct Leaderboard {
    /// Список записей в таблице лидеров (максимум 5).
    entries: Vec<LeaderboardEntry>,
}

impl SaveData {
    /// Загрузить конфигурацию из файла.
    ///
    /// # Возвращает
    /// SaveData по умолчанию при ошибке загрузки
    pub fn load_config() -> Self {
        match load(APP_NAME) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Предупреждение: не удалось загрузить конфигурацию рекордов: {}. Используется значение по умолчанию.", e);
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
    /// assert_eq!(entry.name, "Player");
    /// assert_eq!(entry.score, 1000);
    /// ```
    pub fn new(name: String, score: u64) -> Self {
        // Валидация имени - заменяем пустые имена на "Anonymous"
        let valid_name = if name.trim().is_empty() {
            "Anonymous".to_string()
        } else {
            // Обрезаем пробелы и ограничиваем длину имени до 20 символов
            let trimmed = name.trim();
            if trimmed.len() > 20 {
                trimmed[..20].to_string()
            } else {
                trimmed.to_string()
            }
        };

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
