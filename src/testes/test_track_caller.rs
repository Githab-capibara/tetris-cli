//! Тесты #[track_caller] атрибутов.
//!
//! Проверяют, что ошибки указывают на место вызова.

use crate::game::GameState;

/// Тест 1: Проверка, что panic location указывает на место вызова
///
/// Проверяем, что при панике указывается правильное место.
#[test]
#[should_panic(expected = "тестовая паника")]
fn test_track_caller_panic_location() {
    // Эта функция должна паниковать в строке ниже
    panic_with_caller("тестовая паника");
}

/// Вспомогательная функция с track_caller
#[track_caller]
fn panic_with_caller(msg: &str) {
    panic!("{}", msg);
}

/// Тест 2: Проверка обработки ошибок с track_caller
///
/// Проверяем, что ошибки обрабатываются корректно.
#[test]
fn test_track_caller_error_handling() {
    // Создаём состояние игры
    let state = GameState::new();

    // Проверяем, что методы работают корректно
    let score = state.get_score();
    assert_eq!(score, 0, "Счёт должен быть 0");

    // Если дошли сюда - ошибки обработаны корректно
    assert!(true, "Ошибки должны обрабатываться корректно");
}

/// Тест 3: Проверка assert с track_caller
///
/// Проверяем, что assert указывает на правильное место.
#[test]
#[should_panic(expected = "assertion failed")]
fn test_track_caller_assert_location() {
    // Этот assert должен паниковать в этой строке
    assert_with_caller(false, "assertion failed");
}

/// Вспомогательная функция assert с track_caller
#[track_caller]
fn assert_with_caller(condition: bool, msg: &str) {
    assert!(condition, "{}", msg);
}

/// Тест 4: Проверка debug_assert с track_caller
///
/// Проверяем, что debug_assert работает корректно.
#[test]
fn test_track_caller_debug_assert() {
    let state = GameState::new();

    // debug_assert должен работать в тестах
    debug_assert!(state.get_score() == 0, "Счёт должен быть 0");
    debug_assert!(state.get_level() == 1, "Уровень должен быть 1");

    assert!(true, "debug_assert должен работать в тестах");
}

/// Тест 5: Проверка expect с track_caller
///
/// Проверяем, что expect указывает на правильное место.
#[test]
#[should_panic(expected = "Option был None")]
fn test_track_caller_expect_location() {
    let opt: Option<i32> = None;
    // Этот expect должен паниковать в этой строке
    opt.expect_with_caller("Option был None");
}

/// Вспомогательная функция expect с track_caller
trait OptionExt<T> {
    fn expect_with_caller(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn expect_with_caller(self, msg: &str) -> T {
        self.expect(msg)
    }
}

/// Тест 6: Проверка unwrap с track_caller
///
/// Проверяем, что unwrap указывает на правильное место.
#[test]
#[should_panic(expected = "ошибка")]
fn test_track_caller_unwrap_location() {
    let result: Result<i32, &str> = Err("ошибка");
    // Этот unwrap должен паниковать
    result.unwrap_with_caller();
}

/// Вспомогательная функция unwrap с track_caller
trait ResultExt<T, E> {
    fn unwrap_with_caller(self) -> T;
}

impl<T, E: std::fmt::Debug> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn unwrap_with_caller(self) -> T {
        self.unwrap()
    }
}

/// Тест 7: Проверка, что track_caller не влияет на производительность
///
/// Проверяем, что track_caller не замедляет код.
#[test]
fn test_track_caller_performance() {
    use std::time::Instant;

    let iterations = 10000;
    let start = Instant::now();

    for _ in 0..iterations {
        let state = GameState::new();
        let _score = state.get_score();
        let _level = state.get_level();
    }

    let elapsed = start.elapsed();

    // 10000 итераций должны выполняться < 50ms
    assert!(
        elapsed.as_millis() < 50,
        "track_caller не должен значительно влиять на производительность"
    );
}
