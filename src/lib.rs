#![deny(missing_docs)]

//! # wayclip-core
//! This crate provides methods, models and operations needed for the Wayclip ecosystem to work

/// `app` module provides most of the methods which range from managing clips using FFmpeg and I/O calls,
/// to keyring management
/// This module is primarily used inside the App ecosystem (`wayclip-cli` & `wayclip-gui`)
pub mod app;
/// `client` module provides standardised HTTP calling methods, allowing to interact with the API
/// sepcified under `api.url` in settings.
/// This module is more usually used internally, however `wayclip-cli` or other entities are allowed
/// to call the HTTP clients directly
pub mod client;
/// `models` module is simply responsible for providing structs and enums to be used, as well as the
/// implementations for `From<>` and other
/// This module is globally used in `wayclip-daemon`, `wayclip-cli` & `wayclip-api`
pub mod models;
/// `settings` module is responsible for managing the global Wayclip settings stored on your system.
/// This module is able to load, set, get and migrate between versions.
/// This module is globally used in `wayclip-daemon` and `wayclip-cli`
pub mod settings;

// i18n configuration
// (path relative to Cargo.toml)
rust_i18n::i18n!("assets/locales", fallback = "en-US");
pub use rust_i18n::set_locale;

/// used inside macro
pub fn translate(key: &'static str) -> std::borrow::Cow<'static, str> {
    rust_i18n::t!(key)
}

/// Custom translation macro that automatically detects named parameters
/// and replaces `%{var}` in your locale files.
#[macro_export]
macro_rules! t {
    ($key:expr $(, $var:ident = $val:expr)* $(,)?) => {{
        #[allow(unused_mut)]
        let mut msg = $crate::translate($key).into_owned();
        $(
            msg = msg.replace(concat!("%{", stringify!($var), "}"), &format!("{}", $val));
        )*
        msg
    }};
}
