//! Макросы логирования и утилит.
//!
//! Предоставляют единый интерфейс для логирования ошибок, предупреждений
//! и информационной отладочной информации.
//!
//! Используется `eprintln!` так как это CLI-приложение без logger framework.
//! Все макросы выводят сообщения в stderr с соответствующим префиксом.

/// Макрос для логирования ошибок с префиксом `"[ERROR]"`.
#[macro_export]
#[allow(
    clippy::print_stderr,
    reason = "CLI-приложение использует stderr для логирования ошибок"
)]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] {}", format!($($arg)*));
    }};
}

/// Макрос для логирования предупреждений с префиксом `"[WARN]"`.
#[macro_export]
#[allow(
    clippy::print_stderr,
    reason = "CLI-приложение использует stderr для логирования предупреждений"
)]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        eprintln!("[WARN] {}", format!($($arg)*));
    }};
}

/// Макрос для логирования информации с префиксом `"[INFO]"`.
#[macro_export]
#[allow(
    clippy::print_stderr,
    reason = "CLI-приложение использует stderr для логирования информации"
)]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        eprintln!("[INFO] {}", format!($($arg)*));
    }};
}
