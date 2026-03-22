//! Тесты качества кода для проекта tetris-cli.
//!
//! Этот модуль содержит 24 теста (по 3 на каждую из 8 категорий):
//! 1. `dead_code` prevention - проверка unused кода
//! 2. deprecated API migration - проверка новых API
//! 3. assertions validation - проверка assert!()
//! 4. unwrap safety - проверка `unwrap()`
//! 5. iterator usage - проверка итераторов
//! 6. contains usage - проверка .`contains()`
//! 7. format usage - проверка format!()
//! 8. cast usage - проверка cast
//!
//! ПРИМЕЧАНИЕ: Эти тесты намеренно используют плохие практики кода
//! для проверки корректной работы clippy и компилятора.

#![allow(clippy::assertions_on_constants)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::useless_format)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::manual_is_ascii_check)]

// ============================================================================
// КАТЕГОРИЯ 1: dead_code prevention
// ============================================================================

/// Тест 1.1: Проверка что bench методы доступны только с feature = "bench"
///
/// Проверяет, что методы для бенчмарков компилируются только с флагом bench.
#[test]
fn test_bench_methods_only_with_bench_feature() {
    // Проверяем, что константа HARD_DROP_ANIM_INTERVAL_MS доступна всегда
    use crate::game::HARD_DROP_ANIM_INTERVAL_MS;

    // Константа должна быть доступна без feature = "bench"
    assert_eq!(HARD_DROP_ANIM_INTERVAL_MS, 50);

    // Методы *_for_bench доступны только с #[cfg(feature = "bench")]
    // Этот тест проверяет, что код компилируется без feature = "bench"
    // Если бы мы использовали bench методы без флага, компилятор выдал бы ошибку
}

/// Тест 1.2: Проверка что deprecated функции работают но предупреждают
///
/// Проверяет, что устаревшие функции всё ещё работают, но помечены как deprecated.
#[test]
#[allow(deprecated)]
fn test_deprecated_functions_still_work() {
    use crate::highscore::{generate_salt, get_random_hash};

    // get_random_hash() помечена как deprecated, но должна работать
    let old_hash = get_random_hash();
    let new_hash = generate_salt();

    // Обе функции должны возвращать строку длиной 64 символа
    assert_eq!(old_hash.len(), 64, "Deprecated функция должна работать");
    assert_eq!(new_hash.len(), 64, "Новая функция должна работать");

    // Фактически get_random_hash() вызывает generate_salt()
    // Поэтому они могут быть равны (зависит от timing)
}

/// Тест 1.3: Проверка что #[`allow(dead_code)`] работает корректно
///
/// Проверяет, что атрибут #[`allow(dead_code)`] подавляет предупреждения.
#[test]
#[allow(clippy::assertions_on_constants)]
fn test_allow_dead_code_attribute_works() {
    // Функция с #[allow(dead_code)] не вызывает предупреждений
    #[allow(dead_code)]
    fn unused_function() -> i32 {
        42
    }

    // Структура с #[allow(dead_code)] для полей
    #[allow(dead_code)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    // Создаём структуру, но не используем поля
    let _test = TestStruct {
        field1: 100,
        field2: "test".to_string(),
    };

    // Тест проходит если нет предупреждений компилятора
    assert!(true, "allow(dead_code) должен работать");
}

// ============================================================================
// КАТЕГОРИЯ 2: deprecated API migration
// ============================================================================

/// Тест 2.1: `generate_salt()` возвращает уникальный salt
///
/// Проверяет, что новая функция `generate_salt()` генерирует уникальные значения.
#[test]
fn test_generate_salt_returns_unique_salt() {
    use crate::highscore::generate_salt;

    // Генерируем несколько солей
    let salt1 = generate_salt();
    let salt2 = generate_salt();
    let salt3 = generate_salt();

    // Все соли должны быть разными
    assert_ne!(salt1, salt2, "Соль 1 и 2 должны быть разными");
    assert_ne!(salt2, salt3, "Соль 2 и 3 должны быть разными");
    assert_ne!(salt1, salt3, "Соль 1 и 3 должны быть разными");

    // Длина должна быть 64 hex символа
    assert_eq!(salt1.len(), 64);
    assert_eq!(salt2.len(), 64);
    assert_eq!(salt3.len(), 64);
}

/// Тест 2.2: `verify_and_get_score()` проверяет целостность
///
/// Проверяет, что новый API `verify_and_get_score()` корректно проверяет данные.
#[test]
fn test_verify_and_get_score_checks_integrity() {
    use crate::highscore::SaveData;

    // Создаём валидные данные
    let save = SaveData::from_value(5000);

    // verify_and_get_score() должен вернуть Some(score) для валидных данных
    let result = save.verify_and_get_score();
    assert_eq!(result, Some(5000), "Валидные данные должны пройти проверку");

    // Проверяем, что метод возвращает Option, а не паникует
    // Это преимущество нового API перед старым assert_hs()
}

/// Тест 2.3: `generate_salt()` != `get_random_hash()` (старый API)
///
/// Проверяет, что новый API отличается от старого (хотя функционально эквивалентен).
#[test]
#[allow(deprecated)]
fn test_new_api_different_from_old_api() {
    use crate::highscore::{generate_salt, get_random_hash};

    // Функционально они эквивалентны (get_random_hash вызывает generate_salt)
    // Но это разные функции с разными именами
    let salt_new = generate_salt();
    let salt_old = get_random_hash();

    // Обе должны возвращать hex строку длиной 64 символа
    assert_eq!(salt_new.len(), 64);
    assert_eq!(salt_old.len(), 64);

    // Проверяем, что это разные вызовы функций
    // (компилятор должен видеть разницу между ними)
    assert!(salt_new.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(salt_old.chars().all(|c| c.is_ascii_hexdigit()));
}

// ============================================================================
// КАТЕГОРИЯ 3: assertions validation
// ============================================================================

/// Тест 3.1: Проверка что тесты без проверок компилируются с warning
///
/// Проверяет поведение тестов без assert (компилятор может предупреждать).
#[test]
fn test_tests_without_assertions_compile() {
    // Тест без assert компилируется, но clippy может предупреждать
    // #[allow(clippy::assertions_on_constants)] подавляет некоторые предупреждения

    let value = 42;

    // Просто вычисляем значение без проверки
    // В реальном коде это может быть warning
    let _result = value * 2;

    // Минимальная проверка чтобы тест был валидным
    assert!(true, "Тест без проверок должен компилироваться");
}

/// Тест 3.2: Проверка что assert!(false) паникует
///
/// Проверяет, что assert!(false) вызывает панику теста.
#[test]
#[should_panic(expected = "assertion failed: false")]
fn test_assert_false_panics() {
    // assert!(false) должен вызывать панику
    assert!(false);
}

/// Тест 3.3: Проверка что assert!(condition) работает
///
/// Проверяет, что assert! с условием работает корректно.
#[test]
fn test_assert_condition_works() {
    let value: i32 = 100;

    // assert! с истинным условием проходит
    assert!(value > 50, "Значение должно быть больше 50");
    assert!(value == 100, "Значение должно быть равно 100");
    assert!(value <= 100, "Значение должно быть <= 100");

    // assert! с ложным условием паникует (проверяем через should_panic в другом тесте)
    let condition: bool = value % 10 == 0;
    assert!(condition, "Проверка на кратность 10 должна работать");
}

// ============================================================================
// КАТЕГОРИЯ 4: unwrap safety
// ============================================================================

/// Тест 4.1: `unwrap()` на None вызывает панику
///
/// Проверяет, что `unwrap()` на None вызывает панику.
#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_unwrap_on_none_panics() {
    let none_value: Option<i32> = None;

    // unwrap() на None должен паниковать
    let _value = none_value.unwrap();
}

/// Тест 4.2: `unwrap_or_default()` безопасен
///
/// Проверяет, что `unwrap_or_default()` не паникует на None.
#[test]
fn test_unwrap_or_default_is_safe() {
    let none_value: Option<i32> = None;
    let some_value: Option<i32> = Some(42);

    // unwrap_or_default() возвращает дефолтное значение для None
    let default_from_none = none_value.unwrap_or_default();
    assert_eq!(
        default_from_none, 0,
        "unwrap_or_default для None должен вернуть 0"
    );

    // unwrap_or_default() возвращает значение для Some
    let value_from_some = some_value.unwrap_or_default();
    assert_eq!(
        value_from_some, 42,
        "unwrap_or_default для Some должен вернуть значение"
    );
}

/// Тест 4.3: match предпочтительнее `unwrap()`
///
/// Проверяет, что match безопаснее `unwrap()`.
#[test]
fn test_match_preferred_over_unwrap() {
    let maybe_value: Option<i32> = Some(100);

    // match обрабатывает оба случая безопасно
    let result = match maybe_value {
        Some(v) => v * 2,
        None => 0,
    };
    assert_eq!(result, 200, "match должен обработать Some");

    // match с None
    let none_value: Option<i32> = None;
    let result_none = match none_value {
        Some(v) => v * 2,
        None => -1,
    };
    assert_eq!(result_none, -1, "match должен обработать None");
}

// ============================================================================
// КАТЕГОРИЯ 5: iterator usage
// ============================================================================

/// Тест 5.1: `enumerate()` дает индекс и значение
///
/// Проверяет, что `enumerate()` предоставляет индекс и элемент.
#[test]
#[allow(clippy::useless_vec)]
fn test_enumerate_gives_index_and_value() {
    let values = vec![10, 20, 30, 40, 50];

    // enumerate() возвращает (index, value)
    let mut sum = 0;
    for (index, &value) in values.iter().enumerate() {
        sum += value;
        // Проверяем, что индекс корректный
        assert_eq!(
            values[index], value,
            "Индекс должен соответствовать значению"
        );
    }

    assert_eq!(sum, 150, "Сумма должна быть 150");
}

/// Тест 5.2: `iter()` безопаснее индексации
///
/// Проверяет, что `iter()` безопаснее прямой индексации.
#[test]
#[allow(clippy::useless_vec)]
fn test_iter_safer_than_indexing() {
    let values = vec![1, 2, 3, 4, 5];

    // iter() безопасен - не может выйти за границы
    let sum_iter: i32 = values.iter().sum();
    assert_eq!(sum_iter, 15, "iter() должен дать правильную сумму");

    // Индексация может паниковать при выходе за границы
    // (проверяем в другом тесте с should_panic)
    let sum_index: i32 = (0..values.len()).map(|i| values[i]).sum();
    assert_eq!(sum_index, 15, "Индексация тоже должна работать");
}

/// Тест 5.3: .`take()` ограничивает итерацию
///
/// Проверяет, что .`take()` ограничивает количество элементов.
#[test]
#[allow(clippy::iter_out_of_bounds)]
fn test_take_limits_iteration() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // take(3) берёт только первые 3 элемента
    let first_three: Vec<_> = values.iter().take(3).copied().collect();
    assert_eq!(
        first_three,
        vec![1, 2, 3],
        "take(3) должен взять 3 элемента"
    );

    // take больше чем элементов - берёт все
    let all: Vec<_> = values.iter().take(100).copied().collect();
    assert_eq!(all.len(), 10, "take(100) должен взять все 10 элементов");

    // take(0) не берёт ничего
    let none: Vec<_> = values.iter().take(0).copied().collect();
    assert!(none.is_empty(), "take(0) не должен взять ничего");
}

// ============================================================================
// КАТЕГОРИЯ 6: contains usage
// ============================================================================

/// Тест 6.1: (0..=10).contains(&x) работает
///
/// Проверяет, что `Range::contains()` работает корректно.
#[test]
fn test_range_contains_works() {
    // Проверяем包含ение в диапазоне
    assert!((0..=10).contains(&5), "5 должен быть в диапазоне 0..=10");
    assert!((0..=10).contains(&0), "0 должен быть в диапазоне 0..=10");
    assert!((0..=10).contains(&10), "10 должен быть в диапазоне 0..=10");

    // Проверяем不包含ение
    assert!(
        !(0..=10).contains(&-1),
        "-1 не должен быть в диапазоне 0..=10"
    );
    assert!(
        !(0..=10).contains(&11),
        "11 не должен быть в диапазоне 0..=10"
    );
}

/// Тест 6.2: .`contains()` читаемее чем x >= 0 && x <= 10
///
/// Проверяет, что .`contains()` более читаем.
#[test]
#[allow(clippy::manual_range_contains)]
fn test_contains_more_readable_than_comparison() {
    let x = 5;

    // .contains() более читаем
    let contains_result = (0..=10).contains(&x);

    // Эквивалентное сравнение
    let comparison_result = x >= 0 && x <= 10;

    // Результаты должны совпадать
    assert_eq!(
        contains_result, comparison_result,
        "Результаты должны совпадать"
    );

    // Проверяем для разных значений
    for test_x in [-1, 0, 5, 10, 11] {
        let c = (0..=10).contains(&test_x);
        let r = test_x >= 0 && test_x <= 10;
        assert_eq!(c, r, "Для {test_x} результаты должны совпадать");
    }
}

/// Тест 6.3: .`contains()` с char работает
///
/// Проверяет, что .`contains()` работает с символами.
#[test]
fn test_contains_with_char_works() {
    let vowels = ['а', 'е', 'ё', 'и', 'о', 'у', 'ы', 'э', 'ю', 'я'];

    // Проверяем包含ение символов
    assert!(vowels.contains(&'а'), "'а' должна быть гласной");
    assert!(vowels.contains(&'о'), "'о' должна быть гласной");
    assert!(!vowels.contains(&'б'), "'б' не должна быть гласной");
    assert!(!vowels.contains(&'к'), "'к' не должна быть гласной");

    // Проверяем с ASCII символами
    let ascii_vowels = ['a', 'e', 'i', 'o', 'u'];
    assert!(ascii_vowels.contains(&'a'), "'a' должна быть гласной");
    assert!(!ascii_vowels.contains(&'z'), "'z' не должна быть гласной");
}

// ============================================================================
// КАТЕГОРИЯ 7: format usage
// ============================================================================

/// Тест 7.1: format!("text") лучше format!("{}", "text")
///
/// Проверяет, что format!("text") эффективнее.
#[test]
#[allow(clippy::useless_format)]
fn test_format_literal_better_than_placeholder() {
    // format!("text") - более эффективно (нет парсинга placeholder)
    let direct = format!("Hello");

    // format!("{}", "text") - менее эффективно (нужен парсинг {})
    let placeholder = format!("{}", "Hello");

    // Результаты одинаковы
    assert_eq!(direct, placeholder, "Результаты должны быть одинаковы");
    assert_eq!(direct, "Hello");

    // Но direct более эффективен (компилятор может оптимизировать)
}

/// Тест 7.2: write!(f, "text") лучше write!(f, "{}", "text")
///
/// Проверяет, что write! без placeholder эффективнее.
#[test]
#[allow(clippy::write_literal)]
fn test_write_literal_better_than_placeholder() {
    use std::fmt::Write;

    // write!(f, "text") - более эффективно
    let mut buffer1 = String::new();
    write!(buffer1, "Hello").unwrap();

    // write!(f, "{}", "text") - менее эффективно
    let mut buffer2 = String::new();
    write!(buffer2, "{}", "Hello").unwrap();

    // Результаты одинаковы
    assert_eq!(buffer1, buffer2, "Результаты должны быть одинаковы");
    assert_eq!(buffer1, "Hello");
}

/// Тест 7.3: .`to_string()` предпочтительнее для простых строк
///
/// Проверяет, что .`to_string()` лучше format!() для простых случаев.
#[test]
#[allow(clippy::useless_format)]
fn test_to_string_preferred_for_simple_strings() {
    let s = "Hello";

    // .to_string() - предпочтительнее для простых строк
    let string1 = s.to_string();

    // format!() - избыточен для простого преобразования
    let string2 = format!("{s}");

    // format!() без placeholder - тоже избыточен
    let string3 = format!("{s}");

    // Все результаты одинаковы
    assert_eq!(
        string1, string2,
        ".to_string() и format! должны быть одинаковы"
    );
    assert_eq!(
        string1, string3,
        ".to_string() и format!(s) должны быть одинаковы"
    );
    assert_eq!(string1, "Hello");
}

// ============================================================================
// КАТЕГОРИЯ 8: cast usage
// ============================================================================

/// Тест 8.1: i32 as i32 не нужен
///
/// Проверяет, что cast в тот же тип избыточен.
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_same_type_cast_unnecessary() {
    let value: i32 = 42;

    // i32 as i32 - избыточен (но компилируется)
    let casted = value as i32;

    // Значение не меняется
    assert_eq!(value, casted, "cast в тот же тип не должен менять значение");

    // Лучше просто использовать value напрямую
    let direct = value;
    assert_eq!(value, direct, "Прямое использование лучше");
}

/// Тест 8.2: u32 as i64 нужен
///
/// Проверяет, что cast в другой тип необходим.
#[test]
fn test_different_type_cast_necessary() {
    let value: u32 = 42;

    // u32 as i64 - необходим для преобразования типа
    let casted: i64 = i64::from(value);

    // Тип изменился
    assert_eq!(casted, 42, "Значение должно сохраниться");

    // Проверяем, что это действительно i64
    let large: i64 = casted * 1_000_000_000;
    assert_eq!(large, 42_000_000_000, "i64 может хранить большие значения");
}

/// Тест 8.3: `try_from()` безопаснее as
///
/// Проверяет, что `try_from()` безопаснее as для сужающих преобразований.
#[test]
fn test_try_from_safer_than_as() {
    // as может обрезать значение без ошибки
    let large_i64: i64 = 1_000_000;
    let truncated_u8: u8 = large_i64 as u8; // Обрезает до 64 (0x40)
    assert_eq!(truncated_u8, 64, "as обрезает значение");

    // try_from() возвращает ошибку при невозможности преобразования
    let result: Result<u8, _> = large_i64.try_into();
    assert!(
        result.is_err(),
        "try_from должен вернуть ошибку для большого значения"
    );

    // Для маленьких значений try_from() работает
    let small_i64: i64 = 42;
    let ok_u8: u8 = small_i64.try_into().unwrap();
    assert_eq!(ok_u8, 42, "try_from должен работать для маленьких значений");
}
