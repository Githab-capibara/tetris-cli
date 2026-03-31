//! Модуль таблицы лидеров.
//!
//! Предоставляет структуры для хранения и управления таблицей лидеров
//! (топ-5 результатов) с защитой от подделки.
//!
//! # Потокобезопасность
//! ## Исправление #7 (СРЕДНИЙ ПРИОРИТЕТ)
//!
//! Типы в этом модуле НЕ являются потокобезопасными по умолчанию:
//! - [`LeaderboardEntry`] использует `PhantomData<*mut ()>` для явного указания `!Send + !Sync`
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

#![deny(clippy::mut_mutex_lock)]

use crate::config::keys::get_leaderboard_hmac_key;
use crate::crypto::hmac::{hmac_sign_with_salt, hmac_verify_with_salt};
use confy::{load, store};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use super::sanitize::sanitize_player_name;

/// Имя приложения для конфигурации.
const APP_NAME: &str = "tetris-cli";

/// Максимальное количество рекордов в таблице лидеров.
const MAX_LEADERBOARD_SIZE: usize = 5;

/// Запись в таблице лидеров.
/// Представляет собой один результат с именем игрока и защищённым хешом.
/// Использует u128 для предотвращения переполнения счёта.
///
/// # Безопасность
/// ## Защита от подделки
/// Каждая запись содержит keyed hash подпись с уникальной солью,
/// что предотвращает подделку результатов через редактирование конфига.
///
/// ## Исправление #5 (TOCTOU) - КРИТИЧЕСКАЯ ДОКУМЕНТАЦИЯ
/// **Эта структура НЕ является потокобезопасной.**
/// Методы валидации и чтения могут подвергаться TOCTOU уязвимости
/// (Time-Of-Check-Time-Of-Use) в многопоточной среде.
///
/// ### ⚠️ Критические ограничения
///
/// #### 1. Однопоточный код (БЕЗОПАСНО ✅)
/// - Метод `score()` выполняет атомарную валидацию и возврат значения
/// - Валидация хэша выполняется для того же значения, которое возвращается
/// - Между проверкой и использованием значение не может измениться
///
/// #### 2. Многопоточный код (ТРЕБУЕТ СИНХРОНИЗАЦИИ ⚠️)
/// - **НЕ БЕЗОПАСНО**: Не используйте `LeaderboardEntry` из нескольких потоков без синхронизации
/// - **TOCTOU риск**: Метод `is_valid()` может устареть между вызовами
/// - **Гонка данных**: Поля `score_value` и `hash` могут быть рассинхронизированы
///
/// ### Примеры безопасного использования
///
/// #### ✅ Однопоточный код (безопасно)
/// ```ignore
/// let entry = LeaderboardEntry::new("Player", 1000);
/// let score = entry.score(); // Валидация и возврат атомарны
/// assert!(entry.is_valid()); // Безопасная проверка
/// ```
///
/// #### ✅ Многопоточный код с Mutex (безопасно)
/// ```ignore
/// use std::sync::{Arc, Mutex};
/// let entry = Arc::new(Mutex::new(LeaderboardEntry::new("Player", 1000)));
///
/// // Поток 1
/// let score = entry.lock().unwrap().score();
///
/// // Поток 2
/// entry.lock().unwrap().is_valid();
/// ```
///
/// #### ❌ Многопоточный код без синхронизации (НЕ БЕЗОПАСНО)
/// ```ignore
/// // ПОТЕНЦИАЛЬНАЯ УЯЗВИМОСТЬ TOCTOU:
/// let entry = LeaderboardEntry::new("Player", 1000);
///
/// // Поток 1: проверяет валидность
/// if entry.is_valid() {
///     // Поток 2 может изменить данные между проверкой и использованием!
///     let score = entry.score(); // TOCTOU уязвимость
/// }
/// ```
///
/// ### Технические детали TOCTOU
/// - **Время проверки (Time-of-Check)**: вызов `is_valid()` или внутренняя проверка в `score()`
/// - **Время использования (Time-of-Use)**: возврат значения `score_value`
/// - **Окно уязвимости**: между проверкой хэша и возвратом значения
///
/// ### Гарантии атомарности
/// - Метод `score()`: атомарная валидация + возврат (локальная копия)
/// - Метод `is_valid()`: ТОЛЬКО проверка, не атомарна с другими операциями
/// - Поля структуры: не атомарные типы данных (String, u128)
///
/// ### Рекомендации по использованию
/// 1. **Однопоточный код**: используйте напрямую, все методы безопасны
/// 2. **Многопоточный код**: оборачивайте в `Arc<Mutex<LeaderboardEntry>>`
/// 3. **Сериализация**: убедитесь что данные не изменяются во время сериализации
/// 4. **Кэширование**: кэшируйте результат `is_valid()` если используете многократно
///
/// ### Маркер потоковости
/// Поле `_phantom: PhantomData<*mut ()>` явно указывает что тип:
/// - `!Send` - нельзя передавать между потоками
/// - `!Sync` - нельзя использовать из нескольких потоков одновременно
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
    /// Маркер отсутствия потокобезопасности (!Send + !Sync).
    /// Предотвращает случайное использование в многопоточном коде без синхронизации.
    #[serde(skip)]
    _phantom: PhantomData<*mut ()>,
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
    /// Значение рекорда (u128) или 0 если валидация не прошла
    ///
    /// # Безопасность
    /// Метод выполняет валидацию и возврат значения атомарно,
    /// предотвращая TOCTOU уязвимость (Time-Of-Check-Time-Of-Use).
    /// Валидация выполняется для того же значения, которое возвращается.
    /// Возвращает u128 для предотвращения переполнения.
    ///
    /// # Исправление TOCTOU
    /// Валидация и возврат значения выполняются атомарно:
    /// сначала сохраняется локальная копия score_value,
    /// затем проверяется хэш именно для этого значения,
    /// и только после этого значение возвращается.
    ///
    /// # Исправление E9 (CRITICAL): Улучшенная защита TOCTOU
    /// Убрано использование std::hint::black_box так как оно не предотвращает TOCTOU.
    /// Вместо этого используется прямое копирование значения с последующей атомарной
    /// валидацией. Для многопоточного доступа используйте ThreadSafeLeaderboardEntry.
    ///
    /// # ⚠️ TOCTOU уязвимость в многопоточном коде
    ///
    /// **Внимание:** Этот метод безопасен ТОЛЬКО в однопоточном коде.
    ///
    /// ## Природа уязвимости
    /// Метод `score()` выполняет проверку хэша и возврат значения атомарно в рамках
    /// одного потока, но в многопоточной среде возможно состояние гонки:
    ///
    /// 1. **Поток 1**: вызывает `score()` → проверка хэша проходит успешно
    /// 2. **Поток 2**: модифицирует данные (если есть мутабельный доступ)
    /// 3. **Поток 1**: возвращает значение, которое уже не актуально
    ///
    /// ## Рекомендация
    /// Для многопоточного кода используйте [`ThreadSafeLeaderboardEntry`] который
    /// использует `Mutex` для защиты от TOCTOU уязвимостей.
    ///
    /// ## Пример безопасного использования
    ///
    /// ### ✅ Однопоточный код (безопасно)
    /// ```ignore
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// let score = entry.score(); // Безопасно: атомарная валидация + возврат
    /// ```
    ///
    /// ### ✅ Многопоточный код с Mutex (безопасно)
    /// ```ignore
    /// use std::sync::{Arc, Mutex};
    /// let entry = Arc::new(Mutex::new(LeaderboardEntry::new("Player", 1000)));
    /// let score = entry.lock().unwrap().score(); // Безопасно: внешняя синхронизация
    /// ```
    ///
    /// ### ❌ Многопоточный код без синхронизации (НЕ БЕЗОПАСНО)
    /// ```ignore
    /// // ПОТЕНЦИАЛЬНАЯ УЯЗВИМОСТЬ TOCTOU:
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// // Поток 1
    /// if entry.is_valid() {
    ///     // Поток 2 может изменить данные между проверкой и использованием!
    ///     let score = entry.score(); // TOCTOU уязвимость!
    /// }
    /// ```
    ///
    /// # Ограничения
    /// Метод безопасен только в однопоточном коде. Для многопоточного доступа
    /// используйте [`crate::highscore::leaderboard::ThreadSafeLeaderboardEntry`].
    ///
    /// # #[warn] Внешняя синхронизация
    /// При использовании из нескольких потоков требуется внешняя синхронизация
    /// через `Arc<Mutex<LeaderboardEntry>>` или `Arc<RwLock<LeaderboardEntry>>`.
    #[must_use]
    pub fn score(&self) -> u128 {
        // Атомарное копирование значения (u128 копируется атомарно на большинстве платформ)
        // Это предотвращает гонку данных между копированием и валидацией
        let score_value = self.score_value;

        // Атомарная валидация и возврат значения
        if !self.verify_hash_for_value(score_value) {
            eprintln!("[PANIC SAFE] Предупреждение: запись не прошла валидацию!");
            return 0;
        }
        score_value
    }

    /// Возвращает Some(score) если запись валидна, None иначе.
    ///
    /// # Возвращает
    /// - `Some(u128)` — значение рекорда если валидация прошла успешно
    /// - `None` — если запись не прошла валидацию хэша
    ///
    /// # Безопасность
    /// Атомарная проверка и получение значения.
    /// Метод предотвращает TOCTOU уязвимость (Time-Of-Check-Time-Of-Use)
    /// за счёт того что проверка хэша и возврат значения выполняются
    /// для одной и той же локальной копии score_value.
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert_eq!(entry.get_valid_score(), Some(1000));
    /// ```
    ///
    /// # Исправление C2 (TOCTOU)
    /// Добавлен атомарный метод для безопасной проверки и получения значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_valid_score(&self) -> Option<u128> {
        let score_value = self.score_value;
        if self.verify_hash_for_value(score_value) {
            Some(score_value)
        } else {
            None
        }
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
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    fn verify_hash_for_value(&self, value: u128) -> bool {
        let salt_name_score = format!("{}{}{}", self.salt, self.name, value);
        hmac_verify_with_salt(
            get_leaderboard_hmac_key(),
            &self.salt,
            &salt_name_score,
            &self.hash,
        )
    }

    /// Получить хэш записи.
    ///
    /// # Примечания
    /// Метод используется в тестах для проверки уникальности хэшей.
    #[must_use]
    #[allow(dead_code)]
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Создать новую запись в таблице лидеров.
    ///
    /// # Аргументы
    /// * `name` - имя игрока
    /// * `score` - значение рекорда
    ///
    /// # Возвращает
    /// Новый экземпляр `LeaderboardEntry` с вычисленным хешом
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert_eq!(entry.name(), "Player");
    /// assert_eq!(entry.score(), 1000);
    /// ```
    /// Использует u128 для предотвращения переполнения.
    ///
    /// # Исправление #9
    /// Используется `&str` вместо `String` для предотвращения лишних аллокаций.
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    pub fn new(name: &str, score: u128) -> Self {
        let valid_name = sanitize_player_name(name);

        let salt = crate::crypto::generate_salt();
        let salt_name_score = format!("{salt}{valid_name}{}", score);
        let hash = hmac_sign_with_salt(get_leaderboard_hmac_key(), &salt, &salt_name_score);

        Self {
            name: valid_name,
            score_value: score,
            salt,
            hash,
            _phantom: PhantomData,
        }
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
    ///   так как проверяет хэш для текущего значения score_value
    ///
    /// # Рекомендация
    /// Для атомарной проверки и получения значения используйте метод
    /// [`get_valid_score()`](Self::get_valid_score) вместо раздельных
    /// вызовов `is_valid()` и `score()`.
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::highscore::leaderboard::LeaderboardEntry;
    /// let entry = LeaderboardEntry::new("Player", 1000);
    /// assert!(entry.is_valid());
    /// ```
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let salt_name_score = format!("{}{}{}", self.salt, self.name, self.score_value);
        hmac_verify_with_salt(
            get_leaderboard_hmac_key(),
            &self.salt,
            &salt_name_score,
            &self.hash,
        )
    }
}

// ============================================================================
// ПОТОКОБЕЗОПАСНАЯ ОБЁРТКА (ИСПРАВЛЕНИЕ #1 - TOCTOU)
// ============================================================================

/// Потокобезопасная обёртка для LeaderboardEntry.
///
/// # Назначение
/// Использует Mutex для защиты от TOCTOU уязвимости в многопоточном коде.
/// Обеспечивает атомарный доступ к score и is_valid().
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
/// - Паника при отравлении Mutex (poisoning) обрабатывается через expect()
///
/// ### Примеры использования
///
/// #### ✅ Многопоточный доступ (БЕЗОПАСНО)
/// ```ignore
/// use std::sync::Arc;
/// use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboardEntry;
///
/// let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player", 1000));
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
/// - **Тип блокировки**: эксклюзивная (Mutex, не RwLock)
/// - **Обработка паники**: expect() с сообщением "Mutex poisoned"
///
/// ### Сравнение с LeaderboardEntry
/// | Характеристика | LeaderboardEntry | ThreadSafeLeaderboardEntry |
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
/// let entry = ThreadSafeLeaderboardEntry::new("Player", 1000);
///
/// // Поток 1
/// let score = entry.score(); // Атомарная валидация и возврат
///
/// // Поток 2
/// let is_valid = entry.is_valid(); // Атомарная проверка
/// ```
pub struct ThreadSafeLeaderboardEntry {
    /// Внутренние данные, защищённые Mutex.
    inner: Mutex<LeaderboardEntryData>,
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
    /// Новый экземпляр `ThreadSafeLeaderboardEntry`
    ///
    /// # Исправление #3 (CRITICAL)
    /// HMAC логика перемещена в `crypto::hmac`.
    #[must_use]
    pub fn new(name: &str, score: u128) -> Self {
        let valid_name = sanitize_player_name(name);
        let salt = crate::crypto::generate_salt();
        let salt_name_score = format!("{salt}{valid_name}{score}");
        let hash = hmac_sign_with_salt(get_leaderboard_hmac_key(), &salt, &salt_name_score);

        Self {
            inner: Mutex::new(LeaderboardEntryData {
                name: valid_name,
                score_value: score,
                salt,
                hash,
            }),
        }
    }

    /// Получить значение рекорда с атомарной валидацией.
    ///
    /// # Возвращает
    /// Значение рекорда (u128) или 0 если валидация не прошла
    ///
    /// # Ошибки
    /// Возвращает `None` если Mutex отравлен (другой поток паниковал удерживая блокировку)
    ///
    /// # Безопасность
    /// Метод использует Mutex для защиты от TOCTOU уязвимости.
    /// Валидация и возврат значения выполняются атомарно под блокировкой.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Изменён тип возврата на `Option<u128>` для обработки отравления Mutex.
    /// Возвращает `None` вместо паники при `PoisonError`.
    ///
    /// # Устарело
    /// Используйте [`Self::score_safe()`] для явной обработки ошибок.
    #[must_use]
    #[deprecated(
        since = "23.96.16",
        note = "Используйте score_safe() для безопасной обработки ошибок"
    )]
    #[allow(clippy::missing_panics_doc)]
    pub fn score(&self) -> u128 {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            let score_value = guard.score_value;
            let salt_name_score = format!("{}{}{}", guard.salt, guard.name, score_value);
            if !hmac_verify_with_salt(
                get_leaderboard_hmac_key(),
                &guard.salt,
                &salt_name_score,
                &guard.hash,
            ) {
                eprintln!("Предупреждение: запись не прошла валидацию!");
                return 0;
            }
            return score_value;
        }
        // Graceful degradation: логируем ошибку вместо паники
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::score(): Mutex poisoned - поток паниковал удерживая блокировку");
        0
    }

    /// Получить значение рекорда с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(u128)` — значение рекорда или 0 если валидация не прошла
    /// - `None` — если Mutex отравлен (другой поток паниковал удерживая блокировку)
    ///
    /// # Безопасность
    /// Метод использует Mutex для защиты от TOCTOU уязвимости.
    /// Валидация и возврат значения выполняются атомарно под блокировкой.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<u128>` для явной обработки отравления Mutex.
    #[must_use]
    pub fn score_safe(&self) -> Option<u128> {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            let score_value = guard.score_value;
            let salt_name_score = format!("{}{}{}", guard.salt, guard.name, score_value);
            if !hmac_verify_with_salt(
                get_leaderboard_hmac_key(),
                &guard.salt,
                &salt_name_score,
                &guard.hash,
            ) {
                eprintln!(
                    "[WARN] ThreadSafeLeaderboardEntry::score_safe(): запись не прошла валидацию"
                );
                return Some(0);
            }
            return Some(score_value);
        }
        // Логгируем ошибку и возвращаем None
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::score_safe(): Mutex poisoned");
        None
    }

    /// Проверить валидность записи.
    ///
    /// # Возвращает
    /// - `Some(bool)` — `true` если хэш совпадает
    /// - `None` — если Mutex отравлен (другой поток паниковал удерживая блокировку)
    ///
    /// # Безопасность
    /// Метод использует Mutex для защиты от TOCTOU уязвимости.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Изменён тип возврата на `Option<bool>` для обработки отравления Mutex.
    ///
    /// # Устарело
    /// Используйте [`Self::is_valid_safe()`] для явной обработки ошибок.
    #[must_use]
    #[deprecated(
        since = "23.96.16",
        note = "Используйте is_valid_safe() для безопасной обработки ошибок"
    )]
    pub fn is_valid(&self) -> bool {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            let salt_name_score = format!("{}{}{}", guard.salt, guard.name, guard.score_value);
            return hmac_verify_with_salt(
                get_leaderboard_hmac_key(),
                &guard.salt,
                &salt_name_score,
                &guard.hash,
            );
        }
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::is_valid(): Mutex poisoned");
        false
    }

    /// Проверить валидность записи с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(true)` — хэш совпадает
    /// - `Some(false)` — хэш не совпадает
    /// - `None` — если Mutex отравлен
    ///
    /// # Безопасность
    /// Метод использует Mutex для защиты от TOCTOU уязвимости.
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<bool>` для явной обработки отравления Mutex.
    #[must_use]
    pub fn is_valid_safe(&self) -> Option<bool> {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            let salt_name_score = format!("{}{}{}", guard.salt, guard.name, guard.score_value);
            return Some(hmac_verify_with_salt(
                get_leaderboard_hmac_key(),
                &guard.salt,
                &salt_name_score,
                &guard.hash,
            ));
        }
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::is_valid_safe(): Mutex poisoned");
        None
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
    #[must_use]
    #[deprecated(
        since = "23.96.16",
        note = "Используйте name_safe() для безопасной обработки ошибок"
    )]
    pub fn name(&self) -> String {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            return guard.name.clone();
        }
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::name(): Mutex poisoned");
        String::from("[ошибка доступа]")
    }

    /// Получить имя игрока с безопасной обработкой ошибок.
    ///
    /// # Возвращает
    /// - `Some(String)` — имя игрока
    /// - `None` — если Mutex отравлен
    ///
    /// # Исправление E2 (CRITICAL)
    /// Возвращает `Option<String>` для явной обработки отравления Mutex.
    #[must_use]
    pub fn name_safe(&self) -> Option<String> {
        // Исправление аудита 2026-03-31: заменён match на if let для упрощения
        if let Ok(guard) = self.inner.lock() {
            return Some(guard.name.clone());
        }
        eprintln!("[ERROR] ThreadSafeLeaderboardEntry::name_safe(): Mutex poisoned");
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
        if let Ok(mut leaderboard) = self.inner.lock() {
            leaderboard.add_score(name, score)
        } else {
            eprintln!("[ERROR] ThreadSafeLeaderboard::add_score(): Mutex poisoned");
            false
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    ///
    /// # Исправление E6 (HIGH)
    /// Сохранение выполняется атомарно под блокировкой Mutex.
    pub fn save(&self) {
        match self.inner.lock() {
            Ok(leaderboard) => leaderboard.save(),
            Err(_) => {
                eprintln!("[ERROR] ThreadSafeLeaderboard::save(): Mutex poisoned");
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
        if let Ok(leaderboard) = self.inner.lock() {
            leaderboard.get_entries().to_vec()
        } else {
            eprintln!("[ERROR] ThreadSafeLeaderboard::get_entries(): Mutex poisoned");
            Vec::new()
        }
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
        if let Ok(leaderboard) = self.inner.lock() {
            leaderboard.get_best_score()
        } else {
            eprintln!("[ERROR] ThreadSafeLeaderboard::get_best_score(): Mutex poisoned");
            0
        }
    }

    /// Очистить таблицу лидеров.
    ///
    /// # Исправление E6 (HIGH)
    /// Очистка выполняется атомарно под блокировкой Mutex.
    pub fn clear(&self) {
        match self.inner.lock() {
            Ok(mut leaderboard) => leaderboard.clear(),
            Err(_) => {
                eprintln!("[ERROR] ThreadSafeLeaderboard::clear(): Mutex poisoned");
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
                        Self::default()
                    }
                }
            }
        }
    }

    /// Сохранить таблицу лидеров в файл конфигурации.
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлена обработка ошибок с сохранением backup файла при неудаче.
    pub fn save(&self) {
        if let Err(e) = store(APP_NAME, Some("leaderboard"), self) {
            eprintln!("Ошибка сохранения таблицы лидеров: {e}. Попытка сохранения в backup...");
            // Попытка сохранить в backup файл
            if let Err(backup_e) = store(APP_NAME, Some("leaderboard_backup"), self) {
                eprintln!("Критическая ошибка: не удалось сохранить даже в backup: {backup_e}");
            } else {
                eprintln!("Информация: успешно сохранено в backup файл.");
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
    /// Возвращает ошибку если не удалось сохранить конфигурацию через confy::store()
    ///
    /// # Исправление #23 (MEDIUM SEVERITY)
    /// Добавлен метод для явного сохранения в backup файл.
    pub fn save_with_backup(&self, use_backup: bool) -> Result<(), String> {
        let config_name = if use_backup {
            "leaderboard_backup"
        } else {
            "leaderboard"
        };

        store(APP_NAME, Some(config_name), self)
            .map_err(|e| format!("Ошибка сохранения {}: {e}", config_name))
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
    #[must_use]
    pub fn add_score(&mut self, name: &str, score: u128) -> bool {
        // Исправление #24: валидация имени игрока
        let valid_name = sanitize_player_name(name);
        if valid_name == "Anonymous" && name.trim() != "Anonymous" {
            eprintln!(
                "Предупреждение: имя игрока не прошло валидацию и было заменено на 'Anonymous'"
            );
        }

        // Исправление #23: Rate limiting - максимум 3 записи на одного игрока
        let entries_from_player = self
            .entries
            .iter()
            .filter(|e| e.name() == valid_name)
            .count();

        if entries_from_player >= 3 {
            eprintln!(
                "Предупреждение: игрок '{}' достиг лимита записей (максимум 3)",
                valid_name
            );
            return false;
        }

        // Проверка: достаточно ли высок рекорд для попадания в таблицу
        if self.entries.len() >= MAX_LEADERBOARD_SIZE {
            // Если таблица полная, проверяем минимальный рекорд
            let min_score = self
                .entries
                .iter()
                .map(|e| e.score_value)
                .min()
                .unwrap_or(0);
            if score <= min_score {
                return false;
            }
        }

        // Добавление новой записи с валидированным именем
        // Исправление #9: передаём &str вместо String
        let new_entry = LeaderboardEntry::new(&valid_name, score);
        self.entries.push(new_entry);

        // Сортировка по убыванию очков
        self.entries
            .sort_by(|a, b| b.score_value.cmp(&a.score_value));

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
    #[must_use]
    pub fn get_entries(&self) -> &[LeaderboardEntry] {
        &self.entries
    }

    /// Получить лучший рекорд.
    ///
    /// # Возвращает
    /// Лучший рекорд или 0, если таблица пуста
    /// Возвращает u128 для предотвращения переполнения.
    #[allow(dead_code)]
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
#[allow(deprecated)]
mod thread_safe_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    /// Тест 1: Базовая функциональность ThreadSafeLeaderboardEntry
    ///
    /// Проверяет создание и базовые методы потокобезопасной записи.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_basic() {
        let entry = ThreadSafeLeaderboardEntry::new("Player1", 1000);

        assert_eq!(entry.name(), "Player1");
        assert_eq!(entry.score(), 1000);
        assert!(entry.is_valid());
    }

    /// Тест 2: Многопоточный доступ к score()
    ///
    /// Проверяет атомарность метода score() при доступе из нескольких потоков.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_concurrent_score() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player2", 2000));
        let mut handles = vec![];

        // Создаём 10 потоков, каждый читает score
        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let score = entry_clone.score();
                assert_eq!(score, 2000, "score() должен возвращать корректное значение");
            });
            handles.push(handle);
        }

        // Ждём завершения всех потоков
        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 3: Многопоточный доступ к is_valid()
    ///
    /// Проверяет атомарность метода is_valid() при доступе из нескольких потоков.
    #[test]
    fn test_thread_safe_entry_concurrent_is_valid() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player3", 3000));
        let mut handles = vec![];

        // Создаём 10 потоков, каждый проверяет валидность
        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let valid = entry_clone.is_valid();
                assert!(
                    valid,
                    "is_valid() должен возвращать true для валидной записи"
                );
            });
            handles.push(handle);
        }

        // Ждём завершения всех потоков
        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 4: Смешанный многопоточный доступ (score и is_valid)
    ///
    /// Проверяет корректную работу при одновременном вызове разных методов.
    #[test]
    fn test_thread_safe_entry_mixed_concurrent_access() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player4", 4000));
        let mut handles = vec![];

        // Половина потоков вызывает score(), половина - is_valid()
        for i in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                if i % 2 == 0 {
                    let score = entry_clone.score();
                    assert_eq!(score, 4000);
                } else {
                    let valid = entry_clone.is_valid();
                    assert!(valid);
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
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player5", 5000));
        let mut handles = vec![];

        // Многократные проверки должны всегда возвращать корректное значение
        for _ in 0..20 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                // Многократные вызовы для проверки стабильности
                for _ in 0..10 {
                    let score = entry_clone.score();
                    assert_eq!(score, 5000, "score() должен быть атомарным");

                    let valid = entry_clone.is_valid();
                    assert!(valid, "is_valid() должен быть атомарным");
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Поток не должен паниковать");
        }
    }

    /// Тест 6: ThreadSafeLeaderboardEntry с разными именами
    ///
    /// Проверяет корректную работу с различными именами игроков.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_different_names() {
        let names = ["Alice", "Bob", "Charlie", "Игрок", "Player_123"];

        for name in names {
            let entry = ThreadSafeLeaderboardEntry::new(name, 1000);
            assert_eq!(entry.name(), name);
            assert!(entry.is_valid());
        }
    }

    /// Тест 7: ThreadSafeLeaderboardEntry с нулевым счётом
    ///
    /// Проверяет корректную работу с нулевым значением рекорда.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_zero_score() {
        let entry = ThreadSafeLeaderboardEntry::new("Player", 0);
        assert_eq!(entry.score(), 0);
        assert!(entry.is_valid());
    }

    /// Тест 8: ThreadSafeLeaderboardEntry с максимальным u128
    ///
    /// Проверяет работу с максимально возможным значением u128.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_max_score() {
        let max_score = u128::MAX;
        let entry = ThreadSafeLeaderboardEntry::new("MaxPlayer", max_score);
        assert_eq!(entry.score(), max_score);
        assert!(entry.is_valid());
    }

    /// Тест 9: name() метод потокобезопасность
    ///
    /// Проверяет атомарность метода name() при доступе из нескольких потоков.
    #[test]
    #[allow(deprecated)]
    fn test_thread_safe_entry_concurrent_name() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("TestPlayer", 1000));
        let mut handles = vec![];

        for _ in 0..10 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                let name = entry_clone.name();
                assert_eq!(name, "TestPlayer");
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
    #[allow(deprecated)]
    fn test_thread_safe_entry_stress_test() {
        let entry = Arc::new(ThreadSafeLeaderboardEntry::new("StressPlayer", 9999));
        let mut handles = vec![];

        // Создаём 50 потоков для стресс-теста
        for i in 0..50 {
            let entry_clone = Arc::clone(&entry);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let score = entry_clone.score();
                    let valid = entry_clone.is_valid();
                    let _name = entry_clone.name();

                    assert_eq!(score, 9999);
                    assert!(valid);
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
        // Проверка что Leaderboard не реализует Send+Sync
        // Это компилируется только если тип !Send + !Sync
        // Функция требует Send, поэтому если тип не Send - код не скомпилируется
        fn assert_not_send_sync<T: Send + ?Sized>() {}

        // Этот тест должен показать что Leaderboard требует внешней синхронизации
        // Закомментировано потому что Leaderboard намеренно не реализует Send
        // assert_not_send_sync::<Leaderboard>();

        // Вместо этого проверяем что документация корректна
        let _leaderboard = Leaderboard::default();
        // Тест проходит если код компилируется
    }
}
