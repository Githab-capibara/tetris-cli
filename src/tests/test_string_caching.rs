//! Тесты кэширования строк.
//!
//! Проверяют, что кэш строк счёта, уровня и линий обновляется корректно.

use crate::game::GameState;

/// Тест 1: Проверка, что кэш обновляется при изменении счёта
///
/// При изменении счёта кэшированная строка должна обновляться.
#[test]
fn test_cache_update_on_score_change() {
    let mut state = GameState::new();

    // Начальный счёт
    assert_eq!(
        state.get_cached_score_str().trim(),
        "0",
        "Начальный счёт должен быть '0'"
    );
    assert_eq!(
        state.render_cache().last_cached_score,
        0,
        "last_cached_score должен быть 0"
    );

    // Изменяем счёт
    state.set_score(100);

    // Кэш должен обновиться (через update_cached_strings_extended)
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    assert_eq!(
        state.get_cached_score_str().trim(),
        "100",
        "Кэш счёта должен обновиться до '100'"
    );
    assert_eq!(
        state.render_cache().last_cached_score,
        100,
        "last_cached_score должен обновиться до 100"
    );
}

/// Тест 2: Проверка, что кэш не обновляется без изменений
///
/// Если счёт не изменился, кэш не должен обновляться.
#[test]
fn test_cache_no_update_without_changes() {
    let mut state = GameState::new();

    // Устанавливаем счёт и обновляем кэш
    state.set_score(500);
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    let cached_score_str = state.get_cached_score_str().to_string();
    let last_cached_score = state.render_cache().last_cached_score;

    // Обновляем кэш снова (без изменений)
    update_cached_strings_extended(&mut state, "0");

    // Кэш не должен измениться
    assert_eq!(
        state.get_cached_score_str(),
        cached_score_str,
        "Кэш не должен измениться без изменений счёта"
    );
    assert_eq!(
        state.render_cache().last_cached_score,
        last_cached_score,
        "last_cached_score не должен измениться без изменений"
    );
}

/// Тест 3: Проверка кэширования уровня
///
/// Проверяем, что кэш уровня обновляется корректно.
#[test]
fn test_level_cache_update() {
    let mut state = GameState::new();

    // Начальный уровень
    assert_eq!(
        state.render_cache().cached_level_str.trim(),
        "1",
        "Начальный уровень должен быть '1'"
    );
    assert_eq!(
        state.render_cache().last_cached_level,
        1,
        "last_cached_level должен быть 1"
    );

    // Изменяем уровень
    state.set_level(5);

    // Обновляем кэш
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    assert_eq!(
        state.render_cache().cached_level_str.trim(),
        "5",
        "Кэш уровня должен обновиться до '5'"
    );
    assert_eq!(
        state.render_cache().last_cached_level,
        5,
        "last_cached_level должен обновиться до 5"
    );
}

/// Тест 4: Проверка кэширования количества линий
///
/// Проверяем, что кэш линий обновляется корректно.
#[test]
fn test_lines_cache_update() {
    let mut state = GameState::new();

    // Начальное количество линий
    assert_eq!(
        state.render_cache().cached_lines_str.trim(),
        "0",
        "Начальное количество линий должно быть '0'"
    );
    assert_eq!(
        state.render_cache().last_cached_lines,
        0,
        "last_cached_lines должен быть 0"
    );

    // Изменяем количество линий
    state.set_lines_cleared(25);

    // Обновляем кэш
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    assert_eq!(
        state.render_cache().cached_lines_str.trim(),
        "25",
        "Кэш линий должен обновиться до '25'"
    );
    assert_eq!(
        state.render_cache().last_cached_lines,
        25,
        "last_cached_lines должен обновиться до 25"
    );
}

/// Тест 5: Проверка кэширования комбо
///
/// Проверяем, что кэш комбо обновляется корректно.
#[test]
fn test_combo_cache_update() {
    let mut state = GameState::new();

    // Начальное комбо
    assert_eq!(
        state.render_cache().cached_combo_str,
        "",
        "Начальное комбо должно быть пустой строкой"
    );
    assert_eq!(
        state.render_cache().last_cached_combo,
        0,
        "last_cached_combo должен быть 0"
    );

    // Изменяем комбо через геттер
    state.get_stats_mut().combo_counter = 3;

    // Обновляем кэш
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    // Проверяем, что кэш комбо обновился
    assert!(
        !state.render_cache().cached_combo_str.is_empty(),
        "Кэш комбо должен обновиться"
    );
    assert_eq!(
        state.render_cache().last_cached_combo,
        3,
        "last_cached_combo должен обновиться до 3"
    );
}

/// Тест 6: Проверка производительности кэширования
///
/// Бенчмарк: кэширование должно быть быстрым.
#[test]
fn test_caching_performance() {
    use std::time::Instant;

    let mut state = GameState::new();
    let iterations = 10000;

    let start = Instant::now();

    for i in 0..iterations {
        state.set_score(i as u128);
        state.set_level((i / 10) as u32 + 1);
        state.set_lines_cleared(i as u32);

        use crate::game::render::update_cached_strings_extended;
        update_cached_strings_extended(&mut state, "0");
    }

    let elapsed = start.elapsed();

    // 10000 итераций кэширования должны выполняться < 50ms
    assert!(
        elapsed.as_millis() < 50,
        "Кэширование {iterations} итераций должно выполняться < 50ms (прошло {:?})",
        elapsed
    );
}

/// Тест 7: Проверка кэширования рекорда
///
/// Проверяем, что кэш рекорда обновляется корректно.
#[test]
fn test_high_score_cache_update() {
    let mut state = GameState::new();

    // Начальный рекорд (инициализируется "0" через init_with_values)
    assert_eq!(
        state.render_cache().cached_high_score_str,
        "0",
        "Начальный рекорд должен быть '0'"
    );

    // Обновляем кэш с рекордом
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "1000");

    assert_eq!(
        state.render_cache().cached_high_score_str,
        "1000",
        "Кэш рекорда должен обновиться до '1000'"
    );
}

/// Тест 8: Проверка кэширования таймера
///
/// Проверяем, что кэш таймера обновляется корректно.
#[test]
fn test_timer_cache_update() {
    let mut state = GameState::new_sprint(); // Таймер кэшируется только для режима Sprint

    // Запускаем таймер (уже запущен в new_sprint)
    // Небольшая задержка
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Обновляем кэш
    use crate::game::render::update_cached_strings_extended;
    update_cached_strings_extended(&mut state, "0");

    // Проверяем, что кэш таймера обновился
    assert!(
        !state.render_cache().cached_timer_str.is_empty(),
        "Кэш таймера должен обновиться"
    );
}
