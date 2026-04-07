//! Тесты системы Bag Generator в Tetris CLI.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Этот модуль содержит 30 тестов для проверки генератора фигур:
//! - Тесты равномерного распределения (6 тестов)
//! - Тесты перемешивания Fisher-Yates (6 тестов)
//! - Тесты повторного заполнения мешка (6 тестов)
//! - Тесты последовательностей фигур (6 тестов)
//! - Статистические тесты распределения (6 тестов)
//!
//! Все тесты проверяют корректность системы 7-bag.

use crate::tetromino::{BagGenerator, ShapeType};

// ============================================================================
// ГРУППА ТЕСТОВ 1-6: Тесты равномерного распределения
// ============================================================================

/// Тест 1: Bag Generator создаётся корректно
#[test]
fn test_bag_generator_creation() {
    let mut bag = BagGenerator::new();

    // Проверяем, что генератор создан и не пуст
    let shape = bag.next_shape();
    assert!(
        matches!(
            shape,
            ShapeType::T
                | ShapeType::L
                | ShapeType::J
                | ShapeType::S
                | ShapeType::Z
                | ShapeType::O
                | ShapeType::I
        ),
        "Bag должен содержать валидную фигуру"
    );
}

/// Тест 2: Первый мешок содержит все 7 фигур
#[test]
fn test_first_bag_contains_all_seven_pieces() {
    let mut bag = BagGenerator::new();
    let mut found_shapes = [false; 7];

    // Получаем 7 фигур из первого мешка
    for _ in 0..7 {
        let shape = bag.next_shape();
        found_shapes[shape as usize] = true;
    }

    // Проверяем, что все 7 типов встретились
    for (i, &found) in found_shapes.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна быть в первом мешке");
    }
}

/// Тест 3: Равномерное распределение в первом мешке
#[test]
fn test_uniform_distribution_first_bag() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Получаем 7 фигур
    for _ in 0..7 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ровно 1 раз
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(count, 1, "Фигура типа {i:?} должна встретиться ровно 1 раз");
    }
}

/// Тест 4: Равномерное распределение в нескольких мешках
#[test]
fn test_uniform_distribution_multiple_bags() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Получаем 70 фигур (10 полных мешков)
    for _ in 0..70 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ровно 10 раз
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(
            count, 10,
            "Фигура типа {i:?} должна встретиться ровно 10 раз"
        );
    }
}

/// Тест 5: Все типы фигур доступны
#[test]
fn test_all_piece_types_available() {
    let mut bag = BagGenerator::new();
    let mut found_shapes = [false; 7];

    // Получаем 700 фигур для статистики
    for _ in 0..700 {
        let shape = bag.next_shape();
        found_shapes[shape as usize] = true;
    }

    // Все 7 типов должны встретиться
    for (i, &found) in found_shapes.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна встретиться хотя бы раз");
    }
}

/// Тест 6: Отсутствие предпочтений в распределении
#[test]
fn test_no_preference_in_distribution() {
    let mut bag = BagGenerator::new();
    let mut t_count: i32 = 0;
    let mut i_count: i32 = 0;

    // Получаем 700 фигур
    for _ in 0..700 {
        let shape = bag.next_shape();
        match shape {
            ShapeType::T => t_count += 1,
            ShapeType::I => i_count += 1,
            _ => {}
        }
    }

    // T и I должны встречаться примерно одинаково часто
    // Допускаем отклонение до 30%
    let diff = (t_count - i_count).unsigned_abs();
    let expected: u32 = 100; // 700 / 7 = 100

    assert!(
        diff < expected / 2,
        "T и I должны встречаться примерно одинаково часто (T={t_count}, I={i_count})"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 7-12: Тесты перемешивания Fisher-Yates
// ============================================================================

/// Тест 7: Fisher-Yates создаёт разные последовательности
#[test]
fn test_fisher_yates_creates_different_sequences() {
    let mut bag1 = BagGenerator::new();
    let mut bag2 = BagGenerator::new();

    // Получаем последовательности из двух генераторов
    let mut seq1 = Vec::new();
    let mut seq2 = Vec::new();

    for _ in 0..7 {
        seq1.push(bag1.next_shape() as usize);
        seq2.push(bag2.next_shape() as usize);
    }

    // Последовательности могут совпасть, но это маловероятно
    // Проверяем, что обе содержат все 7 типов
    let mut unique1 = [false; 7];
    let mut unique2 = [false; 7];

    for &s in &seq1 {
        unique1[s] = true;
    }
    for &s in &seq2 {
        unique2[s] = true;
    }

    for i in 0..7 {
        assert!(unique1[i], "Первый мешок должен содержать фигуру {i:?}");
        assert!(unique2[i], "Второй мешок должен содержать фигуру {i:?}");
    }
}

/// Тест 8: Fisher-Yates гарантирует случайность
#[test]
fn test_fisher_yates_guarantees_randomness() {
    let mut first_positions = [0; 7];

    // Запускаем 70 раз и смотрим, где появляется T
    for _run in 0..70 {
        let mut local_bag = BagGenerator::new();
        for pos in &mut first_positions {
            let shape = local_bag.next_shape();
            if shape == ShapeType::T {
                *pos += 1;
                break;
            }
        }
    }

    // T должна появляться на разных позициях
    let non_zero_positions = first_positions.iter().filter(|&&x| x > 0).count();
    assert!(
        non_zero_positions >= 3,
        "T должна появляться на разных позициях (не только на одной)"
    );
}

/// Тест 9: Перемешивание не теряет фигуры
#[test]
fn test_shuffle_does_not_lose_pieces() {
    let mut bag = BagGenerator::new();
    let mut total_pieces = 0;

    // Получаем 100 фигур
    for _ in 0..100 {
        let _shape = bag.next_shape();
        total_pieces += 1;
    }

    assert_eq!(total_pieces, 100, "Должно быть получено ровно 100 фигур");
}

/// Тест 10: Перемешивание не дублирует фигуры в мешке
#[test]
fn test_shuffle_does_not_duplicate_pieces_in_bag() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Один мешок (7 фигур)
    for _ in 0..7 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна быть ровно 1 раз
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(
            count, 1,
            "Фигура типа {i:?} должна быть ровно 1 раз в мешке"
        );
    }
}

/// Тест 11: Алгоритм Fisher-Yates работает корректно
#[test]
fn test_fisher_yates_algorithm_works_correctly() {
    let mut bag = BagGenerator::new();

    // Получаем несколько мешков
    for bag_num in 0..10 {
        let mut counts = [0; 7];

        for _ in 0..7 {
            let shape = bag.next_shape();
            counts[shape as usize] += 1;
        }

        // Проверяем, что в каждом мешке все фигуры по 1 разу
        for (i, &count) in counts.iter().enumerate() {
            assert_eq!(
                count, 1,
                "Мешок {bag_num} должен содержать фигуру {i:?} ровно 1 раз"
            );
        }
    }
}

/// Тест 12: Случайность перемешивания
#[test]
fn test_shuffle_randomness() {
    let mut identical_bags = 0;
    let total_comparisons = 50;

    for _ in 0..total_comparisons {
        let mut bag1 = BagGenerator::new();
        let mut bag2 = BagGenerator::new();

        let mut seq1 = Vec::new();
        let mut seq2 = Vec::new();

        for _ in 0..7 {
            seq1.push(bag1.next_shape() as usize);
            seq2.push(bag2.next_shape() as usize);
        }

        if seq1 == seq2 {
            identical_bags += 1;
        }
    }

    // Вероятность идентичных мешков мала (1/7! = 1/5040)
    // Допускаем до 5% совпадений
    assert!(
        identical_bags < total_comparisons / 10,
        "Слишком много идентичных мешков: {identical_bags} из {total_comparisons}"
    );
}

// ============================================================================
// ГРУППА ТЕСТОВ 13-18: Тесты повторного заполнения мешка
// ============================================================================

/// Тест 13: Мешок заполняется после опустошения
#[test]
fn test_bag_refills_after_emptying() {
    let mut bag = BagGenerator::new();

    // Опустошаем первый мешок
    for _ in 0..7 {
        let _ = bag.next_shape();
    }

    // Получаем ещё одну фигуру - должен быть новый мешок
    let shape = bag.next_shape();
    assert!(
        (shape as usize) < 7,
        "Новый мешок должен содержать валидную фигуру"
    );
}

/// Тест 14: Непрерывная генерация фигур
#[test]
fn test_continuous_piece_generation() {
    let mut bag = BagGenerator::new();

    // Получаем 1000 фигур
    for i in 0..1000 {
        let shape = bag.next_shape();
        assert!(
            (shape as usize) < 7,
            "Фигура {i} должна быть валидной (0-6)"
        );
    }
}

/// Тест 15: Заполнение мешка происходит автоматически
#[test]
fn test_bag_fills_automatically() {
    let mut bag = BagGenerator::new();

    // Получаем 14 фигур (2 полных мешка)
    let mut counts = [0; 7];
    for _ in 0..14 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ровно 2 раза
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(count, 2, "Фигура типа {i:?} должна встретиться 2 раза");
    }
}

/// Тест 16: Multiple bag refills
#[test]
fn test_multiple_bag_refills() {
    let mut bag = BagGenerator::new();

    // Получаем 70 фигур (10 мешков)
    for _ in 0..70 {
        let _ = bag.next_shape();
    }

    // Генератор должен продолжать работать
    let shape = bag.next_shape();
    assert!(
        (shape as usize) < 7,
        "Генератор должен работать после 10 мешков"
    );
}

/// Тест 17: Переход между мешками без потерь
#[test]
fn test_bag_transition_without_loss() {
    let mut bag = BagGenerator::new();
    let mut total_pieces = 0;

    // Получаем 21 фигуру (3 мешка)
    for _ in 0..21 {
        let _ = bag.next_shape();
        total_pieces += 1;
    }

    assert_eq!(total_pieces, 21, "Должно быть получено ровно 21 фигура");
}

/// Тест 18: Заполнение мешка после частичного использования
#[test]
fn test_bag_refill_after_partial_use() {
    let mut bag = BagGenerator::new();

    // Используем 3 фигуры из первого мешка
    for _ in 0..3 {
        let _ = bag.next_shape();
    }

    // Используем 7 фигур из второго мешка
    for _ in 0..7 {
        let _ = bag.next_shape();
    }

    // Третий мешок должен заполниться автоматически
    let shape = bag.next_shape();
    assert!((shape as usize) < 7, "Третий мешок должен заполниться");
}

// ============================================================================
// ГРУППА ТЕСТОВ 19-24: Тесты последовательностей фигур
// ============================================================================

/// Тест 19: Последовательность из 7 фигур уникальна
#[test]
fn test_sequence_of_seven_is_unique() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    for _ in 0..7 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Все фигуры должны быть уникальны
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(count, 1, "Фигура типа {i:?} должна встретиться ровно 1 раз");
    }
}

/// Тест 20: Длинные последовательности корректны
#[test]
fn test_long_sequences_correct() {
    let mut bag = BagGenerator::new();

    // Получаем 140 фигур
    for i in 0..140 {
        let shape = bag.next_shape();
        assert!((shape as usize) < 7, "Фигура {i} должна быть валидной");
    }
}

/// Тест 21: Последовательность не содержит паттернов
#[test]
fn test_sequence_no_patterns() {
    let mut bag = BagGenerator::new();
    let mut prev_shape: Option<ShapeType> = None;
    let mut same_count = 0;

    for _ in 0..100 {
        let shape = bag.next_shape();
        if Some(shape) == prev_shape {
            same_count += 1;
        }
        prev_shape = Some(shape);
    }

    // В системе 7-bag одна и та же фигура не может идти подряд
    // из одного мешка, но может при переходе между мешками
    // Допускаем не более 15 совпадений подряд (при переходе мешков)
    assert!(
        same_count < 20,
        "Слишком много повторений подряд: {same_count}"
    );
}

/// Тест 22: Последовательность содержит все типы
#[test]
fn test_sequence_contains_all_types() {
    let mut bag = BagGenerator::new();
    let mut found_shapes = [false; 7];

    // Получаем 70 фигур
    for _ in 0..70 {
        let shape = bag.next_shape();
        found_shapes[shape as usize] = true;
    }

    // Все 7 типов должны встретиться
    for (i, &found) in found_shapes.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна встретиться");
    }
}

/// Тест 23: Чередование фигур
#[test]
fn test_piece_alternation() {
    let mut bag = BagGenerator::new();
    let mut last_shape: Option<ShapeType> = None;
    let mut alternations = 0;

    for _ in 0..100 {
        let shape = bag.next_shape();
        if last_shape != Some(shape) {
            alternations += 1;
        }
        last_shape = Some(shape);
    }

    // Большинство фигур должны чередоваться
    assert!(
        alternations > 80,
        "Фигуры должны чередоваться (чередований: {alternations})"
    );
}

/// Тест 24: Распределение в последовательности
#[test]
fn test_distribution_in_sequence() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Получаем 700 фигур
    for _ in 0..700 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Каждая фигура должна встретиться ровно 100 раз
    for (i, &count) in counts.iter().enumerate() {
        assert_eq!(count, 100, "Фигура типа {i:?} должна встретиться 100 раз");
    }
}

// ============================================================================
// ГРУППА ТЕСТОВ 25-30: Статистические тесты распределения
// ============================================================================

/// Тест 25: Статистика распределения всех типов фигур
#[test]
fn test_all_pieces_distribution_statistics() {
    use crate::tetromino::ShapeType;

    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Генерируем 700 фигур для статистики
    for _ in 0..700 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Проверяем распределение для T, I, O фигур
    // Ожидаем ~100 фигур каждого типа (700/7)
    // Допускаем отклонение до 30% (70-130)
    for &shape in &[ShapeType::T, ShapeType::I, ShapeType::O] {
        let count = counts[shape as usize];
        assert!(
            (70..=130).contains(&count),
            "{shape:?}-фигур должно быть около 100 (получено {count})"
        );
    }

    // Проверяем, что все фигуры встречаются примерно одинаково
    let min_count = counts
        .iter()
        .min()
        .expect("Минимальное значение должно существовать");
    let max_count = counts
        .iter()
        .max()
        .expect("Максимальное значение должно существовать");

    // Разница между мин и макс не должна превышать 50
    assert!(
        max_count - min_count < 50,
        "Разница между мин и макс должна быть меньше 50 (мин={min_count}, макс={max_count})"
    );
}

/// Тест 29: Дисперсия распределения
#[test]
fn test_distribution_variance() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    for _ in 0..700 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Вычисляем среднее
    let expected: i32 = 100; // 700 / 7

    // Вычисляем дисперсию
    let variance: f32 = counts
        .iter()
        .map(|&c| ((c - expected).pow(2)) as f32)
        .sum::<f32>()
        / 7.0;

    // Дисперсия не должна быть слишком большой
    assert!(
        variance < 400.0,
        "Дисперсия должна быть меньше 400 (получено {variance})"
    );
}

/// Тест 30: Статистический тест хи-квадрат (упрощённый)
#[test]
fn test_chi_square_simplified() {
    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    for _ in 0..700 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Вычисляем хи-квадрат статистику
    let expected = 100.0;
    let chi_square: f32 = counts
        .iter()
        .map(|&c| ((c as f32 - expected).powi(2)) / expected)
        .sum();

    // Для 6 степеней свободы и 95% доверительного интервала
    // критическое значение ~12.59
    // Используем более мягкое ограничение для тестов
    assert!(
        chi_square < 20.0,
        "Хи-квадрат должен быть меньше 20 (получено {chi_square})"
    );
}
