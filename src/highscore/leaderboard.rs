//! Модуль таблицы лидеров.
//!
//! Предоставляет структуры для хранения и управления таблицей лидеров
//! (топ-5 результатов) с защитой от подделки.

use crate::crypto::{self, hash};
use confy::{load, store};
use serde::{Deserialize, Serialize};

use super::sanitize::sanitize_player_name;

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Максимальное количество рекордов в таблице лидеров.
const MAX_LEADERBOARD_SIZE: usize = 5;

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
    /// Значение рекорда (u128) или 0 если валидация не прошла
    ///
    /// # Безопасность
    /// Метод выполняет валидацию и возврат значения атомарно,
    /// предотвращая TOCTOU уязвимость (Time-Of-Check-Time-Of-Use).
    /// Валидация выполняется для того же значения, которое возвращается.
    /// Возвращает u128 для предотвращения переполнения.
    ///
    /// # Исправление TOCTOU
    /// Валидация и возврат значения выполняются атомарно:
    /// сначала сохраняется локальная копия score_value,
    /// затем проверяется хэш именно для этого значения,
    /// и только после этого значение возвращается.
    #[must_use]
    pub fn score(&self) -> u128 {
        let score_value = self.score_value;
        if !self.verify_hash_for_value(score_value) {
            eprintln!("Предупреждение: запись не прошла валидацию!");
            return 0;
        }
        score_value
    }

    /// Проверить хэш для конкретного значения счёта.
    ///
    /// # Аргументы
    /// * `value` - значение счёта для проверки
    ///
    /// # Возвращает
    /// `true` если хэш совпадает для данного значения, `false` если запись была подделана
    ///
    /// # Безопасность
    /// Этот метод позволяет выполнить валидацию для конкретного значения,
    /// что предотвращает TOCTOU уязвимость при использовании в методе `score()`.
    #[must_use]
    fn verify_hash_for_value(&self, value: u128) -> bool {
        let mut salt_and_score = String::with_capacity(self.salt.len() + self.name.len() + 21);
        salt_and_score.push_str(&self.salt);
        salt_and_score.push_str(&self.name);
        salt_and_score.push_str(&value.to_string());
        let test_hash = hash(&salt_and_score);
        self.hash == test_hash
    }

    /// Получить хэш записи.
    ///
    /// # Примечания
    /// Метод используется в тестах для проверки уникальности хэшей.
    #[allow(dead_code)]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Создать новую запись в таблице лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// Новый экземпляр `LeaderboardEntry` с вычисленным хешом
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert_eq!(entry.name(), "Player");
    /// assert_eq!(entry.score(), 1000);
    /// ```
    /// Использует u128 для предотвращения переполнения.
    ///
    /// # Исправление #9
    /// Используется `&str` вместо `String` для предотвращения лишних аллокаций.
    pub fn new(name: &str, score: u128) -> Self {
        let valid_name = sanitize_player_name(name);

        let salt = crypto::generate_salt();
        // Исправление #10: используем push_str() вместо write!() для конкатенации
        // Это упрощает код и устраняет необходимость обработки Result
        let mut salt_and_score = String::with_capacity(salt.len() + valid_name.len() + 21); // 21 = макс. длина u128
        salt_and_score.push_str(&salt);
        salt_and_score.push_str(&valid_name);
        salt_and_score.push_str(&score.to_string());
        let hash = hash(&salt_and_score);

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
    /// # Алгоритм работы
    /// 1. Создаётся буфер для конкатенации: salt + name + score
    /// 2. Вычисляется хэш от конкатенации
    /// 3. Сравнивается с сохранённым хэшем
    ///
    /// # Безопасность
    /// - Защита от подделки: хэш вычисляется с уникальной солью
    /// - TOCTOU: метод не подвержен уязвимости Time-Of-Check-Time-Of-Use,
    ///   так как проверяет хэш для текущего значения score_value
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert!(entry.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // Исправление #10: используем push_str() вместо write!() для конкатенации
        let mut salt_and_score = String::with_capacity(self.salt.len() + self.name.len() + 21); // 21 = макс. длина u128
        salt_and_score.push_str(&self.salt);
        salt_and_score.push_str(&self.name);
        salt_and_score.push_str(&self.score_value.to_string());
        let test_hash = hash(&salt_and_score);
        self.hash == test_hash
    }
}

/// Таблица лидеров - коллекция из топ-5 рекордов.
/// Сохраняется в конфигурационном файле и защищена от подделки.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Leaderboard {
    /// Список записей в таблице лидеров (максимум 5).
    entries: Vec<LeaderboardEntry>,
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
    /// `false` если рекорд недостаточно высок
    ///
    /// # Исправление #24
    /// Добавлена валидация имени игрока перед добавлением в таблицу лидеров.
    ///
    /// # Исправление #23: Rate limiting
    /// Добавлена проверка на максимальное количество записей от одного имени
    /// (максимум 3 записи на одного игрока для предотвращения спама).
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        // Исправление #24: валидация имени игрока
        let valid_name = sanitize_player_name(name);
        if valid_name == "Anonymous" && name.trim() != "Anonymous" {
            eprintln!(
                "Предупреждение: имя игрока не прошло валидацию и было заменено на 'Anonymous'"
            );
        }

        // Исправление #23: Rate limiting - максимум 3 записи на одного игрока
        let entries_from_player = self
            .entries
            .iter()
            .filter(|e| e.name() == valid_name)
            .count();

        if entries_from_player >= 3 {
            eprintln!(
                "Предупреждение: игрок '{}' достиг лимита записей (максимум 3)",
                valid_name
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

        // Добавление новой записи с валидированным именем
        // Исправление #9: передаём &str вместо String
        let new_entry = LeaderboardEntry::new(&valid_name, score);
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

    /// Получить количество записей в таблице.
    ///
    /// # Возвращает
    /// Количество записей
    // TODO: для будущей функциональности
    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Проверить, пуста ли таблица.
    ///
    /// # Возвращает
    /// `true` если таблица пуста
    // TODO: для будущей функциональности
    #[allow(dead_code)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Очистить таблицу лидеров.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
