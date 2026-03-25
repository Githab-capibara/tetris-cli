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
pub trait GameModeTrait: Send + Sync {
    /// Проверить условие победы.
    ///
    /// # Аргументы
    /// * `lines` - количество очищенных линий
    ///
    /// # Возвращает
    /// `true` если условие победы выполнено
    fn check_win_condition(&self, lines: u32) -> bool;

    /// Получить целевое количество линий.
    ///
    /// # Возвращает
    /// `Some(target)` для режимов с целью (Sprint, Marathon)
    /// `None` для режимов без цели (Classic)
    fn get_target_lines(&self) -> Option<u32>;

    /// Получить название режима.
    ///
    /// # Возвращает
    /// Человекочитаемое название режима
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

    fn name(&self) -> &str {
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
        Self { target: 40 }
    }
}

impl Default for SprintMode {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeTrait for SprintMode {
    fn check_win_condition(&self, lines: u32) -> bool {
        lines >= self.target
    }

    fn get_target_lines(&self) -> Option<u32> {
        Some(self.target)
    }

    fn name(&self) -> &str {
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
        Self { target: 150 }
    }
}

impl Default for MarathonMode {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeTrait for MarathonMode {
    fn check_win_condition(&self, lines: u32) -> bool {
        lines >= self.target
    }

    fn get_target_lines(&self) -> Option<u32> {
        Some(self.target)
    }

    fn name(&self) -> &str {
        "Марафон"
    }
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
}
