//! Утилиты для контроля FPS и таймингов кадров.
//!
//! # Ответственность
//! - Централизованная логика поддержания стабильного FPS
//! - Устранение дублирования между игровым циклом и циклом меню
//!
//! ## Архитектурные заметки
//! Выделено 2026-04-11 (Пакет 2, #21): `maintain_fps` из `cycle.rs` и
//! `wait_for_next_frame` из `app/mod.rs` дублировали идентичную логику.

use std::thread::sleep;
use std::time::{Duration, Instant};

/// Поддержать стабильный FPS.
///
/// Вычисляет время с последнего кадра и, если прошло меньше `interval_ms`,
/// приостанавливает выполнение на оставшееся время.
///
/// # Аргументы
/// * `last_time` — время последнего кадра (изменяемое)
/// * `interval_ms` — целевой интервал между кадрами (мс)
///
/// # Возвращает
/// - `Some(delta_time_ms)` — время прошло между кадрами, можно обновлять кадр
/// - `None` — нужно продолжить ожидание (`delta_time_ms` < `interval_ms`)
///
/// # Исправление аудита 2026-04-11 (Пакет 2, #21)
/// Общая функция для игрового цикла и цикла меню.
/// Использует `try_into().unwrap_or(0)` для безопасной конвертации u128 → u64.
#[must_use = "Результат контроля FPS должен быть использован для решения, обновлять ли кадр"]
pub fn maintain_fps(last_time: &mut Instant, interval_ms: u64) -> Option<u64> {
    let now = Instant::now();
    // Безопасная конвертация u128 -> u64
    // unwrap_or(0): если delta > u64::MAX (практически невозможно),
    // используем 0 — sleep(interval_ms) обеспечит корректную задержку
    let delta_time_ms: u64 = now
        .duration_since(*last_time)
        .as_millis()
        .try_into()
        .unwrap_or(0);

    if delta_time_ms < interval_ms {
        sleep(Duration::from_millis(
            interval_ms.saturating_sub(delta_time_ms),
        ));
        return None;
    }

    *last_time = now;
    Some(delta_time_ms)
}
