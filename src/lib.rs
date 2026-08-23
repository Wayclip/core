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
