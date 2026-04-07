//! Трейт режима игры.
//!
//! Этот модуль предоставляет трейт `GameModeTrait` для абстракции режимов игры.
//! Позволяет легко добавлять новые режимы без изменения существующего кода.
//!
//! ## Архитектурные заметки
//!
//! Трейт используется для:
//! - Уменьшения связанности между модулями
//! - Упрощения тестирования через моки
//! - Расширяемости (добавление новых режимов без изменения кода)
//!
//! ## Пример использования
//!
//! ```ignore
//! use crate::game::mode_trait::{GameModeTrait, ClassicMode};
//!
//! fn play_game<T: GameModeTrait>(mode: &T) {
//!     println!("Играем в режим: {}", mode.name());
//!     if mode.check_win_condition(40) {
//!         println!("Победа!");
//!     }
//! }
//!
//! let mode = ClassicMode;
//! play_game(&mode);
//! ```

/// Трейт режима игры.
///
/// Определяет интерфейс для всех режимов игры.
/// Каждый режим должен реализовать проверку условия победы,
/// получение целевого количества линий и название режима.
///
/// # Реализации
/// Этот трейт реализован для следующих режимов:
/// | Режим | Описание | Цель |
/// |-------|----------|------|
/// | [`ClassicMode`] | Классическая игра до проигрыша | Нет цели (бесконечная игра) |
/// | [`SprintMode`] | Спринт на скорость | Очистить 40 линий как можно быстрее |
/// | [`MarathonMode`] | Марафон на выносливость | Очистить 150 линий с нарастающей сложностью |
///
/// # Методы
/// - [`check_win_condition`](Self::check_win_condition) - проверка условия победы
/// - [`get_target_lines`](Self::get_target_lines) - получение целевого количества линий
/// - [`name`](Self::name) - получение названия режима
///
/// # Пример реализации
/// ```ignore
/// use crate::game::mode_trait::GameModeTrait;
///
/// struct CustomMode;
///
/// impl GameModeTrait for CustomMode {
///     fn check_win_condition(&self, lines: u32) -> bool {
///         lines >= 100
///     }
///
///     fn get_target_lines(&self) -> Option<u32> {
///         Some(100)
///     }
///
///     fn name(&self) -> &str {
///         "Custom"
///     }
/// }
/// ```
pub trait GameModeTrait: Send + Sync {
    /// Проверить условие победы.
    ///
    /// # Аргументы
    /// * `lines` - количество очищенных линий
    ///
    /// # Возвращает
    /// `true` если условие победы выполнено, `false` иначе
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::mode_trait::{GameModeTrait, SprintMode};
    ///
    /// let mode = SprintMode::new();
    /// assert!(!mode.check_win_condition(39));  // Ещё не победа
    /// assert!(mode.check_win_condition(40));   // Победа!
    /// ```
    fn check_win_condition(&self, lines: u32) -> bool;

    /// Получить целевое количество линий.
    ///
    /// # Возвращает
    /// - `Some(target)` для режимов с целью (Sprint, Marathon)
    /// - `None` для режимов без цели (Classic)
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::mode_trait::{GameModeTrait, SprintMode, ClassicMode};
    ///
    /// let sprint = SprintMode::new();
    /// assert_eq!(sprint.get_target_lines(), Some(40));
    ///
    /// let classic = ClassicMode;
    /// assert_eq!(classic.get_target_lines(), None);
    /// ```
    fn get_target_lines(&self) -> Option<u32>;

    /// Получить название режима.
    ///
    /// # Возвращает
    /// Человекочитаемое название режима на русском языке
    ///
    /// # Пример использования
    /// ```
    /// use tetris_cli::game::mode_trait::{GameModeTrait, SprintMode};
    ///
    /// let mode = SprintMode::new();
    /// assert_eq!(mode.name(), "Спринт");
    /// ```
    fn name(&self) -> &str;
}

/// Классический режим игры.
///
/// Игра до проигрыша (заполнения поля).
/// Победного условия нет.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassicMode;

impl GameModeTrait for ClassicMode {
    fn check_win_condition(&self, _lines: u32) -> bool {
        // В классическом режиме нет победы
        false
    }

    fn get_target_lines(&self) -> Option<u32> {
        // В классическом режиме нет цели по линиям
        None
    }

    fn name(&self) -> &'static str {
        "Классика"
    }
}

/// Режим спринт.
///
/// Цель: очистить 40 линий как можно быстрее.
#[derive(Debug, Clone, Copy)]
pub struct SprintMode {
    /// Целевое количество линий.
    target: u32,
}

impl SprintMode {
    /// Создать новый режим спринт.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SprintMode {
    fn default() -> Self {
        Self { target: 40 }
    }
}

impl GameModeTrait for SprintMode {
    fn check_win_condition(&self, lines: u32) -> bool {
        lines >= self.target
    }

    fn get_target_lines(&self) -> Option<u32> {
        Some(self.target)
    }

    fn name(&self) -> &'static str {
        "Спринт"
    }
}

/// Режим марафон.
///
/// Цель: очистить 150 линий с нарастающей сложностью.
#[derive(Debug, Clone, Copy)]
pub struct MarathonMode {
    /// Целевое количество линий.
    target: u32,
}

impl MarathonMode {
    /// Создать новый режим марафон.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MarathonMode {
    fn default() -> Self {
        Self { target: 150 }
    }
}

impl GameModeTrait for MarathonMode {
    fn check_win_condition(&self, lines: u32) -> bool {
        lines >= self.target
    }

    fn get_target_lines(&self) -> Option<u32> {
        Some(self.target)
    }

    fn name(&self) -> &'static str {
        "Марафон"
    }
}

// ============================================================================
// FACTORY ФУНКЦИЯ ДЛЯ СОЗДАНИЯ РЕЖИМОВ (Архитектурное улучшение 2026-04-01)
// ============================================================================

/// Тип результата для factory функции.
pub type GameModeResult = Box<dyn GameModeTrait>;

/// Factory функция для создания режима игры.
///
/// # Аргументы
/// * `name` - название режима ("classic", "sprint", "marathon")
///
/// # Возвращает
/// - `Some(Box<dyn GameModeTrait>)` - объект режима
/// - `None` - если название режима не распознано
///
/// # Пример использования
/// ```ignore
/// use crate::game::mode_trait::create_game_mode;
///
/// let mode = create_game_mode("sprint").expect("sprint mode");
/// assert_eq!(mode.name(), "Спринт");
/// ```
///
/// Архитектурное улучшение 2026-04-01 (O1): Делегирует [`ModeRegistry::global()`](super::mode_registry::ModeRegistry::global).
#[must_use]
pub fn create_game_mode(name: &str) -> Option<GameModeResult> {
    super::mode_registry::ModeRegistry::global().create(name)
}

/// Factory функция для создания режима игры по умолчанию (Classic).
///
/// # Возвращает
/// `Box<dyn GameModeTrait>` с режимом Classic
///
/// Архитектурное улучшение 2026-04-01 (O1): Делегирует [`ModeRegistry::global()`](super::mode_registry::ModeRegistry::global).
#[must_use]
pub fn create_default_game_mode() -> GameModeResult {
    super::mode_registry::ModeRegistry::global()
        .create("classic")
        .unwrap_or_else(|| Box::new(ClassicMode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_mode_no_win_condition() {
        let mode = ClassicMode;
        assert!(!mode.check_win_condition(0));
        assert!(!mode.check_win_condition(40));
        assert!(!mode.check_win_condition(1000));
        assert_eq!(mode.get_target_lines(), None);
        assert_eq!(mode.name(), "Классика");
    }

    #[test]
    fn test_sprint_mode_win_condition() {
        let mode = SprintMode::new();
        assert!(!mode.check_win_condition(0));
        assert!(!mode.check_win_condition(39));
        assert!(mode.check_win_condition(40));
        assert!(mode.check_win_condition(50));
        assert_eq!(mode.get_target_lines(), Some(40));
        assert_eq!(mode.name(), "Спринт");
    }

    #[test]
    fn test_marathon_mode_win_condition() {
        let mode = MarathonMode::new();
        assert!(!mode.check_win_condition(0));
        assert!(!mode.check_win_condition(149));
        assert!(mode.check_win_condition(150));
        assert!(mode.check_win_condition(200));
        assert_eq!(mode.get_target_lines(), Some(150));
        assert_eq!(mode.name(), "Марафон");
    }

    #[test]
    fn test_trait_object() {
        // Проверка что трейт может использоваться как trait object
        let modes: Vec<Box<dyn GameModeTrait>> = vec![
            Box::new(ClassicMode),
            Box::new(SprintMode::new()),
            Box::new(MarathonMode::new()),
        ];

        assert_eq!(modes[0].name(), "Классика");
        assert_eq!(modes[1].name(), "Спринт");
        assert_eq!(modes[2].name(), "Марафон");

        assert!(!modes[0].check_win_condition(100));
        assert!(modes[1].check_win_condition(40));
        assert!(modes[2].check_win_condition(150));
    }

    /// Тест: проверка динамической диспетчеризации трейтов
    /// Исправление #33: добавлен тест для trait objects
    #[test]
    fn test_trait_object_dynamic_dispatch() {
        let modes: Vec<Box<dyn GameModeTrait>> = vec![
            Box::new(ClassicMode),
            Box::new(SprintMode::new()),
            Box::new(MarathonMode::new()),
        ];

        for (i, mode) in modes.iter().enumerate() {
            match i {
                0 => assert_eq!(mode.name(), "Классика"),
                1 => assert_eq!(mode.name(), "Спринт"),
                2 => assert_eq!(mode.name(), "Марафон"),
                _ => panic!(
                    "Неизвестный индекс режима: {i}. Если добавлен новый режим, обновите этот match."
                ),
            }
        }
    }

    /// Тест: проверка clone для режимов
    #[test]
    fn test_mode_clone() {
        let sprint = SprintMode::new();
        let sprint_clone = sprint;
        assert_eq!(sprint_clone.get_target_lines(), Some(40));

        let marathon = MarathonMode::new();
        let marathon_clone = marathon;
        assert_eq!(marathon_clone.get_target_lines(), Some(150));
    }

    /// Тест: проверка граничных значений для `SprintMode`
    #[test]
    fn test_sprint_mode_boundaries() {
        let mode = SprintMode::new();

        // Граница: 39 линий - ещё не победа
        assert!(!mode.check_win_condition(39));

        // Граница: 40 линий - победа
        assert!(mode.check_win_condition(40));

        // Выше границы - тоже победа
        assert!(mode.check_win_condition(41));
        assert!(mode.check_win_condition(100));
    }

    /// Тест: проверка граничных значений для `MarathonMode`
    #[test]
    fn test_marathon_mode_boundaries() {
        let mode = MarathonMode::new();

        // Граница: 149 линий - ещё не победа
        assert!(!mode.check_win_condition(149));

        // Граница: 150 линий - победа
        assert!(mode.check_win_condition(150));

        // Выше границы - тоже победа
        assert!(mode.check_win_condition(151));
        assert!(mode.check_win_condition(500));
    }

    /// Тест: factory функция для создания режимов
    #[test]
    fn test_create_game_mode_classic() {
        let mode = create_game_mode("classic").unwrap();
        assert_eq!(mode.name(), "Классика");
        assert_eq!(mode.get_target_lines(), None);
        assert!(!mode.check_win_condition(1000));
    }

    #[test]
    fn test_create_game_mode_sprint() {
        let mode = create_game_mode("sprint").unwrap();
        assert_eq!(mode.name(), "Спринт");
        assert_eq!(mode.get_target_lines(), Some(40));
        assert!(mode.check_win_condition(40));
    }

    #[test]
    fn test_create_game_mode_marathon() {
        let mode = create_game_mode("marathon").unwrap();
        assert_eq!(mode.name(), "Марафон");
        assert_eq!(mode.get_target_lines(), Some(150));
        assert!(mode.check_win_condition(150));
    }

    #[test]
    fn test_create_game_mode_russian_names() {
        assert_eq!(create_game_mode("классика").unwrap().name(), "Классика");
        assert_eq!(create_game_mode("спринт").unwrap().name(), "Спринт");
        assert_eq!(create_game_mode("марафон").unwrap().name(), "Марафон");
    }

    #[test]
    fn test_create_game_mode_invalid() {
        assert!(create_game_mode("invalid").is_none());
        assert!(create_game_mode("").is_none());
    }

    #[test]
    fn test_create_default_game_mode() {
        let mode = create_default_game_mode();
        assert_eq!(mode.name(), "Классика");
        assert_eq!(mode.get_target_lines(), None);
    }
}
