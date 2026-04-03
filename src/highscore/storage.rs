//! Хранилище и валидатор записей таблицы лидеров.
#![allow(dead_code)]
//!
//! # Ответственность
//! - Хранение списка записей (Vec<LeaderboardEntry>)
//! - Управление размером таблицы (максимум 5 записей)
//! - Сортировка записей по убыванию счёта
//! - HMAC валидация записей
//! - Проверка целостности данных
//!
//! ## Архитектурные заметки
//! Выделено из `Leaderboard` для соблюдения Single Responsibility Principle.
//! Объединяет хранение и валидацию записей.
//!
//! Архитектурное улучшение 2026-04-01 (CRITICAL #2): Разделение Large Class leaderboard.rs

use super::LeaderboardEntry;
use crate::config::keys::get_leaderboard_hmac_key;
use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};

/// Максимальное количество рекордов в таблице лидеров.
/// Переэкспорт из constants.rs для централизации констант (ISSUE-137).
use crate::constants::MAX_LEADERBOARD_ENTRIES as MAX_LEADERBOARD_SIZE;

/// Хранилище записей таблицы лидеров.
///
/// Инкапсулирует хранение и базовое управление записями.
///
/// ## Архитектурные заметки
/// Выделено из `Leaderboard` для соблюдения Single Responsibility Principle.
/// Отвечает ТОЛЬКО за хранение и управление коллекцией записей.
pub struct LeaderboardStorage {
    /// Список записей в таблице лидеров (максимум 5).
    entries: Vec<LeaderboardEntry>,
}

impl Default for LeaderboardStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaderboardStorage {
    /// Создать новое хранилище записей.
    ///
    /// # Возвращает
    /// Новый экземпляр `LeaderboardStorage` с пустым списком записей
    #[must_use = "Хранилище рекордов должно быть использовано"]
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(MAX_LEADERBOARD_SIZE),
        }
    }

    /// Получить список записей.
    ///
    /// # Возвращает
    /// Ссылка на вектор записей
    #[must_use]
    pub fn entries(&self) -> &[LeaderboardEntry] {
        &self.entries
    }

    /// Добавить новую запись в хранилище.
    ///
    /// # Аргументы
    /// * `entry` - запись для добавления
    ///
    /// # Возвращает
    /// `true` если запись была добавлена, `false` если запись не вошла в топ-5
    ///
    /// ## Алгоритм
    /// 1. Добавляет запись в вектор
    /// 2. Сортирует по убыванию счёта
    /// 3. Обрезает до `MAX_LEADERBOARD_SIZE`
    pub fn add_entry(&mut self, entry: LeaderboardEntry) -> bool {
        let score = entry.score().unwrap_or(0);

        // Проверяем, войдёт ли запись в топ-5
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный счёт
            if let Some(min_entry) = self.entries.last() {
                if min_entry.score().unwrap_or(0) >= score {
                    return false; // Не входит в топ-5
                }
            }
        }

        self.entries.push(entry);

        // Сортируем по убыванию счёта
        self.entries
            .sort_by_key(|b| std::cmp::Reverse(b.score().unwrap_or(0)));

        // Обрезаем до максимального размера
        if self.entries.len() > MAX_LEADERBOARD_SIZE {
            self.entries.truncate(MAX_LEADERBOARD_SIZE);
        }

        true
    }

    /// Добавить рекорд напрямую (без валидации).
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// `true` если рекорд был добавлен в таблицу (вошёл в топ-5)
    ///
    /// ## Примечания
    /// Этот метод создаёт `LeaderboardEntry` internally.
    /// Для готовых записей используйте `add_entry()`.
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        let entry = LeaderboardEntry::new(name, score);
        self.add_entry(entry)
    }

    /// Получить лучший рекорд.
    ///
    /// # Возвращает
    /// Лучший рекорд или 0 если таблица пуста
    #[must_use]
    pub fn get_best_score(&self) -> u128 {
        self.entries
            .first()
            .and_then(super::leaderboard::LeaderboardEntry::score)
            .unwrap_or(0)
    }

    /// Получить количество записей.
    ///
    /// # Возвращает
    /// Количество записей в таблице
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Проверить, пуста ли таблица.
    ///
    /// # Возвращает
    /// `true` если таблица пуста
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Очистить таблицу лидеров.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Получить запись по индексу.
    ///
    /// # Аргументы
    /// * `index` - индекс записи
    ///
    /// # Возвращает
    /// Some(&LeaderboardEntry) если запись существует, None иначе
    #[must_use]
    pub fn get_entry(&self, index: usize) -> Option<&LeaderboardEntry> {
        self.entries.get(index)
    }

    /// Удалить запись по индексу.
    ///
    /// # Аргументы
    /// * `index` - индекс записи для удаления
    ///
    /// # Возвращает
    /// Some(LeaderboardEntry) если запись существовала, None иначе
    pub fn remove_entry(&mut self, index: usize) -> Option<LeaderboardEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }
}

// ============================================================================
// ВАЛИДАТОР (объединено из leaderboard_validator.rs)
// ============================================================================

/// Валидатор записей таблицы лидеров.
///
/// Инкапсулирует логику HMAC валидации и проверки целостности.
///
/// ## Архитектурные заметки
/// Выделено из `LeaderboardEntry` для соблюдения Single Responsibility Principle.
/// Отвечает ТОЛЬКО за валидацию записей.
pub struct LeaderboardValidator;

impl LeaderboardValidator {
    /// Проверить хэш для конкретного значения счёта.
    ///
    /// # Аргументы
    /// * `salt` - соль для хэша
    /// * `name` - имя игрока
    /// * `score_value` - значение рекорда
    /// * `hash` - хэш для проверки
    ///
    /// # Возвращает
    /// `true` если хэш совпадает для данного значения, `false` если запись была подделана
    ///
    /// # Безопасность
    /// Этот метод позволяет выполнить валидацию для конкретного значения,
    /// что предотвращает TOCTOU уязвимость.
    #[must_use]
    pub fn verify_hash(salt: &str, name: &str, score_value: u128, hash: &str) -> bool {
        let salt_name_score = format!("{salt}{name}{score_value}");
        hmac_verify_with_salt(get_leaderboard_hmac_key(), salt, &salt_name_score, hash)
    }

    /// Вычислить HMAC подпись для записи.
    ///
    /// # Аргументы
    /// * `salt` - соль для хэша
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// HMAC подпись в виде hex строки
    ///
    /// # Безопасность
    /// Использует keyed hash с уникальной солью для защиты от подделки.
    #[must_use]
    pub fn compute_signature(salt: &str, name: &str, score: u128) -> String {
        let salt_name_score = format!("{salt}{name}{score}");
        hmac_sign_with_salt(get_leaderboard_hmac_key(), salt, &salt_name_score)
    }

    /// Проверить целостность записи.
    ///
    /// # Аргументы
    /// * `salt` - соль для хэша
    /// * `name` - имя игрока
    /// * `score_value` - значение рекорда
    /// * `hash` - хэш для проверки
    ///
    /// # Возвращает
    /// `true` если запись валидна, `false` если хэш не совпадает
    ///
    /// # Алгоритм работы
    /// 1. Создаётся буфер для конкатенации: salt + name + score
    /// 2. Вычисляется хэш от конкатенации
    /// 3. Сравнивается с сохранённым хэшем
    #[must_use]
    pub fn is_valid(salt: &str, name: &str, score_value: u128, hash: &str) -> bool {
        Self::verify_hash(salt, name, score_value, hash)
    }

    /// Создать новую валидированную запись.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// Кортеж (salt, hash) для новой записи
    ///
    /// # Безопасность
    /// Генерирует уникальную соль для каждой записи.
    #[must_use = "Результат создания записи должен быть использован"]
    pub fn create_validated_entry(name: &str, score: u128) -> (String, String) {
        let salt = crate::crypto::generate_salt();
        let hash = Self::compute_signature(&salt, name, score);
        (salt, hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_new() {
        let storage = LeaderboardStorage::new();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_storage_add_score() {
        let mut storage = LeaderboardStorage::new();
        assert!(storage.add_score("Player1", 1000));
        assert_eq!(storage.len(), 1);
        assert_eq!(storage.get_best_score(), 1000);
    }

    #[test]
    fn test_validator_compute_signature() {
        let salt = "test_salt";
        let name = "Player";
        let score = 1000u128;

        let hash = LeaderboardValidator::compute_signature(salt, name, score);
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA256 hex = 64 символа
    }

    #[test]
    fn test_validator_verify_hash() {
        let salt = "test_salt";
        let name = "Player";
        let score = 1000u128;

        let hash = LeaderboardValidator::compute_signature(salt, name, score);
        assert!(LeaderboardValidator::verify_hash(salt, name, score, &hash));
    }
}
