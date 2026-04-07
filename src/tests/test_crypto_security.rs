//! Тесты безопасности криптографического модуля.
//!
//! PROB-159: Тесты проверяют корректность HMAC-SHA256 реализации.

use crate::crypto::hmac::{hmac_sha256, hmac_sign_with_salt, verify_hmac_sha256};

/// Тест: разные ключи дают разные подписи
#[test]
fn test_hmac_different_keys() {
    let sig1 = hmac_sha256("key1", "data");
    let sig2 = hmac_sha256("key2", "data");
    assert_ne!(sig1, sig2, "Разные ключи должны давать разные подписи");
}

/// Тест: HMAC-SHA256 всегда 64 hex символа для пустых данных
#[test]
fn test_hmac_empty_data() {
    let sig = hmac_sha256("key", "");
    assert_eq!(sig.len(), 64, "HMAC-SHA256 всегда 64 hex символа");
}

/// Тест: HMAC детерминированный
#[test]
fn test_hmac_deterministic() {
    let sig1 = hmac_sha256("key", "data");
    let sig2 = hmac_sha256("key", "data");
    assert_eq!(sig1, sig2, "HMAC должен быть детерминированным");
}

/// Тест: разные данные дают разные подписи
#[test]
fn test_hmac_different_data() {
    let sig1 = hmac_sha256("key", "data1");
    let sig2 = hmac_sha256("key", "data2");
    assert_ne!(sig1, sig2, "Разные данные должны давать разные подписи");
}

/// Тест: `verify_hmac_sha256` отклоняет подделанную подпись
#[test]
fn test_hmac_rejects_tampered_signature() {
    let sig = hmac_sha256("key", "data");
    // Изменяем первый символ
    let mut tampered = sig;
    let first_char = tampered
        .chars()
        .next()
        .expect("HMAC сигнатура не должна быть пустой");
    let new_char = if first_char == '0' { '1' } else { '0' };
    tampered.replace_range(..1, &new_char.to_string());

    assert!(
        !verify_hmac_sha256("key", "data", &tampered),
        "Подделанная подпись должна быть отклонена"
    );
}

/// Тест: HMAC с солью даёт уникальные подписи
#[test]
fn test_hmac_with_salt_unique_signatures() {
    let sig1 = hmac_sign_with_salt("key", "salt1", "data");
    let sig2 = hmac_sign_with_salt("key", "salt2", "data");
    assert_ne!(sig1, sig2, "Разные соли должны давать разные подписи");
}

/// Тест: HMAC-SHA256 вывод является валидным hex
#[test]
fn test_hmac_output_is_valid_hex() {
    let sig = hmac_sha256("key", "data");
    assert!(
        sig.chars().all(|c| c.is_ascii_hexdigit()),
        "HMAC подпись должна быть валидной hex строкой"
    );
}

/// Тест: HMAC-SHA256 с Unicode входом
#[test]
fn test_hmac_unicode_input() {
    let sig = hmac_sha256("ключ", "данные");
    assert_eq!(sig.len(), 64);
    assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
}
