//! Состояние игры и связанные структуры.
//!
//! Этот модуль содержит основные структуры данных для представления состояния игры:
//! - `GameState` — основное состояние игры
//! - `GameStats` — статистика прошедшей игры
//! - `GameMode` — режим игры
//! - `Achievement` — достижения (заготовка)
//! - Константы игры

use crate::io::{DISP_HEIGHT, GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
use std::time::Instant;

// Импорт из io для использования в state
use termion::color::White;

// ============================================================================
// КОНСТАНТЫ ИГРЫ
// ============================================================================

/// Цвет границ.
pub const BORDER_COLOR: &dyn termion::color::Color = &White;

/// Количество кадров в секунду.
///
/// Обеспечивает плавную анимацию игры.
/// Стандартное значение для большинства игр - 60 FPS.
pub const FPS: u64 = 60;

/// Начальная скорость падения.
///
/// Измеряется в блоках за секунду.
pub const INITIAL_FALL_SPD: f32 = 0.9;

/// Максимальная скорость падения.
///
/// Ограничивает максимальную скорость фигуры для предотвращения переполнения
/// при расчёте очков за падение.
pub const MAX_FALL_SPEED: f32 = 1000.0;

/// Задержка времени приземления (секунды).
///
/// Даёт игроку время на перемещение фигуры после касания.
pub const LAND_TIME_DELAY_S: f64 = 0.1;

/// Прирост скорости за уровень.
pub const SPD_INC: f32 = 0.05;

/// Очки за фигуру.
pub const PIECE_SCORE_INC: u128 = 100;

/// Множитель очков за падение.
pub const PIECE_SCORE_FALL_MULT: f32 = 50.0;

/// Очки за ячейку при Soft Drop.
pub const SOFT_DROP_POINTS: u128 = 1;

/// Очки за ячейку при Hard Drop.
pub const HARD_DROP_POINTS: u128 = 2;

/// Бонус за комбо: 50 × (номер комбо - 1).
pub const COMBO_BONUS: u128 = 50;

/// Количество линий для повышения уровня.
pub const LINES_PER_LEVEL: u32 = 10;

/// Бонус за повышение уровня: 500 × (номер уровня - 1).
/// Уровень 2: 500, Уровень 3: 1000, Уровень 11: 5000.
pub const LEVEL_BONUS_MULT: u128 = 500;

/// Lookup таблица очков за очистку линий.
/// Индекс 0 = 1 линия, индекс 1 = 2 линии, индекс 2 = 3 линии, индекс 3 = 4 линии.
/// Формула: 100 × 2^(линии-1), для 4 линий дополнительный бонус 1000.
pub const LINE_SCORES: [u128; 4] = [
    100,  // 1 линия: 100 × 2^0 = 100
    200,  // 2 линии: 100 × 2^1 = 200
    400,  // 3 линии: 100 × 2^2 = 400
    1800, // 4 линии (Tetris): 100 × 2^3 + 1000 = 1800
];

/// Максимальное количество линий, которое можно удалить за один ход.
///
/// В классическом тетрисе максимально возможно удалить 4 линии одновременно (Tetris).
pub const MAX_LINES_PER_CLEAR: u32 = 4;

/// Ширина игрового поля в блоках.
#[allow(dead_code)]
pub const FIELD_WIDTH: usize = GRID_WIDTH;

/// Высота игрового поля в блоках.
#[allow(dead_code)]
pub const FIELD_HEIGHT: usize = GRID_HEIGHT;

/// Смещение игрового поля по горизонтали при отрисовке.
#[allow(dead_code)]
pub const FIELD_OFFSET_X: usize = 5;

/// Количество линий для режима спринт.
pub const SPRINT_LINES: u32 = 40;

/// Количество линий для режима марафон.
#[allow(dead_code)]
pub const MARATHON_LINES: u32 = 150;

/// Минимальная допустимая координата Y для блоков фигуры.
pub const MIN_Y: i16 = 0;

/// Символ терминального bell для звуковых эффектов.
pub const BELL: &str = "\x07";

/// Интервал анимации мигания Hard Drop в миллисекундах.
pub const HARD_DROP_ANIM_INTERVAL_MS: u16 = 50;

/// Количество кадров для пропуска при анимации.
pub const ANIMATION_FRAME_SKIP: u16 = 2;

// ============================================================================
// КОНСТАНТЫ ПОЗИЦИЙ ОТРИСОВКИ UI
// ============================================================================

/// Позиция X для отрисовки счёта (строка 2).
pub const SCORE_X: u16 = 7;
/// Позиция Y для отрисовки счёта (строка 2).
pub const SCORE_Y: u16 = 2;
/// Позиция X для отрисовки рекорда (строка 3).
pub const HIGH_SCORE_X: u16 = 7;
/// Позиция Y для отрисовки рекорда (строка 3).
pub const HIGH_SCORE_Y: u16 = 3;
/// Позиция X для отрисовки уровня (строка 4).
pub const LEVEL_X: u16 = 10;
/// Позиция Y для отрисовки уровня (строка 4).
pub const LEVEL_Y: u16 = 4;
/// Позиция X для отрисовки линий (строка 5).
pub const LINES_X: u16 = 10;
/// Позиция Y для отрисовки линий (строка 5).
pub const LINES_Y: u16 = 5;

/// Позиция предпросмотра следующей фигуры по X (справа от поля).
pub const PREVIEW_X: u16 = 24;
/// Позиция предпросмотра следующей фигуры по Y.
pub const PREVIEW_Y: u16 = 8;

/// Позиция предпросмотра удержанной фигуры по X (слева от поля).
pub const HOLD_PREVIEW_X: u16 = 2;
/// Позиция предпросмотра удержанной фигуры по Y.
pub const HOLD_PREVIEW_Y: u16 = 8;

/// Смещение отрисовки фигур по вертикали.
pub const SHAPE_DRAW_OFFSET: i16 = 5;

/// Смещение отрисовки фигур по горизонтали.
pub const SHAPE_OFFSET_X: i16 = 2;

/// Смещение отрисовки фигур по вертикали (дополнительное).
pub const SHAPE_OFFSET_Y: i16 = 0;

/// Смещение отрисовки фигур по горизонтали (для предпросмотра).
pub const DRAW_OFFSET_X: i16 = 2;

// ============================================================================
// КОНСТАНТЫ ГРАНИЦ
// ============================================================================

/// Границы игрового поля с заголовками.
pub const BORDER: [&str; DISP_HEIGHT as usize] = [
    "                      ",
    "Счёт:                 ",
    "Рекорд:               ",
    "Уровень:              ",
    "Линии:                ",
    "╔════════════════════╗",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "╚════════════════════╝",
];

/// Сообщение о паузе.
pub const PAUSE: [&str; 3] = ["╔════════╗", "║ ПАУЗА  ║", "╚════════╝"];

/// Сообщение о проигрыше.
pub const GAME_OVER: [&str; 3] = ["╔════════════╗", "║ ИГРА ОКОНЧЕНА ║", "╚════════════╝"];

/// Задержка перед возвратом в меню после проигрыша (мс).
pub const GAME_OVER_DELAY_MS: u64 = 1500;

/// Количество миллисекунд в секунде.
pub const MILLIS_PER_SECOND: f32 = 1000.0;

// ============================================================================
// РЕЖИМ ИГРЫ
// ============================================================================

/// Режим игры.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    /// Классический режим — игра до проигрыша.
    Classic,
    /// Спринт — очистить 40 линий как можно быстрее.
    Sprint,
    /// Марафон — очистить 150 линий с нарастающей сложностью.
    Marathon,
}

// ============================================================================
// ДОСТИЖЕНИЯ
// ============================================================================

/// Достижение в игре (заготовка для будущей системы достижений).
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Achievement {
    /// Название достижения.
    pub name: String,
    /// Описание достижения.
    pub description: String,
    /// Очки за достижение.
    pub points: u32,
}

#[allow(dead_code)]
impl Achievement {
    /// Создать новое достижение.
    pub fn new(name: &str, description: &str, points: u32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            points,
        }
    }

    /// Достижение "Первый Tetris" — удалить 4 линии одновременно.
    pub fn first_tetris() -> Self {
        Self::new("🏆 TETRIS!", "Удалите 4 линии одновременно", 100)
    }

    /// Достижение "Комбо-мастер" — достичь комбо x5.
    pub fn combo_master(combo: u32) -> Self {
        Self::new(
            "🔥 Комбо-мастер",
            &format!("Достигните комбо x{combo}"),
            50 * combo,
        )
    }

    /// Достижение "Спринтер" — завершить режим спринт.
    pub fn sprinter() -> Self {
        Self::new("⚡ Спринтер", "Завершите режим спринт", 200)
    }

    /// Достижение "Марафонец" — завершить режим марафон.
    pub fn marathoner() -> Self {
        Self::new("🏃 Марафонец", "Завершите режим марафон", 500)
    }

    /// Достижение "Ветеран" — достичь уровня 10.
    pub fn veteran(level: u32) -> Self {
        Self::new(
            "⭐ Ветеран",
            &format!("Достигните уровня {level}"),
            100 * level,
        )
    }
}

// ============================================================================
// СТАТИСТИКА ИГРЫ
// ============================================================================

/// Статистика игры.
///
/// Содержит подробную информацию о прошедшей игре:
/// - Количество использованных фигур каждого типа
/// - Общее количество очищенных линий
/// - Максимальное комбо
/// - Время игры
/// - Полученные достижения
#[derive(Default, Clone)]
pub struct GameStats {
    /// Количество фигур типа T.
    pub t_pieces: u32,
    /// Количество фигур типа L.
    pub l_pieces: u32,
    /// Количество фигур типа J.
    pub j_pieces: u32,
    /// Количество фигур типа S.
    pub s_pieces: u32,
    /// Количество фигур типа Z.
    pub z_pieces: u32,
    /// Количество фигур типа O.
    pub o_pieces: u32,
    /// Количество фигур типа I.
    pub i_pieces: u32,
    /// Максимальное комбо (одновременное удаление линий).
    pub max_combo: u32,
    /// Текущее комбо (последовательные удаления в нескольких ходах).
    pub combo_counter: u32,
    /// Время начала игры.
    pub start_time: Option<Instant>,
    /// Время окончания игры.
    pub end_time: Option<Instant>,
    /// Полученные достижения.
    #[allow(dead_code)]
    pub achievements: Vec<Achievement>,
    /// Количество Tetris (4 линии одновременно).
    #[allow(dead_code)]
    pub tetris_count: u32,
    /// Общее количество удалённых линий.
    #[allow(dead_code)]
    pub total_lines: u32,
}

#[allow(dead_code)]
impl GameStats {
    /// Создать новую статистику.
    pub fn new() -> Self {
        Self::default()
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

    /// Проверить и добавить достижения.
    pub fn check_achievements(
        &mut self,
        lines: u32,
        level: u32,
        mode: GameMode,
    ) -> Vec<Achievement> {
        let mut new_achievements = Vec::new();

        // Достижение за Tetris (4 линии одновременно)
        if lines == 4 {
            self.tetris_count += 1;

            if !self.achievements.iter().any(|a| a.name == "🏆 TETRIS!") {
                new_achievements.push(Achievement::first_tetris());
            }
        }

        // Достижения за комбо
        if self.combo_counter >= 5 && !self.achievements.iter().any(|a| a.name.starts_with("🔥"))
        {
            new_achievements.push(Achievement::combo_master(self.combo_counter));
        }

        // Достижение за завершение спринта
        if mode == GameMode::Sprint
            && self.total_lines >= SPRINT_LINES
            && !self.achievements.iter().any(|a| a.name == "⚡ Спринтер")
        {
            new_achievements.push(Achievement::sprinter());
        }

        // Достижение за завершение марафона
        if mode == GameMode::Marathon
            && self.total_lines >= MARATHON_LINES
            && !self.achievements.iter().any(|a| a.name == "🏃 Марафонец")
        {
            new_achievements.push(Achievement::marathoner());
        }

        // Достижения за уровни (каждые 5 уровней)
        if level >= 5
            && level.is_multiple_of(5)
            && !self.achievements.iter().any(|a| a.name.starts_with("⭐"))
        {
            new_achievements.push(Achievement::veteran(level));
        }

        // Добавляем достижения в список
        for achievement in &new_achievements {
            if !self.achievements.iter().any(|a| a.name == achievement.name) {
                self.achievements.push(achievement.clone());
            }
        }

        new_achievements
    }
}

// ============================================================================
// СОСТОЯНИЕ ИГРЫ
// ============================================================================

/// Состояние игры.
///
/// Содержит всю информацию о текущем состоянии игры:
/// - Счёт, уровень, количество линий
/// - Текущая и следующая фигуры
/// - Удержанная фигура (Hold)
/// - Игровое поле
/// - Таймеры и скорость
/// - Статистика игры
/// - Режим игры
pub struct GameState {
    /// Текущий счёт.
    pub score: u128,
    /// Текущий уровень.
    pub level: u32,
    /// Количество удалённых линий.
    pub lines_cleared: u32,
    /// Текущая фигура.
    pub curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    pub next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    pub held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    pub can_hold: bool,
    /// Скорость падения.
    pub fall_spd: f32,
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), 0 = пусто.
    pub blocks: Box<[[i8; GRID_WIDTH]; GRID_HEIGHT]>,
    /// Таймер приземления.
    pub land_timer: f64,
    /// Статистика игры.
    pub stats: GameStats,
    /// Режим игры.
    pub mode: GameMode,
    /// Строки для анимации (мигание при очистке).
    pub animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    pub is_hard_dropping: bool,
    /// Количество ячеек, пройденных при Soft Drop.
    pub soft_drop_distance: u32,
    /// Генератор фигур по системе 7-bag.
    pub bag: BagGenerator,
    /// Кэшированная строка счёта для оптимизации отрисовки.
    pub cached_score_str: String,
    /// Кэшированная строка уровня для оптимизации отрисовки.
    pub cached_level_str: String,
    /// Кэшированная строка количества линий для оптимизации отрисовки.
    pub cached_lines_str: String,
    /// Последнее закэшированное значение счёта.
    pub last_cached_score: u128,
    /// Последнее закэшированное значение уровня.
    pub last_cached_level: u32,
    /// Последнее закэшированное значение количества линий.
    pub last_cached_lines: u32,
    /// Кэшированная строка рекорда для оптимизации отрисовки.
    pub cached_high_score_str: String,
    /// Кэшированная строка комбо для оптимизации отрисовки.
    pub cached_combo_str: String,
    /// Кэшированная строка таймера для оптимизации отрисовки.
    pub cached_timer_str: String,
    /// Последнее закэшированное значение комбо.
    pub last_cached_combo: u32,
}

/// Состояние завершения обновления.
pub enum UpdateEndState {
    /// Выход из игры.
    Quit,
    /// Проигрыш.
    Lost,
    /// Продолжить.
    Continue,
    /// Пауза.
    Pause,
    /// Победа (завершение режима спринт/марафон).
    Won,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl GameState {
    /// Создать новое состояние игры.
    pub fn new() -> Self {
        Self::new_internal(GameMode::Classic, false)
    }

    /// Создать новое состояние игры для режима спринт.
    pub fn new_sprint() -> Self {
        Self::new_internal(GameMode::Sprint, true)
    }

    /// Создать новое состояние игры для режима марафон.
    pub fn new_marathon() -> Self {
        Self::new_internal(GameMode::Marathon, true)
    }

    /// Внутренний метод создания состояния игры.
    fn new_internal(mode: GameMode, start_timer: bool) -> Self {
        let mut bag = BagGenerator::new();
        let curr_shape = Tetromino::from_bag(&mut bag);
        let next_shape = Tetromino::from_bag(&mut bag);
        let mut stats = GameStats::new();
        stats.add_piece(curr_shape.shape);
        if start_timer {
            stats.start_timer();
        }
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            blocks: Box::new([[-1; GRID_WIDTH]; GRID_HEIGHT]),
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode,
            animating_rows_mask: 0,
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
            cached_score_str: "0".to_string(),
            cached_level_str: "1".to_string(),
            cached_lines_str: "0".to_string(),
            last_cached_score: 0,
            last_cached_level: 1,
            last_cached_lines: 0,
            cached_high_score_str: String::new(),
            cached_combo_str: String::new(),
            cached_timer_str: String::new(),
            last_cached_combo: 0,
        }
    }

    // ========================================================================
    // ГЕТТЕРЫ ДЛЯ ДОСТУПА К ПОЛЯМ
    // ========================================================================

    /// Получить текущий счёт.
    #[must_use]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Получить текущий уровень.
    #[must_use]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество удалённых линий.
    #[must_use]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить статистику игры.
    #[must_use]
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить режим игры.
    #[must_use]
    pub fn get_mode(&self) -> GameMode {
        self.mode
    }

    /// Получить игровое поле.
    #[must_use]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить текущую фигуру.
    #[must_use]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить следующую фигуру.
    #[must_use]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить удержанную фигуру.
    #[must_use]
    pub fn get_held_shape(&self) -> Option<&Tetromino> {
        self.held_shape.as_ref()
    }

    /// Получить текущую фигуру (мутуабельная ссылка для тестов).
    #[must_use]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.curr_shape
    }

    /// Получить следующую фигуру (мутуабельная ссылка для тестов).
    #[must_use]
    pub fn get_next_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.next_shape
    }

    /// Получить скорость падения.
    #[must_use]
    pub fn get_fall_spd(&self) -> f32 {
        self.fall_spd
    }

    // ========================================================================
    // СЕТТЕРЫ ДЛЯ ИЗМЕНЕНИЯ ПОЛЕЙ (ДЛЯ ТЕСТОВ)
    // ========================================================================

    /// Установить счёт (для тестов).
    pub fn set_score(&mut self, score: u128) {
        self.score = score;
    }

    /// Установить уровень (для тестов).
    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    /// Установить количество удалённых линий (для тестов).
    pub fn set_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = lines;
    }

    /// Добавить линии (для тестов).
    pub fn add_lines_cleared(&mut self, count: u32) {
        self.lines_cleared = self.lines_cleared.saturating_add(count);
        self.stats.total_lines = self.stats.total_lines.saturating_add(count);
    }

    /// Удалить заполненные линии (для тестов).
    pub fn remove_full_rows(&mut self) {
        let (rows_mask, _) = crate::game::find_full_rows(&self.blocks);
        crate::game::remove_rows(&mut self.blocks, rows_mask);
    }

    /// Добавить очки без проверки (для тестов).
    ///
    /// # Аргументы
    /// * `score` - Количество очков для добавления
    ///
    /// # Пример
    /// ```ignore
    /// let mut state = GameState::new();
    /// state.add_score_no_check(100);
    /// assert_eq!(state.get_score(), 100);
    /// ```
    pub fn add_score_no_check(&mut self, score: u128) {
        self.score = self.score.saturating_add(score);
    }
}
