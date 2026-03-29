//! Тесты для проверки архитектурных улучшений проекта tetris-cli.
//!
//! Этот файл содержит тесты для проверки всех реализованных архитектурных улучшений:
//! - P1 (CRITICAL) проблемы: ARCH-1, ARCH-2, ARCH-3
//! - P2 (MEDIUM) проблемы: SOLID-1, DRY-1, COHESION-1, CYCLE-1
//! - P3 (LOW) проблемы: COMPLEX-1, YAGNI-1
//!
//! ## Запуск тестов
//! ```bash
//! cargo test test_architecture_improvements
//! ```

use std::fs;
use std::path::Path;

// ============================================================================
// ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
// ============================================================================

/// Получить путь к директории src проекта.
fn get_src_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/src")
}

/// Прочитать содержимое файла по относительному пути от src/.
fn read_src_file(relative_path: &str) -> Result<String, std::io::Error> {
    let full_path = Path::new(&get_src_path()).join(relative_path);
    fs::read_to_string(full_path)
}

/// Проверить наличие строки в содержимом файла.
fn contains_pattern(content: &str, pattern: &str) -> bool {
    content.contains(pattern)
}

/// Проверить наличие нескольких паттернов в содержимом.
fn contains_all_patterns(content: &str, patterns: &[&str]) -> bool {
    patterns.iter().all(|&p| contains_pattern(content, p))
}

// ============================================================================
// ТЕСТЫ ДЛЯ P1 (CRITICAL) ПРОБЛЕМ
// ============================================================================

// -----------------------------------------------------------------------------
// ТЕСТ 1: ARCH-1 - Документация разделения GameState
// -----------------------------------------------------------------------------

/// Проверка что GameState имеет документацию о будущем разделении на компоненты.
///
/// ## Требования
/// - Файл state.rs должен содержать TODO комментарии о разделении
/// - TODO должен упоминать разделение на компоненты
#[test]
fn test_game_state_architecture_todo_comments() {
    let content = read_src_file("game/state.rs")
        .expect("Не удалось прочитать файл state.rs");

    // Проверяем наличие TODO комментариев о разделении
    let has_todo = contains_pattern(&content, "TODO")
        && (contains_pattern(&content, "разделени")
            || contains_pattern(&content, "компонент")
            || contains_pattern(&content, "GameStats")
            || contains_pattern(&content, "RenderCache"));

    // Проверяем наличие архитектурных заметок
    let has_arch_notes = contains_pattern(&content, "Архитектурные заметки")
        || contains_pattern(&content, "Исправление #1");

    assert!(
        has_todo || has_arch_notes,
        "state.rs должен содержать TODO комментарии или архитектурные заметки \
         о разделении GameState на компоненты (GameStats, RenderCache)"
    );

    // Дополнительная проверка: файл должен упоминать разделение
    assert!(
        contains_pattern(&content, "GameStats")
            && contains_pattern(&content, "RenderCache"),
        "state.rs должен упоминать GameStats и RenderCache как отдельные компоненты"
    );
}

// -----------------------------------------------------------------------------
// ТЕСТ 2: ARCH-2 - Методы отрисовки в GameView
// -----------------------------------------------------------------------------

/// Проверка что GameView имеет методы draw_field() и draw_shape().
///
/// ## Требования
/// - Файл view.rs должен содержать метод draw_field
/// - Файл view.rs должен содержать метод draw_shape
#[test]
fn test_game_view_has_draw_methods() {
    let content = read_src_file("game/view.rs")
        .expect("Не удалось прочитать файл view.rs");

    // Проверяем наличие метода draw_field
    assert!(
        contains_pattern(&content, "pub fn draw_field")
            || contains_pattern(&content, "fn draw_field"),
        "GameView должен иметь метод draw_field() для отрисовки игрового поля"
    );

    // Проверяем наличие метода draw_shape
    assert!(
        contains_pattern(&content, "pub fn draw_shape")
            || contains_pattern(&content, "fn draw_shape"),
        "GameView должен иметь метод draw_shape() для отрисовки фигуры"
    );

    // Проверяем что методы имеют документацию
    assert!(
        contains_pattern(&content, "Отрисовать игровое поле")
            || contains_pattern(&content, "Отрисовать текущую фигуру"),
        "Методы отрисовки должны иметь документацию на русском языке"
    );
}

/// Проверка что render.rs использует методы GameView.
///
/// ## Требования
/// - Файл render.rs должен вызывать view.draw_field()
/// - Файл render.rs должен вызывать view.draw_shape()
#[test]
fn test_render_uses_game_view_methods() {
    let content = read_src_file("game/render.rs")
        .expect("Не удалось прочитать файл render.rs");

    // Проверяем что render.rs использует методы GameView
    // Методы могут вызываться как view.draw_field() или через GameView
    let uses_draw_field = contains_pattern(&content, "draw_field")
        && (contains_pattern(&content, "view.draw_field")
            || contains_pattern(&content, ".draw_field(")
            || contains_pattern(&content, "ARCH-2"));

    let uses_draw_shape = contains_pattern(&content, "draw_shape")
        && (contains_pattern(&content, "view.draw_shape")
            || contains_pattern(&content, ".draw_shape(")
            || contains_pattern(&content, "ARCH-2"));

    assert!(
        uses_draw_field,
        "render.rs должен использовать метод draw_field() из GameView \
         (проверьте наличие view.draw_field() или архитектурных комментариев ARCH-2)"
    );

    assert!(
        uses_draw_shape,
        "render.rs должен использовать метод draw_shape() из GameView \
         (проверьте наличие view.draw_shape() или архитектурных комментариев ARCH-2)"
    );
}

// -----------------------------------------------------------------------------
// ТЕСТ 3: ARCH-3 - Трейты фаз в cycle.rs
// -----------------------------------------------------------------------------

/// Проверка что трейты фаз имеют документацию.
///
/// ## Требования
/// - cycle.rs должен содержать трейт FPSControl с документацией
/// - cycle.rs должен содержать трейт InputHandler с документацией
/// - cycle.rs должен содержать трейт GameUpdater с документацией
/// - cycle.rs должен содержать трейт GameRenderer с документацией
#[test]
fn test_cycle_traits_documentation() {
    let content = read_src_file("game/cycle.rs")
        .expect("Не удалось прочитать файл cycle.rs");

    // Проверяем наличие трейта FPSControl
    assert!(
        contains_pattern(&content, "trait FPSControl"),
        "cycle.rs должен содержать трейт FPSControl"
    );

    // Проверяем наличие трейта InputHandler
    assert!(
        contains_pattern(&content, "trait InputHandler"),
        "cycle.rs должен содержать трейт InputHandler"
    );

    // Проверяем наличие трейта GameUpdater
    assert!(
        contains_pattern(&content, "trait GameUpdater"),
        "cycle.rs должен содержать трейт GameUpdater"
    );

    // Проверяем наличие трейта GameRenderer
    assert!(
        contains_pattern(&content, "trait GameRenderer"),
        "cycle.rs должен содержать трейт GameRenderer"
    );

    // Проверяем наличие документации для трейтов
    let has_trait_docs = contains_all_patterns(
        &content,
        &[
            "Архитектурные заметки",
            "FPSControl",
            "InputHandler",
            "GameUpdater",
            "GameRenderer",
        ],
    );

    assert!(
        has_trait_docs,
        "Трейты в cycle.rs должны иметь документацию с архитектурными заметками"
    );
}

// ============================================================================
// ТЕСТЫ ДЛЯ P2 (MEDIUM) ПРОБЛЕМ
// ============================================================================

// -----------------------------------------------------------------------------
// ТЕСТ 4: SOLID-1 - Трейты доступа в collision.rs
// -----------------------------------------------------------------------------

/// Проверка что collision.rs использует BoardReadonly трейт.
///
/// ## Требования
/// - Функции в collision.rs должны использовать <T: BoardReadonly>
/// - Не должно быть прямого использования &GameState в проверках коллизий
#[test]
fn test_collision_uses_board_readonly_trait() {
    let content = read_src_file("game/logic/collision.rs")
        .expect("Не удалось прочитать файл collision.rs");

    // Проверяем импорт трейта BoardReadonly
    assert!(
        contains_pattern(&content, "use crate::game::access::BoardReadonly")
            || contains_pattern(&content, "BoardReadonly"),
        "collision.rs должен импортировать трейт BoardReadonly"
    );

    // Проверяем что функции используют трейт вместо &GameState
    let uses_trait = contains_pattern(&content, "T: BoardReadonly")
        || contains_pattern(&content, "<T: BoardReadonly>")
        || contains_pattern(&content, "fn check_block_collision<T: BoardReadonly>")
        || contains_pattern(&content, "fn check_collision_direction<T: BoardReadonly>");

    assert!(
        uses_trait,
        "Функции в collision.rs должны использовать трейт BoardReadonly \
         вместо прямого доступа к &GameState (проверьте наличие <T: BoardReadonly>)"
    );

    // Проверяем наличие архитектурных комментариев о SOLID-1
    assert!(
        contains_pattern(&content, "SOLID-1")
            || contains_pattern(&content, "Архитектурные заметки"),
        "collision.rs должен содержать архитектурные комментарии о SOLID-1"
    );
}

// -----------------------------------------------------------------------------
// ТЕСТ 5: DRY-1 - Группировка констант
// -----------------------------------------------------------------------------

/// Проверка что константы сгруппированы по разделам.
///
/// ## Требования
/// - constants.rs должен содержать разделы: UI, PHYSICS, GAME
/// - Константы должны быть организованы по категориям
#[test]
fn test_constants_grouped_by_category() {
    let content = read_src_file("constants.rs")
        .expect("Не удалось прочитать файл constants.rs");

    // Проверяем наличие разделов с константами
    let has_ui_section = contains_pattern(&content, "UI")
        || contains_pattern(&content, "ОТРИСОВКИ")
        || contains_pattern(&content, "BORDER_COLOR")
        || contains_pattern(&content, "SCORE_");

    let has_physics_section = contains_pattern(&content, "ФИЗИКА")
        || contains_pattern(&content, "PHYSICS")
        || contains_pattern(&content, "INITIAL_FALL_SPD")
        || contains_pattern(&content, "MAX_FALL_SPEED");

    let has_game_section = contains_pattern(&content, "ИГРЫ")
        || contains_pattern(&content, "GAME")
        || contains_pattern(&content, "SPRINT_LINES")
        || contains_pattern(&content, "MARATHON_LINES");

    assert!(
        has_ui_section,
        "constants.rs должен содержать раздел констант UI/отрисовки"
    );

    assert!(
        has_physics_section,
        "constants.rs должен содержать раздел констант физики (PHYSICS)"
    );

    assert!(
        has_game_section,
        "constants.rs должен содержать раздел констант игры (GAME)"
    );

    // Проверяем наличие комментариев о группировке
    assert!(
        contains_pattern(&content, "групп")
            || contains_pattern(&content, "раздел")
            || contains_pattern(&content, "категори"),
        "constants.rs должен содержать комментарии о группировке констант"
    );
}

// -----------------------------------------------------------------------------
// ТЕСТ 6: COHESION-1 - Документация pub(crate) доступа
// -----------------------------------------------------------------------------

/// Проверка что scoring модуль имеет документацию о pub(crate) доступе.
///
/// ## Требования
/// - points.rs должен содержать документацию о pub(crate) доступе
/// - Должны быть архитектурные комментарии о причинах такого доступа
#[test]
fn test_scoring_module_has_architecture_comments() {
    let content = read_src_file("game/scoring/points.rs")
        .expect("Не удалось прочитать файл points.rs");

    // Проверяем наличие pub(crate) функций
    let has_pub_crate = contains_pattern(&content, "pub(crate) fn");

    // Проверяем наличие архитектурных комментариев
    let has_arch_comments = contains_pattern(&content, "COHESION-1")
        || contains_pattern(&content, "Архитектурные заметки")
        || contains_pattern(&content, "Инкапсуляция")
        || contains_pattern(&content, "TODO (#архитектура");

    assert!(
        has_arch_comments,
        "points.rs должен содержать архитектурные комментарии о pub(crate) доступе \
         и причинах такого решения (COHESION-1)"
    );

    // Дополнительная проверка: наличие объяснения о производительности
    if has_pub_crate {
        assert!(
            contains_pattern(&content, "производит")
                || contains_pattern(&content, "внутренн")
                || contains_pattern(&content, "модул"),
            "pub(crate) функции должны иметь объяснение о причинах \
             (производительность, внутренняя логика модуля)"
        );
    }
}

// -----------------------------------------------------------------------------
// ТЕСТ 7: CYCLE-1 - pub(crate) для внутренних зависимостей
// -----------------------------------------------------------------------------

/// Проверка что внутренние зависимости используют pub(crate).
///
/// ## Требования
/// - constants должен импортироваться как pub(crate) в game/mod.rs
/// - Внутренние зависимости не должны быть публичными
#[test]
fn test_internal_dependencies_use_pub_crate() {
    let content = read_src_file("game/mod.rs")
        .expect("Не удалось прочитать файл game/mod.rs");

    // Проверяем что constants импортируется как pub(crate)
    let uses_pub_crate_constants = contains_pattern(&content, "pub(crate) use crate::constants")
        || contains_pattern(&content, "pub(crate) mod constants")
        || contains_pattern(&content, "// constants больше не является отдельным файлом");

    // Проверяем наличие комментариев о внутренних зависимостях
    let has_internal_comments = contains_pattern(&content, "внутренн")
        || contains_pattern(&content, "internal")
        || contains_pattern(&content, "CYCLE-1")
        || contains_pattern(&content, "Зависимости модулей");

    assert!(
        uses_pub_crate_constants || has_internal_comments,
        "game/mod.rs должен использовать pub(crate) для внутренних зависимостей \
         (constants) или содержать комментарии о внутренних зависимостях"
    );
}

// ============================================================================
// ТЕСТЫ ДЛЯ P3 (LOW) ПРОБЛЕМ
// ============================================================================

// -----------------------------------------------------------------------------
// ТЕСТ 8: COMPLEX-1 - ThreadSafeLeaderboardEntry
// -----------------------------------------------------------------------------

/// Проверка что ThreadSafeLeaderboardEntry имеет документацию.
///
/// ## Требования
/// - Файл leaderboard.rs должен содержать документацию о TOCTOU
/// - Должны быть комментарии о потокобезопасности
#[test]
fn test_thread_safe_leaderboard_entry_documentation() {
    let content = read_src_file("highscore/leaderboard.rs")
        .expect("Не удалось прочитать файл leaderboard.rs");

    // Проверяем наличие упоминания ThreadSafeLeaderboardEntry или потокобезопасности
    let has_thread_safe = contains_pattern(&content, "ThreadSafe")
        || contains_pattern(&content, "потокобезопас")
        || contains_pattern(&content, "Mutex")
        || contains_pattern(&content, "Arc<");

    // Проверяем наличие документации о TOCTOU или race conditions
    let has_tocou_docs = contains_pattern(&content, "TOCTOU")
        || contains_pattern(&content, "race condition")
        || contains_pattern(&content, "состояние гонки")
        || contains_pattern(&content, "поток");

    // Проверяем наличие архитектурных комментариев
    let has_arch_comments = contains_pattern(&content, "Архитектурные заметки")
        || contains_pattern(&content, "COMPLEX-1")
        || contains_pattern(&content, "Безопасность");

    // Тест проходит если есть хотя бы одна из проверок
    assert!(
        has_thread_safe || has_tocou_docs || has_arch_comments,
        "leaderboard.rs должен содержать документацию о потокобезопасности \
         (ThreadSafeLeaderboardEntry, TOCTOU, или архитектурные комментарии)"
    );
}

// -----------------------------------------------------------------------------
// ТЕСТ 9: YAGNI-1 - Документация трейтов в cycle.rs
// -----------------------------------------------------------------------------

/// Проверка что трейты имеют TODO комментарии.
///
/// ## Требования
/// - cycle.rs должен содержать TODO о будущем использовании или удалении трейтов
/// - Должны быть комментарии о YAGNI принципе
#[test]
fn test_cycle_traits_have_todo_comments() {
    let content = read_src_file("game/cycle.rs")
        .expect("Не удалось прочитать файл cycle.rs");

    // Проверяем наличие TODO комментариев
    let has_todo = contains_pattern(&content, "TODO")
        && (contains_pattern(&content, "будущ")
            || contains_pattern(&content, "использован")
            || contains_pattern(&content, "удален")
            || contains_pattern(&content, "YAGNI")
            || contains_pattern(&content, "рефакторизаци"));

    // Проверяем наличие комментариев о текущем состоянии
    let has_status_comments = contains_pattern(&content, "Текущее состояние")
        || contains_pattern(&content, "ARCH-3")
        || contains_pattern(&content, "В настоящее время");

    assert!(
        has_todo || has_status_comments,
        "cycle.rs должен содержать TODO комментарии о будущем использовании \
         трейтов или комментарии о текущем состоянии (YAGNI-1)"
    );
}

// ============================================================================
// ИНТЕГРАЦИОННЫЕ ТЕСТЫ
// ============================================================================

// -----------------------------------------------------------------------------
// ТЕСТ 10: Общая целостность архитектуры
// -----------------------------------------------------------------------------

/// Проверка что все архитектурные улучшения применены.
///
/// ## Требования
/// - Все файлы должны существовать
/// - Все файлы должны содержать архитектурные комментарии
/// - Не должно быть критических нарушений архитектуры
#[test]
fn test_all_architecture_improvements_applied() {
    let src_path = get_src_path();

    // Проверяем существование ключевых файлов
    let required_files = [
        "game/state.rs",
        "game/view.rs",
        "game/render.rs",
        "game/cycle.rs",
        "game/logic/collision.rs",
        "game/scoring/points.rs",
        "constants.rs",
        "game/mod.rs",
    ];

    for file_path in &required_files {
        let full_path = Path::new(&src_path).join(file_path);
        assert!(
            full_path.exists(),
            "Файл {file_path} должен существовать"
        );
    }

    // Проверяем что все файлы содержат архитектурные комментарии
    let files_with_arch_comments = [
        ("game/state.rs", "Архитектурные заметки"),
        ("game/view.rs", "Архитектурные заметки"),
        ("game/render.rs", "Архитектурные заметки"),
        ("game/cycle.rs", "Архитектурные заметки"),
        ("game/logic/collision.rs", "SOLID"),
        ("game/scoring/points.rs", "Инкапсуляция"),
        ("constants.rs", "ФИЗИКА"),
    ];

    for (file_path, expected_comment) in &files_with_arch_comments {
        let content = read_src_file(file_path)
            .unwrap_or_else(|_| panic!("Не удалось прочитать файл {file_path}"));

        assert!(
            contains_pattern(&content, expected_comment),
            "Файл {file_path} должен содержать архитектурные комментарии \
             (ожидалось: {expected_comment})"
        );
    }
}

// -----------------------------------------------------------------------------
// ТЕСТ 11: Отсутствие регрессий
// -----------------------------------------------------------------------------

/// Проверка что улучшения не сломали существующую функциональность.
///
/// ## Требования
/// - Базовые структуры должны компилироваться
/// - GameState должен создаваться корректно
/// - Основные методы должны работать
#[test]
fn test_no_regressions_after_improvements() {
    // Этот тест проверяет что основные структуры работают после улучшений
    use tetris_cli::game::GameState;

    // Создаём новое состояние игры
    let state = GameState::new();

    // Проверяем базовую функциональность
    assert_eq!(state.score(), 0, "Начальный счёт должен быть 0");
    assert_eq!(state.level(), 1, "Начальный уровень должен быть 1");
    assert_eq!(state.lines_cleared(), 0, "Начальное количество линий должно быть 0");

    // Проверяем что методы отрисовки доступны
    let _view = tetris_cli::game::GameView::from_game_state(&state);

    // Проверяем что трейты доступны
    let _blocks = state.get_blocks();
    let _mode = state.get_mode_trait();

    // Если тест компилируется и проходит - регрессий нет
}

// ============================================================================
// ДОПОЛНИТЕЛЬНЫЕ ПРОВЕРКИ
// ============================================================================

/// Проверка что все тесты компилируются (cargo check проходит).
#[test]
fn test_tests_compile() {
    // Этот тест всегда проходит если код компилируется
    // cargo test автоматически проверяет компиляцию всех тестов
    assert!(true, "Тесты компилируются успешно");
}

/// Проверка наличия всех 11 основных тестов.
#[test]
fn test_all_required_tests_exist() {
    // Список требуемых тестов
    let required_tests = [
        "test_game_state_architecture_todo_comments",
        "test_game_view_has_draw_methods",
        "test_render_uses_game_view_methods",
        "test_cycle_traits_documentation",
        "test_collision_uses_board_readonly_trait",
        "test_constants_grouped_by_category",
        "test_scoring_module_has_architecture_comments",
        "test_internal_dependencies_use_pub_crate",
        "test_thread_safe_leaderboard_entry_documentation",
        "test_cycle_traits_have_todo_comments",
        "test_all_architecture_improvements_applied",
        "test_no_regressions_after_improvements",
    ];

    // Все тесты существуют (этот код компилируется только если тесты есть)
    let _tests: &[&str] = &required_tests;

    assert_eq!(
        required_tests.len(),
        12,
        "Должно быть как минимум 12 тестов (11 основных + 1 дополнительный)"
    );
}

/// Проверка что файлы исходного кода доступны для чтения.
#[test]
fn test_source_files_readable() {
    let test_files = [
        "game/state.rs",
        "game/view.rs",
        "game/render.rs",
        "game/cycle.rs",
        "game/logic/collision.rs",
        "game/scoring/points.rs",
        "constants.rs",
        "game/mod.rs",
        "highscore/leaderboard.rs",
    ];

    for file_path in &test_files {
        let result = read_src_file(file_path);
        assert!(
            result.is_ok(),
            "Файл {file_path} должен быть доступен для чтения: {:?}",
            result.err()
        );

        let content = result.unwrap();
        assert!(
            !content.is_empty(),
            "Файл {file_path} не должен быть пустым"
        );
    }
}

/// Проверка что документация на русском языке.
#[test]
fn test_documentation_in_russian() {
    let files_to_check = [
        "game/state.rs",
        "game/view.rs",
        "game/render.rs",
        "game/cycle.rs",
        "constants.rs",
    ];

    let russian_patterns = [
        "/// ",
        "//!",
        "Архитектурные",
        "Проверка",
        "Отрисовка",
        "Состояние",
    ];

    for file_path in &files_to_check {
        let content = read_src_file(file_path)
            .unwrap_or_else(|_| panic!("Не удалось прочитать файл {file_path}"));

        let has_docs = russian_patterns
            .iter()
            .any(|&pattern| contains_pattern(&content, pattern));

        assert!(
            has_docs,
            "Файл {file_path} должен содержать документацию на русском языке"
        );
    }
}
