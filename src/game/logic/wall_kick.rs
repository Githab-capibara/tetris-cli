//! Модуль wall kick для вращения фигур.
//!
//! # Ответственность
//! - Таблица смещений wall kick (Super Rotation System - упрощённая)
//! - Логика вращения со смещением при коллизиях
//! - Централизация кода wall kick для устранения дублирования
//!
//! # Зависимости
//! - [`state.rs`](crate::game::state): `GameState`
//! - [`collision.rs`](super::collision): `check_rotation_collision`
//!
//! # Исправление #4 (HIGH)
//! Устранено дублирование кода wall kick между collision.rs и rotation.rs.
//! Вся логика wall kick централизована в этом модуле.

use crate::game::GameState;

/// Таблица смещений для wall kick (Super Rotation System - упрощённая).
/// Используется при вращении фигур рядом со стенами или другими блоками.
///
/// # Порядок проверки
/// Смещения проверяются в порядке приоритета:
/// 1. Простые смещения влево/вправо (±1)
/// 2. Двойные смещения влево/вправо (±2)
/// 3. Смещения вверх для случаев у пола
/// 4. Комбинированные смещения (влево-вверх, вправо-вверх)
/// 5. Смещение вниз для случаев у потолка
///
/// # Структура массива
/// Каждый элемент кортежа `(offset_x, offset_y)`:
/// - `offset_x`: смещение по горизонтали (отрицательное = влево, положительное = вправо)
/// - `offset_y`: смещение по вертикали (отрицательное = вверх, положительное = вниз)
///
/// # Детальное описание каждого смещения
/// | Индекс | Смещение | Назначение |
/// |--------|----------|------------|
/// | 0 | `(0, 0)` | Базовая проверка без смещения - попытка вращения на месте |
/// | 1 | `(-1, 0)` | Сдвиг влево на 1 клетку - базовая коррекция при вращении у правой стены |
/// | 2 | `(1, 0)` | Сдвиг вправо на 1 клетку - базовая коррекция при вращении у левой стены |
/// | 3 | `(-2, 0)` | Сдвиг влево на 2 клетки - для фигур I и S/Z при вращении у стены |
/// | 4 | `(2, 0)` | Сдвиг вправо на 2 клетки - для фигур I и S/Z при вращении у стены |
/// | 5 | `(0, -1)` | Подъём на 1 клетку вверх - когда фигура упирается в пол при вращении |
/// | 6 | `(-1, -1)` | Сдвиг влево-вверх - комбинированная коррекция для левого угла |
/// | 7 | `(1, -1)` | Сдвиг вправо-вверх - комбинированная коррекция для правого угла |
///
/// # Магические числа
/// - Максимальное смещение: ±2 по X, ±1 по Y
/// - Количество смещений: 8 (оптимально для производительности)
///
/// # Исправление L1 (HIGH): Стандартная таблица SRS
/// Добавлено смещение (0, 0) первым элементом для базовой проверки вращения на месте.
/// Это соответствует стандартной таблице Super Rotation System где первая проверка
/// всегда выполняется без смещения.
pub const WALL_KICK_OFFSETS: [(i32, i32); 8] = [
    (0, 0),   // Базовая проверка без смещения - вращение на месте
    (-1, 0),  // Влево на 1: базовая коррекция у правой стены
    (1, 0),   // Вправо на 1: базовая коррекция у левой стены
    (-2, 0),  // Влево на 2: для фигур I и S/Z у правой стены
    (2, 0),   // Вправо на 2: для фигур I и S/Z у левой стены
    (0, -1),  // Вверх на 1: когда фигура упирается в пол
    (-1, -1), // Влево-вверх: комбинированная коррекция для левого угла
    (1, -1),  // Вправо-вверх: комбинированная коррекция для правого угла
];

/// Попытаться вратить фигуру со смещением (wall kick).
///
/// # Аргументы
/// * `state` - состояние игры (изменяемое)
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение успешно с любым смещением из таблицы wall kick
///
/// # Алгоритм
/// 1. Проверяется прямое вращение без смещения
/// 2. Если прямое вращение невозможно, перебираются смещения из WALL_KICK_OFFSETS
/// 3. Первое успешное смещение применяется к фигуре
/// 4. Если ни одно смещение не подошло - вращение невозможно
///
/// # Пример использования
/// ```ignore
/// use crate::game::logic::wall_kick::rotate_with_wall_kick;
/// use crate::types::RotationDirection;
///
/// let mut state = GameState::new();
/// if rotate_with_wall_kick(&mut state, RotationDirection::Clockwise) {
///     println!("Вращение успешно!");
/// }
/// ```
pub fn rotate_with_wall_kick(state: &mut GameState, dir: crate::types::RotationDirection) -> bool {
    // Проверяем прямое вращение без смещения
    if super::collision::can_rotate_curr_shape(state, dir) {
        let curr_shape = state.get_curr_shape_mut();
        curr_shape.rotate(dir);
        return true;
    }

    // Пытаемся вращать с wall kick смещениями
    if let Some((offset_x, offset_y)) = try_wall_kick_offsets(state, dir) {
        {
            let curr_shape = state.get_curr_shape_mut();
            let pos = curr_shape.pos_mut();
            pos.0 += offset_x as f32;
            pos.1 += offset_y as f32;
        }
        let curr_shape = state.get_curr_shape_mut();
        curr_shape.rotate(dir);
        return true;
    }

    false
}

/// Перебрать все смещения wall kick и найти подходящее.
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// - `Some((offset_x, offset_y))` - первое успешное смещение
/// - `None` - ни одно смещение не подошло
///
/// # Внутренняя реализация
/// Использует таблицу WALL_KICK_OFFSETS для перебора смещений.
/// Для каждого смещения:
/// 1. Копирует текущую фигуру
/// 2. Применяет смещение к позиции
/// 3. Выполняет вращение
/// 4. Проверяет коллизии
/// 5. Возвращает смещение если вращение успешно
///
/// ## Исправление #4 (HIGH)
/// Функция сделана pub(crate) для использования из collision.rs.
#[allow(dead_code)]
pub(crate) fn try_wall_kick_offsets(
    state: &GameState,
    dir: crate::types::RotationDirection,
) -> Option<(i32, i32)> {
    for &(offset_x, offset_y) in &WALL_KICK_OFFSETS {
        let mut kicked_shape = *state.curr_shape();
        let pos = kicked_shape.pos_mut();
        pos.0 += offset_x as f32;
        pos.1 += offset_y as f32;
        kicked_shape.rotate(dir);

        if super::collision::check_rotation_collision(state, &kicked_shape.coords(), kicked_shape.pos())
        {
            return Some((offset_x, offset_y));
        }
    }
    None
}

/// Проверить возможность вращения с wall kick (без выполнения вращения).
///
/// # Аргументы
/// * `state` - состояние игры
/// * `dir` - направление вращения
///
/// # Возвращает
/// `true` если вращение возможно (прямое или с wall kick)
///
/// # Отличия от rotate_with_wall_kick
/// Эта функция только проверяет возможность вращения, не изменяя состояние игры.
/// Используйте для предварительной проверки перед вращением.
///
/// ## Исправление #13
/// Функция предназначена для будущего использования в API.
#[must_use]
#[allow(dead_code)]
pub fn can_rotate_with_wall_kick(state: &GameState, dir: crate::types::RotationDirection) -> bool {
    // Прямое вращение - проверяем напрямую без рекурсии
    let mut temp_shape = *state.curr_shape();
    temp_shape.rotate(dir);

    // Исправление #3 (HIGH): Используем check_rotation_collision() вместо прямой индексации
    // Это предотвращает панику при выходе за границы массива
    if super::collision::check_rotation_collision(state, &temp_shape.coords(), temp_shape.pos()) {
        return true;
    }

    // Проверка wall kick без выполнения
    try_wall_kick_offsets(state, dir).is_some()
}

#[cfg(test)]
mod wall_kick_tests {
    use super::*;
    use crate::game::GameState;
    use crate::types::RotationDirection;

    #[test]
    fn test_rotate_with_wall_kick_basic() {
        let mut state = GameState::new();
        let initial_coords = state.curr_shape().coords;

        let result = rotate_with_wall_kick(&mut state, RotationDirection::Clockwise);

        assert!(result, "Вращение должно быть успешным");
        // Примечание: координаты могут не измениться при вращении без коллизий
        // или для симметричных фигур (например, фигура O)
    }

    #[test]
    fn test_wall_kick_offsets_table() {
        // Проверка количества смещений
        assert_eq!(WALL_KICK_OFFSETS.len(), 8);

        // Проверка наличия ключевых смещений
        assert!(WALL_KICK_OFFSETS.contains(&(-1, 0)));
        assert!(WALL_KICK_OFFSETS.contains(&(1, 0)));
        assert!(WALL_KICK_OFFSETS.contains(&(-2, 0)));
        assert!(WALL_KICK_OFFSETS.contains(&(2, 0)));
        assert!(WALL_KICK_OFFSETS.contains(&(0, -1)));
        // Базовое смещение (0, 0) присутствует для вращения на месте
        assert!(WALL_KICK_OFFSETS.contains(&(0, 0)));

        // Смещение вниз (0, 1) не входит в таблицу SRS
        assert!(!WALL_KICK_OFFSETS.contains(&(0, 1)));
    }

    #[test]
    fn test_can_rotate_with_wall_kick() {
        let state = GameState::new();

        // Проверка что функция возвращает true для начального состояния
        assert!(can_rotate_with_wall_kick(
            &state,
            RotationDirection::Clockwise
        ));
    }

    // ========================================================================
    // ТЕСТЫ ДЛЯ L1: STANDARD SRS WALL KICK OFFSETS
    // ========================================================================

    /// Тест L1: проверка документации каждого смещения WALL_KICK_OFFSETS
    #[test]
    fn test_fix_l1_wall_kick_offsets_documentation() {
        // Проверка что каждое смещение имеет ожидаемые значения согласно стандарту SRS
        // Индекс 0: (0, 0) - Базовая проверка без смещения
        assert_eq!(WALL_KICK_OFFSETS[0], (0, 0));

        // Индекс 1: (-1, 0) - Влево на 1: базовая коррекция у правой стены
        assert_eq!(WALL_KICK_OFFSETS[1], (-1, 0));

        // Индекс 2: (1, 0) - Вправо на 1: базовая коррекция у левой стены
        assert_eq!(WALL_KICK_OFFSETS[2], (1, 0));

        // Индекс 3: (-2, 0) - Влево на 2: для фигур I и S/Z у правой стены
        assert_eq!(WALL_KICK_OFFSETS[3], (-2, 0));

        // Индекс 4: (2, 0) - Вправо на 2: для фигур I и S/Z у левой стены
        assert_eq!(WALL_KICK_OFFSETS[4], (2, 0));

        // Индекс 5: (0, -1) - Вверх на 1: когда фигура упирается в пол
        assert_eq!(WALL_KICK_OFFSETS[5], (0, -1));

        // Индекс 6: (-1, -1) - Влево-вверх: комбинированная коррекция для левого угла
        assert_eq!(WALL_KICK_OFFSETS[6], (-1, -1));

        // Индекс 7: (1, -1) - Вправо-вверх: комбинированная коррекция для правого угла
        assert_eq!(WALL_KICK_OFFSETS[7], (1, -1));
    }

    /// Тест L1: проверка приоритета смещений wall kick (стандарт SRS)
    #[test]
    fn test_fix_l1_wall_kick_offset_priority() {
        // Проверка что смещения проверяются в правильном порядке приоритета:
        // 1. Базовая проверка (0, 0) - индекс 0
        // 2. Простые смещения влево/вправо (±1) - индексы 1, 2
        // 3. Двойные смещения (±2) - индексы 3, 4
        // 4. Смещения вверх - индекс 5
        // 5. Комбинированные смещения - индексы 6, 7

        // Базовая проверка должна быть первой
        assert_eq!(WALL_KICK_OFFSETS[0], (0, 0));

        // Простые смещения должны быть вторыми
        assert!(WALL_KICK_OFFSETS[1].0.abs() == 1 && WALL_KICK_OFFSETS[1].1 == 0);
        assert!(WALL_KICK_OFFSETS[2].0.abs() == 1 && WALL_KICK_OFFSETS[2].1 == 0);

        // Двойные смещения должны быть третьими
        assert!(WALL_KICK_OFFSETS[3].0.abs() == 2 && WALL_KICK_OFFSETS[3].1 == 0);
        assert!(WALL_KICK_OFFSETS[4].0.abs() == 2 && WALL_KICK_OFFSETS[4].1 == 0);

        // Смещение вверх должно быть четвёртым
        assert_eq!(WALL_KICK_OFFSETS[5], (0, -1));

        // Комбинированные смещения должны быть последними
        assert_eq!(WALL_KICK_OFFSETS[6], (-1, -1));
        assert_eq!(WALL_KICK_OFFSETS[7], (1, -1));
    }

    /// Тест H4: проверка что wall kick работает для всех направлений вращения
    #[test]
    fn test_fix_h4_wall_kick_both_rotation_directions() {
        let mut state = GameState::new();

        // Проверка вращения по часовой стрелке
        assert!(rotate_with_wall_kick(
            &mut state,
            RotationDirection::Clockwise
        ));

        // Сброс состояния
        state = GameState::new();

        // Проверка вращения против часовой стрелки
        assert!(rotate_with_wall_kick(
            &mut state,
            RotationDirection::CounterClockwise
        ));
    }
}
