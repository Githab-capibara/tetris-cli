//! Модуль компонентов игрового состояния.
//!
//! Этот модуль содержит компоненты для разделения `GameState`:
//! - [`animation_state`] — состояние анимаций
//! - [`board_state`] — состояние игрового поля
//! - [`figure_manager`] — менеджер фигур
//!
//! ## Архитектурные заметки
//! Выделено из `GameState` для улучшения организации кода и разделения ответственности.
//! Каждый компонент инкапсулирует своё состояние и предоставляет методы доступа.

// Подмодули
pub mod animation_state;
pub mod board_state;
pub mod figure_manager;

// Re-export для удобства использования
pub use animation_state::AnimationState;
pub use figure_manager::FigureManager;
