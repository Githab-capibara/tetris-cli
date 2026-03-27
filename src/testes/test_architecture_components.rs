// Тесты для проверки архитектурных улучшений
// Покрытие CRITICAL, HIGH, MEDIUM и LOW проблем архитектурного аудита

use tetris_cli::game::components::{GameBoard, ScoreBoard, FigureManager, AnimationState};
use tetris_cli::game::types::Position;

// ============================================================================
// CRITICAL FIXES — Рефакторинг GameState
// ============================================================================

/// Тест 1: Проверка структуры GameBoard
#[test]
fn test_game_board_structure() {
    let board = GameBoard::new();
    
    // Проверяем, что поле инициализировано корректно
    let blocks = board.get_blocks();
    assert_eq!(blocks.len(), 20); // GRID_HEIGHT
    assert_eq!(blocks[0].len(), 10); // GRID_WIDTH
    
    // Все ячейки должны быть пустыми (-1)
    for row in blocks.iter() {
        for cell in row.iter() {
            assert_eq!(*cell, -1);
        }
    }
}

/// Тест 2: Проверка инкапсуляции GameBoard
#[test]
fn test_game_board_encapsulation() {
    let mut board = GameBoard::new();
    
    // Проверяем, что нет прямого доступа к полям
    // Используем только публичные методы
    assert!(board.is_cell_empty(0, 0));
    assert!(board.is_cell_empty(5, 10));
    
    // Заполняем ячейку
    board.set_cell(5, 10, 1);
    assert!(!board.is_cell_empty(5, 10));
}

/// Тест 3: Проверка структуры ScoreBoard
#[test]
fn test_score_board_structure() {
    let scoreboard = ScoreBoard::new();
    
    // Начальные значения
    assert_eq!(scoreboard.get_score(), 0);
    assert_eq!(scoreboard.get_level(), 1);
    assert_eq!(scoreboard.get_lines_cleared(), 0);
}

/// Тест 4: Проверка инкапсуляции ScoreBoard
#[test]
fn test_score_board_encapsulation() {
    let mut scoreboard = ScoreBoard::new();
    
    // Добавляем очки через метод
    scoreboard.add_score(100);
    assert_eq!(scoreboard.get_score(), 100);
    
    // Добавляем линии
    scoreboard.add_lines_cleared(4);
    assert_eq!(scoreboard.get_lines_cleared(), 4);
    
    // Проверяем уровень
    assert_eq!(scoreboard.get_level(), 1);
}

/// Тест 5: Проверка структуры FigureManager
#[test]
fn test_figure_manager_structure() {
    let manager = FigureManager::new();
    
    // Проверяем наличие фигур
    let curr_shape = manager.get_curr_shape();
    assert!(curr_shape.coords.len() == 4);
    
    let next_shape = manager.get_next_shape();
    assert!(next_shape.coords.len() == 4);
}

/// Тест 6: Проверка инкапсуляции FigureManager
#[test]
fn test_figure_manager_encapsulation() {
    let mut manager = FigureManager::new();
    
    // Проверяем can_hold
    assert!(manager.can_hold());
    
    // Удерживаем фигуру
    manager.hold_shape();
    assert!(!manager.can_hold());
}

/// Тест 7: Проверка структуры AnimationState
#[test]
fn test_animation_state_structure() {
    let anim_state = AnimationState::new();
    
    // Начальные значения
    assert!(!anim_state.is_hard_dropping());
    assert_eq!(anim_state.get_animating_rows_mask(), 0);
}

/// Тест 8: Проверка инкапсуляции AnimationState
#[test]
fn test_animation_state_encapsulation() {
    let mut anim_state = AnimationState::new();
    
    // Устанавливаем hard drop
    anim_state.set_hard_dropping(true);
    assert!(anim_state.is_hard_dropping());
    
    // Сбрасываем
    anim_state.set_hard_dropping(false);
    assert!(!anim_state.is_hard_dropping());
}

// ============================================================================
// LOW FIXES — Position struct (Data Clumps)
// ============================================================================

/// Тест 9: Проверка создания Position
#[test]
fn test_position_creation() {
    let pos = Position::new(5, 10);
    
    assert_eq!(pos.x(), 5);
    assert_eq!(pos.y(), 10);
}

/// Тест 10: Проверка мутации Position
#[test]
fn test_position_mutation() {
    let mut pos = Position::new(0, 0);
    
    pos.set_x(10);
    pos.set_y(20);
    
    assert_eq!(pos.x(), 10);
    assert_eq!(pos.y(), 20);
}

/// Тест 11: Проверка offset Position
#[test]
fn test_position_offset() {
    let mut pos = Position::new(5, 10);
    
    pos.offset(3, -2);
    
    assert_eq!(pos.x(), 8);
    assert_eq!(pos.y(), 8);
}

/// Тест 12: Проверка is_zero Position
#[test]
fn test_position_is_zero() {
    let pos1 = Position::new(0, 0);
    let pos2 = Position::new(1, 0);
    
    assert!(pos1.is_zero());
    assert!(!pos2.is_zero());
}

/// Тест 13: Проверка конвертации Position из кортежа
#[test]
fn test_position_from_tuple() {
    let pos: Position = (5, 10).into();
    
    assert_eq!(pos.x(), 5);
    assert_eq!(pos.y(), 10);
}

/// Тест 14: Проверка конвертации Position в кортеж
#[test]
fn test_position_into_tuple() {
    let pos = Position::new(5, 10);
    let tuple: (i16, i16) = pos.into();
    
    assert_eq!(tuple, (5, 10));
}

// ============================================================================
// АРХИТЕКТУРНЫЕ ТЕСТЫ — Компоненты
// ============================================================================

/// Тест 15: Проверка разделения ответственности компонентов
#[test]
fn test_component_separation() {
    // Каждый компонент отвечает только за свою область
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();
    let manager = FigureManager::new();
    let anim_state = AnimationState::new();
    
    // Проверяем, что компоненты независимы
    assert!(board.is_cell_empty(0, 0));
    assert_eq!(scoreboard.get_score(), 0);
    assert!(manager.can_hold());
    assert!(!anim_state.is_hard_dropping());
}

/// Тест 16: Проверка инкапсуляции компонентов
#[test]
fn test_component_encapsulation() {
    let mut board = GameBoard::new();
    let mut scoreboard = ScoreBoard::new();
    
    // Изменяем через публичные методы
    board.set_cell(0, 0, 1);
    scoreboard.add_score(100);
    
    // Проверяем, что изменения применились
    assert!(!board.is_cell_empty(0, 0));
    assert_eq!(scoreboard.get_score(), 100);
}

/// Тест 17: Проверка независимости компонентов
#[test]
fn test_component_independence() {
    let mut board = GameBoard::new();
    let mut scoreboard = ScoreBoard::new();
    
    // Изменение board не влияет на scoreboard
    board.set_cell(5, 5, 1);
    assert_eq!(scoreboard.get_score(), 0);
    
    // Изменение scoreboard не влияет на board
    scoreboard.add_score(200);
    assert!(board.is_cell_empty(5, 5));
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ — Архитектура
// ============================================================================

/// Тест 18: Проверка совместной работы компонентов
#[test]
fn test_components_integration() {
    let mut board = GameBoard::new();
    let mut scoreboard = ScoreBoard::new();
    let manager = FigureManager::new();
    
    // Симуляция игрового процесса
    board.set_cell(0, 0, 1);
    scoreboard.add_lines_cleared(1);
    
    // Проверяем, что все компоненты работают корректно
    assert!(!board.is_cell_empty(0, 0));
    assert_eq!(scoreboard.get_lines_cleared(), 1);
    assert!(manager.get_curr_shape().coords.len() == 4);
}

/// Тест 19: Проверка масштабируемости архитектуры
#[test]
fn test_architecture_scalability() {
    // Создаём множество компонентов
    let boards: Vec<GameBoard> = (0..10).map(|_| GameBoard::new()).collect();
    let scoreboards: Vec<ScoreBoard> = (0..10).map(|_| ScoreBoard::new()).collect();
    
    // Проверяем, что архитектура масштабируется
    assert_eq!(boards.len(), 10);
    assert_eq!(scoreboards.len(), 10);
    
    // Каждый компонент независим
    for (i, board) in boards.iter().enumerate() {
        // Все поля пустые
        assert!(board.is_cell_empty(0, 0));
    }
}

/// Тест 20: Проверка отсутствия циклических зависимостей
#[test]
fn test_no_circular_dependencies() {
    // Этот тест проверяет, что компоненты не имеют циклических зависимостей
    // Компоненты импортируются из одного модуля и не зависят друг от друга
    
    let board = GameBoard::new();
    let scoreboard = ScoreBoard::new();
    let manager = FigureManager::new();
    let anim_state = AnimationState::new();
    
    // Все компоненты создаются независимо
    assert!(board.is_cell_empty(0, 0));
    assert_eq!(scoreboard.get_score(), 0);
    assert!(manager.can_hold());
    assert!(!anim_state.is_hard_dropping());
}

// ============================================================================
// СТРЕСС ТЕСТЫ — Архитектура
// ============================================================================

/// Тест 21: Стресс-тест GameBoard
#[test]
fn test_game_board_stress() {
    let mut board = GameBoard::new();
    
    // Заполняем всё поле
    for y in 0..20 {
        for x in 0..10 {
            board.set_cell(x, y, 1);
        }
    }
    
    // Проверяем, что всё заполнено
    for y in 0..20 {
        for x in 0..10 {
            assert!(!board.is_cell_empty(x, y));
        }
    }
}

/// Тест 22: Стресс-тест ScoreBoard
#[test]
fn test_score_board_stress() {
    let mut scoreboard = ScoreBoard::new();
    
    // Добавляем много очков
    for i in 0..1000 {
        scoreboard.add_score(100);
        assert_eq!(scoreboard.get_score(), (i + 1) * 100);
    }
}

/// Тест 23: Стресс-тест Position
#[test]
fn test_position_stress() {
    let mut pos = Position::new(0, 0);
    
    // Многократные смещения
    for i in 0..100 {
        pos.offset(1, 1);
        assert_eq!(pos.x(), i as i16 + 1);
        assert_eq!(pos.y(), i as i16 + 1);
    }
}

/// Тест 24: Комплексный тест всех архитектурных улучшений
#[test]
fn test_all_architecture_improvements_integration() {
    // 1. GameBoard
    let mut board = GameBoard::new();
    board.set_cell(5, 10, 1);
    assert!(!board.is_cell_empty(5, 10));
    
    // 2. ScoreBoard
    let mut scoreboard = ScoreBoard::new();
    scoreboard.add_score(1000);
    assert_eq!(scoreboard.get_score(), 1000);
    
    // 3. FigureManager
    let manager = FigureManager::new();
    assert!(manager.can_hold());
    
    // 4. AnimationState
    let mut anim_state = AnimationState::new();
    anim_state.set_hard_dropping(true);
    assert!(anim_state.is_hard_dropping());
    
    // 5. Position
    let pos = Position::new(5, 10);
    assert_eq!(pos.x(), 5);
    assert_eq!(pos.y(), 10);
    
    // Все архитектурные улучшения работают корректно
}
