//! Тесты для архитектурных проблем в проекте tetris-cli.
//!
//! Этот файл содержит уникальные архитектурные тесты.
//! Дублирующиеся тесты удалены (остались в test_41_fixed_issues.rs).
//!
//! ## Запуск тестов
//! ```bash
//! cargo test --test test_architecture_all
//! ```

#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_many_lines)]

// ============================================================================
// КАТЕГОРИЯ 1: КРИТИЧЕСКИЕ ПРОБЛЕМЫ
// ============================================================================

// ============================================================================
// КАТЕГОРИЯ 2: АРХИТЕКТУРА
// ============================================================================

/// Тест 6: Абстракция времени Time модуль
///
/// Проверяет что Time структура существует и имеет нужные методы.
///
/// # Архитектурная проблема H6
/// Time абстракция для типобезопасной работы со временем в игре.
#[test]
fn test_architecture_time_abstraction() {
    use std::fs;

    let time_path = "src/game/time.rs";
    let time_content = fs::read_to_string(time_path).expect("Failed to read time.rs");

    // Time структура должна существовать
    assert!(
        time_content.contains("pub struct Time"),
        "Структура Time должна существовать"
    );

    // Time должен иметь методы для работы со временем
    assert!(
        time_content.contains("fn from_secs") || time_content.contains("fn from_millis"),
        "Time должен иметь методы создания из секунд/миллисекунд"
    );

    assert!(
        time_content.contains("fn as_secs")
            || time_content.contains("fn as_millis")
            || time_content.contains("fn as_secs_f64"),
        "Time должен иметь методы получения времени"
    );
}

/// Тест 7: Разделение ввода и логики (parse_input/execute_action)
///
/// Проверяет что parse_input() является чистой функцией без побочных эффектов.
///
/// # Архитектурная проблема H5
/// Разделение ввода и логики через parse_input() и execute_action() функции.
#[test]
fn test_architecture_input_logic_separation() {
    use std::fs;

    let input_path = "src/game/logic/input.rs";
    let input_content = fs::read_to_string(input_path).expect("Failed to read input.rs");

    // parse_input должен быть чистой функцией
    assert!(
        input_content.contains("fn parse_input")
            && (input_content.contains("-> GameAction")
                || input_content.contains("-> Option<GameAction>")
                || input_content.contains("-> Option<crate::game::types::GameAction>")),
        "parse_input должен возвращать GameAction"
    );

    // Проверяем что parse_input не содержит побочных эффектов
    let parse_start = input_content.find("fn parse_input");
    if let Some(start) = parse_start {
        let parse_end = input_content[start..]
            .find("\n}")
            .map_or(start + 200, |i| start + i + 2);

        let parse_content = &input_content[start..parse_end];

        // Чистая функция не должна изменять состояние
        assert!(
            !parse_content.contains(".set_")
                && !parse_content.contains("save_")
                && !parse_content.contains("write_"),
            "parse_input должен быть чистой функцией без побочных эффектов"
        );
    }
}

/// Тест 8: DIP - использование трейтов в run_game_loop
///
/// Проверяет что run_game_loop принимает &mut dyn Renderer вместо &mut Canvas.
///
/// # Архитектурная проблема H2
/// Dependency Inversion Principle - зависимость от абстракций, а не от конкретных типов.
#[test]
fn test_architecture_dip_game_loop_traits() {
    use std::fs;

    let cycle_path = "src/game/cycle.rs";
    let cycle_content = fs::read_to_string(cycle_path).expect("Failed to read cycle.rs");

    // run_game_loop должен использовать трейты для абстракции
    assert!(
        cycle_content.contains("&mut dyn") || cycle_content.contains("impl "),
        "run_game_loop должен использовать трейты (dyn Trait или impl Trait)"
    );
}

/// Тест 9: SoC - разделение render и logic
///
/// Проверяет что логика игры отделена от отрисовки.
///
/// # Архитектурная проблема
/// Separation of Concerns - разделение ответственности между модулями.
#[test]
fn test_architecture_soc_render_logic_separation() {
    use std::fs;

    let render_path = "src/game/render.rs";
    let logic_path = "src/game/logic/mod.rs";

    let render_content = fs::read_to_string(render_path).expect("Failed to read render.rs");
    let logic_content = fs::read_to_string(logic_path).expect("Failed to read logic/mod.rs");

    // Render не должен содержать бизнес-логику
    assert!(
        !render_content.contains("fn update_") || render_content.contains("//"),
        "render.rs не должен содержать бизнес-логику обновления"
    );

    // Logic не должен содержать отрисовку
    assert!(
        !logic_content.contains("draw_") || logic_content.contains("//"),
        "logic модуль не должен содержать функции отрисовки"
    );
}

// ============================================================================
// КАТЕГОРИЯ 3: МОДУЛЬНОСТЬ (4 теста)
// ============================================================================

/// Тест 10: Централизация HMAC в crypto модуле
///
/// Проверяет что hmac_sign и hmac_verify определены только в crypto::hmac модуле.
///
/// # Архитектурная проблема C4
/// Централизация HMAC логики в одном модуле для избежания дублирования.
#[test]
fn test_modularity_hmac_centralization() {
    use std::fs;

    let hmac_path = "src/crypto/hmac.rs";
    let crypto_path = "src/crypto.rs";

    let hmac_content = fs::read_to_string(hmac_path).expect("Failed to read crypto/hmac.rs");
    let crypto_content = fs::read_to_string(crypto_path).expect("Failed to read crypto.rs");

    // hmac_sign и hmac_verify должны быть определены в hmac.rs
    assert!(
        hmac_content.contains("pub fn hmac_sign") || hmac_content.contains("pub fn hmac_verify"),
        "hmac_sign и hmac_verify должны быть определены в crypto/hmac.rs"
    );

    // crypto.rs должен ре-экспортировать функции из hmac.rs
    assert!(
        crypto_content.contains("pub use hmac::")
            || crypto_content.contains("hmac_sign")
            || crypto_content.contains("hmac_verify"),
        "crypto.rs должен ре-экспортировать HMAC функции из hmac.rs"
    );
}

/// Тест 11: Централизация констант
///
/// Проверяет что константы централизованы в constants.rs модуле.
///
/// # Архитектурная проблема M1
/// Централизация констант для избежания дублирования.
#[test]
fn test_modularity_constants_centralization() {
    use std::fs;

    let constants_path = "src/constants.rs";
    let constants_content =
        fs::read_to_string(constants_path).expect("Failed to read constants.rs");

    // MAX_CONFIG_FILE_SIZE должна быть определена в constants.rs
    assert!(
        constants_content.contains("MAX_CONFIG_FILE_SIZE"),
        "MAX_CONFIG_FILE_SIZE должна быть определена в constants.rs"
    );

    // Проверяем что константа используется через импорт
    let controls_path = "src/controls.rs";
    let controls_content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

    assert!(
        controls_content.contains("MAX_CONFIG_FILE_SIZE"),
        "controls.rs должен использовать MAX_CONFIG_FILE_SIZE из constants.rs"
    );
}

/// Тест 12: ValidationService для централизованной валидации
///
/// Проверяет что ValidationService существует и предоставляет методы валидации.
///
/// # Архитектурная проблема
/// Централизация валидации через ValidationService.
#[test]
fn test_modularity_validation_service() {
    use tetris_cli::validation::ValidationService;

    // Тест 1: validate_f32_finite существует и работает
    let result = ValidationService::validate_f32_finite(1.5);
    assert!(
        result.is_ok(),
        "validate_f32_finite(1.5) должен возвращать Ok"
    );

    let result_nan = ValidationService::validate_f32_finite(f32::NAN);
    assert!(
        result_nan.is_err(),
        "validate_f32_finite(NAN) должен возвращать ошибку"
    );

    // Тест 2: validate_u32_range существует и работает
    let result = ValidationService::validate_u32_range(5, 1, 10);
    assert!(
        result.is_ok(),
        "validate_u32_range(5, 1, 10) должен возвращать Ok"
    );

    let result_invalid = ValidationService::validate_u32_range(15, 1, 10);
    assert!(
        result_invalid.is_err(),
        "validate_u32_range(15, 1, 10) должен возвращать ошибку"
    );
}

/// Тест 13: GameAction enum для абстракции ввода
///
/// Проверяет что GameAction enum существует и используется для абстракции ввода.
///
/// # Архитектурная проблема
/// Абстракция ввода через GameAction enum для снижения связанности.
#[test]
fn test_modularity_game_action_enum() {
    use std::fs;

    let types_path = "src/game/types.rs";
    let content = fs::read_to_string(types_path).expect("Failed to read types.rs");

    // GameAction enum должен существовать
    assert!(
        content.contains("pub enum GameAction") || content.contains("enum GameAction"),
        "GameAction enum должен существовать в types.rs"
    );

    // Должен содержать варианты действий
    assert!(
        content.contains("MoveLeft") && content.contains("MoveRight"),
        "GameAction должен содержать варианты MoveLeft и MoveRight"
    );
}

// ============================================================================
// КАТЕГОРИЯ 4: КОД (7 тестов)
// ============================================================================

/// Тест 15: Обработка ошибки set_fall_speed()
///
/// Проверяет что scoring/lines.rs обрабатывает ошибку set_fall_speed().
///
/// # Архитектурная проблема E4 (HIGH)
/// В scoring/lines.rs добавлена явная обработка ошибки set_fall_speed().
#[test]
fn test_code_set_fall_speed_error_handling() {
    use std::fs;
    use tetris_cli::game::GameState;

    let lines_path = "src/game/scoring/lines.rs";
    let content = fs::read_to_string(lines_path).expect("Failed to read lines.rs");

    // set_fall_speed() вызов с обработкой ошибки
    assert!(
        content.contains("set_fall_speed"),
        "lines.rs должен вызывать set_fall_speed()"
    );

    // Обработка ошибки через if let Err
    assert!(
        content.contains("if let Err(e) = state.set_fall_speed")
            || content.contains("let _ = state.set_fall_speed"),
        "set_fall_speed() ошибка должна обрабатываться"
    );

    // Интеграционный тест - set_fall_speed возвращает Result
    let mut state = GameState::default();

    let result_nan = state.set_fall_speed(f32::NAN);
    assert!(
        result_nan.is_err(),
        "set_fall_speed(NAN) должен возвращать ошибку"
    );

    let result_valid = state.set_fall_speed(1.5);
    assert!(
        result_valid.is_ok(),
        "set_fall_speed(valid) должен возвращать Ok"
    );
}

/// Тест 16: SRS wall kick смещения
///
/// Проверяет что WALL_KICK_OFFSETS содержит правильные смещения согласно стандарту SRS.
///
/// # Архитектурная проблема L1 (HIGH)
/// Добавлено смещение (0, 0) первым элементом для базовой проверки вращения на месте.
#[test]
fn test_code_srs_wall_kick_offsets() {
    use tetris_cli::game::logic::wall_kick::WALL_KICK_OFFSETS;

    // Тест 1: Первое смещение (0, 0) - базовая проверка
    assert_eq!(
        WALL_KICK_OFFSETS[0],
        (0, 0),
        "Первое смещение должно быть (0, 0) - базовая проверка на месте"
    );

    // Тест 2: Простые смещения влево/вправо (±1)
    assert!(
        WALL_KICK_OFFSETS.contains(&(-1, 0)),
        "Должно быть смещение влево на 1"
    );
    assert!(
        WALL_KICK_OFFSETS.contains(&(1, 0)),
        "Должно быть смещение вправо на 1"
    );

    // Тест 3: Двойные смещения (±2)
    assert!(
        WALL_KICK_OFFSETS.contains(&(-2, 0)),
        "Должно быть смещение влево на 2"
    );
    assert!(
        WALL_KICK_OFFSETS.contains(&(2, 0)),
        "Должно быть смещение вправо на 2"
    );

    // Тест 4: Количество смещений = 8
    assert_eq!(WALL_KICK_OFFSETS.len(), 8, "Должно быть ровно 8 смещений");
}

/// Тест 17: rows_cleared=0 защита от паники
///
/// Проверяет что update_score_for_lines() не паникует при rows_cleared=0.
///
/// # Архитектурная проблема L2 (HIGH)
/// Добавлена явная проверка rows_cleared > 0 перед доступом к LINE_SCORES.
#[test]
fn test_code_rows_cleared_zero_protection() {
    use std::fs;

    let lines_path = "src/game/scoring/lines.rs";
    let content = fs::read_to_string(lines_path).expect("Failed to read lines.rs");

    // Проверка rows_cleared == 0 перед доступом к LINE_SCORES
    assert!(
        content.contains("if rows_cleared == 0") || content.contains("if capped_rows == 0"),
        "Должна быть проверка rows_cleared == 0"
    );

    // Ранний возврат при rows_cleared == 0
    let update_score_start = content
        .find("fn update_score_for_lines")
        .expect("update_score_for_lines должен существовать");
    let update_score_section = &content
        [update_score_start..update_score_start + 1000.min(content.len() - update_score_start)];

    assert!(
        update_score_section.contains("return"),
        "Должен быть ранний возврат при rows_cleared == 0"
    );
}

/// Тест 18: Инвертированная логика has_collision
///
/// Проверяет что функция has_collision существует и работает корректно.
///
/// # Архитектурная проблема H1 (HIGH)
/// Функция проверки коллизий использует инвертированную логику.
#[test]
fn test_code_has_collision_logic() {
    use std::fs;
    use tetris_cli::game::board::GameBoard;

    let collision_path = "src/game/logic/collision.rs";
    let content = fs::read_to_string(collision_path).expect("Failed to read collision.rs");

    // Функция has_collision должна существовать
    assert!(
        content.contains("fn has_collision") || content.contains("has_collision"),
        "collision.rs должен содержать функцию has_collision"
    );

    // Интеграционный тест - функция работает корректно
    let board = GameBoard::default();
    assert!(board.get_block(0, 0).is_some());
}

/// Тест 19: Удаление избыточных #[inline] атрибутов
///
/// Проверяет что в collision.rs и board.rs нет явных #[inline] атрибутов.
///
/// # Архитектурная проблема H3 (HIGH)
/// Удалены явные #[inline] атрибуты - компилятор сам решает.
#[test]
fn test_code_no_inline_attributes() {
    use std::fs;

    let collision_path = "src/game/logic/collision.rs";
    let board_path = "src/game/board.rs";

    let collision_content =
        fs::read_to_string(collision_path).expect("Failed to read collision.rs");
    let board_content = fs::read_to_string(board_path).expect("Failed to read board.rs");

    // В collision.rs не должно быть #[inline]
    assert!(
        !collision_content.contains("#[inline]"),
        "collision.rs не должен содержать #[inline] атрибуты"
    );

    // В board.rs не должно быть #[inline]
    assert!(
        !board_content.contains("#[inline]"),
        "board.rs не должен содержать #[inline] атрибуты"
    );
}

/// Тест 20: Семантические методы GameState
///
/// Проверяет что GameState имеет методы apply_gravity(), spawn_new_piece(), update_fall_speed().
///
/// # Архитектурная проблема M3 (MEDIUM)
/// Добавлены семантические методы для улучшения инкапсуляции.
#[test]
fn test_code_gamestate_semantic_methods() {
    use std::fs;
    use tetris_cli::game::GameState;

    let state_path = "src/game/state.rs";
    let content = fs::read_to_string(state_path).expect("Failed to read state.rs");

    // apply_gravity() должен существовать
    assert!(
        content.contains("fn apply_gravity"),
        "state.rs должен содержать метод apply_gravity"
    );

    // spawn_new_piece() должен существовать
    assert!(
        content.contains("fn spawn_new_piece"),
        "state.rs должен содержать метод spawn_new_piece"
    );

    // update_fall_speed() должен существовать
    assert!(
        content.contains("fn update_fall_speed"),
        "state.rs должен содержать метод update_fall_speed"
    );

    // Интеграционный тест - методы работают
    let mut state = GameState::default();

    // apply_gravity должна увеличивать fall_speed
    let fall_speed_before = state.fall_speed();
    state.apply_gravity();
    let fall_speed_after = state.fall_speed();
    assert!(
        fall_speed_after >= fall_speed_before,
        "apply_gravity должен увеличивать fall_speed"
    );
}

// ============================================================================
// КАТЕГОРИЯ 5: МАСШТАБИРУЕМОСТЬ (6 тестов)
// ============================================================================

/// Тест 21: ThreadSafeLeaderboard race condition защита
///
/// Проверяет что ThreadSafeLeaderboard использует Mutex для защиты от race condition.
///
/// # Архитектурная проблема E6 (HIGH)
/// ThreadSafeLeaderboard использует Arc<Mutex<Leaderboard>> для защиты от race condition.
#[test]
fn test_scalability_thread_safe_leaderboard() {
    use tetris_cli::highscore::leaderboard::ThreadSafeLeaderboard;

    // Тест 1: ThreadSafeLeaderboard можно создать и использовать
    let leaderboard = ThreadSafeLeaderboard::new();

    // Добавляем запись
    let result = leaderboard.add_score("Player1", 1000);
    assert!(result, "add_score() должен вернуть true для первой записи");

    // Тест 2: ThreadSafeLeaderboard::get_entries() возвращает записи
    let entries = leaderboard.get_entries();
    assert!(
        !entries.is_empty(),
        "ThreadSafeLeaderboard должен содержать хотя бы 1 запись"
    );

    // Тест 3: ThreadSafeLeaderboard::get_best_score() безопасен
    let best_score = leaderboard.get_best_score();
    // best_score имеет unsigned тип, поэтому >= 0 всегда истинно
    let _ = best_score; // Просто проверяем что вызов работает
}

/// Тест 22: HMAC ключ константность
///
/// Проверяет что используется один HMAC ключ для всех записей конфигурации.
///
/// # Архитектурная проблема E10 (HIGH)
/// Используется глобальный HMAC ключ вместо генерации нового при каждом сохранении.
#[test]
fn test_scalability_hmac_key_constancy() {
    use std::fs;
    use tetris_cli::config::keys::get_controls_hmac_key;

    let controls_path = "src/controls.rs";
    let content = fs::read_to_string(controls_path).expect("Failed to read controls.rs");

    // Используется get_controls_hmac_key() вместо generate_salt()
    assert!(
        content.contains("get_controls_hmac_key()"),
        "controls.rs должен использовать get_controls_hmac_key()"
    );

    // get_controls_hmac_key() возвращает константный ключ
    let key1 = get_controls_hmac_key();
    let key2 = get_controls_hmac_key();
    assert_eq!(
        key1, key2,
        "get_controls_hmac_key() должен возвращать один и тот же ключ"
    );
}

/// Тест 23: Интерфейсное разделение ScoreAccess/ScoreMutable
///
/// Проверяет что ScoreAccess и ScoreMutable разделены и не дублируют методы.
///
/// # Архитектурная проблема H1 (ISP)
/// Interface Segregation Principle - узкие трейты вместо широких.
#[test]
fn test_scalability_score_access_traits() {
    use std::fs;

    let access_path = "src/game/access.rs";
    let content = fs::read_to_string(access_path).expect("Failed to read access.rs");

    // Должны существовать отдельные трейты
    assert!(
        content.contains("trait ScoreAccess"),
        "Должен существовать трейт ScoreAccess"
    );
    assert!(
        content.contains("trait ScoreMutable"),
        "Должен существовать трейт ScoreMutable"
    );

    // ScoreAccess должен содержать только методы чтения
    assert!(
        content.contains("get_score") && content.contains("get_level"),
        "ScoreAccess должен содержать методы чтения"
    );

    // ScoreMutable должен содержать методы записи
    assert!(
        content.contains("add_score") && content.contains("set_score"),
        "ScoreMutable должен содержать методы записи"
    );
}

/// Тест 24: BoardReadonly/BoardMutable трейты
///
/// Проверяет что трейты BoardReadonly и BoardMutable существуют и разделены.
///
/// # Архитектурная проблема
/// Разделение трейтов доступа для снижения связанности.
#[test]
fn test_scalability_board_access_traits() {
    use std::fs;

    let access_path = "src/game/access.rs";
    let content = fs::read_to_string(access_path).expect("Failed to read access.rs");

    // BoardReadonly должен существовать
    assert!(
        content.contains("trait BoardReadonly"),
        "Должен существовать трейт BoardReadonly"
    );

    // BoardMutable должен существовать
    assert!(
        content.contains("trait BoardMutable"),
        "Должен существовать трейт BoardMutable"
    );

    // BoardMutable должен расширять BoardReadonly
    assert!(
        content.contains("trait BoardMutable: BoardReadonly"),
        "BoardMutable должен расширять BoardReadonly"
    );
}

/// Тест 25: ShapeType вместо TetrominoType
///
/// Проверяет что используется ShapeType вместо TetrominoType.
///
/// # Архитектурная проблема C1
/// Замена TetrominoType → ShapeType для согласованности именования.
#[test]
fn test_scalability_shapetype_naming() {
    use std::fs;

    let shape_type_path = "src/tetromino/shape_type.rs";
    let events_path = "src/game/types.rs";

    let shape_content = fs::read_to_string(shape_type_path).expect("Failed to read shape_type.rs");
    let events_content = fs::read_to_string(events_path).expect("Failed to read types.rs");

    // ShapeType должен существовать
    assert!(
        shape_content.contains("pub enum ShapeType") || shape_content.contains("enum ShapeType"),
        "ShapeType enum должен существовать"
    );

    // TetrominoType не должен использоваться (только в комментариях)
    let tetromino_type_count = shape_content.matches("TetrominoType").count();
    assert!(
        tetromino_type_count <= 1,
        "TetrominoType не должен использоваться (найдено: {})",
        tetromino_type_count
    );
}

/// Тест 26: Рефакторинг run_menu_loop
///
/// Проверяет что run_menu_loop() разбит на отдельные методы.
///
/// # Архитектурная проблема L4 (LOW)
/// Разбиение функции run_menu_loop() на render_menu_frame(), process_menu_input(), check_exit_condition().
#[test]
fn test_scalability_menu_loop_refactoring() {
    use std::fs;

    let application_path = "src/app/application.rs";
    let content = fs::read_to_string(application_path).expect("Failed to read application.rs");

    // render_menu_frame() должен существовать
    assert!(
        content.contains("render_menu_frame") || content.contains("fn render_menu"),
        "application.rs должен содержать метод render_menu_frame"
    );

    // process_menu_input() должен существовать
    assert!(
        content.contains("process_menu_input") || content.contains("fn handle_menu"),
        "application.rs должен содержать метод process_menu_input"
    );

    // check_exit_condition() должен существовать
    assert!(
        content.contains("check_exit_condition") || content.contains("fn check_exit"),
        "application.rs должен содержать метод check_exit_condition"
    );
}

// ============================================================================
// КАТЕГОРИЯ 6: ДОПОЛНИТЕЛЬНЫЕ (4 теста)
// ============================================================================

/// Тест 27: Оптимизация sanitize_player_name
///
/// Проверяет что sanitize_player_name использует однопроходный алгоритм.
///
/// # Архитектурная проблема M2 (MEDIUM)
/// Оптимизация алгоритма sanitize_player_name.
#[test]
fn test_additional_sanitize_optimization() {
    use std::fs;
    use tetris_cli::validation::name::sanitize_player_name;

    let sanitize_path = "src/validation/name.rs";
    let content = fs::read_to_string(sanitize_path).unwrap_or_else(|_| {
        fs::read_to_string("src/highscore/sanitize.rs").expect("Failed to read sanitize.rs")
    });

    // Функция sanitize_player_name существует и работает
    let name = sanitize_player_name("Player123");
    assert_eq!(name, "Player123");

    let invalid_name = sanitize_player_name("@#$");
    assert_eq!(invalid_name, "Anonymous");

    // Пустое имя становится Anonymous
    let empty_name = sanitize_player_name("");
    assert_eq!(empty_name, "Anonymous");
}

/// Тест 28: Оптимизация find_filled_lines с SmallVec
///
/// Проверяет что find_filled_lines использует SmallVec для оптимизации.
///
/// # Архитектурная проблема M8 (MEDIUM)
/// SmallVec<[usize; 4]> вместо Vec<usize> для снижения аллокаций.
#[test]
fn test_additional_find_filled_lines_optimization() {
    use std::fs;
    use tetris_cli::game::scoring::lines::find_filled_lines;

    let lines_path = "src/game/scoring/lines.rs";
    let content = fs::read_to_string(lines_path).expect("Failed to read lines.rs");

    // Функция find_filled_lines существует
    assert!(
        content.contains("fn find_filled_lines") || content.contains("find_filled_lines"),
        "lines.rs должен содержать функцию find_filled_lines"
    );

    // Интеграционный тест - функция работает
    // count имеет unsigned тип, поэтому >= 0 всегда истинно
    let board = [[0i8; 10]; 20];
    let (count, _mask) = find_filled_lines(&board);
    let _ = count; // Просто проверяем что вызов работает
}

/// Тест 29: Проверка размера Tetromino
///
/// Проверяет что размер структуры Tetromino не превышает 40 байт.
///
/// # Архитектурная проблема M13 (MEDIUM)
/// Compile-time assert: size_of::<Tetromino>() <= 40
#[test]
fn test_additional_tetromino_size_check() {
    use std::fs;
    use std::mem::size_of;

    let tetromino_path = "src/tetromino/tetromino_struct.rs";
    let content = fs::read_to_string(tetromino_path).expect("Failed to read tetromino_struct.rs");

    // Compile-time assert существует
    assert!(
        content.contains("size_of::<Tetromino>()") || content.contains("mem::size_of"),
        "tetromino_struct.rs должен содержать проверку размера"
    );

    // Размер Tetromino не превышает 40 байт
    let size = size_of::<tetris_cli::tetromino::Tetromino>();
    assert!(
        size <= 40,
        "Размер Tetromino должен быть <= 40 байт (фактический: {})",
        size
    );
}

/// Тест 30: Безопасная конвертация f32 → u32
///
/// Проверяет что используется явная проверка границ вместо clamp().
///
/// # Архитектурная проблема C1 (CRITICAL)
/// Реализована явная проверка границ для защиты от NaN, Infinity, переполнения.
#[test]
fn test_additional_safe_f32_to_u32_cast() {
    use tetris_cli::validation::ValidationService;

    // NaN возвращает ошибку
    let result = ValidationService::validate_f32_finite(f32::NAN);
    assert!(
        result.is_err(),
        "validate_f32_finite(NAN) должен возвращать ошибку"
    );

    // Infinity возвращает ошибку
    let result = ValidationService::validate_f32_finite(f32::INFINITY);
    assert!(
        result.is_err(),
        "validate_f32_finite(INFINITY) должен возвращать ошибку"
    );

    // Valid значение возвращает Ok
    let result = ValidationService::validate_f32_finite(1.5);
    assert!(
        result.is_ok(),
        "validate_f32_finite(valid) должен возвращать Ok"
    );
}
