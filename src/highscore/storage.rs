//! Хранилище и валидатор записей таблицы лидеров.
//!
//! # Ответственность
//! - Хранение списка записей (`Vec<LeaderboardEntry>`)
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
        // Каждый вызов score() выполняет HMAC верификацию — вызываем ровно один раз на запись
        let score = entry.score().unwrap_or(0);

        // Проверяем, войдёт ли запись в топ-5
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный счёт
            if let Some(min_entry) = self.entries.last() {
                let min_score = min_entry.score().unwrap_or(0);
                if min_score >= score {
                    return false; // Не входит в топ-5
                }
            }
        }

        self.entries.push(entry);

        // P3-ID56: Schwartzian transform — score() (HMAC-верификация) вызывается
        // ровно один раз для каждой записи, затем сортировка по предвычисленным значениям.
        let mut scored_entries: Vec<_> = self
            .entries
            .drain(..)
            .map(|e| (e.score().unwrap_or(0), e))
            .collect();
        scored_entries.sort_by_key(|(s, _)| std::cmp::Reverse(*s));
        self.entries = scored_entries.into_iter().map(|(_, e)| e).collect();

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
    ///
    /// ## Исправление проблемы 34
    /// Двойная криптографическая операция (подпись при создании + верификация при добавлении)
    /// — это осознанное решение для безопасности: подпись создаётся при генерации записи,
    /// а верификация выполняется при чтении в `add_entry()` для подтверждения целостности данных.
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        let Some(entry) = LeaderboardEntry::new(name, score) else {
            crate::log_error!("Не удалось создать запись — HMAC подпись вернула ошибку");
            return false;
        };
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
    #[allow(unused_variables)]
    pub fn verify_hash(salt: &str, name: &str, score_value: u128, hash: &str) -> bool {
        // Используем разделители ':' для предотвращения коллизий конкатенации
        let salt_name_score = format!("{salt}:{name}:{score_value}");
        hmac_verify_with_salt(get_leaderboard_hmac_key(), salt, &salt_name_score, hash)
            .unwrap_or_else(|e| {
                crate::log_error!("Ошибка HMAC проверки в storage: {e}");
                false
            })
    }

    /// Вычислить HMAC-SHA256 подпись для данных рекорда с солью и именем.
    ///
    /// # Аргументы
    /// * `salt` - уникальная соль
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// HMAC-SHA256 подпись в hex формате
    ///
    /// # Panics
    /// Паникует при невозможной ошибке инициализации HMAC (теоретически недостижимо
    /// для валидных UTF-8 строк и корректного HMAC ключа).
    #[must_use]
    pub fn compute_signature(salt: &str, name: &str, score: u128) -> String {
        // Используем разделители ':' для предотвращения коллизий конкатенации
        let salt_name_score = format!("{salt}:{name}:{score}");
        hmac_sign_with_salt(get_leaderboard_hmac_key(), salt, &salt_name_score).unwrap_or_else(
            |e| {
                // Недостижимый путь: HMAC-SHA256 для валидных UTF-8 строк всегда успешен
                panic!("Невозможная ошибка HMAC при создании подписи рекорда: {e}");
            },
        )
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
