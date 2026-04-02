//! Статистика игры.
//!
//! Модуль содержит структуру `GameStats` для хранения статистики прошедшей игры:
//! - Количество использованных фигур каждого типа
//! - Общее количество очищенных линий
//! - Максимальное комбо
//! - Время игры
//!
//! ## Архитектурные заметки
//! Выделено из `state.rs` для улучшения организации кода и разделения ответственности.

#![allow(dead_code)]

use crate::tetromino::ShapeType;
use std::time::Instant;

/// Статистика игры.
///
/// Содержит подробную информацию о прошедшей игре:
/// - Количество использованных фигур каждого типа
/// - Общее количество очищенных линий
/// - Максимальное комбо
/// - Время игры
///
/// # Потокобезопасность
/// ## Текущая реализация
/// - **НЕ является `Send` или `Sync`**: Эта структура не предназначена для многопоточного доступа
/// - **Однопоточное использование**: Безопасно использовать в основном потоке игры
/// - **Внутренняя мутабельность**: Отсутствует, требует `&mut self` для изменения
///
/// ## Рекомендации по использованию
/// ### Однопоточный код (рекомендуется)
/// ```ignore
/// let mut stats = GameStats::new();
/// stats.add_piece(ShapeType::T);
/// ```
///
/// ### Многопоточный код (требует синхронизации)
/// ```ignore
/// use std::sync::{Arc, Mutex};
/// let stats = Arc::new(Mutex::new(GameStats::new()));
/// stats.lock().unwrap().add_piece(ShapeType::T);
/// ```
///
/// # Инкапсуляция (Исправление #2 - MEDIUM SEVERITY)
/// Поля структуры сделаны полностью приватными для улучшения инкапсуляции.
/// Доступ к полям осуществляется только через геттеры/сеттеры.
/// Это предотвращает прямое изменение полей извне модуля и позволяет
/// добавлять валидацию при изменении значений.
///
/// # Требования к валидации
/// - Все счётчики фигур не могут быть отрицательными (гарантируется типом u32)
/// - `max_combo` не может уменьшаться при обновлении
/// - `start_time` должен быть раньше `end_time` если оба установлены
#[derive(Default, Clone)]
pub struct GameStats {
    /// Количество фигур типа T.
    t_pieces: u32,
    /// Количество фигур типа L.
    l_pieces: u32,
    /// Количество фигур типа J.
    j_pieces: u32,
    /// Количество фигур типа S.
    s_pieces: u32,
    /// Количество фигур типа Z.
    z_pieces: u32,
    /// Количество фигур типа O.
    o_pieces: u32,
    /// Количество фигур типа I.
    i_pieces: u32,
    /// Максимальное комбо (одновременное удаление линий).
    max_combo: u32,
    /// Текущее комбо (последовательные удаления в нескольких ходах).
    combo_counter: u32,
    /// Время начала игры.
    start_time: Option<Instant>,
    /// Время окончания игры.
    end_time: Option<Instant>,
}

impl GameStats {
    /// Создать новую статистику.
    pub fn new() -> Self {
        Self::default()
    }

    // ========================================================================
    // ГЕТТЕРЫ ДЛЯ ПОЛЕЙ
    // ========================================================================

    /// Получить количество фигур типа T.
    #[must_use]
    #[inline]
    pub fn t_pieces(&self) -> u32 {
        self.t_pieces
    }

    /// Получить количество фигур типа L.
    #[must_use]
    #[inline]
    pub fn l_pieces(&self) -> u32 {
        self.l_pieces
    }

    /// Получить количество фигур типа J.
    #[must_use]
    #[inline]
    pub fn j_pieces(&self) -> u32 {
        self.j_pieces
    }

    /// Получить количество фигур типа S.
    #[must_use]
    #[inline]
    pub fn s_pieces(&self) -> u32 {
        self.s_pieces
    }

    /// Получить количество фигур типа Z.
    #[must_use]
    #[inline]
    pub fn z_pieces(&self) -> u32 {
        self.z_pieces
    }

    /// Получить количество фигур типа O.
    #[must_use]
    #[inline]
    pub fn o_pieces(&self) -> u32 {
        self.o_pieces
    }

    /// Получить количество фигур типа I.
    #[must_use]
    #[inline]
    pub fn i_pieces(&self) -> u32 {
        self.i_pieces
    }

    /// Получить максимальное комбо.
    #[must_use]
    #[inline]
    pub fn max_combo(&self) -> u32 {
        self.max_combo
    }

    /// Получить текущее комбо.
    #[must_use]
    #[inline]
    pub fn combo_counter(&self) -> u32 {
        self.combo_counter
    }

    /// Получить время начала игры.
    #[must_use]
    #[inline]
    pub fn start_time(&self) -> Option<Instant> {
        self.start_time
    }

    /// Получить время окончания игры.
    #[must_use]
    #[inline]
    pub fn end_time(&self) -> Option<Instant> {
        self.end_time
    }

    // ========================================================================
    // СЕТТЕРЫ ДЛЯ ПОЛЕЙ С ВАЛИДАЦИЕЙ
    // ========================================================================
    // Сеттеры предназначены для внутреннего использования и тестов.
    // Все сеттеры включают валидацию диапазонов для предотвращения некорректных значений.

    /// Разумный предел для счётчиков фигур.
    const MAX_PIECES: u32 = 10_000;

    /// Разумный предел для комбо.
    const MAX_COMBO: u32 = 1_000;

    /// Установить количество фигур типа T.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_t_pieces(&mut self, value: u32) {
        self.t_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа L.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_l_pieces(&mut self, value: u32) {
        self.l_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа J.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_j_pieces(&mut self, value: u32) {
        self.j_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа S.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_s_pieces(&mut self, value: u32) {
        self.s_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа Z.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_z_pieces(&mut self, value: u32) {
        self.z_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа O.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_o_pieces(&mut self, value: u32) {
        self.o_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить количество фигур типа I.
    ///
    /// # Валидация
    /// Значение должно быть в разумных пределах (менее 10000 для предотвращения ошибок)
    pub fn set_i_pieces(&mut self, value: u32) {
        self.i_pieces = value.min(Self::MAX_PIECES);
    }

    /// Установить максимальное комбо.
    ///
    /// # Валидация
    /// Комбо не может быть отрицательным (гарантируется типом u32)
    /// и ограничено разумным пределом в 1000
    pub fn set_max_combo(&mut self, value: u32) {
        self.max_combo = value.min(Self::MAX_COMBO);
    }

    /// Установить текущее комбо.
    ///
    /// # Валидация
    /// Комбо не может быть отрицательным (гарантируется типом u32)
    pub fn set_combo_counter(&mut self, value: u32) {
        self.combo_counter = value;
    }

    /// Установить время начала игры.
    ///
    /// # Валидация
    /// Если `end_time` уже установлен, `start_time` не может быть позже него
    pub fn set_start_time(&mut self, value: Option<Instant>) {
        // Валидация: start_time не должен быть позже end_time если оба установлены
        if let Some(start) = value {
            if let Some(end) = self.end_time {
                if start > end {
                    // Если start позже end, сбрасываем end
                    self.end_time = None;
                }
            }
        }
        self.start_time = value;
    }

    /// Установить время окончания игры.
    ///
    /// # Валидация
    /// Если `start_time` установлен, `end_time` не может быть раньше него
    pub fn set_end_time(&mut self, value: Option<Instant>) {
        // Валидация: end_time не должен быть раньше start_time если оба установлены
        if let Some(end) = value {
            if let Some(start) = self.start_time {
                if end < start {
                    // Если end раньше start, не устанавливаем end
                    return;
                }
            }
        }
        self.end_time = value;
    }

    /// Увеличить счётчик для указанной фигуры.
    pub fn add_piece(&mut self, piece_type: ShapeType) {
        match piece_type {
            ShapeType::T => self.t_pieces += 1,
            ShapeType::L => self.l_pieces += 1,
            ShapeType::J => self.j_pieces += 1,
            ShapeType::S => self.s_pieces += 1,
            ShapeType::Z => self.z_pieces += 1,
            ShapeType::O => self.o_pieces += 1,
            ShapeType::I => self.i_pieces += 1,
        }
    }

    /// Получить общее количество использованных фигур.
    #[must_use]
    pub fn total_pieces(&self) -> u32 {
        self.t_pieces
            + self.l_pieces
            + self.j_pieces
            + self.s_pieces
            + self.z_pieces
            + self.o_pieces
            + self.i_pieces
    }

    /// Обновить максимальное комбо.
    pub fn update_max_combo(&mut self, lines: u32) {
        if lines > self.max_combo {
            self.max_combo = lines;
        }
    }

    /// Получить время игры в секундах.
    #[must_use]
    pub fn get_elapsed_time(&self) -> f64 {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f64(),
            (Some(start), None) => Instant::now().duration_since(start).as_secs_f64(),
            _ => 0.0,
        }
    }

    /// Начать отсчёт времени.
    pub fn start_timer(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Остановить отсчёт времени.
    pub fn stop_timer(&mut self) {
        self.end_time = Some(Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_stats_new() {
        let stats = GameStats::new();
        assert_eq!(stats.total_pieces(), 0);
        assert_eq!(stats.max_combo(), 0);
    }

    #[test]
    fn test_game_stats_add_piece() {
        let mut stats = GameStats::new();
        stats.add_piece(ShapeType::T);
        stats.add_piece(ShapeType::T);
        stats.add_piece(ShapeType::I);
        assert_eq!(stats.t_pieces(), 2);
        assert_eq!(stats.i_pieces(), 1);
        assert_eq!(stats.total_pieces(), 3);
    }

    #[test]
    fn test_game_stats_timer() {
        let mut stats = GameStats::new();
        stats.start_timer();
        assert!(stats.start_time().is_some());
        stats.stop_timer();
        assert!(stats.end_time().is_some());
    }

    #[test]
    fn test_game_stats_combo() {
        let mut stats = GameStats::new();
        stats.update_max_combo(3);
        assert_eq!(stats.max_combo(), 3);
        stats.update_max_combo(2);
        assert_eq!(stats.max_combo(), 3); // Не уменьшилось
        stats.update_max_combo(5);
        assert_eq!(stats.max_combo(), 5);
    }
}
