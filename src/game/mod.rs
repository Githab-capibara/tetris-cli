//! Основной игровой модуль Tetris CLI.
//!
//! Этот модуль содержит всю игровую логику Tetris и разделён на подмодули:
//! - [`state`] — структуры состояния игры ([`GameState`], [`GameStats`], [`GameMode`])
//! - [`logic`] — игровая логика (физика, коллизии, движение, ввод)
//! - [`scoring`] — система очков и уровней
//! - [`render`] — отрисовка и анимации
//! - [`view`] — представление игры для отрисовки ([`GameView`])
//! - [`access`] — трейты доступа к состоянию игры ([`BoardReadonly`], [`BoardMutable`])
//! - [`cache`] — кэширование строк для отрисовки ([`StringCache`])
//! - [`cycle`] — игровой цикл (FPS, ввод, отрисовка)
//! - [`mode_trait`] — трейт режима игры ([`GameModeTrait`])
//!
//! ## Архитектурные заметки
//! ## Исправление #26: Зависимости модулей
//! ```text
//! game/
//! ├── constants.rs (базовый, нет зависимостей)
//! ├── types.rs (базовый, нет зависимостей)
//! ├── state.rs (базовый, нет зависимостей от game/*)
//! ├── mode_trait.rs (базовый, нет зависимостей от game/*)
//! ├── access.rs (зависит от state.rs, io.rs)
//! ├── cache.rs (зависит от state.rs)
//! ├── view.rs (зависит от state.rs, mode_trait.rs)
//! ├── logic/ (зависит от state.rs, tetromino.rs, types.rs)
//! ├── scoring/ (зависит от state.rs, logic/)
//! ├── render/ (зависит от state.rs, view.rs, io.rs, tetromino.rs)
//! └── cycle.rs (зависит от всех вышеперечисленных)
//! ```
//!
//! ## Порядок инициализации
//! 1. `constants.rs` - централизованные константы
//! 2. `types.rs` - типобезопасные обёртки ([`Score`], [`Level`], [`LinesCount`])
//! 3. `state.rs` - базовые структуры и константы
//! 4. `mode_trait.rs` - трейты режимов
//! 5. `access.rs` - трейты доступа
//! 6. `cache.rs` - кэширование строк
//! 7. `view.rs` - представление для отрисовки
//! 8. `logic/` - игровая логика
//! 9. `scoring/` - система очков
//! 10. `render/` - отрисовка
//! 11. `cycle.rs` - игровой цикл (использует все модули)
//!
//! ## Пример использования
//!
//! ```ignore
//! use tetris_cli::game::{GameState, GameView};
//!
//! // Создание нового состояния игры
//! let mut game = GameState::new();
//!
//! // Создание представления для отрисовки
//! let view = GameView::from_game_state(&game);
//!
//! // Запуск игрового цикла
//! // game.play(&mut cnv, &mut inp, &high_score_display);
//! ```

// Подмодули
pub mod access;
pub mod board;
pub mod cache;
// constants больше не является отдельным файлом - используем ре-экспорт из crate::constants
pub mod components;
pub mod cycle;
pub mod logic;
pub mod mode_trait;
pub mod render;
pub mod rules;
pub mod scoreboard;
pub mod scoring;
pub mod state;
pub mod stats;
pub mod types;
pub mod view;

// Ре-экспорт констант для внутренних модулей game
// Это позволяет использовать super::constants внутри game/
pub mod constants {
    pub use crate::constants::*;
}

// Подмодули scoring

// Подмодули logic

// Re-export основных типов для обратной совместимости
pub use state::GameError;
#[allow(deprecated, unused_imports)]
pub use state::GameMode;
#[allow(unused_imports)]
pub use state::GameResult;
pub use state::GameState;
#[allow(unused_imports)]
pub use stats::GameStats;

// Re-export констант из constants.rs (обратная совместимость)
#[allow(unused_imports)]
pub use constants::{
    ANIMATION_FRAME_SKIP, COMBO_BONUS, FPS, HARD_DROP_ANIM_INTERVAL_MS, HARD_DROP_POINTS,
    INITIAL_FALL_SPD, LAND_TIME_DELAY_S, LEVEL_BONUS_MULT, LINES_PER_LEVEL, LINE_SCORES,
    MARATHON_LINES, MAX_FALL_SPEED, MAX_LINES_PER_CLEAR, MIN_Y, PIECE_SCORE_FALL_MULT,
    PIECE_SCORE_INC, SOFT_DROP_POINTS, SPD_INC, SPRINT_LINES,
};

// Re-export трейтов и типов из access
#[allow(unused_imports)]
pub use access::{BoardMutable, BoardReadonly, ScoreAccess, ScoreMutable};

// Re-export устаревшего трейта GameBoardAccess для обратной совместимости
#[allow(unused_imports, deprecated)]
#[deprecated(
    since = "0.3.0",
    note = "Используйте BoardMutable + ScoreAccess напрямую"
)]
pub use access::GameBoardAccess;

// Re-export трейтов и типов из board
#[allow(unused_imports)]
pub use board::{
    BoardMutable as BoardMutableTrait, BoardReadonly as BoardReadonlyTrait, GameBoard,
};

// Re-export ScoreBoard из scoreboard (трейты ScoreAccess и ScoreMutable теперь в access.rs)
#[allow(unused_imports)]
pub use scoreboard::ScoreBoard;

// Re-export GameView и StringCache для отрисовки и кэширования
#[allow(unused_imports)]
pub use cache::StringCache;
#[allow(unused_imports)]
pub use scoring::lines::find_filled_lines;
#[allow(unused_imports)]
pub use view::GameView;

// Константы для тестов (обратная совместимость)

// Re-export типов из types.rs (обратная совместимость)

pub use logic::{
    can_move_curr_shape_direction, can_rotate_curr_shape, rotate_with_wall_kick, save_tetromino,
};

#[allow(unused_imports)]
pub use scoring::{check_rows, find_full_rows, handle_hold, remove_rows};

// Экспорт GameView для отрисовки (обратная совместимость)

// Экспорт StringCache для кэширования (обратная совместимость)

// Экспорт игрового цикла (обратная совместимость)

// ============================================================================
// МЕТОДЫ ДЛЯ GameState
// ============================================================================
// Расширяем GameState методами из подмодулей

#[allow(dead_code)]
impl GameState {
    /// Запустить игровой цикл и вернуть финальный счёт.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки
    /// * `inp` - читатель нажатий клавиш
    /// * `high_score_display` - строка для отображения рекорда
    ///
    /// # Возвращает
    /// - `Ok(u128)` - финальный счёт игрока
    /// - `Err(GameError)` - ошибка во время игрового цикла
    ///
    /// # Errors
    /// Возвращает ошибку `GameError` при сбое ввода/вывода, ошибке терминала или других критических ошибках во время игрового цикла.
    ///
    /// # Архитектурные заметки (A8)
    /// Метод делегирует логику игрового цикла модулю [`cycle`]:
    /// - `handle_fps_control()` - поддержание стабильного FPS
    /// - `handle_input()` - обработка ввода пользователя
    /// - `render()` - отрисовка текущего кадра
    /// - `handle_game_over()` - обработка конца игры
    ///
    /// Возвращает `Result` для явной обработки ошибок.
    pub fn play(
        &mut self,
        cnv: &mut crate::io::Canvas,
        inp: &mut crate::io::KeyReader,
        high_score_display: &str,
    ) -> Result<u128, state::GameError> {
        cycle::run_game_loop(self, cnv, inp, high_score_display)
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
        save_tetromino(self);
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
        handle_hold(self);
    }

    /// Запустить таймер.
    pub fn start_timer(&mut self) {
        self.stats_mut().start_timer();
    }

    /// Остановить таймер.
    pub fn stop_timer(&mut self) {
        self.stats_mut().stop_timer();
    }

    /// Проверить, может ли призрак двигаться в указанном направлении.
    ///
    /// Используется для отрисовки призрачной фигуры (предпросмотр приземления).
    pub fn can_move_ghost_shape_direction(&self, dir: crate::types::Direction) -> bool {
        can_move_curr_shape_direction(self, dir)
    }

    /// Увеличить счетчик очищенных линий.
    ///
    /// Используется в тестах для проверки обновления счетчика.
    pub fn increment_lines_cleared(&mut self) {
        let lines = self.lines_cleared().saturating_add(1);
        self.set_lines_cleared(lines);
    }

    // ========================================================================
    // МЕТОДЫ ДЛЯ БЕНЧМАРКОВ (feature = "bench")
    // ========================================================================

    /// Заполнить указанную линию для бенчмарков.
    ///
    /// # Аргументы
    /// * `line` - индекс линии (0..GRID_HEIGHT)
    ///
    /// # Паника
    /// Паникует если индекс линии выходит за пределы поля
    ///
    /// # Пример
    /// ```ignore
    /// let mut state = GameState::new();
    /// state.fill_line_for_bench(10); // Заполнить линию 10
    /// ```
    #[cfg(feature = "bench")]
    #[allow(clippy::missing_panics_doc)]
    pub fn fill_line_for_bench(&mut self, line: usize) {
        use crate::io::GRID_HEIGHT;
        assert!(
            line < GRID_HEIGHT,
            "Индекс линии должен быть меньше {}",
            GRID_HEIGHT
        );
        self.get_blocks_mut()[line] = [1i8; crate::io::GRID_WIDTH];
    }

    /// Очистить заполненные линии для бенчмарков.
    ///
    /// Использует `check_rows()` для удаления заполненных линий.
    #[cfg(feature = "bench")]
    pub fn clear_lines_for_bench(&mut self) {
        self.check_rows();
    }

    /// Сохранить текущую фигуру для бенчмарков.
    ///
    /// Использует `save_tetromino()` для сохранения фигуры в поле.
    #[cfg(feature = "bench")]
    pub fn save_tetromino_for_bench(&mut self) {
        self.save_tetromino();
    }
}

// Импортируем типы из state для использования в impl

// ============================================================================
// МОДУЛЬ ТЕСТОВ
// ============================================================================

#[cfg(test)]
mod game_tests {
    use super::*;
    use crate::constants::{
        COMBO_BONUS, HARD_DROP_POINTS, INITIAL_FALL_SPD, LINE_SCORES, SOFT_DROP_POINTS,
    };
    use crate::game::state::GameStats;
    use crate::io::GRID_HEIGHT;
    use crate::types::Direction;

    // Тесты Hard Drop
    #[test]
    fn test_hard_drop_height_calculation() {
        let mut state = GameState::new();
        let mut drop_height = 0;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
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
        let start_y = state.curr_shape().pos.1;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }

        let drop_distance = (state.curr_shape().pos.1 - start_y) as u64;

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
            !state.is_hard_dropping(),
            "До Hard Drop флаг должен быть false"
        );

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }
        state.set_is_hard_dropping(true);

        assert!(
            state.is_hard_dropping(),
            "После Hard Drop флаг должен быть true для анимации"
        );

        state.set_is_hard_dropping(false);
        assert!(
            !state.is_hard_dropping(),
            "После анимации флаг должен сбрасываться"
        );
    }

    #[test]
    fn test_hard_drop_boundary() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape().pos.1;

        while state.can_move_curr_shape_direction(Direction::Down) {
            state.get_curr_shape_mut().pos.1 += 1.0;
        }

        assert!(
            state.curr_shape().pos.1 > initial_y,
            "Фигура должна опуститься после Hard Drop"
        );
        assert!(
            !state.can_move_curr_shape_direction(Direction::Down),
            "Движение вниз должно быть заблокировано после приземления"
        );
        assert!(
            state.curr_shape().pos.1 <= GRID_HEIGHT as f32,
            "Фигура не должна выходить за границы поля"
        );
    }

    // Тесты Soft Drop
    #[test]
    fn test_soft_drop_speed_increase() {
        let state = GameState::new();
        let initial_fall_spd = state.fall_speed();

        assert!(
            (initial_fall_spd - INITIAL_FALL_SPD).abs() < f32::EPSILON,
            "Начальная скорость должна быть INITIAL_FALL_SPD"
        );
        assert!(
            state.fall_speed() > 0.0,
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
            state.get_curr_shape_mut().pos.1 += 1.0;
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
            state.soft_drop_distance(),
            0,
            "Начальная дистанция Soft Drop должна быть 0"
        );

        let test_moves = 5;
        for _ in 0..test_moves {
            if state.can_move_curr_shape_direction(Direction::Down) {
                state.get_curr_shape_mut().pos.1 += 1.0;
                state.set_soft_drop_distance(state.soft_drop_distance() + 1);
            }
        }

        assert_eq!(
            state.soft_drop_distance(),
            test_moves,
            "Дистанция должна равняться количеству шагов"
        );

        state.set_soft_drop_distance(0);
        assert_eq!(
            state.soft_drop_distance(),
            0,
            "После сброса дистанция должна быть 0"
        );
    }

    // Тесты Combo системы
    #[test]
    fn test_combo_counter_increment() {
        let mut stats = GameStats::new();

        assert_eq!(
            stats.combo_counter(),
            0,
            "Начальное значение комбо должно быть 0"
        );

        stats.set_combo_counter(1);
        assert_eq!(
            stats.combo_counter(),
            1,
            "После первого удаления комбо должно быть 1"
        );

        stats.set_combo_counter(2);
        assert_eq!(
            stats.combo_counter(),
            2,
            "После второго удаления комбо должно быть 2"
        );

        stats.set_combo_counter(3);
        assert_eq!(
            stats.combo_counter(),
            3,
            "После третьего удаления комбо должно быть 3"
        );

        stats.update_max_combo(stats.combo_counter());
        assert_eq!(stats.max_combo(), 3, "Максимальное комбо должно быть 3");
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

        stats.set_combo_counter(3);
        assert_eq!(stats.combo_counter(), 3, "Комбо должно быть 3");

        stats.set_combo_counter(0);
        assert_eq!(
            stats.combo_counter(),
            0,
            "После хода без удаления комбо должно сбрасываться в 0"
        );

        stats.set_combo_counter(1);
        assert_eq!(
            stats.combo_counter(),
            1,
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
