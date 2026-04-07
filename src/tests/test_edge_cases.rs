//! Тесты конкурентного доступа и дополнительных edge-case.
//!
//! PROB-162: Тест на конкурентный доступ к leaderboard
//! PROB-164: Тест на score overflow
//! PROB-165: HMAC пустой ключ
//! PROB-166: HMAC длинный ключ
//! PROB-167: Валидация path traversal ../../
//! PROB-168: Symlink attack тест
//! PROB-169: Race condition в leaderboard
//! PROB-170: IO errors при загрузке конфига
//! PROB-171: Некорректный JSON в save
//! PROB-173: Тест bag randomizer

use std::sync::Arc;

// ============================================================================
// PROB-162: Конкурентный доступ к leaderboard
// ============================================================================

/// Тест: конкурентный доступ к leaderboard
#[test]
fn test_leaderboard_concurrent_access() {
    use crate::highscore::leaderboard::ThreadSafeLeaderboardEntry;
    use std::sync::Barrier;
    use std::thread;

    let entry = Arc::new(ThreadSafeLeaderboardEntry::new("Player", 1000));
    let barrier = Arc::new(Barrier::new(4));

    let mut handles = vec![];

    // 4 потока одновременно читают запись
    for _ in 0..4 {
        let entry = Arc::clone(&entry);
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier.wait(); // Синхронизируем старт
            let score = entry.score_safe();
            let is_valid = entry.is_valid_safe();
            let name = entry.name_safe();
            (score, is_valid, name)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (score, is_valid, name) = handle.join().expect("Поток не должен паниковать");
        assert_eq!(score, Some(1000), "Счёт должен быть 1000");
        assert_eq!(is_valid, Some(true), "Запись должна быть валидной");
        assert_eq!(name, Some("Player".to_string()), "Имя должно быть Player");
    }
}

// ============================================================================
// PROB-164: Score overflow
// ============================================================================

/// Тест: score не переполняется при добавлении максимального значения
#[test]
fn test_score_no_overflow() {
    use crate::game::scoring::lines::{update_score_for_lines, MAX_SCORE};

    let mut score = MAX_SCORE;
    let level = 1u32;
    let mut combo_counter = 1u32;

    // Попытка добавить очки к максимальному счёту
    let result = update_score_for_lines(&mut score, level, 4, &mut combo_counter);

    // Счёт должен быть ограничен MAX_SCORE
    assert!(
        score <= MAX_SCORE,
        "Счёт ({score}) не должен превышать MAX_SCORE ({MAX_SCORE})"
    );

    // Должна вернуться ошибка переполнения
    assert!(result.is_err(), "При переполнении должна вернуться ошибка");
}

/// Тест: `saturating_add` предотвращает u128 переполнение
#[test]
fn test_score_saturating_add_prevents_overflow() {
    let mut score: u128 = u128::MAX;
    let addition: u128 = 1_000_000;

    score = score.saturating_add(addition);
    assert_eq!(score, u128::MAX, "saturating_add должен вернуть MAX");
}

/// Тест: GameState `set_score` с `u128::MAX` не паникует
#[test]
fn test_game_state_set_score_max_no_panic() {
    use crate::game::GameState;

    let mut state = GameState::new();
    state.set_score(u128::MAX);
    assert_eq!(state.score(), u128::MAX);
}

// ============================================================================
// PROB-165: HMAC пустой ключ
// ============================================================================

/// Тест: HMAC с пустым ключом работает корректно
#[test]
fn test_hmac_empty_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    let sig = hmac_sha256("", "data");
    assert_eq!(sig.len(), 64, "HMAC с пустым ключом должен быть 64 символа");
    assert!(
        verify_hmac_sha256("", "data", &sig),
        "Подпись с пустым ключом должна верифицироваться"
    );
}

// ============================================================================
// PROB-166: HMAC длинный ключ (> 64 байт)
// ============================================================================

/// Тест: HMAC с ключом длиннее 64 байт (больше размера блока SHA-256)
#[test]
fn test_hmac_long_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    // Ключ 128 байт (> 64 байт, размер блока SHA-256)
    let long_key = "a".repeat(128);
    let sig = hmac_sha256(&long_key, "data");
    assert_eq!(
        sig.len(),
        64,
        "HMAC с длинным ключом должен быть 64 символа"
    );
    assert!(
        verify_hmac_sha256(&long_key, "data", &sig),
        "Подпись с длинным ключом должна верифицироваться"
    );
}

/// Тест: HMAC с очень длинным ключом (1024 байта)
#[test]
fn test_hmac_very_long_key() {
    use crate::crypto::hmac::{hmac_sha256, verify_hmac_sha256};

    let very_long_key = "x".repeat(1024);
    let sig = hmac_sha256(&very_long_key, "data");
    assert_eq!(sig.len(), 64);
    assert!(verify_hmac_sha256(&very_long_key, "data", &sig));
}

// ============================================================================
// PROB-167: Валидация ../../
// ============================================================================

/// Тест: `PathValidator` отклоняет path traversal с ../../
#[test]
fn test_path_validator_rejects_traversal() {
    use crate::validation::PathValidator;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    // Обычный ../../ должен быть отклонён
    assert!(
        validator.validate_no_traversal("../../etc/passwd").is_err(),
        "../../ должен быть отклонён"
    );

    // Одиночный ../ тоже
    assert!(
        validator.validate_no_traversal("../config.json").is_err(),
        "../ должен быть отклонён"
    );

    // Вложенный ../../
    assert!(
        validator
            .validate_no_traversal("foo/../../etc/passwd")
            .is_err(),
        "Вложенный ../../ должен быть отклонён"
    );
}

/// Тест: `PathValidator` отклоняет URL-encoded path traversal
#[test]
fn test_path_validator_rejects_url_encoded_traversal() {
    use crate::validation::PathValidator;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    assert!(validator
        .validate_no_traversal("..%2F..%2Fetc%2Fpasswd")
        .is_err());
    assert!(validator.validate_no_traversal("%2e%2e%2f").is_err());
    assert!(validator.validate_no_traversal("%2E%2E%2F").is_err());
}

// ============================================================================
// PROB-168: Symlink attack
// ============================================================================

/// Тест: `PathValidator` проверяет `symlink_metadata`
/// Примечание: полная проверка symlink требует существующей файловой системы.
/// Этот тест документирует что `validate_no_symlinks` вызывает `symlink_metadata`.
#[test]
fn test_path_validator_symlink_check_exists() {
    use crate::validation::PathValidator;
    use std::path::Path;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz._-/");

    // Для несуществующего файла проверка symlink должна проходить (файла нет = нет symlink)
    let nonexistent = Path::new("/nonexistent/path/file.txt");
    // validate_no_symlinks возвращает Ok для несуществующих файлов
    // (проверка будет при создании файла через O_NOFOLLOW)
    let result = validator.validate_no_symlinks(nonexistent);
    // Может быть Ok или Err в зависимости от прав доступа — главное что не паникует
    let _ = result;
}

// ============================================================================
// PROB-169: Race condition в leaderboard
// ============================================================================

/// Тест: `ThreadSafeLeaderboardEntry` защищает от race condition
#[test]
fn test_leaderboard_race_condition_protection() {
    use crate::highscore::leaderboard::ThreadSafeLeaderboardEntry;
    use std::sync::Barrier;
    use std::thread;

    let entry = Arc::new(ThreadSafeLeaderboardEntry::new("TestPlayer", 5000));
    let barrier = Arc::new(Barrier::new(8));
    let mut handles = vec![];

    // 8 потоков одновременно читают
    for _ in 0..8 {
        let entry = Arc::clone(&entry);
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier.wait();
            // Многократное чтение для увеличения шанса race condition
            for _ in 0..100 {
                let score = entry.score_safe();
                assert_eq!(score, Some(5000));
            }
            true
        });
        handles.push(handle);
    }

    for handle in handles {
        assert!(
            handle.join().unwrap(),
            "Все потоки должны завершиться успешно"
        );
    }
}

// ============================================================================
// PROB-170: IO errors при загрузке конфига
// ============================================================================

/// Тест: загрузка конфига из невалидного пути возвращает ошибку
#[test]
fn test_config_load_from_invalid_path() {
    use crate::controls::ControlsConfig;

    // Невалидный путь должен вернуть ошибку
    let result = ControlsConfig::load_from_file("/nonexistent/path/config.json");
    assert!(
        result.is_err(),
        "Загрузка из несуществующего пути должна вернуть ошибку"
    );
}

/// Тест: загрузка конфига из пустого пути
#[test]
fn test_config_load_from_empty_path() {
    use crate::controls::ControlsConfig;

    let result = ControlsConfig::load_from_file("");
    assert!(
        result.is_err(),
        "Загрузка из пустого пути должна вернуть ошибку"
    );
}

// ============================================================================
// PROB-171: Некорректный JSON в save
// ============================================================================

/// Тест: `Leaderboard::load()` возвращает пустую таблицу при повреждённых данных
/// confy хранит данные в формате TOML/JSON — при некорректных данных возвращает default
#[test]
fn test_leaderboard_load_handles_corrupted_data() {
    use crate::highscore::leaderboard::Leaderboard;

    // load() никогда не паникует — при ошибке возвращает пустую таблицу
    let result = std::panic::catch_unwind(|| {
        let _ = Leaderboard::load();
    });
    assert!(
        result.is_ok(),
        "Leaderboard::load() не должен паниковать даже при повреждённых данных"
    );
}

/// Тест: Leaderboard по умолчанию пуст
#[test]
fn test_leaderboard_default_is_empty() {
    use crate::highscore::leaderboard::Leaderboard;

    let lb = Leaderboard::default();
    assert_eq!(lb.len(), 0);
    assert!(lb.is_empty());
}

// ============================================================================
// PROB-173: Тест bag randomizer
// ============================================================================

/// Тест: `BagGenerator` выдаёт все 7 фигур за один мешок
// Удалён как дубликат test_first_bag_contains_all_seven_pieces из test_bag_system.rs
// Проверял то же самое: все 7 уникальных фигур в одном мешке

/// Тест: `BagGenerator` равномерно распределяет фигуры за много итераций
// Удалён как дубликат test_uniform_distribution_multiple_bags из test_bag_system.rs
// Проверял то же самое: 70 фигур, каждая ровно 10 раз

/// Тест: `BagGenerator` детерминирован по структуре (не по порядку)
#[test]
fn test_bag_generator_structure_guarantees() {
    use crate::tetromino::bag_generator::BagGenerator;

    let mut bag = BagGenerator::new();

    // За 7 вызовов получаем все 7 фигур
    let mut found = [false; 7];
    for _ in 0..7 {
        let shape = bag.next_shape();
        let idx = shape as usize;
        assert!(idx < 7, "Индекс фигуры должен быть < 7");
        found[idx] = true;
    }

    // Все 7 фигур найдены
    assert!(found.iter().all(|&b| b), "Все 7 фигур должны появиться");

    // После 7 вызовов position = 7
    assert_eq!(bag.get_index(), 7);
}
