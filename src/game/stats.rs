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
/// # Инкапсуляция (Задача 4 CRITICAL)
/// Поля структуры сделаны приватными для улучшения инкапсуляции.
/// Доступ к полям осуществляется через геттеры/сеттеры.
#[derive(Default, Clone)]
pub struct GameStats {
    /// Количество фигур типа T.
    pub(crate) t_pieces: u32,
    /// Количество фигур типа L.
    pub(crate) l_pieces: u32,
    /// Количество фигур типа J.
    pub(crate) j_pieces: u32,
    /// Количество фигур типа S.
    pub(crate) s_pieces: u32,
    /// Количество фигур типа Z.
    pub(crate) z_pieces: u32,
    /// Количество фигур типа O.
    pub(crate) o_pieces: u32,
    /// Количество фигур типа I.
    pub(crate) i_pieces: u32,
    /// Максимальное комбо (одновременное удаление линий).
    pub(crate) max_combo: u32,
    /// Текущее комбо (последовательные удаления в нескольких ходах).
    pub(crate) combo_counter: u32,
    /// Время начала игры.
    pub(crate) start_time: Option<Instant>,
    /// Время окончания игры.
    pub(crate) end_time: Option<Instant>,
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
    pub fn t_pieces(&self) -> u32 {
        self.t_pieces
    }

    /// Получить количество фигур типа L.
    #[must_use]
    pub fn l_pieces(&self) -> u32 {
        self.l_pieces
    }

    /// Получить количество фигур типа J.
    #[must_use]
    pub fn j_pieces(&self) -> u32 {
        self.j_pieces
    }

    /// Получить количество фигур типа S.
    #[must_use]
    pub fn s_pieces(&self) -> u32 {
        self.s_pieces
    }

    /// Получить количество фигур типа Z.
    #[must_use]
    pub fn z_pieces(&self) -> u32 {
        self.z_pieces
    }

    /// Получить количество фигур типа O.
    #[must_use]
    pub fn o_pieces(&self) -> u32 {
        self.o_pieces
    }

    /// Получить количество фигур типа I.
    #[must_use]
    pub fn i_pieces(&self) -> u32 {
        self.i_pieces
    }

    /// Получить максимальное комбо.
    #[must_use]
    pub fn max_combo(&self) -> u32 {
        self.max_combo
    }

    /// Получить текущее комбо.
    #[must_use]
    pub fn combo_counter(&self) -> u32 {
        self.combo_counter
    }

    /// Получить время начала игры.
    #[must_use]
    pub fn start_time(&self) -> Option<Instant> {
        self.start_time
    }

    /// Получить время окончания игры.
    #[must_use]
    pub fn end_time(&self) -> Option<Instant> {
        self.end_time
    }

    // ========================================================================
    // СЕТТЕРЫ ДЛЯ ПОЛЕЙ
    // ========================================================================
    // Сеттеры предназначены для внутреннего использования и тестов.

    /// Установить количество фигур типа T.
    pub fn set_t_pieces(&mut self, value: u32) {
        self.t_pieces = value;
    }

    /// Установить количество фигур типа L.
    pub fn set_l_pieces(&mut self, value: u32) {
        self.l_pieces = value;
    }

    /// Установить количество фигур типа J.
    pub fn set_j_pieces(&mut self, value: u32) {
        self.j_pieces = value;
    }

    /// Установить количество фигур типа S.
    pub fn set_s_pieces(&mut self, value: u32) {
        self.s_pieces = value;
    }

    /// Установить количество фигур типа Z.
    pub fn set_z_pieces(&mut self, value: u32) {
        self.z_pieces = value;
    }

    /// Установить количество фигур типа O.
    pub fn set_o_pieces(&mut self, value: u32) {
        self.o_pieces = value;
    }

    /// Установить количество фигур типа I.
    pub fn set_i_pieces(&mut self, value: u32) {
        self.i_pieces = value;
    }

    /// Установить максимальное комбо.
    pub fn set_max_combo(&mut self, value: u32) {
        self.max_combo = value;
    }

    /// Установить текущее комбо.
    pub fn set_combo_counter(&mut self, value: u32) {
        self.combo_counter = value;
    }

    /// Установить время начала игры.
    pub fn set_start_time(&mut self, value: Option<Instant>) {
        self.start_time = value;
    }

    /// Установить время окончания игры.
    pub fn set_end_time(&mut self, value: Option<Instant>) {
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
