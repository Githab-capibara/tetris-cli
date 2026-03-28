//! Модуль кэширования строк для отрисовки.
//!
//! Этот модуль содержит структуру `StringCache` для кэширования строк UI,
//! что предотвращает лишние аллокации при отрисовке.
//!
//! ## Архитектурные заметки
//! ## Data Clumps Problem (Problem 2.4)
//! Выделено из state.rs для устранения Data Clumps проблемы.
//!
//! ## Статус использования (ИСПРАВЛЕНИЕ #11)
//! **Этот модуль используется только в тестах и как пример архитектуры.**
//!
//! В основном коде используется встроенное кэширование в `GameState` через поля:
//! - `cached_score_str`
//! - `cached_level_str`
//! - `cached_lines_str`
//! - `cached_high_score_str`
//! - `cached_combo_str`
//! - `cached_timer_str`
//!
//! Этот модуль экспортируется для:
//! 1. Модульных тестов кэширования
//! 2. Как пример отдельной структуры кэша для возможной будущей миграции
//! 3. Обратной совместимости
//!
//! ## Возможная будущая миграция
//! В будущем `GameState` может быть рефакторирован для использования `StringCache`:
//! ```ignore
//! pub struct GameState {
//!     // ...
//!     cache: StringCache,  // Вместо отдельных полей cached_*_str
//!     // ...
//! }
//! ```

use std::fmt::Write;

use super::state::{GameMode, GameStats};

/// Кэш строк для отрисовки UI.
///
/// Содержит кэшированные строки для счёта, уровня, линий, комбо, рекорда и таймера.
/// Используется для предотвращения лишних аллокаций при отрисовке.
///
/// ## Преимущества:
/// - **Производительность**: избегаем форматирования строк каждый кадр
/// - **Инкапсуляция**: логика кэширования отделена от состояния игры
/// - **Тестируемость**: можно тестировать кэширование отдельно
///
/// # Архитектурные заметки
/// TODO (#архитектура): Добавить #[`allow(dead_code)`] если модуль не используется в основном коде.
/// В настоящее время используется только в тестах.
#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct StringCache {
    /// Кэшированная строка счёта.
    pub score_str: String,
    /// Кэшированная строка уровня.
    pub level_str: String,
    /// Кэшированная строка количества линий.
    pub lines_str: String,
    /// Кэшированная строка рекорда.
    pub high_score_str: String,
    /// Кэшированная строка комбо.
    pub combo_str: String,
    /// Кэшированная строка таймера.
    pub timer_str: String,
    /// Последнее закэшированное значение счёта.
    last_score: u128,
    /// Последнее закэшированное значение уровня.
    last_level: u32,
    /// Последнее закэшированное значение количества линий.
    last_lines: u32,
    /// Последнее закэшированное значение комбо.
    last_combo: u32,
    /// Последнее закэшированное значение таймера (для режима спринт).
    last_timer: f64,
}

#[allow(dead_code)]
impl StringCache {
    /// Создать новый кэш строк.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Обновить кэшированные строки.
    ///
    /// # Аргументы
    /// * `score` - текущий счёт
    /// * `level` - текущий уровень
    /// * `lines_cleared` - количество очищенных линий
    /// * `high_score_display` - строка рекорда извне
    /// * `combo` - текущее комбо
    /// * `mode` - режим игры (для таймера)
    /// * `stats` - статистика игры (для таймера)
    #[track_caller]
    #[allow(clippy::too_many_arguments)] // Кэширование требует множественных параметров
    pub fn update(
        &mut self,
        score: u128,
        level: u32,
        lines_cleared: u32,
        high_score_display: &str,
        combo: u32,
        mode: GameMode,
        stats: &GameStats,
    ) {
        // Обновление счёта
        if score != self.last_score {
            self.score_str.clear();
            let _ = write!(self.score_str, "{score:10}");
            self.last_score = score;
        }

        // Обновление уровня
        if level != self.last_level {
            self.level_str.clear();
            let _ = write!(self.level_str, "{level:10}");
            self.last_level = level;
        }

        // Обновление линий
        if lines_cleared != self.last_lines {
            self.lines_str.clear();
            let _ = write!(self.lines_str, "{lines_cleared:10}");
            self.last_lines = lines_cleared;
        }

        // Обновление рекорда
        if self.high_score_str != high_score_display {
            self.high_score_str = high_score_display.to_string();
        }

        // Обновление комбо
        if combo != self.last_combo {
            if combo > 1 {
                self.combo_str = format!("Комбо: x{combo}");
            } else {
                self.combo_str.clear();
            }
            self.last_combo = combo;
        }

        // Обновление таймера (только для режима спринт)
        if mode == GameMode::Sprint {
            let elapsed = stats.get_elapsed_time();
            if (elapsed - self.last_timer).abs() > f64::EPSILON {
                self.timer_str = format!("Время: {elapsed:.2}с");
                self.last_timer = elapsed;
            }
        }
    }

    /// Получить кэшированную строку счёта.
    #[must_use]
    pub fn score(&self) -> &str {
        &self.score_str
    }

    /// Получить кэшированную строку уровня.
    #[must_use]
    pub fn level(&self) -> &str {
        &self.level_str
    }

    /// Получить кэшированную строку линий.
    #[must_use]
    pub fn lines(&self) -> &str {
        &self.lines_str
    }

    /// Получить кэшированную строку рекорда.
    #[must_use]
    pub fn high_score(&self) -> &str {
        &self.high_score_str
    }

    /// Получить кэшированную строку комбо.
    #[must_use]
    pub fn combo(&self) -> &str {
        &self.combo_str
    }

    /// Получить кэшированную строку таймера.
    #[must_use]
    pub fn timer(&self) -> &str {
        &self.timer_str
    }

    /// Сбросить все кэшированные строки.
    pub fn clear(&mut self) {
        self.score_str.clear();
        self.level_str.clear();
        self.lines_str.clear();
        self.high_score_str.clear();
        self.combo_str.clear();
        self.timer_str.clear();
        self.last_score = 0;
        self.last_level = 0;
        self.last_lines = 0;
        self.last_combo = 0;
        self.last_timer = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_cache_new() {
        let cache = StringCache::new();
        assert!(cache.score_str.is_empty());
        assert!(cache.level_str.is_empty());
        assert!(cache.lines_str.is_empty());
    }

    #[test]
    fn test_string_cache_update_score() {
        let mut cache = StringCache::new();
        cache.update(100, 1, 0, "0", 0, GameMode::Classic, &GameStats::default());
        assert_eq!(cache.score(), "       100");
    }

    #[test]
    fn test_string_cache_update_level() {
        let mut cache = StringCache::new();
        cache.update(0, 5, 0, "0", 0, GameMode::Classic, &GameStats::default());
        assert_eq!(cache.level(), "         5");
    }

    #[test]
    fn test_string_cache_update_combo() {
        let mut cache = StringCache::new();
        cache.update(0, 1, 0, "0", 3, GameMode::Classic, &GameStats::default());
        assert_eq!(cache.combo(), "Комбо: x3");
    }

    #[test]
    fn test_string_cache_no_update_if_unchanged() {
        let mut cache = StringCache::new();
        cache.update(100, 1, 0, "0", 0, GameMode::Classic, &GameStats::default());
        let old_score_str = cache.score_str.clone();
        cache.update(100, 1, 0, "0", 0, GameMode::Classic, &GameStats::default());
        assert_eq!(cache.score_str, old_score_str);
    }
}
