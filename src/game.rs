//! Основной игровой цикл.
//!
//! Автор: Dylan Turner

use crate::io::{Canvas, KeyReader, DISP_HEIGHT, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH};
use crate::tetromino::{Tetromino, SHAPE_COLORS};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use termion::color::{Color, Reset, White};

/// Количество кадров в секунду.
pub const FPS: u64 = 60;
/// Границы игрового поля с заголовками.
const BORDER: [&str; DISP_HEIGHT as usize] = [
    "                      ",
    "Счёт:                 ",
    "Рекорд:               ",
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
const SHAPE_DRAW_OFFSET: i16 = 5;
/// Начальная скорость падения.
const INITIAL_FALL_SPD: f32 = 0.9;
/// Задержка времени приземления (секунды).
const LAND_TIME_DELAY_S: f64 = 0.1;
/// Прирост скорости.
const SPD_INC: f32 = 0.05;
/// Очки за заполненную линию.
const ROW_SCORE_INC: u64 = 100;
/// Очки за фигуру.
const PIECE_SCORE_INC: u64 = 100;
/// Множитель очков за падение.
const PIECE_SCORE_FALL_MULT: f32 = 50.0;

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

/// Состояние игры.
pub struct GameState {
    /// Текущий счёт.
    score: u64,
    /// Текущая фигура.
    curr_shape: Tetromino,
    /// Скорость падения.
    fall_spd: f32,
    /// Сетка игрового поля (-1 = пусто, 0-6 = цвет).
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    /// Таймер приземления.
    land_timer: f64,
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
    pub fn new() -> Self {
        Self {
            score: 0,
            curr_shape: Tetromino::select(),
            fall_spd: INITIAL_FALL_SPD,
            // Инициализация поля пустыми клетками (-1)
            blocks: [[-1; GRID_WIDTH]; GRID_HEIGHT],
            land_timer: LAND_TIME_DELAY_S,
        }
    }

    /// Запустить игровой цикл и вернуть финальный счёт.
    pub fn play(&mut self, cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &str) -> u64 {
        let mut last_time = Instant::now();
        let interval_ms = 1_000 / FPS;
        loop {
            // Поддержание стабильного FPS
            let now = Instant::now();
            let delta_time_ms = now.duration_since(last_time).subsec_millis() as u64;
            if delta_time_ms < interval_ms {
                sleep(Duration::from_millis(interval_ms - delta_time_ms));
                continue;
            }
            last_time = now;

            match self.update(inp, delta_time_ms) {
                UpdateEndState::Continue => {}
                UpdateEndState::Quit => {
                    return 0;
                }
                UpdateEndState::Lost => {
                    // Отображение сообщения о проигрыше
                    cnv.draw_strs(&GAME_OVER, (10, 12), BORDER_COLOR, &Reset);
                    cnv.flush();
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
                            // Backspace во время паузы — выход
                            cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                            return 0;
                        }
                        cnv.draw_strs(&PAUSE, (7, 13), BORDER_COLOR, &Reset);
                        sleep(Duration::from_millis(interval_ms));
                    }
                }
            }
            self.draw(cnv, hs_disp);
        }

        self.score
    }

    /// Обновить состояние игры.
    fn update(&mut self, inp: &mut KeyReader, delta_time_ms: u64) -> UpdateEndState {
        let key = inp.get_key();
        match key {
            127 => return UpdateEndState::Quit,   // Backspace — выход в меню
            b'p' => return UpdateEndState::Pause, // p — пауза
            b'a' => {
                if self.can_move_curr_shape(Dir::Left) {
                    self.curr_shape.pos.0 -= 1.0;
                }
            }
            b'd' => {
                if self.can_move_curr_shape(Dir::Right) {
                    self.curr_shape.pos.0 += 1.0;
                }
            }
            b'q' => {
                if self.can_rotate_curr_shape(Dir::Left) {
                    self.curr_shape.rotate(Dir::Left);
                }
            }
            b'e' => {
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
            _ => {}
        }

        if self.can_move_curr_shape(Dir::Down) {
            // Плавное падение с учётом скорости и времени
            self.curr_shape.pos.1 += self.fall_spd * (delta_time_ms as f32 / 1_000.0);
        } else if self.land_timer > 0.0 {
            // Таймер перед фиксацией фигуры (даёт время на перемещение)
            self.land_timer -= delta_time_ms as f64 / 1000.0;
        } else if self.curr_shape.pos.1 <= 1.0 {
            // Фигура застряла вверху — проигрыш
            return UpdateEndState::Lost;
        } else {
            // Фиксация фигуры и начисление очков
            self.score += PIECE_SCORE_INC + (self.fall_spd * PIECE_SCORE_FALL_MULT) as u64;

            self.save_tetromino();
            self.check_rows();

            self.land_timer = LAND_TIME_DELAY_S;
            self.curr_shape = Tetromino::select();
        }

        UpdateEndState::Continue
    }

    /// Сохранить данные текущей фигуры в сетке после приземления.
    fn save_tetromino(&mut self) {
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in self.curr_shape.coords {
            let (coord_x, coord_y) = coord;
            let x = (coord_x + shape_block_x) as usize;
            let y = (coord_y + shape_block_y) as usize;

            self.blocks[y][x] = self.curr_shape.fg as i8;
        }
    }

    /// Проверить заполненные линии и удалить их.
    fn check_rows(&mut self) {
        let mut rows_to_remove = Vec::new();

        // Поиск заполненных линий
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

        // Удаление заполненных линий и сдвиг верхних строк вниз
        for (shift_count, &row) in rows_to_remove.iter().rev().enumerate() {
            // Сдвиг всех строк выше на (shift_count + 1) вниз
            for y in (0..row).rev() {
                self.blocks[y + shift_count + 1] = self.blocks[y];
            }
            // Очистка верхней строки
            self.blocks[shift_count] = [-1; GRID_WIDTH];
        }

        if num_filled_rows > 0 {
            // Увеличение скорости игры
            self.fall_spd += SPD_INC * num_filled_rows as f32;

            // Начисление очков за линии
            // Бонус за несколько линий: 100, 200, 400, 800...
            self.score += ROW_SCORE_INC * (1 << (num_filled_rows - 1));
        }
    }

    /// Отрисовать текущее состояние игры.
    fn draw(&mut self, cnv: &mut Canvas, hs_disp: &str) {
        cnv.draw_strs(&BORDER, (1, 1), BORDER_COLOR, &Reset);

        // Отрисовка рекорда и текущего счёта
        let score_str = format!("{:020}", self.score);
        cnv.draw_string(&score_str, (7, 2), BORDER_COLOR, &Reset);
        cnv.draw_string(hs_disp, (9, 2), BORDER_COLOR, &Reset);

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

        cnv.flush();
    }

    /// Проверить возможность движения текущей фигуры в заданном направлении.
    fn can_move_curr_shape(&mut self, dir: Dir) -> bool {
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;

        for coord in self.curr_shape.coords {
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

    /// Проверить возможность вращения текущей фигуры.
    fn can_rotate_curr_shape(&mut self, dir: Dir) -> bool {
        // Создание временной копии фигуры для проверки вращения
        let mut temp_shape = self.curr_shape;
        temp_shape.rotate(dir);

        // Проверка валидности новой позиции
        let (shape_x, shape_y) = temp_shape.pos;
        let shape_block_x = shape_x as i16;
        let shape_block_y = shape_y as i16;
        for coord in temp_shape.coords {
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
}
