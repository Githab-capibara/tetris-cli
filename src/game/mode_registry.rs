//! Реестр режимов игры.
//!
//! # Ответственность
//! - Регистрация режимов игры
//! - Создание экземпляров режимов по имени
//! - Управление коллекцией доступных режимов
//!
//! ## Архитектурные заметки
//! Выделено для соблюдения Open/Closed Principle (OCP).
//! Позволяет добавлять новые режимы без изменения существующего кода.
//!
//! Архитектурное улучшение 2026-04-01 (HIGH #5): `ModeRegistry` для регистрации режимов

use std::collections::HashMap;

use super::mode_trait::GameModeTrait;
use super::mode_trait::{ClassicMode, MarathonMode, SprintMode};

/// Тип функции-фабрики для создания режимов.
pub type ModeFactory = Box<dyn Fn() -> Box<dyn GameModeTrait> + Send + Sync>;

/// Реестр режимов игры.
///
/// Предоставляет централизованную регистрацию и создание режимов игры.
///
/// ## Архитектурные заметки
/// Выделено для соблюдения Open/Closed Principle (OCP).
/// Позволяет добавлять новые режимы через регистрацию без изменения кода.
///
/// ## Пример использования
/// ```ignore
/// use tetris_cli::game::mode_registry::ModeRegistry;
///
/// // Получаем глобальный реестр
/// let registry = ModeRegistry::global();
///
/// // Создаём режим по имени
/// let mode = registry.create("classic").unwrap();
/// ```
pub struct ModeRegistry {
    /// Карта зарегистрированных режимов.
    factories: HashMap<String, ModeFactory>,
}

impl ModeRegistry {
    /// Создать новый реестр режимов.
    ///
    /// # Возвращает
    /// Новый экземпляр `ModeRegistry` с зарегистрированными стандартными режимами
    #[must_use = "Реестр режимов должен быть использован"]
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };

        // Регистрируем стандартные режимы
        registry.register_default_modes();

        registry
    }

    /// Зарегистрировать стандартные режимы.
    fn register_default_modes(&mut self) {
        self.register("classic", Box::new(|| Box::new(ClassicMode)));
        self.register("sprint", Box::new(|| Box::new(SprintMode::new())));
        self.register("marathon", Box::new(|| Box::new(MarathonMode::new())));

        // Русские названия для обратной совместимости
        self.register("классика", Box::new(|| Box::new(ClassicMode)));
        self.register("спринт", Box::new(|| Box::new(SprintMode::new())));
        self.register("марафон", Box::new(|| Box::new(MarathonMode::new())));
    }

    /// Зарегистрировать новый режим игры.
    ///
    /// # Аргументы
    /// * `name` - имя режима (ключ для создания)
    /// * `factory` - функция-фабрика для создания экземпляра режима
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::mode_registry::ModeRegistry;
    ///
    /// let mut registry = ModeRegistry::new();
    /// registry.register("custom", Box::new(|| Box::new(CustomMode)));
    /// ```
    pub fn register(&mut self, name: &str, factory: ModeFactory) {
        self.factories.insert(name.to_lowercase(), factory);
    }

    /// Создать экземпляр режима по имени.
    ///
    /// # Аргументы
    /// * `name` - имя зарегистрированного режима
    ///
    /// # Возвращает
    /// - `Some(Box<dyn GameModeTrait>)` - новый экземпляр режима
    /// - `None` - если режим с таким именем не зарегистрирован
    ///
    /// # Пример
    /// ```ignore
    /// use tetris_cli::game::mode_registry::ModeRegistry;
    ///
    /// let registry = ModeRegistry::new();
    /// let mode = registry.create("sprint").unwrap();
    /// assert_eq!(mode.name(), "Спринт");
    /// ```
    #[must_use = "Режим игры должен быть использован"]
    pub fn create(&self, name: &str) -> Option<Box<dyn GameModeTrait>> {
        let factory = self.factories.get(&name.to_lowercase())?;
        Some(factory())
    }

    /// Проверить, зарегистрирован ли режим с таким именем.
    ///
    /// # Аргументы
    /// * `name` - имя режима для проверки
    ///
    /// # Возвращает
    /// `true` если режим зарегистрирован
    #[must_use = "Результат проверки регистрации режима должен быть использован"]
    pub fn is_registered(&self, name: &str) -> bool {
        self.factories.contains_key(&name.to_lowercase())
    }

    /// Получить список зарегистрированных имён режимов.
    ///
    /// # Возвращает
    /// Вектор с именами всех зарегистрированных режимов
    #[must_use = "Список зарегистрированных режимов должен быть использован"]
    pub fn registered_names(&self) -> Vec<&str> {
        self.factories
            .keys()
            .map(std::string::String::as_str)
            .collect()
    }

    /// Получить глобальный реестр режимов.
    ///
    /// # Возвращает
    /// Ссылка на глобальный экземпляр реестра
    ///
    /// # Примечания
    /// Использует `std::sync::OnceLock` для ленивой инициализации.
    pub fn global() -> &'static Self {
        use std::sync::OnceLock;

        static REGISTRY: OnceLock<ModeRegistry> = OnceLock::new();
        REGISTRY.get_or_init(ModeRegistry::new)
    }
}

impl Default for ModeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory функция для создания режима игры через глобальный реестр.
///
/// # Аргументы
/// * `name` - название режима
///
/// # Возвращает
/// - `Some(Box<dyn GameModeTrait>)` - объект режима
/// - `None` - если режим не зарегистрирован
///
/// # Пример использования
/// ```ignore
/// use tetris_cli::game::mode_registry::create_mode;
///
/// let mode = create_mode("sprint").unwrap();
/// assert_eq!(mode.name(), "Спринт");
/// ```
#[must_use]
pub fn create_mode(name: &str) -> Option<Box<dyn GameModeTrait>> {
    ModeRegistry::global().create(name)
}

/// Factory функция для создания режима игры по умолчанию (Classic).
///
/// # Возвращает
/// `Box<dyn GameModeTrait>` с режимом Classic
#[must_use]
pub fn create_default_mode() -> Box<dyn GameModeTrait> {
    ModeRegistry::global()
        .create("classic")
        .unwrap_or_else(|| Box::new(ClassicMode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = ModeRegistry::new();
        assert!(registry.is_registered("classic"));
        assert!(registry.is_registered("sprint"));
        assert!(registry.is_registered("marathon"));
    }

    #[test]
    fn test_registry_create() {
        let registry = ModeRegistry::new();

        let classic = registry.create("classic").unwrap();
        assert_eq!(classic.name(), "Классика");

        let sprint = registry.create("sprint").unwrap();
        assert_eq!(sprint.name(), "Спринт");

        let marathon = registry.create("marathon").unwrap();
        assert_eq!(marathon.name(), "Марафон");
    }

    #[test]
    fn test_registry_create_invalid() {
        let registry = ModeRegistry::new();
        assert!(registry.create("invalid").is_none());
    }

    #[test]
    fn test_registry_case_insensitive() {
        let registry = ModeRegistry::new();
        assert!(registry.is_registered("CLASSIC"));
        assert!(registry.is_registered("Classic"));
        assert!(registry.is_registered("classic"));
    }

    #[test]
    fn test_registry_russian_names() {
        let registry = ModeRegistry::new();
        assert!(registry.is_registered("классика"));
        assert!(registry.is_registered("спринт"));
        assert!(registry.is_registered("марафон"));
    }

    #[test]
    fn test_global_registry() {
        let registry = ModeRegistry::global();
        assert!(registry.is_registered("classic"));
    }

    #[test]
    fn test_create_mode_function() {
        let mode = create_mode("sprint");
        assert!(mode.is_some());
        assert_eq!(mode.unwrap().name(), "Спринт");
    }

    #[test]
    fn test_create_default_mode_function() {
        let mode = create_default_mode();
        assert_eq!(mode.name(), "Классика");
    }
}
