//! Модуль сохранения данных рекордов.
//!
//! Предоставляет структуры для хранения одиночного рекорда
//! с защитой от подделки через хэширование с солью.
//!
//! # Исправление #3 (CRITICAL)
//! HMAC логика перемещена в модуль `crypto::hmac`.

use crate::config::keys::get_save_data_hmac_key;
use crate::constants::MAX_CONFIG_FILE_SIZE;
use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};
use confy::{load, store};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Получить путь к файлу конфигурации confy.
///
/// # Возвращает
/// Путь к файлу конфигурации или None при ошибке
fn get_config_file_path() -> Option<PathBuf> {
    // confy 0.6 сам управляет путями к конфигурации
    // Используем стандартный подход с переменной окружения HOME
    let home_dir = std::env::var("HOME").ok()?;
    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".config");
    config_path.push(APP_NAME);
    config_path.push("config.toml");

    Some(config_path)
}

/// Проверить размер файла конфигурации.
///
/// # Аргументы
/// * `path` - путь к файлу для проверки
///
/// # Возвращает
/// - `Ok(())` если размер файла в пределах нормы
/// - `Err(String)` если файл слишком большой или не существует
fn check_config_file_size(path: &PathBuf) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            let file_size = metadata.len();
            if file_size > MAX_CONFIG_FILE_SIZE {
                return Err(format!(
                    "Файл конфигурации слишком большой: {} байт (максимум {} байт)",
                    file_size, MAX_CONFIG_FILE_SIZE
                ));
            }
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Файл не существует — это нормально, будет создан новый
            Ok(())
        }
        Err(e) => Err(format!("Ошибка проверки файла: {e}")),
    }
}

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
    ///
    /// # Исправление #23
    /// Добавлена проверка размера файла перед загрузкой для защиты от атак через большие файлы.
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлена обработка ошибок с попыткой загрузки из backup файла.
    pub fn load_config() -> Self {
        Self::load_config_result().unwrap_or_else(|e| {
            eprintln!("Предупреждение: {e}. Попытка загрузки из backup...");
            // Попытка загрузить из backup файла
            match Self::load_backup_config() {
                Ok(backup_data) => {
                    eprintln!("Информация: успешно загружено из backup файла.");
                    backup_data
                }
                Err(backup_e) => {
                    eprintln!("Предупреждение: не удалось загрузить backup: {backup_e}. Используется значение по умолчанию.");
                    Self::default()
                }
            }
        })
    }

    /// Загрузить конфигурацию из backup файла.
    ///
    /// # Возвращает
    /// - `Ok(SaveData)` - успешно загруженные данные из backup
    /// - `Err(String)` - ошибка загрузки
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлен метод для загрузки из backup файла.
    fn load_backup_config() -> Result<Self, String> {
        let data: Self = load::<Self>(APP_NAME, Some("config_backup"))
            .map_err(|e| format!("Ошибка загрузки backup конфигурации: {e}"))?;

        // Проверка целостности backup данных
        match data.verify_and_get_score() {
            Some(score) => {
                if score > 0 {
                    eprintln!("Информация: загружен backup рекорд со значением {score}");
                }
                Ok(data)
            }
            None if data.score != 0 => Err("Backup: обнаружена подделка рекорда".to_string()),
            None => Err("Backup: рекорд не прошёл валидацию".to_string()),
        }
    }

    /// Загрузить конфигурацию из файла с возвратом Result.
    ///
    /// # Возвращает
    /// - `Ok(SaveData)` - успешно загруженные данные
    /// - `Err(String)` - ошибка загрузки
    ///
    /// # Примечания
    /// Этот метод использует единый стиль обработки ошибок с Result.
    fn load_config_result() -> Result<Self, String> {
        // Проверка размера файла перед загрузкой (Исправление #23)
        if let Some(config_path) = get_config_file_path() {
            check_config_file_size(&config_path)?;
        }

        let data: Self = load::<Self>(APP_NAME, Some("config"))
            .map_err(|e| format!("Ошибка загрузки конфигурации: {e}"))?;

        // Дополнительная проверка целостности
        match data.verify_and_get_score() {
            Some(score) => {
                // Логирование успешной загрузки
                if score > 0 {
                    eprintln!("Информация: загружен рекорд со значением {score}");
                }
                Ok(data)
            }
            None if data.score != 0 => Err("Обнаружена подделка рекорда".to_string()),
            None => Err("Рекорд не прошёл валидацию".to_string()),
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
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    pub fn from_value(score: u128) -> Self {
        let score_str = score.to_string();
        let salt = crate::crypto::generate_salt();
        let hash = hmac_sign_with_salt(get_save_data_hmac_key(), &salt, &score_str);

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
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлена обработка ошибок с сохранением backup файла при неудаче.
    pub fn save_value(high_score: u128) {
        let save = Self::from_value(high_score);
        if let Err(e) = store(APP_NAME, Some("config"), save.clone()) {
            eprintln!("Ошибка сохранения рекорда: {e}. Попытка сохранения в backup...");
            // Попытка сохранить в backup файл
            if let Err(backup_e) = store(APP_NAME, Some("config_backup"), save) {
                eprintln!("Критическая ошибка: не удалось сохранить даже в backup: {backup_e}");
            } else {
                eprintln!("Информация: успешно сохранено в backup файл.");
            }
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
    ///
    /// # Note
    /// Этот метод используется только в тестах
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn save_value_result(high_score: u128) -> Result<(), ConfigError> {
        let save = Self::from_value(high_score);
        store(APP_NAME, Some("config"), save)
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
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    pub fn verify_and_get_score(&self) -> Option<u128> {
        let score_str = self.score.to_string();
        if hmac_verify_with_salt(get_save_data_hmac_key(), &self.salt, &score_str, &self.hash) {
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

// ============================================================================
// ТЕСТЫ ДЛЯ ПРОВЕРКИ РАЗМЕРА ФАЙЛА
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    /// Тест для check_config_file_size() с файлом больше 1MB
    /// Проверяет, что функция возвращает ошибку для файла > 1MB
    #[test]
    fn test_check_config_file_size_too_large() {
        // Создаём временный файл
        let test_path = PathBuf::from("test_config_large.toml");

        // Создаём файл размером больше 1MB (1MB + 1 байт)
        let large_size = (MAX_CONFIG_FILE_SIZE + 1) as usize;
        let mut file = File::create(&test_path).expect("Не удалось создать тестовый файл");

        // Записываем данные размером больше 1MB
        let data = vec![b'a'; large_size];
        file.write_all(&data)
            .expect("Не удалось записать данные в файл");
        drop(file);

        // Проверяем, что функция возвращает ошибку
        let result = check_config_file_size(&test_path);
        assert!(
            result.is_err(),
            "Проверка файла > 1MB должна вернуть ошибку"
        );

        let error_msg = result.unwrap_err();
        assert!(
            error_msg.contains("слишком большим") || error_msg.contains("максимум"),
            "Ошибка должна упоминать превышение размера"
        );

        // Очищаем тестовый файл
        let _ = fs::remove_file(&test_path);
    }

    /// Тест для check_config_file_size() с файлом меньше 1MB
    /// Проверяет, что функция возвращает Ok для файла < 1MB
    #[test]
    fn test_check_config_file_size_ok() {
        // Создаём временный файл
        let test_path = PathBuf::from("test_config_small.toml");

        // Создаём небольшой файл (1KB)
        let small_size = 1024;
        let mut file = File::create(&test_path).expect("Не удалось создать тестовый файл");

        // Записываем небольшие данные
        let data = vec![b'b'; small_size];
        file.write_all(&data)
            .expect("Не удалось записать данные в файл");
        drop(file);

        // Проверяем, что функция возвращает Ok
        let result = check_config_file_size(&test_path);
        assert!(result.is_ok(), "Проверка файла < 1MB должна вернуть Ok");

        // Очищаем тестовый файл
        let _ = fs::remove_file(&test_path);
    }

    /// Тест для check_config_file_size() с несуществующим файлом
    /// Проверяет, что функция возвращает Ok для несуществующего файла
    #[test]
    fn test_check_config_file_size_not_found() {
        // Путь к несуществующему файлу
        let test_path = PathBuf::from("test_config_nonexistent.toml");

        // Убеждаемся, что файл не существует
        let _ = fs::remove_file(&test_path);

        // Проверяем, что функция возвращает Ok (файл будет создан)
        let result = check_config_file_size(&test_path);
        assert!(
            result.is_ok(),
            "Проверка несуществующего файла должна вернуть Ok"
        );
    }
}
