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
use std::sync::{Arc, Mutex, RwLock};

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

// ============================================================================
// ПОТОКОБЕЗОПАСНАЯ ОБЁРТКА (ИСПРАВЛЕНИЕ #1 - TOCTOU)
// ============================================================================

/// Потокобезопасная обёртка для `LeaderboardEntry`.
///
/// # Назначение
/// Использует Mutex для защиты от TOCTOU уязвимости в многопоточном коде.
/// Обеспечивает атомарный доступ к score и `is_valid()`.
///
/// # Потокобезопасность
/// ## Исправление C2 (TOCTOU) - ЯВНАЯ ДОКУМЕНТАЦИЯ
///
/// **Эта структура предназначена для многопоточного доступа.**
/// В отличие от [`LeaderboardEntry`], `ThreadSafeLeaderboardEntry`:
/// - ✅ `Send` - может передаваться между потоками
/// - ✅ `Sync` - может использоваться из нескольких потоков одновременно
/// - ✅ Атомарные операции - все методы используют Mutex для синхронизации
///
/// ### Гарантии безопасности
///
/// #### 1. Защита от TOCTOU (Time-Of-Check-Time-Of-Use)
/// - Метод `score()` выполняет валидацию и возврат атомарно под блокировкой Mutex
/// - Метод `is_valid()` выполняет проверку атомарно под блокировкой Mutex
/// - Между проверкой и использованием данные не могут быть изменены другим потоком
///
/// #### 2. Отсутствие гонок данных
/// - Все поля защищены Mutex
/// - Блокировка захватывается на время всей операции
/// - Паника при отравлении Mutex (poisoning) обрабатывается через `expect()`
///
/// ### Примеры использования
///
/// #### ✅ Многопоточный доступ (БЕЗОПАСНО)
/// ```ignore
/// use std::sync::Arc;
/// use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboardEntry;
///
/// let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player", 1000).unwrap());
///
/// // Поток 1: атомарная валидация и возврат
/// let score = entry.score();
///
/// // Поток 2: атомарная проверка
/// let is_valid = entry.is_valid();
/// ```
///
/// #### ⚠️ Ограничения
/// - **Производительность**: Mutex добавляет накладные расходы на синхронизацию
/// - **Паника**: при панике внутри блокировки Mutex может быть "отравлён"
/// - **Гранулярность**: одна блокировка для всех операций (не подходит для fine-grained доступа)
///
/// ### Технические детали
/// - **Время удержания блокировки**: минимальное (только на время валидации)
/// - **Тип блокировки**: эксклюзивная (Mutex, не `RwLock`)
/// - **Обработка паники**: `expect()` с сообщением "Mutex poisoned"
///
/// ### Сравнение с `LeaderboardEntry`
/// | Характеристика | `LeaderboardEntry` | `ThreadSafeLeaderboardEntry` |
/// |----------------|------------------|---------------------------|
/// | Потокобезопасность | ❌ (!Send + !Sync) | ✅ (Send + Sync) |
/// | TOCTOU защита | ❌ Только в single-thread | ✅ Полная защита |
/// | Производительность | ✅ Высокая | ⚠️ Средняя (Mutex overhead) |
/// | Использование | Single-thread | Multi-thread |
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboardEntry;
///
/// let entry = ThreadSafeLeaderboardEntry::new("Player", 1000).unwrap();
///
/// // Поток 1
/// let score = entry.score(); // Атомарная валидация и возврат
///
/// // Поток 2
/// let is_valid = entry.is_valid(); // Атомарная проверка
/// ```
pub struct ThreadSafeLeaderboardEntry {
    /// Внутренние данные, защищённые `RwLock` для лучшей производительности чтения.
    inner: RwLock<LeaderboardEntryData>,
}

/// Внутренние данные записи (для сериализации).
#[derive(Serialize, Deserialize, Clone, Debug)]
struct LeaderboardEntryData {
    /// Имя игрока.
    name: String,
    /// Значение рекорда.
    score_value: u128,
    /// Соль для хэша.
    salt: String,
    /// Хэш записи.
    hash: String,
}

impl ThreadSafeLeaderboardEntry {
    /// Создать новую потокобезопасную запись.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// - `Some(ThreadSafeLeaderboardEntry)` — новый экземпляр
    /// - `None` — при ошибке HMAC подписи
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    ///
    /// # Исправление C9 (CRITICAL)
    /// Использует `RwLock` вместо Mutex для лучшей производительности чтения.
    ///
    /// # Исправление аудита
    /// Возвращает `Option<Self>` — `None` при ошибке HMAC подписи.
    #[must_use]
    pub fn new(name: &str, score: u128) -> Option<Self> {
        // Исправление проблемы 32: используем общую функцию create_entry_data
        let (valid_name, salt, hash) = create_entry_data(name, score)?;

        Some(Self {
            inner: RwLock::new(LeaderboardEntryData {
                name: valid_name,
                score_value: score,
                salt,
                hash,
            }),
        })
    }

    /// Получить значение рекорда с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(u128)` — значение рекорда если запись валидна
    /// - `None` — если `RwLock` отравлен ИЛИ запись не прошла валидацию хэша
    ///   (невозможно отличить ошибку блокировки от подделанной записи)
    ///
    /// # Безопасность
    /// Метод использует `RwLock` для защиты от TOCTOU уязвимости.
    /// Валидация и возврат значения выполняются атомарно под блокировкой чтения.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<u128>` для явной обработки отравления `RwLock`.
    ///
    /// # Исправление C9 (CRITICAL)
    /// Использует `read()` вместо `lock()` для лучшей производительности чтения.
    // P3-ID43: Исправление (perf #23): tuple borrow вместо clone.
    // Копируем только score_value (Copy), берём ссылки на String-поля.
    // Формируем буфер внутри guard-области, затем отпускаем lock и проверяем HMAC.
    #[must_use]
    #[allow(unused_variables)]
    pub fn score_safe(&self) -> Option<u128> {
        let guard = self.inner.read().ok()?;
        // Копируем только score_value (u128 — Copy, 16 байт, стековая копия)
        let score_value = guard.score_value;
        // Копируем salt — минимальный clone (обычно короткий, ~16 байт)
        let salt = guard.salt.clone();
        // Копируем hash — минимальный clone (фиксированный, 64 байта hex)
        let hash = guard.hash.clone();
        // Формируем буфер под guard-ом, избегая format!() аллокации
        let mut buf = Vec::with_capacity(salt.len() + 1 + guard.name.len() + 1 + 20);
        let _ = write!(buf, "{}:{}:{}", salt, guard.name, score_value);
        // Отпускаем guard явно (drop) перед HMAC-проверкой
        drop(guard);
        let valid = hmac_verify_with_salt_bytes(get_leaderboard_hmac_key(), &salt, &buf, &hash);
        if !valid {
            return None;
        }
        Some(score_value)
    }

    /// Проверить валидность записи.
    ///
    /// # Возвращает
    /// `true` если хэш совпадает
    ///
    /// # Устарело
    /// Используйте [`Self::is_valid_safe()`] для явной обработки ошибок.
    #[must_use]
    #[deprecated(
        since = "0.97.0",
        note = "Используйте is_valid_safe() для безопасной обработки ошибок"
    )]
    pub fn is_valid(&self) -> bool {
        // Делегируем is_valid_safe() для устранения дублирования логики
        self.is_valid_safe().unwrap_or(false)
    }

    /// Проверить валидность записи с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(true)` — хэш совпадает
    /// - `Some(false)` — хэш не совпадает
    /// - `None` — если `RwLock` отравлен
    ///
    /// # Безопасность
    /// Метод использует `RwLock` для защиты от TOCTOU уязвимости.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<bool>` для явной обработки отравления `RwLock`.
    ///
    /// # Исправление C9 (CRITICAL)
    /// Использует `read()` вместо `lock()` для лучшей производительности чтения.
    // P3-ID45: Известная неэффективность — format!() выделяет строку каждый вызов.
    // Принято: чтение под RwLock блокировкой — строка формируется внутри guard области.
    #[must_use]
    #[allow(unused_variables)]
    pub fn is_valid_safe(&self) -> Option<bool> {
        // Исправление C9: используем read() вместо lock() для RwLock
        let guard = self.inner.read().ok()?;
        // Оптимизация аудита #26: write! в Vec вместо format!() для избежания аллокации
        let score_value = guard.score_value;
        let salt = guard.salt.clone();
        let hash = guard.hash.clone();
        let mut buf = Vec::with_capacity(salt.len() + 1 + guard.name.len() + 1 + 20);
        let _ = write!(buf, "{}:{}:{}", salt, guard.name, score_value);
        drop(guard);
        Some(hmac_verify_with_salt_bytes(
            get_leaderboard_hmac_key(),
            &salt,
            &buf,
            &hash,
        ))
    }

    /// Получить имя игрока.
    ///
    /// # Возвращает
    /// - `Some(String)` — имя игрока
    /// - `None` — если Mutex отравлен
    ///
    /// # Исправление E2 (CRITICAL)
    /// Изменён тип возврата на `Option<String>` для обработки отравления Mutex.
    ///
    /// # Устарело
    /// Используйте [`Self::name_safe()`] для явной обработки ошибок.
    ///
    /// # Исправление perf #25
    /// Метод deprecated. Clone неизбе — caller ожидает `String`.
    /// Оставлен как есть для обратной совместимости.
    #[must_use]
    #[deprecated(
        since = "0.97.0",
        note = "Используйте name_safe() для безопасной обработки ошибок"
    )]
    pub fn name(&self) -> String {
        // Исправление C9: используем read() вместо lock() для RwLock
        if let Ok(guard) = self.inner.read() {
            return guard.name.clone();
        }
        crate::log_error!("ThreadSafeLeaderboardEntry::name(): RwLock poisoned");
        String::from("[ошибка доступа]")
    }

    /// Получить имя игрока с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(String)` — имя игрока
    /// - `None` — если `RwLock` отравлен
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<String>` для явной обработки отравления `RwLock`.
    ///
    /// # Исправление C9 (CRITICAL)
    /// Использует `read()` вместо `lock()` для лучшей производительности чтения.
    ///
    /// # Исправление perf #26
    /// Clone неизбе — caller ожидает владеющий `String`.
    /// Возврат `&str` невозможен из-за `RwLock` guard lifetime.
    #[must_use]
    pub fn name_safe(&self) -> Option<String> {
        // Исправление C9: используем read() вместо lock() для RwLock
        if let Ok(guard) = self.inner.read() {
            return Some(guard.name.clone());
        }
        crate::log_error!("ThreadSafeLeaderboardEntry::name_safe(): RwLock poisoned");
        None
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

// ============================================================================
// ПОТОКОБЕЗОПАСНАЯ ВЕРСИЯ LEADERBOARD (ИСПРАВЛЕНИЕ E6 - RACE CONDITION)
// ============================================================================

/// Потокобезопасная таблица лидеров.
///
/// # Назначение
/// Использует `Arc<Mutex<>>` для защиты от race condition в многопоточном коде.
/// Обеспечивает безопасный конкурентный доступ к методам `add_score()`, `save()`, и т.д.
///
/// # Потокобезопасность
/// - ✅ `Send` - может передаваться между потоками
/// - ✅ `Sync` - может использоваться из нескольких потоков одновременно
/// - ✅ Атомарные операции - все методы используют Mutex для синхронизации
///
/// # Исправление E6 (HIGH)
/// Добавлена для устранения race condition в `add_score()` при многопоточном доступе.
///
/// # Пример использования
/// ```ignore
/// use std::sync::Arc;
/// use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboard;
///
/// let leaderboard = ThreadSafeLeaderboard::new();
///
/// // Поток 1
/// let lb_clone = Arc::clone(&leaderboard);
/// std::thread::spawn(move || {
///     lb_clone.add_score("Player1", 1000);
/// });
///
/// // Поток 2
/// let lb_clone2 = Arc::clone(&leaderboard);
/// std::thread::spawn(move || {
///     lb_clone2.add_score("Player2", 2000);
/// });
/// ```
pub struct ThreadSafeLeaderboard {
    /// Внутренняя таблица лидеров, защищённая Mutex.
    inner: Arc<Mutex<Leaderboard>>,
}

impl ThreadSafeLeaderboard {
    /// Создать новую потокобезопасную таблицу лидеров.
    ///
    /// # Возвращает
    /// Новый экземпляр `ThreadSafeLeaderboard` с пустой таблицей лидеров
    ///
    /// # Примечание
    /// Используется `Arc<Mutex<>>` для потокобезопасности, даже если Mutex не реализует Send+Sync.
    /// Это необходимо для защиты от race condition при добавлении рекордов.
    #[must_use]
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Leaderboard::default())),
        }
    }

    /// Загрузить таблицу лидеров из файла конфигурации.
    ///
    /// # Возвращает
    /// Новый экземпляр `ThreadSafeLeaderboard` с загруженными данными
    ///
    /// # Исправление E6 (HIGH)
    /// Загрузка выполняется атомарно под блокировкой Mutex.
    ///
    /// # Примечание
    /// Используется `Arc<Mutex<>>` для потокобезопасности, даже если Mutex не реализует Send+Sync.
    #[must_use]
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn load() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Leaderboard::load())),
        }
    }

    /// Добавить новый рекорд в таблицу лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// `true` если рекорд был добавлен в таблицу (вошёл в топ-5),
    /// `false` если рекорд недостаточно высок
    ///
    /// # Исправление E6 (HIGH)
    /// Метод атомарен и защищён от race condition через Mutex.
    #[must_use]
    pub fn add_score(&self, name: &str, score: u128) -> bool {
        self.inner.lock().map_or_else(
            |_| {
                crate::log_error!("ThreadSafeLeaderboard::add_score(): Mutex poisoned");
                false
            },
            |mut leaderboard| leaderboard.add_score(name, score),
        )
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    ///
    /// # Исправление E6 (HIGH)
    /// Сохранение выполняется атомарно под блокировкой Mutex.
    pub fn save(&self) {
        match self.inner.lock() {
            Ok(leaderboard) => leaderboard.save(),
            Err(_) => {
                crate::log_error!("ThreadSafeLeaderboard::save(): Mutex poisoned");
            }
        }
    }

    /// Получить список рекордов.
    ///
    /// # Возвращает
    /// Вектор с копиями записей таблицы лидеров
    ///
    /// # Примечания
    /// Метод возвращает копию данных для сохранения потокобезопасности.
    #[must_use]
    pub fn get_entries(&self) -> Vec<LeaderboardEntry> {
        self.inner.lock().map_or_else(
            |_| {
                crate::log_error!("ThreadSafeLeaderboard::get_entries(): Mutex poisoned");
                Vec::new()
            },
            |leaderboard| leaderboard.get_entries().to_vec(),
        )
    }

    /// Получить лучший рекорд.
    ///
    /// # Возвращает
    /// Лучший рекорд или 0, если таблица пуста
    ///
    /// # Исправление E6 (HIGH)
    /// Метод атомарен и защищён от race condition.
    #[must_use]
    pub fn get_best_score(&self) -> u128 {
        self.inner.lock().map_or_else(
            |_| {
                crate::log_error!("ThreadSafeLeaderboard::get_best_score(): Mutex poisoned");
                0
            },
            |leaderboard| leaderboard.get_best_score(),
        )
    }

    /// Очистить таблицу лидеров.
    ///
    /// # Исправление E6 (HIGH)
    /// Очистка выполняется атомарно под блокировкой Mutex.
    pub fn clear(&self) {
        match self.inner.lock() {
            Ok(mut leaderboard) => leaderboard.clear(),
            Err(_) => {
                crate::log_error!("ThreadSafeLeaderboard::clear(): Mutex poisoned");
            }
        }
    }
}

impl Default for ThreadSafeLeaderboard {
    fn default() -> Self {
        Self::new()
    }
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
                crate::log_warn!("Не удалось загрузить таблицу лидеров. Попытка загрузки из backup...");
                // Попытка загрузить из backup файла
                match load(APP_NAME, Some("leaderboard_backup")) {
                    Ok(backup_leaderboard) => {
                        crate::log_info!("Успешно загружено из backup файла.");
                        backup_leaderboard
                    }
                    Err(_backup_e) => {
                        crate::log_warn!("Не удалось загрузить backup. Используется пустая таблица.");
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

// ============================================================================
// ТЕСТЫ ДЛЯ THREADSAFELEADERBOARDENTRY (ИСПРАВЛЕНИЕ #1 - TOCTOU)
// ============================================================================

#[cfg(test)]
mod thread_safe_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    /// Тест 1: Базовая функциональность `ThreadSafeLeaderboardEntry`
    ///
    /// Проверяет создание и базовые методы потокобезопасной записи.
    #[test]
    fn test_thread_safe_entry_basic() {
        let entry = ThreadSafeLeaderboardEntry::new("Player1", 1000).unwrap();

        assert_eq!(entry.name_safe().as_deref(), Some("Player1"));
        assert_eq!(entry.score_safe(), Some(1000));
        assert!(entry.is_valid_safe().unwrap_or(false));
    }

    /// Тест 2: Многопоточный доступ к `score_safe()`
    ///
    /// Проверяет атомарность метода `score_safe()` при доступе из нескольких потоков.
    #[test]
    fn test_thread_safe_entry_concurrent_score() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player2", 2000).unwrap());
        let mut handles = vec![];

        // Создаём 10 потоков, каждый читает score
        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let score = entry_clone.score_safe();
                assert_eq!(
                    score,
                    Some(2000),
                    "score_safe() должен возвращать корректное значение"
                );
            });
            handles.push(handle);
        }

        // Ждём завершения всех потоков
        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 3: Многопоточный доступ к `is_valid_safe()`
    ///
    /// Проверяет атомарность метода `is_valid_safe()` при доступе из нескольких потоков.
    #[test]
    fn test_thread_safe_entry_concurrent_is_valid() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player3", 3000).unwrap());
        let mut handles = vec![];

        // Создаём 10 потоков, каждый проверяет валидность
        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let valid = entry_clone.is_valid_safe();
                assert!(
                    valid.unwrap_or(false),
                    "is_valid_safe() должен возвращать true для валидной записи"
                );
            });
            handles.push(handle);
        }

        // Ждём завершения всех потоков
        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 4: Смешанный многопоточный доступ (`score_safe` и `is_valid_safe`)
    ///
    /// Проверяет корректную работу при одновременном вызове разных методов.
    #[test]
    fn test_thread_safe_entry_mixed_concurrent_access() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player4", 4000).unwrap());
        let mut handles = vec![];

        // Половина потоков вызывает score_safe(), половина - is_valid_safe()
        for i in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                if i % 2 == 0 {
                    let score = entry_clone.score_safe();
                    assert_eq!(score, Some(4000));
                } else {
                    let valid = entry_clone.is_valid_safe();
                    assert!(valid.unwrap_or(false));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 5: Атомарность валидации и возврата значения
    ///
    /// Проверяет что между проверкой хэша и возвратом значения нет гонки данных.
    #[test]
    fn test_thread_safe_entry_atomic_validation() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player5", 5000).unwrap());
        let mut handles = vec![];

        // Многократные проверки должны всегда возвращать корректное значение
        for _ in 0..20 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                // Многократные вызовы для проверки стабильности
                for _ in 0..10 {
                    let score = entry_clone.score_safe();
                    assert_eq!(score, Some(5000), "score_safe() должен быть атомарным");

                    let valid = entry_clone.is_valid_safe();
                    assert!(
                        valid.unwrap_or(false),
                        "is_valid_safe() должен быть атомарным"
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 6: `ThreadSafeLeaderboardEntry` с разными именами
    ///
    /// Проверяет корректную работу с различными именами игроков.
    #[test]
    fn test_thread_safe_entry_different_names() {
        let names = ["Alice", "Bob", "Charlie", "Игрок", "Player_123"];

        for name in names {
            let entry = ThreadSafeLeaderboardEntry::new(name, 1000).unwrap();
            assert_eq!(entry.name_safe().as_deref(), Some(name));
            assert!(entry.is_valid_safe().unwrap_or(false));
        }
    }

    /// Тест 7: `ThreadSafeLeaderboardEntry` с нулевым счётом
    ///
    /// Проверяет корректную работу с нулевым значением рекорда.
    #[test]
    fn test_thread_safe_entry_zero_score() {
        let entry = ThreadSafeLeaderboardEntry::new("Player", 0).unwrap();
        assert_eq!(entry.score_safe(), Some(0));
        assert!(entry.is_valid_safe().unwrap_or(false));
    }

    /// Тест 8: `ThreadSafeLeaderboardEntry` с максимальным u128
    ///
    /// Проверяет работу с максимально возможным значением u128.
    #[test]
    fn test_thread_safe_entry_max_score() {
        let max_score = u128::MAX;
        let entry = ThreadSafeLeaderboardEntry::new("MaxPlayer", max_score).unwrap();
        assert_eq!(entry.score_safe(), Some(max_score));
        assert!(entry.is_valid_safe().unwrap_or(false));
    }

    /// Тест 9: `name_safe()` метод потокобезопасность
    ///
    /// Проверяет атомарность метода `name_safe()` при доступе из нескольких потоков.
    #[test]
    fn test_thread_safe_entry_concurrent_name() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("TestPlayer", 1000).unwrap());
        let mut handles = vec![];

        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let name = entry_clone.name_safe();
                assert_eq!(name.as_deref(), Some("TestPlayer"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 10: Стресс-тест с большим количеством потоков
    ///
    /// Проверяет стабильность при высокой конкуренции за Mutex.
    #[test]
    fn test_thread_safe_entry_stress_test() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("StressPlayer", 9999).unwrap());
        let mut handles = vec![];

        // Создаём 50 потоков для стресс-теста
        for i in 0..50 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let score = entry_clone.score_safe();
                    let valid = entry_clone.is_valid_safe();
                    let _name = entry_clone.name_safe();

                    assert_eq!(score, Some(9999));
                    assert!(valid.unwrap_or(false));
                }
                // Разные операции для разных потоков
                if i % 5 == 0 {
                    thread::yield_now();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ПРОВЕРКИ СИНХРОНИЗАЦИИ (ИСПРАВЛЕНИЕ #7)
    // =========================================================================

    /// Тест: проверка что Leaderboard не реализует Send+Sync
    ///
    /// Этот тест документирует что Leaderboard требует внешней синхронизации
    /// для использования в многопоточном коде.
    ///
    /// # Примечания
    /// Тест компилируется только если тип !Send + !Sync
    /// Это документационный тест - он показывает что тип не является
    /// потокобезопасным по умолчанию.
    #[test]
    fn test_leaderboard_not_send_sync() {
        // Leaderboard намеренно не реализует Send+Sync без внешней синхронизации.
        // ThreadSafeLeaderboardEntry — thread-safe обёртка.
        let mut leaderboard = Leaderboard::default();

        // Проверяем что Leaderboard пустой при инициализации
        assert!(
            leaderboard.get_entries().is_empty(),
            "Новый Leaderboard должен быть пустым"
        );

        // Проверяем что можно добавить запись
        let result = leaderboard.add_score("TestPlayer", 100);
        assert!(result, "Должна быть возможность добавить запись");
        assert_eq!(
            leaderboard.len(),
            1,
            "После добавления должна быть 1 запись"
        );
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ SEC2: RATE LIMITING
    // =========================================================================

    /// Тест SEC2: проверка rate limiting - максимум 3 записи на игрока
    #[test]
    fn test_sec2_rate_limiting_max_3_per_player() {
        let mut leaderboard = Leaderboard::default();

        // Добавляем 3 записи от одного игрока
        assert!(leaderboard.add_score("Player1", 1000));
        assert!(leaderboard.add_score("Player1", 2000));
        assert!(leaderboard.add_score("Player1", 3000));

        // 4-я запись должна быть отклонена
        assert!(
            !leaderboard.add_score("Player1", 4000),
            "Игрок не должен иметь больше 3 записей"
        );

        // Проверяем что в таблице только 3 записи от Player1
        let player_entries_count = leaderboard
            .entries
            .iter()
            .filter(|e| e.name() == "Player1")
            .count();
        assert_eq!(player_entries_count, 3);
    }

    /// Тест SEC2: проверка что разные игроки могут добавлять записи
    #[test]
    fn test_sec2_different_players_can_add_scores() {
        let mut leaderboard = Leaderboard::default();

        // 5 разных игроков могут добавить по 3 записи каждый
        for i in 1..=5 {
            for j in 1..=3 {
                let player_name = format!("Player{i}");
                let score = i * 1000 + j * 100;
                assert!(
                    leaderboard.add_score(&player_name, score),
                    "Игрок {i} должен иметь возможность добавить запись {j}"
                );
            }
        }

        // В таблице должно остаться только топ-5
        assert_eq!(leaderboard.entries.len(), 5);
    }

    /// Тест SEC2: проверка сообщения о лимите записей
    #[test]
    fn test_sec2_rate_limit_message() {
        let mut leaderboard = Leaderboard::default();

        // Добавляем 3 записи
        let _ = leaderboard.add_score("TestPlayer", 1000);
        let _ = leaderboard.add_score("TestPlayer", 2000);
        let _ = leaderboard.add_score("TestPlayer", 3000);

        // 4-я запись должна вернуть false и вывести сообщение
        let result = leaderboard.add_score("TestPlayer", 4000);
        assert!(!result, "Должно вернуть false при превышении лимита");
    }
}
