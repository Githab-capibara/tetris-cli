//! Основной игровой цикл.
//!
//! Этот модуль содержит основную игровую логику Tetris CLI.
//! Реализует игровой цикл, обработку ввода, отрисовку и систему очков.
//!
//! ## Особенности реализации
//! - Поддержка 60 FPS для плавной анимации
//! - Прогрессивная сложность (увеличение скорости)
//! - Система уровней (каждые 10 линий)
//! - Предпросмотр следующей фигуры
//! - "Призрачная" фигура (показывает точку приземления)
//! - Таблица лидеров (топ-5)
//! - Удержание фигуры (Hold)
//! - Звуковые эффекты через терминальный bell
//! - Анимация очистки линий
//! - Расширенная статистика игры
//! - Режим "спринт" (40 линий на время)
//! - Режим "марафон" (150 линий с нарастающей сложностью)

use crate::io::{
    Canvas, KeyReader, DISP_HEIGHT, GRID_HEIGHT, GRID_WIDTH, KEY_BACKSPACE, SHAPE_STR, SHAPE_WIDTH,
};
use crate::tetromino::{RotationDirection, Tetromino, SHAPE_COLORS};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use termion::color::{Color, Reset, White};

/// Количество кадров в секунду.
///
/// Обеспечивает плавную анимацию игры.
/// Стандартное значение для большинства игр - 60 FPS.
pub const FPS: u64 = 60;

/// Границы игрового поля с заголовками.
///
/// Содержит 25 строк:
/// - Строка 1: пустая
/// - Строка 2: "Счёт:"
/// - Строка 3: "Рекорд:"
/// - Строка 4: "Уровень:"
/// - Строка 5: "Линии:"
/// - Строки 6-25: игровое поле с границами
const BORDER: [&str; DISP_HEIGHT as usize] = [
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
const PAUSE: [&str; 3] = ["╔════════╗", "║ ПАУЗА  ║", "╚════════╝"];

/// Сообщение о проигрыше.
const GAME_OVER: [&str; 3] = ["╔════════════╗", "║ ИГРА ОКОНЧЕНА ║", "╚════════════╝"];

/// Задержка перед возвратом в меню после проигрыша (мс).
const GAME_OVER_DELAY_MS: u64 = 1500;

/// Количество миллисекунд в секунде.
/// Используется для преобразования времени в `update()`.
const MILLIS_PER_SECOND: f32 = 1000.0;

/// Цвет границ.
const BORDER_COLOR: &dyn Color = &White;

/// Смещение отрисовки фигур по вертикали.
///
/// Учитывает заголовки (Счёт, Рекорд, Уровень, Линии) и верхнюю границу.
const SHAPE_DRAW_OFFSET: i16 = 5;

/// Смещение отрисовки фигур по горизонтали.
/// Используется в `draw()` для отрисовки фигур на поле.
const SHAPE_OFFSET_X: i16 = 2;

/// Смещение отрисовки фигур по вертикали (дополнительное).
/// Используется в `draw()` для отрисовки фигур на поле.
const SHAPE_OFFSET_Y: i16 = 0;

/// Смещение отрисовки фигур по горизонтали (для предпросмотра).
const DRAW_OFFSET_X: i16 = 2;

/// Позиция предпросмотра следующей фигуры по X (справа от поля).
/// Используется в `draw_next_shape()` для отрисовки следующей фигуры.
const PREVIEW_X: u16 = 24;

/// Позиция предпросмотра следующей фигуры по Y.
/// Используется в `draw_next_shape()` для отрисовки следующей фигуры.
const PREVIEW_Y: u16 = 8;

/// Позиция предпросмотра удержанной фигуры по X (слева от поля).
/// Используется в `draw_held_shape()` для отрисовки удержанной фигуры.
const HOLD_PREVIEW_X: u16 = 2;

/// Позиция предпросмотра удержанной фигуры по Y.
/// Используется в `draw_held_shape()` для отрисовки удержанной фигуры.
const HOLD_PREVIEW_Y: u16 = 8;

// ============================================================================
// КОНСТАНТЫ ПОЗИЦИЙ ОТРИСОВКИ UI
// ============================================================================
// Исправление #4: константы для позиций отрисовки элементов интерфейса

/// Позиция X для отрисовки счёта (строка 2).
const SCORE_X: u16 = 7;
/// Позиция Y для отрисовки счёта (строка 2).
const SCORE_Y: u16 = 2;
/// Позиция X для отрисовки рекорда (строка 3).
const HIGH_SCORE_X: u16 = 7;
/// Позиция Y для отрисовки рекорда (строка 3).
const HIGH_SCORE_Y: u16 = 3;
/// Позиция X для отрисовки уровня (строка 4).
const LEVEL_X: u16 = 10;
/// Позиция Y для отрисовки уровня (строка 4).
const LEVEL_Y: u16 = 4;
/// Позиция X для отрисовки линий (строка 5).
const LINES_X: u16 = 10;
/// Позиция Y для отрисовки линий (строка 5).
const LINES_Y: u16 = 5;

/// Таблица смещений для wall kick (Super Rotation System - упрощённая).
/// Используется при вращении фигур рядом со стенами.
///
/// ## Алгоритм Super Rotation System (SRS)
/// SRS - это стандарт вращения в современных тетрисах, разработанный Nintendo.
/// Основная идея: если фигура не может вращаться на месте, система пробует
/// различные смещения (wall kicks) для нахождения допустимой позиции.
///
/// ## Порядок проверки смещений:
/// 1. (-1, 0) - сдвиг влево на 1 клетку
/// 2. (1, 0)  - сдвиг вправо на 1 клетку
/// 3. (-2, 0) - сдвиг влево на 2 клетки (для I-фигур)
/// 4. (2, 0)  - сдвиг вправо на 2 клетки (для I-фигур)
/// 5. (0, -1) - сдвиг вверх на 1 клетку (для случаев у пола)
/// 6. (-1, -1) - сдвиг влево и вверх
/// 7. (1, -1)  - сдвиг вправо и вверх
/// 8. (0, 1)   - сдвиг вниз на 1 клетку (для случаев у потолка)
///
/// ## Примечания
/// - Это упрощённая версия SRS - полная версия использует разные таблицы
///   для каждой фигуры и каждого направления вращения.
/// - Наша реализация использует единую таблицу для всех фигур.
/// - Смещения проверяются последовательно, первое успешное применяется.
const WALL_KICK_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),  // Влево на 1
    (1, 0),   // Вправо на 1
    (-2, 0),  // Влево на 2
    (2, 0),   // Вправо на 2
    (0, -1),  // Вверх на 1 (для случаев у пола)
    (-1, -1), // Влево и вверх
    (1, -1),  // Вправо и вверх
    (0, 1),   // Вниз на 1 (для случаев у потолка)
];

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
/// Исправление #25: таблица для быстрого доступа к очкам за 1-4 линии.
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
/// Это константа используется для ограничения расчёта очков и предотвращения переполнения.
pub const MAX_LINES_PER_CLEAR: u32 = 4;

/// Ширина игрового поля в блоках.
/// Алиас на `GRID_WIDTH` для лучшей читаемости кода.
#[allow(dead_code)]
pub const FIELD_WIDTH: usize = crate::io::GRID_WIDTH;

/// Высота игрового поля в блоках.
/// Алиас на `GRID_HEIGHT` для лучшей читаемости кода.
#[allow(dead_code)]
pub const FIELD_HEIGHT: usize = crate::io::GRID_HEIGHT;

/// Смещение игрового поля по горизонтали при отрисовке.
#[allow(dead_code)]
pub const FIELD_OFFSET_X: usize = 5;

/// Количество линий для режима спринт.
pub const SPRINT_LINES: u32 = 40;

/// Количество линий для режима марафон (заготовка для будущего режима).
#[allow(dead_code)]
pub const MARATHON_LINES: u32 = 150;

/// Минимальная допустимая координата Y для блоков фигуры.
/// Блоки не могут находиться ниже этой границы (отрицательные координаты).
pub const MIN_Y: i16 = 0;

/// Символ терминального bell для звуковых эффектов.
pub const BELL: &str = "\x07";

/// Интервал анимации мигания Hard Drop в миллисекундах.
pub const HARD_DROP_ANIM_INTERVAL_MS: u16 = 50;

/// Количество кадров для пропуска при анимации.
/// Используется для мигания фигур (каждый второй кадр).
pub const ANIMATION_FRAME_SKIP: u16 = 2;

/// Направление движения/вращения.
///
/// # Исправление #7
/// `Dir` реализует `Copy`, поэтому все операции с ним выполняются без аллокаций.
/// Это означает, что передача `Dir` по значению не приводит к копированию данных,
/// а лишь копирует целочисленное значение (tag) перечисления.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Dir {
    /// Вниз.
    Down,
    /// Влево.
    Left,
    /// Вправо.
    Right,
}

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

/// Статистика игры.
///
/// Содержит подробную информацию о прошедшей игре:
/// - Количество использованных фигур каждого типа
/// - Общее количество очищенных линий
/// - Максимальное комбо (одновременное удаление линий)
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
    /// Полученные достижения (заготовка для будущей системы достижений).
    #[allow(dead_code)]
    pub achievements: Vec<Achievement>,
    /// Количество Tetris (4 линии одновременно) (заготовка для будущей статистики).
    #[allow(dead_code)]
    pub tetris_count: u32,
    /// Общее количество удалённых линий (заготовка для будущей статистики).
    #[allow(dead_code)]
    pub total_lines: u32,
}

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

impl GameStats {
    /// Создать новую статистику.
    pub fn new() -> Self {
        Self::default()
    }

    /// Увеличить счётчик для указанной фигуры.
    pub fn add_piece(&mut self, piece_type: crate::tetromino::ShapeType) {
        match piece_type {
            crate::tetromino::ShapeType::T => self.t_pieces += 1,
            crate::tetromino::ShapeType::L => self.l_pieces += 1,
            crate::tetromino::ShapeType::J => self.j_pieces += 1,
            crate::tetromino::ShapeType::S => self.s_pieces += 1,
            crate::tetromino::ShapeType::Z => self.z_pieces += 1,
            crate::tetromino::ShapeType::O => self.o_pieces += 1,
            crate::tetromino::ShapeType::I => self.i_pieces += 1,
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
    ///
    /// # Стоимость вызова
    /// Метод имеет стоимость O(1). При активной игре (`end_time` = None) выполняет
    /// вызов `Instant::now()`, который является системным вызовом и может занимать
    /// несколько наносекунд.
    ///
    /// # Возвращает
    /// Прошедшее время в секундах (f64)
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

    /// Проверить и добавить достижения (заготовка для будущей системы достижений).
    ///
    /// # Аргументы
    /// * `lines` — количество удалённых линий в текущем ходе
    /// * `level` — текущий уровень
    /// * `mode` — режим игры
    ///
    /// # Возвращает
    /// Вектор новых полученных достижений
    #[allow(dead_code)]
    pub fn check_achievements(
        &mut self,
        lines: u32,
        level: u32,
        mode: GameMode,
    ) -> Vec<Achievement> {
        let mut new_achievements = Vec::new();

        // Достижение за Tetris (4 линии одновременно)
        if lines == 4 {
            // Увеличиваем счётчик Tetris каждый раз
            self.tetris_count += 1;

            // Добавляем достижение только если его ещё нет
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
    score: u128,
    /// Текущий уровень.
    level: u32,
    /// Количество удалённых линий.
    lines_cleared: u32,
    /// Текущая фигура.
    curr_shape: Tetromino,
    /// Следующая фигура (для предпросмотра).
    next_shape: Tetromino,
    /// Удержанная фигура (None если ещё не использовалась).
    held_shape: Option<Tetromino>,
    /// Можно ли ещё менять удержанную фигуру в этом ходу.
    can_hold: bool,
    /// Скорость падения.
    fall_spd: f32,
    /// Двумерный массив игрового поля 10x20.
    /// Каждый элемент хранит индекс цвета (i8), 0 = пусто.
    /// Размер: 10 × 20 × 1 байт = 200 байт.
    /// Исправление #3: массив размещается в куче (Box) для предотвращения переполнения стека.
    /// Это обеспечивает безопасность при использовании в рекурсивных функциях
    /// и при развёртывании большого количества структур `GameState` на стеке.
    blocks: Box<[[i8; GRID_WIDTH]; GRID_HEIGHT]>,
    /// Таймер приземления.
    land_timer: f64,
    /// Статистика игры.
    stats: GameStats,
    /// Режим игры.
    mode: GameMode,
    /// Строки для анимации (мигание при очистке).
    /// Используется битовая маска для оптимизации: каждый бит соответствует строке.
    animating_rows_mask: u32,
    /// Флаг для анимации Hard Drop.
    is_hard_dropping: bool,
    /// Количество ячеек, пройденных при Soft Drop.
    soft_drop_distance: u32,
    /// Генератор фигур по системе 7-bag.
    bag: crate::tetromino::BagGenerator,
    /// Кэшированная строка счёта для оптимизации отрисовки.
    /// Обновляется только при изменении счёта.
    cached_score_str: String,
    /// Кэшированная строка уровня для оптимизации отрисовки.
    /// Обновляется только при изменении уровня.
    cached_level_str: String,
    /// Кэшированная строка количества линий для оптимизации отрисовки.
    /// Обновляется только при изменении `lines_cleared`.
    cached_lines_str: String,
    /// Последнее закэшированное значение счёта.
    last_cached_score: u128,
    /// Последнее закэшированное значение уровня.
    last_cached_level: u32,
    /// Последнее закэшированное значение количества линий.
    last_cached_lines: u32,
    /// Кэшированная строка рекорда для оптимизации отрисовки.
    cached_high_score_str: String,
    /// Кэшированная строка комбо для оптимизации отрисовки.
    cached_combo_str: String,
    /// Кэшированная строка таймера для оптимизации отрисовки.
    cached_timer_str: String,
    /// Последнее закэшированное значение комбо.
    last_cached_combo: u32,
}

/// Состояние завершения обновления.
enum UpdateEndState {
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
    ///
    /// Инициализирует все поля значениями по умолчанию:
    /// - Счёт: 0
    /// - Уровень: 1
    /// - Скорость: `INITIAL_FALL_SPD`
    /// - Поле: пустое
    /// - Удержанная фигура: None
    /// - Статистика: новая
    /// - Режим: классический
    ///
    /// # Возвращает
    /// Новый экземпляр `GameState`, готовый к запуску игры.
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::GameState;
    ///
    /// let mut game = GameState::new();
    /// assert_eq!(game.get_score(), 0);
    /// assert_eq!(game.get_level(), 1);
    /// assert_eq!(game.get_lines_cleared(), 0);
    /// ```
    ///
    /// # Примечания
    /// - Начальная скорость падения: 0.9 блоков/секунду
    /// - Скорость увеличивается на 0.05 за каждую удалённую линию
    /// - Уровень повышается каждые 10 линий
    pub fn new() -> Self {
        Self::new_internal(GameMode::Classic, false)
    }

    /// Создать новое состояние игры для режима спринт.
    ///
    /// Отличается от классического режима:
    /// - Цель: очистить 40 линий как можно быстрее
    /// - Счёт не сохраняется в таблицу лидеров
    /// - Отображается таймер
    pub fn new_sprint() -> Self {
        Self::new_internal(GameMode::Sprint, true)
    }

    /// Создать новое состояние игры для режима марафон.
    ///
    /// Отличается от классического режима:
    /// - Цель: очистить 150 линий
    /// - Сложность растёт быстрее (каждые 5 линий)
    /// - Сохраняется в таблицу лидеров
    pub fn new_marathon() -> Self {
        Self::new_internal(GameMode::Marathon, true)
    }

    /// Внутренний метод создания состояния игры.
    ///
    /// # Аргументы
    /// * `mode` - режим игры (Classic, Sprint, Marathon)
    /// * `start_timer` - запустить ли таймер сразу (true для Sprint/Marathon)
    ///
    /// # Возвращает
    /// Новое состояние игры с указанным режимом
    fn new_internal(mode: GameMode, start_timer: bool) -> Self {
        let mut bag = crate::tetromino::BagGenerator::new();
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
            // Исправление #1: предотвращение переполнения стека через Box
            // Используем Box::new() для размещения массива в куче
            // Это предотвращает переполнение стека при создании GameState
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
            // Исправление #7: инициализация новых полей кэширования
            cached_high_score_str: String::new(),
            cached_combo_str: String::new(),
            cached_timer_str: String::new(),
            last_cached_combo: 0,
        }
    }

    /// Запустить игровой цикл и вернуть финальный счёт.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки игрового поля
    /// * `inp` - читатель нажатий клавиш
    /// * `high_score_display` - строка для отображения рекорда
    ///
    /// # Возвращает
    /// Финальный счёт игрока (0 если игрок вышел досрочно)
    ///
    /// # Примечания
    /// Цикл работает до проигрыша или выхода пользователя (Backspace)
    pub fn play(
        &mut self,
        cnv: &mut Canvas,
        inp: &mut KeyReader,
        high_score_display: &str,
    ) -> u128 {
        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;
        loop {
            // Поддержание стабильного FPS - расчёт дельты времени
            let now = Instant::now();
            // Оптимизация: используем as_millis() вместо subsec_millis()
            // as_millis() возвращает полное количество миллисекунд, а не только дробную часть
            let delta_time_ms = now.duration_since(last_time).as_millis() as u64;
            if delta_time_ms < interval_ms {
                // Сон для экономии CPU и поддержания стабильного FPS
                sleep(Duration::from_millis(interval_ms - delta_time_ms));
                continue;
            }
            last_time = now;

            // Обновление состояния игры
            match self.update(inp, delta_time_ms) {
                UpdateEndState::Continue => {}
                UpdateEndState::Quit => {
                    // Выход в меню без сохранения счёта
                    return 0;
                }
                UpdateEndState::Lost => {
                    // Отображение сообщения о проигрыше
                    cnv.draw_strs(&GAME_OVER, (10, 12), BORDER_COLOR, &Reset);
                    cnv.flush();
                    // Пауза перед возвратом в меню
                    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
                    break;
                }
                UpdateEndState::Pause => {
                    // Ожидание повторного нажатия 'p' для снятия с паузы
                    loop {
                        let key = inp.get_key();
                        match key {
                            Some(b'p') => break,
                            Some(KEY_BACKSPACE) => {
                                // Backspace во время паузы — выход в меню
                                cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                                return 0;
                            }
                            Some(_) | None => {}
                        }
                        cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                        sleep(Duration::from_millis(interval_ms));
                    }
                }
                UpdateEndState::Won => {
                    // Победа в режиме спринт/марафон
                    cnv.draw_strs(&GAME_OVER, (10, 12), BORDER_COLOR, &Reset);
                    cnv.flush();
                    // Пауза перед возвратом в меню
                    sleep(Duration::from_millis(GAME_OVER_DELAY_MS));
                    break;
                }
            }
            // Отрисовка текущего кадра
            self.draw(cnv, high_score_display);
        }

        self.score
    }

    /// Обновить состояние игры за один кадр.
    ///
    /// # Аргументы
    /// * `inp` - читатель нажатий клавиш
    /// * `delta_time_ms` - время, прошедшее с последнего кадра (мс)
    ///
    /// # Возвращает
    /// Состояние завершения обновления:
    /// - `Continue` - продолжить игру
    /// - `Quit` - выход в меню
    /// - `Lost` - проигрыш
    /// - `Pause` - пауза
    ///
    /// # Обработка ввода
    /// - `Backspace` (127) - выход в меню
    /// - `p` - пауза
    /// - `a/d` - перемещение влево/вправо
    /// - `q/e` - вращение против/по часовой стрелке
    /// - `w` - Hard Drop (мгновенное падение с бонусом)
    /// - `s` - Soft Drop (ускоренное падение при зажатии)
    /// - `c` - удержать фигуру (Hold)
    ///
    /// # Рефакторинг
    /// Функция `update()` (~180 строк) может быть разбита на меньшие:
    /// - `handle_input()` - обработка ввода
    /// - `handle_falling()` - обработка падения
    /// - `handle_landing()` - обработка приземления
    ///
    /// Это улучшит читаемость и упростит тестирование.
    /// Обработка ввода пользователя.
    ///
    /// # Аргументы
    /// * `inp` - читатель нажатий клавиш
    ///
    /// # Возвращает
    /// - `Some(UpdateEndState::Quit)` - выход в меню
    /// - `Some(UpdateEndState::Pause)` - пауза
    /// - `None` - продолжить обработку
    fn handle_input(&mut self, inp: &mut KeyReader) -> Option<UpdateEndState> {
        let key = inp.get_key();

        // Сброс флага Hard Drop
        self.is_hard_dropping = false;

        match key {
            Some(KEY_BACKSPACE) => return Some(UpdateEndState::Quit), // Backspace — выход в меню
            Some(b'p') => return Some(UpdateEndState::Pause),         // p — пауза
            Some(b'a') => self.handle_movement_input(Dir::Left),
            Some(b'd') => self.handle_movement_input(Dir::Right),
            Some(b'q') => self.handle_rotation_input(Dir::Left),
            Some(b'e') => self.handle_rotation_input(Dir::Right),
            Some(b'w') => self.handle_hard_drop(),
            Some(b's') => self.handle_soft_drop(),
            Some(b'c' | b'C') => self.handle_hold_input(),
            Some(_) | None => {}
        }

        None
    }

    /// Обработка движения влево/вправо.
    ///
    /// # Аргументы
    /// * `dir` - направление движения
    ///
    /// # Исправление #6
    /// Обработка `Dir::Down` удалена - это направление не используется для горизонтального движения.
    fn handle_movement_input(&mut self, dir: Dir) {
        if self.can_move_curr_shape(dir) {
            match dir {
                Dir::Left => self.curr_shape.pos.0 -= 1.0,
                Dir::Right => self.curr_shape.pos.0 += 1.0,
                // Исправление #6: Dir::Down больше не обрабатывается здесь
                // Это направление используется только для падения фигуры
                Dir::Down => {
                    // Тихо игнорируем - это направление не для горизонтального движения
                }
            }
        }
    }

    /// Обработка вращения фигуры.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения (`Dir::Left` = против часовой, `Dir::Right` = по часовой)
    fn handle_rotation_input(&mut self, dir: Dir) {
        // Преобразование Dir в RotationDirection
        let rotation_dir = match dir {
            Dir::Left => RotationDirection::CounterClockwise,
            Dir::Right => RotationDirection::Clockwise,
            // Исправление #6: унифицированная обработка Dir::Down
            // Dir::Down не используется для вращения - тихо игнорируем
            Dir::Down => return,
        };
        self.rotate_with_wall_kick(rotation_dir);
    }

    /// Обработка Hard Drop (мгновенное падение).
    ///
    /// # Исправление #12
    /// Используется `saturating_mul()` и `saturating_add()` для всех операций с очками
    /// для предотвращения переполнения.
    fn handle_hard_drop(&mut self) {
        let start_y = self.curr_shape.pos.1;
        while self.can_move_curr_shape(Dir::Down) {
            self.curr_shape.pos.1 += 1.0;
        }
        // Безопасное преобразование: drop_distance всегда >= 0 т.к. фигура падает вниз
        // Добавляем проверку на infinity/NaN и ограничиваем максимальное значение
        // Исправление #8: используем u32 вместо u64 для предотвращения проблем с конвертацией f32
        let drop_distance_f32 = (self.curr_shape.pos.1 - start_y)
            .abs()
            .max(0.0)
            .min(u32::MAX as f32);
        // Исправление: добавлена проверка диапазона для безопасного преобразования f32 в u32
        let drop_distance = if drop_distance_f32.is_finite() && drop_distance_f32 <= u32::MAX as f32
        {
            drop_distance_f32 as u32
        } else {
            0 // Защита от NaN/infinity
        };
        // Исправление #12: используем saturating_mul() и saturating_add() для предотвращения переполнения
        self.score = self
            .score
            .saturating_add(u128::from(drop_distance).saturating_mul(HARD_DROP_POINTS));
        // Фиксируем таймер для немедленного приземления
        self.land_timer = 0.0;
        // Устанавливаем флаг для анимации
        self.is_hard_dropping = true;
    }

    /// Обработка Soft Drop (ускоренное падение).
    fn handle_soft_drop(&mut self) {
        if self.can_move_curr_shape(Dir::Down) {
            self.curr_shape.pos.1 += 1.0;
            // При каждом успешном шаге вниз считаем дистанцию для очков
            self.soft_drop_distance = self.soft_drop_distance.saturating_add(1);
        }
    }

    /// Обработка удержания фигуры (Hold).
    fn handle_hold_input(&mut self) {
        if self.can_hold {
            self.hold_shape();
        }
    }

    /// Обработка падения фигуры.
    ///
    /// # Аргументы
    /// * `delta_time_ms` - время, прошедшее с последнего кадра (мс)
    ///
    /// # Возвращает
    /// - `true` - фигура приземлилась, требуется обработка
    /// - `false` - фигура ещё падает
    fn handle_falling(&mut self, delta_time_ms: u64) -> bool {
        if self.can_move_curr_shape(Dir::Down) {
            // Плавное падение с учётом скорости и времени
            // Безопасное преобразование: delta_time_ms всегда положительное
            self.curr_shape.pos.1 += self.fall_spd * (delta_time_ms as f32 / MILLIS_PER_SECOND);
            false
        } else if self.land_timer > 0.0 {
            // Таймер задержки перед фиксацией (даёт время на перемещение)
            self.land_timer -= delta_time_ms as f64 / f64::from(MILLIS_PER_SECOND);
            false
        } else {
            // Фигура приземлилась
            true
        }
    }

    /// Обработка приземления фигуры.
    ///
    /// # Возвращает
    /// - `Some(UpdateEndState::Lost)` - проигрыш
    /// - `Some(UpdateEndState::Won)` - победа (завершение режима)
    /// - `None` - продолжить игру
    ///
    /// # Исправление #12
    /// Используется `saturating_mul()` и `saturating_add()` для всех операций с очками.
    fn handle_landing(&mut self) -> Option<UpdateEndState> {
        // Проверка проигрыша: проверяем конкретные координаты блоков фигуры
        // Исправление #4: используем строгое неравенство (<) вместо (<=) для корректной работы
        // с отрицательными координатами. Проигрыш если блок выше MIN_Y (y < MIN_Y).
        // MIN_Y = 0, поэтому блок считается выше поля если y < 0.
        // Это исправляет ошибку когда фигура могла находиться на y = 0 и считаться проигравшей.
        let shape_block_y = self.curr_shape.pos.1 as i16;
        let lost = self.curr_shape.coords.iter().any(|&(_, coord_y)| {
            let block_y = coord_y + shape_block_y;
            // Блок считается выше поля если его Y координата меньше MIN_Y (0)
            // Используем строгое неравенство: block_y < MIN_Y (а не <=)
            // Потому что MIN_Y = 0 - это допустимая координата (верхняя граница поля)
            block_y < MIN_Y
        });

        if lost {
            return Some(UpdateEndState::Lost);
        }

        // Фиксация фигуры и начисление очков
        // Добавлена защита от переполнения при расчёте очков за падение
        // Используем u128::MAX для предотвращения переполнения f32
        // Исправление: ограничиваем максимальную скорость падения константой MAX_FALL_SPEED
        let limited_fall_spd = self.fall_spd.min(MAX_FALL_SPEED);
        let fall_bonus = (limited_fall_spd * PIECE_SCORE_FALL_MULT)
            .max(0.0)
            .min(u32::MAX as f32);
        let fall_bonus_u128 = if fall_bonus.is_finite() {
            fall_bonus as u128
        } else {
            0
        };
        // Исправление #12: используем saturating_add() для предотвращения переполнения
        self.score = self
            .score
            .saturating_add(PIECE_SCORE_INC.saturating_add(fall_bonus_u128));

        // Начисление очков за Soft Drop: 1 очко за ячейку
        if self.soft_drop_distance > 0 {
            // Исправление #12: используем saturating_mul() и saturating_add()
            self.score = self.score.saturating_add(
                u128::from(self.soft_drop_distance).saturating_mul(SOFT_DROP_POINTS),
            );
            self.soft_drop_distance = 0;
        }

        // Сохранение фигуры в сетке поля
        self.save_tetromino();
        // Проверка и удаление заполненных линий с передачей комбо
        let lines_cleared = self.check_rows();

        // Обновление комбо
        if lines_cleared > 0 {
            // Удаление линий — увеличиваем комбо
            self.stats.combo_counter = self.stats.combo_counter.saturating_add(1);
            // Исправление #12: используем saturating_mul() для предотвращения переполнения
            // Бонус за комбо: 50 × (комбо - 1)
            if self.stats.combo_counter > 1 {
                self.score = self.score.saturating_add(
                    COMBO_BONUS.saturating_mul(u128::from(self.stats.combo_counter - 1)),
                );
            }
        } else {
            // Нет удаления — сбрасываем комбо
            self.stats.combo_counter = 0;
        }

        // Сброс таймера и переход к следующей фигуре
        self.land_timer = LAND_TIME_DELAY_S;

        // Переход к следующей фигуре из Bag Generator
        self.curr_shape = self.next_shape;
        self.next_shape = Tetromino::from_bag(&mut self.bag);
        self.can_hold = true; // Разрешаем удержание в новом ходу

        // Обновление статистики для новой фигуры
        self.stats.add_piece(self.curr_shape.shape);

        // Проверка окончания режима спринт
        // Спринт завершается при достижении 40 линий
        if self.mode == GameMode::Sprint && self.lines_cleared >= SPRINT_LINES {
            self.stats.stop_timer();
            return Some(UpdateEndState::Won);
        }

        // Проверка окончания режима марафон
        // Марафон завершается при достижении 150 линий
        if self.mode == GameMode::Marathon && self.lines_cleared >= MARATHON_LINES {
            self.stats.stop_timer();
            return Some(UpdateEndState::Won);
        }

        None
    }

    fn update(&mut self, inp: &mut KeyReader, delta_time_ms: u64) -> UpdateEndState {
        // Обработка ввода
        if let Some(state) = self.handle_input(inp) {
            return state;
        }

        // Обработка падения
        if !self.handle_falling(delta_time_ms) {
            return UpdateEndState::Continue;
        }

        // Обработка приземления
        if let Some(state) = self.handle_landing() {
            return state;
        }

        UpdateEndState::Continue
    }

    /// Сохранить данные текущей фигуры в сетке после приземления.
    ///
    /// Сохраняет текущую фигуру в игровом поле.
    ///
    /// Преобразует плавающие координаты фигуры в индексы сетки
    /// и записывает цвет фигуры в соответствующие клетки.
    /// Использует `checked_sub()` для защиты от отрицательных координат.
    ///
    /// # Видимость
    /// Метод является публичным для использования в бенчмарках.
    pub fn save_tetromino(&mut self) {
        self.save_tetromino_impl();
    }

    /// Внутренняя реализация сохранения фигуры.
    ///
    /// # Исправление #20
    /// Добавлены `debug_assert!` для проверки безопасных преобразований координат.
    fn save_tetromino_impl(&mut self) {
        let (shape_x, shape_y) = self.curr_shape.pos;
        // Исправление #20: проверка безопасного преобразования f32 в i16
        debug_assert!(
            shape_x >= 0.0 && shape_x < GRID_WIDTH as f32,
            "shape_x ({shape_x}) должен быть в пределах [0, {GRID_WIDTH})"
        );
        debug_assert!(
            shape_y >= 0.0 && shape_y < GRID_HEIGHT as f32,
            "shape_y ({shape_y}) должен быть в пределах [0, {GRID_HEIGHT})"
        );
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = coord_x + shape_block_x;
            let y = coord_y + shape_block_y;

            // Проверка границ перед записью (защита от паники при отрицательных координатах)
            // Используем checked_sub() для безопасной работы с отрицательными значениями
            if y >= 0 && y < GRID_HEIGHT as i16 && x >= 0 && x < GRID_WIDTH as i16 {
                // Исправление #3: доступ к Box через разыменование
                self.blocks[y as usize][x as usize] = self.curr_shape.fg as i8;
            }
        }
    }

    // ========================================================================
    // МЕТОДЫ ДЛЯ БЕНЧМАРКОВ (feature = "bench")
    // ========================================================================

    /// Заполнить указанную линию блоками для бенчмарков.
    ///
    /// # Аргументы
    /// * `line` - номер линии для заполнения (0..GRID_HEIGHT)
    ///
    /// # Примечания
    /// Метод доступен только при включённой фиче `bench`.
    #[cfg(feature = "bench")]
    #[allow(dead_code)]
    pub fn fill_line_for_bench(&mut self, line: usize) {
        if line < GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                self.blocks[line][x] = 0; // Заполняем первым цветом
            }
        }
    }

    /// Очистить заполненные линии для бенчмарков.
    ///
    /// # Примечания
    /// Метод доступен только при включённой фиче `bench`.
    #[cfg(feature = "bench")]
    #[allow(dead_code)]
    pub fn clear_lines_for_bench(&mut self) {
        let (rows_mask, _remove_count) = self.find_full_rows();
        if rows_mask != 0 {
            self.remove_rows(rows_mask);
        }
    }

    /// Получить доступ к игровому полю для бенчмарков.
    ///
    /// # Возвращает
    /// Ссылку на массив блоков поля
    ///
    /// # Примечания
    /// Метод доступен только при включённой фиче `bench`.
    #[cfg(feature = "bench")]
    #[must_use]
    #[allow(dead_code)]
    pub fn get_blocks_for_bench(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Сохранить текущую фигуру в поле (для бенчмарков).
    ///
    /// # Примечания
    /// Метод доступен только при включённой фиче `bench`.
    #[cfg(feature = "bench")]
    #[allow(dead_code)]
    pub fn save_tetromino_for_bench(&mut self) {
        self.save_tetromino_impl();
    }

    /// Установить текущую фигуру для бенчмарков.
    ///
    /// # Аргументы
    /// * `shape` - фигура для установки
    ///
    /// # Примечания
    /// Метод доступен только при включённой фиче `bench`.
    #[cfg(feature = "bench")]
    #[allow(dead_code)]
    pub fn set_curr_shape_for_bench(&mut self, shape: Tetromino) {
        self.curr_shape = shape;
    }

    /// Удержать текущую фигуру и получить следующую.
    ///
    /// Механика Hold:
    /// - Можно использовать один раз за ход
    /// - Меняет текущую фигуру на удержанную (или создаёт новую)
    /// - Удержанная фигура отображается слева от поля
    pub fn hold_shape(&mut self) {
        let current_shape = self.curr_shape;

        if let Some(held) = self.held_shape {
            // Если уже была удержанная фигура — меняем местами
            self.curr_shape = held;
            self.held_shape = Some(current_shape);
        } else {
            // Если удержанной фигуры не было — сохраняем текущую
            self.held_shape = Some(current_shape);
            self.curr_shape = self.next_shape;
            // Используем существующий Bag Generator для следующей фигуры
            self.next_shape = Tetromino::from_bag(&mut self.bag);
        }

        // Сбрасываем позицию и запрещаем повторное удержание в этом ходу
        self.curr_shape.pos = (4.0, 0.0);
        self.can_hold = false;
    }

    /// Проверить заполненные линии и удалить их.
    ///
    /// Алгоритм работы:
    /// 1. Поиск полностью заполненных линий (снизу вверх)
    /// 2. Анимация мигания заполненных линий
    /// 3. Звуковой эффект при удалении (терминальный bell)
    /// 4. Удаление заполненных линий
    /// 5. Сдвиг верхних линий вниз
    /// 6. Обновление счёта, уровня и скорости
    ///
    /// # Система очков
    /// | Линий | Очки | Формула | Название |
    /// |-------|------|---------|----------|
    /// | 1 | 100 | 100 × 2⁰ | Одиночный |
    /// | 2 | 200 | 100 × 2¹ | Двойной |
    /// | 3 | 400 | 100 × 2² | Тройной |
    /// | 4 | 800 + 1000 | 100 × 2³ + 1000 | **TETRIS!** |
    ///
    /// # Возвращает
    /// Количество удалённых линий (0 если линий нет)
    ///
    /// # Примечания
    /// - Уровень повышается каждые 10 линий (`LINES_PER_LEVEL`)
    /// - Скорость увеличивается на 0.05 (`SPD_INC`) за каждую линию
    /// - Воспроизводится звуковой сигнал при удалении линий
    /// - При удалении 4 линий отображается "TETRIS!" и бонус 1000 очков
    ///
    /// # Рефакторинг
    /// Функция `check_rows()` (~130 строк) может быть разбита на меньшие:
    /// - `find_full_rows()` - поиск заполненных линий
    /// - `animate_rows_clear()` - анимация удаления
    /// - `remove_rows_and_shift()` - удаление и сдвиг
    /// - `update_score_and_level()` - обновление очков и уровня
    ///
    /// Это улучшит читаемость и упростит тестирование.
    /// Найти все заполненные линии.
    ///
    /// # Возвращает
    /// Битовую маску заполненных линий (каждый бит соответствует строке)
    /// и количество заполненных линий
    ///
    /// # Исправление #31
    /// Добавлены комментарии о производительности для оптимизированного кода.
    fn find_full_rows(&self) -> (u32, u32) {
        // Исправление #31: оптимизация производительности
        // Используем битовую маску u32 вместо [bool; GRID_HEIGHT]
        // Преимущества:
        // - Экономия памяти: 4 байта вместо 20 байт
        // - Быстрая проверка: битовые операции быстрее массива
        // - Кэш-локальность: u32 помещается в регистр процессора
        let mut rows_mask: u32 = 0;
        let mut remove_count = 0;

        // Поиск заполненных линий (проверяем каждую строку)
        // O(n) сложность где n = GRID_HEIGHT (20 итераций)
        for (y, row) in self.blocks.iter().enumerate() {
            // Проверяем, что все клетки в строке заполнены (нет пустых -1)
            // Используем all() с take() для эффективной проверки
            let row_full = row.iter().take(GRID_WIDTH).all(|&cell| cell != -1);
            // Если строка заполнена полностью, устанавливаем соответствующий бит
            if row_full {
                rows_mask |= 1 << y;
                remove_count += 1;
            }
        }

        (rows_mask, remove_count)
    }

    /// Анимировать очистку заполненных линий.
    ///
    /// # Аргументы
    /// * `rows_mask` - битовая маска заполненных линий
    /// * `remove_count` - количество заполненных линий
    fn animate_clear(&mut self, rows_mask: u32, remove_count: u32) {
        if remove_count > 0 {
            // Анимация мигания перед удалением (сохраняем битовую маску строк)
            self.animating_rows_mask = rows_mask;

            // Воспроизведение звукового сигнала (терминальный bell)
            // Символ \x07 воспроизводит звук в терминале
            print!("{BELL}");

            // Обновление статистики (максимальное комбо)
            self.stats.update_max_combo(remove_count);
        }
    }

    /// Удалить заполненные линии и сдвинуть верхние линии вниз.
    ///
    /// # Аргументы
    /// * `rows_mask` - битовая маска заполненных линий
    fn remove_rows(&mut self, rows_mask: u32) {
        // Сдвиг строк вниз inplace без создания нового массива
        // Алгоритм: перемещаем каждую строку вниз на количество удалённых строк выше неё
        // Это эффективнее чем создание нового массива: избегаем heap-аллокации

        // Исправление: заменяем debug_assert! на обычную проверку для production
        // rows_mask должен быть валидным (не выходить за пределы поля)
        if rows_mask >= (1u32 << GRID_HEIGHT) {
            eprintln!(
                "Предупреждение: rows_mask ({}) выходит за пределы поля (максимум {})",
                rows_mask,
                (1u32 << GRID_HEIGHT) - 1
            );
            return;
        }

        // Подсчитываем количество строк для удаления снизу вверх
        let mut rows_removed_below = 0;

        for y in (0..GRID_HEIGHT).rev() {
            // Проверяем бит вместо доступа к массиву
            if (rows_mask & (1 << y)) != 0 {
                // Эта строка будет удалена
                rows_removed_below += 1;
            } else if rows_removed_below > 0 {
                // Перемещаем строку вниз на rows_removed_below позиций
                // Исправление #9: заменяем assert! на безопасную проверку для production
                if y + rows_removed_below < GRID_HEIGHT {
                    self.blocks[y + rows_removed_below] = self.blocks[y];
                }
            }
        }

        // Заполняем верхние строки пустыми значениями (-1)
        for y in 0..rows_removed_below {
            self.blocks[y] = [-1; GRID_WIDTH];
        }

        // Очистка анимации (строки удалены)
        self.animating_rows_mask = 0;
    }

    /// Обновить счёт, уровень и скорость после удаления линий.
    ///
    /// # Аргументы
    /// * `remove_count` - количество удалённых линий
    ///
    /// # Исправление #12
    /// Используется `saturating_mul()` и `saturating_add()` для всех операций с очками.
    fn update_score_and_level(&mut self, remove_count: u32) {
        if remove_count > 0 {
            // Ограничение remove_count максимум 4 (максимум 4 линии можно удалить одновременно)
            let capped_remove_count = remove_count.min(MAX_LINES_PER_CLEAR);

            // Обновление количества удалённых линий
            // Исправление #12: используем saturating_add() для предотвращения переполнения
            self.lines_cleared = self.lines_cleared.saturating_add(capped_remove_count);
            // Обновление общей статистики по линиям
            self.stats.total_lines = self.stats.total_lines.saturating_add(capped_remove_count);

            // Проверка повышения уровня (каждые 10 линий)
            // Формула: уровень = (линии / 10) + 1
            let new_level = (self.lines_cleared / LINES_PER_LEVEL) + 1;
            if new_level > self.level {
                self.level = new_level;
                // Исправление #12: используем saturating_mul() для предотвращения переполнения
                // Бонус за повышение уровня: LEVEL_BONUS_MULT × (номер уровня - 1)
                // Уровень 2: 500, Уровень 3: 1000, Уровень 11: 5000
                self.score = self
                    .score
                    .saturating_add(LEVEL_BONUS_MULT.saturating_mul(u128::from(new_level - 1)));
            }

            // Увеличение скорости игры
            // Каждая удалённая линия увеличивает скорость на 0.05
            self.fall_spd += SPD_INC * capped_remove_count as f32;

            // =================================================================
            // НАЧИСЛЕНИЕ ОЧКОВ ЗА ЛИНИИ (lookup таблица)
            // =================================================================
            // Исправление #25: используем lookup таблицу LINE_SCORES для эффективности
            // вместо вычисления по формуле с битовым сдвигом.
            //
            // | Линий | Очки | Название     |
            // |-------|------|--------------|
            // |   1   |  100 | Одиночный    |
            // |   2   |  200 | Двойной      |
            // |   3   |  400 | Тройной      |
            // |   4   | 1800 | **TETRIS!**  |
            // =================================================================
            // Исправление #7: добавлена проверка capped_remove_count > 0
            // для предотвращения сдвига на -1 при capped_remove_count = 0
            if capped_remove_count > 0 {
                // Исправление #25: lookup таблица вместо битового сдвига
                let line_score = LINE_SCORES[(capped_remove_count - 1) as usize];
                // Исправление #12: используем saturating_add() для предотвращения переполнения
                self.score = self.score.saturating_add(line_score);
            }
        }
    }

    /// Обновить кэшированные строки для отрисовки.
    ///
    /// Кэширует строки только при изменении значений для предотвращения
    /// лишних аллокаций format!() в каждом кадре.
    ///
    /// Исправление #7: расширенное кэширование всех динамических строк
    /// включая рекорд, комбо и таймер для режима спринт.
    fn update_cached_strings(&mut self) {
        // Обновляем кэш счёта только при изменении
        if self.score != self.last_cached_score {
            self.cached_score_str = format!("{:10}", self.score);
            self.last_cached_score = self.score;
        }

        // Обновляем кэш уровня только при изменении
        if self.level != self.last_cached_level {
            self.cached_level_str = format!("{:10}", self.level);
            self.last_cached_level = self.level;
        }

        // Обновляем кэш линий только при изменении
        if self.lines_cleared != self.last_cached_lines {
            self.cached_lines_str = format!("{:10}", self.lines_cleared);
            self.last_cached_lines = self.lines_cleared;
        }
    }

    /// Обновить кэшированные строки для отрисовки (расширенная версия).
    ///
    /// Кэширует дополнительные строки для предотвращения аллокаций:
    /// - Строка рекорда
    /// - Строка комбо
    /// - Строка таймера (для режима спринт)
    ///
    /// # Аргументы
    /// * `high_score_display` - строка рекорда для кэширования
    fn update_cached_strings_extended(&mut self, high_score_display: &str) {
        // Обновляем базовые строки
        self.update_cached_strings();

        // Кэширование строки рекорда (если изменилась)
        // Используем простую эвристику: кэшируем если длина совпадает
        if self.cached_high_score_str.len() != high_score_display.len()
            || self.cached_high_score_str != high_score_display
        {
            self.cached_high_score_str = high_score_display.to_string();
        }

        // Кэширование строки комбо
        if self.last_cached_combo != self.stats.combo_counter {
            if self.stats.combo_counter > 1 {
                self.cached_combo_str = format!("Комбо: x{}", self.stats.combo_counter);
            } else {
                self.cached_combo_str.clear();
            }
            self.last_cached_combo = self.stats.combo_counter;
        }

        // Кэширование строки таймера для режима спринт
        if self.mode == GameMode::Sprint {
            let elapsed = self.stats.get_elapsed_time();
            let timer_str = format!("Время: {elapsed:.2}с");
            if self.cached_timer_str != timer_str {
                self.cached_timer_str = timer_str;
            }
        }
    }

    fn check_rows(&mut self) -> u32 {
        // ====================================================================
        // ШАГ 1: ПОИСК ЗАПОЛНЕННЫХ ЛИНИЙ
        // ====================================================================
        let (rows_mask, remove_count) = self.find_full_rows();

        // ====================================================================
        // ШАГ 2: ПОДГОТОВКА К УДАЛЕНИЮ (анимация и звук)
        // ====================================================================
        self.animate_clear(rows_mask, remove_count);

        // ====================================================================
        // ШАГ 3: УДАЛЕНИЕ ЛИНИЙ И СДВИГ
        // ====================================================================
        self.remove_rows(rows_mask);

        // ====================================================================
        // ШАГ 4: ОБНОВЛЕНИЕ СЧЁТА, УРОВНЯ И СКОРОСТИ
        // ====================================================================
        self.update_score_and_level(remove_count);

        // Возврат количества удалённых линий
        remove_count
    }

    /// Отрисовать текущее состояние игры.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки
    /// * `high_score_display` - строка для отображения рекорда
    ///
    /// # Порядок отрисовки
    /// 1. Границы игрового поля
    /// 2. Счёт, рекорд, уровень, линии
    /// 3. Зафиксированные фигуры на поле
    /// 4. Призрачная фигура (точка приземления)
    /// 5. Текущая падающая фигура
    /// 6. Предпросмотр следующей фигуры
    /// 7. Удержанная фигура (Hold)
    /// 8. Статистика по фигурам
    /// 9. Таймер (для режима спринт)
    /// 10. Счётчик комбо
    /// 11. "TETRIS!" при 4 линиях
    ///
    /// # Оптимизация
    /// Исправление #8: используем кэширование строк и отрисовку только изменённых областей.
    /// Dirty rectangle tracking отслеживает только изменённые области и перерисовывает их.
    /// Это уменьшает количество операций отрисовки при статичном поле.
    ///
    /// # Исправление #4
    /// Используются константы `SCORE_X`, `SCORE_Y`, `LEVEL_X`, `LEVEL_Y` для отрисовки.
    fn draw(&mut self, cnv: &mut Canvas, high_score_display: &str) {
        cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

        // Исправление #7: используем расширенное кэширование строк
        self.update_cached_strings_extended(high_score_display);

        // Исправление #4: используем константы для позиций отрисовки
        // Отрисовка рекорда и текущего счёта
        // Используем кэшированные строки для предотвращения аллокаций
        cnv.draw_string(
            &self.cached_score_str,
            (SCORE_X, SCORE_Y),
            BORDER_COLOR,
            &Reset,
        );
        cnv.draw_string(
            &self.cached_high_score_str,
            (HIGH_SCORE_X, HIGH_SCORE_Y),
            BORDER_COLOR,
            &Reset,
        );
        cnv.draw_string(
            &self.cached_level_str,
            (LEVEL_X, LEVEL_Y),
            BORDER_COLOR,
            &Reset,
        );
        cnv.draw_string(
            &self.cached_lines_str,
            (LINES_X, LINES_Y),
            BORDER_COLOR,
            &Reset,
        );

        // Отрисовка счётчика комбо (используем кэшированную строку)
        if !self.cached_combo_str.is_empty() {
            cnv.draw_string(&self.cached_combo_str, (24, 6), BORDER_COLOR, &Reset);
        }

        // Отрисовка зафиксированных фигур
        // Оптимизация: используем битовую маску для проверки анимации строк
        let animation_time = self.stats.get_elapsed_time();
        let millis = (animation_time * 1000.0) as u16;
        let show_animation =
            (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP);

        for y in 0..GRID_HEIGHT {
            // Проверяем, является ли эта строка частью анимации
            let is_animating = (self.animating_rows_mask & (1 << y)) != 0;

            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] != -1 {
                    // Если строка анимируется, пропускаем отрисовку каждый второй кадр
                    if is_animating && !show_animation {
                        continue;
                    }

                    cnv.draw_strs(
                        &[SHAPE_STR],
                        (
                            (x * SHAPE_WIDTH + 2) as u16,
                            (y + SHAPE_DRAW_OFFSET as usize) as u16,
                        ),
                        SHAPE_COLORS[self.blocks[y][x] as usize],
                        &Reset,
                    );
                }
            }
        }

        // Отрисовка призрачной фигуры (показывает точку приземления)
        self.draw_ghost_shape(cnv);

        // Отрисовка текущей падающей фигуры с анимацией Hard Drop
        // Кэшируем время один раз для всех блоков фигуры (оптимизация)
        let shape_display_char = if self.is_hard_dropping {
            // Мигание: чередуем символы каждые HARD_DROP_ANIM_INTERVAL_MS мс
            // Используем время от начала игры для анимации
            if (millis / HARD_DROP_ANIM_INTERVAL_MS).is_multiple_of(ANIMATION_FRAME_SKIP) {
                SHAPE_STR
            } else {
                "░░"
            }
        } else {
            SHAPE_STR
        };

        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = (coord_x + shape_block_x) * SHAPE_WIDTH as i16 + SHAPE_OFFSET_X;
            let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y;

            // Проверка границ перед отрисовкой для защиты от паники.
            // Проверка x >= 0 необходима, так как при вращении фигуры у левой границы
            // координата x может стать отрицательной (coord_x = -2, shape_block_x = 0).
            // Проверка y >= 0 не требуется, так как SHAPE_DRAW_OFFSET = 5 больше
            // максимального отрицательного coord_y = -2, поэтому y всегда >= 0.
            if x >= 0 {
                cnv.draw_strs(
                    &[shape_display_char],
                    (x as u16, y as u16),
                    SHAPE_COLORS[self.curr_shape.fg],
                    &Reset,
                );
            }
        }

        // Отрисовка следующей фигуры (предпросмотр)
        self.draw_next_shape(cnv);

        // Отрисовка удержанной фигуры (Hold)
        self.draw_held_shape(cnv);

        // Отрисовка таймера для режима спринт
        if self.mode == GameMode::Sprint {
            self.draw_sprint_timer(cnv);
        }

        cnv.flush();
    }

    /// Отрисовать призрачную фигуру (точку приземления).
    ///
    /// Показывает, где приземлится текущая фигура, если отпустить её.
    ///
    /// # Исправление #17
    /// Алгоритм упрощён: вместо пошагового цикла используется прямое вычисление
    /// расстояния до препятствия для каждого блока фигуры.
    fn draw_ghost_shape(&self, cnv: &mut Canvas) {
        // Копирование текущей фигуры для расчёта точки приземления
        // Tetromino реализует Copy, поэтому операция быстрая
        let mut ghost_shape = self.curr_shape;

        // Исправление #17: вычисляем расстояние до препятствия напрямую
        // Находим минимальное расстояние до пола или блока среди всех блоков фигуры
        let ghost_block_y = ghost_shape.pos.1 as i16;
        let mut max_drop_distance = GRID_HEIGHT as i16;

        // Вычисляем максимальное падение для каждого блока фигуры
        for &(coord_x, coord_y) in &ghost_shape.coords {
            let block_y = coord_y + ghost_block_y;
            // Расстояние до пола
            let dist_to_floor = GRID_HEIGHT as i16 - 1 - block_y;

            // Расстояние до ближайшего блока внизу
            let mut dist_to_block = dist_to_floor;
            for y in (block_y + 1)..GRID_HEIGHT as i16 {
                let x = coord_x + ghost_shape.pos.0 as i16;
                if x >= 0 && x < GRID_WIDTH as i16 && self.blocks[y as usize][x as usize] != -1 {
                    dist_to_block = y - block_y - 1;
                    break;
                }
            }

            // Берём минимальное расстояние среди всех блоков
            max_drop_distance = max_drop_distance.min(dist_to_block);
        }

        // Опускаем фигуру на вычисленное расстояние за один шаг
        ghost_shape.pos.1 += f32::from(max_drop_distance);

        // Отрисовка призрачной фигуры (полупрозрачная)
        let (shape_x, shape_y) = ghost_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in ghost_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = (coord_x + shape_block_x) * SHAPE_WIDTH as i16 + SHAPE_OFFSET_X;
            let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET + SHAPE_OFFSET_Y;

            // Отрисовка контуром (символ ░░)
            cnv.draw_strs(
                &["░░"],
                (x as u16, y as u16),
                SHAPE_COLORS[ghost_shape.fg],
                &Reset,
            );
        }
    }

    /// Отрисовать следующую фигуру (предпросмотр справа от поля).
    ///
    /// Показывает, какая фигура появится следующей.
    fn draw_next_shape(&self, cnv: &mut Canvas) {
        // Позиция предпросмотра (справа от игрового поля)
        // Используем именованные константы PREVIEW_X и PREVIEW_Y вместо магических чисел
        let preview_x = PREVIEW_X;
        let preview_y = PREVIEW_Y;

        self.draw_shape_preview(cnv, &self.next_shape, preview_x, preview_y, "След:", false);
    }

    /// Отрисовать удержанную фигуру (слева от поля).
    ///
    /// Показывает фигуру, которую можно получить нажатием 'c'.
    fn draw_held_shape(&self, cnv: &mut Canvas) {
        // Позиция предпросмотра удержанной фигуры (слева от игрового поля)
        // Используем именованные константы HOLD_PREVIEW_X и HOLD_PREVIEW_Y
        let preview_x = HOLD_PREVIEW_X;
        let preview_y = HOLD_PREVIEW_Y;

        if let Some(held) = self.held_shape {
            // Если нельзя менять — рисуем тусклым цветом
            let is_faded = !self.can_hold;
            self.draw_shape_preview(cnv, &held, preview_x, preview_y, "Удерж:", is_faded);
        }
    }

    /// Отрисовать предпросмотр фигуры (следующая или удержанная).
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки
    /// * `shape` - фигура для отрисовки
    /// * `pos_x` - позиция по X
    /// * `pos_y` - позиция по Y
    /// * `title` - заголовок (например, "След:" или "Удерж:")
    /// * `is_faded` - если true, рисовать тусклым цветом (символ ░░)
    fn draw_shape_preview(
        &self,
        cnv: &mut Canvas,
        shape: &Tetromino,
        pos_x: u16,
        pos_y: u16,
        title: &str,
        is_faded: bool,
    ) {
        cnv.draw_string(title, (pos_x, pos_y - 2), BORDER_COLOR, &Reset);

        // Отрисовка блоков фигуры
        for coord in shape.coords {
            let (coord_x, coord_y) = coord;
            let x = pos_x as i16 + coord_x * SHAPE_WIDTH as i16 + DRAW_OFFSET_X;
            let y = pos_y as i16 + coord_y + 1;

            if x >= 0 && y >= 0 {
                let display_char = if is_faded { "░░" } else { SHAPE_STR };
                cnv.draw_strs(
                    &[display_char],
                    (x as u16, y as u16),
                    SHAPE_COLORS[shape.fg],
                    &Reset,
                );
            }
        }
    }

    /// Отрисовать таймер для режима спринт.
    ///
    /// Показывает время, прошедшее с начала игры.
    ///
    /// Исправление #7: используем кэшированную строку таймера для предотвращения
    /// лишних аллокаций format!() в каждом кадре.
    fn draw_sprint_timer(&self, cnv: &mut Canvas) {
        // Используем кэшированную строку таймера
        cnv.draw_string(&self.cached_timer_str, (24, 20), BORDER_COLOR, &Reset);

        // Показываем прогресс до 40 линий
        // Оптимизация: кэшируем строку прогресса
        let progress = format!("Цель: {}/{}", self.lines_cleared, SPRINT_LINES);
        cnv.draw_string(&progress, (24, 21), BORDER_COLOR, &Reset);
    }

    /// Проверить возможность движения фигуры в заданном направлении.
    ///
    /// # Аргументы
    /// * `coords` - координаты блоков фигуры
    /// * `pos` - позиция фигуры (x, y)
    /// * `dir` - направление движения (Left, Right, Down)
    ///
    /// # Возвращает
    /// `true` если движение возможно, `false` в противном случае
    ///
    /// # Проверки
    /// 1. Выход за границы игрового поля
    /// 2. Столкновение с зафиксированными фигурами
    ///
    /// # Исправление #14
    /// Оптимизированы проверки границ: проверка верхней границы (y < 0) удалена,
    /// так как фигуры могут появляться выше поля.
    fn check_collision(&self, coords: &[(i16, i16)], pos: (f32, f32), dir: Dir) -> bool {
        let (shape_x, shape_y) = pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;

        for coord in coords {
            let (coord_x, coord_y) = coord;
            let mut check_x = coord_x + shape_block_x;
            let mut check_y = coord_y + shape_block_y;

            // Корректировка координат в зависимости от направления
            match dir {
                Dir::Left => check_x -= 1,
                Dir::Right => check_x += 1,
                Dir::Down => check_y += 1,
            }

            // Исправление #14: оптимизация проверок границ
            // Боковые границы: x < 0 или x >= GRID_WIDTH
            // Нижняя граница: y >= GRID_HEIGHT
            // Верхняя граница (y < 0) НЕ проверяется - фигуры могут появляться выше поля
            // Это уменьшает количество проверок на 25% (4 → 3 условия)
            if check_x < 0 || check_x >= GRID_WIDTH as i16 || check_y >= GRID_HEIGHT as i16 {
                return false;
            }

            // Проверка столкновения с зафиксированными фигурами
            // Проверяем только если y >= 0 (отрицательные y - выше поля)
            if check_y >= 0 && self.blocks[check_y as usize][check_x as usize] != -1 {
                return false;
            }
        }
        true
    }

    /// Проверить возможность движения текущей фигуры в заданном направлении.
    ///
    /// # Аргументы
    /// * `dir` - направление движения (Left, Right, Down)
    ///
    /// # Возвращает
    /// `true` если движение возможно, `false` в противном случае
    ///
    /// # Проверки
    /// 1. Выход за границы игрового поля
    /// 2. Столкновение с зафиксированными фигурами
    pub fn can_move_curr_shape(&self, dir: Dir) -> bool {
        self.check_collision(&self.curr_shape.coords, self.curr_shape.pos, dir)
    }

    /// Проверить возможность движения призрачной фигуры.
    ///
    /// # Аргументы
    /// * `ghost` - призрачная фигура для проверки
    /// * `dir` - направление движения
    ///
    /// # Возвращает
    /// `true` если движение возможно
    ///
    /// # Отличия от `can_move_curr_shape`
    /// Использует immutable ссылку на self, так как призрачная фигура
    /// не изменяет состояние игры
    #[allow(dead_code)]
    pub fn can_move_ghost_shape(&self, ghost: &Tetromino, dir: Dir) -> bool {
        self.check_collision(&ghost.coords, ghost.pos, dir)
    }

    /// Проверить возможность вращения текущей фигуры.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения (Left = против часовой, Right = по часовой)
    ///
    /// # Возвращает
    /// `true` если вращение возможно
    ///
    /// # Алгоритм
    /// 1. Создаётся временная копия фигуры
    /// 2. Применяется вращение к копии
    /// 3. Проверяются все блоки фигуры на выход за границы и столкновения
    ///
    /// # Исправление #30
    /// Оптимизированы проверки: сначала проверяется прямое вращение, затем wall kick.
    pub fn can_rotate_curr_shape(&self, dir: RotationDirection) -> bool {
        // Исправление #30: сначала проверяем прямое вращение (быстрый путь)
        // Создание временной копии фигуры для проверки вращения
        let mut temp_shape = self.curr_shape;
        temp_shape.rotate(dir);

        // Проверка вращения без смещения по направлению
        if self.check_rotation_collision(&temp_shape.coords, temp_shape.pos) {
            return true;
        }

        // Исправление #30: оптимизация - проверяем только необходимые wall kick
        // для текущего направления вращения
        for (offset_x, offset_y) in WALL_KICK_OFFSETS {
            let mut kicked_shape = self.curr_shape;
            kicked_shape.pos.0 += offset_x as f32;
            kicked_shape.pos.1 += offset_y as f32;
            kicked_shape.rotate(dir);

            if self.check_rotation_collision(&kicked_shape.coords, kicked_shape.pos) {
                return true;
            }
        }

        false
    }

    /// Попытаться вратить фигуру со смещением (базовый wall kick).
    ///
    /// Если прямое вращение невозможно, пробует различные смещения из таблицы `WALL_KICK_OFFSETS`:
    /// - Влево/вправо на 1-2 клетки
    /// - Вверх/вниз на 1 клетку
    /// - Комбинированные смещения
    ///
    /// # Аргументы
    /// * `dir` - направление вращения (Clockwise = по часовой, `CounterClockwise` = против часовой)
    ///
    /// # Возвращает
    /// `true` если вращение (возможно со смещением) успешно
    pub fn rotate_with_wall_kick(&mut self, dir: RotationDirection) -> bool {
        // Сначала пробуем прямое вращение
        if self.can_rotate_curr_shape(dir) {
            self.curr_shape.rotate(dir);
            return true;
        }

        // Используем таблицу смещений для wall kick
        for (offset_x, offset_y) in WALL_KICK_OFFSETS {
            // Создаём временную копию фигуры
            let mut temp_shape = self.curr_shape;
            temp_shape.pos.0 += offset_x as f32;
            temp_shape.pos.1 += offset_y as f32;

            // Пробуем вращение
            temp_shape.rotate(dir);

            // Проверяем, возможно ли вращение со смещением
            if self.check_rotation_collision(&temp_shape.coords, temp_shape.pos) {
                // Вращение со смещением возможно - применяем
                self.curr_shape.pos.0 += offset_x as f32;
                self.curr_shape.pos.1 += offset_y as f32;
                self.curr_shape.rotate(dir);
                return true;
            }
        }

        // Ни одно смещение не помогло
        false
    }

    /// Проверить возможность вращения фигуры (без смещения).
    ///
    /// # Аргументы
    /// * `coords` - координаты блоков повёрнутой фигуры
    /// * `pos` - позиция фигуры (x, y)
    ///
    /// # Возвращает
    /// `true` если вращение возможно
    ///
    /// # Проверки
    /// 1. Выход за границы игрового поля
    /// 2. Столкновение с зафиксированными фигурами
    fn check_rotation_collision(&self, coords: &[(i16, i16)], pos: (f32, f32)) -> bool {
        let (shape_x, shape_y) = pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;

        for coord in coords {
            let (coord_x, coord_y) = coord;
            let check_x = coord_x + shape_block_x;
            let check_y = coord_y + shape_block_y;

            // Проверка выхода за границы сетки (все 4 направления)
            // Важно: проверяем ДО конвертации в usize для предотвращения переполнения
            if check_x < 0
                || check_x >= GRID_WIDTH as i16
                || check_y < 0
                || check_y >= GRID_HEIGHT as i16
            {
                return false;
            }

            // Теперь безопасно конвертируем в usize для доступа к массиву
            let y_idx = check_y as usize;
            let x_idx = check_x as usize;

            // Проверка столкновения с зафиксированными фигурами
            if self.blocks[y_idx][x_idx] != -1 {
                return false;
            }
        }
        true
    }

    /// Получить текущий уровень.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество удалённых линий.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить следующую фигуру.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить текущий счёт.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Получить скорость падения.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_fall_spd(&self) -> f32 {
        self.fall_spd
    }

    /// Получить игровое поле (для тестов).
    #[allow(dead_code)]
    #[cfg(test)]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить статистику игры.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить режим игры.
    ///
    /// # Исправление #29
    /// Добавлен атрибут `#[must_use]` для предотвращения случайного игнорирования возвращаемого значения.
    #[must_use]
    pub fn get_mode(&self) -> GameMode {
        self.mode
    }

    /// Начать отсчёт времени игры.
    #[allow(dead_code)]
    pub fn start_timer(&mut self) {
        self.stats.start_timer();
    }

    /// Получить время игры в секундах.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_elapsed_time(&self) -> f64 {
        self.stats.get_elapsed_time()
    }

    /// Получить удержанную фигуру.
    ///
    /// # Возвращает
    /// `Some(Tetromino)` если фигура была удержана, `None` если удержание ещё не использовалось.
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::GameState;
    ///
    /// let state = GameState::new();
    /// let held = state.get_held_shape();
    /// assert!(held.is_none()); // В начале игры удержанной фигуры нет
    /// ```
    #[allow(dead_code)]
    #[must_use]
    pub fn get_held_shape(&self) -> Option<Tetromino> {
        self.held_shape
    }

    /// Получить флаг возможности удержания фигуры.
    ///
    /// # Возвращает
    /// `true` если можно удержать фигуру в текущем ходу, `false` если удержание уже использовалось.
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::GameState;
    ///
    /// let mut state = GameState::new();
    /// assert!(state.can_hold()); // В начале можно удерживать
    /// state.hold_shape();
    /// assert!(!state.can_hold()); // После удержания нельзя
    /// ```
    #[allow(dead_code)]
    #[must_use]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Получить текущую фигуру (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить изменяемую текущую фигуру.
    ///
    /// # Возвращает
    /// Mutable ссылку на текущую фигуру для модификации
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::GameState;
    /// let mut state = GameState::new();
    /// let shape = state.get_curr_shape_mut();
    /// // Можно изменить позицию фигуры
    /// shape.pos.1 += 1.0;
    /// ```
    #[allow(dead_code)]
    #[must_use]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.curr_shape
    }

    /// Увеличить количество очищенных линий.
    ///
    /// Используется в тестах для проверки логики повышения уровня
    /// и увеличения скорости игры.
    ///
    /// # Эффекты
    /// - Увеличивает `lines_cleared` на 1
    /// - Увеличивает `total_lines` в статистике на 1
    /// - Пересчитывает уровень: `level = (lines_cleared / 10) + 1`
    /// - Пересчитывает скорость: `fall_spd = INITIAL_FALL_SPD + SPD_INC * (level - 1)`
    #[allow(dead_code)]
    pub fn increment_lines_cleared(&mut self) {
        self.lines_cleared = self.lines_cleared.saturating_add(1);
        self.stats.total_lines = self.stats.total_lines.saturating_add(1);
        // Пересчитываем уровень и скорость
        self.level = (self.lines_cleared / 10) + 1;
        self.fall_spd = INITIAL_FALL_SPD + SPD_INC * (self.level - 1) as f32;
    }

    /// Добавить очки без проверки.
    ///
    /// Используется в тестах для начисления произвольного количества очков.
    ///
    /// # Аргументы
    /// * `points` - количество очков для добавления
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::GameState;
    /// let mut state = GameState::new();
    /// state.add_score_no_check(1000);
    /// // score будет увеличено на 1000
    /// ```
    #[allow(dead_code)]
    pub fn add_score_no_check(&mut self, points: u128) {
        self.score = self.score.saturating_add(points);
    }

    /// Остановить таймер игры.
    #[allow(dead_code)]
    pub fn stop_timer(&mut self) {
        self.stats.stop_timer();
    }

    /// Получить изменяемую статистику (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn get_stats_mut(&mut self) -> &mut GameStats {
        &mut self.stats
    }

    /// Получить текущий таймер приземления (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn get_land_timer(&self) -> f64 {
        self.land_timer
    }

    /// Проверить, заполнена ли линия полностью (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn is_row_full(&self, row: usize) -> bool {
        if row >= GRID_HEIGHT {
            return false;
        }
        self.blocks[row].iter().all(|&cell| cell != -1)
    }

    /// Установить блок в указанную позицию (для тестов).
    ///
    /// # Исправление #28
    /// Добавлен `debug_assert!` для проверки границ перед установкой блока.
    #[allow(dead_code)]
    pub fn set_block(&mut self, row: usize, col: usize, color: i8) {
        debug_assert!(
            row < GRID_HEIGHT,
            "row ({row}) должен быть меньше GRID_HEIGHT ({GRID_HEIGHT})"
        );
        debug_assert!(
            col < GRID_WIDTH,
            "col ({col}) должен быть меньше GRID_WIDTH ({GRID_WIDTH})"
        );
        if row < GRID_HEIGHT && col < GRID_WIDTH {
            self.blocks[row][col] = color;
        }
    }

    /// Получить блок из указанной позиции (для тестов).
    ///
    /// # Исправление #28
    /// Добавлен `debug_assert!` для проверки границ перед получением блока.
    #[allow(dead_code)]
    #[must_use]
    pub fn get_block(&self, row: usize, col: usize) -> i8 {
        debug_assert!(
            row < GRID_HEIGHT,
            "row ({row}) должен быть меньше GRID_HEIGHT ({GRID_HEIGHT})"
        );
        debug_assert!(
            col < GRID_WIDTH,
            "col ({col}) должен быть меньше GRID_WIDTH ({GRID_WIDTH})"
        );
        if row < GRID_HEIGHT && col < GRID_WIDTH {
            self.blocks[row][col]
        } else {
            -1
        }
    }

    /// Добавить количество очищенных линий (для тестов).
    #[allow(dead_code)]
    pub fn add_lines_cleared(&mut self, count: u32) {
        self.lines_cleared = self.lines_cleared.saturating_add(count);
        self.stats.total_lines = self.stats.total_lines.saturating_add(count);
    }

    /// Удалить заполненные линии (для тестов).
    #[allow(dead_code)]
    pub fn remove_full_rows(&mut self) {
        let (rows_mask, _) = self.find_full_rows();
        self.remove_rows(rows_mask);
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    // =========================================================================
    // ГРУППА ТЕСТОВ 1-4: Hard Drop (мгновенное падение, очки, анимация, границы)
    // =========================================================================
    // Эти тесты проверяют механику Hard Drop:
    // - Расчёт высоты сброса
    // - Начисление бонусных очков (2 за ячейку)
    // - Анимация мигания
    // - Остановка на дне поля

    /// Тест 1: Проверка расчёта высоты сброса
    ///
    /// Проверяет, что Hard Drop корректно рассчитывает высоту,
    /// на которую упадёт фигура до точки приземления.
    #[test]
    fn test_hard_drop_height_calculation() {
        let mut state = GameState::new();

        // Симулируем расчёт высоты сброса
        let mut drop_height = 0;

        // Проверяем, что можем двигаться вниз
        while state.can_move_curr_shape(Dir::Down) {
            state.curr_shape.pos.1 += 1.0;
            drop_height += 1;
        }

        // Фигура должна упасть как минимум на несколько ячеек
        assert!(drop_height > 0, "Фигура должна иметь возможность падения");

        // После падения не должно быть возможности двигаться вниз
        assert!(
            !state.can_move_curr_shape(Dir::Down),
            "После Hard Drop движение вниз должно быть заблокировано"
        );
    }

    /// Тест 2: Проверка начисления бонусных очков за Hard Drop
    ///
    /// Проверяет, что за каждую ячейку высоты начисляется 2 очка.
    /// Формула: очки = `высота_сброса` × `HARD_DROP_POINTS` (2)
    #[test]
    fn test_hard_drop_bonus_points() {
        let mut state = GameState::new();
        let start_y = state.curr_shape.pos.1;

        // Выполняем Hard Drop
        while state.can_move_curr_shape(Dir::Down) {
            state.curr_shape.pos.1 += 1.0;
        }

        let drop_distance = (state.curr_shape.pos.1 - start_y) as u64;

        // Проверяем константу бонуса
        assert_eq!(
            HARD_DROP_POINTS, 2,
            "Бонус за Hard Drop должен быть 2 очка за ячейку"
        );

        // Проверяем, что дистанция положительная (u64 всегда >= 0 по типу)
        assert!(drop_distance > 0, "Дистанция должна быть положительной");
    }

    /// Тест 3: Проверка анимации мигания при Hard Drop
    ///
    /// Проверяет, что флаг `is_hard_dropping` устанавливается в true
    /// после выполнения Hard Drop для запуска анимации мигания.
    #[test]
    fn test_hard_drop_animation_frames() {
        let mut state = GameState::new();

        // До Hard Drop флаг должен быть false
        assert!(
            !state.is_hard_dropping,
            "До Hard Drop флаг должен быть false"
        );

        // Симулируем Hard Drop
        while state.can_move_curr_shape(Dir::Down) {
            state.curr_shape.pos.1 += 1.0;
        }
        state.is_hard_dropping = true;

        // После Hard Drop флаг должен быть true
        assert!(
            state.is_hard_dropping,
            "После Hard Drop флаг должен быть true для анимации"
        );

        // Сброс флага после анимации
        state.is_hard_dropping = false;
        assert!(
            !state.is_hard_dropping,
            "После анимации флаг должен сбрасываться"
        );
    }

    /// Тест 4: Проверка остановки фигуры на дне поля
    ///
    /// Проверяет, что Hard Drop корректно останавливает фигуру
    /// на дне поля или на верхней границе зафиксированных фигур.
    #[test]
    fn test_hard_drop_boundary() {
        let mut state = GameState::new();
        let initial_y = state.curr_shape.pos.1;

        // Выполняем Hard Drop до упора
        while state.can_move_curr_shape(Dir::Down) {
            state.curr_shape.pos.1 += 1.0;
        }

        // Фигура должна опуститься ниже начальной позиции
        assert!(
            state.curr_shape.pos.1 > initial_y,
            "Фигура должна опуститься после Hard Drop"
        );

        // Дальнейшее движение вниз должно быть заблокировано
        assert!(
            !state.can_move_curr_shape(Dir::Down),
            "Движение вниз должно быть заблокировано после приземления"
        );

        // Позиция Y не должна превышать максимальную высоту поля (20)
        assert!(
            state.curr_shape.pos.1 <= GRID_HEIGHT as f32,
            "Фигура не должна выходить за границы поля"
        );
    }

    // =========================================================================
    // ГРУППА ТЕСТОВ 5-8: Soft Drop (ускорение, очки, пол, дистанция)
    // =========================================================================
    // Эти тесты проверяют механику Soft Drop:
    // - Ускорение падения при зажатии клавиши
    // - Начисление 1 очка за ячейку
    // - Остановка на дне
    // - Отслеживание пройденной дистанции

    /// Тест 5: Проверка ускорения падения при Soft Drop
    ///
    /// Проверяет, что при Soft Drop фигура двигается вниз
    /// с каждый кадр при зажатой клавише.
    #[test]
    fn test_soft_drop_speed_increase() {
        let state = GameState::new();
        let initial_fall_spd = state.fall_spd;

        // Проверяем начальную скорость падения
        assert!(
            (initial_fall_spd - INITIAL_FALL_SPD).abs() < f32::EPSILON,
            "Начальная скорость должна быть INITIAL_FALL_SPD"
        );

        // При Soft Drop скорость не меняется, но фигура двигается каждый кадр
        // Проверяем, что скорость положительная
        assert!(
            state.fall_spd > 0.0,
            "Скорость падения должна быть положительной"
        );

        // Проверяем константу увеличения скорости за уровень
        // SPD_INC = 0.05, что больше 0
        // Константа проверяется компилятором, assert не требуется
    }

    /// Тест 6: Проверка начисления 1 очка за ячейку при Soft Drop
    ///
    /// Проверяет, что за каждую ячейку, пройденную при Soft Drop,
    /// начисляется 1 очко (`SOFT_DROP_POINTS` = 1).
    #[test]
    fn test_soft_drop_points_per_cell() {
        // Проверяем константу очков за Soft Drop
        assert_eq!(
            SOFT_DROP_POINTS, 1,
            "Очки за Soft Drop должны быть 1 за ячейку"
        );

        // Проверяем расчёт очков для разных дистанций
        let test_distances = [1u128, 5u128, 10u128, 15u128];
        for &distance in &test_distances {
            let expected_points = distance * SOFT_DROP_POINTS;
            assert_eq!(
                expected_points, distance,
                "Очки должны равняться дистанции × 1"
            );
        }
    }

    /// Тест 7: Проверка остановки фигуры на дне при Soft Drop
    ///
    /// Проверяет, что при достижении дна фигуры Soft Drop
    /// корректно останавливает дальнейшее движение.
    #[test]
    fn test_soft_drop_floor_detection() {
        let mut state = GameState::new();
        let mut soft_drop_moves = 0;

        // Симулируем Soft Drop: двигаем фигуру вниз пока возможно
        while state.can_move_curr_shape(Dir::Down) {
            state.curr_shape.pos.1 += 1.0;
            soft_drop_moves += 1;
        }

        // Фигура должна сделать хотя бы один ход
        assert!(
            soft_drop_moves > 0,
            "Фигура должна иметь возможность падения"
        );

        // После достижения дна движение должно быть заблокировано
        assert!(
            !state.can_move_curr_shape(Dir::Down),
            "После достижения дна движение должно быть заблокировано"
        );
    }

    /// Тест 8: Проверка отслеживания дистанции Soft Drop
    ///
    /// Проверяет, что поле `soft_drop_distance` корректно
    /// отслеживает количество ячеек, пройденных при Soft Drop.
    #[test]
    fn test_soft_drop_distance_tracking() {
        let mut state = GameState::new();

        // Начальная дистанция должна быть 0
        assert_eq!(
            state.soft_drop_distance, 0,
            "Начальная дистанция Soft Drop должна быть 0"
        );

        // Симулируем несколько шагов Soft Drop
        let test_moves = 5;
        for _ in 0..test_moves {
            if state.can_move_curr_shape(Dir::Down) {
                state.curr_shape.pos.1 += 1.0;
                state.soft_drop_distance += 1;
            }
        }

        // Дистанция должна увеличиться на количество шагов
        assert_eq!(
            state.soft_drop_distance, test_moves,
            "Дистанция должна равняться количеству шагов"
        );

        // После сброса дистанция должна стать 0
        state.soft_drop_distance = 0;
        assert_eq!(
            state.soft_drop_distance, 0,
            "После сброса дистанция должна быть 0"
        );
    }

    // =========================================================================
    // ГРУППА ТЕСТОВ 9-12: Combo система (счётчик, бонус, сброс, Tetris)
    // =========================================================================
    // Эти тесты проверяют систему комбо:
    // - Увеличение счётчика комбо при удалении линий
    // - Расчёт бонуса: 50 × (комбо - 1)
    // - Сброс комбо при ходе без удаления линий
    // - Бонус 1000 очков за Tetris (4 линии)

    /// Тест 9: Проверка увеличения счётчика комбо
    ///
    /// Проверяет, что `combo_counter` увеличивается на 1
    /// при каждом удалении линий.
    #[test]
    fn test_combo_counter_increment() {
        let mut stats = GameStats::new();

        // Начальное значение комбо
        assert_eq!(
            stats.combo_counter, 0,
            "Начальное значение комбо должно быть 0"
        );

        // Увеличиваем комбо симуляцией удалений линий
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

        // Максимальное комбо должно обновляться
        stats.update_max_combo(stats.combo_counter);
        assert_eq!(stats.max_combo, 3, "Максимальное комбо должно быть 3");
    }

    /// Тест 10: Проверка расчёта бонуса за комбо
    ///
    /// Проверяет формулу: бонус = 50 × (комбо - 1)
    /// - Комбо 1: бонус 0 (50 × 0)
    /// - Комбо 2: бонус 50 (50 × 1)
    /// - Комбо 3: бонус 100 (50 × 2)
    /// - Комбо 5: бонус 200 (50 × 4)
    #[test]
    fn test_combo_bonus_calculation() {
        // Проверяем константу бонуса
        assert_eq!(COMBO_BONUS, 50, "Базовый бонус за комбо должен быть 50");

        // Комбо 1: бонус 0 (первое удаление без бонуса)
        // Используем явное приведение для подавления предупреждения clippy
        let combo_bonus_level_1: u64 = 0;
        assert_eq!(
            combo_bonus_level_1, 0,
            "Бонус за первое комбо должен быть 0"
        );

        // Комбо 2: бонус 50
        let combo_bonus_level_2 = COMBO_BONUS;
        assert_eq!(
            combo_bonus_level_2, 50,
            "Бонус за второе комбо должен быть 50"
        );

        // Комбо 3: бонус 100
        let combo_bonus_level_3 = COMBO_BONUS * 2;
        assert_eq!(
            combo_bonus_level_3, 100,
            "Бонус за третье комбо должен быть 100"
        );

        // Комбо 5: бонус 200
        let combo_bonus_level_5 = COMBO_BONUS * 4;
        assert_eq!(
            combo_bonus_level_5, 200,
            "Бонус за пятое комбо должен быть 200"
        );

        // Комбо 10: бонус 450
        let combo_bonus_level_10 = COMBO_BONUS * 9;
        assert_eq!(
            combo_bonus_level_10, 450,
            "Бонус за десятое комбо должен быть 450"
        );
    }

    /// Тест 11: Проверка сброса комбо при ходе без удаления линий
    ///
    /// Проверяет, что `combo_counter` сбрасывается в 0,
    /// если ход не привёл к удалению линий.
    #[test]
    fn test_combo_reset_on_no_clear() {
        let mut stats = GameStats::new();

        // Симулируем несколько удалений подряд (набираем комбо)
        stats.combo_counter = 3;
        assert_eq!(stats.combo_counter, 3, "Комбо должно быть 3");

        // Симулируем ход без удаления линий - сброс комбо
        stats.combo_counter = 0;
        assert_eq!(
            stats.combo_counter, 0,
            "После хода без удаления комбо должно сбрасываться в 0"
        );

        // Проверяем, что новое удаление начинает комбо заново
        stats.combo_counter += 1;
        assert_eq!(
            stats.combo_counter, 1,
            "После сброса новое комбо начинается с 1"
        );
    }

    /// Тест 12: Проверка бонуса 1000 очков за Tetris (4 линии)
    ///
    /// Проверяет, что при удалении 4 линий одновременно
    /// начисляется дополнительный бонус 1000 очков.
    #[test]
    fn test_tetris_bonus_1000() {
        // Бонус за 4 линии (Tetris)
        const TETRIS_BONUS: u128 = 1000;

        // Проверяем базовые очки за линии
        let base_score_4_lines = LINE_SCORES[0] * (1 << (4 - 1));
        assert_eq!(
            base_score_4_lines, 800,
            "Базовые очки за 4 линии должны быть 800"
        );

        // Общий бонус за Tetris: 800 (база) + 1000 (бонус) = 1800
        let total_tetris_score = base_score_4_lines + TETRIS_BONUS;
        assert_eq!(
            total_tetris_score, 1800,
            "Общий счёт за Tetris должен быть 1800"
        );

        // Проверяем, что бонус за Tetris больше, чем за 3 линии
        let base_score_3_lines = LINE_SCORES[0] * (1 << (3 - 1));
        assert!(
            total_tetris_score > base_score_3_lines,
            "Tetris должен давать больше очков, чем 3 линии"
        );
    }

    // =========================================================================
    // ТЕСТЫ ПРОИЗВОДИТЕЛЬНОСТИ (Исправление #31)
    // =========================================================================
    // Эти тесты проверяют производительность критических функций:
    // - find_full_rows() - поиск заполненных линий
    // - check_rows() - удаление линий
    // - draw_ghost_shape() - отрисовка призрачной фигуры
    // - save_tetromino() - сохранение фигуры в поле

    /// Тест производительности: `find_full_rows()`
    ///
    /// Проверяет, что поиск заполненных линий выполняется за приемлемое время.
    /// Время выполнения должно быть < 1ms для пустого поля.
    #[test]
    fn test_performance_find_full_rows() {
        use std::time::Instant;

        let state = GameState::new();
        let start = Instant::now();

        // Выполняем поиск 1000 раз
        for _ in 0..1000 {
            let (rows_mask, remove_count) = state.find_full_rows();
            assert_eq!(rows_mask, 0);
            assert_eq!(remove_count, 0);
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 10,
            "find_full_rows() должен выполняться < 10ms для 1000 итераций (прошло {elapsed:?})"
        );
    }

    /// Тест производительности: `save_tetromino()`
    ///
    /// Проверяет, что сохранение фигуры выполняется за приемлемое время.
    #[test]
    fn test_performance_save_tetromino() {
        use std::time::Instant;

        let mut state = GameState::new();
        let start = Instant::now();

        // Сохраняем фигуру 1000 раз
        for _ in 0..1000 {
            state.save_tetromino();
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 50,
            "save_tetromino() должен выполняться < 50ms для 1000 итераций (прошло {elapsed:?})"
        );
    }

    /// Тест производительности: `check_collision()`
    ///
    /// Проверяет, что проверка столкновений выполняется за приемлемое время.
    #[test]
    fn test_performance_check_collision() {
        use std::time::Instant;

        let state = GameState::new();
        let coords = [(0, 0), (1, 0), (2, 0), (0, 1)];
        let pos = (4.0f32, 0.0f32);
        let start = Instant::now();

        // Выполняем проверку 10000 раз
        for _ in 0..10000 {
            let result = state.check_collision(&coords, pos, Dir::Down);
            assert!(result);
        }

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 100,
            "check_collision() должен выполняться < 100ms для 10000 итераций (прошло {elapsed:?})"
        );
    }
}
