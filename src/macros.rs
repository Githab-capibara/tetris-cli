//! Макросы логирования и утилит.
//!
//! Используется `eprintln!` так как это CLI-приложение без logger framework.

/// Макрос для логирования ошибок с префиксом `"[ERROR]"`.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] $($arg)*");
    }};
}

/// Макрос для логирования предупреждений с префиксом `"[WARN]"`.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        eprintln!("[WARN] $($arg)*");
    }};
}

/// Макрос для логирования информации с префиксом `"[INFO]"`.
#[macro_export]
#[allow(clippy::print_stderr)]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        eprintln!("[INFO] $($arg)*");
    }};
}
