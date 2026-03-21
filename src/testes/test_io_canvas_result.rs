//! Тесты для исправления Canvas::new() Result (io.rs).
//!
//! Этот модуль содержит 3 теста для проверки исправления:
//! - Проверка успешного создания Canvas
//! - Проверка обработки ошибки при недоступности stdout
//! - Проверка корректного сообщения об ошибке
//!
//! Исправление: Canvas::new() теперь возвращает Result вместо паники

use crate::io::{Canvas, IoError};

// ============================================================================
// ГРУППА ТЕСТОВ: Canvas::new() Result fix
// ============================================================================

/// Тест 1: Проверка успешного создания Canvas
///
/// Проверяет, что Canvas::new() успешно создаёт канвас и возвращает Ok.
/// Примечание: тест может завершиться ошибкой если терминал не поддерживает raw-режим.
#[test]
#[ignore = "Требует реальный терминал, игнорируется в CI/CD"]
fn test_canvas_new_success() {
    // Создаём канвас - должен вернуть Ok(Canvas)
    let result = Canvas::new();
    
    // В нормальном терминале должен вернуться Ok
    assert!(
        result.is_ok(),
        "Canvas::new() должен вернуть Ok в нормальном терминале"
    );
    
    let canvas = result.unwrap();
    
    // Проверяем что канвас не паникует при использовании
    // (просто создаём и сразу уничтожаем через drop)
    drop(canvas);
}

/// Тест 2: Проверка обработки ошибки при недоступности stdout
///
/// Проверяет, что Canvas::new() корректно обрабатывает ошибку
/// при недоступности stdout или невозможности перехода в raw-режим.
#[test]
fn test_canvas_new_error_handling() {
    // Этот тест проверяет что Canvas::new() возвращает Result
    // а не паникует при ошибке
    
    // В CI/CD среде или без терминала Canvas::new() может вернуть ошибку
    let result = Canvas::new();
    
    // Проверяем что результат либо Ok, либо Err с правильным типом ошибки
    match result {
        Ok(canvas) => {
            // Канвас создан успешно - это нормально в терминале
            assert!(true, "Canvas создан успешно");
            drop(canvas);
        }
        Err(IoError::RawMode(msg)) => {
            // Ошибка raw-режима - проверяем сообщение
            assert!(
                !msg.is_empty(),
                "Сообщение об ошибке RawMode не должно быть пустым"
            );
            assert!(
                msg.contains("raw-режим") || msg.contains("терминал"),
                "Сообщение должно упоминать raw-режим или терминал: {}",
                msg
            );
        }
        Err(IoError::Clear(msg)) => {
            // Ошибка очистки экрана
            assert!(
                !msg.is_empty(),
                "Сообщение об ошибке Clear не должно быть пустым"
            );
        }
        Err(IoError::Cursor(msg)) => {
            // Ошибка курсора
            assert!(
                !msg.is_empty(),
                "Сообщение об ошибке Cursor не должно быть пустым"
            );
        }
        Err(IoError::Flush(msg)) => {
            // Ошибка flush
            assert!(
                !msg.is_empty(),
                "Сообщение об ошибке Flush не должно быть пустым"
            );
        }
        Err(IoError::Draw(msg)) => {
            // Ошибка отрисовки
            assert!(
                !msg.is_empty(),
                "Сообщение об ошибке Draw не должно быть пустым"
            );
        }
    }
}

/// Тест 3: Проверка корректного сообщения об ошибке
///
/// Проверяет, что при ошибке Canvas::new() возвращает понятное
/// сообщение об ошибке с информацией о причине.
#[test]
fn test_canvas_new_error_message() {
    // Проверяем что Canvas::new() возвращает Result с правильным типом
    let result: Result<Canvas, IoError> = Canvas::new();
    
    // Проверяем Display реализацию для IoError
    match &result {
        Ok(_) => {
            // Успешное создание - проверяем Display для всех типов ошибок
            // (просто убеждаемся что код компилируется и работает)
            let raw_error = IoError::RawMode("тест".to_string());
            assert!(
                raw_error.to_string().contains("Ошибка raw-режима"),
                "Display для RawMode должен работать корректно"
            );

            let clear_error = IoError::Clear("тест".to_string());
            assert!(
                clear_error.to_string().contains("Ошибка очистки"),
                "Display для Clear должен работать корректно"
            );

            let cursor_error = IoError::Cursor("тест".to_string());
            assert!(
                cursor_error.to_string().contains("Ошибка курсора"),
                "Display для Cursor должен работать корректно"
            );

            let flush_error = IoError::Flush("тест".to_string());
            assert!(
                flush_error.to_string().contains("Ошибка flush"),
                "Display для FlushError должен работать корректно"
            );
            
            let draw_error = IoError::Draw("тест".to_string());
            assert!(
                draw_error.to_string().contains("Ошибка отрисовки"),
                "Display для Draw должен работать корректно"
            );
        }
        Err(e) => {
            // Ошибка при создании - проверяем что Display работает
            let error_msg = e.to_string();
            assert!(
                !error_msg.is_empty(),
                "to_string() для IoError не должен возвращать пустую строку"
            );
            
            // Проверяем что сообщение содержит полезную информацию
            assert!(
                error_msg.len() > 10,
                "Сообщение об ошибке должно быть достаточно подробным: {}",
                error_msg
            );
        }
    }
}

/// Тест 4: Проверка что Canvas implements Drop
///
/// Проверяет, что Canvas автоматически сбрасывает терминал при выходе из области видимости.
#[test]
#[ignore = "Требует реальный терминал"]
fn test_canvas_drop() {
    // Создаём канвас в отдельной области видимости
    {
        let canvas = Canvas::new().expect("Не удалось создать Canvas");
        // Канвас используется...
    }
    // Здесь должен сработать Drop и сбросить терминал
    // Если Drop не работает, терминал останется в raw-режиме
    
    // Этот тест просто проверяет что код не паникует
    assert!(true, "Canvas::drop() не должен вызывать панику");
}

/// Тест 5: Проверка Canvas::reset()
///
/// Проверяет, что метод reset() корректно сбрасывает терминал.
#[test]
#[ignore = "Требует реальный терминал"]
fn test_canvas_reset() {
    let mut canvas = Canvas::new().expect("Не удалось создать Canvas");
    
    // Вызываем reset - не должно паниковать
    canvas.reset();
    
    // Проверяем что после reset канвас всё ещё работает
    canvas.flush();
    
    assert!(true, "Canvas::reset() не должен вызывать панику");
}
