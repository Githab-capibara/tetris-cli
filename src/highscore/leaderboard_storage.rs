//! Хранилище записей таблицы лидеров.
//!
//! # Ответственность
//! - Хранение списка записей (Vec<LeaderboardEntry>)
//! - Управление размером таблицы (максимум 5 записей)
//! - Сортировка записей по убыванию счёта
//!
//! ## Архитектурные заметки
//! Выделено из `Leaderboard` для соблюдения Single Responsibility Principle.
//! `LeaderboardStorage` инкапсулирует только хранение и базовое управление записями.
//!
//! Архитектурное улучшение 2026-04-01 (CRITICAL #2): Разделение Large Class leaderboard.rs

#![allow(dead_code)]

use super::LeaderboardEntry;

/// Максимальное количество рекордов в таблице лидеров.
const MAX_LEADERBOARD_SIZE: usize = 5;

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
    /// 3. Обрезает до MAX_LEADERBOARD_SIZE
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
            .sort_by(|a, b| b.score().unwrap_or(0).cmp(&a.score().unwrap_or(0)));

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
    /// Этот метод создаёт LeaderboardEntry internally.
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
    fn test_storage_sorting() {
        let mut storage = LeaderboardStorage::new();
        storage.add_score("Player1", 1000);
        storage.add_score("Player2", 2000);
        storage.add_score("Player3", 500);

        // Проверяем что отсортировано по убыванию
        assert_eq!(storage.get_best_score(), 2000);
        assert_eq!(storage.get_entry(0).unwrap().score(), Some(2000));
        assert_eq!(storage.get_entry(1).unwrap().score(), Some(1000));
        assert_eq!(storage.get_entry(2).unwrap().score(), Some(500));
    }

    #[test]
    fn test_storage_max_size() {
        let mut storage = LeaderboardStorage::new();

        // Добавляем 6 записей (максимум 5)
        for i in 1..=6 {
            storage.add_score(&format!("Player{i}"), i * 100);
        }

        // Должно остаться только 5 записей
        assert_eq!(storage.len(), 5);
        // Лучший счёт должен быть 600 (последний добавленный)
        assert_eq!(storage.get_best_score(), 600);
        // Худший счёт 100 должен быть удалён
        assert!(storage.get_entry(5).is_none());
    }

    #[test]
    fn test_storage_clear() {
        let mut storage = LeaderboardStorage::new();
        storage.add_score("Player1", 1000);
        storage.clear();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }
}
