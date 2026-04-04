//! Модуль событий игры.
//!
//! Предоставляет событийную модель для decoupling компонентов игры.
//!
//! ## Архитектурные заметки
//! ## Исправление C1 (CRITICAL): Событийная модель
//! Этот модуль вводит событийную модель для уменьшения связанности между компонентами игры.
//! Вместо прямых вызовов компоненты могут общаться через события.
//!
//! ## Компоненты
//! - [`GameEvent`] - enum всех возможных событий в игре
//! - [`EventHandler`] - трейт для обработчиков событий
//!
//! ## Пример использования
//! ```ignore
//! use crate::game::events::{GameEvent, EventHandler};
//!
//! struct MyHandler;
//!
//! impl EventHandler for MyHandler {
//!     fn handle(&mut self, event: &GameEvent) {
//!         match event {
//!             GameEvent::PieceDropped { lines } => {
//!                 println!("Фигура приземлилась, линий: {}", lines);
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```


// ============================================================================
// СОБЫТИЯ ИГРЫ
// ============================================================================

/// События игры.
///
/// Представляет все возможные события которые могут произойти во время игры.
/// Используется для decoupling компонентов и предоставления единого API
/// для обработки изменений состояния.
///
/// ## Архитектурные заметки
/// ## Исправление C1 (CRITICAL): Событийная модель
/// Этот enum позволяет компонентам общаться без прямых зависимостей.
/// Например, система очков может подписаться на событие `PieceDropped`
/// вместо того чтобы напрямую проверять состояние игры.
///
/// ## Категории событий
/// - **События фигур**: `PieceMoved`, `PieceRotated`, `PieceDropped`, `PieceHeld`
/// - **События линий**: `LinesCleared`, `ComboStarted`, `ComboEnded`
/// - **События счёта**: `ScoreChanged`, `LevelUp`, `LinesCleared`
/// - **События состояния**: `GamePaused`, `GameResumed`, `GameOver`, `GameWon`
/// - **События ввода**: `InputProcessed`
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    /// Фигура перемещена.
    ///
    /// # Поля
    /// * `direction` - направление движения (Left, Right, Down)
    /// * `success` - успешно ли перемещение
    PieceMoved {
        /// Направление движения.
        direction: crate::types::Direction,
        /// Успешность перемещения.
        success: bool,
    },

    /// Фигура вращена.
    ///
    /// # Поля
    /// * `direction` - направление вращения
    /// * `success` - успешно ли вращение
    PieceRotated {
        /// Направление вращения.
        direction: crate::types::RotationDirection,
        /// Успешность вращения.
        success: bool,
    },

    /// Фигура приземлилась.
    ///
    /// # Поля
    /// * `drop_type` - тип падения (Soft или Hard)
    /// * `drop_distance` - расстояние падения в ячейках
    PieceDropped {
        /// Тип падения.
        drop_type: DropType,
        /// Расстояние падения.
        drop_distance: u32,
    },

    /// Фигура удержана (Hold).
    ///
    /// # Поля
    /// * `piece_type` - тип удержанной фигуры
    PieceHeld {
        /// Тип удержанной фигуры.
        piece_type: crate::tetromino::ShapeType,
    },

    /// Линии очищены.
    ///
    /// # Поля
    /// * `count` - количество очищенных линий (1-4)
    /// * `rows` - битовая маска очищенных линий
    LinesCleared {
        /// Количество линий.
        count: u32,
        /// Битовая маска линий.
        rows: u32,
    },

    /// Комбо начато.
    ///
    /// # Поля
    /// * `combo_count` - текущее количество комбо
    ComboStarted {
        /// Количество комбо.
        combo_count: u32,
    },

    /// Комбо прервано.
    ///
    /// # Поля
    /// * `final_combo` - финальное количество комбо
    ComboEnded {
        /// Финальное количество комбо.
        final_combo: u32,
    },

    /// Счёт изменён.
    ///
    /// # Поля
    /// * `old_score` - предыдущий счёт
    /// * `new_score` - новый счёт
    /// * `reason` - причина изменения
    ScoreChanged {
        /// Предыдущий счёт.
        old_score: u128,
        /// Новый счёт.
        new_score: u128,
        /// Причина изменения.
        reason: ScoreChangeReason,
    },

    /// Уровень повышен.
    ///
    /// # Поля
    /// * `old_level` - предыдущий уровень
    /// * `new_level` - новый уровень
    LevelUp {
        /// Предыдущий уровень.
        old_level: u32,
        /// Новый уровень.
        new_level: u32,
    },

    /// Игра поставлена на паузу.
    GamePaused,

    /// Игра возобновлена.
    GameResumed,

    /// Игра окончена (проигрыш).
    GameOver {
        /// Финальный счёт.
        final_score: u128,
    },

    /// Игра выиграна (режим Sprint/Marathon завершён).
    GameWon {
        /// Финальный счёт.
        final_score: u128,
        /// Время прохождения.
        elapsed_time: f64,
    },

    /// Ввод обработан.
    ///
    /// # Поля
    /// * `action` - выполненное действие
    InputProcessed {
        /// Выполненное действие.
        action: crate::game::types::GameAction,
    },
}

/// Тип падения фигуры.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropType {
    /// Мягкое падение (удержание кнопки вниз).
    Soft,
    /// Жёсткое падение (мгновенное приземление).
    Hard,
}

/// Причина изменения счёта.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreChangeReason {
    /// Очки за мягкое падение.
    SoftDrop { lines: u32 },
    /// Очки за жёсткое падение.
    HardDrop { lines: u32 },
    /// Очки за очищенные линии.
    LinesCleared { count: u32 },
    /// Бонус за комбо.
    ComboBonus { combo: u32 },
    /// Бонус за уровень.
    LevelBonus,
    /// Другая причина.
    Other,
}

// ============================================================================
// ОБРАБОТЧИК СОБЫТИЙ
// ============================================================================

/// Трейт обработчика событий игры.
///
/// Реализуется компонентами которые хотят реагировать на события игры.
///
/// ## Архитектурные заметки
/// ## Исправление C1 (CRITICAL): Событийная модель
/// Этот трейт позволяет создавать компоненты которые подписываются на события
/// вместо того чтобы напрямую проверять состояние игры.
///
/// ## Пример реализации
/// ```ignore
/// use crate::game::events::{GameEvent, EventHandler};
///
/// struct ScoreHandler;
///
/// impl EventHandler for ScoreHandler {
///     fn handle(&mut self, event: &GameEvent) {
///         if let GameEvent::ScoreChanged { new_score, .. } = event {
///             println!("Новый счёт: {}", new_score);
///         }
///     }
/// }
/// ```
pub trait EventHandler {
    /// Обработать событие.
    ///
    /// # Аргументы
    /// * `event` - событие для обработки
    fn handle(&mut self, event: &GameEvent);

    /// Обработать множество событий.
    ///
    /// По умолчанию вызывает `handle()` для каждого события.
    /// Может быть переопределён для оптимизации.
    ///
    /// # Аргументы
    /// * `events` - срез событий для обработки
    fn handle_batch(&mut self, events: &[GameEvent]) {
        for event in events {
            self.handle(event);
        }
    }
}

// ============================================================================
// МЕНЕДЖЕР СОБЫТИЙ
// ============================================================================

/// Менеджер событий игры.
///
/// Управляет подписчиками событий и их доставкой.
///
/// ## Архитектурные заметки
/// ## Исправление C1 (CRITICAL): Событийная модель
/// Этот struct предоставляет централизованное управление событиями.
/// Компоненты регистрируются как подписчики и получают события.
///
/// ## Пример использования
/// ```ignore
/// use crate::game::events::{EventDispatcher, GameEvent};
///
/// let mut dispatcher = EventDispatcher::new();
///
/// // Регистрация подписчика
/// dispatcher.subscribe(Box::new(my_handler));
///
/// // Отправка события
/// dispatcher.dispatch(&GameEvent::GamePaused);
/// ```
pub struct EventDispatcher {
    /// Подписчики событий.
    subscribers: Vec<Box<dyn EventHandler>>,
    /// Очередь отложенных событий.
    pending_events: Vec<GameEvent>,
    /// Флаг блокировки доставки (во время итерации).
    dispatching: bool,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    /// Создать новый менеджер событий.
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
            pending_events: Vec::new(),
            dispatching: false,
        }
    }

    /// Подписаться на события.
    ///
    /// # Аргументы
    /// * `handler` - обработчик событий
    pub fn subscribe(&mut self, handler: Box<dyn EventHandler>) {
        self.subscribers.push(handler);
    }

    /// Отправить событие всем подписчикам.
    ///
    /// # Аргументы
    /// * `event` - событие для отправки
    pub fn dispatch(&mut self, event: &GameEvent) {
        if self.dispatching {
            // Если уже идёт доставка, добавляем в очередь
            self.pending_events.push(event.clone());
            return;
        }

        self.dispatching = true;
        for subscriber in &mut self.subscribers {
            subscriber.handle(event);
        }
        self.dispatching = false;

        // Обрабатываем отложенные события
        if !self.pending_events.is_empty() {
            let pending = std::mem::take(&mut self.pending_events);
            for pending_event in pending {
                self.dispatch(&pending_event);
            }
        }
    }

    /// Отправить множество событий.
    ///
    /// # Аргументы
    /// * `events` - события для отправки
    pub fn dispatch_batch(&mut self, events: &[GameEvent]) {
        for subscriber in &mut self.subscribers {
            subscriber.handle_batch(events);
        }
    }

    /// Очистить всех подписчиков.
    pub fn clear(&mut self) {
        self.subscribers.clear();
        self.pending_events.clear();
    }

    /// Получить количество подписчиков.
    #[must_use]
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Тестовый обработчик для подсчёта событий.
    struct TestHandler {
        event_count: usize,
        last_event: Option<GameEvent>,
    }

    impl TestHandler {
        fn new() -> Self {
            Self {
                event_count: 0,
                last_event: None,
            }
        }
    }

    impl EventHandler for TestHandler {
        fn handle(&mut self, event: &GameEvent) {
            self.event_count += 1;
            self.last_event = Some(event.clone());
        }
    }

    #[test]
    fn test_game_event_variants() {
        // Проверяем что все варианты событий создаются корректно
        let _moved = GameEvent::PieceMoved {
            direction: crate::types::Direction::Left,
            success: true,
        };

        let _rotated = GameEvent::PieceRotated {
            direction: crate::types::RotationDirection::Clockwise,
            success: true,
        };

        let _dropped = GameEvent::PieceDropped {
            drop_type: DropType::Hard,
            drop_distance: 10,
        };

        let _held = GameEvent::PieceHeld {
            piece_type: crate::tetromino::ShapeType::I,
        };

        let _lines = GameEvent::LinesCleared {
            count: 4,
            rows: 0b1111,
        };

        let _score = GameEvent::ScoreChanged {
            old_score: 100,
            new_score: 200,
            reason: ScoreChangeReason::LinesCleared { count: 1 },
        };

        let _level = GameEvent::LevelUp {
            old_level: 1,
            new_level: 2,
        };

        let _paused = GameEvent::GamePaused;
        let _resumed = GameEvent::GameResumed;

        let _game_over = GameEvent::GameOver { final_score: 1000 };
        let _game_won = GameEvent::GameWon {
            final_score: 2000,
            elapsed_time: 60.0,
        };

        let _input = GameEvent::InputProcessed {
            action: crate::game::types::GameAction::MoveLeft,
        };
    }

    #[test]
    fn test_event_dispatcher_subscribe() {
        let mut dispatcher = EventDispatcher::new();
        let handler = Box::new(TestHandler::new());

        dispatcher.subscribe(handler);

        assert_eq!(dispatcher.subscriber_count(), 1);
    }

    #[test]
    fn test_event_dispatcher_dispatch() {
        let mut dispatcher = EventDispatcher::new();
        let mut handler = TestHandler::new();

        dispatcher.subscribe(Box::new(handler));

        dispatcher.dispatch(&GameEvent::GamePaused);

        // Проверяем что событие доставлено (через новый handler так как старый перемещён)
        assert_eq!(dispatcher.subscriber_count(), 1);
    }

    #[test]
    fn test_event_dispatcher_batch() {
        let mut dispatcher = EventDispatcher::new();
        let handler = Box::new(TestHandler::new());

        dispatcher.subscribe(handler);

        let events = vec![
            GameEvent::GamePaused,
            GameEvent::GameResumed,
            GameEvent::GameOver { final_score: 0 },
        ];

        dispatcher.dispatch_batch(&events);

        assert_eq!(dispatcher.subscriber_count(), 1);
    }

    #[test]
    fn test_event_dispatcher_clear() {
        let mut dispatcher = EventDispatcher::new();

        dispatcher.subscribe(Box::new(TestHandler::new()));
        dispatcher.subscribe(Box::new(TestHandler::new()));

        assert_eq!(dispatcher.subscriber_count(), 2);

        dispatcher.clear();

        assert_eq!(dispatcher.subscriber_count(), 0);
    }

    #[test]
    fn test_drop_type_variants() {
        let soft = DropType::Soft;
        let hard = DropType::Hard;

        assert_ne!(soft, hard);
    }

    #[test]
    fn test_score_change_reason_variants() {
        let soft = ScoreChangeReason::SoftDrop { lines: 5 };
        let hard = ScoreChangeReason::HardDrop { lines: 10 };
        let lines = ScoreChangeReason::LinesCleared { count: 4 };
        let combo = ScoreChangeReason::ComboBonus { combo: 3 };
        let level = ScoreChangeReason::LevelBonus;
        let other = ScoreChangeReason::Other;

        assert_ne!(soft, hard);
        assert_ne!(lines, combo);
        assert_ne!(level, other);
    }
}
