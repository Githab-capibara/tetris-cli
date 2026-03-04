//! Система сохранения рекордов.
//!
//! Автор: Dylan Turner

use confy::{load, store};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Данные для сохранения рекорда.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    /// Значение рекорда.
    high_score: u64,
    /// Соль для хэша (защита от подделки).
    high_score_salt: String,
    /// Хэш рекорда с солью.
    high_score_hash: String,
}

impl SaveData {
    /// Загрузить конфигурацию из файла.
    pub fn load_config() -> Self {
        load(APP_NAME).unwrap_or_default()
    }

    /// Сгенерировать случайный хэш из 20 цифр.
    fn get_random_hash() -> String {
        let mut rng = thread_rng();
        (0..20).map(|_| rng.gen_range(0..10).to_string()).collect()
    }

    /// Получить хэш строки в шестнадцатеричном формате.
    fn get_hash(msg: &str) -> String {
        let mut hasher = DefaultHasher::new();
        msg.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    /// Создать SaveData из значения рекорда.
    pub fn from_value(high_score: u64) -> Self {
        let high_score_str = high_score.to_string();
        let salt = Self::get_random_hash();
        let salt_and_hs = salt.clone() + &high_score_str;
        let hash = Self::get_hash(&salt_and_hs);

        Self {
            high_score,
            high_score_salt: salt,
            high_score_hash: hash,
        }
    }

    /// Сохранить значение рекорда в файл.
    pub fn save_value(high_score: u64) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, save) {
            eprintln!("Ошибка сохранения рекорда: {}", e);
        }
    }

    /// Проверить целостность рекорда и вернуть значение.
    /// Возвращает 0, если хэш не совпадает (подделка).
    pub fn assert_hs(&self) -> u64 {
        let high_score_str = self.high_score.to_string();
        let salt_and_hs = self.high_score_salt.clone() + &high_score_str;
        let test_hash = Self::get_hash(&salt_and_hs);

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
