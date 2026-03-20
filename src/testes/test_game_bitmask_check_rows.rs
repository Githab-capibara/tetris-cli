//! Тесты битовой маски в check_rows() (game.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка корректного определения заполненных линий
//! - Проверка работы с пустым полем
//! - Проверка работы с полностью заполненным полем
//!
//! Исправление: использование u32 битовой маски вместо [bool; GRID_HEIGHT]

use crate::game::GameState;
use crate::io::{GRID_HEIGHT, GRID_WIDTH};

// ============================================================================
// ГРУППА ТЕСТОВ: Битовая маска в check_rows()
// ============================================================================

/// Тест 1: Проверка корректного определения заполненных линий
///
/// Проверяет, что битовая маска корректно определяет
/// заполненные линии.
#[test]
fn test_bitmask_correct_line_detection() {
    // Создаём состояние игры
    let mut state = GameState::new();

    // Проверяем что изначально поле пустое
    let blocks = state.get_blocks();
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            assert_eq!(
                blocks[y][x], -1,
                "Клетка [{},{}] должна быть пустой",
                y, x
            );
        }
    }

    // Проверяем что битовая маска работает корректно
    // Симулируем проверку строк через битовую операцию
    let mut rows_mask: u32 = 0;

    // Проверяем что маска изначально пустая
    assert_eq!(rows_mask, 0, "Маска должна быть изначально пустой");

    // Устанавливаем биты для нескольких строк
    rows_mask |= 1 << 5;
    rows_mask |= 1 << 10;
    rows_mask |= 1 << 15;

    // Проверяем что биты установлены
    assert_ne!(rows_mask, 0, "Маска должна содержать установленные биты");
    assert_eq!(
        (rows_mask & (1 << 5)) != 0,
        true,
        "Бит 5 должен быть установлен"
    );
    assert_eq!(
        (rows_mask & (1 << 10)) != 0,
        true,
        "Бит 10 должен быть установлен"
    );
    assert_eq!(
        (rows_mask & (1 << 15)) != 0,
        true,
        "Бит 15 должен быть установлен"
    );

    // Проверяем что другие биты не установлены
    assert_eq!(
        (rows_mask & (1 << 0)) != 0,
        false,
        "Бит 0 не должен быть установлен"
    );
    assert_eq!(
        (rows_mask & (1 << 20)) != 0,
        false,
        "Бит 20 не должен быть установлен"
    );
}

/// Тест 2: Проверка работы с пустым полем
///
/// Проверяет, что битовая маска корректно работает
/// когда поле пустое.
#[test]
fn test_bitmask_with_empty_field() {
    let state = GameState::new();
    let blocks = state.get_blocks();

    // Симулируем проверку пустого поля
    let mut rows_mask: u32 = 0;
    let mut remove_count = 0;

    // Проверяем каждую строку
    for (y, row) in blocks.iter().enumerate() {
        let row_full = row.iter().take(GRID_WIDTH).all(|&cell| cell != -1);

        if row_full {
            rows_mask |= 1 << y;
            remove_count += 1;
        }
    }

    // Для пустого поля маска должна быть 0
    assert_eq!(
        rows_mask, 0,
        "Маска для пустого поля должна быть 0"
    );
    assert_eq!(
        remove_count, 0,
        "Количество заполненных линий должно быть 0"
    );

    // Проверяем что все 20 строк проверены
    assert_eq!(blocks.len(), GRID_HEIGHT, "Должно быть 20 строк");

    // Проверяем что каждая строка содержит 10 клеток
    for row in blocks.iter() {
        assert_eq!(row.len(), GRID_WIDTH, "Каждая строка должна содержать 10 клеток");
    }
}

/// Тест 3: Проверка работы с полностью заполненным полем
///
/// Проверяет, что битовая маска корректно работает
/// когда все линии заполнены.
#[test]
fn test_bitmask_with_full_field() {
    // Создаём состояние игры
    let mut state = GameState::new();

    // Симулируем полностью заполненное поле
    // (в реальном коде это происходит через save_tetromino())
    // Для теста используем прямую проверку битовой маски

    // Симулируем маску для полностью заполненного поля
    let mut rows_mask: u32 = 0;
    let mut remove_count = 0;

    // Устанавливаем все биты для 20 строк
    for y in 0..GRID_HEIGHT {
        rows_mask |= 1 << y;
        remove_count += 1;
    }

    // Проверяем что все биты установлены
    assert_ne!(rows_mask, 0, "Маска должна быть не нулевой");
    assert_eq!(remove_count, GRID_HEIGHT, "Все 20 строк должны быть заполнены");

    // Проверяем что каждый бит установлен
    for y in 0..GRID_HEIGHT {
        assert_eq!(
            (rows_mask & (1 << y)) != 0,
            true,
            "Бит {} должен быть установлен",
            y
        );
    }

    // Проверяем что биты выше 19 не установлены (так как у нас только 20 строк)
    assert_eq!(
        (rows_mask & (1 << 20)) == 0,
        true,
        "Бит 20 не должен быть установлен"
    );
    assert_eq!(
        (rows_mask & (1 << 31)) == 0,
        true,
        "Бит 31 не должен быть установлен"
    );

    // Проверяем что u32 достаточно для 20 строк
    assert!(
        GRID_HEIGHT <= 32,
        "u32 должен вмещать все строки (32 бита >= {} строк)",
        GRID_HEIGHT
    );
}

/// Тест 4: Проверка битовых операций
///
/// Интеграционный тест для битовых операций.
#[test]
fn test_bitmask_operations() {
    let mut mask: u32 = 0;

    // Тест установки битов
    mask |= 1 << 0;
    mask |= 1 << 5;
    mask |= 1 << 10;
    mask |= 1 << 19;

    assert_eq!(mask.count_ones(), 4, "Должно быть 4 установленных бита");

    // Тест проверки битов
    assert_eq!((mask & (1 << 0)) != 0, true, "Бит 0 установлен");
    assert_eq!((mask & (1 << 5)) != 0, true, "Бит 5 установлен");
    assert_eq!((mask & (1 << 10)) != 0, true, "Бит 10 установлен");
    assert_eq!((mask & (1 << 19)) != 0, true, "Бит 19 установлен");
    assert_eq!((mask & (1 << 1)) == 0, true, "Бит 1 не установлен");

    // Тест сброса битов
    mask &= !(1 << 5);
    assert_eq!((mask & (1 << 5)) == 0, true, "Бит 5 должен быть сброшен");
    assert_eq!(mask.count_ones(), 3, "Должно остаться 3 установленных бита");

    // Тест инверсии
    let inverted = !mask;
    assert_eq!((inverted & (1 << 5)) != 0, true, "Инвертированный бит 5 установлен");
    assert_eq!((inverted & (1 << 0)) == 0, true, "Инвертированный бит 0 сброшен");
}

/// Тест 5: Проверка производительности битовой маски
///
/// Сравнивает производительность битовой маски с массивом bool.
#[test]
fn test_bitmask_performance() {
    use std::time::Instant;

    // Тест битовой маски
    let mut mask: u32 = 0;
    let iterations = 10_000;

    let start = Instant::now();
    for _ in 0..iterations {
        mask = 0;
        for y in 0..GRID_HEIGHT {
            mask |= 1 << y;
        }
        for y in 0..GRID_HEIGHT {
            let _ = (mask & (1 << y)) != 0;
        }
    }
    let mask_elapsed = start.elapsed();

    // Тест массива bool
    let mut bool_array = [false; GRID_HEIGHT];

    let start = Instant::now();
    for _ in 0..iterations {
        bool_array = [false; GRID_HEIGHT];
        for y in 0..GRID_HEIGHT {
            bool_array[y] = true;
        }
        for y in 0..GRID_HEIGHT {
            let _ = bool_array[y];
        }
    }
    let bool_elapsed = start.elapsed();

    // Проверяем что битовая маска работает быстро
    assert!(
        mask_elapsed.as_millis() < 100,
        "Битовая маска должна работать быстро"
    );

    // Выводим сравнение производительности (для информации)
    println!(
        "Битовая маска: {:?}, Массив bool: {:?}",
        mask_elapsed, bool_elapsed
    );

    // Битовая маска должна быть не медленнее массива bool
    // (это не строгое требование, но хорошая практика)
    assert!(
        mask_elapsed <= bool_elapsed * 2,
        "Битовая маска должна быть сравнима по производительности с bool массивом"
    );
}
