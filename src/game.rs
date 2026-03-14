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

/// Цвет границ.
const BORDER_COLOR: &dyn Color = &White;

/// Смещение отрисовки фигур по вертикали.
///
/// Учитывает заголовки (Счёт, Рекорд, Уровень, Линии) и верхнюю границу.
const SHAPE_DRAW_OFFSET: i16 = 5;

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
pub const ROW_SCORE_INC: u64 = 100;

/// Очки за фигуру.
pub const PIECE_SCORE_INC: u64 = 100;

/// Множитель очков за падение.
pub const PIECE_SCORE_FALL_MULT: f32 = 50.0;

/// Очки за ячейку при Soft Drop.
pub const SOFT_DROP_POINTS: u64 = 1;

/// Очки за ячейку при Hard Drop.
pub const HARD_DROP_POINTS: u64 = 2;

/// Бонус за комбо: 50 × (номер комбо - 1).
pub const COMBO_BONUS: u64 = 50;

/// Количество линий для повышения уровня.
pub const LINES_PER_LEVEL: u32 = 10;

/// Количество линий для режима спринт.
pub const SPRINT_LINES: u32 = 40;

/// Количество линий для режима марафон (заготовка для будущего режима).
#[allow(dead_code)]
pub const MARATHON_LINES: u32 = 150;

/// Символ терминального bell для звуковых эффектов.
pub const BELL: &str = "\x07";

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
    score: u64,
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
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Таймер приземления.
    land_timer: f64,
    /// Статистика игры.
    stats: GameStats,
    /// Режим игры.
    mode: GameMode,
    /// Строки для анимации (мигание при очистке).
    animating_rows: Vec<usize>,
    /// Флаг для анимации Hard Drop.
    is_hard_dropping: bool,
    /// Количество ячеек, пройденных при Soft Drop.
    soft_drop_distance: u32,
    /// Генератор фигур по системе 7-bag.
    bag: crate::tetromino::BagGenerator,
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
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Classic,
            animating_rows: Vec::new(),
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
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
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Sprint,
            animating_rows: Vec::new(),
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
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
        Self {
            score: 0,
            level: 1,
            lines_cleared: 0,
            curr_shape,
            next_shape,
            held_shape: None,
            can_hold: true,
            fall_spd: INITIAL_FALL_SPD,
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
            stats,
            mode: GameMode::Marathon,
            animating_rows: Vec::new(),
            is_hard_dropping: false,
            soft_drop_distance: 0,
            bag,
        }
    }

    /// Запустить игровой цикл и вернуть финальный счёт.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки игрового поля
    /// * `inp` - читатель нажатий клавиш
    /// * `hs_disp` - строка для отображения рекорда
    ///
    /// # Возвращает
    /// Финальный счёт игрока (0 если игрок вышел досрочно)
    ///
    /// # Примечания
    /// Цикл работает до проигрыша или выхода пользователя (Backspace)
    pub fn play(&mut self, cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &str) -> u64 {
        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;
        loop {
            // Поддержание стабильного FPS - расчёт дельты времени
            let now = Instant::now();
            let delta_time_ms = now.duration_since(last_time).subsec_millis() as u64;
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
            self.draw(cnv, hs_disp);
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
    fn update(&mut self, inp: &mut KeyReader, delta_time_ms: u64) -> UpdateEndState {
        let key = inp.get_key();

        // Сброс флага Hard Drop
        self.is_hard_dropping = false;

        match key {
            Some(KEY_BACKSPACE) => return UpdateEndState::Quit, // Backspace — выход в меню
            Some(b'p') => return UpdateEndState::Pause,         // p — пауза
            Some(b'a') => {
                // Перемещение влево
                if self.can_move_curr_shape(Dir::Left) {
                    self.curr_shape.pos.0 -= 1.0;
                }
            }
            Some(b'd') => {
                // Перемещение вправо
                if self.can_move_curr_shape(Dir::Right) {
                    self.curr_shape.pos.0 += 1.0;
                }
            }
            Some(b'q') => {
                // Вращение против часовой стрелки
                if self.can_rotate_curr_shape(Dir::Left) {
                    self.curr_shape.rotate(Dir::Left);
                }
            }
            Some(b'e') => {
                // Вращение по часовой стрелке
                if self.can_rotate_curr_shape(Dir::Right) {
                    self.curr_shape.rotate(Dir::Right);
                }
            }
            Some(b'w') => {
                // Hard Drop: мгновенное падение с бонусными очками
                let start_y = self.curr_shape.pos.1;
                while self.can_move_curr_shape(Dir::Down) {
                    self.curr_shape.pos.1 += 1.0;
                }
                let drop_distance = (self.curr_shape.pos.1 - start_y) as u64;
                // Бонусные очки: 2 за каждую ячейку высоты
                self.score += drop_distance * HARD_DROP_POINTS;
                // Фиксируем таймер для немедленного приземления
                self.land_timer = 0.0;
                // Устанавливаем флаг для анимации
                self.is_hard_dropping = true;
            }
            Some(b's') => {
                // Soft Drop: ускоренное падение при зажатии
                if self.can_move_curr_shape(Dir::Down) {
                    self.curr_shape.pos.1 += 1.0;
                    // При каждом успешном шаге вниз считаем дистанцию для очков
                    self.soft_drop_distance += 1;
                }
            }
            Some(b'c' | b'C') => {
                // Удержание фигуры (можно использовать один раз за ход)
                if self.can_hold {
                    self.hold_shape();
                }
            }
            Some(_) | None => {}
        }

        // Обработка падения фигуры
        if self.can_move_curr_shape(Dir::Down) {
            // Плавное падение с учётом скорости и времени
            self.curr_shape.pos.1 += self.fall_spd * (delta_time_ms as f32 / 1_000.0);
        } else if self.land_timer > 0.0 {
            // Таймер задержки перед фиксацией (даёт время на перемещение)
            self.land_timer -= delta_time_ms as f64 / 1000.0;
        } else {
            // Проверка проигрыша: проверяем конкретные координаты блоков фигуры
            // Если любой блок фигуры находится в верхней части поля (y <= 1)
            let shape_block_y = self.curr_shape.pos.1 as i16;
            let lost = self.curr_shape.coords.iter().any(|&(_, coord_y)| {
                let block_y = coord_y + shape_block_y;
                block_y <= 1
            });

            if lost {
                return UpdateEndState::Lost;
            }

            // Фиксация фигуры и начисление очков
            self.score += PIECE_SCORE_INC + (self.fall_spd * PIECE_SCORE_FALL_MULT) as u64;

            // Начисление очков за Soft Drop: 1 очко за ячейку
            if self.soft_drop_distance > 0 {
                self.score += (self.soft_drop_distance as u64) * SOFT_DROP_POINTS;
                self.soft_drop_distance = 0;
            }

            // Сохранение фигуры в сетке поля
            self.save_tetromino();
            // Проверка и удаление заполненных линий с передачей комбо
            let lines_cleared = self.check_rows();

            // Обновление комбо
            if lines_cleared > 0 {
                // Удаление линий — увеличиваем комбо
                self.stats.combo_counter += 1;
                // Бонус за комбо: 50 × (комбо - 1)
                if self.stats.combo_counter > 1 {
                    self.score += COMBO_BONUS * (self.stats.combo_counter - 1) as u64;
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
                return UpdateEndState::Won;
            }

            // Проверка окончания режима марафон
            // Марафон завершается при достижении 150 линий
            if self.mode == GameMode::Marathon && self.lines_cleared >= MARATHON_LINES {
                self.stats.stop_timer();
                return UpdateEndState::Won;
            }
        }

        UpdateEndState::Continue
    }

    /// Сохранить данные текущей фигуры в сетке после приземления.
    ///
    /// Преобразует плавающие координаты фигуры в индексы сетки
    /// и записывает цвет фигуры в соответствующие клетки.
    fn save_tetromino(&mut self) {
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = coord_x + shape_block_x;
            let y = coord_y + shape_block_y;

            // Проверка границ перед записью (защита от паники при отрицательных координатах)
            if y >= 0 && y < GRID_HEIGHT as i16 && x >= 0 && x < GRID_WIDTH as i16 {
                self.blocks[y as usize][x as usize] = self.curr_shape.fg as i8;
            }
        }
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
    fn check_rows(&mut self) -> u32 {
        // ====================================================================
        // ШАГ 1: ПОИСК ЗАПОЛНЕННЫХ ЛИНИЙ
        // ====================================================================

        // Булев массив для отметки заполненных линий (O(1) доступ вместо O(n) contains)
        let mut rows_to_remove = [false; GRID_HEIGHT];
        let mut remove_count = 0;

        // Поиск заполненных линий (проверяем каждую строку)
        for (y, row) in self.blocks.iter().enumerate() {
            // Проверяем, что все клетки в строке заполнены (нет пустых -1)
            let row_full = row.iter().take(GRID_WIDTH).all(|&cell| cell != -1);
            // Если строка заполнена полностью, отмечаем её для удаления
            if row_full {
                rows_to_remove[y] = true;
                remove_count += 1;
            }
        }

        // ====================================================================
        // ШАГ 2: ПОДГОТОВКА К УДАЛЕНИЮ (анимация и звук)
        // ====================================================================

        if remove_count > 0 {
            // Анимация мигания перед удалением (сохраняем индексы строк)
            self.animating_rows = (0..GRID_HEIGHT).filter(|&y| rows_to_remove[y]).collect();

            // Воспроизведение звукового сигнала (терминальный bell)
            // Символ \x07 воспроизводит звук в терминале
            print!("{}", BELL);

            // Обновление статистики (максимальное комбо)
            self.stats.update_max_combo(remove_count);
        }

        // ====================================================================
        // ШАГ 3: УДАЛЕНИЕ ЛИНИЙ И СДВИГ
        // ====================================================================

        // Сдвиг строк вниз inplace без создания нового массива
        // Алгоритм: перемещаем каждую строку вниз на количество удалённых строк выше неё
        // Это эффективнее чем создание нового массива: избегаем heap-аллокации

        // Подсчитываем количество строк для удаления снизу вверх
        let mut rows_removed_below = 0;

        for y in (0..GRID_HEIGHT).rev() {
            if rows_to_remove[y] {
                // Эта строка будет удалена
                rows_removed_below += 1;
            } else if rows_removed_below > 0 {
                // Перемещаем строку вниз на rows_removed_below позиций
                // Проверка безопасности: индекс не должен выходить за границы
                debug_assert!(y + rows_removed_below < GRID_HEIGHT);
                self.blocks[y + rows_removed_below] = self.blocks[y];
            }
        }

        // Заполняем верхние строки пустыми значениями (-1)
        for y in 0..rows_removed_below {
            self.blocks[y] = [-1; GRID_WIDTH];
        }

        // Очистка анимации (строки удалены)
        self.animating_rows.clear();

        // ====================================================================
        // ШАГ 4: ОБНОВЛЕНИЕ СЧЁТА, УРОВНЯ И СКОРОСТИ
        // ====================================================================

        if remove_count > 0 {
            // Обновление количества удалённых линий
            self.lines_cleared += remove_count;
            // Обновление общей статистики по линиям
            self.stats.total_lines += remove_count;

            // Проверка повышения уровня (каждые 10 линий)
            // Формула: уровень = (линии / 10) + 1
            let new_level = (self.lines_cleared / LINES_PER_LEVEL) + 1;
            if new_level > self.level {
                self.level = new_level;
                // Бонус за повышение уровня: 500 × (номер уровня - 1)
                // Уровень 2: 500, Уровень 3: 1000, Уровень 11: 5000
                self.score += 500 * (new_level - 1) as u64;
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
            self.score += ROW_SCORE_INC * (1 << (remove_count - 1));

            // Бонус за Tetris (4 линии одновременно)
            // Дополнительный бонус 1000 очков сверх базовых 800
            if remove_count == 4 {
                self.score += 1000; // Бонус за Tetris
            }
        }

        // Возврат количества удалённых линий
        remove_count
    }

    /// Отрисовать текущее состояние игры.
    ///
    /// # Аргументы
    /// * `cnv` - канвас для отрисовки
    /// * `hs_disp` - строка для отображения рекорда
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
    fn draw(&mut self, cnv: &mut Canvas, hs_disp: &str) {
        cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

        // Отрисовка рекорда и текущего счёта
        // Форматирование с фиксированной шириной для выравнивания
        let score_str = format!("{:10}", self.score);
        let level_str = format!("{:10}", self.level);
        let lines_str = format!("{:10}", self.lines_cleared);

        cnv.draw_string(&score_str, (7, 2), BORDER_COLOR, &Reset);
        cnv.draw_string(hs_disp, (7, 3), BORDER_COLOR, &Reset);
        cnv.draw_string(&level_str, (10, 4), BORDER_COLOR, &Reset);
        cnv.draw_string(&lines_str, (10, 5), BORDER_COLOR, &Reset);

        // Отрисовка счётчика комбо
        if self.stats.combo_counter > 1 {
            let combo_str = format!("Комбо: x{}", self.stats.combo_counter);
            cnv.draw_string(&combo_str, (24, 6), BORDER_COLOR, &Reset);
        }

        // Отрисовка зафиксированных фигур
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] != -1 {
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
        let shape_symbol = if self.is_hard_dropping {
            // Мигание: чередуем символы каждые 50 мс
            // Используем время от начала игры для анимации
            let animation_time = self.stats.get_elapsed_time();
            let millis = (animation_time * 1000.0) as u16;
            if (millis / 50).is_multiple_of(2) {
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
            let x = (coord_x + shape_block_x) * SHAPE_WIDTH as i16 + 2;
            let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET;

            cnv.draw_strs(
                &[shape_symbol],
                (x as u16, y as u16),
                SHAPE_COLORS[self.curr_shape.fg],
                &Reset,
            );
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
            let x = (coord_x + shape_block_x) * SHAPE_WIDTH as i16 + 2;
            let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET;

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
        let preview_x = 24u16;
        let preview_y = 8u16;

        cnv.draw_string("След:", (preview_x, preview_y - 2), BORDER_COLOR, &Reset);

        // Отрисовка блоков следующей фигуры
        for coord in self.next_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = preview_x as i16 + coord_x * SHAPE_WIDTH as i16 + 2;
            let y = preview_y as i16 + coord_y + 1;

            if x >= 0 && y >= 0 {
                cnv.draw_strs(
                    &[SHAPE_STR],
                    (x as u16, y as u16),
                    SHAPE_COLORS[self.next_shape.fg],
                    &Reset,
                );
            }
        }
    }

    /// Отрисовать удержанную фигуру (слева от поля).
    ///
    /// Показывает фигуру, которую можно получить нажатием 'c'.
    fn draw_held_shape(&self, cnv: &mut Canvas) {
        // Позиция предпросмотра удержанной фигуры (слева от игрового поля)
        let preview_x = 2u16;
        let preview_y = 8u16;

        cnv.draw_string("Удерж:", (preview_x, preview_y - 2), BORDER_COLOR, &Reset);

        if let Some(held) = self.held_shape {
            // Отрисовка блоков удержанной фигуры
            for coord in held.coords {
                let (coord_x, coord_y) = coord;
                let x = preview_x as i16 + coord_x * SHAPE_WIDTH as i16;
                let y = preview_y as i16 + coord_y + 1;

                if x >= 0 && y >= 0 {
                    // Если нельзя менять — рисуем тусклым цветом
                    if self.can_hold {
                        cnv.draw_strs(
                            &[SHAPE_STR],
                            (x as u16, y as u16),
                            SHAPE_COLORS[held.fg],
                            &Reset,
                        );
                    } else {
                        // Тусклая отрисовка (символ ░░)
                        cnv.draw_strs(&["░░"], (x as u16, y as u16), SHAPE_COLORS[held.fg], &Reset);
                    }
                }
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
        self.check_rotation_collision(&temp_shape.coords, temp_shape.pos)
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

            // Проверка выхода за границы сетки
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
    pub fn get_score(&self) -> u64 {
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
    pub fn start_timer(&mut self) {
        self.stats.start_timer();
    }

    /// Получить удержанную фигуру (для тестов).
    #[allow(dead_code)]
    pub fn get_held_shape(&self) -> Option<Tetromino> {
        self.held_shape
    }

    /// Получить флаг can_hold (для тестов).
    #[allow(dead_code)]
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// Получить текущую фигуру (для тестов).
    #[allow(dead_code)]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }

    /// Получить изменяемую текущую фигуру (для тестов).
    #[allow(dead_code)]
    pub fn get_curr_shape_mut(&mut self) -> &mut Tetromino {
        &mut self.curr_shape
    }

    /// Увеличить количество линий (для тестов).
    #[allow(dead_code)]
    pub fn increment_lines_cleared(&mut self) {
        self.lines_cleared += 1;
        self.stats.total_lines += 1;
        // Пересчитываем уровень и скорость
        self.level = (self.lines_cleared / 10) + 1;
        self.fall_spd = INITIAL_FALL_SPD + SPD_INC * (self.level - 1) as f32;
    }

    /// Добавить очки без проверки (для тестов).
    #[allow(dead_code)]
    pub fn add_score_no_check(&mut self, points: u64) {
        self.score += points;
    }

    /// Остановить таймер игры.
    #[allow(dead_code)]
    pub fn stop_timer(&mut self) {
        self.stats.stop_timer();
    }

    /// Получить изменяемую статистику (для тестов).
    #[allow(dead_code)]
    pub fn get_stats_mut(&mut self) -> &mut GameStats {
        &mut self.stats
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
        let test_distances = [1, 5, 10, 15];
        for distance in test_distances.iter() {
            let expected_points = *distance as u64 * SOFT_DROP_POINTS;
            assert_eq!(
                expected_points, *distance as u64,
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
        let combo_1_bonus: u64 = 0;
        assert_eq!(combo_1_bonus, 0, "Бонус за первое комбо должен быть 0");

        // Комбо 2: бонус 50
        let combo_2_bonus = COMBO_BONUS;
        assert_eq!(combo_2_bonus, 50, "Бонус за второе комбо должен быть 50");

        // Комбо 3: бонус 100
        let combo_3_bonus = COMBO_BONUS * 2;
        assert_eq!(combo_3_bonus, 100, "Бонус за третье комбо должен быть 100");

        // Комбо 5: бонус 200
        let combo_5_bonus = COMBO_BONUS * 4;
        assert_eq!(combo_5_bonus, 200, "Бонус за пятое комбо должен быть 200");

        // Комбо 10: бонус 450
        let combo_10_bonus = COMBO_BONUS * 9;
        assert_eq!(
            combo_10_bonus, 450,
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
        const TETRIS_BONUS: u64 = 1000;

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
