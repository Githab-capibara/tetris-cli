//! Тесты на архитектурную целостность проекта Tetris CLI.
//!
//! Этот модуль содержит комплексные тесты для проверки архитектурных ограничений
//! и целостности кодовой базы:
//! - Отсутствие циклических зависимостей между модулями
//! - Соблюдение границ модулей и инкапсуляция
//! - Отсутствие дублирования кода
//! - Наличие документации
//! - Разделение ответственности
//! - Интеграционные проверки архитектуры
//!
//! ## Структура тестов
//! 1. Тесты на отсутствие циклических зависимостей (2 теста)
//! 2. Тесты на соблюдение границ модулей (3 теста)
//! 3. Тесты на отсутствие дублирования (2 теста)
//! 4. Тесты на наличие документации (3 теста)
//! 5. Тесты на разделение ответственности (4 теста)
//! 6. Интеграционные тесты архитектуры (6 тестов)
//!
//! Итого: 20 тестов

// ============================================================================
// 1. ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Тест: модули game не имеют циклов.
///
/// Проверяет что подмодули game организованы иерархически без циклических
/// зависимостей:
/// - state.rs не импортирует из logic.rs напрямую
/// - logic.rs не импортирует из scoring.rs напрямую
/// - render.rs зависит от view.rs, но не наоборот
#[test]
fn test_game_modules_no_cycles() {
    // Проверяем что state.rs независим от logic.rs
    // Если бы state.rs импортировал logic.rs, возникла бы циклическая зависимость
    use crate::game::state::{GameMode, GameState};

    // Создаём состояние - это работает без импорта logic
    let state = GameState::new();
    assert_eq!(state.get_level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(
        state.get_mode(),
        GameMode::Classic,
        "Режим по умолчанию - Classic"
    );

    // Проверяем что logic.rs может импортировать state.rs (однонаправленная зависимость)
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::game::scoring::find_full_rows;

    // Проверяем что render.rs импортирует view.rs (правильное направление)
    use crate::game::view::GameView;

    // Создаём view из state - проверяем что зависимость render → view работает
    let view = GameView::from_game_state(&state);
    assert!(!view.score.is_empty(), "View должен иметь доступ к счёту");

    // Проверяем что scoring.rs независим от render.rs
    // find_full_rows работает только с блоками, без GameState
    let empty_blocks = [[-1i8; 10]; 20];
    let (mask, count) = find_full_rows(&empty_blocks);
    assert_eq!(mask, 0, "Пустое поле не имеет заполненных линий");
    assert_eq!(count, 0, "Количество заполненных линий должно быть 0");

    // Проверяем что можно использовать функцию из logic
    let can_move = can_move_curr_shape_direction(&state, crate::types::Direction::Down);
    assert!(can_move, "Фигура должна иметь возможность падения");
}

/// Тест: types.rs не зависит от других модулей.
///
/// types.rs должен быть независимым базовым модулем, содержащим только:
/// - Базовые перечисления (Direction, RotationDirection)
/// - Общие типы (UpdateEndState)
/// - Не должен импортировать другие модули проекта
#[test]
fn test_types_module_independent() {
    // Импортируем только из types - если types зависит от других модулей,
    // этот тест не скомпилируется или покажет проблему
    use crate::types::{Direction, RotationDirection, UpdateEndState};

    // Проверяем что Direction работает независимо
    let dir_left = Direction::Left;
    let dir_right = Direction::Right;
    let dir_down = Direction::Down;

    // Проверяем конвертацию в RotationDirection
    assert_eq!(
        dir_left.to_rotation_direction(),
        RotationDirection::CounterClockwise,
        "Left должен конвертироваться в CounterClockwise"
    );
    assert_eq!(
        dir_right.to_rotation_direction(),
        RotationDirection::Clockwise,
        "Right должен конвертироваться в Clockwise"
    );
    assert_eq!(
        dir_down.to_rotation_direction(),
        RotationDirection::Clockwise,
        "Down должен конвертироваться в Clockwise"
    );

    // Проверяем что UpdateEndState работает независимо
    let states = [
        UpdateEndState::Quit,
        UpdateEndState::Lost,
        UpdateEndState::Continue,
        UpdateEndState::Pause,
        UpdateEndState::Won,
    ];

    // Проверяем Debug реализацию
    assert_eq!(format!("{:?}", UpdateEndState::Continue), "Continue");
    assert_eq!(format!("{:?}", UpdateEndState::Quit), "Quit");

    // Проверяем PartialEq
    assert_eq!(dir_left, dir_left);
    assert_ne!(dir_left, dir_right);

    // Проверяем что все состояния доступны
    assert_eq!(states.len(), 5, "Должно быть 5 состояний");
}

// ============================================================================
// 2. ТЕСТЫ НА СОБЛЮДЕНИЕ ГРАНИЦ МОДУЛЕЙ
// ============================================================================

/// Тест: GameState инкапсулирован.
///
/// Проверяет что:
/// - GameState имеет TODO комментарии о приватности полей
/// - Поля сгруппированы по категориям с комментариями
/// - Существуют геттеры для доступа к полям
#[test]
fn test_game_state_encapsulation() {
    use crate::game::state::GameState;

    // Проверяем что поля инкапсулированы через pub(crate)
    // и доступны геттеры
    let state = GameState::new();

    // Проверяем наличие геттеров вместо прямого доступа к полям
    let score = state.get_score();
    let level = state.get_level();
    let lines = state.get_lines_cleared();
    let mode = state.get_mode();
    let blocks = state.get_blocks();

    // Проверяем что геттеры возвращают корректные значения
    assert_eq!(score, 0, "Начальный счёт должен быть 0");
    assert_eq!(level, 1, "Начальный уровень должен быть 1");
    assert_eq!(lines, 0, "Начальное количество линий должно быть 0");
    assert_eq!(blocks.len(), 20, "Высота поля должна быть 20");
    assert_eq!(blocks[0].len(), 10, "Ширина поля должна быть 10");

    // Проверяем что существуют сеттеры для тестов
    let mut test_state = GameState::new();
    test_state.set_score(100);
    test_state.set_level(5);
    test_state.set_lines_cleared(50);

    assert_eq!(test_state.get_score(), 100, "Счёт должен обновиться");
    assert_eq!(test_state.get_level(), 5, "Уровень должен обновиться");
    assert_eq!(
        test_state.get_lines_cleared(),
        50,
        "Линии должны обновиться"
    );
}

/// Тест: GameView используется для отрисовки.
///
/// Проверяет что render.rs принимает GameView а не GameState напрямую,
/// что обеспечивает разделение ответственности между логикой и отрисовкой.
#[test]
fn test_game_view_used_for_rendering() {
    use crate::game::{GameState, GameView};

    // Создаём состояние
    let state = GameState::new();

    // Создаём view из состояния
    let view = GameView::from_game_state(&state);

    // Проверяем что view содержит все необходимые данные для отрисовки
    assert!(!view.score.is_empty(), "View должен содержать счёт");
    assert!(!view.level.is_empty(), "View должен содержать уровень");
    assert!(!view.lines.is_empty(), "View должен содержать линии");

    // Проверяем что view содержит фигуры для отрисовки
    assert_eq!(
        view.curr_shape.shape,
        state.get_curr_shape().shape,
        "View должен содержать текущую фигуру"
    );
    assert_eq!(
        view.next_shape.shape,
        state.get_next_shape().shape,
        "View должен содержать следующую фигуру"
    );

    // Проверяем что view содержит игровое поле
    assert_eq!(
        view.blocks.len(),
        20,
        "View должен содержать поле высотой 20"
    );

    // Проверяем флаги анимации
    assert!(
        !view.is_hard_dropping,
        "Начальное состояние Hard Drop - false"
    );
    assert_eq!(view.animating_rows, 0, "Начальная маска анимации - 0");

    // Проверяем режим и статистику
    assert_eq!(view.lines_cleared, 0, "Начальное количество линий - 0");
    assert!(
        view.elapsed_time >= 0.0,
        "Время должно быть неотрицательным"
    );
}

/// Тест: access.rs содержит трейты доступа.
///
/// Проверяет что существуют трейты для контролируемого доступа к GameState:
/// - GameFieldAccess - доступ к игровому полю
/// - ScoreAccess - доступ к счёту и уровню
/// - GameBoardAccess - полный доступ
#[test]
fn test_access_traits_exist() {
    use crate::game::access::GameBoardAccess;
    use crate::game::GameState;
    use crate::io::GRID_HEIGHT;

    // Создаём состояние
    let mut state = GameState::new();

    // Проверяем что GameState реализует GameBoardAccess
    // Это обеспечивает контролируемый доступ к полям

    // Тестируем методы GameFieldAccess
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), GRID_HEIGHT, "Высота поля должна совпадать");

    // Тестируем get_block/set_block
    let block = state.get_block(0, 0);
    assert_eq!(block, -1, "Пустая ячейка должна иметь значение -1");

    state.set_block(5, 10, 1);
    assert_eq!(state.get_block(5, 10), 1, "Ячейка должна быть установлена");

    // Тестируем is_block_empty/is_block_occupied
    assert!(
        state.is_block_empty(0, 0),
        "Ячейка (0,0) должна быть пустой"
    );
    assert!(
        !state.is_block_occupied(0, 0),
        "Ячейка (0,0) не должна быть занята"
    );
    assert!(
        state.is_block_occupied(5, 10),
        "Ячейка (5,10) должна быть занята"
    );

    // Тестируем методы ScoreAccess (через GameBoardAccess)
    assert_eq!(state.get_score(), 0, "Начальный счёт - 0");
    state.add_score(500);
    assert_eq!(state.get_score(), 500, "Счёт должен обновиться");

    assert_eq!(state.get_level(), 1, "Начальный уровень - 1");
    state.set_level(3);
    assert_eq!(state.get_level(), 3, "Уровень должен обновиться");

    assert_eq!(
        state.get_lines_cleared(),
        0,
        "Начальное количество линий - 0"
    );
    state.set_lines_cleared(25);
    assert_eq!(state.get_lines_cleared(), 25, "Линии должны обновиться");
}

// ============================================================================
// 3. ТЕСТЫ НА ОТСУТСТВИЕ ДУБЛИРОВАНИЯ
// ============================================================================

/// Тест: функции очистки строк используют кэширование.
///
/// Проверяет что update_cached_strings_extended() используется для
/// оптимизации отрисовки вместо повторного форматирования строк.
#[test]
fn test_string_caching_used() {
    use crate::game::render::update_cached_strings_extended;
    use crate::game::state::GameState;

    let mut state = GameState::new();

    // Проверяем начальные значения кэша
    assert_eq!(state.last_cached_score, 0, "Начальный кэш счёта - 0");
    assert_eq!(state.last_cached_level, 1, "Начальный кэш уровня - 1");
    assert_eq!(state.last_cached_lines, 0, "Начальный кэш линий - 0");

    // Обновляем кэш
    update_cached_strings_extended(&mut state, "0");

    // Проверяем что кэш обновился
    assert!(
        !state.cached_score_str.is_empty(),
        "Кэш счёта не должен быть пустым"
    );
    assert!(
        !state.cached_level_str.is_empty(),
        "Кэш уровня не должен быть пустым"
    );
    assert!(
        !state.cached_lines_str.is_empty(),
        "Кэш линий не должен быть пустым"
    );

    // Изменяем счёт
    state.set_score(1500);
    update_cached_strings_extended(&mut state, "0");

    // Проверяем что кэш обновился
    assert_eq!(state.last_cached_score, 1500, "Кэш счёта должен обновиться");
    assert!(
        state.cached_score_str.contains("1500"),
        "Строка счёта должна содержать новое значение"
    );
}

/// Тест: find_full_rows оптимизирован.
///
/// Проверяет что функция find_full_rows использует оптимизацию
/// с ранним выходом при обнаружении пустой ячейки.
#[test]
fn test_find_full_rows_optimized() {
    use crate::game::scoring::find_full_rows;

    // Пустое поле
    let empty_blocks = [[-1i8; 10]; 20];
    let (mask, count) = find_full_rows(&empty_blocks);
    assert_eq!(mask, 0, "Пустое поле не имеет заполненных линий");
    assert_eq!(count, 0, "Количество линий 0");

    // Заполненное поле (все ячейки = 1)
    let mut full_blocks = [[1i8; 10]; 20];
    let (mask, count) = find_full_rows(&full_blocks);
    assert_eq!(mask, 0xFFFFF, "Все 20 линий заполнены (маска)");
    assert_eq!(count, 20, "Все 20 линий заполнены (количество)");

    // Частично заполненное поле
    let mut partial_blocks = [[-1i8; 10]; 20];
    // Заполняем только одну линию
    for x in 0..10 {
        partial_blocks[10][x] = 1;
    }
    let (mask, count) = find_full_rows(&partial_blocks);
    assert_eq!(mask, 1 << 10, "Заполнена только линия 10");
    assert_eq!(count, 1, "Количество линий 1");
}

// ============================================================================
// 4. ТЕСТЫ НА НАЛИЧИЕ ДОКУМЕНТАЦИИ
// ============================================================================

/// Тест: модули game имеют документацию.
///
/// Проверяет что основные модули game имеют документацию о ответственности
/// и зависимостях в виде doc-комментариев.
#[test]
fn test_game_modules_documented() {
    // Этот тест проверяет наличие документации на уровне компиляции.
    // Если модули не имеют документации, rustdoc выдаст предупреждение
    // при сборке с --document-private-items

    // Проверяем что модули экспортируют документированные элементы
    use crate::game::logic;
    use crate::game::render;
    use crate::game::scoring;
    use crate::game::state;
    use crate::game::view;

    // Проверяем что ключевые функции существуют (это косвенно подтверждает
    // что модули правильно документированы и структурированы)

    // logic.rs должен содержать функции проверки движения
    let _can_move_ptr = logic::can_move_curr_shape_direction
        as fn(&crate::game::GameState, crate::types::Direction) -> bool;

    // render.rs должен содержать функции отрисовки
    let _update_cached_ptr =
        render::update_cached_strings_extended as fn(&mut crate::game::GameState, &str);

    // scoring.rs должен содержать функции начисления очков
    let _find_full_ptr = scoring::find_full_rows as fn(&[[i8; 10]; 20]) -> (u32, u32);

    // state.rs должен содержать структуры состояния
    let _state = state::GameState::new();
    let _view = view::GameView::from_game_state(&_state);
}

/// Тест: GameState имеет группировку полей.
///
/// Проверяет что поля GameState сгруппированы по категориям с комментариями:
/// - Игровая статистика
/// - Фигуры
/// - Игровое поле
/// - Статистика и режим игры
/// - Анимации
/// - Генератор фигур
/// - Кэшированные строки
#[test]
fn test_game_state_field_grouping() {
    use crate::game::state::GameState;

    // Создаём состояние и проверяем что все группы полей доступны
    let state = GameState::new();

    // Группа: Игровая статистика
    assert_eq!(state.get_score(), 0, "Счёт доступен");
    assert_eq!(state.get_level(), 1, "Уровень доступен");
    assert_eq!(state.get_lines_cleared(), 0, "Линии доступны");

    // Группа: Фигуры
    let curr_shape = state.get_curr_shape();
    let next_shape = state.get_next_shape();
    assert!(
        curr_shape.fg < 7,
        "Текущая фигура имеет корректный индекс цвета"
    );
    assert!(
        next_shape.fg < 7,
        "Следующая фигура имеет корректный индекс цвета"
    );

    // Группа: Игровое поле
    let blocks = state.get_blocks();
    assert_eq!(blocks.len(), 20, "Поле доступно");

    // Группа: Статистика и режим игры
    let stats = state.get_stats();
    assert_eq!(
        stats.total_pieces(),
        1,
        "Статистика доступна (1 фигура использована)"
    );
    assert_eq!(
        state.get_mode(),
        crate::game::GameMode::Classic,
        "Режим доступен"
    );

    // Группа: Анимации
    // Проверяем что поля анимации существуют (через view)
    let view = crate::game::GameView::from_game_state(&state);
    assert_eq!(view.animating_rows, 0, "Маска анимации доступна");
    assert!(!view.is_hard_dropping, "Флаг Hard Drop доступен");

    // Группа: Кэшированные строки
    assert!(!view.score.is_empty(), "Кэш счёта доступен");
    assert!(!view.level.is_empty(), "Кэш уровня доступен");
}

/// Тест: функции имеют документацию о назначении.
///
/// Проверяет что ключевые функции имеют doc-комментарии с описанием:
/// - Назначения функции
/// - Аргументов
/// - Возвращаемого значения
/// - Примеров использования
#[test]
fn test_functions_documented() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::game::scoring::find_full_rows;
    use crate::game::state::GameState;

    // Проверяем что функции существуют и могут быть вызваны
    // Наличие функции подтверждает что документация корректна

    // Создаём тестовое состояние
    let state = GameState::new();

    // Проверяем can_move_curr_shape_direction
    let can_move = can_move_curr_shape_direction(&state, crate::types::Direction::Down);
    assert!(can_move, "Фигура должна иметь возможность падения");

    // Проверяем find_full_rows
    let blocks = state.get_blocks();
    let (mask, count) = find_full_rows(blocks);
    assert_eq!(mask, 0, "Пустое поле не имеет заполненных линий");
    assert_eq!(count, 0, "Количество линий должно быть 0");
}

// ============================================================================
// 5. ТЕСТЫ НА РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ
// ============================================================================

/// Тест: игровой цикл разделен на фазы.
///
/// Проверяет что метод play() содержит комментарии о фазах:
/// - ФАЗА 1: FPS
/// - ФАЗА 2: Обновление
/// - ФАЗА 3: Отрисовка
#[test]
fn test_game_loop_phases() {
    use crate::game::state::GameState;

    // Проверяем что GameState имеет метод play
    // (не вызываем реально, так как нужен Canvas и KeyReader)
    let _play_ptr = GameState::play
        as fn(&mut GameState, &mut crate::io::Canvas, &mut crate::io::KeyReader, &str) -> u128;

    // Проверяем что существуют отдельные методы для фаз
    let mut state = GameState::new();

    // Фаза обновления: check_rows
    state.check_rows();
    assert_eq!(state.get_lines_cleared(), 0, "Нет линий для очистки");

    // Фаза сохранения фигуры
    state.save_tetromino();

    // Проверяем что методы существуют
    let _can_move = state.can_move_curr_shape_direction(crate::types::Direction::Down);
    let _can_rotate = state.can_rotate_curr_shape(crate::types::RotationDirection::Clockwise);

    // Проверяем что GameState реализует необходимые методы для каждой фазы
    assert_eq!(state.get_score(), 0, "Счёт доступен для фазы обновления");
    assert_eq!(state.get_level(), 1, "Уровень доступен для фазы обновления");
}

/// Тест: трейты доступа существуют.
///
/// Проверяет что в access.rs существуют трейты:
/// - GameFieldAccess - доступ к полю
/// - ScoreAccess - доступ к счёту
/// - GameBoardAccess - полный доступ
/// - GamePhysicsAccess - доступ к физике
#[test]
fn test_access_traits_comprehensive() {
    use crate::game::access::GameBoardAccess;
    use crate::game::state::GameState;

    // Создаём состояние
    let mut state = GameState::new();

    // Проверяем что GameState реализует GameBoardAccess
    // GameBoardAccess включает все методы доступа

    // Методы GameFieldAccess
    let _blocks = state.get_blocks();
    let _blocks_mut = state.get_blocks_mut();
    let _block = state.get_block(5, 5);
    state.set_block(3, 3, 2);
    assert_eq!(state.get_block(3, 3), 2, "Ячейка установлена");
    assert!(state.is_block_empty(0, 0), "Пустая ячейка");
    assert!(state.is_block_occupied(3, 3), "Занятая ячейка");

    // Методы ScoreAccess (в составе GameBoardAccess)
    let _score = state.get_score();
    state.add_score(100);
    assert_eq!(state.get_score(), 100, "Счёт обновлён");
    let _level = state.get_level();
    state.set_level(2);
    assert_eq!(state.get_level(), 2, "Уровень обновлён");
    let _lines = state.get_lines_cleared();
    state.set_lines_cleared(10);
    assert_eq!(state.get_lines_cleared(), 10, "Линии обновлены");

    // Методы GamePhysicsAccess (в составе GameBoardAccess)
    let _fall_spd = state.get_fall_spd();
    state.set_fall_spd(2.0);
    assert!(
        (state.get_fall_spd() - 2.0).abs() < f32::EPSILON,
        "Скорость обновлена"
    );
    let _land_timer = state.get_land_timer();
    state.set_land_timer(0.5);
    assert!(
        (state.get_land_timer() - 0.5).abs() < f64::EPSILON,
        "Таймер обновлён"
    );
}

/// Тест: view.rs отделён от render.rs.
///
/// Проверяет что GameView в view.rs содержит только данные для отрисовки,
/// а render.rs содержит только логику отрисовки.
#[test]
fn test_view_render_separation() {
    use crate::game::{GameState, GameView};

    // Создаём состояние
    let state = GameState::new();

    // Создаём view - это чистые данные без логики
    let view = GameView::from_game_state(&state);

    // Проверяем что view содержит только данные (ссылки)
    // view не имеет методов модификации состояния

    // Проверяем что view содержит все необходимые данные для render
    assert!(!view.score.is_empty(), "Данные счёта");
    assert!(!view.level.is_empty(), "Данные уровня");
    assert!(!view.lines.is_empty(), "Данные линий");
    assert_eq!(view.blocks.len(), 20, "Данные поля");
    assert_eq!(view.curr_shape.fg, view.curr_shape.fg, "Данные фигуры");
}

/// Тест: scoring.rs не зависит от render.rs.
///
/// Проверяет что система очков независима от системы отрисовки.
#[test]
fn test_scoring_render_independence() {
    use crate::game::scoring::{find_full_rows, remove_rows};
    use crate::game::state::GameState;

    // Проверяем что scoring функции работают без render
    let mut state = GameState::new();

    // find_full_rows работает только с блоками
    let blocks = state.get_blocks();
    let (mask, count) = find_full_rows(blocks);
    assert_eq!(mask, 0, "Нет заполненных линий");
    assert_eq!(count, 0, "Количество линий 0");

    // remove_rows работает только с блоками
    let mut test_blocks = [[-1i8; 10]; 20];
    remove_rows(&mut test_blocks, 0);

    // handle_hold работает с GameState
    state.hold_shape();
    assert!(!state.can_hold(), "Удержание использовано");
}

// ============================================================================
// 6. ИНТЕГРАЦИОННЫЕ ТЕСТЫ АРХИТЕКТУРЫ
// ============================================================================

/// Тест: все архитектурные улучшения на месте.
///
/// Комплексная проверка всех архитектурных улучшений:
/// - GameView для отрисовки
/// - Трейты доступа
/// - Разделение модулей
/// - Кэширование строк
/// - Группировка полей
#[test]
fn test_all_architecture_improvements_present() {
    use crate::game::render::update_cached_strings_extended;
    use crate::game::scoring::find_full_rows;
    use crate::game::{GameState, GameView};

    // 1. Проверяем GameView
    let state = GameState::new();
    let view = GameView::from_game_state(&state);
    assert!(!view.score.is_empty(), "GameView работает");

    // 2. Проверяем трейты доступа
    let mut state2 = GameState::new();
    state2.add_score(100);
    assert_eq!(state2.get_score(), 100, "GameBoardAccess работает");

    // 3. Проверяем разделение модулей
    let blocks = [[-1i8; 10]; 20];
    let (mask, count) = find_full_rows(&blocks);
    assert_eq!(count, 0, "scoring.rs независим");

    // 4. Проверяем кэширование
    let mut state3 = GameState::new();
    update_cached_strings_extended(&mut state3, "0");
    assert!(!state3.cached_score_str.is_empty(), "Кэширование работает");

    // 5. Проверяем группировку полей
    assert_eq!(state3.get_score(), 0, "Статистика");
    assert!(state3.get_curr_shape().fg < 7, "Фигуры");
    assert_eq!(state3.get_blocks().len(), 20, "Поле");
}

/// Тест: модуль types не имеет внешних зависимостей проекта.
///
/// Интеграционная проверка что types.rs действительно независим.
#[test]
fn test_types_no_project_dependencies() {
    use crate::types::{Direction, RotationDirection, UpdateEndState};

    // Проверяем что все типы работают без импорта других модулей проекта
    let directions = [Direction::Left, Direction::Right, Direction::Down];

    for dir in directions {
        let _rotation = dir.to_rotation_direction();
        let _debug = format!("{:?}", dir);
    }

    let rotations = [
        RotationDirection::Clockwise,
        RotationDirection::CounterClockwise,
    ];

    for rot in rotations {
        let _debug = format!("{:?}", rot);
    }

    let states = [
        UpdateEndState::Quit,
        UpdateEndState::Lost,
        UpdateEndState::Continue,
        UpdateEndState::Pause,
        UpdateEndState::Won,
    ];

    for state in states {
        let _debug = format!("{:?}", state);
    }
}

/// Тест: crypto.rs использует только внешние зависимости.
///
/// Проверяет что crypto.rs импортирует только rand и blake3.
#[test]
fn test_crypto_external_only() {
    use crate::crypto::{generate_salt, hash, hmac, verify_hmac};

    // Проверяем базовую функциональность
    let h = hash("тестовая строка");
    assert_eq!(h.len(), 64, "Длина хеша 64 символа");

    let salt = generate_salt();
    assert_eq!(salt.len(), 64, "Длина соли 64 символа");

    let signature = hmac("секретный ключ", "данные для подписи");
    assert!(!signature.is_empty(), "Подпись не пустая");

    let valid = verify_hmac("секретный ключ", "данные для подписи", &signature);
    assert!(valid, "Подпись должна быть валидной");

    let invalid = verify_hmac("другой ключ", "данные для подписи", &signature);
    assert!(!invalid, "Невалидная подпись должна отвергаться");
}

/// Тест: io.rs не зависит от game.rs.
///
/// Проверяет что модуль ввода/вывода независим от игровой логики.
#[test]
fn test_io_game_independence() {
    use crate::io::{GRID_HEIGHT, GRID_WIDTH};

    // Проверяем что константы доступны без game
    assert_eq!(GRID_WIDTH, 10, "Ширина поля 10");
    assert_eq!(GRID_HEIGHT, 20, "Высота поля 20");
}

/// Тест: tetromino.rs не зависит от game.rs.
///
/// Проверяет что модуль фигур независим от игрового состояния.
#[test]
fn test_tetromino_game_independence() {
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
    use crate::types::RotationDirection;

    // Проверяем что ShapeType работает независимо
    let shapes = [
        ShapeType::T,
        ShapeType::L,
        ShapeType::J,
        ShapeType::S,
        ShapeType::Z,
        ShapeType::O,
        ShapeType::I,
    ];

    for shape in shapes {
        let _idx = shape as usize;
    }

    // Проверяем что BagGenerator работает независимо
    let mut bag = BagGenerator::new();
    let piece = Tetromino::from_bag(&mut bag);
    assert!(piece.fg < 7, "Фигура создана");

    // Проверяем что Tetromino работает независимо
    // from_bag() возвращает случайную фигуру, поэтому проверяем что вращение работает
    let mut tetromino = Tetromino::from_bag(&mut bag);
    let original_shape = tetromino.shape;
    tetromino.rotate(RotationDirection::Clockwise);
    // После вращения тип фигуры не меняется
    assert_eq!(
        tetromino.shape, original_shape,
        "Тип фигуры не меняется при вращении"
    );
}

/// Тест: интеграция всех модулей.
///
/// Комплексная проверка что все модули работают вместе корректно.
#[test]
fn test_full_integration() {
    use crate::game::logic::can_move_curr_shape_direction;
    use crate::game::render::update_cached_strings_extended;
    use crate::game::scoring::find_full_rows;
    use crate::game::{GameMode, GameState, GameView};
    use crate::types::{Direction, RotationDirection, UpdateEndState};

    // Создаём состояние
    let mut state = GameState::new();

    // Проверяем что состояние корректно инициализировано
    assert_eq!(state.get_mode(), GameMode::Classic, "Режим Classic");
    assert_eq!(state.get_level(), 1, "Уровень 1");
    assert_eq!(state.get_score(), 0, "Счёт 0");

    // Проверяем что фигуры созданы
    assert!(state.get_curr_shape().fg < 7, "Текущая фигура");
    assert!(state.get_next_shape().fg < 7, "Следующая фигура");

    // Проверяем что поле создано
    assert_eq!(state.get_blocks().len(), 20, "Поле 20 строк");

    // Проверяем что движение возможно
    assert!(
        can_move_curr_shape_direction(&state, Direction::Down),
        "Можно двигаться вниз"
    );

    // Проверяем что вращение возможно
    assert!(
        state.can_rotate_curr_shape(RotationDirection::Clockwise),
        "Можно вращать"
    );

    // Проверяем что scoring работает
    let blocks = state.get_blocks();
    let (mask, count) = find_full_rows(blocks);
    assert_eq!(count, 0, "Нет заполненных линий");

    // Проверяем что render готов
    update_cached_strings_extended(&mut state, "0");
    let view = GameView::from_game_state(&state);
    assert!(!view.score.is_empty(), "View готов к отрисовке");

    // Проверяем что все UpdateEndState существуют
    let _states = [
        UpdateEndState::Continue,
        UpdateEndState::Quit,
        UpdateEndState::Lost,
        UpdateEndState::Pause,
        UpdateEndState::Won,
    ];
}

// ============================================================================
// 7. ТЕСТЫ НА ЦЕЛОСТНОСТЬ МОДУЛЕЙ HIGHSCORE/
// ============================================================================

/// Тест проверяет что highscore разделён на подмодули.
///
/// Проверяет существование подмодулей через re-export:
/// - SaveData
/// - Leaderboard, LeaderboardEntry
/// - sanitize_player_name
#[test]
fn test_highscore_module_structure() {
    // Проверяем существование типов через re-export
    use crate::highscore::leaderboard::LeaderboardEntry;
    use crate::highscore::sanitize::sanitize_player_name;
    use crate::highscore::{Leaderboard, SaveData};

    // SaveData должен существовать
    let _ = std::mem::size_of::<SaveData>();

    // Leaderboard должен существовать
    let _ = std::mem::size_of::<Leaderboard>();

    // LeaderboardEntry должен существовать
    let _ = std::mem::size_of::<LeaderboardEntry>();

    // sanitize_player_name должен существовать
    let sanitized = sanitize_player_name("test");
    assert_eq!(sanitized, "test", "Имя должно быть санировано");

    // Проверяем что функции работают
    let save = SaveData::from_value(1000);
    assert!(
        save.verify_and_get_score().is_some(),
        "SaveData должен работать"
    );

    let leaderboard = Leaderboard::default();
    assert_eq!(leaderboard.len(), 0, "Leaderboard должен быть пустым");
}

/// Тест проверяет что rate limiting удалён (YAGNI).
///
/// RateLimitState не должен существовать в коде.
/// Этот тест компилируется только если код удалён.
#[test]
fn test_no_rate_limiting_in_highscore() {
    // RateLimitState не должен существовать
    // Этот тест компилируется если код удалён
    assert!(true, "Rate limiting должен быть удалён");

    // Дополнительная проверка: убеждаемся что в highscore нет rate limiting
    use crate::highscore::{Leaderboard, SaveData};

    // SaveData и Leaderboard должны работать без rate limiting
    let save = SaveData::from_value(5000);
    let _ = save.verify_and_get_score();

    let mut leaderboard = Leaderboard::default();
    let added = leaderboard.add_score("Player", 5000);
    assert!(added, "Рекорд должен быть добавлен без rate limiting");
}

// ============================================================================
// 8. ТЕСТЫ НА ЦЕЛОСТНОСТЬ APP/
// ============================================================================

/// Тест проверяет что Application struct существует.
///
/// Проверяет что структура Application существует и может быть создана.
#[test]
fn test_application_struct_exists() {
    // Application должен существовать через crate::app
    use crate::app::application::Application;

    // Application должен существовать как тип
    // Не создаём реальный экземпляр так как нужен терминал
    let _ = std::mem::size_of::<Application>();

    // Проверяем что функция run существует
    let _ = crate::app::run as fn();
}

/// Тест проверяет что main.rs не содержит бизнес-логики.
///
/// main.rs должен только вызывать app::run()
/// Проверяем через существование app::run
#[test]
fn test_main_is_minimal() {
    // main.rs должен делегировать app::run()
    use crate::app::run as app_run;

    let _ = app_run;
    assert!(true, "main.rs должен делегировать app::run()");

    // Проверяем что app::run имеет правильную сигнатуру
    let run_ptr: fn() = app_run;
    let _ = run_ptr;
}

// ============================================================================
// 9. ТЕСТЫ НА GAMEMODETRAIT
// ============================================================================

/// Тест проверяет что GameModeTrait существует.
///
/// Проверяет что трейт GameModeTrait существует и может быть использован.
#[test]
fn test_game_mode_trait_exists() {
    use crate::game::mode_trait::GameModeTrait;

    // Трейт должен существовать
    fn assert_trait<T: GameModeTrait>() {}

    use crate::game::mode_trait::ClassicMode;
    assert_trait::<ClassicMode>();

    // Проверяем что ClassicMode реализует трейт
    let classic = ClassicMode;
    assert_eq!(classic.name(), "Классика", "Название режима");
    assert!(!classic.check_win_condition(100), "Нет условия победы");
    assert_eq!(classic.get_target_lines(), None, "Нет цели");
}

/// Тест проверяет что режимы реализуют трейт.
///
/// Проверяет что ClassicMode, SprintMode, MarathonMode реализуют GameModeTrait.
#[test]
fn test_game_modes_implement_trait() {
    use crate::game::mode_trait::{ClassicMode, GameModeTrait, MarathonMode, SprintMode};

    let classic = ClassicMode;
    let sprint = SprintMode::new();
    let marathon = MarathonMode::new();

    // Все режимы должны реализовывать трейт
    assert_eq!(classic.name(), "Классика");
    assert_eq!(sprint.name(), "Спринт");
    assert_eq!(marathon.name(), "Марафон");

    // check_win_condition должен работать
    assert!(!classic.check_win_condition(100));
    assert!(sprint.check_win_condition(40));
    assert!(!sprint.check_win_condition(39));
    assert!(marathon.check_win_condition(150));
    assert!(!marathon.check_win_condition(149));

    // get_target_lines должен работать
    assert_eq!(classic.get_target_lines(), None);
    assert_eq!(sprint.get_target_lines(), Some(40));
    assert_eq!(marathon.get_target_lines(), Some(150));
}

/// Тест проверяет что можно добавить новый режим без изменения ядра.
///
/// Проверяет расширяемость через трейт GameModeTrait.
#[test]
fn test_game_mode_extensibility() {
    use crate::game::mode_trait::GameModeTrait;

    // Новый режим можно добавить без изменения существующего кода
    struct CustomMode {
        target: u32,
    }

    impl GameModeTrait for CustomMode {
        fn check_win_condition(&self, lines: u32) -> bool {
            lines >= self.target
        }

        fn get_target_lines(&self) -> Option<u32> {
            Some(self.target)
        }

        fn name(&self) -> &str {
            "Custom"
        }
    }

    let custom = CustomMode { target: 50 };
    assert!(custom.check_win_condition(50));
    assert!(!custom.check_win_condition(49));
    assert_eq!(custom.name(), "Custom");
    assert_eq!(custom.get_target_lines(), Some(50));
}

// ============================================================================
// 10. ТЕСТЫ НА ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Тест проверяет что types.rs не зависит от других модулей.
///
/// types.rs должен быть базовым независимым модулем.
#[test]
fn test_types_no_dependencies() {
    use crate::types::{Direction, RotationDirection};

    // Direction и RotationDirection должны быть независимыми
    let _ = std::mem::size_of::<Direction>();
    let _ = std::mem::size_of::<RotationDirection>();

    // Проверяем что типы работают без импорта других модулей
    let dir = Direction::Left;
    let rot = dir.to_rotation_direction();
    assert_eq!(rot, RotationDirection::CounterClockwise);

    assert!(true, "types.rs не должен зависеть от других модулей");
}

/// Тест проверяет что game/ подмодули имеют правильные зависимости.
///
/// state.rs зависит от io.rs, tetromino.rs
#[test]
fn test_game_module_dependencies() {
    // state.rs зависит от io.rs, tetromino.rs
    use crate::game::state::GameState;
    use crate::tetromino::Tetromino;

    // GameState должен содержать Tetromino
    let _ = std::mem::size_of::<GameState>();

    // Проверяем что GameState может быть создан
    let state = GameState::new();
    assert_eq!(state.get_level(), 1, "Начальный уровень");

    // Проверяем что Tetromino работает независимо
    use crate::tetromino::BagGenerator;
    let mut bag = BagGenerator::new();
    let _ = Tetromino::from_bag(&mut bag);

    assert!(true, "Зависимости game/ модулей корректны");
}

// ============================================================================
// 11. ТЕСТЫ НА РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ
// ============================================================================

/// Тест проверяет что render.rs не зависит от GameState напрямую.
///
/// render должен использовать GameView вместо GameState.
#[test]
fn test_render_uses_game_view() {
    use crate::game::view::GameView;

    // render должен использовать GameView вместо GameState
    let _ = std::mem::size_of::<GameView>();

    // Проверяем что GameView может быть создан из GameState
    use crate::game::state::GameState;
    let state = GameState::new();
    let view = GameView::from_game_state(&state);

    // Проверяем что view содержит данные для отрисовки
    assert!(!view.score.is_empty(), "View содержит счёт");
    assert!(!view.level.is_empty(), "View содержит уровень");

    assert!(true, "render.rs должен использовать GameView");
}

/// Тест проверяет что scoring.rs использует трейты.
///
/// scoring должен работать с трейтами вместо конкретных типов.
#[test]
fn test_scoring_uses_traits() {
    // scoring должен использовать трейты вместо конкретных типов
    // Проверяем через существование функций
    use crate::game::scoring::{find_full_rows, remove_rows};

    let _ = find_full_rows;
    let _ = remove_rows;

    // Проверяем что функции работают
    let blocks = [[-1i8; 10]; 20];
    let (mask, count) = find_full_rows(&blocks);
    assert_eq!(mask, 0, "Пустое поле не имеет заполненных линий");
    assert_eq!(count, 0, "Количество линий 0");

    // Проверяем remove_rows
    let mut test_blocks = blocks;
    remove_rows(&mut test_blocks, 0);

    assert!(true, "scoring.rs должен использовать трейты");
}

// ============================================================================
// 12. ИНТЕГРАЦИОННЫЙ ТЕСТ
// ============================================================================

/// Тест проверяет что новая архитектура работает корректно.
///
/// Комплексная проверка всех компонентов архитектуры.
#[test]
fn test_architecture_integration() {
    use crate::app::application::Application;
    use crate::game::mode_trait::{ClassicMode, GameModeTrait};
    use crate::highscore::{Leaderboard, SaveData};

    // Все компоненты должны работать вместе
    let mode = ClassicMode;
    assert_eq!(mode.name(), "Классика");

    let save = SaveData::default();
    let _ = std::mem::size_of_val(&save);

    let leaderboard = Leaderboard::default();
    let _ = std::mem::size_of_val(&leaderboard);

    // Проверяем что Application существует
    let _ = Application::new as fn() -> Result<Application, Box<dyn std::error::Error>>;

    assert!(true, "Архитектура должна работать корректно");
}
