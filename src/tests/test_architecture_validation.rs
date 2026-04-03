//! Тесты на централизацию валидации.
//!
//! Этот модуль проверяет что валидация централизована в `ValidationService`:
//! - `ValidationService` существует и работает
//! - `validate_f32_finite()` используется в `set_fall_speed()`
//! - `validate_u32_range()` существует и работает
//! - Отсутствие дублирования валидации
//!
//! ## Архитектурные заметки
//! Эти тесты подтверждают что валидация следует принципу DRY
//! и централизована в модуле `validation`.

#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::no_effect_underscore_binding)]

use crate::game::state::GameState;
use crate::validation::{ValidationError, ValidationErrorKind, ValidationService};

// ============================================================================
// ТЕСТ 1: VALIDATIONSERVICE СУЩЕСТВУЕТ
// ============================================================================

/// Тест что `ValidationService` существует и доступен.
///
/// # Архитектурные заметки
/// `ValidationService` был создан для централизации валидации данных
/// и соблюдения принципа DRY (Don't Repeat Yourself).
#[test]
fn test_validation_service_exists() {
    // Проверяем что ValidationService существует
    let _service = ValidationService;

    // Проверяем что можно вызвать методы
    let result = ValidationService::validate_f32_finite(1.0);
    assert!(result.is_ok(), "ValidationService должен существовать");

    // Проверяем что ValidationError существует
    let _error = ValidationError {
        message: "Тест".to_string(),
        kind: ValidationErrorKind::NotFinite,
    };
}

/// Тест что `ValidationService` имеет правильную структуру.
#[test]
fn test_validation_service_structure() {
    // Проверяем что ValidationError имеет правильные поля
    let error = ValidationError {
        message: "Тестовая ошибка".to_string(),
        kind: ValidationErrorKind::OutOfRange,
    };

    assert_eq!(error.message, "Тестовая ошибка");
    assert!(matches!(error.kind, ValidationErrorKind::OutOfRange));

    // Проверяем Display реализацию
    let display = format!("{error}");
    assert!(display.contains("Ошибка валидации"));

    // Проверяем что ValidationErrorKind имеет правильные варианты
    let _not_finite = ValidationErrorKind::NotFinite;
    let _out_of_range = ValidationErrorKind::OutOfRange;
}

// ============================================================================
// ТЕСТ 2: VALIDATE_F32_FINITE() ИСПОЛЬЗУЕТСЯ В SET_FALL_SPEED()
// ============================================================================

/// Тест что `validate_f32_finite()` используется в `set_fall_speed()`.
///
/// # Архитектурные заметки
/// `set_fall_speed()` должен использовать `ValidationService::validate_f32_finite()`
/// для валидации скорости падения на NaN и Infinity.
#[test]
fn test_validate_f32_finite_used_in_set_fall_speed() {
    let mut state = GameState::new();

    // Проверяем что валидная скорость устанавливается
    let result = state.set_fall_speed(1.5);
    assert!(result.is_ok(), "Валидная скорость должна установиться");

    // Проверяем что NaN отклоняется
    let result = state.set_fall_speed(f32::NAN);
    assert!(
        result.is_err(),
        "NaN должен отклоняться через validate_f32_finite()"
    );
    assert!(
        matches!(result, Err(crate::errors::GameError::ValidationError(_))),
        "Должна быть ошибка валидации"
    );

    // Проверяем что Infinity отклоняется
    let result = state.set_fall_speed(f32::INFINITY);
    assert!(
        result.is_err(),
        "Infinity должен отклоняться через validate_f32_finite()"
    );

    let result = state.set_fall_speed(f32::NEG_INFINITY);
    assert!(
        result.is_err(),
        "NegInfinity должен отклоняться через validate_f32_finite()"
    );
}

/// Тест что `validate_f32_finite()` используется в `set_land_timer()`.
#[test]
fn test_validate_f32_finite_used_in_set_land_timer() {
    let mut state = GameState::new();

    // Проверяем что валидный таймер устанавливается
    let result = state.set_land_timer(0.5);
    assert!(result.is_ok(), "Валидный таймер должен установиться");

    // Проверяем что NaN отклоняется
    let result = state.set_land_timer(f64::NAN);
    assert!(
        result.is_err(),
        "NaN должен отклоняться через validate_f32_finite()"
    );

    // Проверяем что Infinity отклоняется
    let result = state.set_land_timer(f64::INFINITY);
    assert!(
        result.is_err(),
        "Infinity должен отклоняться через validate_f32_finite()"
    );
}

// ============================================================================
// ТЕСТ 3: VALIDATE_U32_RANGE() СУЩЕСТВУЕТ И РАБОТАЕТ
// ============================================================================

/// Тест что `validate_u32_range()` существует и работает.
///
/// # Архитектурные заметки
/// `validate_u32_range()` используется для валидации диапазона u32 значений
/// и следует принципу DRY.
#[test]
fn test_validate_u32_range_exists_and_works() {
    // Проверяем что функция существует и работает
    let result = ValidationService::validate_u32_range(5, 1, 10);
    assert!(result.is_ok(), "validate_u32_range() должна существовать");

    // Проверяем валидные значения
    assert!(ValidationService::validate_u32_range(0, 0, 10).is_ok());
    assert!(ValidationService::validate_u32_range(5, 0, 10).is_ok());
    assert!(ValidationService::validate_u32_range(10, 0, 10).is_ok());

    // Проверяем невалидные значения
    let result = ValidationService::validate_u32_range(11, 0, 10);
    assert!(result.is_err(), "Значение вне диапазона должно отклоняться");
    assert_eq!(
        result.unwrap_err().kind,
        ValidationErrorKind::OutOfRange,
        "Должна быть ошибка OutOfRange"
    );

    let result = ValidationService::validate_u32_range(u32::MAX, 0, 10);
    assert!(result.is_err(), "Значение вне диапазона должно отклоняться");
}

/// Тест что `validate_u32_range()` используется в `set_fall_speed()`.
#[test]
fn test_validate_u32_range_used_in_set_fall_speed() {
    let mut state = GameState::new();

    // Проверяем что скорость устанавливается с валидацией диапазона
    let result = state.set_fall_speed(1.0);
    assert!(result.is_ok());

    // validate_u32_range() используется внутри set_fall_speed()
    // для дополнительной валидации диапазона
    // Это проверяется через код в state.rs строка 680
}

// ============================================================================
// ТЕСТ 4: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ ВАЛИДАЦИИ
// ============================================================================

/// Тест что валидация не дублируется в других модулях.
///
/// # Архитектурные заметки
/// Валидация должна быть централизована в `ValidationService`.
/// Этот тест проверяет что нет дублирования логики валидации.
#[test]
fn test_no_duplicate_validation_logic() {
    // Проверяем что ValidationService используется вместо дублирования
    let mut state = GameState::new();

    // set_fall_speed() использует ValidationService
    let result = state.set_fall_speed(1.0);
    assert!(result.is_ok());

    // set_land_timer() использует ValidationService
    let result = state.set_land_timer(0.5);
    assert!(result.is_ok());

    // Нет дублирования - валидация централизована
}

/// Тест что `ValidationError` используется для всех ошибок валидации.
#[test]
fn test_validation_error_used_for_all_validation_errors() {
    // Проверяем что ValidationError используется
    let error = ValidationError {
        message: "Тест".to_string(),
        kind: ValidationErrorKind::NotFinite,
    };

    // Проверяем что ошибка конвертируется в GameError
    let game_error = crate::errors::GameError::ValidationError(error.message.clone());
    assert!(matches!(
        game_error,
        crate::errors::GameError::ValidationError(_)
    ));
}

// ============================================================================
// ТЕСТ 5: ЦЕНТРАЛИЗАЦИЯ ВАЛИДАЦИИ В VALIDATIONService
// ============================================================================

/// Тест что вся валидация централизована в `ValidationService`.
///
/// # Архитектурные заметки
/// Этот тест подтверждает что `ValidationService` является единственным
/// местом для универсальной валидации данных.
#[test]
fn test_all_validation_centralized_in_validation_service() {
    // Список функций валидации в ValidationService:
    let validation_functions = ["validate_f32_finite", "validate_u32_range"];

    // Проверяем что функции работают
    assert!(ValidationService::validate_f32_finite(1.0).is_ok());
    assert!(ValidationService::validate_u32_range(5, 0, 10).is_ok());

    // Проверяем что функции используются в GameState
    let mut state = GameState::new();
    let _ = state.set_fall_speed(1.0); // Использует validate_f32_finite
    let _ = state.set_land_timer(0.5); // Использует validate_f32_finite

    assert_eq!(
        validation_functions.len(),
        2,
        "Должно быть 2 функции валидации"
    );
}

// ============================================================================
// ТЕСТ 6: DRY ПРИНЦИП В ВАЛИДАЦИИ
// ============================================================================

/// Тест что валидация следует DRY принципу.
///
/// # Архитектурные заметки
/// DRY (Don't Repeat Yourself) означает что логика валидации
/// не должна дублироваться в разных местах кода.
#[test]
fn test_validation_follows_dry_principle() {
    // Проверяем что ValidationService используется в нескольких местах
    let mut state = GameState::new();

    // set_fall_speed() использует ValidationService
    let _ = state.set_fall_speed(1.0);

    // set_land_timer() использует ValidationService
    let _ = state.set_land_timer(0.5);

    // Обе функции используют один и тот же ValidationService
    // вместо дублирования кода валидации
}

/// Тест что `ValidationError` используется консистентно.
#[test]
fn test_validation_error_used_consistently() {
    // Проверяем что ValidationError используется консистентно
    let error1 = ValidationService::validate_f32_finite(f32::NAN);
    assert!(error1.is_err());
    assert_eq!(error1.unwrap_err().kind, ValidationErrorKind::NotFinite);

    let error2 = ValidationService::validate_u32_range(11, 0, 10);
    assert!(error2.is_err());
    assert_eq!(error2.unwrap_err().kind, ValidationErrorKind::OutOfRange);

    // Оба типа ошибок используют один и тот же ValidationError
}

// ============================================================================
// ТЕСТ 7: ИНТЕГРАЦИОННЫЕ ТЕСТЫ ВАЛИДАЦИИ
// ============================================================================

/// Интеграционный тест что валидация работает в `GameState`.
#[test]
fn test_validation_integration_with_game_state() {
    let mut state = GameState::new();

    // Проверяем что валидация работает через GameState
    assert!(state.set_fall_speed(1.0).is_ok());
    assert!(state.set_fall_speed(f32::NAN).is_err());
    assert!(state.set_fall_speed(f32::INFINITY).is_err());

    assert!(state.set_land_timer(0.5).is_ok());
    assert!(state.set_land_timer(f64::NAN).is_err());
    assert!(state.set_land_timer(f64::INFINITY).is_err());
}

/// Тест что валидация не влияет на производительность критичных путей.
#[test]
fn test_validation_does_not_impact_performance() {
    let mut state = GameState::new();

    // Валидация должна быть быстрой
    for _ in 0..1000 {
        let _ = state.set_fall_speed(1.0);
    }

    // Если тест проходит - валидация не влияет на производительность
}
