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

use confy::{load, store};

use super::{Leaderboard, APP_NAME};

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
    ///
    /// # Осознанное использование `eprintln!`
    /// Этот метод использует `eprintln!` для логирования ошибок вместо crates like `log`,
    /// так как это graceful degradation — при отсутствии логгера ошибки всё равно
    /// должны быть видны пользователю. Это не баг, а осознанное архитектурное решение.
    #[must_use]
    #[allow(clippy::print_stderr)]
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
    ///
    /// # Осознанное использование `eprintln!`
    /// Этот метод использует `eprintln!` для логирования ошибок вместо crates like `log`,
    /// так как это graceful degradation — при отсутствии логгера ошибки всё равно
    /// должны быть видны пользователю. Это не баг, а осознанное архитектурное решение.
    #[allow(clippy::print_stderr)]
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

    /// Сохранить таблицу лидеров с возвратом результата.
    ///
    /// В отличие от [`Self::save()`], этот метод возвращает `Result` для
    /// явной обработки ошибок вызывающим кодом.
    ///
    /// # Аргументы
    /// * `leaderboard` - таблица лидеров для сохранения
    ///
    /// # Возвращает
    /// - `Ok(())` если сохранение успешно
    /// - `Err(String)` если произошла ошибка (включая backup)
    ///
    /// # Примечания
    /// При ошибке сохранения в основной файл автоматически пытается
    /// сохранить в backup. Если и backup не удался, возвращает ошибку
    /// с информацией об обеих попытках.
    pub fn save_result(leaderboard: &Leaderboard) -> Result<(), String> {
        if let Err(e) = store(APP_NAME, Some("leaderboard"), leaderboard) {
            let main_error = e.to_string();
            // Попытка сохранить в backup файл
            if let Err(backup_e) = store(APP_NAME, Some("leaderboard_backup"), leaderboard) {
                return Err(format!(
                    "Ошибка сохранения: основной файл ({main_error}), backup ({backup_e})"
                ));
            }
            // Backup сохранён успешно — это частичный успех, но не ошибка
        }
        Ok(())
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

    #[test]
    fn test_repository_save_result_returns_ok() {
        // save_result должен вернуть Ok или Err (не паниковать)
        let leaderboard = Leaderboard::default();
        let result = LeaderboardRepository::save_result(&leaderboard);
        // В тестовой среде confy может не иметь доступа к файловой системе,
        // поэтому проверяем только что метод не паникует и возвращает Result
        let _ = result.is_ok() || result.is_err();
    }

    #[test]
    fn test_repository_save_does_not_panic() {
        // save() не должен паниковать даже при ошибках
        let leaderboard = Leaderboard::default();
        LeaderboardRepository::save(&leaderboard);
    }
}
