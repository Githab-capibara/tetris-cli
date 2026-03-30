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
//!
//! ## Исправление #12 (MEDIUM SEVERITY) - SOLID принципы
//! Начато разделение GameState для соблюдения Single Responsibility Principle:
//! - `GameBoard` (в процессе) - состояние поля (blocks, filled_lines)
//! - `ScoreBoard` (в процессе) - состояние очков (score, level, lines_cleared)
//! - `FigureManager` (в процессе) - состояние фигур (curr_shape, next_shape, held_shape, bag)
//! - `AnimationState` (в процессе) - состояние анимаций (animating_rows_mask, is_hard_dropping)
//!
//! ### Выполнено:
//! - ✅ `GameStats` вынесен в отдельный модуль `game/stats.rs`
//! - ✅ `RenderCache` вынесен в отдельный модуль `game/cache.rs`
//! - ✅ GameState использует композицию вместо наследования
//!
//! ### В процессе (TODO):
//! - ⏳ Выделить `GameBoard` - инкапсуляция состояния поля
//! - ⏳ Выделить `ScoreBoard` - инкапсуляция состояния очков
//! - ⏳ Выделить `FigureManager` - управление фигурами
//! - ⏳ Выделить `AnimationState` - управление анимациями

use crate::io::GRID_HEIGHT;
use crate::tetromino::{BagGenerator, Tetromino};

use super::board::GameBoard;
use super::cache::RenderCache;
use super::constants::{GRID_WIDTH, INITIAL_FALL_SPD, LAND_TIME_DELAY_S};
use super::mode_trait::GameModeTrait;
use super::scoreboard::ScoreBoard;
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
///
/// ## Архитектурные заметки
///
/// Рассмотреть разделение на компоненты:
/// - `GameBoard` - состояние поля (blocks, filled_lines)
/// - `ScoreBoard` - очки (score, level, lines_cleared)
/// - `FigureManager` - фигуры (curr_shape, next_shape, held_shape, bag)
/// - `AnimationState` - анимации (animating_rows_mask, is_hard_dropping)
///
/// Текущая структура нарушает Single Responsibility Principle.
///
/// ## Исправление A1 (HIGH) - Разделение ответственности
/// Начиная с версии 23.96.18, `GameState` использует композицию:
/// - `board: GameBoard` - инкапсуляция состояния поля
/// - `scoreboard: ScoreBoard` - инкапсуляция состояния очков
///
/// Старые поля (`blocks`, `filled_lines`, `score`, `level`, `lines_cleared`)
/// сохранены как deprecated для обратной совместимости и делегируют вызовы
/// новым компонентам.
pub struct GameState {
    // ========================================================================
    // === НОВЫЕ КОМПОНЕНТЫ (A1: Разделение ответственности) ===
    // ========================================================================
    /// Состояние игрового поля.
    ///
    /// Инкапсулирует состояние поля (blocks, filled_lines).
    /// Новый способ доступа к полю - используйте `board()` и `board_mut()`.
    board: GameBoard,
    /// Состояние счёта и уровней.
    ///
    /// Инкапсулирует состояние очков (score, level, lines_cleared).
    /// Новый способ доступа к очкам - используйте `scoreboard()` и `scoreboard_mut()`.
    scoreboard: ScoreBoard,

    // ========================================================================
    // === СОСТОЯНИЕ ПОЛЯ (будущий GameBoard) ===
    // ========================================================================
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), -1 = пусто.
    ///
    /// # Устарело
    /// Используйте `board.get_blocks()` вместо прямого доступа.
    #[deprecated(since = "23.96.18", note = "Используйте board.get_blocks()")]
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Битовая маска заполненных линий (для будущей оптимизации).
    ///
    /// # Устарело
    /// Используйте `board.get_filled_lines_mask()` вместо прямого доступа.
    #[deprecated(since = "23.96.18", note = "Используйте board.get_filled_lines_mask()")]
    filled_lines: u32,

    // ========================================================================
    // === СОСТОЯНИЕ ОЧКОВ (будущий ScoreBoard) ===
    // ========================================================================
    /// Текущий счёт.
    ///
    /// # Устарело
    /// Используйте `scoreboard.get_score()` вместо прямого доступа.
    #[deprecated(since = "23.96.18", note = "Используйте scoreboard.get_score()")]
    score: u128,
    /// Текущий уровень.
    ///
    /// # Устарело
    /// Используйте `scoreboard.get_level()` вместо прямого доступа.
    #[deprecated(since = "23.96.18", note = "Используйте scoreboard.get_level()")]
    level: u32,
    /// Количество удалённых линий.
    ///
    /// # Устарело
    /// Используйте `scoreboard.get_lines_cleared()` вместо прямого доступа.
    #[deprecated(
        since = "23.96.18",
        note = "Используйте scoreboard.get_lines_cleared()"
    )]
    lines_cleared: u32,

    // ========================================================================
    // === СОСТОЯНИЕ ФИГУР (будущий FigureManager) ===
    // ========================================================================
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
    #[allow(deprecated)]
    pub fn new() -> Self {
        Self::new_internal(GameMode::Classic, false)
    }

    /// Создать новое состояние игры для режима спринт.
    #[allow(deprecated)]
    pub fn new_sprint() -> Self {
        Self::new_internal(GameMode::Sprint, true)
    }

    /// Создать новое состояние игры для режима марафон.
    #[allow(deprecated)]
    pub fn new_marathon() -> Self {
        Self::new_internal(GameMode::Marathon, true)
    }

    /// Внутренний метод создания состояния игры.
    #[allow(deprecated)]
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

        // Создаём новые компоненты
        let board = GameBoard::new();
        let scoreboard = ScoreBoard::new();

        let mut game_state = Self {
            // Новые компоненты
            board,
            scoreboard,

            // Старые поля для обратной совместимости (deprecated)
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

    /// Получить текущий счёт (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`score()`] вместо этого.
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
    pub fn get_score(&self) -> u128 {
        self.score()
    }

    /// Получить текущий уровень (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`level()`] вместо этого.
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
    pub fn get_level(&self) -> u32 {
        self.level()
    }

    /// Получить количество удалённых линий (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`lines_cleared()`] вместо этого.
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
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
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
    pub fn get_curr_shape(&self) -> &Tetromino {
        self.curr_shape()
    }

    /// Получить следующую фигуру (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`next_shape()`] вместо этого.
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
    pub fn get_next_shape(&self) -> &Tetromino {
        self.next_shape()
    }

    /// Получить удержанную фигуру (устаревшее имя).
    ///
    /// # Устарело
    /// Используйте [`held_shape()`] вместо этого.
    ///
    /// # Планируется к удалению
    /// Будет удалён в версии 24.x.x после завершения миграции на новые методы.
    #[must_use]
    #[deprecated(
        since = "23.96.14",
        note = "Используйте score(), board(), level() вместо get_*"
    )]
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
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn set_score(&mut self, value: u128) {
        self.scoreboard.set_score(value);
        self.score = value; // Для обратной совместимости
    }

    /// Установить уровень.
    ///
    /// Уровень не может быть меньше 1.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn set_level(&mut self, value: u32) {
        self.scoreboard.set_level(value);
        self.level = value.max(1); // Для обратной совместимости
    }

    /// Установить количество удалённых линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn set_lines_cleared(&mut self, value: u32) {
        self.scoreboard.set_lines_cleared(value);
        self.lines_cleared = value; // Для обратной совместимости
    }

    /// Установить скорость падения.
    ///
    /// Скорость ограничена диапазоном от [`INITIAL_FALL_SPD`] до [`MAX_FALL_SPEED`].
    ///
    /// # Валидация (H3)
    /// Проверяет значение на NaN и Infinity. Неверные значения игнорируются.
    pub fn set_fall_speed(&mut self, value: f32) {
        use super::constants::{INITIAL_FALL_SPD, MAX_FALL_SPEED};

        // Валидация на NaN и Infinity (H3)
        if value.is_nan() || value.is_infinite() {
            eprintln!(
                "[ERROR] Неверная скорость падения: {} (NaN/Infinity)",
                value
            );
            return;
        }

        self.fall_speed = value.clamp(INITIAL_FALL_SPD, MAX_FALL_SPEED);
    }

    /// Установить таймер приземления.
    ///
    /// # Валидация (H3)
    /// Проверяет значение на NaN и Infinity. Неверные значения игнорируются.
    /// Отрицательные значения заменяются на 0.
    pub fn set_land_timer(&mut self, value: f64) {
        // Валидация на NaN и Infinity (H3)
        if value.is_nan() || value.is_infinite() {
            eprintln!(
                "[ERROR] Неверный таймер приземления: {} (NaN/Infinity)",
                value
            );
            return;
        }

        self.land_timer = value.max(0.0);
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
    #[allow(deprecated)]
    pub fn set_filled_lines(&mut self, value: u32) {
        self.board.set_filled_lines_mask(value);
        self.filled_lines = value; // Для обратной совместимости
    }

    /// Добавить очки к текущему счёту.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn add_score(&mut self, points: u128) {
        self.scoreboard.add_score(points);
        self.score = self.score.saturating_add(points); // Для обратной совместимости
    }

    /// Добавить очки к текущему счёту без проверки (для тестов).
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn add_score_no_check(&mut self, points: u128) {
        self.scoreboard.add_score(points);
        self.score = self.score.saturating_add(points); // Для обратной совместимости
    }

    /// Добавить количество очищенных линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    #[allow(deprecated)]
    pub fn add_lines_cleared(&mut self, lines: u32) {
        self.scoreboard.add_lines_cleared(lines);
        self.lines_cleared = self.lines_cleared.saturating_add(lines); // Для обратной совместимости
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

    /// Получить удержанную фигуру (ссылка на ссылку).
    #[must_use]
    #[allow(clippy::ref_option)]
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
    #[allow(deprecated)]
    pub fn get_filled_lines(&self) -> u32 {
        self.filled_lines
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod state_tests {
    use super::*;

    /// Тест: проверка что deprecated методы всё ещё работают
    ///
    /// Проверяет что устаревшие методы помечены корректно
    /// и всё ещё функционируют для обратной совместимости.
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_methods_still_work() {
        // Проверка что deprecated методы всё ещё работают
        // и помечены корректно
        let state = GameState::new();

        // Тест deprecated методов get_*
        let _score = state.get_score();
        let _level = state.get_level();
        let _lines = state.get_lines_cleared();

        // Проверка что методы возвращают корректные значения
        assert_eq!(state.get_score(), 0, "Начальный счёт должен быть 0");
        assert_eq!(state.get_level(), 1, "Начальный уровень должен быть 1");
        assert_eq!(
            state.get_lines_cleared(),
            0,
            "Начальное количество линий должно быть 0"
        );

        // Проверка что новые методы возвращают те же значения
        assert_eq!(state.score(), state.get_score());
        assert_eq!(state.level(), state.get_level());
        assert_eq!(state.lines_cleared(), state.get_lines_cleared());
    }
}
