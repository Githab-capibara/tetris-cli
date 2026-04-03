//! Репозиторий для сериализации/десериализации таблицы лидеров.
//!
//! # Ответственность
//! - Сериализация в JSON/TOML
//! - Десериализация из JSON/TOML
//! - Сохранение в файл конфигурации
//! - Загрузка из файла конфигурации
//!
//! ## Архитектурные заметки
//! Выделено из `Leaderboard` для соблюдения Single Responsibility Principle.
//! `LeaderboardRepository` инкапсулирует только логику персистентности.
//!
//! Архитектурное улучшение 2026-04-01 (CRITICAL #2): Разделение Large Class leaderboard.rs

#![allow(dead_code)]

use confy::{load, store};

use super::Leaderboard;

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Репозиторий для работы с персистентностью таблицы лидеров.
///
/// Инкапсулирует логику сохранения и загрузки данных.
///
/// ## Архитектурные заметки
/// Выделено из `Leaderboard` для соблюдения Single Responsibility Principle.
/// Отвечает ТОЛЬКО за сериализацию/десериализацию и персистентность.
pub struct LeaderboardRepository;

impl LeaderboardRepository {
    /// Загрузить таблицу лидеров из файла конфигурации.
    ///
    /// # Возвращает
    /// Загруженную таблицу лидеров или пустую при ошибке
    ///
    /// # Примечания
    /// При ошибке загрузки пытается загрузить из backup файла.
    #[must_use]
    pub fn load() -> Leaderboard {
        match load(APP_NAME, Some("leaderboard")) {
            Ok(leaderboard) => leaderboard,
            Err(e) => {
                eprintln!("Предупреждение: не удалось загрузить таблицу лидеров: {e}. Попытка загрузки из backup...");
                // Попытка загрузить из backup файла
                match load(APP_NAME, Some("leaderboard_backup")) {
                    Ok(backup_leaderboard) => {
                        eprintln!("Информация: успешно загружено из backup файла.");
                        backup_leaderboard
                    }
                    Err(backup_e) => {
                        eprintln!("Предупреждение: не удалось загрузить backup: {backup_e}. Используется пустая таблица.");
                        Leaderboard::default()
                    }
                }
            }
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    ///
    /// # Аргументы
    /// * `leaderboard` - таблица лидеров для сохранения
    ///
    /// # Примечания
    /// При ошибке сохранения пытается сохранить в backup файл.
    pub fn save(leaderboard: &Leaderboard) {
        if let Err(e) = store(APP_NAME, Some("leaderboard"), leaderboard) {
            eprintln!("Ошибка сохранения таблицы лидеров: {e}. Попытка сохранения в backup...");
            // Попытка сохранить в backup файл
            if let Err(backup_e) = store(APP_NAME, Some("leaderboard_backup"), leaderboard) {
                eprintln!("Критическая ошибка: не удалось сохранить даже в backup: {backup_e}");
            } else {
                eprintln!("Информация: успешно сохранено в backup файл.");
            }
        }
    }

    /// Сохранить таблицу лидеров с явным указанием backup режима.
    ///
    /// # Аргументы
    /// * `leaderboard` - таблица лидеров для сохранения
    /// * `use_backup` - если true, сохраняет в backup файл
    ///
    /// # Возвращает
    /// - `Ok(())` если сохранение успешно
    /// - `Err(String)` если произошла ошибка сохранения конфигурации
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось сохранить конфигурацию через `confy::store()`
    pub fn save_with_backup(leaderboard: &Leaderboard, use_backup: bool) -> Result<(), String> {
        let config_name = if use_backup {
            "leaderboard_backup"
        } else {
            "leaderboard"
        };

        store(APP_NAME, Some(config_name), leaderboard)
            .map_err(|e| format!("Ошибка сохранения: {e}"))
    }

    /// Очистить файл конфигурации (удалить данные).
    ///
    /// # Примечания
    /// Это не удаляет файл физически, а сохраняет пустую таблицу.
    pub fn clear() {
        let empty = Leaderboard::default();
        Self::save(&empty);
    }

    /// Проверить существование файла конфигурации.
    ///
    /// # Возвращает
    /// `true` если файл существует и может быть загружен
    ///
    /// # Примечания
    /// Пытается загрузить данные для проверки доступности.
    #[must_use]
    pub fn exists() -> bool {
        load::<Leaderboard>(APP_NAME, Some("leaderboard")).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_load_default() {
        // Загрузка должна вернуть пустую таблицу если файл не существует
        let leaderboard = LeaderboardRepository::load();
        // Пустая таблица должна иметь 0 записей
        assert_eq!(leaderboard.get_entries().len(), 0);
    }

    #[test]
    fn test_repository_exists() {
        // exists() может вернуть true или false в зависимости от наличия файла
        // Главное что метод не паникует
        let _ = LeaderboardRepository::exists();
    }
}
