//! Модуль сохранения данных рекордов.
//!
//! Предоставляет структуры для хранения одиночного рекорда
//! с защитой от подделки через хэширование с солью.

use crate::crypto::{self, hash};
use confy::{load, store};
use serde::{Deserialize, Serialize};

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

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
                write!(f, "Директория конфигурации недоступна для записи: {dir}")
            }
            ConfigError::IoError(msg) => write!(f, "Ошибка ввода/вывода: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Данные для сохранения рекорда.
/// Содержит значение рекорда, соль для хеширования и сам хеш для защиты от подделки.
/// Использует u128 для предотвращения переполнения счёта.
///
/// # Исправление #16
/// Поля переименованы: `high_score` → `score`, `high_score_salt` → `salt`, `high_score_hash` → `hash`.
#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    /// Значение рекорда.
    score: u128,
    /// Соль для хэша (защита от подделки).
    salt: String,
    /// Хэш рекорда с солью.
    hash: String,
}

impl SaveData {
    /// Загрузить конфигурацию из файла.
    ///
    /// # Возвращает
    /// `SaveData` по умолчанию при ошибке загрузки или при обнаружении подделки
    ///
    /// # Исправление #26
    /// Добавлено предупреждение в UI о проблемах с загрузкой конфигурации.
    pub fn load_config() -> Self {
        match load::<Self>(APP_NAME) {
            Ok(data) => {
                // Дополнительная проверка целостности
                // Исправление: используем [`verify_and_get_score()`] вместо deprecated `assert_hs()`
                match data.verify_and_get_score() {
                    Some(score) => {
                        // Логирование успешной загрузки
                        if score > 0 {
                            eprintln!("Информация: загружен рекорд со значением {score}");
                        }
                        data
                    }
                    None if data.score != 0 => {
                        // Исправление #26: подробное предупреждение о подделке
                        eprintln!("Предупреждение: обнаружена подделка рекорда! Используется значение по умолчанию.");
                        eprintln!("  Если вы не пытались изменить файл конфигурации вручную, это может быть ошибкой.");
                        Self::default()
                    }
                    None => {
                        eprintln!("Предупреждение: рекорд не прошёл валидацию. Используется значение по умолчанию.");
                        Self::default()
                    }
                }
            }
            Err(e) => {
                // Подробное логирование ошибок загрузки
                // Используем [`Display`] trait для форматирования ошибки
                let error_msg = format!("{e}");
                // Исправление #26: добавлено предупреждение в UI
                eprintln!("Ошибка загрузки конфигурации: {error_msg}. Используется значение по умолчанию.");
                eprintln!(
                    "  Проверьте права доступа к файлу конфигурации или запустите игру снова."
                );
                Self::default()
            }
        }
    }

    /// Создать `SaveData` из значения рекорда.
    ///
    /// # Аргументы
    /// * `score` - значение рекорда для сохранения
    ///
    /// # Возвращает
    /// Новый экземпляр `SaveData` с вычисленным хешом
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// let save = SaveData::from_value(1000);
    /// // [`score`] содержит значение 1000
    /// ```
    /// Использует u128 для предотвращения переполнения.
    ///
    /// # Исправление #16
    /// Поля переименованы: используется `score`, `salt`, `hash`.
    pub fn from_value(score: u128) -> Self {
        let score_str = score.to_string();
        let salt = crypto::generate_salt();
        let salt_and_score = salt.clone() + &score_str;
        let hash = hash(&salt_and_score);

        Self { score, salt, hash }
    }

    /// Сохранить значение рекорда в файл.
    ///
    /// # Аргументы
    /// * `high_score` - значение рекорда для сохранения
    ///
    /// # Ошибки
    /// При ошибке сохранения выводит сообщение в stderr
    /// Использует u128 для предотвращения переполнения.
    pub fn save_value(high_score: u128) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, save) {
            eprintln!("Ошибка сохранения рекорда: {e}");
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
    /// # Errors
    /// Возвращает [`ConfigError::IoError`] если произошла ошибка при сохранении
    /// конфигурации через `confy::store()`.
    ///
    /// # Пример
    /// ```no_run
    /// use tetris_cli::highscore::SaveData;
    /// match SaveData::save_value_result(1000) {
    ///     Ok(()) => println!("Рекорд успешно сохранён"),
    ///     Err(e) => eprintln!("Ошибка сохранения: {}", e),
    /// }
    /// ```
    /// Использует u128 для предотвращения переполнения.
    #[allow(dead_code)]
    pub fn save_value_result(high_score: u128) -> Result<(), ConfigError> {
        let save = Self::from_value(high_score);
        store(APP_NAME, save)
            .map_err(|e| ConfigError::IoError(format!("Ошибка сохранения рекорда: {e}")))
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
        // Исправление #16: используем новые имена полей
        let score_str = self.score.to_string();
        let salt_and_score = self.salt.clone() + &score_str;
        let test_hash = hash(&salt_and_score);

        if self.hash == test_hash {
            Some(self.score)
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
