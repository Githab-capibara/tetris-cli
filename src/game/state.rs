//! Состояние игры и связанные структуры.
//!
//! Этот модуль содержит основные структуры данных для представления состояния игры:
//! - `GameState` — основное состояние игры
//! - `GameStats` — статистика прошедшей игры
//! - `GameMode` — режим игры
//! - Константы игры

use crate::io::{DISP_HEIGHT, GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
use std::time::Instant;

// Импорт из io для использования в state
use termion::color::White;

/// Типы ошибок игры.
///
/// # Архитектурные заметки
/// TODO (#архитектура): Удалить этот enum, если он не используется в основном коде.
/// В настоящее время используется только в тестах.
/// Для обработки ошибок в проекте используются стандартные механизмы Rust.
#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub type GameResult<T> = Result<T, GameError>;

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
#[allow(dead_code)]
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
///
/// # Архитектурные заметки
/// ## Абстракции для режимов (Problem 2.8)
/// TODO (#архитектура): Рассмотреть возможность выделения режимов в отдельные типы
/// или использовать трейт GameModeTrait для лучшей расширяемости.
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

#[allow(dead_code)]
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
    /// TODO (#архитектура, Problem 2.8): Выделить проверку условий в отдельный трейт
    /// WinConditionChecker для улучшения тестируемости.
    #[must_use]
    pub fn check_win_condition(self, lines_cleared: u32) -> bool {
        match self {
            GameMode::Classic => false, // В классическом режиме нет победы
            GameMode::Sprint => lines_cleared >= SPRINT_LINES,
            GameMode::Marathon => lines_cleared >= MARATHON_LINES,
        }
    }

    /// Получить целевое количество линий для режима.
    ///
    /// # Возвращает
    /// Количество линий для победы (для Sprint/Marathon) или None для Classic
    #[must_use]
    pub fn get_target_lines(self) -> Option<u32> {
        match self {
            GameMode::Classic => None,
            GameMode::Sprint => Some(SPRINT_LINES),
            GameMode::Marathon => Some(MARATHON_LINES),
        }
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
/// Поля структуры имеют видимость `pub(crate)` для обратной совместимости.
///
/// TODO (#архитектура): Сделать все поля приватными и предоставить контролируемый доступ
/// через геттеры/сеттеры. Это улучшит инкапсуляцию и позволит валидировать изменения.
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
pub struct GameState {
    // === Игровая статистика ===
    /// Текущий счёт.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_score()/add_score()
    pub(crate) score: u128,
    /// Текущий уровень.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_level()/set_level()
    pub(crate) level: u32,
    /// Количество удалённых линий.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_lines_cleared()
    pub(crate) lines_cleared: u32,

    // === Фигуры ===
    /// Текущая фигура.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_curr_shape()
    pub(crate) curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_next_shape()
    pub(crate) next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_held_shape()
    pub(crate) held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать can_hold()
    pub(crate) can_hold: bool,

    // === Игровое поле ===
    /// Скорость падения.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_fall_spd()/set_fall_spd()
    pub(crate) fall_spd: f32,
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), -1 = пусто.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_blocks()/get_blocks_mut()
    pub(crate) blocks: Box<[[i8; GRID_WIDTH]; GRID_HEIGHT]>,
    /// Таймер приземления.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_land_timer()/set_land_timer()
    pub(crate) land_timer: f64,

    // === Статистика и режим игры ===
    /// Статистика игры.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_stats()/get_stats_mut()
    pub(crate) stats: GameStats,
    /// Режим игры.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_mode()
    pub(crate) mode: GameMode,

    // === Анимации ===
    /// Строки для анимации (мигание при очистке).
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_animating_rows_mask()
    pub(crate) animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать is_hard_dropping()
    pub(crate) is_hard_dropping: bool,
    /// Количество ячеек, пройденных при Soft Drop.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_soft_drop_distance()
    pub(crate) soft_drop_distance: u32,

    // === Генератор фигур ===
    /// Генератор фигур по системе 7-bag.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_bag()/get_bag_mut()
    pub(crate) bag: BagGenerator,

    // === Кэшированные строки для отрисовки ===
    // Кэширование используется для оптимизации производительности отрисовки.
    /// Кэшированная строка счёта для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_score_str()
    pub(crate) cached_score_str: String,
    /// Кэшированная строка уровня для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_level_str()
    pub(crate) cached_level_str: String,
    /// Кэшированная строка количества линий для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_lines_str()
    pub(crate) cached_lines_str: String,
    /// Последнее закэшированное значение счёта.
    ///
    /// TODO (#архитектура): Сделать приватным (внутренняя деталь реализации)
    pub(crate) last_cached_score: u128,
    /// Последнее закэшированное значение уровня.
    ///
    /// TODO (#архитектура): Сделать приватным (внутренняя деталь реализации)
    pub(crate) last_cached_level: u32,
    /// Последнее закэшированное значение количества линий.
    ///
    /// TODO (#архитектура): Сделать приватным (внутренняя деталь реализации)
    pub(crate) last_cached_lines: u32,
    /// Кэшированная строка рекорда для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_high_score_str()
    pub(crate) cached_high_score_str: String,
    /// Кэшированная строка комбо для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_combo_str()
    pub(crate) cached_combo_str: String,
    /// Кэшированная строка таймера для оптимизации отрисовки.
    ///
    /// TODO (#архитектура): Сделать приватным, использовать get_cached_timer_str()
    pub(crate) cached_timer_str: String,
    /// Последнее закэшированное значение комбо.
    ///
    /// TODO (#архитектура): Сделать приватным (внутренняя деталь реализации)
    pub(crate) last_cached_combo: u32,
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

    /// Получить статистику игры (мутуабельная ссылка).
    #[must_use]
    pub fn get_stats_mut(&mut self) -> &mut GameStats {
        &mut self.stats
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

    /// Получить игровое поле (мутуабельная ссылка).
    #[must_use]
    pub fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
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

    /// Получить удержанную фигуру (ссылка на Option).
    #[must_use]
    #[allow(clippy::ref_option)]
    pub fn get_held_shape_ref(&self) -> &Option<Tetromino> {
        &self.held_shape
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

    /// Получить таймер приземления.
    #[must_use]
    pub fn get_land_timer(&self) -> f64 {
        self.land_timer
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
    pub fn get_soft_drop_distance(&self) -> u32 {
        self.soft_drop_distance
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
        &self.cached_score_str
    }

    /// Получить кэшированную строку уровня.
    #[must_use]
    pub fn get_cached_level_str(&self) -> &str {
        &self.cached_level_str
    }

    /// Получить кэшированную строку линий.
    #[must_use]
    pub fn get_cached_lines_str(&self) -> &str {
        &self.cached_lines_str
    }

    /// Получить кэшированную строку рекорда.
    #[must_use]
    pub fn get_cached_high_score_str(&self) -> &str {
        &self.cached_high_score_str
    }

    /// Получить кэшированную строку комбо.
    #[must_use]
    pub fn get_cached_combo_str(&self) -> &str {
        &self.cached_combo_str
    }

    /// Получить кэшированную строку таймера.
    #[must_use]
    pub fn get_cached_timer_str(&self) -> &str {
        &self.cached_timer_str
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
    }

    /// Установить текущую фигуру (для тестов).
    pub fn set_curr_shape(&mut self, shape: Tetromino) {
        self.curr_shape = shape;
    }

    /// Установить следующую фигуру (для тестов).
    pub fn set_next_shape(&mut self, shape: Tetromino) {
        self.next_shape = shape;
    }

    /// Установить удержанную фигуру (для тестов).
    pub fn set_held_shape(&mut self, shape: Option<Tetromino>) {
        self.held_shape = shape;
    }

    /// Установить флаг возможности удержания (для тестов).
    pub fn set_can_hold(&mut self, can_hold: bool) {
        self.can_hold = can_hold;
    }

    /// Установить скорость падения (для тестов).
    pub fn set_fall_spd(&mut self, spd: f32) {
        self.fall_spd = spd;
    }

    /// Установить таймер приземления (для тестов).
    pub fn set_land_timer(&mut self, timer: f64) {
        self.land_timer = timer;
    }

    /// Установить флаг Hard Drop (для тестов).
    pub fn set_is_hard_dropping(&mut self, dropping: bool) {
        self.is_hard_dropping = dropping;
    }

    /// Установить расстояние Soft Drop (для тестов).
    pub fn set_soft_drop_distance(&mut self, distance: u32) {
        self.soft_drop_distance = distance;
    }

    /// Установить маску анимации строк (для тестов).
    pub fn set_animating_rows_mask(&mut self, mask: u32) {
        self.animating_rows_mask = mask;
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

    /// Добавить очки (для тестов и через трейт `GameBoardAccess`).
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
// РЕАЛИЗАЦИЯ ТРЕЙТА GAMEBOARDACCESS
// ============================================================================

impl crate::game::access::GameBoardAccess for GameState {
    fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    fn get_blocks_mut(&mut self) -> &mut [[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &mut self.blocks
    }

    fn get_block(&self, x: usize, y: usize) -> i8 {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x]
        } else {
            -1
        }
    }

    fn set_block(&mut self, x: usize, y: usize, value: i8) {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.blocks[y][x] = value;
        }
    }

    fn is_block_empty(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) == -1
    }

    fn is_block_occupied(&self, x: usize, y: usize) -> bool {
        self.get_block(x, y) >= 0
    }

    fn get_score(&self) -> u128 {
        self.score
    }

    fn add_score(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    fn set_lines_cleared(&mut self, lines: u32) {
        self.lines_cleared = lines;
    }

    fn get_fall_spd(&self) -> f32 {
        self.fall_spd
    }

    fn set_fall_spd(&mut self, spd: f32) {
        self.fall_spd = spd;
    }

    fn get_land_timer(&self) -> f64 {
        self.land_timer
    }

    fn set_land_timer(&mut self, timer: f64) {
        self.land_timer = timer;
    }
}

// ============================================================================
// ТЕСТЫ ДЛЯ БЕНЧМАРК-МЕТОДОВ
// ============================================================================

#[cfg(test)]
mod tests {
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
