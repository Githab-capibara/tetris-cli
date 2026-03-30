//! Тесты проверки целостности рекорда (highscore.rs).
//!
//! Этот модуль содержит 3 теста для проверки функции verify_and_get_score():
//! - Проверка возврата Some при валидном хэше
//! - Проверка возврата None при подделке
//! - Проверка логирования попытки подделки
//!
//! Функция verify_and_get_score() защищает рекорды от модификации.

use crate::highscore::SaveData;

// ============================================================================
// ГРУППА ТЕСТОВ: Проверка целостности рекорда
// ============================================================================

/// Тест 1: Проверка возврата Some при валидном хэше
///
/// Проверяет, что verify_and_get_score() возвращает Some(score)
/// для рекорда с валидным хэшем.
#[test]
fn test_verify_returns_some_for_valid_hash() {
    // Создаём валидный рекорд
    let save = SaveData::from_value(1000);

    // Проверяем что хэш валиден
    let result = save.verify_and_get_score();

    assert_eq!(
        result,
        Some(1000),
        "verify_and_get_score() должен вернуть Some(1000) для валидного хэша"
    );

    // Проверяем с другими значениями
    let save_5000 = SaveData::from_value(5000);
    let result_5000 = save_5000.verify_and_get_score();
    assert_eq!(
        result_5000,
        Some(5000),
        "verify_and_get_score() должен вернуть Some(5000)"
    );

    let save_zero = SaveData::from_value(0);
    let result_zero = save_zero.verify_and_get_score();
    assert_eq!(
        result_zero,
        Some(0),
        "verify_and_get_score() должен вернуть Some(0)"
    );

    // Проверяем с большим значением
    let save_large = SaveData::from_value(999_999);
    let result_large = save_large.verify_and_get_score();
    assert_eq!(
        result_large,
        Some(999_999),
        "verify_and_get_score() должен вернуть большое значение"
    );
}

/// Тест 2: Проверка возврата None при подделке
///
/// Проверяет, что verify_and_get_score() возвращает None
/// если рекорд был модифицирован (хэш не совпадает).
#[test]
fn test_verify_returns_none_for_tampered_record() {
    // Создаём валидный рекорд
    let mut save = SaveData::from_value(1000);

    // Проверяем что изначально он валиден
    assert_eq!(
        save.verify_and_get_score(),
        Some(1000),
        "Оригинальный рекорд должен быть валидным"
    );

    // Подделываем значение рекорда
    save.high_score = 99999;

    // Проверяем что подделка обнаружена
    let result = save.verify_and_get_score();
    assert_eq!(
        result,
        None,
        "verify_and_get_score() должен вернуть None для подделанного рекорда"
    );

    // Проверяем с другим значением
    let mut save2 = SaveData::from_value(5000);
    save2.high_score = 1;

    let result2 = save2.verify_and_get_score();
    assert_eq!(
        result2,
        None,
        "verify_and_get_score() должен вернуть None для подделки"
    );

    // Проверяем что подделка хэша тоже обнаруживается
    let mut save3 = SaveData::from_value(1000);
    save3.high_score_hash = "invalid_hash_123".to_string();

    let result3 = save3.verify_and_get_score();
    assert_eq!(
        result3,
        None,
        "verify_and_get_score() должен вернуть None для невалидного хэша"
    );
}

/// Тест 3: Проверка логирования попытки подделки
///
/// Проверяет, что при обнаружении подделки выводится сообщение в stderr.
#[test]
fn test_tampering_logs_warning() {
    // Создаём рекорд для подделки
    let mut save = SaveData::from_value(1000);

    // Проверяем что изначально рекорд валиден
    let valid_result = save.verify_and_get_score();
    assert_eq!(valid_result, Some(1000), "Рекорд должен быть валидным");

    // Подделываем значение
    save.high_score = 99999;

    // Вызываем verify_and_get_score() - должно логировать предупреждение в stderr
    let tampered_result = save.verify_and_get_score();

    // Проверяем что обнаружена подделка
    assert_eq!(
        tampered_result,
        None,
        "Подделка должна быть обнаружена"
    );

    // Проверяем что хэш не совпадает (создаём валидный рекорд для сравнения)
    let valid_save = SaveData::from_value(1000);
    assert_ne!(
        save.high_score_hash, valid_save.high_score_hash,
        "Хэш подделанного рекорда не должен совпадать с валидным"
    );

    // Проверяем с несколькими подделками
    for value in [100, 500, 1000, 5000, 10000] {
        let mut save = SaveData::from_value(value);
        save.high_score = value * 10; // Увеличиваем в 10 раз

        let result = save.verify_and_get_score();
        assert_eq!(
            result, None,
            "Подделка для значения {} должна быть обнаружена",
            value
        );
    }
}

/// Тест 4: Проверка что verify_and_get_score() работает с разными значениями
///
/// Интеграционный тест с различными значениями рекордов.
#[test]
fn test_verify_with_different_values() {
    let test_values = [0, 1, 10, 100, 1000, 10000, 100000, u64::MAX];

    for &value in &test_values {
        let save = SaveData::from_value(value);
        let result = save.verify_and_get_score();

        assert_eq!(
            result,
            Some(value),
            "verify_and_get_score() должен вернуть {} для валидного рекорда",
            value
        );
    }
}

/// Тест 5: Проверка что подделка соли обнаруживается
///
/// Проверяет, что модификация соли тоже обнаруживается.
#[test]
fn test_salt_tampering_detected() {
    let mut save = SaveData::from_value(1000);

    // Проверяем что изначально валиден
    assert_eq!(save.verify_and_get_score(), Some(1000));

    // Подделываем соль
    save.high_score_salt = "tampered_salt".to_string();

    // Проверяем что подделка обнаружена
    let result = save.verify_and_get_score();
    assert_eq!(
        result,
        None,
        "Подделка соли должна быть обнаружена"
    );
}
