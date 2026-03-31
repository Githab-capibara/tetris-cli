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
/// - Счёт, уровень, количество линий (через ScoreBoard)
/// - Текущая и следующая фигуры
/// - Удержанная фигура (Hold)
/// - Игровое поле (через GameBoard)
/// - Таймеры и скорость
/// - Статистика игры
/// - Режим игры (GameModeTrait)
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
    /// Инкапсулирует состояние поля (blocks, filled_lines).
    /// Используйте `board()` и `board_mut()` для доступа.
    board: GameBoard,
    /// Состояние счёта и уровней.
    ///
    /// Инкапсулирует состояние очков (score, level, lines_cleared).
    /// Используйте `scoreboard()` и `scoreboard_mut()` для доступа.
    scoreboard: ScoreBoard,

    // ========================================================================
    // === СОСТОЯНИЕ ФИГУР ===
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
    // === СОСТОЯНИЕ АНИМАЦИЙ ===
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

            // Состояние фигур
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,

            // Состояние анимаций
            animating_rows_mask: 0,
            is_hard_dropping: false,

            // Игровая логика
            fall_speed: INITIAL_FALL_SPD,
            land_timer: LAND_TIME_DELAY_S,
            soft_drop_distance: 0,

            // Статистика и режим
            stats,
            mode_trait,

            // Кэш для отрисовки
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

    /// Получить скорость падения.
    #[must_use]
    pub fn fall_speed(&self) -> f32 {
        self.fall_speed
    }

    /// Получить таймер приземления.
    #[must_use]
    pub fn land_timer(&self) -> f64 {
        self.land_timer
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
    pub fn set_score(&mut self, value: u128) {
        self.scoreboard.set_score(value);
    }

    /// Установить уровень.
    ///
    /// Уровень не может быть меньше 1.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn set_level(&mut self, value: u32) {
        self.scoreboard.set_level(value);
    }

    /// Установить количество удалённых линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn set_lines_cleared(&mut self, value: u32) {
        self.scoreboard.set_lines_cleared(value);
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
    /// Возвращает [`GameError::Validation`] если значение является NaN/Infinity или вне диапазона.
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
    pub fn set_fall_speed(&mut self, value: f32) -> Result<(), GameError> {
        use super::constants::{INITIAL_FALL_SPD, MAX_FALL_SPEED};
        use crate::validation::ValidationService;

        // Валидация на NaN и Infinity через централизованный сервис (DRY-2)
        if let Err(e) = ValidationService::validate_f32_finite(value) {
            return Err(GameError::Validation(format!(
                "Неверная скорость падения: {}",
                e.message
            )));
        }

        // Валидация диапазона через ValidationService (вместо clamp)
        if let Err(e) =
            ValidationService::validate_f32_range(value, INITIAL_FALL_SPD, MAX_FALL_SPEED)
        {
            return Err(GameError::Validation(format!(
                "Неверный диапазон скорости: {}",
                e.message
            )));
        }

        self.fall_speed = value;
        Ok(())
    }

    /// Установить таймер приземления.
    ///
    /// # Возвращает
    /// - `Ok(())` если значение установлено успешно
    /// - `Err(GameError::Validation)` если значение невалидно (NaN/Infinity)
    ///
    /// # Errors
    /// Возвращает [`GameError::Validation`] если значение является NaN или Infinity.
    ///
    /// # Валидация (H3)
    /// Проверяет значение на NaN и Infinity. Возвращает ошибку при невалидных значениях.
    /// Отрицательные значения заменяются на 0.
    ///
    /// # DRY-2: Централизация валидации
    /// Использует `ValidationService::validate_f32_finite()` для валидации.
    pub fn set_land_timer(&mut self, value: f64) -> Result<(), GameError> {
        use crate::validation::ValidationService;

        // Валидация на NaN и Infinity через централизованный сервис (DRY-2)
        if let Err(e) = ValidationService::validate_f32_finite(value as f32) {
            return Err(GameError::Validation(format!(
                "Неверный таймер приземления: {}",
                e.message
            )));
        }

        self.land_timer = value.max(0.0);
        Ok(())
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
    pub fn set_filled_lines(&mut self, value: u32) {
        self.board.set_filled_lines_mask(value);
    }

    /// Добавить очки к текущему счёту.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn add_score(&mut self, points: u128) {
        self.scoreboard.add_score(points);
    }

    /// Добавить очки к текущему счёту без проверки (для тестов).
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn add_score_no_check(&mut self, points: u128) {
        self.scoreboard.add_score(points);
    }

    /// Добавить количество очищенных линий.
    ///
    /// # Архитектурные заметки (A1)
    /// Делегирует вызов компоненту `ScoreBoard`.
    pub fn add_lines_cleared(&mut self, lines: u32) {
        self.scoreboard.add_lines_cleared(lines);
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
        let new_speed = self.fall_speed + super::constants::SPD_INC;
        // Валидация: скорость не должна превышать максимальную
        if new_speed <= super::constants::MAX_FALL_SPEED {
            self.fall_speed = new_speed;
        } else {
            self.fall_speed = super::constants::MAX_FALL_SPEED;
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
    pub fn spawn_new_piece(&mut self) -> Option<()> {
        // Предыдущая следующая фигура становится текущей
        self.curr_shape = self.next_shape;

        // Генерируем новую следующую фигуру
        self.next_shape = Tetromino::from_bag(&mut self.bag);

        // Сбрасываем позицию текущей фигуры
        self.curr_shape.pos = (4.0, 0.0);

        // Добавляем в статистику
        self.stats.add_piece(self.curr_shape.shape);

        // Проверяем коллизию при появлении (игра окончена если коллизия)
        if !self.can_move_curr_shape_direction(crate::types::Direction::Down) {
            return None;
        }

        Some(())
    }

    /// Обновить скорость падения на основе уровня.
    ///
    /// Вычисляет скорость по формуле: INITIAL_FALL_SPD + (level - 1) * SPD_INC
    ///
    /// # Исправление M3 (MEDIUM)
    /// Инкапсулирует логику расчёта скорости с валидацией диапазона.
    pub fn update_fall_speed(&mut self) {
        let level = self.level();
        let calculated_speed = INITIAL_FALL_SPD + ((level - 1) as f32 * super::constants::SPD_INC);

        // Валидация: скорость должна быть в допустимых пределах
        self.fall_speed = calculated_speed.clamp(0.0, super::constants::MAX_FALL_SPEED);
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
        self.animating_rows_mask
    }

    fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
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

impl crate::game::scoring::ScoreAccess for GameState {
    fn get_score(&self) -> u128 {
        self.score()
    }

    fn set_score(&mut self, score: u128) {
        self.set_score(score);
    }

    fn add_score(&mut self, points: u128) {
        self.add_score(points);
    }
}

impl crate::game::scoring::LevelAccess for GameState {
    fn get_level(&self) -> u32 {
        self.level()
    }

    fn set_level(&mut self, level: u32) {
        self.set_level(level);
    }
}

impl crate::game::scoring::LinesAccess for GameState {
    fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared()
    }

    fn set_lines_cleared(&mut self, lines: u32) {
        self.set_lines_cleared(lines);
    }

    fn add_lines(&mut self, lines: u32) {
        self.set_lines_cleared(self.lines_cleared() + lines);
    }
}

impl crate::game::scoring::ComboAccess for GameState {
    fn get_combo(&self) -> u32 {
        self.stats().combo_counter()
    }

    fn set_combo(&mut self, combo: u32) {
        self.stats_mut().set_combo_counter(combo);
    }

    fn reset_combo(&mut self) {
        self.stats_mut().set_combo_counter(0);
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod state_tests {
    use super::*;

    /// Тест: проверка методов доступа к состоянию
    ///
    /// Проверяет что методы score(), level(), lines_cleared()
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

    use crate::game::constants::MAX_FALL_SPEED;

    /// Тест: валидация set_fall_speed() через ValidationService без clamp()
    #[test]
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

    /// Тест: обработка NaN в set_fall_speed()
    #[test]
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

        if let Err(GameError::Validation(msg)) = result {
            assert!(
                msg.contains("Неверная скорость падения"),
                "Сообщение должно указывать на ошибку скорости"
            );
        } else {
            panic!("Ожидалась ошибка Validation");
        }
    }

    /// Тест: обработка Infinity в set_fall_speed()
    #[test]
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

    /// Тест: обработка отрицательных значений в set_fall_speed()
    #[test]
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

    /// Тест: обработка значений вне диапазона в set_fall_speed()
    #[test]
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

    /// Тест: set_fall_speed() использует ValidationService (DRY-2)
    #[test]
    fn test_set_fall_speed_uses_validation_service() {
        let mut state = GameState::new();

        // Проверяем что валидные значения устанавливаются
        for valid_speed in [INITIAL_FALL_SPD, 1.0, 5.0, MAX_FALL_SPEED] {
            let result = state.set_fall_speed(valid_speed);
            assert!(
                result.is_ok(),
                "Валидное значение {} должно устанавливаться",
                valid_speed
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
                "Невалидное значение {} должно отклоняться",
                invalid_speed
            );
        }
    }
}
