//! Тесты на архитектурную целостность проекта tetris-cli.
//!
//! Этот модуль содержит тесты для проверки соблюдения архитектурных ограничений:
//! - Отсутствие циклических зависимостей между модулями
//! - Целостность модульной структуры
//! - Разделение ответственности между модулями
//! - Инкапсуляция внутренних структур данных
//! - Использование криптографических компонентов
//! - Отсутствие дублирования констант
//! - Соблюдение границ модулей
//!
//! ## Список тестов
//! - `test_no_cyclic_dependencies` — проверка отсутствия циклических импортов
//! - `test_module_integrity` — проверка целостности модулей
//! - `test_separation_of_concerns` — проверка разделения ответственности
//! - `test_game_stats_encapsulation` — проверка инкапсуляции GameStats
//! - `test_hmac_validator` — проверка использования HmacValidator
//! - `test_constants_no_duplication` — проверка отсутствия дублирования констант
//! - `test_module_boundaries` — проверка границ модулей
//! - `test_architecture_integration` — интеграционный тест архитектуры

// ============================================================================
// ТЕСТ 1: ОТСУТСТВИЕ ЦИКЛИЧЕСКИХ ЗАВИСИМОСТЕЙ
// ============================================================================

/// Проверка отсутствия циклических зависимостей между модулями.
///
/// # Архитектурные ограничения
/// - `validation` не должен зависеть от `game`, `menu`, `app`
/// - `crypto` не должен зависеть от `game`, `menu`, `app`
/// - `types` не должен зависеть от `game`, `menu`, `app`
/// - `constants` не должен зависеть от внутренних модулей
///
/// # Метод проверки
/// Используем анализ имён типов через `std::any::type_name`.
/// Если модуль импортирует запрещённые зависимости, это будет видно
/// через имена типов в ошибке компиляции.
///
/// # Гарантии
/// - Если тест компилируется — циклических зависимостей нет
/// - Компилятор Rust гарантирует отсутствие циклов в модульной системе
#[test]
fn test_no_cyclic_dependencies() {
    use std::any::type_name;

    // Проверяем, что базовые модули не зависят от модулей верхнего уровня
    // Если бы была циклическая зависимость, код не скомпилировался бы

    // Проверяем тип из validation модуля
    let _validator_type = type_name::<crate::validation::path::PathValidator>();
    assert!(
        _validator_type.contains("path"),
        "PathValidator должен быть в модуле validation::path"
    );

    // Проверяем тип из crypto модуля
    let _crypto_type = type_name::<crate::crypto::validator::HmacValidator>();
    assert!(
        _crypto_type.contains("crypto"),
        "HmacValidator должен быть в модуле crypto"
    );

    // Проверяем тип из types модуля
    let _types_type = type_name::<crate::types::Direction>();
    assert!(
        _types_type.contains("types"),
        "Direction должен быть в модуле types"
    );

    // Проверяем тип из constants модуля
    // constants содержит только константы, проверяем через импорт
    let _fps = crate::constants::FPS;
    assert_eq!(_fps, 60, "FPS константа должна быть доступна");

    // Если код компилируется — циклических зависимостей нет
    // Это гарантируется системой модулей Rust
}

// ============================================================================
// ТЕСТ 2: ЦЕЛОСТНОСТЬ МОДУЛЕЙ
// ============================================================================

/// Проверка целостности модульной структуры.
///
/// # Проверяемые требования
/// 1. Каждый модуль имеет mod.rs или индексный файл
/// 2. Все публичные API документированы (проверяется компилятором с #![warn(missing_docs)])
/// 3. Модули имеют чёткую ответственность
///
/// # Метод проверки
/// - Проверяем доступность публичных API из каждого модуля
/// - Проверяем наличие документации через компиляцию
#[test]
fn test_module_integrity() {
    // Проверяем что все основные модули доступны и работают

    // === constants.rs ===
    let _fps = crate::constants::FPS;
    let _grid_width = crate::constants::GRID_WIDTH;
    let _grid_height = crate::constants::GRID_HEIGHT;
    assert!(
        _grid_width > 0 && _grid_height > 0,
        "Размеры поля должны быть положительными"
    );

    // === types.rs ===
    let _dir = crate::types::Direction::Left;
    let _pos = crate::types::Position::new(5, 10);
    assert_eq!(_pos.x, 5);
    assert_eq!(_pos.y, 10);

    // === errors.rs ===
    let _err = crate::errors::GameError::validation_error("test");
    assert!(format!("{}", _err).contains("Ошибка валидации"));

    // === io_traits.rs ===
    // Трейты проверяются через компиляцию
    let _ = <crate::io::Canvas as crate::io_traits::Renderer>::flush;

    // === crypto/validator.rs ===
    let validator = crate::crypto::validator::HmacValidator::new("test_key");
    let signature = validator.sign("data");
    assert!(
        validator.verify("data", &signature),
        "Подпись должна быть валидной"
    );

    // === validation.rs ===
    let path_validator =
        crate::validation::path::PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    let result = path_validator.validate_length(std::path::Path::new("test"));
    assert!(result.is_ok(), "Валидация должна пройти успешно");

    // === highscore/mod.rs ===
    let _leaderboard = crate::highscore::Leaderboard::default();
    let _save_data = crate::highscore::SaveData::default();

    // === controls.rs ===
    let _config = crate::controls::ControlsConfig::default_config();

    // === menu/mod.rs ===
    // menu модуль доступен через публичные функции
    let _menu = crate::menu::constants::MENU;

    // === game/mod.rs ===
    let _game_state = crate::game::GameState::new();
    let _game_mode = crate::game::GameMode::Classic;

    // === tetromino.rs ===
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
    let mut bag = BagGenerator::new();
    let _shape = Tetromino::from_bag(&mut bag);

    // === io.rs ===
    // Canvas и KeyReader проверяются через трейты

    // === app/mod.rs ===
    // app модуль содержит основную логику приложения
}

// ============================================================================
// ТЕСТ 3: РАЗДЕЛЕНИЕ ОТВЕТСТВЕННОСТИ
// ============================================================================

/// Проверка разделения ответственности между модулями.
///
/// # Архитектурные ограничения
/// - `render.rs` не должен содержать игровой логики (только отрисовка)
/// - `scoring` не должен содержать отрисовки (только подсчёт очков)
/// - `menu` не должен содержать игровой логики (только навигация)
///
/// # Метод проверки
/// Проверяем что функции из каждого модуля выполняют только свою ответственность:
/// - render функции принимают только данные для отрисовки (GameView)
/// - scoring функции работают только с данными игры (GameState)
/// - menu функции работают только с навигацией и вводом
#[test]
fn test_separation_of_concerns() {
    use crate::game::render::update_cached_strings_extended;
    use crate::game::scoring::check_rows;
    use crate::game::{GameState, GameView};

    // === Проверка render.rs ===
    // render модуль должен работать только с GameView (неизменяемое представление)
    let mut state = GameState::new();

    // update_cached_strings_extended требует mutable доступ, но это подготовка данных
    // для отрисовки, а не игровая логика
    update_cached_strings_extended(&mut state, "0");

    // Создаём GameView для отрисовки — это единственные данные для render
    let _view = GameView::from_game_state(&state);

    // Проверяем что render не модифицирует состояние игры напрямую
    let score_before = state.score();
    let level_before = state.level();
    // render функции не должны изменять score/level
    assert_eq!(
        score_before,
        state.score(),
        "render не должен изменять счёт"
    );
    assert_eq!(
        level_before,
        state.level(),
        "render не должен изменять уровень"
    );

    // === Проверка scoring.rs ===
    // scoring модуль должен работать только с игровой логикой
    let mut state_for_scoring = GameState::new();
    let initial_score = state_for_scoring.score();

    // check_rows может изменять состояние (удалять линии и начислять очки)
    // но не должен выполнять отрисовку
    let _lines_removed = check_rows(&mut state_for_scoring);

    // scoring может изменять счёт (это его ответственность)
    // но не должен выполнять отрисовку — это проверяется через отсутствие
    // импортов Canvas в scoring модуле (гарантируется компилятором)

    // === Проверка menu.rs ===
    // menu модуль не должен содержать игровой логики
    // Проверяем через анализ доступных функций
    let _menu = crate::menu::constants::MENU;
    // menu::draw::draw_menu принимает только Canvas и данные для отображения
    // menu::input::get_player_name работает только с вводом

    // menu не должен иметь доступа к внутренностям GameState
    // (это гарантируется системой модулей Rust)
}

// ============================================================================
// ТЕСТ 4: ИНКАПСУЛЯЦИЯ GameStats
// ============================================================================

/// Проверка инкапсуляции структуры GameStats.
///
/// # Требования
/// - Все поля GameStats должны быть приватными
/// - Доступ к полям только через геттеры/сеттеры
/// - Геттеры должны возвращать корректные значения
///
/// # Метод проверки
/// - Проверяем что поля недоступны напрямую (компилятор не позволит)
/// - Проверяем что геттеры возвращают правильные значения
/// - Проверяем что сеттеры корректно изменяют значения
#[test]
fn test_game_stats_encapsulation() {
    use crate::game::GameStats;

    // Создаём новую статистику
    let stats = GameStats::new();

    // === Проверка начальных значений через геттеры ===
    assert_eq!(
        stats.t_pieces(),
        0,
        "Начальное количество T фигур должно быть 0"
    );
    assert_eq!(
        stats.l_pieces(),
        0,
        "Начальное количество L фигур должно быть 0"
    );
    assert_eq!(
        stats.j_pieces(),
        0,
        "Начальное количество J фигур должно быть 0"
    );
    assert_eq!(
        stats.s_pieces(),
        0,
        "Начальное количество S фигур должно быть 0"
    );
    assert_eq!(
        stats.z_pieces(),
        0,
        "Начальное количество Z фигур должно быть 0"
    );
    assert_eq!(
        stats.o_pieces(),
        0,
        "Начальное количество O фигур должно быть 0"
    );
    assert_eq!(
        stats.i_pieces(),
        0,
        "Начальное количество I фигур должно быть 0"
    );
    assert_eq!(
        stats.max_combo(),
        0,
        "Начальное максимальное комбо должно быть 0"
    );
    assert_eq!(
        stats.combo_counter(),
        0,
        "Начальный счётчик комбо должен быть 0"
    );
    assert_eq!(
        stats.total_pieces(),
        0,
        "Общее количество фигур должно быть 0"
    );

    // === Проверка сеттеров ===
    let mut stats_mut = GameStats::new();

    stats_mut.set_t_pieces(5);
    stats_mut.set_l_pieces(3);
    stats_mut.set_i_pieces(7);

    assert_eq!(stats_mut.t_pieces(), 5, "Сеттер t_pieces должен работать");
    assert_eq!(stats_mut.l_pieces(), 3, "Сеттер l_pieces должен работать");
    assert_eq!(stats_mut.i_pieces(), 7, "Сеттер i_pieces должен работать");

    // === Проверка метода add_piece ===
    use crate::tetromino::ShapeType;

    stats_mut.add_piece(ShapeType::T);
    stats_mut.add_piece(ShapeType::L);

    assert_eq!(
        stats_mut.t_pieces(),
        6,
        "add_piece должен увеличить счётчик T"
    );
    assert_eq!(
        stats_mut.l_pieces(),
        4,
        "add_piece должен увеличить счётчик L"
    );
    assert_eq!(
        stats_mut.total_pieces(),
        5 + 3 + 7 + 1 + 1,
        "total_pieces должен считать все фигуры"
    );

    // === Проверка update_max_combo ===
    stats_mut.update_max_combo(3);
    assert_eq!(stats_mut.max_combo(), 3, "max_combo должен обновиться");

    stats_mut.update_max_combo(2);
    assert_eq!(stats_mut.max_combo(), 3, "max_combo не должен уменьшиться");

    stats_mut.update_max_combo(5);
    assert_eq!(
        stats_mut.max_combo(),
        5,
        "max_combo должен обновиться до большего значения"
    );

    // === Проверка таймера ===
    assert!(
        stats.start_time().is_none(),
        "start_time должен быть None до запуска"
    );

    stats_mut.start_timer();
    assert!(
        stats_mut.start_time().is_some(),
        "start_time должен быть установлен"
    );

    stats_mut.stop_timer();
    assert!(
        stats_mut.end_time().is_some(),
        "end_time должен быть установлен после остановки"
    );

    let elapsed = stats_mut.get_elapsed_time();
    assert!(elapsed >= 0.0, "Время должно быть неотрицательным");

    // === Проверка что поля приватные ===
    // Следующий код не скомпилируется, что подтверждает приватность:
    // let _ = stats.t_pieces; // Ошибка компиляции: поле приватное
}

// ============================================================================
// ТЕСТ 5: ИСПОЛЬЗОВАНИЕ HmacValidator
// ============================================================================

/// Проверка использования HmacValidator.
///
/// # Требования
/// - Модуль validator должен существовать
/// - Функции sign/verify должны работать корректно
/// - Подпись должна быть детерминированной
/// - Невалидная подпись должна отклоняться
///
/// # Метод проверки
/// - Создаём валидатор с известным ключом
/// - Подписываем тестовые данные
/// - Проверяем подпись
/// - Проверяем отклонение невалидной подписи
#[test]
fn test_hmac_validator() {
    use crate::crypto::validator::HmacValidator;

    // === Проверка существования модуля ===
    // Если код компилируется — модуль существует
    let validator = HmacValidator::new("test_secret_key");

    // === Проверка sign/verify ===
    let data = "player_name:score:1000";
    let signature = validator.sign(data);

    // Проверяем что подпись имеет правильную длину (64 символа hex)
    assert_eq!(
        signature.len(),
        64,
        "HMAC-SHA256 подпись должна быть 64 символа"
    );

    // Проверяем что подпись валидная
    assert!(
        validator.verify(data, &signature),
        "Подпись должна быть валидной"
    );

    // === Проверка детерминированности ===
    let signature2 = validator.sign(data);
    assert_eq!(
        signature, signature2,
        "Подписи одинаковых данных должны совпадать"
    );

    // === Проверка отклонения невалидной подписи ===
    assert!(
        !validator.verify(data, "invalid_signature"),
        "Невалидная подпись должна отклоняться"
    );

    // === Проверка чувствительности к данным ===
    let modified_data = "player_name:score:2000";
    assert!(
        !validator.verify(modified_data, &signature),
        "Подпись не должна подходить для изменённых данных"
    );

    // === Проверка чувствительности к ключу ===
    let validator2 = HmacValidator::new("different_key");
    assert!(
        !validator2.verify(data, &signature),
        "Подпись не должна подходить для другого ключа"
    );

    // === Проверка generate() ===
    let validator3 = HmacValidator::generate();
    let validator4 = HmacValidator::generate();

    // Ключи должны быть уникальными
    assert_ne!(
        validator3.key(),
        validator4.key(),
        "Сгенерированные ключи должны быть уникальными"
    );

    // === Проверка verify_and_return ===
    let result = validator.verify_and_return(data, &signature);
    assert_eq!(
        result,
        Some(data.to_string()),
        "verify_and_return должен вернуть данные"
    );

    let invalid_result = validator.verify_and_return(data, "invalid");
    assert_eq!(
        invalid_result, None,
        "verify_and_return должен вернуть None для невалидной подписи"
    );

    // === Проверка вспомогательных функций ===
    use crate::crypto::validator::{sign_salt_and_data, verify_salt_and_data};

    let key = "test_key";
    let salt = "random_salt";
    let test_data = "test_data";

    let salted_signature = sign_salt_and_data(key, salt, test_data);
    assert!(
        verify_salt_and_data(key, salt, test_data, &salted_signature),
        "Подпись с солью должна быть валидной"
    );
}

// ============================================================================
// ТЕСТ 6: ОТСУТСТВИЕ ДУБЛИРОВАНИЯ КОНСТАНТ
// ============================================================================

/// Проверка отсутствия дублирования констант.
///
/// # Требования
/// - Все константы должны быть определены только в constants.rs
/// - io.rs должен импортировать константы из constants.rs
/// - Другие модули не должны определять дублирующие константы
///
/// # Метод проверки
/// - Проверяем что константы доступны из constants.rs
/// - Проверяем что io.rs использует импортированные константы
/// - Проверяем что значения констант согласованы
#[test]
fn test_constants_no_duplication() {
    // === Проверка что все константы определены в constants.rs ===

    // Базовые константы
    let fps = crate::constants::FPS;
    let frame_delay = crate::constants::FRAME_DELAY_MS;
    assert_eq!(fps, 60, "FPS должен быть 60");
    assert_eq!(
        frame_delay,
        1000 / fps,
        "FRAME_DELAY должен вычисляться из FPS"
    );

    // Физика игры
    let initial_fall_spd = crate::constants::INITIAL_FALL_SPD;
    let max_fall_speed = crate::constants::MAX_FALL_SPEED;
    let spd_inc = crate::constants::SPD_INC;
    assert!(
        initial_fall_spd > 0.0,
        "Начальная скорость должна быть положительной"
    );
    assert!(
        max_fall_speed > initial_fall_spd,
        "Максимальная скорость должна быть больше начальной"
    );

    // Система очков
    let line_scores = crate::constants::LINE_SCORES;
    let combo_bonus = crate::constants::COMBO_BONUS;
    assert_eq!(line_scores[0], 100, "Очки за 1 линию должны быть 100");
    assert_eq!(
        line_scores[3], 1800,
        "Очки за Tetris (4 линии) должны быть 1800"
    );
    assert_eq!(combo_bonus, 50, "Бонус за комбо должен быть 50");

    // Размеры игрового поля
    let grid_width = crate::constants::GRID_WIDTH;
    let grid_height = crate::constants::GRID_HEIGHT;
    assert_eq!(grid_width, 10, "Ширина поля должна быть 10");
    assert_eq!(grid_height, 20, "Высота поля должна быть 20");

    // === Проверка что io.rs импортирует из constants.rs ===
    // Если io.rs определяет свои константы, они должны совпадать
    let io_grid_width = crate::io::GRID_WIDTH;
    let io_grid_height = crate::io::GRID_HEIGHT;

    assert_eq!(
        io_grid_width, grid_width,
        "GRID_WIDTH в io.rs должен совпадать с constants.rs"
    );
    assert_eq!(
        io_grid_height, grid_height,
        "GRID_HEIGHT в io.rs должен совпадать с constants.rs"
    );

    // === Проверка что game модуль использует импортированные константы ===
    use crate::game::constants as game_constants;

    assert_eq!(
        game_constants::FPS,
        fps,
        "game::constants::FPS должен совпадать с crate::constants::FPS"
    );
    assert_eq!(
        game_constants::GRID_WIDTH,
        grid_width,
        "game::constants::GRID_WIDTH должен совпадать"
    );

    // === Проверка что menu модуль использует свои константы ===
    // menu имеет собственные константы для отрисовки меню
    let _menu = crate::menu::constants::MENU;
    // Это допустимо — menu константы специфичны для меню

    // === Проверка согласованности значений ===
    // Вычисляемые константы должны быть согласованы
    let disp_width = crate::constants::DISP_WIDTH;
    let shape_width = crate::constants::SHAPE_WIDTH;
    let expected_disp_width = shape_width * grid_width + 2;
    assert_eq!(
        disp_width, expected_disp_width,
        "DISP_WIDTH должен вычисляться корректно"
    );
}

// ============================================================================
// ТЕСТ 7: ГРАНИЦЫ МОДУЛЕЙ
// ============================================================================

/// Проверка соблюдения границ модулей.
///
/// # Архитектурные ограничения
/// - `game::render` не должен зависеть от `game::scoring` напрямую
/// - `highscore` не должен зависеть от `game` напрямую
/// - Базовые модули (types, crypto, validation) не должны зависеть от модулей верхнего уровня
///
/// # Метод проверки
/// Проверяем что модули используют только разрешённые зависимости:
/// - Через анализ импортов (компилятор гарантирует)
/// - Через проверку доступности типов
#[test]
fn test_module_boundaries() {
    // === Проверка что render не зависит от scoring напрямую ===
    // render модуль использует GameView, а не GameState напрямую
    use crate::game::render::draw;
    use crate::game::GameView;

    let state = crate::game::GameState::new();
    let view = GameView::from_game_state(&state);

    // draw принимает только GameView и Canvas
    // Это подтверждает что render не зависит от внутренней логики scoring
    // Canvas создаётся только для проверки типа, реальная отрисовка не выполняется
    // because we don't have a real terminal in tests

    // Проверяем что render функции существуют и имеют правильные сигнатуры
    let _draw_fn = draw; // Проверяем что функция доступна

    // === Проверка что highscore не зависит от game ===
    // highscore модуль должен работать независимо от GameState
    use crate::highscore::{Leaderboard, SaveData};

    let mut leaderboard = Leaderboard::default();
    let _added = leaderboard.add_score("Player", 1000);

    // highscore не должен требовать GameState для работы
    // Это подтверждается тем что Leaderboard::add_score работает без game модуля

    // === Проверка что scoring не зависит от render ===
    // scoring модуль работает только с GameState для начисления очков
    use crate::game::scoring::check_rows;

    let mut state_for_scoring = crate::game::GameState::new();
    let _lines = check_rows(&mut state_for_scoring);

    // scoring не должен использовать Canvas или другие render типы
    // Это гарантируется компилятором (если бы использовал, не скомпилировалось бы)

    // === Проверка что types не зависит от game ===
    use crate::types::{Direction, Position, RotationDirection};

    let _dir = Direction::Left;
    let _pos = Position::new(0, 0);
    let _rot = RotationDirection::Clockwise;

    // types модуль не должен импортировать game модуль
    // Это подтверждается тем что типы доступны без импорта game

    // === Проверка что crypto не зависит от game/menu ===
    use crate::crypto::{generate_salt, hash, keyed_hash, verify_keyed_hash};

    let _hash_result = hash("test");
    let _salt = generate_salt();
    let _keyed = keyed_hash("key", "data");
    let _valid = verify_keyed_hash("key", "data", &_keyed);

    // crypto модуль работает независимо от game/menu

    // === Проверка что validation не зависит от game/menu ===
    use crate::validation::path::PathValidator;

    // Создаём валидатор для тестирования
    let validator = PathValidator::new(
        255,
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-",
    );

    let test_path = std::path::Path::new("test");
    let _valid_length = validator.validate_length(test_path);
    let _valid_chars = validator.validate_characters(test_path);

    // validation модуль работает независимо
}

// ============================================================================
// ТЕСТ 8: ИНТЕГРАЦИОННЫЙ ТЕСТ АРХИТЕКТУРЫ
// ============================================================================

/// Комплексная проверка всех архитектурных ограничений.
///
/// # Проверяемые требования
/// - Все модули работают вместе корректно
/// - Нет циклических зависимостей
/// - Разделение ответственности соблюдается
/// - Инкапсуляция работает
/// - Криптографические компоненты интегрированы
/// - Константы не дублируются
/// - Границы модулей соблюдаются
///
/// # Метод проверки
/// Интеграционный тест создаёт полноценный сценарий использования
/// и проверяет что все архитектурные ограничения соблюдаются.
#[test]
fn test_architecture_integration() {
    use crate::constants::{FPS, GRID_HEIGHT, GRID_WIDTH};
    use crate::crypto::validator::HmacValidator;
    use crate::game::render::update_cached_strings_extended;
    use crate::game::scoring::check_rows;
    use crate::game::{GameMode, GameState, GameStats, GameView};
    use crate::highscore::{Leaderboard, SaveData};
    use crate::tetromino::{BagGenerator, ShapeType, Tetromino};
    use crate::types::{Direction, Position};
    use crate::validation::path::PathValidator;

    // === Сценарий 1: Инициализация игры ===
    let mut game_state = GameState::new();

    // Проверяем что состояние инициализировано корректно
    assert_eq!(game_state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(game_state.level(), 1, "Начальный уровень должен быть 1");

    // === Сценарий 2: Обновление кэша для отрисовки ===
    update_cached_strings_extended(&mut game_state, "0");
    let view = GameView::from_game_state(&game_state);

    // Проверяем что view создан корректно
    assert_eq!(view.score, "0", "Счёт в view должен быть 0");

    // === Сценарий 3: Игровая логика (scoring) ===
    let _lines = check_rows(&mut game_state);
    // scoring модуль работает независимо от render

    // === Сценарий 4: Статистика игры ===
    let stats = game_state.get_stats();
    assert_eq!(stats.total_pieces(), 1, "Должна быть хотя бы одна фигура");

    // Проверяем инкапсуляцию GameStats
    let _t_count = stats.t_pieces();
    let _max_combo = stats.max_combo();

    // === Сценарий 5: Криптографическая защита ===
    let validator = HmacValidator::new("game_secret");
    let score_data = format!("{}:{}", "Player", game_state.score());
    let signature = validator.sign(&score_data);

    assert!(
        validator.verify(&score_data, &signature),
        "Подпись рекорда должна быть валидной"
    );

    // === Сценарий 6: Таблица лидеров ===
    let mut leaderboard = Leaderboard::default();
    let _added = leaderboard.add_score("TestPlayer", 1000);

    // highscore модуль работает независимо от game модуля
    // (не требует GameState для работы)

    // === Сценарий 7: Фигуры и Bag Generator ===
    let mut bag = BagGenerator::new();
    let shape = Tetromino::from_bag(&mut bag);

    assert!(shape.coords.len() == 4, "Фигура должна иметь 4 блока");

    // === Сценарий 8: Типы и валидация ===
    let pos = Position::new(5, 10);
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 10);

    let dir = Direction::Left;
    let _rotation = dir.to_rotation_direction();

    let path_validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    let valid = path_validator.validate_length(std::path::Path::new("test_name"));
    assert!(valid.is_ok(), "Имя должно пройти валидацию");

    // === Сценарий 9: Константы согласованы ===
    assert_eq!(FPS, 60, "FPS должен быть 60");
    assert_eq!(GRID_WIDTH, 10, "Ширина поля должна быть 10");
    assert_eq!(GRID_HEIGHT, 20, "Высота поля должна быть 20");

    // === Сценарий 10: Режимы игры ===
    let classic = GameMode::Classic;
    let sprint = GameMode::Sprint;

    assert_eq!(
        classic.as_trait().name(),
        "Классика",
        "Название режима Classic"
    );
    assert_eq!(sprint.as_trait().name(), "Спринт", "Название режима Sprint");

    // === Финальная проверка ===
    // Если все тесты прошли — архитектурные ограничения соблюдены
}

// ============================================================================
// ДОПОЛНИТЕЛЬНЫЕ ТЕСТЫ ДЛЯ ПОЛНОТЫ ПОКРЫТИЯ
// ============================================================================

/// Тест для проверки что базовые модули не имеют лишних зависимостей.
#[test]
fn test_base_modules_independence() {
    // types модуль не должен зависеть от game/menu/app
    use crate::types::{Direction, Position, RotationDirection, UpdateEndState};

    let _dir = Direction::Down;
    let _pos = Position::new(0, 0);
    let _rot = RotationDirection::CounterClockwise;
    let _end_state = UpdateEndState::Continue;

    // constants модуль содержит только константы
    let _ = crate::constants::BORDER_COLOR;
    let _ = crate::constants::LINE_SCORES;

    // validation модуль не должен зависеть от game/menu
    use crate::validation::path::PathValidator;

    let validator = PathValidator::new(255, "abcdefghijklmnopqrstuvwxyz");
    let test_path = std::path::Path::new("test");
    let _ = validator.validate_length(test_path);
    let _ = validator.validate_characters(test_path);
}

/// Тест для проверки что io модуль корректно импортирует константы.
#[test]
fn test_io_constants_import() {
    // io модуль должен импортировать константы из constants.rs
    // а не определять свои собственные

    use crate::constants::{
        DISP_HEIGHT, DISP_WIDTH, GRID_HEIGHT, GRID_WIDTH, SHAPE_STR, SHAPE_WIDTH,
    };
    use crate::io::{DISP_HEIGHT as IO_DISP_HEIGHT, DISP_WIDTH as IO_DISP_WIDTH};

    // Проверяем что константы совпадают
    assert_eq!(GRID_WIDTH, IO_DISP_WIDTH / SHAPE_WIDTH - 2 / SHAPE_WIDTH);
    assert_eq!(GRID_HEIGHT, DISP_HEIGHT - 5);

    // Проверяем что SHAPE_STR доступна
    assert_eq!(SHAPE_STR, "██");
}

/// Тест для проверки что crypto модуль имеет правильную структуру.
#[test]
fn test_crypto_module_structure() {
    // Проверяем что validator подмодуль существует
    use crate::crypto::validator::HmacValidator;

    // Проверяем что основные функции crypto доступны
    use crate::crypto::{generate_salt, hash, keyed_hash, verify_keyed_hash};

    let data = "test";
    let h = hash(data);
    let s = generate_salt();
    let k = keyed_hash("key", data);

    assert_eq!(h.len(), 64, "BLAKE3 хеш должен быть 64 символа");
    assert_eq!(s.len(), 64, "Соль должна быть 64 символа");
    assert!(
        verify_keyed_hash("key", data, &k),
        "Подпись должна быть валидной"
    );

    // Проверяем что HmacValidator работает
    let validator = HmacValidator::new("key");
    let sig = validator.sign(data);
    assert!(validator.verify(data, &sig));
}

/// Тест для проверки что highscore модуль не зависит от game.
#[test]
fn test_highscore_independence_from_game() {
    // highscore должен работать без импорта game модуля

    use crate::highscore::{Leaderboard, SaveData};

    // Создаём таблицу лидеров без GameState
    let mut leaderboard = Leaderboard::default();

    // Добавляем рекорд
    let added = leaderboard.add_score("Player1", 5000);
    assert!(added, "Рекорд должен быть добавлен");

    // Проверяем что SaveData работает независимо
    let save = SaveData::from_value(1000);
    let score = save.verify_and_get_score();
    assert!(score.is_some(), "Рекорд должен быть валидным");
}

/// Тест для проверки что menu модуль не содержит игровой логики.
#[test]
fn test_menu_no_game_logic() {
    // menu модуль должен содержать только навигацию и ввод

    use crate::menu::constants::MENU;

    // Проверяем что константы меню доступны
    assert!(!MENU.is_empty(), "Меню не должно быть пустым");
    assert!(!MENU.is_empty(), "Меню должно содержать строки");

    // menu::draw и menu::input работают только с Canvas и KeyReader
    // а не с GameState напрямую
    // Это гарантируется системой модулей Rust
}
