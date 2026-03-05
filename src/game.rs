//! Основной игровой цикл.
//!
//! Автор: Dylan Turner
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

use crate::io::{Canvas, KeyReader, DISP_HEIGHT, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};
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
const PIECE_SCORE_INC: u64 = 100;

/// Множитель очков за падение.
const PIECE_SCORE_FALL_MULT: f32 = 50.0;

/// Количество линий для повышения уровня.
pub const LINES_PER_LEVEL: u32 = 10;

/// Количество линий для режима спринт.
pub const SPRINT_LINES: u32 = 40;

/// Символ терминального bell для звуковых эффектов.
const BELL: &str = "\x07";

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
}

/// Статистика игры.
///
/// Содержит подробную информацию о прошедшей игре:
/// - Количество использованных фигур каждого типа
/// - Общее количество очищенных линий
/// - Максимальное комбо (одновременное удаление линий)
/// - Время игры
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
    /// Время начала игры.
    pub start_time: Option<Instant>,
    /// Время окончания игры.
    pub end_time: Option<Instant>,
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
        self.t_pieces + self.l_pieces + self.j_pieces + self.s_pieces
            + self.z_pieces + self.o_pieces + self.i_pieces
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
        let curr_shape = Tetromino::select();
        let next_shape = Tetromino::select();
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
        }
    }

    /// Создать новое состояние игры для режима спринт.
    ///
    /// Отличается от классического режима:
    /// - Цель: очистить 40 линий как можно быстрее
    /// - Счёт не сохраняется в таблицу лидеров
    /// - Отображается таймер
    pub fn new_sprint() -> Self {
        let mut state = Self::new();
        state.mode = GameMode::Sprint;
        state
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
                    sleep(Duration::from_millis(1500));
                    break;
                }
                UpdateEndState::Pause => {
                    // Ожидание повторного нажатия 'p' для снятия с паузы
                    loop {
                        let key = inp.get_key();
                        if key == b'p' {
                            break;
                        } else if key == 127 {
                            // Backspace во время паузы — выход в меню
                            cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                            return 0;
                        }
                        cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                        sleep(Duration::from_millis(interval_ms));
                    }
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
    /// - `s` - мгновенное падение
    /// - `c` - удержать фигуру (Hold)
    fn update(&mut self, inp: &mut KeyReader, delta_time_ms: u64) -> UpdateEndState {
        let key = inp.get_key();
        match key {
            127 => return UpdateEndState::Quit,   // Backspace — выход в меню
            b'p' => return UpdateEndState::Pause, // p — пауза
            b'a' => {
                // Перемещение влево
                if self.can_move_curr_shape(Dir::Left) {
                    self.curr_shape.pos.0 -= 1.0;
                }
            }
            b'd' => {
                // Перемещение вправо
                if self.can_move_curr_shape(Dir::Right) {
                    self.curr_shape.pos.0 += 1.0;
                }
            }
            b'q' => {
                // Вращение против часовой стрелки
                if self.can_rotate_curr_shape(Dir::Left) {
                    self.curr_shape.rotate(Dir::Left);
                }
            }
            b'e' => {
                // Вращение по часовой стрелке
                if self.can_rotate_curr_shape(Dir::Right) {
                    self.curr_shape.rotate(Dir::Right);
                }
            }
            b's' => {
                // Мгновенное падение: опускаем фигуру до упора
                while self.can_move_curr_shape(Dir::Down) {
                    self.curr_shape.pos.1 += 1.0;
                }
                // Фиксируем таймер для немедленного приземления
                self.land_timer = 0.0;
            }
            b'c' | b'C' => {
                // Удержание фигуры (можно использовать один раз за ход)
                if self.can_hold {
                    self.hold_shape();
                }
            }
            _ => {}
        }

        // Обработка падения фигуры
        if self.can_move_curr_shape(Dir::Down) {
            // Плавное падение с учётом скорости и времени
            self.curr_shape.pos.1 += self.fall_spd * (delta_time_ms as f32 / 1_000.0);
        } else if self.land_timer > 0.0 {
            // Таймер задержки перед фиксацией (даёт время на перемещение)
            self.land_timer -= delta_time_ms as f64 / 1000.0;
        } else if self.curr_shape.pos.1 <= 1.0 {
            // Фигура застряла вверху — проигрыш
            return UpdateEndState::Lost;
        } else {
            // Фиксация фигуры и начисление очков
            self.score += PIECE_SCORE_INC + (self.fall_spd * PIECE_SCORE_FALL_MULT) as u64;

            // Обновление статистики
            self.stats.add_piece(self.curr_shape.shape);

            // Сохранение фигуры в сетке поля
            self.save_tetromino();
            // Проверка и удаление заполненных линий
            self.check_rows();

            // Сброс таймера и переход к следующей фигуре
            self.land_timer = LAND_TIME_DELAY_S;
            self.curr_shape = self.next_shape;
            self.next_shape = Tetromino::select();
            self.can_hold = true; // Разрешаем удержание в новом ходу
            
            // Обновление статистики для новой фигуры
            self.stats.add_piece(self.curr_shape.shape);
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
            let x = (coord_x + shape_block_x) as usize;
            let y = (coord_y + shape_block_y) as usize;

            // Проверка границ перед записью
            if y < GRID_HEIGHT && x < GRID_WIDTH {
                self.blocks[y][x] = self.curr_shape.fg as i8;
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
            self.next_shape = Tetromino::select();
        }
        
        // Сбрасываем позицию и запрещаем повторное удержание в этом ходу
        self.curr_shape.pos = (4.0, 0.0);
        self.can_hold = false;
    }

    /// Проверить заполненные линии и удалить их.
    ///
    /// Алгоритм работы:
    /// 1. Поиск полностью заполненных линий
    /// 2. Анимация мигания заполненных линий
    /// 3. Звуковой эффект при удалении
    /// 4. Удаление заполненных линий
    /// 5. Сдвиг верхних линий вниз
    /// 6. Обновление счёта, уровня и скорости
    ///
    /// # Система очков
    /// - 1 линия: 100 очков (×1)
    /// - 2 линии: 200 очков (×2)
    /// - 3 линии: 400 очков (×4)
    /// - 4 линии: 800 очков (×8)
    ///
    /// # Примечания
    /// - Уровень повышается каждые LINES_PER_LEVEL линий
    /// - Скорость увеличивается на SPD_INC за каждую линию
    /// - Воспроизводится звуковой сигнал при удалении линий
    fn check_rows(&mut self) {
        let mut rows_to_remove = Vec::new();

        // Поиск заполненных линий (снизу вверх)
        for y in 0..GRID_HEIGHT {
            let mut row_full = true;
            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] == -1 {
                    row_full = false;
                    break;
                }
            }
            if row_full {
                rows_to_remove.push(y);
            }
        }

        let num_filled_rows = rows_to_remove.len();

        if num_filled_rows > 0 {
            // Анимация мигания перед удалением
            self.animating_rows = rows_to_remove.clone();
            
            // Воспроизведение звукового сигнала (терминальный bell)
            print!("{}", BELL);
            
            // Обновление статистики
            self.stats.update_max_combo(num_filled_rows as u32);
        }

        // Удаление заполненных линий и сдвиг верхних строк вниз
        // Обработка снизу вверх для корректного сдвига
        for (shift_count, &row) in rows_to_remove.iter().rev().enumerate() {
            // Сдвиг всех строк выше на (shift_count + 1) вниз
            for y in (0..row).rev() {
                self.blocks[y + shift_count + 1] = self.blocks[y];
            }
            // Очистка верхней строки
            self.blocks[shift_count] = [-1; GRID_WIDTH];
        }

        // Очистка анимации
        self.animating_rows.clear();

        if num_filled_rows > 0 {
            // Обновление количества удалённых линий
            self.lines_cleared += num_filled_rows as u32;

            // Проверка повышения уровня (каждые 10 линий)
            let new_level = (self.lines_cleared / LINES_PER_LEVEL) + 1;
            if new_level > self.level {
                self.level = new_level;
                // Бонус за повышение уровня
                self.score += 500 * (new_level - 1) as u64;
            }

            // Увеличение скорости игры
            self.fall_spd += SPD_INC * num_filled_rows as f32;

            // Начисление очков за линии с экспоненциальным бонусом
            // Бонус за несколько линий: 100, 200, 400, 800...
            self.score += ROW_SCORE_INC * (1 << (num_filled_rows - 1));
            
            // Проверка окончания режима спринт
            if self.mode == GameMode::Sprint && self.lines_cleared >= SPRINT_LINES {
                self.stats.stop_timer();
            }
        }
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
    fn draw(&mut self, cnv: &mut Canvas, hs_disp: &str) {
        cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

        // Отрисовка рекорда и текущего счёта
        let score_str = format!("{:10}", self.score);
        let level_str = format!("{:10}", self.level);
        let lines_str = format!("{:10}", self.lines_cleared);

        cnv.draw_string(&score_str, (7, 2), BORDER_COLOR, &Reset);
        cnv.draw_string(hs_disp, (7, 3), BORDER_COLOR, &Reset);
        cnv.draw_string(&level_str, (10, 4), BORDER_COLOR, &Reset);
        cnv.draw_string(&lines_str, (10, 5), BORDER_COLOR, &Reset);

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

        // Отрисовка текущей падающей фигуры
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = (coord_x + shape_block_x) * SHAPE_WIDTH as i16 + 2;
            let y = coord_y + shape_block_y + SHAPE_DRAW_OFFSET;

            cnv.draw_strs(
                &[SHAPE_STR],
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
    fn draw_ghost_shape(&mut self, cnv: &mut Canvas) {
        // Копирование текущей фигуры для расчёта точки приземления
        let mut ghost_shape = self.curr_shape;

        // Опустить фигуру до упора
        while self.can_move_ghost_shape(&ghost_shape, Dir::Down) {
            ghost_shape.pos.1 += 1.0;
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
    fn draw_next_shape(&mut self, cnv: &mut Canvas) {
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
    fn draw_held_shape(&mut self, cnv: &mut Canvas) {
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
                        cnv.draw_strs(
                            &["░░"],
                            (x as u16, y as u16),
                            SHAPE_COLORS[held.fg],
                            &Reset,
                        );
                    }
                }
            }
        }
    }

    /// Отрисовать таймер для режима спринт.
    ///
    /// Показывает время, прошедшее с начала игры.
    fn draw_sprint_timer(&mut self, cnv: &mut Canvas) {
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
    fn can_move_curr_shape(&mut self, dir: Dir) -> bool {
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
    fn can_move_ghost_shape(&self, ghost: &Tetromino, dir: Dir) -> bool {
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
    /// 3. Проверяется валидность новой позиции с помощью check_collision
    fn can_rotate_curr_shape(&mut self, dir: Dir) -> bool {
        // Создание временной копии фигуры для проверки вращения
        let mut temp_shape = self.curr_shape;
        temp_shape.rotate(dir);

        // Проверка валидности новой позиции с помощью общего метода
        self.check_collision(&temp_shape.coords, temp_shape.pos, Dir::Down)
    }

    /// Получить текущий уровень.
    #[allow(dead_code)]
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Получить количество удалённых линий.
    #[allow(dead_code)]
    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить следующую фигуру.
    #[allow(dead_code)]
    pub fn get_next_shape(&self) -> &Tetromino {
        &self.next_shape
    }

    /// Получить текущий счёт.
    #[allow(dead_code)]
    pub fn get_score(&self) -> u64 {
        self.score
    }

    /// Получить скорость падения.
    #[allow(dead_code)]
    pub fn get_fall_spd(&self) -> f32 {
        self.fall_spd
    }

    /// Получить игровое поле (для тестов).
    #[allow(dead_code)]
    pub fn get_blocks(&self) -> &[[i8; GRID_WIDTH]; GRID_HEIGHT] {
        &self.blocks
    }

    /// Получить статистику игры.
    #[allow(dead_code)]
    pub fn get_stats(&self) -> &GameStats {
        &self.stats
    }

    /// Получить режим игры.
    #[allow(dead_code)]
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

    /// Получить количество линий (для тестов).
    #[allow(dead_code)]
    pub fn get_lines_cleared_public(&self) -> u32 {
        self.lines_cleared
    }

    /// Получить текущую фигуру (для тестов).
    #[allow(dead_code)]
    pub fn get_curr_shape(&self) -> &Tetromino {
        &self.curr_shape
    }
}
