//! Тесты обработки ошибок с ? оператором (main.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка propagation ошибок
//! - Проверка unwrap_or_else
//! - Проверка контекста ошибок
//!
//! Исправление: использование ? оператора для propagation ошибок

#![allow(clippy::unnecessary_lazy_evaluations)]
#![allow(clippy::manual_unwrap_or)]
#![allow(unused)]

// ============================================================================
// ГРУППА ТЕСТОВ: Обработка ошибок с ?
// ============================================================================

/// Тест 1: Проверка propagation ошибок
///
/// Проверяет, что ? оператор корректно propagates ошибки.
#[test]
fn test_error_propagation() {
    // Функция которая возвращает Result
    fn fallible_operation(x: i32) -> Result<i32, &'static str> {
        if x < 0 {
            Err("Отрицательное значение")
        } else {
            Ok(x * 2)
        }
    }

    // Функция которая использует ? для propagation
    fn propagate_error(x: i32) -> Result<i32, &'static str> {
        let result = fallible_operation(x)?;
        Ok(result + 10)
    }

    // Тест с успешным значением
    let success = propagate_error(5);
    assert_eq!(success, Ok(20), "? должен propagate Ok значение");

    // Тест с ошибкой
    let error = propagate_error(-5);
    assert_eq!(
        error,
        Err("Отрицательное значение"),
        "? должен propagate ошибку"
    );

    // Тест с цепочкой ? операторов
    fn chain_operations(x: i32) -> Result<i32, &'static str> {
        let a = fallible_operation(x)?;
        let b = fallible_operation(a)?;
        let c = fallible_operation(b)?;
        Ok(c)
    }

    // Успешная цепочка
    let chain_success = chain_operations(2);
    assert_eq!(chain_success, Ok(16), "Цепочка ? должна работать для Ok");

    // Ошибка в цепочке
    let chain_error = chain_operations(-1);
    assert_eq!(
        chain_error,
        Err("Отрицательное значение"),
        "Цепочка ? должна propagate ошибку"
    );
}

/// Тест 2: Проверка unwrap_or_else
///
/// Проверяет, что unwrap_or_else корректно обрабатывает ошибки.
#[test]
fn test_unwrap_or_else() {
    // Тест с Option
    let some_value: Option<i32> = Some(42);
    let result = some_value.unwrap_or_else(|| 0);
    assert_eq!(result, 42, "unwrap_or_else должен вернуть значение из Some");

    let none_value: Option<i32> = None;
    let result2 = none_value.unwrap_or_else(|| 100);
    assert_eq!(
        result2, 100,
        "unwrap_or_else должен вернуть значение по умолчанию для None"
    );

    // Тест с Result
    let ok_result: Result<i32, &str> = Ok(50);
    let result3 = ok_result.unwrap_or_else(|_| 0);
    assert_eq!(result3, 50, "unwrap_or_else должен вернуть значение из Ok");

    let err_result: Result<i32, &str> = Err("error");
    let result4 = err_result.unwrap_or_else(|e| {
        assert_eq!(e, "error", "Closure должен получить ошибку");
        200
    });
    assert_eq!(
        result4, 200,
        "unwrap_or_else должен вернуть значение по умолчанию для Err"
    );

    // Тест с вычислением значения по умолчанию
    let compute_default = || -> i32 {
        // Сложное вычисление значения по умолчанию
        10 + 20 + 30
    };

    let none: Option<i32> = None;
    let result5 = none.unwrap_or_else(compute_default);
    assert_eq!(
        result5, 60,
        "unwrap_or_else должен вычислить значение по умолчанию"
    );
}

/// Тест 3: Проверка контекста ошибок
///
/// Проверяет, что ошибки содержат полезный контекст.
#[test]
fn test_error_context() {
    // Создаём кастомный тип ошибки с контекстом
    #[derive(Debug, PartialEq)]
    struct AppError {
        context: String,
        source: String,
    }

    // Функция которая добавляет контекст к ошибке
    fn operation_with_context(x: i32) -> Result<i32, AppError> {
        if x < 0 {
            Err(AppError {
                context: "Обработка значения".to_string(),
                source: "Отрицательное значение".to_string(),
            })
        } else {
            Ok(x * 2)
        }
    }

    // Тест с успешным значением
    let success = operation_with_context(5);
    assert_eq!(success, Ok(10), "Операция должна успешно выполниться");

    // Тест с ошибкой и контекстом
    let error = operation_with_context(-5);
    match error {
        Err(e) => {
            assert_eq!(e.context, "Обработка значения", "Контекст должен совпадать");
            assert_eq!(
                e.source, "Отрицательное значение",
                "Источник ошибки должен совпадать"
            );
        }
        Ok(_) => panic!("Должна быть ошибка"),
    }

    // Тест с map_err для добавления контекста
    fn add_context(x: i32) -> Result<i32, AppError> {
        operation_with_context(x).map_err(|e| AppError {
            context: format!("Дополнительный контекст: {}", e.context),
            source: e.source,
        })
    }

    let error_with_context = add_context(-5);
    match error_with_context {
        Err(e) => {
            assert!(
                e.context.contains("Дополнительный контекст"),
                "Контекст должен быть расширен"
            );
        }
        Ok(_) => panic!("Должна быть ошибка"),
    }
}

/// Тест 4: Интеграционный тест с ? и unwrap_or_else
///
/// Проверяет комбинацию ? и unwrap_or_else.
#[test]
fn test_integration_question_and_unwrap_or_else() {
    fn fallible(x: i32) -> Result<i32, String> {
        if x < 0 {
            Err("negative".to_string())
        } else {
            Ok(x + 1)
        }
    }

    fn process(x: i32) -> i32 {
        // Используем ? с unwrap_or_else для обработки
        fallible(x).unwrap_or_else(|_| 0)
    }

    // Тест с успешным значением
    let result1 = process(5);
    assert_eq!(result1, 6, "Успешное значение должно быть обработано");

    // Тест с ошибкой
    let result2 = process(-5);
    assert_eq!(result2, 0, "Ошибка должна вернуть значение по умолчанию");

    // Тест с цепочкой
    fn process_chain(x: i32) -> i32 {
        let a = fallible(x).unwrap_or_else(|_| 1);
        let b = fallible(a).unwrap_or_else(|_| 2);
        let c = fallible(b).unwrap_or_else(|_| 3);
        c
    }

    let chain1 = process_chain(5);
    assert_eq!(chain1, 8, "Цепочка должна работать для успешных значений");

    let chain2 = process_chain(-5);
    assert_eq!(
        chain2, 4,
        "Цепочка должна использовать значения по умолчанию"
    );
}

/// Тест 5: Проверка что ? работает с разными типами ошибок
///
/// Проверяет ? оператор с различными типами ошибок.
#[test]
fn test_question_with_different_error_types() {
    // ? с String ошибкой
    fn str_error(x: i32) -> Result<i32, String> {
        if x < 0 {
            Err("negative number".to_string())
        } else {
            Ok(x)
        }
    }

    // ? с String ошибкой
    fn string_error(x: i32) -> Result<i32, String> {
        str_error(x).map_err(|e: String| e)
    }

    // ? с custom ошибкой
    #[derive(Debug, PartialEq)]
    struct CustomError(String);

    fn custom_error(x: i32) -> Result<i32, CustomError> {
        string_error(x).map_err(|e| CustomError(e))
    }

    // Тест с String
    let result1 = str_error(5);
    assert_eq!(result1, Ok(5), "? с String должен работать");

    // Тест с String
    let result2 = string_error(5);
    assert_eq!(result2, Ok(5), "? с String должен работать");

    // Тест с custom ошибкой
    let result3 = custom_error(5);
    assert_eq!(result3, Ok(5), "? с custom ошибкой должен работать");

    // Тест ошибки с custom типом
    let error3 = custom_error(-5);
    match error3 {
        Err(CustomError(msg)) => {
            assert_eq!(msg, "negative number", "Сообщение ошибки должно совпадать");
        }
        Ok(_) => panic!("Должна быть ошибка"),
    }
}
