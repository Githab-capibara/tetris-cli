//! Состояние игры и связанные структуры.
//!
//! Модуль содержит основные структуры данных:
//! - `GameState` - основное состояние игры
//! - `GameModeTrait` - трейт режима игры
//! - Константы игры
//!
//! ## Архитектурные заметки
//! ## Исправление #1 (Разделение `GameState`)
//! - `GameStats` перемещён в отдельный модуль [`super::stats`]
//! - `RenderCache` перемещён в отдельный модуль [`super::cache`]
//! - `GameState` использует композицию: содержит `stats: GameStats`, `cache: RenderCache`
//!
//! ## Исправление #12 (MEDIUM SEVERITY) - SOLID принципы
//! Начато разделение `GameState` для соблюдения Single Responsibility Principle:
//! - `GameBoard` (в процессе) - состояние поля (blocks, `filled_lines`)
//! - `ScoreBoard` (в процессе) - состояние очков (score, level, `lines_cleared`)
//! - `FigureManager` (в процессе) - состояние фигур (`curr_shape`, `next_shape`, `held_shape`, bag)
//! - `AnimationState` (в процессе) - состояние анимаций (`animating_rows_mask`, `is_hard_dropping`)
//!
//! ### Выполнено:
//! - ✅ `GameStats` вынесен в отдельный модуль `game/stats.rs`
//! - ✅ `RenderCache` вынесен в отдельный модуль `game/cache.rs`
//! - ✅ `GameState` использует композицию вместо наследования
//! - ✅ `FigureManager` выделен в отдельный компонент `game/components/figure_manager.rs`
//! - ✅ `AnimationState` выделен в отдельный компонент `game/components/animation_state.rs`
//! - ✅ `BoardState`/`FieldState` выделен в отдельный компонент `game/components/board_state.rs`
//!
//! ## Архитектурное улучшение 2026-04-01 (CRITICAL #1)
//! `GameState` разделён на специализированные компоненты:
//! - [`FigureManager`] - управление фигурами (`curr_shape`, `next_shape`, `held_shape`, bag, `can_hold`)
//! - [`AnimationState`] - управление анимациями (`animating_rows_mask`, `is_hard_dropping`)
//! - [`BoardState`]/[`FieldState`] - управление полем (board, `filled_lines_mask`)
//! - `ScoreBoard` - управление очками (score, level, `lines_cleared`)
//! - `GameStats` - статистика игры
//! - `RenderCache` - кэш для отрисовки
//!
//! [`FigureManager`]: crate::game::components::FigureManager
//! [`AnimationState`]: crate::game::components::AnimationState
//! [`BoardState`]: crate::game::components::BoardState
//! [`FieldState`]: crate::game::components::FieldState

#![allow(dead_code)]

// std
// (нет импортов std)

// external
// (нет внешних импортов)

// crate
use crate::constants::{GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S};
use crate::io::GRID_HEIGHT;
use crate::tetromino::{BagGenerator, Tetromino};

// self (super)
use super::board::GameBoard;
use super::cache::RenderCache;
use super::components::{AnimationState, FigureManager};
use super::mode_trait::GameModeTrait;
use super::scoreboard::ScoreBoard;

pub use super::stats::GameStats;

/// Позиция появления фигуры по X (центр поля минус половина ширины фигуры).
// cast: usize -> f32, потеря точности допустима: GRID_WIDTH константа (10)
const SPAWN_X: f32 = (GRID_WIDTH as f32 / 2.0) - 1.0;

// ============================================================================
// ТИПЫ ОШИБОК
// ============================================================================

/// Тип результата игры.
///
/// Использует централизованный `GameError` из модуля errors.
/// Для обратной совместимости с тестами.
pub type GameResult<T> = Result<T, crate::errors::GameError>;

// ============================================================================
// РЕЖИМ ИГРЫ
// ============================================================================

/// Режим игры.
///
/// Режимы игры Tetris CLI.
///
/// # Архитектурные заметки
/// ## Абстракции для режимов (Problem 2.8)
/// Этот enum сохраняется для обратной совместимости.
/// Для нового кода рекомендуется использовать `GameModeTrait` напрямую.
///
/// ## Конвертация в трейт
/// Используйте метод `as_trait()` для получения объекта трейта.
///
/// ## Доступные режимы
/// - [`Classic`](GameMode::Classic) — классическая игра до проигрыша
/// - [`Sprint`](GameMode::Sprint) — скоростной режим до 40 линий
/// - [`Marathon`](GameMode::Marathon) — марафон до 150 линий
///
/// # Устарело
/// Используйте [`GameModeTrait`](crate::game::mode_trait::GameModeTrait) напрямую вместо enum.
///
/// ## Пример использования нового API
/// ```
/// use tetris_cli::game::mode_trait::{GameModeTrait, ClassicMode, SprintMode};
///
/// // Вместо GameMode::Classic используйте:
/// let mode: Box<dyn GameModeTrait> = Box::new(ClassicMode);
///
/// // Вместо GameMode::Sprint используйте:
/// let sprint_mode: Box<dyn GameModeTrait> = Box::new(SprintMode::new());
/// ```
///
/// ## Причина депрекейта
/// Enum нарушает принцип открытости/закрытости (OCP) - для добавления нового режима
/// требуется модификация существующего кода. Трейт `GameModeTrait` позволяет
/// добавлять новые режимы без изменения существующего кода.
#[deprecated(since = "23.96.17", note = "Используйте GameModeTrait напрямую")]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    /// Классический режим — игра до проигрыша.
    ///
    /// # Особенности
    /// - Нет условия победы
    /// - Игра заканчивается при заполнении поля
    /// - Бесконечный геймплей с нарастающей сложностью
    Classic,
    /// Спринт — очистить 40 линий как можно быстрее.
    ///
    /// # Особенности
    /// - Цель: 40 линий ([`crate::constants::SPRINT_LINES`])
    /// - Победа при достижении цели
    /// - Таймер для отслеживания времени прохождения
    Sprint,
    /// Марафон — очистить 150 линий с нарастающей сложностью.
    ///
    /// # Особенности
    /// - Цель: 150 линий ([`crate::constants::MARATHON_LINES`])
    /// - Победа при достижении цели
    /// - Повышение уровня каждые 10 линий
    Marathon,
}

// ============================================================================
// МЕТОДЫ ДЛЯ GAMEMODE
// ============================================================================

#[allow(deprecated)]
impl GameMode {
    /// Преобразовать enum в объект трейта `GameModeTrait`.
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

#[allow(deprecated)]
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
    /// Этот метод делегирует вызов трейту `GameModeTrait`.
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
/// - Счёт, уровень, количество линий (через `ScoreBoard`)
/// - Текущая и следующая фигуры
/// - Удержанная фигура (Hold)
/// - Игровое поле (через `GameBoard`)
/// - Таймеры и скорость
/// - Статистика игры
/// - Режим игры (`GameModeTrait`)
///
/// # Архитектурные заметки
/// ## Инкапсуляция и композиция
/// Начиная с версии 23.96.18, `GameState` использует композицию:
/// - `board: GameBoard` - инкапсуляция состояния поля
/// - `scoreboard: ScoreBoard` - инкапсуляция состояния очков
/// - `stats: GameStats` - статистика игры
/// - `mode_trait: Box<dyn GameModeTrait>` - режим игры
///
/// ## Поля структуры
/// Все поля структуры приватные. Доступ осуществляется через методы:
/// - `board()` / `board_mut()` - доступ к игровому полю
/// - `scoreboard()` / `scoreboard_mut()` - доступ к очкам и уровням
/// - `stats()` / `stats_mut()` - доступ к статистике
/// - `get_mode_trait()` - доступ к режиму игры
///
/// # Panics
/// Методы `GameState` не паникуют в нормальных условиях.
/// Паника возможна только при:
/// - Выходе за пределы массива (внутренняя ошибка, не должно происходить)
/// - Отравлении Mutex (в тестах с потоками)
///
/// ## Пример использования
/// ```ignore
/// let mut game = GameState::new();
/// let score = game.score();  // Используйте геттер
/// let level = game.level();  // Используйте геттер
/// let board = game.board();  // Доступ к полю
/// ```
pub struct GameState {
    // ========================================================================
    // === КОМПОНЕНТЫ СОСТОЯНИЯ ===
    // ========================================================================
    /// Состояние игрового поля.
    ///
    /// Инкапсулирует состояние поля (blocks, `filled_lines`).
    /// Используйте `board()` и `board_mut()` для доступа.
    board: GameBoard,
    /// Состояние счёта и уровней.
    ///
    /// Инкапсулирует состояние очков (score, level, `lines_cleared`).
    /// Используйте `scoreboard()` и `scoreboard_mut()` для доступа.
    scoreboard: ScoreBoard,

    // ========================================================================
    // === КОМПОНЕНТЫ СОСТОЯНИЯ (АРХИТЕКТУРНОЕ УЛУЧШЕНИЕ 2026-04-01) ===
    // ========================================================================
    /// Менеджер фигур.
    ///
    /// Инкапсулирует состояние фигур (`curr_shape`, `next_shape`, `held_shape`, `can_hold`, bag).
    /// Используйте `figure_manager()` и `figure_manager_mut()` для доступа.
    ///
    /// Архитектурное улучшение 2026-04-01 (CRITICAL #1): Выделение `FigureManager`
    figure_manager: FigureManager,
    /// Состояние анимаций.
    ///
    /// Инкапсулирует состояние анимаций (`animating_rows_mask`, `is_hard_dropping`).
    /// Используйте `animation_state()` и `animation_state_mut()` для доступа.
    ///
    /// Архитектурное улучшение 2026-04-01 (CRITICAL #1): Выделение `AnimationState`
    animation_state: AnimationState,

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
    /// Использует трейт `GameModeTrait` вместо enum для лучшей расширяемости.
    mode_trait: Box<dyn GameModeTrait>,

    // ========================================================================
    // === КЭШ ДЛЯ ОТРИСОВКИ ===
    // ========================================================================
    /// Кэш для оптимизации отрисовки.
    render_cache: RenderCache,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Создать новое состояние игры.
    #[allow(deprecated)]
    #[must_use = "Состояние игры должно быть использовано"]
    pub fn new() -> Self {
        Self::new_internal(GameMode::Classic, false)
    }

    /// Создать новое состояние игры для режима спринт.
    #[allow(deprecated)]
    #[must_use = "Состояние игры для спринта должно быть использовано"]
    pub fn new_sprint() -> Self {
        Self::new_internal(GameMode::Sprint, true)
    }

    /// Создать новое состояние игры для режима марафон.
    #[allow(deprecated)]
    #[must_use = "Состояние игры для марафона должно быть использовано"]
    pub fn new_marathon() -> Self {
        Self::new_internal(GameMode::Marathon, true)
    }

    /// Внутренний метод создания состояния игры.
    ///
    /// # ISSUE-084: Исправление
    /// Метод разбит на helper функции для улучшения читаемости:
    /// - `create_mode_trait()` - создание режима игры
    /// - `create_initial_stats()` - создание начальной статистики
    /// - `init_render_cache()` - инициализация кэша отрисовки
    #[allow(deprecated)]
    fn new_internal(mode: GameMode, start_timer: bool) -> Self {
        let mode_trait = Self::create_mode_trait(mode);
        let figure_manager = FigureManager::new();
        let stats = Self::create_initial_stats(&figure_manager, start_timer);
        let mut game_state = Self::create_base_state(mode_trait, figure_manager, stats);

        Self::init_render_cache(&mut game_state);
        game_state
    }

    /// Создать объект режима игры из enum.
    ///
    /// # Аргументы
    /// * `mode` - enum режима игры
    ///
    /// # Возвращает
    /// Box<dyn GameModeTrait> с соответствующим режимом
    #[allow(deprecated)]
    fn create_mode_trait(mode: GameMode) -> Box<dyn GameModeTrait> {
        mode.as_trait()
    }

    /// Создать начальную статистику игры.
    ///
    /// # Аргументы
    /// * `figure_manager` - менеджер фигур для получения первой фигуры
    /// * `start_timer` - запустить ли таймер
    ///
    /// # Возвращает
    /// Инициализированный `GameStats`
    fn create_initial_stats(figure_manager: &FigureManager, start_timer: bool) -> GameStats {
        let mut stats = GameStats::new();
        stats.add_piece(figure_manager.curr_shape().shape());
        if start_timer {
            stats.start_timer();
        }
        stats
    }

    /// Создать базовое состояние игры.
    ///
    /// # Аргументы
    /// * `mode_trait` - режим игры
    /// * `figure_manager` - менеджер фигур
    /// * `stats` - статистика игры
    ///
    /// # Возвращает
    /// Базовый `GameState` с инициализированными компонентами
    fn create_base_state(
        mode_trait: Box<dyn GameModeTrait>,
        figure_manager: FigureManager,
        stats: GameStats,
    ) -> Self {
        Self {
            board: GameBoard::new(),
            scoreboard: ScoreBoard::new(),
            figure_manager,
            animation_state: AnimationState::new(),
            fall_speed: INITIAL_FALL_SPD,
            land_timer: LAND_TIME_DELAY_S,
            soft_drop_distance: 0,
            stats,
            mode_trait,
            render_cache: RenderCache::new(),
        }
    }

    /// Инициализировать кэш отрисовки начальными значениями.
    ///
    /// # Аргументы
    /// * `game_state` - состояние игры (изменяемое)
    fn init_render_cache(game_state: &mut Self) {
        game_state.render_cache.init_with_values(0, 1, 0, 0);
    }

    // ========================================================================
    // ГЕТТЕРЫ ДЛЯ ДОСТУПА К ПОЛЯМ
    // ========================================================================

    /// Получить текущий счёт.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[must_use]
    pub fn score(&self) -> u128 {
        self.scoreboard.get_score()
    }

    /// Получить текущий уровень.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[must_use]
    pub fn level(&self) -> u32 {
        self.scoreboard.get_level()
    }

    /// Получить количество удалённых линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[must_use]
    pub fn lines_cleared(&self) -> u32 {
        self.scoreboard.get_lines_cleared()
    }

    /// Получить статистику игры.
    #[must_use]
    pub fn stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить статистику игры (мутуабельная ссылка).
    #[must_use]
    pub fn stats_mut(&mut self) -> &mut GameStats {
        &mut self.stats
    }

    /// Получить режим игры (объект трейта).
    ///
    /// # Возвращает
    /// Ссылка на объект трейта `GameModeTrait`
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

    // ========================================================================
    // НОВЫЕ МЕТОДЫ ДЛЯ КОМПОНЕНТОВ (A1: Разделение ответственности)
    // ========================================================================

    /// Получить доступ к игровому полю.
    ///
    /// # Возвращает
    /// Ссылка на `GameBoard`
    ///
    /// # Пример использования
    /// ```ignore
    /// let state = GameState::new();
    /// let block = state.board().get_block(5, 10);
    /// ```
    #[must_use]
    pub fn board(&self) -> &GameBoard {
        &self.board
    }

    /// Получить мутуабельный доступ к игровому полю.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `GameBoard`
    #[must_use]
    pub fn board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    /// Получить доступ к состоянию счёта.
    ///
    /// # Возвращает
    /// Ссылка на `ScoreBoard`
    ///
    /// # Пример использования
    /// ```ignore
    /// let state = GameState::new();
    /// let score = state.scoreboard().get_score();
    /// ```
    #[must_use]
    pub fn scoreboard(&self) -> &ScoreBoard {
        &self.scoreboard
    }

    /// Получить мутуабельный доступ к состоянию счёта.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `ScoreBoard`
    #[must_use]
    pub fn scoreboard_mut(&mut self) -> &mut ScoreBoard {
        &mut self.scoreboard
    }

    // ========================================================================
    // МЕТОДЫ ДОСТУПА К КОМПОНЕНТАМ (CRITICAL #1)
    // ========================================================================

    /// Получить доступ к менеджеру фигур.
    ///
    /// # Возвращает
    /// Ссылка на `FigureManager`
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Прямой доступ к компоненту `FigureManager` для сложной логики.
    /// Для простых операций используйте специализированные методы:
    /// - `curr_shape()`, `next_shape()`, `held_shape()`, `can_hold()`, `get_bag()`
    #[must_use]
    pub fn figure_manager(&self) -> &FigureManager {
        &self.figure_manager
    }

    /// Получить мутуабельный доступ к менеджеру фигур.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `FigureManager`
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Прямой доступ к компоненту `FigureManager` для сложной логики.
    #[must_use]
    pub fn figure_manager_mut(&mut self) -> &mut FigureManager {
        &mut self.figure_manager
    }

    /// Получить доступ к состоянию анимаций.
    ///
    /// # Возвращает
    /// Ссылка на `AnimationState`
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Прямой доступ к компоненту `AnimationState` для сложной логики.
    /// Для простых операций используйте специализированные методы:
    /// - `is_hard_dropping()`, `animating_rows_mask()`
    #[must_use]
    pub fn animation_state(&self) -> &AnimationState {
        &self.animation_state
    }

    /// Получить мутуабельный доступ к состоянию анимаций.
    ///
    /// # Возвращает
    /// Мутуабельная ссылка на `AnimationState`
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Прямой доступ к компоненту `AnimationState` для сложной логики.
    #[must_use]
    pub fn animation_state_mut(&mut self) -> &mut AnimationState {
        &mut self.animation_state
    }

    /// Получить режим игры (enum для обратной совместимости).
    ///
    /// # Возвращает
    /// Значение enum `GameMode`
    ///
    /// # Архитектурные заметки
    /// Метод сохранён для обратной совместимости с тестами.
    /// Использует `get_mode_trait()` для получения режима.
    #[must_use]
    #[deprecated(since = "23.96.14", note = "Используйте get_mode_trait() вместо enum")]
    #[allow(deprecated)]
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
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `GameBoard`.
    #[must_use]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.board.get_blocks()
    }

    /// Получить игровое поле (мутуабельная ссылка).
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `GameBoard`.
    #[must_use]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        self.board.get_blocks_mut()
    }

    /// Получить текущую фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn curr_shape(&self) -> &Tetromino {
        self.figure_manager.curr_shape()
    }

    /// Получить следующую фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn next_shape(&self) -> &Tetromino {
        self.figure_manager.next_shape()
    }

    /// Получить удержанную фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn held_shape(&self) -> Option<&Tetromino> {
        self.figure_manager.held_shape()
    }

    /// Получить скорость падения.
    #[must_use]
    #[inline]
    pub fn fall_speed(&self) -> f32 {
        self.fall_speed
    }

    /// Получить таймер приземления.
    #[must_use]
    #[inline]
    pub fn land_timer(&self) -> f64 {
        self.land_timer
    }

    /// Получить расстояние Soft Drop.
    #[must_use]
    #[inline]
    pub fn soft_drop_distance(&self) -> u32 {
        self.soft_drop_distance
    }

    /// Получить флаг Hard Drop.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `AnimationState`.
    #[must_use]
    pub fn is_hard_dropping(&self) -> bool {
        self.animation_state.is_hard_dropping()
    }

    /// Получить флаг возможности удержания.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn can_hold(&self) -> bool {
        self.figure_manager.can_hold()
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
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn set_score(&mut self, value: u128) {
        let () = self.scoreboard.set_score(value);
    }

    /// Установить уровень.
    ///
    /// Уровень не может быть меньше 1.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn set_level(&mut self, value: u32) {
        let () = self.scoreboard.set_level(value);
    }

    /// Установить количество удалённых линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn set_lines_cleared(&mut self, value: u32) {
        let () = self.scoreboard.set_lines_cleared(value);
    }

    /// Установить скорость падения.
    ///
    /// Скорость ограничена диапазоном от [`INITIAL_FALL_SPD`] до [`MAX_FALL_SPEED`].
    ///
    /// # Возвращает
    /// - `Ok(())` если значение установлено успешно
    /// - `Err(GameError::Validation)` если значение невалидно (NaN/Infinity или вне диапазона)
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError::Validation`] если значение является NaN/Infinity или вне диапазона.
    ///
    /// # Валидация (H3)
    /// Проверяет значение на NaN и Infinity. Возвращает ошибку при невалидных значениях.
    ///
    /// # DRY-2: Централизация валидации
    /// Использует `ValidationService::validate_f32_finite()` и `validate_f32_range()` для валидации.
    ///
    /// # Исправление аудита 2026-03-31 (HIGH)
    /// Убран избыточный `clamp()` после валидации. Теперь используется только типизированная
    /// валидация через `ValidationService::validate_f32_range()` для предотвращения дублирования.
    ///
    /// # Исправление аудита 2026-04-02 (B1)
    /// Добавлен #[`must_use`] для предотвращения игнорирования ошибок валидации.
    ///
    /// # Errors
    /// Возвращает [`GameError::ValidationError`] если:
    /// - `value` не является конечным числом (NaN или Infinity)
    /// - `value` выходит за пределы диапазона [`INITIAL_FALL_SPD`, `MAX_FALL_SPEED`]
    #[must_use = "Ошибка установки скорости должна быть обработана"]
    pub fn set_fall_speed(&mut self, value: f32) -> Result<(), crate::errors::GameError> {
        use crate::constants::{INITIAL_FALL_SPD, MAX_FALL_SPEED};
        use crate::errors::GameError;
        use crate::validation::ValidationService;

        // Валидация на NaN и Infinity через централизованный сервис (DRY-2)
        if let Err(e) = ValidationService::validate_f32_finite(value) {
            return Err(GameError::ValidationError(format!(
                "Неверная скорость падения: {}",
                e.message
            )));
        }

        // Валидация диапазона через ValidationService (вместо clamp)
        if let Err(e) =
            ValidationService::validate_f32_range(value, INITIAL_FALL_SPD, MAX_FALL_SPEED)
        {
            return Err(GameError::ValidationError(format!(
                "Неверный диапазон скорости: {}",
                e.message
            )));
        }

        self.fall_speed = value;
        Ok(())
    }

    /// Установить таймер приземления.
    ///
    /// # Аргументы
    /// * `value` - значение таймера в секундах
    ///
    /// # Возвращает
    /// - `Ok(())` если значение установлено успешно
    /// - `Err(GameError::ValidationError)` если значение невалидно
    ///
    /// # Errors
    /// Возвращает [`crate::errors::GameError::ValidationError`] в следующих случаях:
    /// - Значение является NaN или Infinity
    /// - Значение отрицательное
    ///
    /// # Валидация (H3)
    /// Проверяет значение на NaN и Infinity. Возвращает ошибку при невалидных значениях.
    /// Отрицательные значения не допускаются.
    ///
    /// # DRY-2: Централизация валидации
    /// Использует прямую проверку на конечность и неотрицательность.
    ///
    /// # Пример использования
    /// ```ignore
    /// let mut state = GameState::new();
    /// state.set_land_timer(0.5)?;  // Ok
    /// state.set_land_timer(-1.0)?; // Ошибка: отрицательное значение
    /// state.set_land_timer(f64::NAN)?; // Ошибка: NaN
    /// ```
    ///
    /// # Исправление аудита 2026-04-02 (B1)
    /// Добавлен #[`must_use`] для предотвращения игнорирования ошибок валидации.
    #[must_use = "Ошибка установки таймера должна быть обработана"]
    pub fn set_land_timer(&mut self, value: f64) -> Result<(), crate::errors::GameError> {
        use crate::errors::GameError;

        // Валидация на NaN и Infinity через прямую проверку (H3)
        if !value.is_finite() {
            return Err(GameError::ValidationError(
                "Неверный таймер приземления: значение не является конечным".to_string(),
            ));
        }

        // Исправление H3: проверка на неотрицательность вместо .max(0.0)
        if value < 0.0 {
            return Err(GameError::ValidationError(format!(
                "Таймер приземления не может быть отрицательным: {value}"
            )));
        }

        self.land_timer = value;
        Ok(())
    }

    /// Установить расстояние Soft Drop.
    pub fn set_soft_drop_distance(&mut self, value: u32) {
        self.soft_drop_distance = value;
    }

    /// Установить флаг Hard Drop.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `AnimationState`.
    pub fn set_is_hard_dropping(&mut self, value: bool) {
        self.animation_state.set_is_hard_dropping(value);
    }

    /// Установить флаг возможности удержания.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    pub fn set_can_hold(&mut self, value: bool) {
        self.figure_manager.set_can_hold(value);
    }

    /// Установить текущую фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    pub fn set_curr_shape(&mut self, value: Tetromino) {
        self.figure_manager.set_curr_shape(value);
    }

    /// Установить следующую фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    pub fn set_next_shape(&mut self, value: Tetromino) {
        self.figure_manager.set_next_shape(value);
    }

    /// Установить удержанную фигуру.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    pub fn set_held_shape(&mut self, value: Option<Tetromino>) {
        self.figure_manager.set_held_shape(value);
    }

    /// Получить текущую фигуру (мутуабельная ссылка).
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        self.figure_manager.curr_shape_mut()
    }

    /// Получить следующую фигуру (мутуабельная ссылка).
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn get_next_shape_mut(&mut self) -> &mut Tetromino {
        self.figure_manager.next_shape_mut()
    }

    /// Получить удержанную фигуру (мутуабельная ссылка).
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn get_held_shape_mut(&mut self) -> &mut Option<Tetromino> {
        self.figure_manager.held_shape_mut()
    }

    /// Получить генератор фигур.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn get_bag(&self) -> &BagGenerator {
        self.figure_manager.bag()
    }

    /// Получить генератор фигур (мутуабельная ссылка).
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    #[must_use]
    pub fn get_bag_mut(&mut self) -> &mut BagGenerator {
        self.figure_manager.bag_mut()
    }

    /// Получить маску анимации строк.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `AnimationState`.
    #[must_use]
    pub fn animating_rows_mask(&self) -> u32 {
        self.animation_state.animating_rows_mask()
    }

    /// Установить маску анимации строк.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `AnimationState`.
    pub fn set_animating_rows_mask(&mut self, value: u32) {
        self.animation_state.set_animating_rows_mask(value);
    }

    /// Получить маску заполненных линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `GameBoard`.
    #[must_use]
    pub fn filled_lines(&self) -> u32 {
        self.board.get_filled_lines_mask()
    }

    /// Установить маску заполненных линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `GameBoard`.
    pub fn set_filled_lines(&mut self, value: u32) {
        self.board.set_filled_lines_mask(value);
    }

    /// Добавить очки к текущему счёту.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    ///
    /// # Исправление аудита 2026-04-02 (H16)
    /// Добавлен #[`must_use`] так как возвращаемое значение (новый счёт) важно.
    #[must_use = "Новый счёт должен быть использован"]
    pub fn add_score(&mut self, points: u128) -> u128 {
        self.scoreboard.add_score(points)
    }

    /// Добавить количество очищенных линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    ///
    /// # Исправление аудита 2026-04-02 (H16)
    /// Добавлен #[`must_use`] так как возвращаемое значение (новое количество линий) важно.
    #[must_use = "Новое количество линий должно быть использовано"]
    pub fn add_lines_cleared(&mut self, lines: u32) -> u32 {
        self.scoreboard.add_lines_cleared(lines)
    }

    /// Увеличить уровень на 1.
    ///
    /// # Возвращает
    /// Новый уровень после увеличения.
    ///
    /// # Архитектурные заметки (A3)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn increment_level(&mut self) -> u32 {
        self.scoreboard.increment_level()
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

    /// Получить маску анимации строк (для view).
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `AnimationState`.
    #[must_use]
    pub fn get_animating_rows_mask(&self) -> u32 {
        self.animation_state.animating_rows_mask()
    }

    // ========================================================================
    // СЕМАНТИЧЕСКИЕ МЕТОДЫ (ИСПРАВЛЕНИЕ M3)
    // ========================================================================
    // Эти методы инкапсулируют сложную логику и валидацию данных

    /// Применить гравитацию к текущей фигуре.
    ///
    /// Увеличивает скорость падения на основе уровня.
    /// Используется при повышении уровня для автоматического увеличения сложности.
    ///
    /// # Исправление M3 (MEDIUM)
    /// Инкапсулирует логику изменения скорости падения с валидацией.
    pub fn apply_gravity(&mut self) {
        use crate::constants::{MAX_FALL_SPEED, SPD_INC};
        let new_speed = self.fall_speed + SPD_INC;
        // Валидация: скорость не должна превышать максимальную
        if new_speed <= MAX_FALL_SPEED {
            self.fall_speed = new_speed;
        } else {
            self.fall_speed = MAX_FALL_SPEED;
        }
    }

    /// Появить новую фигуру из генератора.
    ///
    /// Берёт следующую фигуру из bag, устанавливает её как текущую,
    /// генерирует новую следующую фигуру.
    ///
    /// # Возвращает
    /// - `Some(())` если фигура успешно появлена
    /// - `None` если новая фигура имеет коллизию (игра окончена)
    ///
    /// # Исправление M3 (MEDIUM)
    /// Инкапсулирует логику появления фигур с проверкой коллизий.
    ///
    /// # Архитектурные заметки (CRITICAL #1)
    /// Делегирует вызов компоненту `FigureManager`.
    pub fn spawn_new_piece(&mut self) -> Option<()> {
        // Используем FigureManager для появления новой фигуры
        self.figure_manager.spawn_new_piece();

        // Сбрасываем позицию текущей фигуры
        *self.figure_manager.curr_shape_mut().pos_mut() = (SPAWN_X, 0.0);

        // Добавляем в статистику
        self.stats
            .add_piece(self.figure_manager.curr_shape().shape());

        // Проверяем коллизию при появлении (игра окончена если коллизия)
        if !self.can_move_curr_shape_direction(crate::types::Direction::Down) {
            return None;
        }

        Some(())
    }

    /// Обновить скорость падения на основе уровня.
    ///
    /// Вычисляет скорость по формуле: `INITIAL_FALL_SPD` + (level - 1) * `SPD_INC`
    ///
    /// # Исправление M3 (MEDIUM)
    /// Инкапсулирует логику расчёта скорости с валидацией диапазона.
    pub fn update_fall_speed(&mut self) {
        use crate::constants::{INITIAL_FALL_SPD, MAX_FALL_SPEED, SPD_INC};
        let level = self.level();
        // Потеря точности допустима: level <= 15 (максимум для тетриса)
        #[allow(clippy::cast_precision_loss)]
        let calculated_speed = INITIAL_FALL_SPD + ((level - 1) as f32 * SPD_INC);

        // Валидация: скорость должна быть в допустимых пределах
        self.fall_speed = calculated_speed.clamp(INITIAL_FALL_SPD, MAX_FALL_SPEED);
    }
}

// ============================================================================
// РЕАЛИЗАЦИЯ TRAIT SCORINGSTATE ДЛЯ GameState
// ============================================================================

impl super::scoring::ScoringState for GameState {
    fn fall_speed(&self) -> f32 {
        self.fall_speed()
    }

    fn set_fall_speed(&mut self, speed: f32) -> Result<(), &'static str> {
        match self.set_fall_speed(speed) {
            Ok(()) => Ok(()),
            Err(_) => Err("Ошибка установки скорости падения"),
        }
    }

    fn animating_rows_mask(&self) -> u32 {
        self.animating_rows_mask()
    }

    fn set_animating_rows_mask(&mut self, mask: u32) {
        self.set_animating_rows_mask(mask);
    }

    fn stats(&self) -> &crate::game::stats::GameStats {
        self.stats()
    }

    fn stats_mut(&mut self) -> &mut crate::game::stats::GameStats {
        self.stats_mut()
    }

    fn get_blocks(&self) -> &[[i8; crate::io::GRID_WIDTH]; crate::io::GRID_HEIGHT] {
        self.get_blocks()
    }

    fn get_blocks_mut(&mut self) -> &mut [[i8; crate::io::GRID_WIDTH]; crate::io::GRID_HEIGHT] {
        self.get_blocks_mut()
    }
}

// ============================================================================
// ISP-1: РЕАЛИЗАЦИЯ УЗКИХ ТРЕЙТОВ ДЛЯ GameState
// ============================================================================

// Архитектурное улучшение 2026-04-01 (DRY2): ScoreAccess определён в access.rs
// Реализации трейтов ScoreAccess, LevelAccess, LinesAccess, ComboAccess
// перемещены в access.rs для централизации и устранения дублирования.
// См. src/game/access.rs для реализаций.

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod state_tests {
    use super::*;

    /// Тест: проверка методов доступа к состоянию
    ///
    /// Проверяет что методы `score()`, `level()`, `lines_cleared()`
    /// возвращают корректные начальные значения.
    #[test]
    fn test_state_initial_values() {
        let state = GameState::new();

        // Проверка начальных значений
        assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
        assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
        assert_eq!(
            state.lines_cleared(),
            0,
            "Начальное количество линий должно быть 0"
        );

        // Проверка что методы возвращают те же значения что и scoreboard
        assert_eq!(state.score(), state.scoreboard().get_score());
        assert_eq!(state.level(), state.scoreboard().get_level());
        assert_eq!(
            state.lines_cleared(),
            state.scoreboard().get_lines_cleared()
        );
    }

    // =========================================================================
    // ТЕСТЫ ДЛЯ ИСПРАВЛЕНИЯ АУДИТА 2026-03-31: DRY-2 ВАЛИДАЦИЯ set_fall_speed()
    // =========================================================================

    use crate::constants::MAX_FALL_SPEED;

    /// Тест: валидация `set_fall_speed()` через `ValidationService` без `clamp()`
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_validation_no_clamp() {
        let mut state = GameState::new();

        // Валидное значение в диапазоне должно устанавливаться без изменений
        let result = state.set_fall_speed(2.0);
        assert!(result.is_ok(), "Валидное значение должно устанавливаться");
        assert_eq!(
            state.fall_speed(),
            2.0,
            "Значение должно установиться точно"
        );

        // Значение на нижней границе
        let result = state.set_fall_speed(INITIAL_FALL_SPD);
        assert!(
            result.is_ok(),
            "Значение на нижней границе должно быть валидно"
        );

        // Значение на верхней границе
        let result = state.set_fall_speed(MAX_FALL_SPEED);
        assert!(
            result.is_ok(),
            "Значение на верхней границе должно быть валидно"
        );
    }

    /// Тест: обработка NaN в `set_fall_speed()`
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_nan_rejected() {
        let mut state = GameState::new();
        let initial_speed = state.fall_speed();

        let result = state.set_fall_speed(f32::NAN);
        assert!(result.is_err(), "NaN должен быть отклонён");
        assert_eq!(
            state.fall_speed(),
            initial_speed,
            "Скорость не должна измениться после NaN"
        );

        if let Err(crate::errors::GameError::ValidationError(msg)) = result {
            assert!(
                msg.contains("Неверная скорость падения"),
                "Сообщение должно указывать на ошибку скорости"
            );
        } else {
            panic!("Ожидалась ошибка Validation");
        }
    }

    /// Тест: обработка Infinity в `set_fall_speed()`
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_infinity_rejected() {
        let mut state = GameState::new();
        let initial_speed = state.fall_speed();

        let result = state.set_fall_speed(f32::INFINITY);
        assert!(result.is_err(), "Infinity должен быть отклонён");
        assert_eq!(
            state.fall_speed(),
            initial_speed,
            "Скорость не должна измениться после Infinity"
        );

        let result = state.set_fall_speed(f32::NEG_INFINITY);
        assert!(result.is_err(), "Negative Infinity должен быть отклонён");
    }

    /// Тест: обработка отрицательных значений в `set_fall_speed()`
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_negative_rejected() {
        let mut state = GameState::new();
        let initial_speed = state.fall_speed();

        let result = state.set_fall_speed(-1.0);
        assert!(
            result.is_err(),
            "Отрицательное значение должно быть отклонено"
        );
        assert_eq!(
            state.fall_speed(),
            initial_speed,
            "Скорость не должна измениться после отрицательного значения"
        );
    }

    /// Тест: обработка значений вне диапазона в `set_fall_speed()`
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_out_of_range() {
        let mut state = GameState::new();
        let initial_speed = state.fall_speed();

        // Значение ниже минимума
        let result = state.set_fall_speed(INITIAL_FALL_SPD - 0.1);
        assert!(
            result.is_err(),
            "Значение ниже минимума должно быть отклонено"
        );
        assert_eq!(
            state.fall_speed(),
            initial_speed,
            "Скорость не должна измениться"
        );

        // Значение выше максимума
        let result = state.set_fall_speed(MAX_FALL_SPEED + 1.0);
        assert!(
            result.is_err(),
            "Значение выше максимума должно быть отклонено"
        );
        assert_eq!(
            state.fall_speed(),
            initial_speed,
            "Скорость не должна измениться"
        );
    }

    /// Тест: `set_fall_speed()` использует `ValidationService` (DRY-2)
    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_set_fall_speed_uses_validation_service() {
        let mut state = GameState::new();

        // Проверяем что валидные значения устанавливаются
        for valid_speed in [INITIAL_FALL_SPD, 1.0, 5.0, MAX_FALL_SPEED] {
            let result = state.set_fall_speed(valid_speed);
            assert!(
                result.is_ok(),
                "Валидное значение {valid_speed} должно устанавливаться"
            );
        }

        // Проверяем что невалидные значения отклоняются
        for invalid_speed in [
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            -1.0,
            INITIAL_FALL_SPD - 0.1,
        ] {
            let result = state.set_fall_speed(invalid_speed);
            assert!(
                result.is_err(),
                "Невалидное значение {invalid_speed} должно отклоняться"
            );
        }
    }
}
