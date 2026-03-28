//! Состояние игры и связанные структуры.
//!
//! Модуль содержит основные структуры данных:
//! - `GameState` - основное состояние игры
//! - `GameStats` - статистика прошедшей игры
//! - `GameModeTrait` - трейт режима игры
//! - Константы игры

use crate::io::GRID_HEIGHT;
use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
use std::time::Instant;

use super::constants::{GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S, MAX_FALL_SPEED};
use super::mode_trait::GameModeTrait;

// ============================================================================
// ТИПЫ ОШИБОК
// ============================================================================

/// Типы ошибок игры.
///
/// # Архитектурные заметки
/// TODO (#архитектура): Удалить этот enum, если он не используется в основном коде.
/// В настоящее время используется только в тестах.
/// Для обработки ошибок в проекте используются стандартные механизмы Rust.
#[derive(Debug)]
pub enum GameError {
    /// Ошибка ввода/вывода.
    Io(std::io::Error),
    /// Ошибка терминала.
    Terminal(String),
    /// Ошибка конфигурации.
    Config(String),
    /// Игра окончена.
    GameOver,
    /// Ошибка валидации.
    Validation(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::Io(e) => write!(f, "Ошибка ввода/вывода: {e}"),
            GameError::Terminal(msg) => write!(f, "Ошибка терминала: {msg}"),
            GameError::Config(msg) => write!(f, "Ошибка конфигурации: {msg}"),
            GameError::GameOver => write!(f, "Игра окончена"),
            GameError::Validation(msg) => write!(f, "Ошибка валидации: {msg}"),
        }
    }
}

impl std::error::Error for GameError {}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::Io(err)
    }
}

/// Тип результата игры.
///
/// # Архитектурные заметки
/// TODO (#архитектура): Удалить этот тип, если GameError будет удалён.
/// В настоящее время используется только в тестах.
pub type GameResult<T> = Result<T, GameError>;

// ============================================================================
// РЕЖИМ ИГРЫ
// ============================================================================

/// Режим игры.
///
/// # Архитектурные заметки
/// ## Абстракции для режимов (Problem 2.8)
/// Этот enum сохраняется для обратной совместимости.
/// Для нового кода рекомендуется использовать `GameModeTrait` напрямую.
///
/// ## Конвертация в трейт
/// Используйте метод `as_trait()` для получения объекта трейта.
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
// МЕТОДЫ ДЛЯ GAMEMODE
// ============================================================================

impl GameMode {
    /// Преобразовать enum в объект трейта GameModeTrait.
    ///
    /// # Возвращает
    /// Box<dyn GameModeTrait> с соответствующим режимом
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::state::GameMode;
    /// let mode = GameMode::Classic;
    /// let trait_obj = mode.as_trait();
    /// assert_eq!(trait_obj.name(), "Классика");
    /// ```
    #[must_use]
    pub fn as_trait(self) -> Box<dyn GameModeTrait> {
        match self {
            GameMode::Classic => Box::new(super::mode_trait::ClassicMode),
            GameMode::Sprint => Box::new(super::mode_trait::SprintMode::new()),
            GameMode::Marathon => Box::new(super::mode_trait::MarathonMode::new()),
        }
    }
}

impl GameMode {
    /// Проверить условие победы для текущего режима.
    ///
    /// # Аргументы
    /// * `lines_cleared` - количество очищенных линий
    ///
    /// # Возвращает
    /// `true` если условие победы выполнено (для режимов Sprint/Marathon)
    /// `false` для классического режима (победы нет, только проигрыш)
    ///
    /// # Архитектурные заметки
    /// Этот метод делегирует вызов трейту GameModeTrait.
    #[must_use]
    pub fn check_win_condition(self, lines_cleared: u32) -> bool {
        self.as_trait().check_win_condition(lines_cleared)
    }

    /// Получить целевое количество линий для режима.
    ///
    /// # Возвращает
    /// Количество линий для победы (для Sprint/Marathon) или None для Classic
    #[must_use]
    pub fn get_target_lines(self) -> Option<u32> {
        self.as_trait().get_target_lines()
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

// ============================================================================
// КЭШ ДЛЯ ОТРИСОВКИ
// ============================================================================

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
    pub fn new() -> Self {
        Self {
            cached_score_str: String::with_capacity(16),
            cached_level_str: String::with_capacity(16),
            cached_lines_str: String::with_capacity(16),
            cached_high_score_str: String::with_capacity(16),
            cached_combo_str: String::with_capacity(16),
            cached_timer_str: String::with_capacity(16),
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
///
/// # Архитектурные заметки
/// ## Инкапсуляция (Problem 2.1)
/// Поля структуры сделаны приватными для улучшения инкапсуляции.
///
/// Доступ к полям осуществляется через геттеры/сеттеры в impl блоке ниже.
/// Это позволяет валидировать изменения и контролировать доступ.
///
/// ## Геттеры
/// Для всех полей существуют геттеры в impl блоке ниже.
/// Используйте их вместо прямого доступа к полям.
///
/// ## Пример использования
/// ```ignore
/// let mut game = GameState::new();
/// let score = game.get_score();  // Используйте геттер
/// let level = game.get_level();  // Используйте геттер
/// ```
///
/// ## Будущая рефакторизация
/// TODO (#архитектура, срок: 1-2 недели): Разделить GameState на специализированные модули:
/// - `GameBoard` - состояние поля (blocks, filled_lines)
/// - `ScoreBoard` - состояние очков (score, level, lines_cleared)
/// - `FigureManager` - состояние фигур (curr_shape, next_shape, held_shape, bag)
/// - `AnimationState` - состояние анимаций (animating_rows_mask, is_hard_dropping)
pub struct GameState {
    // ========================================================================
    // === СОСТОЯНИЕ ПОЛЯ (будущий GameBoard) ===
    // ========================================================================
    // TODO (#архитектура): Выделить в отдельную структуру GameBoard
    // Приоритет: Высокий
    // Срок: 1-2 недели
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), -1 = пусто.
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Битовая маска заполненных линий (для будущей оптимизации).
    filled_lines: u32,

    // ========================================================================
    // === СОСТОЯНИЕ ОЧКОВ (будущий ScoreBoard) ===
    // ========================================================================
    // TODO (#архитектура): Выделить в отдельную структуру ScoreBoard
    // Приоритет: Высокий
    // Срок: 1-2 недели
    /// Текущий счёт.
    score: u128,
    /// Текущий уровень.
    level: u32,
    /// Количество удалённых линий.
    lines_cleared: u32,

    // ========================================================================
    // === СОСТОЯНИЕ ФИГУР (будущий FigureManager) ===
    // ========================================================================
    // TODO (#архитектура): Выделить в отдельную структуру FigureManager
    // Приоритет: Средний
    // Срок: 2-3 недели
    /// Текущая фигура.
    curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    can_hold: bool,
    /// Генератор фигур по системе 7-bag.
    bag: BagGenerator,

    // ========================================================================
    // === СОСТОЯНИЕ АНИМАЦИЙ (будущий AnimationState) ===
    // ========================================================================
    // TODO (#архитектура): Выделить в отдельную структуру AnimationState
    // Приоритет: Средний
    // Срок: 2-3 недели
    /// Строки для анимации (мигание при очистке).
    animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    is_hard_dropping: bool,

    // ========================================================================
    // === ИГРОВАЯ ЛОГИКА ===
    // ========================================================================
    /// Скорость падения.
    fall_speed: f32,
    /// Таймер приземления.
    land_timer: f64,
    /// Количество ячеек, пройденных при Soft Drop.
    soft_drop_distance: u32,

    // ========================================================================
    // === СТАТИСТИКА И РЕЖИМ ИГРЫ ===
    // ========================================================================
    /// Статистика игры.
    stats: GameStats,
    /// Режим игры (объект трейта).
    /// Использует трейт GameModeTrait вместо enum для лучшей расширяемости.
    mode_trait: Box<dyn GameModeTrait>,

    // ========================================================================
    // === КЭШ ДЛЯ ОТРИСОВКИ ===
    // ========================================================================
    /// Кэш для оптимизации отрисовки.
    render_cache: RenderCache,
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
        // Создаём объект трейта из enum
        let mode_trait = mode.as_trait();
        let mut game_state = Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_speed: INITIAL_FALL_SPD,
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            filled_lines: 0,
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode_trait,
            animating_rows_mask: 0,
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
            render_cache: RenderCache::new(),
        };
        // Инициализация кэша начальными значениями
        game_state.render_cache.init_with_values(0, 1, 0, 0);
        game_state
    }

    // ========================================================================
    // ГЕТТЕРЫ ДЛЯ ДОСТУПА К ПОЛЯМ
    // ========================================================================

    /// Получить текущий счёт.
    #[must_use]
    pub fn score(&self) -> u128 {
        self.score
    }

    /// Получить текущий уровень.
    #[must_use]
    pub fn level(&self) -> u32 {
        self.level
    }

    /// Получить количество удалённых линий.
    #[must_use]
    pub fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить текущий счёт (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`score()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте score()")]
    pub fn get_score(&self) -> u128 {
        self.score()
    }

    /// Получить текущий уровень (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`level()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте level()")]
    pub fn get_level(&self) -> u32 {
        self.level()
    }

    /// Получить количество удалённых линий (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`lines_cleared()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте lines_cleared()")]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared()
    }

    /// Получить статистику игры.
    #[must_use]
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить статистику игры (мутуабельная ссылка).
    #[must_use]
    pub fn get_stats_mut(&mut self) -> &mut GameStats {
        &mut self.stats
    }

    /// Получить режим игры (объект трейта).
    ///
    /// # Возвращает
    /// Ссылка на объект трейта GameModeTrait
    ///
    /// # Пример использования
    /// ```ignore
    /// let state = GameState::new();
    /// let mode = state.get_mode_trait();
    /// assert_eq!(mode.name(), "Классика");
    /// ```
    #[must_use]
    pub fn get_mode_trait(&self) -> &dyn GameModeTrait {
        &*self.mode_trait
    }

    /// Получить режим игры (enum для обратной совместимости).
    ///
    /// # Возвращает
    /// Значение enum GameMode
    ///
    /// # Архитектурные заметки
    /// Метод сохранён для обратной совместимости с тестами.
    /// Использует get_mode_trait() для получения режима.
    #[must_use]
    #[deprecated(since = "23.96.14", note = "Используйте get_mode_trait() вместо enum")]
    pub fn get_mode(&self) -> GameMode {
        // Используем name() трейта для определения режима
        match self.get_mode_trait().name() {
            "Классика" => GameMode::Classic,
            "Спринт" => GameMode::Sprint,
            "Марафон" => GameMode::Marathon,
            _ => GameMode::Classic, // Fallback
        }
    }

    /// Получить игровое поле.
    #[must_use]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить игровое поле (мутуабельная ссылка).
    #[must_use]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }

    /// Получить текущую фигуру.
    #[must_use]
    pub fn curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить следующую фигуру.
    #[must_use]
    pub fn next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить удержанную фигуру.
    #[must_use]
    pub fn held_shape(&self) -> Option<&Tetromino> {
        self.held_shape.as_ref()
    }

    /// Получить текущую фигуру (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`curr_shape()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте curr_shape()")]
    pub fn get_curr_shape(&self) -> &Tetromino {
        self.curr_shape()
    }

    /// Получить следующую фигуру (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`next_shape()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте next_shape()")]
    pub fn get_next_shape(&self) -> &Tetromino {
        self.next_shape()
    }

    /// Получить удержанную фигуру (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`held_shape()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте held_shape()")]
    pub fn get_held_shape(&self) -> Option<&Tetromino> {
        self.held_shape()
    }

    /// Получить удержанную фигуру (ссылка на Option).
    #[must_use]
    #[allow(clippy::ref_option)]
    pub fn get_held_shape_ref(&self) -> &Option<Tetromino> {
        &self.held_shape
    }

    /// Получить текущую фигуру (мутуабельная ссылка).
    #[must_use]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.curr_shape
    }

    /// Получить следующую фигуру (мутуабельная ссылка).
    #[must_use]
    pub fn get_next_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.next_shape
    }

    /// Получить скорость падения.
    #[must_use]
    pub fn fall_speed(&self) -> f32 {
        self.fall_speed
    }

    /// Получить скорость падения (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`fall_speed()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте fall_speed()")]
    pub fn get_fall_speed(&self) -> f32 {
        self.fall_speed()
    }

    /// Получить скорость падения (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`fall_speed()`] вместо этого.
    #[must_use]
    #[deprecated(
        since = "23.96.15",
        note = "Используйте get_fall_speed(), затем fall_speed()"
    )]
    pub fn get_fall_spd(&self) -> f32 {
        self.fall_speed()
    }

    /// Получить таймер приземления.
    #[must_use]
    pub fn land_timer(&self) -> f64 {
        self.land_timer
    }

    /// Получить таймер приземления (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`land_timer()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте land_timer()")]
    pub fn get_land_timer(&self) -> f64 {
        self.land_timer()
    }

    /// Получить флаг возможности удержания фигуры.
    #[must_use]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Получить флаг анимации Hard Drop.
    #[must_use]
    pub fn is_hard_dropping(&self) -> bool {
        self.is_hard_dropping
    }

    /// Получить расстояние Soft Drop.
    #[must_use]
    pub fn soft_drop_distance(&self) -> u32 {
        self.soft_drop_distance
    }

    /// Получить расстояние Soft Drop (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`soft_drop_distance()`] вместо этого.
    #[must_use]
    #[deprecated(since = "23.96.16", note = "Используйте soft_drop_distance()")]
    pub fn get_soft_drop_distance(&self) -> u32 {
        self.soft_drop_distance()
    }

    /// Получить маску анимации строк.
    #[must_use]
    pub fn get_animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Получить генератор фигур.
    #[must_use]
    pub fn get_bag(&self) -> &BagGenerator {
        &self.bag
    }

    /// Получить генератор фигур (мутуабельная ссылка).
    #[must_use]
    pub fn get_bag_mut(&mut self) -> &mut BagGenerator {
        &mut self.bag
    }

    /// Получить кэшированную строку счёта.
    #[must_use]
    pub fn get_cached_score_str(&self) -> &str {
        &self.render_cache.cached_score_str
    }

    /// Получить кэшированную строку уровня.
    #[must_use]
    pub fn get_cached_level_str(&self) -> &str {
        &self.render_cache.cached_level_str
    }

    /// Получить кэшированную строку линий.
    #[must_use]
    pub fn get_cached_lines_str(&self) -> &str {
        &self.render_cache.cached_lines_str
    }

    /// Получить кэшированную строку рекорда.
    #[must_use]
    pub fn get_cached_high_score_str(&self) -> &str {
        &self.render_cache.cached_high_score_str
    }

    /// Получить кэшированную строку комбо.
    #[must_use]
    pub fn get_cached_combo_str(&self) -> &str {
        &self.render_cache.cached_combo_str
    }

    /// Получить кэшированную строку таймера.
    #[must_use]
    pub fn get_cached_timer_str(&self) -> &str {
        &self.render_cache.cached_timer_str
    }

    /// Получить битовую маску заполненных линий.
    #[must_use]
    pub fn get_filled_lines(&self) -> u32 {
        self.filled_lines
    }

    /// Получить кэш для отрисовки (ссылка).
    #[must_use]
    pub fn get_render_cache(&self) -> &RenderCache {
        &self.render_cache
    }

    /// Получить кэш для отрисовки (мутуабельная ссылка).
    #[must_use]
    pub fn get_render_cache_mut(&mut self) -> &mut RenderCache {
        &mut self.render_cache
    }

    // ========================================================================
    // ТЕСТОВЫЕ ГЕТТЕРЫ (#[cfg(test)])
    // ========================================================================

    /// Получить игровое поле (для тестов).
    #[cfg(test)]
    pub fn blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить статистику игры (для тестов).
    #[cfg(test)]
    pub fn stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить кэш для отрисовки (для тестов).
    #[cfg(test)]
    pub fn render_cache(&self) -> &RenderCache {
        &self.render_cache
    }

    // ========================================================================
    // СЕТТЕРЫ ДЛЯ ИЗМЕНЕНИЯ ПОЛЕЙ (ДЛЯ ТЕСТОВ) С ВАЛИДАЦИЕЙ
    // ========================================================================

    /// Установить счёт (для тестов).
    #[track_caller]
    pub fn set_score(&mut self, score: u128) {
        self.score = score;
    }

    /// Установить уровень (для тестов).
    ///
    /// Уровень должен быть >= 1.
    #[track_caller]
    pub fn set_level(&mut self, level: u32) {
        self.level = level.max(1);
    }

    /// Установить количество удалённых линий (для тестов).
    #[track_caller]
    pub fn set_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = lines;
    }

    /// Установить режим игры через трейт (для тестов).
    ///
    /// # Аргументы
    /// * `mode_trait` - объект трейта GameModeTrait
    ///
    /// # Пример использования
    /// ```ignore
    /// use crate::game::mode_trait::{ClassicMode, GameModeTrait};
    /// let mut state = GameState::new();
    /// state.set_mode_trait(Box::new(ClassicMode));
    /// ```
    #[track_caller]
    pub fn set_mode_trait(&mut self, mode_trait: Box<dyn GameModeTrait>) {
        self.mode_trait = mode_trait;
    }

    /// Установить режим игры через enum (для обратной совместимости).
    ///
    /// # Аргументы
    /// * `mode` - значение enum GameMode
    ///
    /// # Архитектурные заметки
    /// Устарело, используйте set_mode_trait() для нового кода.
    ///
    /// # Исправление #14
    /// Метод обновлён для работы только с mode_trait (поле mode удалено).
    #[deprecated(since = "23.96.14", note = "Используйте set_mode_trait() вместо enum")]
    #[track_caller]
    pub fn set_mode(&mut self, mode: GameMode) {
        self.mode_trait = mode.as_trait();
    }

    /// Добавить линии (для тестов).
    #[track_caller]
    pub fn add_lines_cleared(&mut self, count: u32) {
        self.lines_cleared = self.lines_cleared.saturating_add(count);
    }

    /// Установить текущую фигуру (для тестов).
    #[track_caller]
    pub fn set_curr_shape(&mut self, shape: Tetromino) {
        self.curr_shape = shape;
    }

    /// Установить следующую фигуру (для тестов).
    #[track_caller]
    pub fn set_next_shape(&mut self, shape: Tetromino) {
        self.next_shape = shape;
    }

    /// Установить удержанную фигуру (для тестов).
    #[track_caller]
    pub fn set_held_shape(&mut self, shape: Option<Tetromino>) {
        self.held_shape = shape;
    }

    /// Установить флаг возможности удержания (для тестов).
    #[track_caller]
    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }

    /// Установить скорость падения (для тестов).
    ///
    /// Скорость падения должна быть в диапазоне [0.1, MAX_FALL_SPEED].
    #[track_caller]
    pub fn set_fall_speed(&mut self, spd: f32) {
        self.fall_speed = spd.clamp(0.1, MAX_FALL_SPEED);
    }

    /// Установить скорость падения (устаревшее имя, для тестов).
    ///
    /// # Устарело
    /// Используйте [`set_fall_speed()`] вместо этого.
    #[deprecated(since = "23.96.15", note = "Используйте set_fall_speed()")]
    #[track_caller]
    pub fn set_fall_spd(&mut self, spd: f32) {
        self.set_fall_speed(spd);
    }

    /// Установить таймер приземления (для тестов).
    ///
    /// Таймер должен быть >= 0.
    #[track_caller]
    pub fn set_land_timer(&mut self, timer: f64) {
        self.land_timer = timer.max(0.0);
    }

    /// Установить флаг Hard Drop (для тестов).
    #[track_caller]
    pub fn set_is_hard_dropping(&mut self, dropping: bool) {
        self.is_hard_dropping = dropping;
    }

    /// Установить расстояние Soft Drop (для тестов).
    #[track_caller]
    pub fn set_soft_drop_distance(&mut self, distance: u32) {
        self.soft_drop_distance = distance;
    }

    /// Установить маску анимации строк (для тестов).
    #[track_caller]
    pub fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
    }

    /// Удалить заполненные линии (для тестов).
    #[track_caller]
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
    #[track_caller]
    pub fn add_score_no_check(&mut self, score: u128) {
        self.score = self.score.saturating_add(score);
    }

    /// Добавить очки (для тестов и через трейт `GameBoardAccess`).
    #[track_caller]
    pub fn add_score(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    // ========================================================================
    // МЕТОДЫ ДЛЯ БЕНЧМАРКОВ
    // ========================================================================

    /// Заполнить линию для бенчмарков.
    ///
    /// Заполняет указанную линию блоками для теста очистки линий.
    /// Используется только при компиляции с feature = "bench".
    ///
    /// # Аргументы
    /// * `line` - номер линии для заполнения (0-19)
    #[cfg(feature = "bench")]
    pub fn fill_line_for_bench(&mut self, line: usize) {
        // Заполняем указанную линию блоками цвета 0
        for x in 0..GRID_WIDTH {
            self.blocks[line][x] = 0;
        }
    }

    /// Очистить линии для бенчмарков.
    ///
    /// Выполняет очистку заполненных линий для теста производительности.
    /// Используется только при компиляции с feature = "bench".
    #[cfg(feature = "bench")]
    pub fn clear_lines_for_bench(&mut self) {
        let (rows_mask, _) = crate::game::find_full_rows(&self.blocks);
        crate::game::remove_rows(&mut self.blocks, rows_mask);
    }

    /// Сохранить фигуру для бенчмарков.
    ///
    /// Сохраняет текущую фигуру в hold для теста механики hold.
    /// Используется только при компиляции с feature = "bench".
    #[cfg(feature = "bench")]
    pub fn save_tetromino_for_bench(&mut self) {
        if self.can_hold {
            self.held_shape = Some(self.curr_shape);
            self.can_hold = false;
        }
    }

    /// Установить текущую фигуру для бенчмарков.
    ///
    /// Создаёт новую фигуру из мешка и устанавливает её как текущую.
    /// Используется только при компиляции с feature = "bench".
    #[cfg(feature = "bench")]
    pub fn set_curr_shape_for_bench(&mut self) {
        self.curr_shape = self.next_shape;
        self.next_shape = Tetromino::from_bag(&mut self.bag);
        self.stats.add_piece(self.curr_shape.shape);
    }
}

// ============================================================================
// ТЕСТЫ ДЛЯ БЕНЧМАРК-МЕТОДОВ
// ============================================================================

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    /// Тест для fill_line_for_bench()
    /// Проверяет, что метод заполняет указанную линию блоками
    #[test]
    #[cfg(feature = "bench")]
    fn test_fill_line_for_bench() {
        let mut state = GameState::new();
        let line = 5;

        // Заполняем линию 5
        state.fill_line_for_bench(line);

        // Проверяем, что все ячейки в линии 5 заполнены блоками цвета 0
        for x in 0..GRID_WIDTH {
            assert_eq!(
                state.blocks[line][x], 0,
                "Ячейка [{}][{}] должна быть заполнена блоком цвета 0",
                line, x
            );
        }

        // Проверяем, что другие линии остались пустыми
        for y in 0..GRID_HEIGHT {
            if y != line {
                for x in 0..GRID_WIDTH {
                    assert_eq!(
                        state.blocks[y][x], -1,
                        "Ячейка [{}][{}] должна оставаться пустой",
                        y, x
                    );
                }
            }
        }
    }

    /// Тест для clear_lines_for_bench()
    /// Проверяет, что метод очищает заполненные линии
    #[test]
    #[cfg(feature = "bench")]
    fn test_clear_lines_for_bench() {
        let mut state = GameState::new();

        // Заполняем линии 3, 5 и 7
        state.fill_line_for_bench(3);
        state.fill_line_for_bench(5);
        state.fill_line_for_bench(7);

        // Проверяем, что линии заполнены
        for &line in &[3, 5, 7] {
            for x in 0..GRID_WIDTH {
                assert_eq!(
                    state.blocks[line][x], 0,
                    "Линия {} должна быть заполнена перед очисткой",
                    line
                );
            }
        }

        // Очищаем линии
        state.clear_lines_for_bench();

        // Проверяем, что линии очищены (сдвинуты вниз)
        // После очистки заполненные линии должны исчезнуть, а верхние сдвинуться
        let mut empty_lines_count = 0;
        for y in 0..GRID_HEIGHT {
            let is_line_empty = (0..GRID_WIDTH).all(|x| state.blocks[y][x] == -1);
            if is_line_empty {
                empty_lines_count += 1;
            }
        }

        // После очистки 3 линий должно быть как минимум 3 пустых линии
        assert!(
            empty_lines_count >= 3,
            "После очистки должно быть как минимум 3 пустых линии"
        );
    }

    /// Тест для save_tetromino_for_bench()
    /// Проверяет, что метод сохраняет текущую фигуру в hold
    #[test]
    #[cfg(feature = "bench")]
    fn test_save_tetromino_for_bench() {
        let mut state = GameState::new();

        // Запоминаем текущую фигуру
        let original_shape = state.curr_shape.shape;

        // Сохраняем фигуру в hold
        state.save_tetromino_for_bench();

        // Проверяем, что фигура сохранена в hold
        assert!(
            state.held_shape.is_some(),
            "Фигура должна быть сохранена в hold"
        );
        assert_eq!(
            state.held_shape.unwrap().shape,
            original_shape,
            "Сохранённая фигура должна совпадать с исходной"
        );

        // Проверяем, что can_hold установлен в false
        assert!(
            !state.can_hold,
            "После удержания фигуры can_hold должен быть false"
        );
    }

    /// Тест для set_curr_shape_for_bench()
    /// Проверяет, что метод устанавливает следующую фигуру как текущую
    #[test]
    #[cfg(feature = "bench")]
    fn test_set_curr_shape_for_bench() {
        let mut state = GameState::new();

        // Запоминаем следующую фигуру
        let next_shape = state.next_shape.shape;

        // Устанавливаем следующую фигуру как текущую
        state.set_curr_shape_for_bench();

        // Проверяем, что текущая фигура стала той, которая была следующей
        assert_eq!(
            state.curr_shape.shape, next_shape,
            "Текущая фигура должна стать следующей"
        );

        // Проверяем, что следующая фигура обновилась (новая из мешка)
        // Новая фигура должна отличаться от предыдущей (с высокой вероятностью)
        // или быть любой из 7 типов
        assert!(
            matches!(
                state.next_shape.shape,
                ShapeType::T
                    | ShapeType::L
                    | ShapeType::J
                    | ShapeType::S
                    | ShapeType::Z
                    | ShapeType::O
                    | ShapeType::I
            ),
            "Следующая фигура должна быть одного из 7 типов"
        );

        // Проверяем, что статистика обновилась
        assert!(
            state.stats.total_pieces() >= 2,
            "В статистике должно быть как минимум 2 фигуры"
        );
    }
}
