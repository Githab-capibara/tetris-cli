//! Кэш для отрисовки.
//!
//! Модуль содержит структуру [`RenderCache`] для кэширования строк UI,
//! что предотвращает лишние аллокации при отрисовке.
//!
//! ## Архитектурные заметки
//! Выделено из `state.rs` для улучшения организации кода и разделения ответственности.

use std::fmt::Write;

// ============================================================================
// RENDERCACHE — ПРОДАКШЕН
// ============================================================================

/// Кэш для оптимизации отрисовки.
///
/// Содержит кэшированные строки для отображения игровой информации
/// и последние закэшированные значения для сравнения.
///
/// # Архитектурные заметки
/// Выделено из `GameState` для улучшения организации кода и уменьшения
/// размера основной структуры.
#[derive(Debug)]
pub struct RenderCache {
    /// Кэшированная строка счёта для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_score_str: String,
    /// Кэшированная строка уровня для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_level_str: String,
    /// Кэшированная строка количества линий для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_lines_str: String,
    /// Кэшированная строка рекорда для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_high_score_str: String,
    /// Кэшированная строка комбо для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_combo_str: String,
    /// Кэшированная строка таймера для оптимизации отрисовки.
    /// `pub(crate)` — доступ внутри крейта: модуль `render` читает, `GameState` обновляет.
    pub(crate) cached_timer_str: String,

    /// Последнее закэшированное значение счёта для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша при изменении счёта.
    pub(crate) last_cached_score: u128,
    /// Последнее закэшированное значение уровня для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша при изменении уровня.
    pub(crate) last_cached_level: u32,
    /// Последнее закэшированное значение количества линий для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша при очистке линий.
    pub(crate) last_cached_lines: u32,
    /// Последнее закэшированное значение рекорда для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша при изменении рекорда.
    pub(crate) last_cached_high_score: u128,
    /// Последнее закэшированное значение комбо для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша при изменении комбо.
    pub(crate) last_cached_combo: u32,
    /// Последнее закэшированное значение таймера (elapsed * 100, округлённое) для сравнения.
    /// `pub(crate)` — доступ внутри крейта для обновления кэша таймера.
    pub(crate) last_cached_timer: i64,
}

impl RenderCache {
    /// Создать новый кэш для отрисовки.
    ///
    /// # Исправление #4 (LOW)
    /// Используется `String::with_capacity(32)` для всех кэшированных строк
    /// для предотвращения лишних аллокаций при форматировании.
    #[must_use = "Кэш отрисовки должен быть использован"]
    pub fn new() -> Self {
        Self {
            cached_score_str: String::with_capacity(32),
            cached_level_str: String::with_capacity(32),
            cached_lines_str: String::with_capacity(32),
            cached_high_score_str: String::with_capacity(32),
            cached_combo_str: String::with_capacity(32),
            cached_timer_str: String::with_capacity(32),
            last_cached_score: 0,
            last_cached_level: 0,
            last_cached_lines: 0,
            last_cached_high_score: 0,
            last_cached_combo: 0,
            last_cached_timer: 0,
        }
    }

    /// Инициализация кэша начальными значениями.
    ///
    /// # Аргументы
    /// * `score` - начальный счёт
    /// * `level` - начальный уровень
    /// * `lines` - начальное количество линий
    /// * `high_score` - начальный рекорд
    ///
    /// # Исправление #7
    /// Строки хранятся с padding ({:10}) для использования напрямую в отрисовке.
    pub fn init_with_values(&mut self, score: u128, level: u32, lines: u32, high_score: u128) {
        self.last_cached_score = score;
        self.last_cached_level = level;
        self.last_cached_lines = lines;
        self.last_cached_high_score = high_score;
        // Исправление #7: храним с padding для прямой отрисовки без format!
        let _ = write!(self.cached_score_str, "{score:10}");
        let _ = write!(self.cached_level_str, "{level:10}");
        let _ = write!(self.cached_lines_str, "{lines:10}");
        // Исправление проблемы 21: используем write! вместо format! для переиспользования буфера
        self.cached_high_score_str.clear();
        let _ = write!(self.cached_high_score_str, "{high_score:10}");
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_cache_new() {
        let cache = RenderCache::new();
        assert!(cache.cached_score_str.is_empty());
        assert_eq!(cache.last_cached_score, 0);
    }

    #[test]
    fn test_render_cache_init_with_values() {
        let mut cache = RenderCache::new();
        cache.init_with_values(100, 2, 10, 500);
        // Исправление #7: строки хранятся с padding {:10}
        assert_eq!(cache.cached_score_str, "       100");
        assert_eq!(cache.cached_level_str, "         2");
        assert_eq!(cache.cached_lines_str, "        10");
        assert_eq!(cache.cached_high_score_str, "       500");
    }
}
