//! Макросы логирования и утилит.
//!
//! Предоставляют единый интерфейс для логирования ошибок, предупреждений
//! и информационной отладочной информации.
//!
//! Используется `eprintln!` так как это CLI-приложение без logger framework.
//! Все макros выводят сообщения в stderr с соответствующим префиксом.

/// Макрос для логирования ошибок с префиксом `"[ERROR]"`.
///
/// # Пример
/// ```ignore
/// use tetris_cli::log_error;
/// log_error!("Произошла ошибка: {}", err);
/// ```
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] $($arg)*");
    }};
}

/// Макрос для логирования предупреждений с префиксом `"[WARN]"`.
///
/// # Пример
/// ```ignore
/// use tetris_cli::log_warn;
/// log_warn!("Предупреждение: данные устарели");
/// ```
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        eprintln!("[WARN] $($arg)*");
    }};
}

/// Макрос для логирования информации с префиксом `"[INFO]"`.
///
/// # Пример
/// ```ignore
/// use tetris_cli::log_info;
/// log_info!("Игра запущена в режиме Classic");
/// ```
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        eprintln!("[INFO] $($arg)*");
    }};
}
