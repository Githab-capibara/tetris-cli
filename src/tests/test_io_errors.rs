//! Тесты для проверки обработки ошибок ввода в `KeyReader`.
//!
//! Содержит только тесты, проверяющие реальное поведение KeyReader,
//! а не диапазоны байт вручную.

use crate::io::KeyReader;

/// Тест: проверка что `get_key()` не паникует при ошибке чтения
#[test]
fn test_get_key_no_panic_on_read_error() {
    let mut reader = KeyReader::new();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = reader.get_key();
    }));

    assert!(
        result.is_ok(),
        "get_key() не должен паниковать при ошибке чтения"
    );
}

/// Тест: проверка Drop реализации KeyReader
#[test]
fn test_key_reader_drop() {
    {
        let _reader = KeyReader::new();
    }
    // Если тест дошёл сюда без паники — Drop сработал корректно
}

/// Тест: проверка что KeyReader реализует InputReader трейт
#[test]
fn test_key_reader_implements_input_reader() {
    fn assert_input_reader<T: crate::io_traits::InputReader>() {}
    assert_input_reader::<KeyReader>();
}
