//! Валидатор записей таблицы лидеров.
//!
//! # Ответственность
//! - HMAC валидация записей
//! - Проверка целостности данных
//! - Генерация и проверка подписей
//!
//! ## Архитектурные заметки
//! Выделено из `LeaderboardEntry` для соблюдения Single Responsibility Principle.
//! `LeaderboardValidator` инкапсулирует только логику валидации.
//!
//! Архитектурное улучшение 2026-04-01 (CRITICAL #2): Разделение Large Class leaderboard.rs

#![allow(dead_code)]

use crate::config::keys::get_leaderboard_hmac_key;
use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};

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

    #[test]
    fn test_validator_invalid_hash() {
        let salt = "test_salt";
        let name = "Player";
        let score = 1000u128;
        let wrong_hash = "invalid_hash";

        assert!(!LeaderboardValidator::verify_hash(
            salt, name, score, wrong_hash
        ));
    }

    #[test]
    fn test_validator_create_entry() {
        let name = "Player";
        let score = 1000u128;

        let (salt, hash) = LeaderboardValidator::create_validated_entry(name, score);
        assert!(!salt.is_empty());
        assert!(!hash.is_empty());
        assert!(LeaderboardValidator::verify_hash(&salt, name, score, &hash));
    }
}
