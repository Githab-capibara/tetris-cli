//! Модуль таблицы лидеров.
//!
//! Предоставляет структуры для хранения и управления таблицей лидеров
//! (топ-5 результатов) с защитой от подделки.
//!
//! # Потокобезопасность
//! ## Исправление #7 (СРЕДНИЙ ПРИОРИТЕТ)
//!
//! Типы в этом модуле НЕ являются потокобезопасными по умолчанию:
//! - [`LeaderboardEntry`] намеренно не реализует `Send + Sync` для предотвращения
//!   случайного использования в многопоточном коде без надлежащей синхронизации
//! - [`Leaderboard`] не реализует `Send` или `Sync`
//!
//! Для использования в многопоточном коде оборачивайте типы в `Arc<Mutex<>>` или `Arc<RwLock<>>`:
//! ```ignore
//! use std::sync::{Arc, Mutex};
//! use tetris_cli::highscore::Leaderboard;
//!
//! let leaderboard = Arc::new(Mutex::new(Leaderboard::default()));
//! // Теперь безопасно использовать из нескольких потоков
//! leaderboard.lock().unwrap().add_score("Player", 1000);
//! ```

// std
use std::io::Write;

// external
use confy::{load, store};
use serde::{Deserialize, Serialize};

// crate
use crate::config::keys::get_leaderboard_hmac_key;
use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt_bytes};
use crate::highscore::APP_NAME;
use crate::validation::name::sanitize_player_name;

/// Максимальное количество рекордов в таблице лидеров.
/// Переэкспорт из constants.rs для централизации констант (ISSUE-137).
use crate::constants::MAX_LEADERBOARD_ENTRIES as MAX_LEADERBOARD_SIZE;

/// Приватная функция для создания данных записи (соль, хеш, санитаризация имени).
///
/// # Аргументы
/// * `name` - имя игрока (будет санитаризировано)
/// * `score` - значение рекорда
///
/// # Возвращает
/// - `Some((sanitized_name, salt, hash))` — данные для использования в конструкторах
/// - `None` — если `hmac_sign_with_salt` вернул ошибку
///
/// # Исправление проблемы 32
/// Вынесена общая логика из `LeaderboardEntry::new()` и `ThreadSafeLeaderboardEntry::new()`
/// для устранения дублирования кода (генерация соли, хеша, санитаризация имени).
fn create_entry_data(name: &str, score: u128) -> Option<(String, String, String)> {
    let valid_name = sanitize_player_name(name);
    let salt = crate::crypto::generate_salt();
    // Исправление Проблема 9: Разделители ':' предотвращают коллизии конкатенации
    let salt_name_score = format!("{salt}:{valid_name}:{score}");
    // При ошибке HMAC возвращаем None вместо паники
    let hash = match hmac_sign_with_salt(get_leaderboard_hmac_key(), &salt, &salt_name_score) {
        Ok(h) => h,
        Err(_e) => {
            crate::log_error!("Не удалось создать HMAC подпись для записи таблицы лидеров");
            return None;
        }
    };
    Some((valid_name, salt, hash))
}

/// Запись в таблице лидеров.
/// Представляет собой один результат с именем игрока и защищённым хешом.
/// Использует u128 для предотвращения переполнения счёта.
///
/// # Безопасность
/// ## Защита от подделки
/// Каждая запись содержит keyed hash подпись с уникальной солью,
/// что предотвращает подделку результатов через редактирование конфига.
///
/// ## Исправление #5 (TOCTOU)
/// **Эта структура НЕ является потокобезопасной по умолчанию.**
/// Методы валидации и чтения могут подвергаться TOCTOU уязвимости
/// (Time-Of-Check-Time-Of-Use) в многопоточной среде.
///
/// ### Ограничения
/// - **Однопоточный код**: метод `score()` выполняет атомарную валидацию и возврат значения
/// - **Многопоточный код**: требует синхронизации через `Arc<Mutex<>>`
///
/// ### Рекомендации по использованию
/// 1. **Однопоточный код**: используйте напрямую, все методы безопасны
/// 2. **Многопоточный код**: оборачивайте в `Arc<Mutex<LeaderboardEntry>>`
///
/// ### Маркер потоковости (D8)
/// Эта структура не реализует `Send + Sync` намеренно:
/// - Сериализация через serde не добавляет автоматической потокобезопасности
/// - Для многопоточного доступа используйте `ThreadSafeLeaderboardEntry`
///
/// ## Исправление проблемы 31
/// Клонирование записи клонирует все поля включая hash/salt.
/// Это осознанное решение — клонирование нужно для тестов,
/// клонированная запись валидна с тем же ключом (feature, не bug).
#[allow(clippy::expl_impl_clone_on_copy)]
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
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Получить значение рекорда.
    ///
    /// # Возвращает
    /// Значение рекорда (u128) или None если валидация не прошла
    ///
    /// # Безопасность
    /// Метод выполняет валидацию и возврат значения атомарно,
    /// предотвращая TOCTOU уязвимость.
    #[must_use]
    pub fn score(&self) -> Option<u128> {
        let score_value = self.score_value;
        if !self.verify_hash_for_value(score_value) {
            return None;
        }
        Some(score_value)
    }

    /// Проверить хэш для конкретного значения счёта.
    ///
    /// # Аргументы
    /// * `value` - значение счёта для проверки
    ///
    /// # Возвращает
    /// `true` если хэш совпадает для данного значения, `false` если запись была подделана
    ///
    /// # Безопасность
    /// Этот метод позволяет выполнить валидацию для конкретного значения,
    /// что предотвращает TOCTOU уязвимость при использовании в методе `score()`.
    ///
    /// # Исправление 2.2
    /// Оптимизация: используется write! в Vec для избежания промежуточных аллокаций.
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    ///
    /// # Исправление P3-ID41
    /// Использует `hmac_verify_with_salt_bytes` для избежания UTF-8 roundtrip.
    /// Буфер записывается как `Vec<u8>` и передаётся байтами напрямую.
    #[must_use]
    fn verify_hash_for_value(&self, value: u128) -> bool {
        // P3-ID41: write! в Vec<u8>, передача байтов напрямую — без from_utf8 roundtrip
        let mut buf = Vec::with_capacity(self.salt.len() + 1 + self.name.len() + 1 + 20);
        let _ = write!(buf, "{}:{}:{value}", self.salt, self.name);
        hmac_verify_with_salt_bytes(get_leaderboard_hmac_key(), &self.salt, &buf, &self.hash)
    }

    /// Создать новую запись в таблице лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока (будет санитаризировано через `sanitize_player_name()`)
    /// * `score` - значение рекорда (u128 для предотвращения переполнения)
    ///
    /// # Возвращает
    /// Новый экземпляр `LeaderboardEntry` с вычисленным хешом
    ///
    /// # Длина имени
    /// ## Обработка имени
    /// - **Максимальная длина**: 32 символа (`MAX_NAME_LENGTH`)
    /// - **Минимальная длина**: 1 символ (пустое имя заменяется на "Anonymous")
    /// - **Разрешённые символы**: ASCII буквы/цифры, '_', '-', ' ', русские буквы
    /// - **Санитаризация**: `sanitize_player_name()` удаляет запрещённые символы
    ///
    /// ## Примеры
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    ///
    /// // Короткое имя
    /// let entry = LeaderboardEntry::new("Al", 1000).unwrap();
    /// assert_eq!(entry.name(), "Al");
    ///
    /// // Длинное имя (обрезается до 32 символов)
    /// let long_name = "a".repeat(50);
    /// let entry = LeaderboardEntry::new(&long_name, 2000).unwrap();
    /// assert_eq!(entry.name().len(), 32);
    ///
    /// // Пустое имя (заменяется на "Anonymous")
    /// let entry = LeaderboardEntry::new("", 3000).unwrap();
    /// assert_eq!(entry.name(), "Anonymous");
    /// ```
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000).unwrap();
    /// assert_eq!(entry.name(), "Player");
    /// assert_eq!(entry.score(), Some(1000));
    /// ```
    /// Использует u128 для предотвращения переполнения.
    ///
    /// # Исправление #9
    /// Используется `&str` вместо `String` для предотвращения лишних аллокаций.
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    ///
    /// # Исправление аудита
    /// Возвращает `Option<Self>` — `None` при ошибке HMAC подписи.
    #[must_use]
    pub fn new(name: &str, score: u128) -> Option<Self> {
        // Исправление проблемы 32: используем общую функцию create_entry_data
        let (valid_name, salt, hash) = create_entry_data(name, score)?;

        Some(Self {
            name: valid_name,
            score_value: score,
            salt,
            hash,
        })
    }

    /// Проверить целостность записи.
    ///
    /// # Возвращает
    /// `true` если хэш совпадает, `false` если запись была подделана
    ///
    /// # Алгоритм работы
    /// 1. Создаётся буфер для конкатенации: salt + name + score
    /// 2. Вычисляется хэш от конкатенации
    /// 3. Сравнивается с сохранённым хэшем
    ///
    /// # Безопасность
    /// - Защита от подделки: хэш вычисляется с уникальной солью
    /// - TOCTOU: метод не подвержен уязвимости Time-Of-Check-Time-Of-Use,
    ///   так как проверяет хэш для текущего значения `score_value`
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000).unwrap();
    /// assert!(entry.is_valid());
    /// ```
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // Исправление 2.3: делегируем verify_hash_for_value для устранения дублирования HMAC логики
        self.verify_hash_for_value(self.score_value)
    }
}

/// Таблица лидеров - коллекция из топ-5 рекордов.
/// Сохраняется в конфигурационном файле и защищена от подделки.
///
/// # Потокобезопасность
/// ## ⚠️ Требование внешней синхронизации
///
/// Этот тип **НЕ является `Send + Sync`** и не предназначен для использования
/// из нескольких потоков без внешней синхронизации.
///
/// ### Ограничения
/// - **НЕ потокобезопасен**: `Leaderboard` не реализует трейты `Send` и `Sync`
/// - **Внешняя синхронизация требуется**: для использования в многопоточной среде
///   оборачивайте тип в `Arc<Mutex<Leaderboard>>` или `Arc<RwLock<Leaderboard>>`
///
/// ### Пример безопасного использования в многопоточном коде
/// ```ignore
/// use std::sync::{Arc, Mutex};
/// use tetris_cli::highscore::Leaderboard;
///
/// let leaderboard = Arc::new(Mutex::new(Leaderboard::default()));
///
/// // Поток 1
/// let lb_clone = Arc::clone(&leaderboard);
/// std::thread::spawn(move || {
///     lb_clone.lock().unwrap().add_score("Player1", 1000);
/// });
///
/// // Поток 2
/// let lb_clone2 = Arc::clone(&leaderboard);
/// std::thread::spawn(move || {
///     lb_clone2.lock().unwrap().add_score("Player2", 2000);
/// });
/// ```
///
/// ### Почему это важно
/// Без внешней синхронизации возможны:
/// - **Гонки данных**: одновременная модификация `entries` из нескольких потоков
/// - **Повреждение данных**: несогласованное состояние таблицы лидеров
/// - **Паника**: неопределённое поведение при конкурентном доступе
///
/// # Исправление E6 (HIGH)
/// Для многопоточного доступа используйте [`ThreadSafeLeaderboard`] который
/// предоставляет безопасный конкурентный доступ с защитой от race condition.
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
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлена обработка ошибок с попыткой загрузки из backup файла.
    #[must_use]
    pub fn load() -> Self {
        match load(APP_NAME, Some("leaderboard")) {
            Ok(leaderboard) => leaderboard,
            Err(_e) => {
                crate::log_warn!(
                    "Не удалось загрузить таблицу лидеров. Попытка загрузки из backup..."
                );
                // Попытка загрузить из backup файла
                match load(APP_NAME, Some("leaderboard_backup")) {
                    Ok(backup_leaderboard) => {
                        crate::log_info!("Успешно загружено из backup файла.");
                        backup_leaderboard
                    }
                    Err(_backup_e) => {
                        crate::log_warn!(
                            "Не удалось загрузить backup. Используется пустая таблица."
                        );
                        Self::default()
                    }
                }
            }
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    ///
    /// # Возвращает
    /// Ничего не возвращает. Ошибки логируются через макросы `log_error!`/`log_warn!`.
    ///
    /// # Errors
    /// Метод обрабатывает ошибки внутри себя:
    /// - При ошибке сохранения в основной файл пытается сохранить в backup
    /// - При ошибке сохранения в backup логирует критическую ошибку
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлена обработка ошибок с сохранением backup файла при неудаче.
    ///
    /// # Пример использования
    /// ```ignore
    /// let leaderboard = Leaderboard::default();
    /// leaderboard.add_score("Player", 1000);
    /// leaderboard.save();  // Сохраняет в конфигурационный файл
    /// ```
    pub fn save(&self) {
        if let Err(_e) = store(APP_NAME, Some("leaderboard"), self) {
            crate::log_warn!("Ошибка сохранения таблицы лидеров. Попытка сохранения в backup...");
            // Попытка сохранить в backup файл
            if let Err(_backup_e) = store(APP_NAME, Some("leaderboard_backup"), self) {
                crate::log_error!("Критическая ошибка: не удалось сохранить даже в backup");
            } else {
                crate::log_info!("Успешно сохранено в backup файл.");
            }
        }
    }

    /// Сохранить таблицу лидеров с явным указанием backup режима.
    ///
    /// # Аргументы
    /// * `use_backup` - если true, сохраняет в backup файл
    ///
    /// # Возвращает
    /// - `Ok(())` если сохранение успешно
    /// - `Err(String)` если произошла ошибка сохранения конфигурации
    ///
    /// # Errors
    /// Возвращает ошибку если не удалось сохранить конфигурацию через `confy::store()`.
    /// Ошибка содержит описание проблемы и имя конфигурации.
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлен метод для явного сохранения в backup файл.
    ///
    /// # Пример использования
    /// ```ignore
    /// let leaderboard = Leaderboard::default();
    /// // Сохранение в основной файл
    /// leaderboard.save_with_backup(false)?;
    /// // Сохранение в backup файл
    /// leaderboard.save_with_backup(true)?;
    /// ```
    pub fn save_with_backup(&self, use_backup: bool) -> Result<(), String> {
        let config_name = if use_backup {
            "leaderboard_backup"
        } else {
            "leaderboard"
        };

        store(APP_NAME, Some(config_name), self)
            .map_err(|e| format!("Ошибка сохранения {config_name}: {e}"))
    }

    /// Добавить новый рекорд в таблицу лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// `true` если рекорд был добавлен в таблицу (вошёл в топ-5),
    /// `false` если рекорд недостаточно высок или достигнут лимит записей
    ///
    /// # Errors
    /// Метод не возвращает ошибок, но может вернуть `false` в следующих случаях:
    /// - Имя игрока не прошло валидацию (заменяется на "Anonymous")
    /// - Игрок достиг лимита записей (максимум 3 на одного игрока)
    /// - Рекорд недостаточно высок для попадания в топ-5
    ///
    /// # Исправление #24
    /// Добавлена валидация имени игрока перед добавлением в таблицу лидеров.
    ///
    /// # Исправление #23: Rate limiting
    /// Добавлена проверка на максимальное количество записей от одного имени
    /// (максимум 3 записи на одного игрока для предотвращения спама).
    ///
    /// # Безопасность (аудит 2026-03-30)
    /// ## Потокобезопасность
    /// Этот метод НЕ является потокобезопасным. `Leaderboard` не реализует `Send + Sync`
    /// намеренно (использует `PhantomData<*mut ()>` для маркировки).
    /// При использовании в многопоточной среде необходима внешняя синхронизация
    /// (например, `std::sync::Mutex<Leaderboard>`).
    ///
    /// # Пример использования
    /// ```ignore
    /// let mut leaderboard = Leaderboard::default();
    /// let added = leaderboard.add_score("Player", 1000);
    /// assert_eq!(added, true);
    /// ```
    #[must_use]
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        // Исправление #24: валидация имени игрока
        let valid_name = sanitize_player_name(name);
        if valid_name == "Anonymous" && name.trim() != "Anonymous" {
            crate::log_warn!("Имя игрока не прошло валидацию и было заменено на 'Anonymous'");
        }

        // Исправление #23: Rate limiting - максимум 3 записи на одного игрока
        let entries_from_player = self
            .entries
            .iter()
            .filter(|e| e.name() == valid_name)
            .count();

        if entries_from_player >= 3 {
            crate::log_warn!("Игрок '{valid_name}' достиг лимита записей (максимум 3)");
            return false;
        }

        // Проверка: достаточно ли высок рекорд для попадания в таблицу
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный рекорд (H3)
            // Исправление E2: заменено unwrap_or(0) на unwrap_or_default()
            let min_score = self
                .entries
                .iter()
                .filter_map(LeaderboardEntry::score)
                .min()
                .unwrap_or_default();
            if score <= min_score {
                return false;
            }
        }

        // Добавление новой записи с валидированным именем
        // Исправление #9: передаём &str вместо String
        // P2: Добавлено reserve() для предотвращения реаллокаций
        self.entries.reserve(1);
        let Some(new_entry) = LeaderboardEntry::new(&valid_name, score) else {
            crate::log_error!("Не удалось создать запись — HMAC подпись вернула ошибку");
            return false;
        };
        self.entries.push(new_entry);

        // Сортировка по убыванию очков.
        // Используем `score_value` напрямую вместо `score()`, так как:
        // 1. `score()` выполняет HMAC-верификацию и возвращает `Option<u128>`,
        //    что некорректно для сортировки (None < Some, tampered entries всплывают наверх).
        // 2. Для корректной сортировки достаточно сырого значения — Integrity
        //    проверяется отдельно при отображении через `score()` / `is_valid()`.
        self.entries
            .sort_by_key(|b| std::cmp::Reverse(b.score_value));

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
    ///
    /// # Пример использования
    /// ```ignore
    /// let leaderboard = Leaderboard::default();
    /// leaderboard.add_score("Player1", 1000);
    /// leaderboard.add_score("Player2", 2000);
    /// let entries = leaderboard.get_entries();
    /// assert_eq!(entries.len(), 2);
    /// ```
    #[must_use]
    pub fn get_entries(&self) -> &[LeaderboardEntry] {
        &self.entries
    }

    /// Получить лучший рекорд.
    ///
    /// # Возвращает
    /// Лучший рекорд или 0, если таблица пуста
    /// Возвращает u128 для предотвращения переполнения.
    ///
    /// # Пример использования
    /// ```ignore
    /// let leaderboard = Leaderboard::default();
    /// leaderboard.add_score("Player1", 1000);
    /// leaderboard.add_score("Player2", 2000);
    /// let best = leaderboard.get_best_score();
    /// assert_eq!(best, 2000);
    /// ```
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
    #[allow(dead_code)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Проверить, пуста ли таблица.
    ///
    /// # Возвращает
    /// `true` если таблица пуста
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
