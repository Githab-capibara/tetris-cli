//! Модуль кэширования для отрисовки.
//!
//! # Ответственность
//! - Обновление кэшированных строк UI (счёт, уровень, линии, комбо, рекорд, таймер)
//! - Оптимизация аллокаций при форматировании строк
//!
//! ## Архитектурные заметки
//! Архитектурное улучшение 2026-04-01 (SOC2): Выделение функций обновления кэша
//! из `render.rs` для улучшения разделения ответственности.

use super::super::state::GameState;
use std::fmt::Write;

/// Обновить кэшированные строки для отрисовки.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
///
/// # Исправление #4 (LOW): Оптимизация кэширования строк
/// - Используется `String::with_capacity(32)` в `RenderCache::new()` для предотвращения аллокаций
/// - Используется `clear()` + `write!()` для переиспользования буфера без deallocation
fn update_cached_strings(state: &mut GameState) {
    let score = state.score();
    let level = state.level();
    let lines_cleared = state.lines_cleared();
    let render_cache = state.get_render_cache_mut();

    if score != render_cache.last_cached_score {
        render_cache.cached_score_str.clear();
        if let Err(e) = write!(render_cache.cached_score_str, "{score:10}") {
            eprintln!("Ошибка записи кэша счёта: {e}");
        }
        render_cache.last_cached_score = score;
    }

    if level != render_cache.last_cached_level {
        render_cache.cached_level_str.clear();
        if let Err(e) = write!(render_cache.cached_level_str, "{level:10}") {
            eprintln!("Ошибка записи кэша уровня: {e}");
        }
        render_cache.last_cached_level = level;
    }

    if lines_cleared != render_cache.last_cached_lines {
        render_cache.cached_lines_str.clear();
        if let Err(e) = write!(render_cache.cached_lines_str, "{lines_cleared:10}") {
            eprintln!("Ошибка записи кэша линий: {e}");
        }
        render_cache.last_cached_lines = lines_cleared;
    }
}

/// Обновить кэшированные строки (расширенная версия).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `high_score_display` - строка рекорда для кэширования
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
///
/// # Исправление #4 (LOW): Оптимизация кэширования строк
/// - Используется `String::with_capacity(32)` в `RenderCache::new()` для предотвращения аллокаций
/// - Используется `clear()` + `write!()` для переиспользования буфера без deallocation
pub fn update_cached_strings_extended(state: &mut GameState, high_score_display: &str) {
    update_cached_strings(state);

    // Исправление проблемы 26: объединяем все обновления в один блок с одним получением render_cache
    let combo_counter = state.stats().combo_counter();
    let is_sprint_mode = state.get_mode_trait().get_target_lines() == Some(40);
    // Вычисляем elapsed ДО получения mutable borrow (проблема 26: borrow conflict)
    let sprint_elapsed = if is_sprint_mode {
        Some(state.stats().get_elapsed_time())
    } else {
        None
    };

    {
        let render_cache = state.get_render_cache_mut();

        // Кэширование строки рекорда
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_high_score_str != high_score_display {
            render_cache.cached_high_score_str.clear();
            render_cache
                .cached_high_score_str
                .push_str(high_score_display);
        }

        // Кэширование строки комбо
        // Исправление #4: используем clear() + write!() для переиспользования буфера
        if render_cache.last_cached_combo != combo_counter {
            render_cache.cached_combo_str.clear();
            if combo_counter > 1 {
                let _ = write!(render_cache.cached_combo_str, "Комбо: x{combo_counter}");
            }
            render_cache.last_cached_combo = combo_counter;
        }

        // Кэширование строки таймера для режима спринт
        if is_sprint_mode {
            let elapsed = sprint_elapsed.unwrap_or(0.0);
            // Сравниваем до форматирования — избегаем аллокации если значение не изменилось
            let timer_matches = render_cache.last_cached_timer == (elapsed * 100.0).round() as i64;
            if !timer_matches {
                // Исправление проблемы 25: write! напрямую в буфер вместо format!
                render_cache.cached_timer_str.clear();
                let _ = write!(render_cache.cached_timer_str, "Время: {elapsed:.2}с");
                render_cache.last_cached_timer = (elapsed * 100.0).round() as i64;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::cache::RenderCache;

    #[test]
    fn test_render_cache_new() {
        let cache = RenderCache::new();
        assert!(cache.cached_score_str.is_empty());
        assert!(cache.cached_level_str.is_empty());
        assert!(cache.cached_lines_str.is_empty());
        assert!(cache.cached_high_score_str.is_empty());
        assert!(cache.cached_combo_str.is_empty());
        assert!(cache.cached_timer_str.is_empty());
        assert_eq!(cache.last_cached_score, 0);
        assert_eq!(cache.last_cached_level, 0);
        assert_eq!(cache.last_cached_lines, 0);
        assert_eq!(cache.last_cached_high_score, 0);
        assert_eq!(cache.last_cached_combo, 0);
        assert_eq!(cache.last_cached_timer, 0);
    }
}
