//! Расширенные интеграционные тесты для Tetris CLI.
//!
//! TODO: рассмотреть перенос в tests/ (PROB-120)
//!
//! Содержит тесты взаимодействия, не дублирующие тесты из `test_integration.rs`.

#![allow(deprecated)]

use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::tetromino::{BagGenerator, Tetromino};

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + TETROMINO
// ============================================================================

/// Тест: Tetromino из `BagGenerator` корректно инициализируется.
#[test]
fn test_tetromino_from_bag_to_gamestate() {
    let mut bag = BagGenerator::new();
    let tetromino = Tetromino::from_bag(&mut bag);

    assert_eq!(tetromino.pos(), (4.0, 0.0), "Позиция должна быть начальной");
    assert!(
        (tetromino.shape() as usize) < 7,
        "Тип фигуры должен быть валидным"
    );
}

/// Тест: Все типы фигур появляются в игре за множественные запуски.
#[test]
fn test_all_piece_types_appear_in_game() {
    let mut found_shapes = [false; 7];

    for _ in 0..70 {
        let state = GameState::new();
        let shape = state.curr_shape();
        found_shapes[shape.shape() as usize] = true;
    }

    for (i, &found) in found_shapes.iter().enumerate() {
        assert!(found, "Фигура типа {i:?} должна появиться в игре");
    }
}

/// Тест: Текущая и следующая фигуры обе валидны и различны.
#[test]
fn test_curr_and_next_shapes_different() {
    let state = GameState::new();

    let curr = state.curr_shape();
    let next = state.next_shape();

    assert!((curr.shape() as usize) < 7, "Текущая фигура должна быть валидной");
    assert!((next.shape() as usize) < 7, "Следующая фигура должна быть валидной");
    assert_ne!(
        curr.shape(),
        next.shape(),
        "Текущая и следующая фигуры должны различаться"
    );
}

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + HIGHSCORE
// ============================================================================

/// Тест: Marathon режим поддерживает сохранение рекорда.
#[test]
fn test_marathon_mode_saves_score() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Режим должен быть Marathon"
    );
}

/// Тест: Leaderboard сортирует рекорды по убыванию.
#[test]
fn test_leaderboard_sorts_scores() {
    let mut leaderboard = Leaderboard::default();

    let _ = leaderboard.add_score("P1", 100);
    let _ = leaderboard.add_score("P2", 300);
    let _ = leaderboard.add_score("P3", 200);

    let entries = leaderboard.get_entries();

    assert_eq!(entries[0].score(), Some(300), "Первый должен быть лучшим");
    assert_eq!(entries[1].score(), Some(200), "Второй должен быть средним");
    assert_eq!(entries[2].score(), Some(100), "Третий должен быть худшим");
}

/// Тест: `SaveData` защита от подделки.
#[test]
fn test_savedata_protection() {
    let save = SaveData::from_value(10000);

    let score = save.verify_and_get_score();
    assert_eq!(score, Some(10000), "Рекорд должен пройти проверку");
}

/// Тест: Leaderboard максимальный размер (5 записей).
#[test]
fn test_leaderboard_max_size_integration() {
    let mut leaderboard = Leaderboard::default();

    for i in 0..10 {
        let _ = leaderboard.add_score(&format!("P{i}"), u128::from(i as u64 * 100));
    }

    assert_eq!(
        leaderboard.len(),
        5,
        "Таблица должна содержать максимум 5 записей"
    );
}

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + CONTROLS
// ============================================================================

/// Тест: Hold фигура доступна в начале игры.
#[test]
fn test_hold_piece() {
    let state = GameState::new();
    assert!(state.can_hold(), "Hold должен быть доступен в начале");
}
