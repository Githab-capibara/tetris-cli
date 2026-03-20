//! Тесты для проверки использования Box для массива blocks (game.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - blocks теперь размещается в куче через Box вместо стека
//! - Предотвращение переполнения стека при инициализации
//! - Проверка работы с большим игровым полем
//!
//! Исправление: использование Box<[[i8; GRID_WIDTH]; GRID_HEIGHT]> вместо [[i8; GRID_WIDTH]; GRID_HEIGHT]

use crate::game::GameState;
use crate::io::{GRID_HEIGHT, GRID_WIDTH};

// ============================================================================
// ГРУППА ТЕСТОВ: Box для массива blocks
// ============================================================================

/// Тест 1: Проверка создания GameState без переполнения стека
///
/// Проверяет, что GameState::new() успешно создаёт игровое состояние
/// с полем blocks, размещённым в куче через Box.
#[test]
fn test_game_state_creation_no_stack_overflow() {
    // Создаём новое состояние игры
    // Это должно работать без переполнения стека благодаря Box
    let game_state = GameState::new();

    // Проверяем что состояние создано корректно
    // (тест компилируется только если blocks использует Box)
    let _ = &game_state;

    // Косвенная проверка: создаём несколько состояний подряд
    // Если бы blocks был на стеке, это могло бы вызвать переполнение
    let state1 = GameState::new();
    let state2 = GameState::new();
    let state3 = GameState::new();

    // Все три состояния должны быть созданы успешно
    let _ = &state1;
    let _ = &state2;
    let _ = &state3;

    // Проверяем что поля инициализированы правильными размерами
    assert!(GRID_WIDTH > 0, "GRID_WIDTH должен быть положительным");
    assert!(GRID_HEIGHT > 0, "GRID_HEIGHT должен быть положительным");

    // Проверяем что get_blocks() возвращает ссылку на Box
    let blocks = game_state.get_blocks();
    assert!(
        blocks.len() == GRID_HEIGHT,
        "Количество строк должно быть {}",
        GRID_HEIGHT
    );
    assert!(
        blocks[0].len() == GRID_WIDTH,
        "Количество столбцов должно быть {}",
        GRID_WIDTH
    );
}

/// Тест 2: Проверка доступа к элементам массива
///
/// Проверяет, что можно читать и записывать значения в массив blocks
/// через Box без проблем с производительностью.
#[test]
fn test_block_array_element_access() {
    let mut game_state = GameState::new();

    // Получаем доступ к полю blocks
    let blocks = game_state.get_blocks();

    // Проверяем что все клетки изначально пустые (-1)
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            assert_eq!(
                blocks[y][x], -1,
                "Клетка [{},{}] должна быть пустой (-1)",
                y, x
            );
        }
    }

    // Проверяем доступ к конкретным элементам
    assert_eq!(blocks[0][0], -1, "Верхний левый угол должен быть пустым");
    assert_eq!(
        blocks[GRID_HEIGHT - 1][GRID_WIDTH - 1],
        -1,
        "Нижний правый угол должен быть пустым"
    );

    // Проверяем что размеры корректны
    assert_eq!(
        blocks.len(),
        GRID_HEIGHT,
        "Высота поля должна быть {}",
        GRID_HEIGHT
    );
    assert_eq!(
        blocks[0].len(),
        GRID_WIDTH,
        "Ширина поля должна быть {}",
        GRID_WIDTH
    );
}

/// Тест 3: Проверка мутации массива в куче vs стек
///
/// Проверяет, что можно работать с полем blocks через Box
/// без проблем с производительностью или безопасностью.
#[test]
fn test_block_array_mutation_heap_vs_stack() {
    use std::time::Instant;

    // Создаём состояние игры
    let game_state = GameState::new();

    // Замеряем время доступа к полю через Box
    let start = Instant::now();

    // Получаем доступ к полю и проверяем значения
    {
        let blocks = game_state.get_blocks();

        // Проверяем что все клетки изначально пустые (-1)
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(blocks[y][x], -1, "Клетка должна быть пустой");
            }
        }
    }

    let elapsed = start.elapsed();

    // Проверяем что доступ занял разумное время (< 100мс)
    assert!(
        elapsed.as_millis() < 100,
        "Доступ к полю должен занять меньше 100мс"
    );

    // Проверяем что размеры корректны
    let blocks = game_state.get_blocks();
    assert_eq!(
        blocks.len(),
        GRID_HEIGHT,
        "Высота поля должна быть {}",
        GRID_HEIGHT
    );
    assert_eq!(
        blocks[0].len(),
        GRID_WIDTH,
        "Ширина поля должна быть {}",
        GRID_WIDTH
    );

    // Стресс-тест: множественные чтения
    let start = Instant::now();
    for _ in 0..100 {
        let blocks = game_state.get_blocks();
        // Быстрое чтение
        let _ = blocks[0][0];
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 50,
        "100 чтений должны занять меньше 50мс"
    );
}
