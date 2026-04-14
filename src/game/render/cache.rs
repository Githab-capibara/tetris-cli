//! Кэширование отрисовки.
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
/// Обновляет строки счёта, уровня и линий если значения изменились.
/// Использует `clear()` + `write!()` для переиспользования буфера без аллокаций.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
fn update_cached_strings(state: &mut GameState) {
    let score = state.score();
    let level = state.level();
    let lines_cleared = state.lines_cleared();
    let render_cache = state.get_render_cache_mut();

    if score != render_cache.last_cached_score {
        render_cache.cached_score_str.clear();
        let _ = write!(render_cache.cached_score_str, "{score:10}");
        render_cache.last_cached_score = score;
    }

    if level != render_cache.last_cached_level {
        render_cache.cached_level_str.clear();
        let _ = write!(render_cache.cached_level_str, "{level:10}");
        render_cache.last_cached_level = level;
    }

    if lines_cleared != render_cache.last_cached_lines {
        render_cache.cached_lines_str.clear();
        let _ = write!(render_cache.cached_lines_str, "{lines_cleared:10}");
        render_cache.last_cached_lines = lines_cleared;
    }
}

/// Обновить кэшированные строки (расширенная версия).
///
/// Обновляет все строки UI включая рекорд, комбо и таймер (в режиме спринт).
/// Использует `clear()` + `write!()` для переиспользования буфера без аллокаций.
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `high_score_display` - строка рекорда для кэширования
///
/// ## Примечания
/// Эта функция требует mutable доступ к `GameState`, поэтому не может
/// использовать `GameView`. Вызывайте её перед созданием `GameView`.
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
            // cast: f64 -> i64, потеря точности допустима: таймер в разумных пределах
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let timer_value = (elapsed * 100.0).round() as i64;
            let timer_matches = render_cache.last_cached_timer == timer_value;
            if !timer_matches {
                // Исправление проблемы 25: write! напрямую в буфер вместо format!
                render_cache.cached_timer_str.clear();
                let _ = write!(render_cache.cached_timer_str, "Время: {elapsed:.2}с");
                render_cache.last_cached_timer = timer_value;
            }
        }
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Тест: `update_cached_strings` обновляет кэш счёта при изменении
    #[test]
    fn test_update_cached_strings_score_changed() {
        let mut state = GameState::new();
        state.set_score(12345);

        update_cached_strings(&mut state);

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_score_str, "     12345");
        assert_eq!(cache.last_cached_score, 12345);
    }

    /// Тест: `update_cached_strings` не обновляет кэш при том же счёте
    #[test]
    fn test_update_cached_strings_score_unchanged() {
        let mut state = GameState::new();
        state.set_score(100);
        // Первый вызов — кэшируем
        update_cached_strings(&mut state);
        let cache_before = state.get_render_cache();
        let score_str_before = cache_before.cached_score_str.clone();

        // Второй вызов — значение не изменилось
        update_cached_strings(&mut state);

        let cache_after = state.get_render_cache();
        assert_eq!(cache_after.cached_score_str, score_str_before);
    }

    /// Тест: `update_cached_strings` обновляет кэш уровня
    #[test]
    fn test_update_cached_strings_level() {
        let mut state = GameState::new();
        state.set_level(15);

        update_cached_strings(&mut state);

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_level_str, "        15");
        assert_eq!(cache.last_cached_level, 15);
    }

    /// Тест: `update_cached_strings` обновляет кэш линий
    #[test]
    fn test_update_cached_strings_lines() {
        let mut state = GameState::new();
        state.set_lines_cleared(42);

        update_cached_strings(&mut state);

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_lines_str, "        42");
        assert_eq!(cache.last_cached_lines, 42);
    }

    /// Тест: `update_cached_strings_extended` обновляет кэш рекорда
    #[test]
    fn test_update_cached_strings_extended_high_score() {
        let mut state = GameState::new();
        let high_score = "      5000";

        update_cached_strings_extended(&mut state, high_score);

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_high_score_str, high_score);
    }

    /// Тест: `update_cached_strings_extended` обновляет кэш комбо
    #[test]
    fn test_update_cached_strings_extended_combo() {
        let mut state = GameState::new();
        state.stats_mut().set_combo_counter(5);

        update_cached_strings_extended(&mut state, "         0");

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_combo_str, "Комбо: x5");
        assert_eq!(cache.last_cached_combo, 5);
    }

    /// Тест: `update_cached_strings_extended` не кэширует комбо при значении <= 1
    #[test]
    fn test_update_cached_strings_extended_no_combo_when_one() {
        let mut state = GameState::new();
        state.stats_mut().set_combo_counter(1);

        update_cached_strings_extended(&mut state, "         0");

        let cache = state.get_render_cache();
        assert_eq!(cache.cached_combo_str, "");
    }

    /// Тест: `update_cached_strings_extended` кэширует таймер в режиме спринт
    #[test]
    fn test_update_cached_strings_extended_sprint_timer() {
        let mut state = GameState::new_sprint();

        // Сбрасываем last_cached_timer чтобы гарантировать обновление
        state.get_render_cache_mut().last_cached_timer = -1;

        update_cached_strings_extended(&mut state, "         0");

        let cache = state.get_render_cache();
        // Таймер должен быть закэширован с начальным временем ~0
        assert!(cache.cached_timer_str.starts_with("Время: "));
    }

    /// Тест: `update_cached_strings_extended` не кэширует таймер в классическом режиме
    #[test]
    fn test_update_cached_strings_extended_no_timer_classic() {
        let mut state = GameState::new();

        update_cached_strings_extended(&mut state, "         0");

        let cache = state.get_render_cache();
        // В классическом режиме таймер не кэшируется
        assert_eq!(cache.cached_timer_str, "");
    }
}
