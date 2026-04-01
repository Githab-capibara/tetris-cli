//! Кэш для отрисовки.
//!
//! Модуль содержит структуру `RenderCache` для кэширования строк UI,
//! что предотвращает лишние аллокации при отрисовке.
//!
//! ## Архитектурные заметки
//! Выделено из `state.rs` для улучшения организации кода и разделения ответственности.
//!
//! Архитектурное улучшение 2026-04-01 (YAGNI3): StringCache используется только в тестах.

use super::mode_trait::GameModeTrait;
use super::stats::GameStats;
use std::fmt::Write;

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
/// StringCache используется только в тестах.
#[derive(Clone, Default)]
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
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        score: u128,
        level: u32,
        lines_cleared: u32,
        high_score_display: &str,
        combo: u32,
        mode: &'_ dyn GameModeTrait,
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
        if mode.get_target_lines() == Some(40) {
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

/// Кэш для оптимизации отрисовки.
///
/// Содержит кэшированные строки для отображения игровой информации
/// и последние закэшированные значения для сравнения.
///
/// # Архитектурные заметки
/// Выделено из GameState для улучшения организации кода и уменьшения
/// размера основной структуры.
pub struct RenderCache {
    /// Кэшированная строка счёта для оптимизации отрисовки.
    pub cached_score_str: String,
    /// Кэшированная строка уровня для оптимизации отрисовки.
    pub cached_level_str: String,
    /// Кэшированная строка количества линий для оптимизации отрисовки.
    pub cached_lines_str: String,
    /// Кэшированная строка рекорда для оптимизации отрисовки.
    pub cached_high_score_str: String,
    /// Кэшированная строка комбо для оптимизации отрисовки.
    pub cached_combo_str: String,
    /// Кэшированная строка таймера для оптимизации отрисовки.
    pub cached_timer_str: String,

    /// Последнее закэшированное значение счёта.
    pub last_cached_score: u128,
    /// Последнее закэшированное значение уровня.
    pub last_cached_level: u32,
    /// Последнее закэшированное значение количества линий.
    pub last_cached_lines: u32,
    /// Последнее закэшированное значение рекорда.
    pub last_cached_high_score: u128,
    /// Последнее закэшированное значение комбо.
    pub last_cached_combo: u32,
}

impl RenderCache {
    /// Создать новый кэш для отрисовки.
    ///
    /// # Исправление #4 (LOW)
    /// Используется `String::with_capacity(32)` для всех кэшированных строк
    /// для предотвращения лишних аллокаций при форматировании.
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
        }
    }

    /// Инициализация кэша начальными значениями.
    ///
    /// # Аргументы
    /// * `score` - начальный счёт
    /// * `level` - начальный уровень
    /// * `lines` - начальное количество линий
    /// * `high_score` - начальный рекорд
    pub fn init_with_values(&mut self, score: u128, level: u32, lines: u32, high_score: u128) {
        self.last_cached_score = score;
        self.last_cached_level = level;
        self.last_cached_lines = lines;
        self.last_cached_high_score = high_score;
        self.cached_score_str = score.to_string();
        self.cached_level_str = level.to_string();
        self.cached_lines_str = lines.to_string();
        self.cached_high_score_str = high_score.to_string();
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
        assert_eq!(cache.cached_score_str, "100");
        assert_eq!(cache.cached_level_str, "2");
        assert_eq!(cache.cached_lines_str, "10");
        assert_eq!(cache.cached_high_score_str, "500");
    }
}
