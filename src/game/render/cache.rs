//! Модуль кэширования для отрисовки.
//!
//! # Ответственность
//! - Обновление кэшированных строк UI (счёт, уровень, линии, комбо, рекорд, таймер)
//! - Оптимизация аллокаций при форматировании строк
//!
//! ## Архитектурные заметки
//! Архитектурное улучшение 2026-04-01 (SOC2): Выделение функций обновления кэша
//! из `render.rs` для улучшения разделения ответственности.

#![allow(dead_code)]

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
        if let Err(e) = write!(render_cache.cached_score_str, "{:10}", score) {
            eprintln!("Ошибка записи кэша счёта: {e}");
        }
        render_cache.last_cached_score = score;
    }

    if level != render_cache.last_cached_level {
        render_cache.cached_level_str.clear();
        if let Err(e) = write!(render_cache.cached_level_str, "{:10}", level) {
            eprintln!("Ошибка записи кэша уровня: {e}");
        }
        render_cache.last_cached_level = level;
    }

    if lines_cleared != render_cache.last_cached_lines {
        render_cache.cached_lines_str.clear();
        if let Err(e) = write!(render_cache.cached_lines_str, "{:10}", lines_cleared) {
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

    // Кэширование строки рекорда
    {
        let render_cache = state.get_render_cache_mut();
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_high_score_str != high_score_display {
            render_cache.cached_high_score_str.clear();
            render_cache
                .cached_high_score_str
                .push_str(high_score_display);
        }
    }

    // Кэширование строки комбо
    let combo_counter = state.stats().combo_counter();
    {
        let render_cache = state.get_render_cache_mut();
        if render_cache.last_cached_combo != combo_counter {
            // Исправление #4: используем clear() + write!() для переиспользования буфера
            render_cache.cached_combo_str.clear();
            if combo_counter > 1 {
                let _ = write!(render_cache.cached_combo_str, "Комбо: x{combo_counter}");
            }
            render_cache.last_cached_combo = combo_counter;
        }
    }

    // Кэширование строки таймера для режима спринт
    if state.get_mode_trait().get_target_lines() == Some(40) {
        let elapsed = state.stats().get_elapsed_time();
        let timer_str = format!("Время: {elapsed:.2}с");
        let render_cache = state.get_render_cache_mut();
        // Исправление #4: используем clear() + push_str() для переиспользования буфера
        if render_cache.cached_timer_str != timer_str {
            render_cache.cached_timer_str.clear();
            render_cache.cached_timer_str.push_str(&timer_str);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_update_cached_strings_exists() {
        // Тест существует для проверки что модуль компилируется
        // Фактическое тестирование требует GameState
    }
}
