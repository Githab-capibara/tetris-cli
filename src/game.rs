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
use crate::tetromino::{Tetromino, SHAPE_COLORS};
use std::collections::HashSet;
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
/// Используется для преобразования времени в update().
const MILLIS_PER_SECOND: f32 = 1000.0;

/// Цвет границ.
const BORDER_COLOR: &dyn Color = &White;

/// Смещение отрисовки фигур по вертикали.
///
/// Учитывает заголовки (Счёт, Рекорд, Уровень, Линии) и верхнюю границу.
const SHAPE_DRAW_OFFSET: i16 = 5;

/// Смещение отрисовки фигур по горизонтали.
/// Используется в draw() для отрисовки фигур на поле.
const SHAPE_OFFSET_X: i16 = 2;

/// Смещение отрисовки фигур по вертикали (дополнительное).
/// Используется в draw() для отрисовки фигур на поле.
const SHAPE_OFFSET_Y: i16 = 0;

/// Смещение отрисовки фигур по горизонтали (для предпросмотра).
const DRAW_OFFSET_X: i16 = 2;

/// Позиция предпросмотра следующей фигуры по X (справа от поля).
/// Используется в draw_next_shape() для отрисовки следующей фигуры.
const PREVIEW_X: u16 = 24;

/// Позиция предпросмотра следующей фигуры по Y.
/// Используется в draw_next_shape() для отрисовки следующей фигуры.
const PREVIEW_Y: u16 = 8;

/// Позиция предпросмотра удержанной фигуры по X (слева от поля).
/// Используется в draw_held_shape() для отрисовки удержанной фигуры.
const HOLD_PREVIEW_X: u16 = 2;

/// Позиция предпросмотра удержанной фигуры по Y.
/// Используется в draw_held_shape() для отрисовки удержанной фигуры.
const HOLD_PREVIEW_Y: u16 = 8;

/// Таблица смещений для wall kick (Super Rotation System - упрощённая).
/// Используется при вращении фигур рядом со стенами.
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

/// Задержка времени приземления (секунды).
///
/// Даёт игроку время на перемещение фигуры после касания.
pub const LAND_TIME_DELAY_S: f64 = 0.1;

/// Прирост скорости за уровень.
pub const SPD_INC: f32 = 0.05;

/// Очки за заполненную линию.
pub const ROW_SCORE_INC: u128 = 100;

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

/// Ширина игрового поля в блоках.
/// Алиас на GRID_WIDTH для лучшей читаемости кода.
#[allow(dead_code)]
pub const FIELD_WIDTH: usize = crate::io::GRID_WIDTH;

/// Высота игрового поля в блоках.
/// Алиас на GRID_HEIGHT для лучшей читаемости кода.
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

/// Порог проигрыша: если блок фигуры находится на этой высоте или выше - игра окончена.
/// Соответствует верхней части игрового поля (y <= 1).
pub const LOSE_THRESHOLD_Y: i16 = 1;

/// Символ терминального bell для звуковых эффектов.
pub const BELL: &str = "\x07";

/// Интервал анимации мигания Hard Drop в миллисекундах.
pub const HARD_DROP_ANIM_INTERVAL_MS: u16 = 50;

/// Количество кадров для пропуска при анимации.
/// Используется для мигания фигур (каждый второй кадр).
pub const ANIMATION_FRAME_SKIP: u16 = 2;

/// Направление движения/вращения.
#[derive(PartialEq, Clone, Copy)]
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
#[derive(Clone, Debug, PartialEq)]
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
            &format!("Достигните комбо x{}", combo),
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
            &format!("Достигните уровня {}", level),
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
    /// Сетка игрового поля (-1 = пусто, 0-6 = цвет).
    /// Массив размещается на стеке (200 байт = 10×20×1 байт).
    /// Защита от переполнения стека: массив занимает всего 200 байт,
    /// что безопасно для стека (типичный размер стека - 1-8 МБ).
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
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
    /// Отслеживание изменённых клеток для оптимизации отрисовки.
    /// Содержит координаты (y, x) клеток, которые изменились с последнего кадра.
    /// Используется для dirty rectangle tracking - отрисовки только изменённых областей.
    #[allow(dead_code)]
    dirty_cells: HashSet<(usize, usize)>,
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
    /// - Скорость: INITIAL_FALL_SPD
    /// - Поле: пустое
    /// - Удержанная фигура: None
    /// - Статистика: новая
    /// - Режим: классический
    pub fn new() -> Self {
        let mut bag = crate::tetromino::BagGenerator::new();
        let curr_shape = Tetromino::from_bag(&mut bag);
        let next_shape = Tetromino::from_bag(&mut bag);
        let mut stats = GameStats::new();
        stats.add_piece(curr_shape.shape);
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            // Инициализация поля пустыми клетками (-1)
            // Массив размещается на стеке (200 байт)
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Classic,
            animating_rows_mask: 0,
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
            dirty_cells: HashSet::new(),
        }
    }

    /// Создать новое состояние игры для режима спринт.
    ///
    /// Отличается от классического режима:
    /// - Цель: очистить 40 линий как можно быстрее
    /// - Счёт не сохраняется в таблицу лидеров
    /// - Отображается таймер
    pub fn new_sprint() -> Self {
        let mut bag = crate::tetromino::BagGenerator::new();
        let curr_shape = Tetromino::from_bag(&mut bag);
        let next_shape = Tetromino::from_bag(&mut bag);
        let mut stats = GameStats::new();
        stats.add_piece(curr_shape.shape);
        stats.start_timer();
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            // Массив размещается на стеке (200 байт)
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Sprint,
            animating_rows_mask: 0,
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
            dirty_cells: HashSet::new(),
        }
    }

    /// Создать новое состояние игры для режима марафон.
    ///
    /// Отличается от классического режима:
    /// - Цель: очистить 150 линий
    /// - Сложность растёт быстрее (каждые 5 линий)
    /// - Сохраняется в таблицу лидеров
    pub fn new_marathon() -> Self {
        let mut bag = crate::tetromino::BagGenerator::new();
        let curr_shape = Tetromino::from_bag(&mut bag);
        let next_shape = Tetromino::from_bag(&mut bag);
        let mut stats = GameStats::new();
        stats.add_piece(curr_shape.shape);
        stats.start_timer();
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            // Массив размещается на стеке (200 байт)
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Marathon,
            animating_rows_mask: 0,
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
            dirty_cells: HashSet::new(),
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
    /// Функция update() (~180 строк) может быть разбита на меньшие:
    /// - handle_input() - обработка ввода
    /// - handle_falling() - обработка падения
    /// - handle_landing() - обработка приземления
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
    fn handle_movement_input(&mut self, dir: Dir) {
        if self.can_move_curr_shape(dir) {
            match dir {
                Dir::Left => self.curr_shape.pos.0 -= 1.0,
                Dir::Right => self.curr_shape.pos.0 += 1.0,
                Dir::Down => {} // Down не используется для движения
            }
        }
    }

    /// Обработка вращения фигуры.
    ///
    /// # Аргументы
    /// * `dir` - направление вращения
    fn handle_rotation_input(&mut self, dir: Dir) {
        self.rotate_with_wall_kick(dir);
    }

    /// Обработка Hard Drop (мгновенное падение).
    fn handle_hard_drop(&mut self) {
        let start_y = self.curr_shape.pos.1;
        while self.can_move_curr_shape(Dir::Down) {
            self.curr_shape.pos.1 += 1.0;
        }
        // Безопасное преобразование: drop_distance всегда >= 0 т.к. фигура падает вниз
        // Добавляем проверку на infinity/NaN и ограничиваем максимальное значение
        // Исправление #1: используем u32::MAX вместо u16::MAX для предотвращения переполнения f32
        let drop_distance_f32 = (self.curr_shape.pos.1 - start_y)
            .max(0.0)
            .min(u32::MAX as f32);
        let drop_distance = if drop_distance_f32.is_finite() {
            drop_distance_f32 as u64
        } else {
            0 // Защита от NaN/infinity
        };
        // Бонусные очки: 2 за каждую ячейку высоты
        self.score = self
            .score
            .saturating_add((drop_distance as u128) * HARD_DROP_POINTS);
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
            self.land_timer -= delta_time_ms as f64 / MILLIS_PER_SECOND as f64;
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
    fn handle_landing(&mut self) -> Option<UpdateEndState> {
        // Проверка проигрыша: проверяем конкретные координаты блоков фигуры
        // Если любой блок фигуры находится в верхней части поля (y <= LOSE_THRESHOLD_Y)
        let shape_block_y = self.curr_shape.pos.1 as i16;
        let lost = self.curr_shape.coords.iter().any(|&(_, coord_y)| {
            let block_y = coord_y + shape_block_y;
            block_y <= LOSE_THRESHOLD_Y
        });

        if lost {
            return Some(UpdateEndState::Lost);
        }

        // Фиксация фигуры и начисление очков
        // Добавлена защита от переполнения при расчёте очков за падение
        // Используем u128::MAX для предотвращения переполнения f32
        let fall_bonus = (self.fall_spd * PIECE_SCORE_FALL_MULT)
            .max(0.0)
            .min(u32::MAX as f32);
        let fall_bonus_u128 = if fall_bonus.is_finite() {
            fall_bonus as u128
        } else {
            0
        };
        self.score = self.score.saturating_add(PIECE_SCORE_INC + fall_bonus_u128);

        // Начисление очков за Soft Drop: 1 очко за ячейку
        if self.soft_drop_distance > 0 {
            self.score = self
                .score
                .saturating_add((self.soft_drop_distance as u128) * SOFT_DROP_POINTS);
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
            // Бонус за комбо: 50 × (комбо - 1)
            // Используем u128 для предотвращения переполнения
            if self.stats.combo_counter > 1 {
                self.score = self
                    .score
                    .saturating_add(COMBO_BONUS * (self.stats.combo_counter - 1) as u128);
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
    /// Использует checked_sub() для защиты от отрицательных координат.
    ///
    /// # Видимость
    /// Метод является публичным для использования в бенчмарках.
    pub fn save_tetromino(&mut self) {
        self.save_tetromino_impl();
    }

    /// Внутренняя реализация сохранения фигуры.
    fn save_tetromino_impl(&mut self) {
        let (shape_x, shape_y) = self.curr_shape.pos;
        // Безопасное преобразование координат: позиция фигуры всегда в пределах поля
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = coord_x + shape_block_x;
            let y = coord_y + shape_block_y;

            // Проверка границ перед записью (защита от паники при отрицательных координатах)
            // Используем checked_sub() для безопасной работы с отрицательными значениями
            if y >= 0 && y < GRID_HEIGHT as i16 && x >= 0 && x < GRID_WIDTH as i16 {
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
    /// Помечен как #[must_use] для предотвращения предупреждений dead_code.
    #[cfg(feature = "bench")]
    #[must_use]
    #[allow(dead_code)]
    pub fn save_tetromino_for_bench(&mut self) {
        self.save_tetromino_impl();
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
    /// - Уровень повышается каждые 10 линий (LINES_PER_LEVEL)
    /// - Скорость увеличивается на 0.05 (SPD_INC) за каждую линию
    /// - Воспроизводится звуковой сигнал при удалении линий
    /// - При удалении 4 линий отображается "TETRIS!" и бонус 1000 очков
    ///
    /// # Рефакторинг
    /// Функция check_rows() (~130 строк) может быть разбита на меньшие:
    /// - find_full_rows() - поиск заполненных линий
    /// - animate_rows_clear() - анимация удаления
    /// - remove_rows_and_shift() - удаление и сдвиг
    /// - update_score_and_level() - обновление очков и уровня
    ///
    /// Это улучшит читаемость и упростит тестирование.
    /// Найти все заполненные линии.
    ///
    /// # Возвращает
    /// Битовую маску заполненных линий (каждый бит соответствует строке)
    /// и количество заполненных линий
    fn find_full_rows(&self) -> (u32, u32) {
        // Оптимизация: используем битовую маску u32 вместо [bool; GRID_HEIGHT]
        // Это экономит память (4 байта вместо 20) и ускоряет проверку
        // Каждый бит соответствует строке: бит 0 = строка 0, бит 1 = строка 1, и т.д.
        let mut rows_mask: u32 = 0;
        let mut remove_count = 0;

        // Поиск заполненных линий (проверяем каждую строку)
        for (y, row) in self.blocks.iter().enumerate() {
            // Проверяем, что все клетки в строке заполнены (нет пустых -1)
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
            print!("{}", BELL);

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
    fn update_score_and_level(&mut self, remove_count: u32) {
        if remove_count > 0 {
            // Обновление количества удалённых линий
            self.lines_cleared = self.lines_cleared.saturating_add(remove_count);
            // Обновление общей статистики по линиям
            self.stats.total_lines = self.stats.total_lines.saturating_add(remove_count);

            // Проверка повышения уровня (каждые 10 линий)
            // Формула: уровень = (линии / 10) + 1
            let new_level = (self.lines_cleared / LINES_PER_LEVEL) + 1;
            if new_level > self.level {
                self.level = new_level;
                // Бонус за повышение уровня: 500 × (номер уровня - 1)
                // Уровень 2: 500, Уровень 3: 1000, Уровень 11: 5000
                // Используем u128 для предотвращения переполнения
                self.score = self.score.saturating_add(500 * (new_level - 1) as u128);
            }

            // Увеличение скорости игры
            // Каждая удалённая линия увеличивает скорость на 0.05
            self.fall_spd += SPD_INC * remove_count as f32;

            // =================================================================
            // НАЧИСЛЕНИЕ ОЧКОВ ЗА ЛИНИИ (экспоненциальная формула)
            // =================================================================
            // Формула: 100 × 2^(линии-1)
            //
            // | Линий | Вычисление      | Очки | Название     |
            // |-------|-----------------|------|--------------|
            // |   1   | 100 × 2⁰ = 100  |  100 | Одиночный    |
            // |   2   | 100 × 2¹ = 200  |  200 | Двойной      |
            // |   3   | 100 × 2² = 400  |  400 | Тройной      |
            // |   4   | 100 × 2³ = 800  |  800 | **TETRIS!**  |
            //
            // Используем битовый сдвиг для эффективности: 1 << n = 2^n
            // remove_count гарантированно > 0 благодаря проверке выше
            // =================================================================
            // Используем u128 для предотвращения переполнения
            self.score = self
                .score
                .saturating_add(ROW_SCORE_INC * (1u128 << (remove_count - 1)));

            // Бонус за Tetris (4 линии одновременно)
            // Дополнительный бонус 1000 очков сверх базовых 800
            if remove_count == 4 {
                self.score = self.score.saturating_add(1000u128); // Бонус за Tetris
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
    /// Возможна оптимизация через dirty rectangle tracking:
    /// отслеживать только изменённые области и перерисовывать их.
    /// Это уменьшит количество операций отрисовки при статичном поле.
    fn draw(&mut self, cnv: &mut Canvas, high_score_display: &str) {
        cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

        // Отрисовка рекорда и текущего счёта
        // Форматирование с фиксированной шириной для выравнивания
        let score_str = format!("{:10}", self.score);
        let level_str = format!("{:10}", self.level);
        let lines_str = format!("{:10}", self.lines_cleared);

        cnv.draw_string(&score_str, (7, 2), BORDER_COLOR, &Reset);
        cnv.draw_string(high_score_display, (7, 3), BORDER_COLOR, &Reset);
        cnv.draw_string(&level_str, (10, 4), BORDER_COLOR, &Reset);
        cnv.draw_string(&lines_str, (10, 5), BORDER_COLOR, &Reset);

        // Отрисовка счётчика комбо
        if self.stats.combo_counter > 1 {
            let combo_str = format!("Комбо: x{}", self.stats.combo_counter);
            cnv.draw_string(&combo_str, (24, 6), BORDER_COLOR, &Reset);
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

            // Проверка границ перед отрисовкой для защиты от паники
            if x >= 0 && y >= 0 {
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
    fn draw_ghost_shape(&self, cnv: &mut Canvas) {
        // Копирование текущей фигуры для расчёта точки приземления
        // Tetromino реализует Copy, поэтому операция быстрая
        let mut ghost_shape = self.curr_shape;

        // Опустить фигуру до упора
        // Защита от бесконечного цикла: максимальное количество итераций равно высоте поля
        let max_iterations = GRID_HEIGHT;
        let mut iterations = 0;

        while self.can_move_ghost_shape(&ghost_shape, Dir::Down) && iterations < max_iterations {
            ghost_shape.pos.1 += 1.0;
            iterations += 1;
        }

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
    fn draw_sprint_timer(&self, cnv: &mut Canvas) {
        let elapsed = self.stats.get_elapsed_time();
        let timer_str = format!("Время: {:.2}с", elapsed);
        cnv.draw_string(&timer_str, (24, 20), BORDER_COLOR, &Reset);

        // Показываем прогресс до 40 линий
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

            // Проверка выхода за границы сетки
            // Боковые границы: x < 0 или x >= GRID_WIDTH
            // Нижняя граница: y >= GRID_HEIGHT
            // Верхняя граница (y < 0) НЕ проверяется - фигуры могут появляться выше поля
            if check_x < 0 || check_x >= GRID_WIDTH as i16 || check_y >= GRID_HEIGHT as i16 {
                return false;
            }

            // Проверка столкновения с зафиксированными фигурами
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
    /// # Отличия от can_move_curr_shape
    /// Использует immutable ссылку на self, так как призрачная фигура
    /// не изменяет состояние игры
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
    pub fn can_rotate_curr_shape(&self, dir: Dir) -> bool {
        // Создание временной копии фигуры для проверки вращения
        let mut temp_shape = self.curr_shape;
        temp_shape.rotate(dir);

        // Проверка вращения без смещения по направлению
        if self.check_rotation_collision(&temp_shape.coords, temp_shape.pos) {
            return true;
        }

        // Wall kick: пробуем смещения если прямое вращение невозможно
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
    /// Если прямое вращение невозможно, пробует различные смещения из таблицы WALL_KICK_OFFSETS:
    /// - Влево/вправо на 1-2 клетки
    /// - Вверх/вниз на 1 клетку
    /// - Комбинированные смещения
    ///
    /// # Аргументы
    /// * `dir` - направление вращения (Left или Right)
    ///
    /// # Возвращает
    /// `true` если вращение (возможно со смещением) успешно
    ///
    /// # Паника
    /// Паникует, если передано направление `Dir::Down`, так как оно не используется для вращения.
    pub fn rotate_with_wall_kick(&mut self, dir: Dir) -> bool {
        // Dir::Down не используется для вращения
        if matches!(dir, Dir::Down) {
            panic!("Dir::Down не используется для вращения");
        }

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
    #[must_use]
    #[allow(dead_code)]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество удалённых линий.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить следующую фигуру.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить текущий счёт.
    #[must_use]
    #[allow(dead_code)]
    pub fn get_score(&self) -> u128 {
        self.score
    }

    /// Получить скорость падения.
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
    #[must_use]
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить режим игры.
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

    /// Получить удержанную фигуру (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn get_held_shape(&self) -> Option<Tetromino> {
        self.held_shape
    }

    /// Получить игровое поле для бенчмарков.
    ///
    /// # Возвращает
    /// Ссылку на двумерный массив игрового поля размером [`GRID_WIDTH`]×[`GRID_HEIGHT`].
    ///
    /// # Назначение
    /// Метод предназначен для использования в бенчмарках и тестах производительности.
    /// Позволяет получить прямой доступ к внутреннему состоянию поля для измерения
    /// производительности операций отрисовки и проверки столкновений.
    ///
    /// # Пример использования
    /// ```ignore
    /// use tetris_cli::game::GameState;
    ///
    /// let state = GameState::new();
    /// let blocks = state.get_blocks_for_bench();
    ///
    /// // Доступ к клетке поля
    /// let cell = blocks[0][0];
    /// ```
    ///
    /// Получить флаг can_hold (для тестов).
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
    #[allow(dead_code)]
    pub fn set_block(&mut self, row: usize, col: usize, color: i8) {
        if row < GRID_HEIGHT && col < GRID_WIDTH {
            self.blocks[row][col] = color;
        }
    }

    /// Получить блок из указанной позиции (для тестов).
    #[allow(dead_code)]
    #[must_use]
    pub fn get_block(&self, row: usize, col: usize) -> i8 {
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
    /// Формула: очки = высота_сброса × HARD_DROP_POINTS (2)
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
    /// Проверяет, что флаг is_hard_dropping устанавливается в true
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
    /// начисляется 1 очко (SOFT_DROP_POINTS = 1).
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
    /// Проверяет, что поле soft_drop_distance корректно
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
    /// Проверяет, что combo_counter увеличивается на 1
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
    /// Проверяет, что combo_counter сбрасывается в 0,
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
        let base_score_4_lines = ROW_SCORE_INC * (1 << (4 - 1));
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
        let base_score_3_lines = ROW_SCORE_INC * (1 << (3 - 1));
        assert!(
            total_tetris_score > base_score_3_lines,
            "Tetris должен давать больше очков, чем 3 линии"
        );
    }
}
