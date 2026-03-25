//! Модуль таблицы лидеров.
//!
//! Предоставляет структуры для хранения и управления таблицей лидеров
//! (топ-5 результатов) с защитой от подделки.

use crate::crypto::{self, hash};
use confy::{load, store};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

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
    /// use tetris_cli::highscore::LeaderboardEntry;
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
        // Оптимизация: используем String::with_capacity() + write!() вместо format!()
        // для предотвращения лишних аллокаций.
        // Используем точную оценку длины числа через ilog10() вместо константы U128_MAX_DIGITS.
        let score_digits = if score > 0 {
            score.ilog10() as usize + 1
        } else {
            1 // Для 0 нужна 1 цифра
        };
        let mut salt_and_score =
            String::with_capacity(salt.len() + valid_name.len() + score_digits);
        // Исправление #4.1: используем unwrap_or_else с логированием вместо let _ =
        write!(salt_and_score, "{salt}{valid_name}{score}").unwrap_or_else(|e| {
            eprintln!("Предупреждение: ошибка записи в строку: {e}");
        });
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
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert!(entry.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // Оптимизация: используем String::with_capacity() + write!() вместо format!().
        // Используем точную оценку длины числа через ilog10().
        let score_digits = if self.score_value > 0 {
            self.score_value.ilog10() as usize + 1
        } else {
            1
        };
        let mut salt_and_score =
            String::with_capacity(self.salt.len() + self.name.len() + score_digits);
        // Исправление #4.1: используем unwrap_or_else с логированием вместо let _ =
        write!(
            salt_and_score,
            "{}{}{}",
            self.salt, self.name, self.score_value
        )
        .unwrap_or_else(|e| {
            eprintln!("Предупреждение: ошибка записи в строку при валидации: {e}");
        });
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
    /// # Исправление #9
    /// Используется `&str` вместо `String` для предотвращения лишних аллокаций.
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        // Исправление #24: валидация имени игрока
        let valid_name = sanitize_player_name(name);
        if valid_name == "Anonymous" && name.trim() != "Anonymous" {
            eprintln!(
                "Предупреждение: имя игрока не прошло валидацию и было заменено на 'Anonymous'"
            );
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
