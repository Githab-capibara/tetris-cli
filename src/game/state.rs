//! Состояние игры и связанные структуры.
//!
//! Модуль содержит основные структуры данных:
//! - `GameState` - основное состояние игры
//! - `GameModeTrait` - трейт режима игры
//! - Константы игры
//!
//! ## Архитектурные заметки
//! ## Исправление #1 (Разделение GameState)
//! - `GameStats` перемещён в отдельный модуль [`super::stats`]
//! - `RenderCache` перемещён в отдельный модуль [`super::cache`]
//! - `GameState` использует композицию: содержит `stats: GameStats`, `cache: RenderCache`

use crate::io::GRID_HEIGHT;
use crate::tetromino::{BagGenerator, Tetromino};

use super::cache::RenderCache;
use super::constants::{GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S};
use super::mode_trait::GameModeTrait;
pub use super::stats::GameStats;

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
///
/// # Устарело
/// Используйте [`GameModeTrait`] напрямую вместо enum.
#[deprecated(since = "23.96.17", note = "Используйте GameModeTrait напрямую")]
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

#[allow(deprecated)]
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

    /// Получить статистику игры (для обратной совместимости).
    #[must_use]
    pub fn stats(&self) -> &GameStats {
        &self.stats
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

    /// Получить скорость падения.
    #[must_use]
    pub fn fall_speed(&self) -> f32 {
        self.fall_speed
    }

    /// Получить скорость падения (для обратной совместимости).
    #[must_use]
    pub fn get_fall_speed(&self) -> f32 {
        self.fall_speed()
    }

    /// Получить таймер приземления.
    #[must_use]
    pub fn land_timer(&self) -> f64 {
        self.land_timer
    }

    /// Получить таймер приземления (для обратной совместимости).
    #[must_use]
    pub fn get_land_timer(&self) -> f64 {
        self.land_timer()
    }

    /// Получить расстояние Soft Drop.
    #[must_use]
    pub fn soft_drop_distance(&self) -> u32 {
        self.soft_drop_distance
    }

    /// Получить флаг Hard Drop.
    #[must_use]
    pub fn is_hard_dropping(&self) -> bool {
        self.is_hard_dropping
    }

    /// Получить флаг возможности удержания.
    #[must_use]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Получить кэш для отрисовки.
    #[must_use]
    pub fn get_render_cache(&self) -> &RenderCache {
        &self.render_cache
    }

    /// Получить кэш для отрисовки (мутуабельная ссылка).
    #[must_use]
    pub fn get_render_cache_mut(&mut self) -> &mut RenderCache {
        &mut self.render_cache
    }

    /// Получить кэш для отрисовки (для обратной совместимости).
    #[must_use]
    pub fn render_cache(&self) -> &RenderCache {
        &self.render_cache
    }

    // ========================================================================
    // СЕТТЕРЫ ДЛЯ ПОЛЕЙ
    // ========================================================================

    /// Установить счёт.
    pub fn set_score(&mut self, value: u128) {
        self.score = value;
    }

    /// Установить уровень.
    ///
    /// Уровень не может быть меньше 1.
    pub fn set_level(&mut self, value: u32) {
        self.level = value.max(1);
    }

    /// Установить количество удалённых линий.
    pub fn set_lines_cleared(&mut self, value: u32) {
        self.lines_cleared = value;
    }

    /// Установить скорость падения.
    ///
    /// Скорость ограничена диапазоном от [`INITIAL_FALL_SPD`] до [`MAX_FALL_SPEED`].
    pub fn set_fall_speed(&mut self, value: f32) {
        use super::constants::{INITIAL_FALL_SPD, MAX_FALL_SPEED};
        self.fall_speed = value.clamp(INITIAL_FALL_SPD, MAX_FALL_SPEED);
    }

    /// Установить таймер приземления.
    pub fn set_land_timer(&mut self, value: f64) {
        self.land_timer = value;
    }

    /// Установить расстояние Soft Drop.
    pub fn set_soft_drop_distance(&mut self, value: u32) {
        self.soft_drop_distance = value;
    }

    /// Установить флаг Hard Drop.
    pub fn set_is_hard_dropping(&mut self, value: bool) {
        self.is_hard_dropping = value;
    }

    /// Установить флаг возможности удержания.
    pub fn set_can_hold(&mut self, value: bool) {
        self.can_hold = value;
    }

    /// Установить текущую фигуру.
    pub fn set_curr_shape(&mut self, value: Tetromino) {
        self.curr_shape = value;
    }

    /// Установить следующую фигуру.
    pub fn set_next_shape(&mut self, value: Tetromino) {
        self.next_shape = value;
    }

    /// Установить удержанную фигуру.
    pub fn set_held_shape(&mut self, value: Option<Tetromino>) {
        self.held_shape = value;
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

    /// Получить удержанную фигуру (мутуабельная ссылка).
    #[must_use]
    pub fn get_held_shape_mut(&mut self) -> &mut Option<Tetromino> {
        &mut self.held_shape
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

    /// Получить маску анимации строк.
    #[must_use]
    pub fn animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Установить маску анимации строк.
    pub fn set_animating_rows_mask(&mut self, value: u32) {
        self.animating_rows_mask = value;
    }

    /// Получить маску заполненных линий.
    #[must_use]
    pub fn filled_lines(&self) -> u32 {
        self.filled_lines
    }

    /// Установить маску заполненных линий.
    pub fn set_filled_lines(&mut self, value: u32) {
        self.filled_lines = value;
    }

    /// Добавить очки к текущему счёту.
    pub fn add_score(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    /// Добавить очки к текущему счёту без проверки (для тестов).
    pub fn add_score_no_check(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    /// Добавить количество очищенных линий.
    pub fn add_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = self.lines_cleared.saturating_add(lines);
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

    /// Получить удержанную фигуру (ссылка на ссылку).
    #[must_use]
    pub fn get_held_shape_ref(&self) -> &Option<Tetromino> {
        &self.held_shape
    }

    /// Получить маску анимации строк (для view).
    #[must_use]
    pub fn get_animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask
    }

    /// Получить заполненные линии (для access.rs).
    #[must_use]
    pub fn get_filled_lines(&self) -> u32 {
        self.filled_lines
    }
}
