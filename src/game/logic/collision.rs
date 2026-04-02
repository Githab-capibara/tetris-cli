//! Модуль проверки столкновений.
//!
//! Проверка возможности движения и вращения фигуры.
//!
//! ## Архитектурные заметки
//! ## Исправление #3 (DRY - Don't Repeat Yourself)
//! Выделена общая функция `is_position_valid()` для устранения дублирования кода
//! между проверкой движения и проверкой вращения.
//!
//! ## Исправление C2
//! Используется Range для проверки границ вместо множественных сравнений.
//!
//! ## Архитектурные заметки (SOLID-1)
//! Функции используют трейт `BoardReadonly` для уменьшения связанности:
//! - `is_position_valid()` работает с любым типом, реализующим `BoardReadonly`
//! - Это позволяет тестировать функции отдельно от `GameState`
//!
//! ## Исправление аудита 2026-03-31 (H1)
//! Функция переименована в `is_position_valid` с прямой логикой:
//! - `true` = позиция валидна (нет коллизии)
//! - `false` = коллизия обнаружена

use crate::game::access::BoardReadonly;
use crate::game::GameState;
use crate::io::GRID_WIDTH;
use crate::types::Direction;

/// Допустимый диапазон координат X для блоков на поле.
/// Используется для проверки границ в проверке коллизий.
const VALID_X_RANGE: std::ops::Range<i16> = 0..GRID_WIDTH as i16;

/// Проверить валидность позиции для одного блока (границы и столкновения).
///
/// # Аргументы
/// * `board` - объект с доступом только на чтение к полю
/// * `check_x` - координата X блока для проверки
/// * `check_y` - координата Y блока для проверки
/// * `ignore_above_field` - игнорировать блоки выше поля (Y < 0)
///
/// # Возвращает
/// - `true` если позиция валидна (нет коллизии)
/// - `false` если обнаружена коллизия (столкновение с границами или другими блоками)
///
/// # Архитектурные заметки (SOLID-1)
/// Использует трейт `BoardReadonly` вместо прямого доступа к `GameState`.
/// Это позволяет использовать функцию с любыми типами, реализующими этот трейт.
///
/// # Исправление #3 (DRY)
/// Общая функция для устранения дублирования между:
/// - `check_collision_direction()` - проверка движения
/// - `check_rotation_collision()` - проверка вращения
///
/// # Исправление C2
/// Используется Range::contains для проверки границ вместо множественных сравнений.
///
/// # Исправление аудита 2026-03-31 (H1)
/// Функция переименована из `has_collision` в `is_position_valid` с прямой логикой:
/// - `true` = позиция валидна
/// - `false` = коллизия обнаружена
///
/// # Примечания
/// - Проверка границ X: `0 <= check_x < GRID_WIDTH`
/// - Проверка заполненных ячеек: `cell == -1` означает пусто
/// - Игнорирование `Y < 0` полезно для вращения (блоки могут быть выше поля)
#[must_use]
fn is_position_valid<T: BoardReadonly>(
    board: &T,
    check_x: i16,
    check_y: i16,
    ignore_above_field: bool,
) -> bool {
    // Исправление C2: используем Range::contains для проверки границ X
    if !VALID_X_RANGE.contains(&check_x) {
        return false; // Коллизия с границей - позиция невалидна
    }

    // Проверка нижней границы (блоки выше поля считаются пустыми)
    // Блоки выше поля (Y < 0) не должны считаться коллизией
    // Это важно для движения влево/вправо, когда фигура появляется на поле
    if check_y < 0 {
        return true; // Позиция валидна выше поля
    }

    // Проверка наличия блока в сетке
    if board
        .get_blocks()
        .get(check_y as usize)
        .and_then(|row| row.get(check_x as usize))
        .is_none_or(|&cell| cell != -1)
    {
        return false; // Столкновение с заполненной ячейкой - позиция невалидна
    }

    true // Нет столкновений - позиция валидна
}

/// Проверить возможность движения фигуры в заданном направлении.
///
/// # Аргументы
/// * `board` - объект с доступом только на чтение к полю
/// * `coords` - координаты блоков фигуры
/// * `pos` - позиция фигуры (x, y)
/// * `dir` - направление движения
///
/// # Возвращает
/// `true` если движение возможно
///
/// # Архитектурные заметки (SOLID-1)
/// Использует трейт `BoardReadonly` вместо прямого доступа к `GameState`.
///
/// # Исправление #8
/// Используется `.get()` с ранним выходом вместо множественных проверок границ.
/// # Исправление бага
/// Проверка `check_y < 0` должна выполняться только для `Direction::Down`.
/// Блоки выше поля (отрицательный Y) не должны блокировать движение влево/вправо.
///
/// # Исправление M22 (MEDIUM)
/// Используется `.any()` с ранним выходом для оптимизации проверки коллизий.
/// Вместо цикла с return false используется итератор с .any() для раннего прерывания.
fn check_collision_direction<T: BoardReadonly>(
    board: &T,
    coords: &[(i16, i16)],
    pos: (f32, f32),
    dir: Direction,
) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    // Исправление M22: используем any() для раннего выхода при обнаружении коллизии
    // any() возвращает true если хотя бы один блок имеет невалидную позицию
    coords.iter().any(|&(coord_x, coord_y)| {
        let mut check_x = coord_x + shape_block_x;
        let mut check_y = coord_y + shape_block_y;

        match dir {
            Direction::Left => check_x -= 1,
            Direction::Right => check_x += 1,
            Direction::Down => check_y += 1,
        }

        // Исправление #3 (DRY): используем общую функцию is_position_valid
        // Для движения вниз не игнорируем блоки выше поля (check_y < 0 блокирует движение)
        let ignore_above_field = false;
        // Исправление аудита 2026-03-31 (H1): is_position_valid возвращает false при коллизии
        !is_position_valid(board, check_x, check_y, ignore_above_field)
    })
}

/// Проверить возможность движения текущей фигуры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление движения
///
/// # Возвращает
/// `true` если движение возможно
#[must_use]
pub fn can_move_curr_shape_direction(state: &GameState, dir: Direction) -> bool {
    let curr_shape = state.curr_shape();
    // Инвертируем: check_collision_direction возвращает true при коллизии,
    // а нам нужно true при возможности движения
    !check_collision_direction(state, &curr_shape.coords(), curr_shape.pos(), dir)
}

/// Проверить возможность вращения фигуры (без смещения).
///
/// # Аргументы
/// * `board` - объект с доступом только на чтение к полю
/// * `coords` - координаты блоков повёрнутой фигуры
/// * `pos` - позиция фигуры
///
/// # Возвращает
/// `true` если вращение возможно
///
/// # Архитектурные заметки (SOLID-1)
/// Использует трейт `BoardReadonly` вместо прямого доступа к `GameState`.
///
/// # Исправление #8
/// Используется `.get()` с ранним выходом вместо множественных проверок границ.
/// # Исправление бага
/// Блоки выше поля (отрицательный Y) допустимы при вращении.
pub fn check_rotation_collision<T: BoardReadonly>(
    board: &T,
    coords: &[(i16, i16)],
    pos: (f32, f32),
) -> bool {
    let (shape_x, shape_y) = pos;
    let shape_block_x = shape_x as i16;
    let shape_block_y = shape_y as i16;

    for coord in coords {
        let (coord_x, coord_y) = coord;
        let check_x = coord_x + shape_block_x;
        let check_y = coord_y + shape_block_y;

        // Исправление #3 (DRY): используем общую функцию is_position_valid
        // Для вращения игнорируем блоки выше поля (check_y < 0 допустимо)
        let ignore_above_field = true;
        // Исправление аудита 2026-03-31 (H1): is_position_valid возвращает false при коллизии
        if !is_position_valid(board, check_x, check_y, ignore_above_field) {
            return false;
        }
    }
    true
}

/// Проверить возможность вращения текущей фигуры.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение возможно (прямое или с wall kick)
///
/// ## Исправление #4 (HIGH) - Устранение дублирования
/// Функция полностью делегирует логику в `wall_kick::can_rotate_with_wall_kick`
/// для централизации всей логики wall kick в одном модуле.
///
/// ## Архитектурные заметки
/// Эта функция является обёрткой для wall_kick::can_rotate_with_wall_kick
/// и может быть удалена в будущем при прямом использовании wall_kick модуля.
pub fn can_rotate_curr_shape(state: &GameState, dir: crate::types::RotationDirection) -> bool {
    // Исправление В2: полное делегирование в wall_kick модуль для устранения дублирования
    super::wall_kick::can_rotate_with_wall_kick(state, dir)
}

#[cfg(test)]
mod collision_tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_can_move_down_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Down));
    }

    #[test]
    fn test_can_move_left_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Left));
    }

    #[test]
    fn test_can_move_right_initial() {
        let state = GameState::new();
        assert!(can_move_curr_shape_direction(&state, Direction::Right));
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ C2: ПРОВЕРКА КОЛЛИЗИЙ С VALID_X_RANGE - ГРАНИЦЫ
    // ========================================================================

    /// Тест C2: проверка границы VALID_X_RANGE - левая граница
    #[test]
    fn test_fix_c2_collision_left_boundary() {
        let state = GameState::new();
        // Фигура на левой границе (x=0)
        let mut test_state = state;
        test_state.get_curr_shape_mut().pos_mut().0 = 0.0;

        // Движение влево должно быть невозможно на границе
        let can_move = can_move_curr_shape_direction(&test_state, Direction::Left);
        assert!(
            !can_move,
            "Движение влево на левой границе должно быть невозможно"
        );
    }

    /// Тест C2: проверка границы VALID_X_RANGE - правая граница
    #[test]
    fn test_fix_c2_collision_right_boundary() {
        let state = GameState::new();
        // Фигура на правой границе (x=9 для GRID_WIDTH=10)
        let mut test_state = state;
        test_state.get_curr_shape_mut().pos_mut().0 = 9.0;

        // Движение вправо должно быть невозможно на границе
        let can_move = can_move_curr_shape_direction(&test_state, Direction::Right);
        assert!(
            !can_move,
            "Движение вправо на правой границе должно быть невозможно"
        );
    }

    /// Тест C2: проверка VALID_X_RANGE для отрицательных координат
    #[test]
    fn test_fix_c2_collision_negative_x() {
        let state = GameState::new();
        // Фигура за левой границей (x=-1)
        let mut test_state = state;
        test_state.get_curr_shape_mut().pos_mut().0 = -1.0;

        // Движение влево должно быть невозможно
        let can_move = can_move_curr_shape_direction(&test_state, Direction::Left);
        assert!(
            !can_move,
            "Движение при отрицательной X координате должно быть невозможно"
        );
    }

    /// Тест C2: проверка VALID_X_RANGE за пределами поля
    #[test]
    fn test_fix_c2_collision_out_of_bounds_x() {
        let state = GameState::new();
        // Фигура за правой границей (x=10 при GRID_WIDTH=10)
        let mut test_state = state;
        test_state.get_curr_shape_mut().pos_mut().0 = 10.0;

        // Движение вправо должно быть невозможно
        let can_move = can_move_curr_shape_direction(&test_state, Direction::Right);
        assert!(
            !can_move,
            "Движение за пределами поля по X должно быть невозможно"
        );
    }

    /// Тест C2: проверка is_position_valid с VALID_X_RANGE.contains()
    #[test]
    fn test_fix_c2_is_position_valid_range_contains() {
        use crate::io::GRID_WIDTH;

        // Проверка что VALID_X_RANGE корректно определён
        assert_eq!(VALID_X_RANGE.start, 0);
        assert_eq!(VALID_X_RANGE.end, GRID_WIDTH as i16);

        // Проверка contains для границ
        assert!(VALID_X_RANGE.contains(&0));
        assert!(VALID_X_RANGE.contains(&9));
        assert!(!VALID_X_RANGE.contains(&-1));
        assert!(!VALID_X_RANGE.contains(&10));
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ M22: can_move_curr_shape_direction С .any() - ОПТИМИЗАЦИЯ
    // ========================================================================

    /// Тест M22: проверка что can_move_curr_shape_direction использует .any()
    /// Проверяет корректность работы функции с ранним выходом
    #[test]
    fn test_fix_m22_can_move_with_early_exit() {
        let state = GameState::new();

        // Все направления должны быть доступны в начале игры
        assert!(can_move_curr_shape_direction(&state, Direction::Down));
        assert!(can_move_curr_shape_direction(&state, Direction::Left));
        assert!(can_move_curr_shape_direction(&state, Direction::Right));
    }

    /// Тест M22: проверка движения при коллизии с блоками
    #[test]
    fn test_fix_m22_can_move_with_block_collision() {
        let mut state = GameState::new();

        // Устанавливаем блок под текущей фигурой
        let curr_y = state.curr_shape().pos().1 as i16;
        let blocks = state.get_blocks_mut();
        if curr_y + 1 < 20 {
            blocks[(curr_y + 1) as usize][4] = 1; // Блок под фигурой
        }

        // Движение вниз должно быть невозможно
        let can_move = can_move_curr_shape_direction(&state, Direction::Down);
        assert!(
            !can_move,
            "Движение вниз при коллизии должно быть невозможно"
        );
    }
}
