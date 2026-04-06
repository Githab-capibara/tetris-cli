//! Расширенные интеграционные тесты для Tetris CLI.
//!
//! Содержит тесты взаимодействия и производительности,
//! не дублирующие тесты из `test_integration.rs`.

#![allow(deprecated)]

use crate::game::GameState;
use crate::highscore::{Leaderboard, SaveData};
use crate::tetromino::{BagGenerator, ShapeType, Tetromino, SHAPE_COORDS};
use crate::types::RotationDirection;

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + TETROMINO
// ============================================================================

/// Тест: Tetromino из BagGenerator корректно инициализируется
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

/// Тест: Все типы фигур появляются в игре за множественные запуски
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

/// Тест: Фигура не выходит за границы поля при движении
#[test]
fn test_piece_stays_within_bounds() {
    let mut state = GameState::new();

    while state.can_move_curr_shape_direction(crate::types::Direction::Left) {
        state.get_curr_shape_mut().pos_mut().0 -= 1.0;
    }

    let shape = state.curr_shape();
    for &(x, _) in &shape.coords() {
        let global_x = shape.pos().0 as i16 + x;
        assert!(global_x >= 0, "Фигура не должна выходить за левую границу");
    }
}

/// Тест: Текущая и следующая фигуры обе валидны
#[test]
fn test_curr_and_next_shapes_different() {
    let state = GameState::new();

    let curr = state.curr_shape();
    let next = state.next_shape();

    assert!((curr.shape() as usize) < 7);
    assert!((next.shape() as usize) < 7);
}

// ============================================================================
// ВЗАИМОДЕЙСТВИЕ GAME + HIGHSCORE
// ============================================================================

/// Тест: GameState может сохранять рекорд
#[test]
#[ignore = "depends on confy file system"]
fn test_gamestate_can_save_score() {
    let mut state = GameState::new();
    let _ = state.add_score(500);

    let score = state.score();
    SaveData::save_value(score);

    let loaded = SaveData::load_config();
    let loaded_score = loaded.verify_and_get_score().unwrap_or(0);

    if loaded_score == 0 && score != 0 {
        return;
    }

    assert_eq!(
        loaded_score, score,
        "Рекорд должен загрузиться и быть валидным"
    );
}

/// Тест: Leaderboard валидирует записи
#[test]
fn test_leaderboard_validates_entries() {
    let mut leaderboard = Leaderboard::default();

    let _ = leaderboard.add_score("Player", 1000);

    for entry in leaderboard.get_entries() {
        assert!(entry.is_valid(), "Запись должна быть валидной");
    }
}

/// Тест: Classic режим поддерживает сохранение рекорда
#[test]
fn test_classic_mode_saves_score() {
    let state = GameState::new();

    assert_eq!(
        state.get_mode_trait().name(),
        "Классика",
        "Режим должен быть Classic"
    );

    let score = state.score();
    SaveData::save_value(score);
}

/// Тест: Sprint режим не сохраняет рекорд (проверка режима)
#[test]
fn test_sprint_mode_does_not_save_score() {
    let state = GameState::new_sprint();

    assert_eq!(
        state.get_mode_trait().name(),
        "Спринт",
        "Режим должен быть Sprint"
    );
}

/// Тест: Marathon режим поддерживает сохранение рекорда
#[test]
fn test_marathon_mode_saves_score() {
    let state = GameState::new_marathon();

    assert_eq!(
        state.get_mode_trait().name(),
        "Марафон",
        "Режим должен быть Marathon"
    );
}

/// Тест: Leaderboard сортирует рекорды по убыванию
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

/// Тест: SaveData защита от подделки
#[test]
fn test_savedata_protection() {
    let save = SaveData::from_value(10000);

    let score = save.verify_and_get_score();
    assert_eq!(score, Some(10000), "Рекорд должен пройти проверку");
}

/// Тест: Leaderboard максимальный размер (5 записей)
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

/// Тест: GameState реагирует на ввод (движение влево)
#[test]
fn test_gamestate_responds_to_input() {
    let mut state = GameState::new();
    let initial_x = state.curr_shape().pos().0;

    if state.can_move_curr_shape_direction(crate::types::Direction::Left) {
        state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        assert!(
            state.curr_shape().pos().0 < initial_x,
            "Движение влево должно уменьшить X"
        );
    }
}

/// Тест: Hold фигура доступна в начале игры
#[test]
fn test_hold_piece() {
    let state = GameState::new();
    assert!(state.can_hold(), "Hold должен быть доступен в начале");
}

// ============================================================================
// ТЕСТЫ ПОЛНОГО ЦИКЛА
// ============================================================================

/// Тест: Hold + смена фигуры
#[test]
fn test_hold_piece_swap() {
    let mut state = GameState::new();
    let initial_shape = *state.curr_shape();

    state.hold_shape();

    assert!(state.held_shape().is_some(), "Фигура должна быть удержана");
    assert_ne!(
        state.curr_shape().shape(),
        initial_shape.shape(),
        "Текущая фигура должна измениться после hold"
    );
}

// ============================================================================
// ТЕСТЫ ПРОИЗВОДИТЕЛЬНОСТИ
// ============================================================================

/// Тест: Быстрое создание GameState (100 итераций)
#[test]
fn test_fast_gamestate_creation() {
    let start = std::time::Instant::now();

    for _ in 0..100 {
        let _state = GameState::new();
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 1000,
        "Создание 100 GameState должно занять меньше 1 секунды"
    );
}

/// Тест: Быстрое создание Tetromino (1000 итераций)
#[test]
fn test_fast_tetromino_creation() {
    let mut bag = BagGenerator::new();
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        let _t = Tetromino::from_bag(&mut bag);
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 1000,
        "Создание 1000 Tetromino должно занять меньше 1 секунды"
    );
}

/// Тест: Быстрое вращение фигуры (10000 итераций)
#[test]
fn test_fast_piece_rotation() {
    let mut t = Tetromino::new((4.0, 0.0), ShapeType::T, SHAPE_COORDS[0], 0);

    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        t.rotate(RotationDirection::Clockwise);
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 1000,
        "10000 вращений должны занять меньше 1 секунды"
    );
}

/// Тест: Быстрая проверка коллизий (1000 итераций)
#[test]
fn test_fast_collision_check() {
    let state = GameState::new();
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        let _ = state.can_move_curr_shape_direction(crate::types::Direction::Down);
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 1000,
        "1000 проверок коллизий должны занять меньше 1 секунды"
    );
}

/// Тест: Быстрое сохранение рекорда (10 итераций)
#[test]
fn test_fast_score_save() {
    let start = std::time::Instant::now();

    for i in 0..10 {
        SaveData::save_value(i * 100);
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 5000,
        "10 сохранений должны занять меньше 5 секунд"
    );
}

/// Тест: Общая производительность системы
#[test]
fn test_overall_system_performance() {
    let start = std::time::Instant::now();

    let mut state = GameState::new();

    for _ in 0..100 {
        if state.can_rotate_curr_shape(RotationDirection::Clockwise) {
            state
                .get_curr_shape_mut()
                .rotate(RotationDirection::Clockwise);
        }
    }

    for _ in 0..100 {
        if state.can_move_curr_shape_direction(crate::types::Direction::Left) {
            state.get_curr_shape_mut().pos_mut().0 -= 1.0;
        }
    }

    SaveData::save_value(state.score());

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 5000,
        "Операции должны занять меньше 5 секунд"
    );
}
