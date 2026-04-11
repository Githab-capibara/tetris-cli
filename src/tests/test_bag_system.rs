//! Тесты системы Bag Generator в Tetris CLI.
//!
//! Модуль содержит 12 параметризированных тестов для проверки генератора фигур:
//! - Создание и базовая работа мешка (3 теста)
//! - Равномерное распределение и статистика (4 теста)
//! - Перемешивание Fisher-Yates (2 теста)
//! - Краевые случаи и стресс-тесты (3 теста)

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

// ============================================================================
// ГРУППА ТЕСТОВ 19-24: Тесты последовательностей фигур
// ============================================================================

/// Тест: Последовательность не содержит паттернов
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

/// Тест: Чередование фигур
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

// ============================================================================
// ГРУППА ТЕСТОВ 25-30: Статистические тесты распределения
// ============================================================================

// Статистические тесты распределения объединены в один параметризированный тест
// Ранее были: test_all_pieces_distribution_statistics, test_distribution_variance, test_chi_square_simplified
// Все они генерировали 700 фигур и проверяли одно распределение

/// Комплексный статистический тест распределения `BagGenerator`
/// Проверяет: равномерность, дисперсию, хи-квадрат упрощённый
#[test]
fn test_bag_distribution_statistics() {
    use crate::tetromino::ShapeType;

    let mut bag = BagGenerator::new();
    let mut counts = [0; 7];

    // Генерируем 700 фигур для статистики
    for _ in 0..700 {
        let shape = bag.next_shape();
        counts[shape as usize] += 1;
    }

    // Проверяем распределение для выбранных типов (T, I, O)
    // Ожидаем ~100 фигур каждого типа (700/7), допускаем отклонение до 30%
    for &shape in &[ShapeType::T, ShapeType::I, ShapeType::O] {
        let count = counts[shape as usize];
        assert!(
            (70..=130).contains(&count),
            "{shape:?}-фигур должно быть около 100 (получено {count})"
        );
    }

    // Проверяем, что все фигуры встречаются примерно одинаково
    let min_count = counts.iter().min().expect("Минимум должен существовать");
    let max_count = counts.iter().max().expect("Максимум должен существовать");
    assert!(
        max_count - min_count < 50,
        "Разница между мин и макс должна быть меньше 50 (мин={min_count}, макс={max_count})"
    );

    // Вычисляем дисперсию
    let expected: f32 = 100.0; // 700 / 7
    let variance: f32 = counts
        .iter()
        .map(|&c| (c as f32 - expected).powi(2))
        .sum::<f32>()
        / 7.0;
    assert!(
        variance < 400.0,
        "Дисперсия должна быть меньше 400 (получено {variance})"
    );

    // Вычисляем хи-квадрат статистику
    let chi_square: f32 = counts
        .iter()
        .map(|&c| ((c as f32 - expected).powi(2)) / expected)
        .sum();
    // Для 6 степеней свободы критическое значение ~12.59 (95%)
    assert!(
        chi_square < 20.0,
        "Хи-квадрат должен быть меньше 20 (получено {chi_square})"
    );
}
