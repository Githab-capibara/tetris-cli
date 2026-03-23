//! Основной игровой модуль Tetris CLI.
//!
//! Этот модуль содержит всю игровую логику Tetris и разделён на подмодули:
//! - [`state`] — структуры состояния игры (GameState, GameStats, GameMode)
//! - [`logic`] — игровая логика (физика, коллизии, движение, ввод)
//! - [`scoring`] — система очков и уровней
//! - [`render`] — отрисовка и анимации
//!
//! ## Пример использования
//!
//! ```ignore
//! use tetris_cli::game::GameState;
//!
//! // Создание нового состояния игры
//! let mut game = GameState::new();
//!
//! // Запуск игрового цикла
//! // game.play(&mut cnv, &mut inp, &high_score_display);
//! ```

// Подмодули
pub mod logic;
pub mod render;
pub mod scoring;
pub mod state;

// Re-export основных типов для обратной совместимости
pub use state::{
    Achievement,
    GameMode,
    GameState,
    GameStats,
    ANIMATION_FRAME_SKIP,
    BELL,
    BORDER,
    COMBO_BONUS,
    DRAW_OFFSET_X,
    FIELD_HEIGHT,
    FIELD_OFFSET_X,
    FIELD_WIDTH,
    // Константы
    FPS,
    GAME_OVER,
    GAME_OVER_DELAY_MS,
    HARD_DROP_ANIM_INTERVAL_MS,
    HARD_DROP_POINTS,
    HIGH_SCORE_X,
    HIGH_SCORE_Y,
    HOLD_PREVIEW_X,
    HOLD_PREVIEW_Y,
    INITIAL_FALL_SPD,
    LAND_TIME_DELAY_S,
    LEVEL_BONUS_MULT,
    LEVEL_X,
    LEVEL_Y,
    LINES_PER_LEVEL,
    LINES_X,
    LINES_Y,
    LINE_SCORES,
    MARATHON_LINES,
    MAX_FALL_SPEED,
    MAX_LINES_PER_CLEAR,
    MILLIS_PER_SECOND,
    MIN_Y,
    PAUSE,
    PIECE_SCORE_FALL_MULT,
    PIECE_SCORE_INC,
    PREVIEW_X,
    PREVIEW_Y,
    SCORE_X,
    SCORE_Y,
    SHAPE_DRAW_OFFSET,
    SHAPE_OFFSET_X,
    SHAPE_OFFSET_Y,
    SOFT_DROP_POINTS,
    SPD_INC,
    SPRINT_LINES,
};

pub use logic::{
    can_move_curr_shape_direction, can_rotate_curr_shape, handle_falling, handle_input,
    rotate_with_wall_kick, save_tetromino, update, WALL_KICK_OFFSETS,
};

pub use scoring::{
    find_full_rows, handle_hard_drop, handle_hold, handle_landing, handle_soft_drop, remove_rows,
    update_score_and_level,
};

pub use render::{animate_clear, check_rows, draw};

// ============================================================================
// МЕТОДЫ ДЛЯ GameState
// ============================================================================
// Расширяем GameState методами из подмодулей

impl GameState {
    /// Запустить игровой цикл и вернуть финальный счёт.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки
    /// * `inp` - читатель нажатий клавиш
    /// * `high_score_display` - строка для отображения рекорда
    ///
    /// # Возвращает
    /// Финальный счёт игрока
    pub fn play(
        &mut self,
        cnv: &mut crate::io::Canvas,
        inp: &mut crate::io::KeyReader,
        high_score_display: &str,
    ) -> u128 {
        use std::{
            thread::sleep,
            time::{Duration, Instant},
        };

        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;

        loop {
            // Поддержание стабильного FPS
            let now = Instant::now();
            let delta_time_ms = now.duration_since(last_time).as_millis() as u64;
            if delta_time_ms < interval_ms {
                sleep(Duration::from_millis(interval_ms - delta_time_ms));
                continue;
            }
            last_time = now;

            // Обновление состояния игры
            match update(self, inp, delta_time_ms) {
                UpdateEndState::Continue => {}
                UpdateEndState::Quit => {
                    return 0;
                }
                UpdateEndState::Lost => {
                    cnv.draw_strs(
                        &GAME_OVER,
                        (10, 12),
                        state::BORDER_COLOR,
                        &termion::color::Reset,
                    );
                    cnv.flush();
                    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
                    break;
                }
                UpdateEndState::Pause => loop {
                    let key = inp.get_key();
                    match key {
                        Some(b'p') => break,
                        Some(crate::io::KEY_BACKSPACE) => {
                            cnv.draw_strs(
                                &PAUSE,
                                (7, 13),
                                state::BORDER_COLOR,
                                &termion::color::Reset,
                            );
                            return 0;
                        }
                        Some(_) | None => {}
                    }
                    cnv.draw_strs(&PAUSE, (7, 13), state::BORDER_COLOR, &termion::color::Reset);
                    sleep(Duration::from_millis(interval_ms));
                },
                UpdateEndState::Won => {
                    cnv.draw_strs(
                        &GAME_OVER,
                        (10, 12),
                        state::BORDER_COLOR,
                        &termion::color::Reset,
                    );
                    cnv.flush();
                    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
                    break;
                }
            }

            // Отрисовка текущего кадра
            draw(self, cnv, high_score_display);
        }

        self.score
    }

    /// Проверить заполненные линии и удалить их.
    ///
    /// # Возвращает
    /// Количество удалённых линий
    pub fn check_rows(&mut self) -> u32 {
        check_rows(self)
    }

    /// Сохранить текущую фигуру в сетке после приземления.
    pub fn save_tetromino(&mut self) {
        save_tetromino(self)
    }

    /// Проверить возможность движения текущей фигуры.
    ///
    /// # Аргументы
    /// * `dir` - направление движения
    ///
    /// # Возвращает
    /// `true` если движение возможно
    pub fn can_move_curr_shape_direction(&self, dir: crate::types::Direction) -> bool {
        can_move_curr_shape_direction(self, dir)
    }

    /// Проверить возможность вращения текущей фигуры.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения
    ///
    /// # Возвращает
    /// `true` если вращение возможно
    pub fn can_rotate_curr_shape(&self, dir: crate::types::RotationDirection) -> bool {
        can_rotate_curr_shape(self, dir)
    }

    /// Попытаться вратить фигуру со смещением (wall kick).
    ///
    /// # Аргументы
    /// * `dir` - направление вращения
    ///
    /// # Возвращает
    /// `true` если вращение успешно
    pub fn rotate_with_wall_kick(&mut self, dir: crate::types::RotationDirection) -> bool {
        rotate_with_wall_kick(self, dir)
    }

    /// Удержать текущую фигуру и получить следующую.
    pub fn hold_shape(&mut self) {
        handle_hold(self)
    }

    /// Запустить таймер.
    pub fn start_timer(&mut self) {
        self.stats.start_timer();
    }

    /// Остановить таймер.
    pub fn stop_timer(&mut self) {
        self.stats.stop_timer();
    }

    /// Получить доступ к игровому полю для бенчмарков.
    #[cfg(feature = "bench")]
    #[must_use]
    pub fn get_blocks_for_bench(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Заполнить указанную линию блоками для бенчмарков.
    #[cfg(feature = "bench")]
    pub fn fill_line_for_bench(&mut self, line: usize) {
        if line < GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.blocks[line][x] = 0;
            }
        }
    }

    /// Очистить заполненные линии для бенчмарков.
    #[cfg(feature = "bench")]
    pub fn clear_lines_for_bench(&mut self) {
        let (rows_mask, remove_count) = find_full_rows(&self.blocks);
        if rows_mask != 0 {
            remove_rows(&mut self.blocks, rows_mask);
        }
    }

    /// Сохранить текущую фигуру для бенчмарков.
    #[cfg(feature = "bench")]
    pub fn save_tetromino_for_bench(&mut self) {
        save_tetromino(self)
    }

    /// Установить текущую фигуру для бенчмарков.
    #[cfg(feature = "bench")]
    pub fn set_curr_shape_for_bench(&mut self, shape: crate::tetromino::Tetromino) {
        self.curr_shape = shape;
    }

    /// Проверить, может ли призрак двигаться в указанном направлении.
    ///
    /// Используется для отрисовки призрачной фигуры (предпросмотр приземления).
    pub fn can_move_ghost_shape_direction(&self, dir: crate::types::Direction) -> bool {
        can_move_curr_shape_direction(self, dir)
    }

    /// Получить значение флага возможности удержания фигуры.
    ///
    /// # Возвращает
    /// `true` если можно удержать текущую фигуру
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Увеличить счетчик очищенных линий.
    ///
    /// Используется в тестах для проверки обновления счетчика.
    pub fn increment_lines_cleared(&mut self) {
        self.lines_cleared = self.lines_cleared.saturating_add(1);
    }
}

// Импортируем типы из state для использования в impl
use crate::io::{GRID_HEIGHT, GRID_WIDTH};
use state::UpdateEndState;

// ============================================================================
// МОДУЛЬ ТЕСТОВ
// ============================================================================

#[cfg(test)]
mod game_tests {
    use super::*;
    use crate::Direction;

    // Тесты Hard Drop
    #[test]
    fn test_hard_drop_height_calculation() {
        let mut state = GameState::new();
        let mut drop_height = 0;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.curr_shape.pos.1 += 1.0;
            drop_height += 1;
        }

        assert!(drop_height > 0, "Фигура должна иметь возможность падения");
        assert!(
            !state.can_move_curr_shape_direction(Direction::Down),
            "После Hard Drop движение вниз должно быть заблокировано"
        );
    }

    #[test]
    fn test_hard_drop_bonus_points() {
        let mut state = GameState::new();
        let start_y = state.curr_shape.pos.1;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.curr_shape.pos.1 += 1.0;
        }

        let drop_distance = (state.curr_shape.pos.1 - start_y) as u64;

        assert_eq!(
            HARD_DROP_POINTS, 2,
            "Бонус за Hard Drop должен быть 2 очка за ячейку"
        );
        assert!(drop_distance > 0, "Дистанция должна быть положительной");
    }

    #[test]
    fn test_hard_drop_animation_frames() {
        let mut state = GameState::new();

        assert!(
            !state.is_hard_dropping,
            "До Hard Drop флаг должен быть false"
        );

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.curr_shape.pos.1 += 1.0;
        }
        state.is_hard_dropping = true;

        assert!(
            state.is_hard_dropping,
            "После Hard Drop флаг должен быть true для анимации"
        );

        state.is_hard_dropping = false;
        assert!(
            !state.is_hard_dropping,
            "После анимации флаг должен сбрасываться"
        );
    }

    #[test]
    fn test_hard_drop_boundary() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape.pos.1;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.curr_shape.pos.1 += 1.0;
        }

        assert!(
            state.curr_shape.pos.1 > initial_y,
            "Фигура должна опуститься после Hard Drop"
        );
        assert!(
            !state.can_move_curr_shape_direction(Direction::Down),
            "Движение вниз должно быть заблокировано после приземления"
        );
        assert!(
            state.curr_shape.pos.1 <= GRID_HEIGHT as f32,
            "Фигура не должна выходить за границы поля"
        );
    }

    // Тесты Soft Drop
    #[test]
    fn test_soft_drop_speed_increase() {
        let state = GameState::new();
        let initial_fall_spd = state.fall_spd;

        assert!(
            (initial_fall_spd - INITIAL_FALL_SPD).abs() < f32::EPSILON,
            "Начальная скорость должна быть INITIAL_FALL_SPD"
        );
        assert!(
            state.fall_spd > 0.0,
            "Скорость падения должна быть положительной"
        );
    }

    #[test]
    fn test_soft_drop_points_per_cell() {
        assert_eq!(
            SOFT_DROP_POINTS, 1,
            "Очки за Soft Drop должны быть 1 за ячейку"
        );

        let test_distances = [1u128, 5u128, 10u128, 15u128];
        for &distance in &test_distances {
            let expected_points = distance * SOFT_DROP_POINTS;
            assert_eq!(
                expected_points, distance,
                "Очки должны равняться дистанции × 1"
            );
        }
    }

    #[test]
    fn test_soft_drop_floor_detection() {
        let mut state = GameState::new();
        let mut soft_drop_moves = 0;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.curr_shape.pos.1 += 1.0;
            soft_drop_moves += 1;
        }

        assert!(
            soft_drop_moves > 0,
            "Фигура должна иметь возможность падения"
        );
        assert!(
            !state.can_move_curr_shape_direction(Direction::Down),
            "После достижения дна движение должно быть заблокировано"
        );
    }

    #[test]
    fn test_soft_drop_distance_tracking() {
        let mut state = GameState::new();

        assert_eq!(
            state.soft_drop_distance, 0,
            "Начальная дистанция Soft Drop должна быть 0"
        );

        let test_moves = 5;
        for _ in 0..test_moves {
            if state.can_move_curr_shape_direction(Direction::Down) {
                state.curr_shape.pos.1 += 1.0;
                state.soft_drop_distance += 1;
            }
        }

        assert_eq!(
            state.soft_drop_distance, test_moves,
            "Дистанция должна равняться количеству шагов"
        );

        state.soft_drop_distance = 0;
        assert_eq!(
            state.soft_drop_distance, 0,
            "После сброса дистанция должна быть 0"
        );
    }

    // Тесты Combo системы
    #[test]
    fn test_combo_counter_increment() {
        let mut stats = GameStats::new();

        assert_eq!(
            stats.combo_counter, 0,
            "Начальное значение комбо должно быть 0"
        );

        stats.combo_counter += 1;
        assert_eq!(
            stats.combo_counter, 1,
            "После первого удаления комбо должно быть 1"
        );

        stats.combo_counter += 1;
        assert_eq!(
            stats.combo_counter, 2,
            "После второго удаления комбо должно быть 2"
        );

        stats.combo_counter += 1;
        assert_eq!(
            stats.combo_counter, 3,
            "После третьего удаления комбо должно быть 3"
        );

        stats.update_max_combo(stats.combo_counter);
        assert_eq!(stats.max_combo, 3, "Максимальное комбо должно быть 3");
    }

    #[test]
    fn test_combo_bonus_calculation() {
        assert_eq!(COMBO_BONUS, 50, "Базовый бонус за комбо должен быть 50");

        let combo_bonus_level_1: u64 = 0;
        assert_eq!(
            combo_bonus_level_1, 0,
            "Бонус за первое комбо должен быть 0"
        );

        let combo_bonus_level_2 = COMBO_BONUS;
        assert_eq!(
            combo_bonus_level_2, 50,
            "Бонус за второе комбо должно быть 50"
        );

        let combo_bonus_level_3 = COMBO_BONUS * 2;
        assert_eq!(
            combo_bonus_level_3, 100,
            "Бонус за третье комбо должно быть 100"
        );

        let combo_bonus_level_5 = COMBO_BONUS * 4;
        assert_eq!(
            combo_bonus_level_5, 200,
            "Бонус за пятое комбо должно быть 200"
        );

        let combo_bonus_level_10 = COMBO_BONUS * 9;
        assert_eq!(
            combo_bonus_level_10, 450,
            "Бонус за десятое комбо должно быть 450"
        );
    }

    #[test]
    fn test_combo_reset_on_no_clear() {
        let mut stats = GameStats::new();

        stats.combo_counter = 3;
        assert_eq!(stats.combo_counter, 3, "Комбо должно быть 3");

        stats.combo_counter = 0;
        assert_eq!(
            stats.combo_counter, 0,
            "После хода без удаления комбо должно сбрасываться в 0"
        );

        stats.combo_counter += 1;
        assert_eq!(
            stats.combo_counter, 1,
            "После сброса новое комбо начинается с 1"
        );
    }

    #[test]
    fn test_tetris_bonus_1000() {
        const TETRIS_BONUS: u128 = 1000;

        let base_score_4_lines = LINE_SCORES[0] * (1 << (4 - 1));
        assert_eq!(
            base_score_4_lines, 800,
            "Базовые очки за 4 линии должны быть 800"
        );

        let total_tetris_score = base_score_4_lines + TETRIS_BONUS;
        assert_eq!(
            total_tetris_score, 1800,
            "Общий счёт за Tetris должен быть 1800"
        );

        let base_score_3_lines = LINE_SCORES[0] * (1 << (3 - 1));
        assert!(
            total_tetris_score > base_score_3_lines,
            "Tetris должен давать больше очков, чем 3 линии"
        );
    }

    // Тесты производительности
    #[test]
    fn test_performance_find_full_rows() {
        use std::time::Instant;

        let state = GameState::new();
        let start = Instant::now();

        for _ in 0..1000 {
            let (rows_mask, remove_count) = find_full_rows(state.get_blocks());
            assert_eq!(rows_mask, 0);
            assert_eq!(remove_count, 0);
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 10,
            "find_full_rows() должен выполняться < 10ms для 1000 итераций (прошло {elapsed:?})"
        );
    }

    #[test]
    fn test_performance_save_tetromino() {
        use std::time::Instant;

        let mut state = GameState::new();
        let start = Instant::now();

        for _ in 0..1000 {
            state.save_tetromino();
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 50,
            "save_tetromino() должен выполняться < 50ms для 1000 итераций (прошло {elapsed:?})"
        );
    }

    #[test]
    fn test_performance_check_collision_direction() {
        use std::time::Instant;

        let state = GameState::new();
        let start = Instant::now();

        for _ in 0..10000 {
            let result = can_move_curr_shape_direction(&state, Direction::Down);
            assert!(result);
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 100,
            "check_collision_direction() должен выполняться < 100ms для 10000 итераций (прошло {elapsed:?})"
        );
    }
}
