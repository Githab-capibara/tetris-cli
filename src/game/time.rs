//! Модуль абстракции времени.
//!
//! Предоставляет типобезопасную обёртку для работы с временем в игре.
//! Решает проблему Primitive Obsession для временных значений.
//!
//! ## Структуры
//! - [`Time`] - типобезопасная обёртка для длительности

use std::cmp::Ordering;
use std::time::Duration;

/// Тип для представления времени в игре.
///
/// Обеспечивает типобезопасность и предотвращает путаницу с другими числовыми типами.
/// Предоставляет удобный API для работы с временем.
///
/// # Точность
/// ## Внутреннее представление
/// - Используется `std::time::Duration` для хранения времени
/// - Базовая единица: миллисекунды (u64)
/// - Точность: до 1 миллисекунды (0.001 секунды)
///
/// ## Конвертации
/// - `from_secs(f64)`: Конвертирует секунды с плавающей точкой в миллисекунды (округление)
/// - `as_secs_f64()`: Возвращает время в секундах с точностью до миллисекунд
/// - `as_millis()`: Возвращает время в миллисекундах (u64)
///
/// ## Ограничения
/// - Не поддерживает микросекунды или наносекунды (избыточно для игры)
/// - Максимальное время: `u64::MAX` миллисекунд (~584 миллиона лет)
/// - Погрешность округления: ±0.5 мс при конвертации из f64
///
/// # Примеры
/// ```
/// use tetris_cli::game::time::Time;
///
/// let time = Time::from_secs(1.5);
/// assert_eq!(time.as_millis(), 1500);
/// assert_eq!(time.as_secs_f64(), 1.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    /// Внутреннее представление длительности.
    inner: Duration,
}

impl Time {
    /// Создать новое время из секунд.
    ///
    /// # Аргументы
    /// * `secs` - время в секундах (f64)
    ///
    /// # Возвращает
    /// Новый экземпляр `Time`
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.5);
    /// assert_eq!(time.as_secs_f64(), 1.5);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn from_secs(secs: f64) -> Self {
        // Конвертируем секунды в миллисекунды для большей точности
        // cast: f64 -> u64, потеря знака допустима: время неотрицательное, округление до 1 мс достаточно для игры
        let millis = (secs * 1000.0).round() as u64;
        Self {
            inner: Duration::from_millis(millis),
        }
    }

    /// Создать новое время из миллисекунд.
    ///
    /// # Аргументы
    /// * `millis` - время в миллисекундах
    ///
    /// # Возвращает
    /// Новый экземпляр `Time`
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_millis(1500);
    /// assert_eq!(time.as_secs_f64(), 1.5);
    /// ```
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            inner: Duration::from_millis(millis),
        }
    }

    /// Получить время в миллисекундах.
    ///
    /// # Возвращает
    /// Время в миллисекундах (u64)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.5);
    /// assert_eq!(time.as_millis(), 1500);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_millis(&self) -> u64 {
        // Потеря точности допустима: Duration хранит время точно в мс
        self.inner.as_millis() as u64
    }

    /// Получить время в секундах.
    ///
    /// # Возвращает
    /// Время в секундах (f64)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.5);
    /// assert_eq!(time.as_secs_f64(), 1.5);
    /// ```
    #[must_use]
    pub const fn as_secs_f64(&self) -> f64 {
        self.inner.as_secs_f64()
    }

    /// Получить время в секундах (целое число).
    ///
    /// # Возвращает
    /// Время в секундах (u64)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.5);
    /// assert_eq!(time.as_secs(), 1);
    /// ```
    #[must_use]
    pub const fn as_secs(&self) -> u64 {
        self.inner.as_secs()
    }

    /// Получить оставшиеся наносекунды.
    ///
    /// # Возвращает
    /// Наносекунды (0-999_999_999)
    #[must_use]
    pub const fn subsec_nanos(&self) -> u32 {
        self.inner.subsec_nanos()
    }

    /// Проверить, равно ли время нулю.
    ///
    /// # Возвращает
    /// `true` если время равно нулю
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    /// Сложить два времени.
    ///
    /// # Аргументы
    /// * `other` - время для добавления
    ///
    /// # Возвращает
    /// Сумму времён
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let t1 = Time::from_secs(1.0);
    /// let t2 = Time::from_secs(0.5);
    /// let sum = t1.add(t2);
    /// assert_eq!(sum.as_secs_f64(), 1.5);
    /// ```
    #[must_use]
    pub const fn add(&self, other: Self) -> Self {
        Self {
            inner: self.inner.saturating_add(other.inner),
        }
    }

    /// Вычесть время из другого времени.
    ///
    /// # Аргументы
    /// * `other` - время для вычитания
    ///
    /// # Возвращает
    /// Разность времён (неотрицательная)
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let t1 = Time::from_secs(2.0);
    /// let t2 = Time::from_secs(0.5);
    /// let diff = t1.sub(t2);
    /// assert_eq!(diff.as_secs_f64(), 1.5);
    /// ```
    #[must_use]
    pub const fn sub(&self, other: Self) -> Self {
        Self {
            inner: self.inner.saturating_sub(other.inner),
        }
    }

    /// Умножить время на скаляр.
    ///
    /// # Аргументы
    /// * `factor` - множитель
    ///
    /// # Возвращает
    /// Произведение времени на скаляр
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.0);
    /// let doubled = time.mul(2);
    /// assert_eq!(doubled.as_secs_f64(), 2.0);
    /// ```
    #[must_use]
    pub const fn mul(&self, factor: u32) -> Self {
        Self {
            inner: self.inner.saturating_mul(factor),
        }
    }

    // Методы gt/lt удалены — используются операторы > и < через PartialOrd/Ord
}

// Реализация трейтов сравнения для Time (исправляет clippy::should_implement_trait)
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl Default for Time {
    /// Создать время по умолчанию (0 секунд).
    fn default() -> Self {
        Self {
            inner: Duration::ZERO,
        }
    }
}

impl From<Duration> for Time {
    /// Создать Time из Duration.
    fn from(duration: Duration) -> Self {
        Self { inner: duration }
    }
}

impl From<Time> for Duration {
    /// Конвертировать Time в Duration.
    fn from(time: Time) -> Self {
        time.inner
    }
}

impl std::fmt::Display for Time {
    /// Форматировать время для отображения.
    ///
    /// # Пример
    /// ```
    /// use tetris_cli::game::time::Time;
    /// let time = Time::from_secs(1.5);
    /// assert_eq!(format!("{time}"), "1.500s");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}s", self.as_secs_f64())
    }
}

// ============================================================================
// ТЕСТЫ
// ============================================================================

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_from_secs() {
        let time = Time::from_secs(1.5);
        assert_eq!(time.as_secs_f64(), 1.5);
        assert_eq!(time.as_millis(), 1500);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_from_millis() {
        let time = Time::from_millis(1500);
        assert_eq!(time.as_secs_f64(), 1.5);
        assert_eq!(time.as_millis(), 1500);
    }

    #[test]
    fn test_time_as_secs() {
        let time = Time::from_secs(2.7);
        assert_eq!(time.as_secs(), 2);
    }

    #[test]
    fn test_time_is_zero() {
        assert!(Time::default().is_zero());
        assert!(!Time::from_secs(0.1).is_zero());
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_add() {
        let t1 = Time::from_secs(1.0);
        let t2 = Time::from_secs(0.5);
        let sum = t1.add(t2);
        assert_eq!(sum.as_secs_f64(), 1.5);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_sub() {
        let t1 = Time::from_secs(2.0);
        let t2 = Time::from_secs(0.5);
        let diff = t1.sub(t2);
        assert_eq!(diff.as_secs_f64(), 1.5);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_sub_saturating() {
        let t1 = Time::from_secs(1.0);
        let t2 = Time::from_secs(2.0);
        let diff = t1.sub(t2);
        assert_eq!(diff.as_secs_f64(), 0.0); // Saturating
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_mul() {
        let time = Time::from_secs(1.5);
        let doubled = time.mul(2);
        assert_eq!(doubled.as_secs_f64(), 3.0);
    }

    #[test]
    fn test_time_cmp() {
        let t1 = Time::from_secs(1.0);
        let t2 = Time::from_secs(2.0);
        let t3 = Time::from_secs(1.0);

        assert_eq!(t1.cmp(&t2), std::cmp::Ordering::Less);
        assert_eq!(t2.cmp(&t1), std::cmp::Ordering::Greater);
        assert_eq!(t1.cmp(&t3), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_time_gt_lt() {
        let t1 = Time::from_secs(1.0);
        let t2 = Time::from_secs(2.0);

        assert!((t1 <= t2));
        assert!(t2 > t1);
        assert!(t1 < t2);
        assert!((t2 >= t1));
    }

    #[test]
    fn test_time_display() {
        let time = Time::from_secs(1.5);
        assert_eq!(format!("{time}"), "1.500s");
    }

    #[test]
    fn test_time_from_duration() {
        let duration = Duration::from_millis(500);
        let time = Time::from(duration);
        assert_eq!(time.as_millis(), 500);
    }

    #[test]
    fn test_time_into_duration() {
        let time = Time::from_millis(500);
        let duration: Duration = time.into();
        assert_eq!(duration.as_millis(), 500);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Допустимо для тестов с константными значениями
    fn test_time_default() {
        let time = Time::default();
        assert_eq!(time.as_secs_f64(), 0.0);
        assert!(time.is_zero());
    }

    #[test]
    fn test_time_copy_clone() {
        let time = Time::from_secs(1.0);
        let time_copy = time; // Copy
        let time_clone = time; // Clone

        assert_eq!(time, time_copy);
        assert_eq!(time, time_clone);
    }
}
