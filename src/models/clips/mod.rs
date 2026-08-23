/// The model which handles game type, which extracts it from games.toml on compile time
pub mod games;
/// The model types for hosted clips, including requests and responses
pub mod hosted;
/// The model types which the local clips use
pub mod local;
/// The model types needed for tags used inside of clips
pub mod tags;
/// The module responsible for UnifiedClip and SelectedClip types
pub mod unified;
