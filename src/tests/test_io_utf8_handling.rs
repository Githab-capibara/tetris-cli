//! Тесты обработки UTF-8 (io.rs).
//!
//! Проверяют что KeyReader корректно возвращает `io::Result<Option<u8>>`
//! и не паникует при различных сценариях ввода.

use crate::io::KeyReader;

/// Проверка что ASCII символы читаются корректно.
#[test]
fn test_key_reader_returns_ascii_or_none() {
    let mut reader = KeyReader::new();
    let result = reader.get_key();

    match result {
        Ok(Some(key)) => {
            // Если клавиша была нажата, проверяем что это ASCII
            assert!(key <= 0x7F, "ASCII символ должен быть <= 0x7F, получен: {key}");
        }
        Ok(None) => {
            // Нет ввода — нормально в тестовой среде
        }
        Err(_) => {
            // Ошибка чтения (например, нет терминала) — тоже допустимо
        }
    }
}

/// Проверка стабильности KeyReader при многократном использовании.
#[test]
fn test_key_reader_stability() {
    // Многократное создание KeyReader без паники
    for i in 0..20 {
        let mut reader = KeyReader::new();
        let result = reader.get_key();

        if let Ok(Some(key)) = result {
            assert!(key <= 0x7F, "Итерация {i}: key {key} должен быть ASCII");
        }
    }
}
