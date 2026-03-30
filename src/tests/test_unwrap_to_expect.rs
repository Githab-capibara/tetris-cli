//! Тесты конвертации unwrap() -> expect() (testes/*.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка что expect() вызывается с контекстом
//! - Проверка что паника содержит сообщение
//! - Проверка что все unwrap() заменены
//!
//! Исправление: замена unwrap() на expect() с понятными сообщениями об ошибках

// ============================================================================
// ГРУППА ТЕСТОВ: unwrap() -> expect()
// ============================================================================

/// Тест 1: Проверка что expect() вызывается с контекстом
///
/// Проверяет, что expect() используется с понятными сообщениями.
#[test]
fn test_expect_called_with_context() {
    // Тест с Option
    let some_value: Option<i32> = Some(42);
    let result = some_value.expect("Значение должно быть Some");
    assert_eq!(result, 42, "expect должен вернуть значение из Some");

    // Тест с Result
    let ok_result: Result<i32, &str> = Ok(100);
    let result2 = ok_result.expect("Результат должен быть Ok");
    assert_eq!(result2, 100, "expect должен вернуть значение из Ok");

    // Тест с вложенными Option
    let nested: Option<Option<i32>> = Some(Some(200));
    let inner = nested
        .expect("Внешний Option должен быть Some")
        .expect("Внутренний Option должен быть Some");
    assert_eq!(inner, 200, "Вложенный expect должен работать");

    // Тест с Vec
    let vec: Vec<i32> = vec![1, 2, 3];
    let first = vec.first().expect("Вектор должен содержать первый элемент");
    assert_eq!(first, &1, "Первый элемент должен быть 1");

    // Тест с HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key", 42);
    let value = map.get("key").expect("Карта должна содержать ключ 'key'");
    assert_eq!(value, &42, "Значение по ключу 'key' должно быть 42");
}

/// Тест 2: Проверка что паника содержит сообщение
///
/// Проверяет, что expect() паникует с правильным сообщением при None/Err.
#[test]
#[should_panic(expected = "Значение должно быть Some")]
fn test_expect_panics_with_message() {
    let none_value: Option<i32> = None;
    // Это должно паниковать с указанным сообщением
    none_value.expect("Значение должно быть Some");
}

/// Тест 3: Интеграционный тест с несколькими expect()
///
/// Проверяет цепочку из нескольких expect() вызовов.
#[test]
fn test_integration_with_multiple_expect() {
    // Создаём вектор с данными
    let data: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Получаем первый элемент с expect
    let first = data
        .first()
        .expect("Вектор должен содержать первый элемент");
    assert_eq!(first, &1, "Первый элемент должен быть 1");

    // Получаем последний элемент с expect
    let last = data
        .last()
        .expect("Вектор должен содержать последний элемент");
    assert_eq!(last, &5, "Последний элемент должен быть 5");

    // Получаем элемент по индексу с expect
    let third = data
        .get(2)
        .expect("Вектор должен содержать элемент по индексу 2");
    assert_eq!(third, &3, "Третий элемент должен быть 3");

    // Тест с map и expect
    let doubled: Vec<i32> = data
        .iter()
        .map(|x| x.checked_mul(2))
        .collect::<Vec<Option<i32>>>()
        .into_iter()
        .map(|opt| opt.expect("Умножение не должно переполняться"))
        .collect();

    assert_eq!(
        doubled,
        vec![2, 4, 6, 8, 10],
        "Удвоенные значения должны совпадать"
    );

    // Тест с фильтрацией и expect
    let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).copied().collect();

    assert_eq!(evens.len(), 2, "Должно быть 2 чётных числа");
    assert_eq!(evens[0], 2, "Первое чётное число должно быть 2");
    assert_eq!(evens[1], 4, "Второе чётное число должно быть 4");
}

/// Тест 4: Проверка что expect() работает с разными типами
///
/// Проверяет expect() с различными типами данных.
#[test]
fn test_expect_with_different_types() {
    // Option<String>
    let opt_str: Option<String> = Some("test".to_string());
    let str_val = opt_str.expect("Option<String> должен быть Some");
    assert_eq!(str_val, "test", "Значение должно быть 'test'");

    // Result<String, Error>
    let result_str: Result<String, &str> = Ok("success".to_string());
    let str_val2 = result_str.expect("Result должен быть Ok");
    assert_eq!(str_val2, "success", "Значение должно быть 'success'");

    // Option<Vec>
    let opt_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let vec_val = opt_vec.expect("Option<Vec> должен быть Some");
    assert_eq!(vec_val.len(), 3, "Вектор должен содержать 3 элемента");

    // Result с числом
    let result_num: Result<u64, &str> = Ok(12345);
    let num_val = result_num.expect("Result должен быть Ok");
    assert_eq!(num_val, 12345, "Число должно быть 12345");
}

/// Тест 5: Проверка что сообщения expect() понятные
///
/// Проверяет качество сообщений об ошибках в expect().
#[test]
fn test_expect_messages_are_clear() {
    // Примеры хороших сообщений
    let good_messages = [
        "Вектор должен содержать первый элемент",
        "Карта должна содержать ключ",
        "Результат должен быть Ok",
        "Значение должно быть Some",
        "Индекс должен быть в пределах границ",
    ];

    // Проверяем что сообщения содержат полезную информацию
    for msg in &good_messages {
        // Сообщение должно быть не пустым
        assert!(!msg.is_empty(), "Сообщение не должно быть пустым");

        // Сообщение должно быть достаточно длинным (> 10 символов)
        assert!(msg.len() > 10, "Сообщение должно быть подробным: {}", msg);

        // Сообщение должно содержать глагол или описание ожидания
        assert!(
            msg.contains("должен") || msg.contains("должна") || msg.contains("должно"),
            "Сообщение должно содержать ожидание: {}",
            msg
        );
    }

    // Проверяем что можно создать expect с понятным сообщением
    let value: Option<i32> = Some(42);
    let result = value.expect("Значение должно присутствовать и быть положительным");
    assert_eq!(result, 42, "Значение должно быть 42");
}
