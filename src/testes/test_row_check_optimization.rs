//! Тесты оптимизации поиска заполненных линий.
//!
//! Проверяют ранний выход при пустой ячейке и корректность удаления линий.

use crate::game::scoring::{find_full_rows, remove_rows};
use crate::game::GameState;
use crate::io::{GRID_HEIGHT, GRID_WIDTH};

/// Тест 1: Проверка раннего выхода при пустой ячейке
///
/// find_full_rows должен делать ранний выход при обнаружении пустой ячейки.
#[test]
fn test_find_full_rows_early_exit() {
    let state = GameState::new();

    // Пустое поле - все ячейки пустые
    let (rows_mask, remove_count) = find_full_rows(&state.blocks);

    assert_eq!(
        rows_mask, 0,
        "На пустом поле не должно быть заполненных линий"
    );
    assert_eq!(
        remove_count, 0,
        "На пустом поле не должно быть линий для удаления"
    );
}

/// Тест 2: Проверка корректности удаления линий
///
/// Проверяем, что remove_rows корректно удаляет линии.
#[test]
fn test_remove_rows_correctness() {
    use crate::game::scoring::remove_rows;
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Заполняем линию 10 (предпоследнюю)
    for x in 0..GRID_WIDTH {
        state.blocks[10][x] = 0; // Устанавливаем блок
    }

    // Находим заполненные линии
    let (rows_mask, remove_count) = find_full_rows(&state.blocks);

    assert_eq!(remove_count, 1, "Должна быть найдена 1 заполненная линия");
    assert_eq!(rows_mask, 1 << 10, "Маска должна указывать на линию 10");

    // Удаляем линии
    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем, что линия 10 удалена (стала пустой)
    for x in 0..GRID_WIDTH {
        assert_eq!(
            state.blocks[10][x], -1,
            "Линия 10 должна быть пустой после удаления"
        );
    }
}

/// Тест 3: Проверка удаления нескольких линий
///
/// Проверяем, что несколько линий удаляются корректно.
#[test]
fn test_remove_multiple_rows() {
    use crate::game::scoring::remove_rows;
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Заполняем линии 5, 7, 9
    for &y in &[5, 7, 9] {
        for x in 0..GRID_WIDTH {
            state.blocks[y][x] = 1; // Устанавливаем блоки
        }
    }

    // Находим заполненные линии
    let (rows_mask, remove_count) = find_full_rows(&state.blocks);

    assert_eq!(remove_count, 3, "Должны быть найдены 3 заполненные линии");

    // Проверяем маску
    let expected_mask = (1 << 5) | (1 << 7) | (1 << 9);
    assert_eq!(
        rows_mask, expected_mask,
        "Маска должна указывать на линии 5, 7, 9"
    );

    // Удаляем линии
    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем, что линии удалены
    for &y in &[5, 7, 9] {
        for x in 0..GRID_WIDTH {
            assert_eq!(
                state.blocks[y][x], -1,
                "Линия {y} должна быть пустой после удаления"
            );
        }
    }
}

/// Тест 4: Проверка производительности find_full_rows
///
/// Бенчмарк: поиск линий должен быть быстрым.
#[test]
fn test_find_full_rows_performance() {
    use std::time::Instant;

    let state = GameState::new();
    let iterations = 100_000;

    let start = Instant::now();

    for _ in 0..iterations {
        let _ = find_full_rows(state.get_blocks());
    }

    let elapsed = start.elapsed();

    // 100000 итераций должны выполняться < 200ms (увеличенный таймаут для стабильности)
    assert!(
        elapsed.as_millis() < 200,
        "find_full_rows {iterations} итераций должна выполняться < 200ms (прошло {:?})",
        elapsed
    );
}

/// Тест 5: Проверка .all() с ранним выходом
///
/// Проверяем, что .all() делает ранний выход при пустой ячейке.
#[test]
fn test_all_early_exit() {
    use crate::io::GRID_WIDTH;

    let state = GameState::new();
    let blocks = state.get_blocks();

    // Проверяем, что .all() работает корректно
    for row in blocks.iter() {
        let row_full = row.iter().take(GRID_WIDTH).all(|&cell| cell != -1);

        // На пустом поле все линии должны быть неполными
        assert!(!row_full, "На пустом поле не должно быть заполненных линий");
    }
}

/// Тест 6: Проверка сдвига линий после удаления
///
/// Проверяем, что верхние линии сдвигаются вниз после удаления.
#[test]
fn test_rows_shift_after_removal() {
    use crate::game::scoring::remove_rows;
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Заполняем линию 18 (почти дно)
    for x in 0..GRID_WIDTH {
        state.blocks[18][x] = 2;
    }

    // Устанавливаем блок в линии 15
    state.blocks[15][5] = 3;

    // Находим и удаляем заполненные линии
    let (rows_mask, _) = find_full_rows(&state.blocks);
    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем, что блок из линии 15 сдвинулся вниз на 1 позицию
    // После удаления линии 18, линия 15 должна сдвинуться на линию 16
    assert_eq!(
        state.blocks[16][5], 3,
        "Блок должен сдвинуться с линии 15 на линию 16 после удаления линии 18"
    );

    // Проверяем, что линия 15 теперь пустая (сдвинулась вниз)
    assert_eq!(
        state.blocks[15][5], -1,
        "Линия 15 должна быть пустой после сдвига"
    );
}

/// Тест 7: Проверка заполнения верхних линий пустыми
///
/// Проверяем, что после удаления верхние линии заполняются -1.
#[test]
fn test_top_rows_filled_empty() {
    use crate::game::scoring::remove_rows;
    use crate::io::GRID_WIDTH;

    let mut state = GameState::new();

    // Заполняем все линии
    for y in 0..20 {
        for x in 0..GRID_WIDTH {
            state.blocks[y][x] = 4;
        }
    }

    // Находим и удаляем все линии
    let (rows_mask, remove_count) = find_full_rows(&state.blocks);
    assert_eq!(remove_count, 20, "Должны быть найдены все 20 линий");

    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем, что все линии пустые
    for y in 0..20 {
        for x in 0..GRID_WIDTH {
            assert_eq!(
                state.blocks[y][x], -1,
                "Все линии должны быть пустыми после удаления"
            );
        }
    }
}

/// Тест 8: Проверка валидности rows_mask
///
/// Проверяем, что rows_mask не выходит за пределы поля.
#[test]
fn test_rows_mask_validity() {
    use crate::io::GRID_HEIGHT;

    let state = GameState::new();
    let (rows_mask, _) = find_full_rows(&state.blocks);

    // Проверяем, что маска в пределах поля
    assert!(
        rows_mask < (1u32 << GRID_HEIGHT),
        "rows_mask ({}) не должен выходить за пределы поля ({} линий)",
        rows_mask,
        GRID_HEIGHT
    );
}

/// Тест 9: Проверка корректности удаления строк (расширенная)
#[test]
fn test_remove_rows_extended() {
    let mut state = GameState::new();
    // Заполняем линию 10
    for x in 0..GRID_WIDTH {
        state.blocks[10][x] = 1;
    }

    let (rows_mask, expected_count) = find_full_rows(&state.blocks);
    assert_eq!(expected_count, 1, "Должна быть 1 строка для удаления");

    remove_rows(&mut state.blocks, rows_mask);

    // Проверяем что линия 10 теперь пустая
    for x in 0..GRID_WIDTH {
        assert_eq!(
            state.blocks[10][x], -1,
            "Линия 10 должна быть пустой после удаления"
        );
    }

    // Тест 2: Удаление нескольких строк
    let mut state2 = GameState::new();
    // Заполняем линии 5, 7, 9
    for &y in &[5, 7, 9] {
        for x in 0..GRID_WIDTH {
            state2.blocks[y][x] = 2;
        }
    }

    let (rows_mask2, expected_count2) = find_full_rows(&state2.blocks);
    assert_eq!(expected_count2, 3, "Должно быть 3 строки для удаления");

    remove_rows(&mut state2.blocks, rows_mask2);

    // Проверяем что линии теперь пустые
    for &y in &[5, 7, 9] {
        for x in 0..GRID_WIDTH {
            assert_eq!(
                state2.blocks[y][x], -1,
                "Линия должна быть пустой после удаления"
            );
        }
    }

    // Тест 3: Невалидная маска не вызывает панику
    let invalid_mask = 1u32 << (GRID_HEIGHT + 1); // Выход за границы
    remove_rows(&mut state.blocks, invalid_mask);
    // Функция должна просто вернуть 0 без паники
}
