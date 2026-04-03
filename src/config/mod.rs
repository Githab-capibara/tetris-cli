//! Модуль конфигурации.
//!
//! # Ответственность
//! - Централизованное управление конфигурацией приложения
//! - Определение ключей и настроек в одном месте

pub mod keys;

// Ре-экспорт ключей для удобства (публичный API)
#[allow(unused_imports)]
pub use keys::{
    get_controls_hmac_key, get_leaderboard_hmac_key, get_save_data_hmac_key, CONTROLS_HMAC_KEY,
    LEADERBOARD_HMAC_KEY, SAVE_DATA_HMAC_KEY,
};
