/// `actions` module handles different actions the user can take on a clip, depending on its type.
pub mod actions;
/// `ffmpeg` module contains methods assosciated with Ffmpeg and clip handling (not I/O directly but
/// close)
pub mod ffmpeg;
/// `hosted` module acts as a wrapper to make uploading, removing, patching clips easier
pub mod hosted;
/// `io` module is responsible for I/O related operations, such as writting metadata, deleting,
/// renaming
pub mod io;
/// `local` module contains all the methods that are used to control local clips. This module then
/// dispatches needed calls to `io`, `ffmpeg` and others
pub mod local;
/// `query` module serves as a way to find, query and provide clips based on `clip_identifier`,
/// `regex` and `clip_type`. This module is quite broad and covers lots of different aspects, from
/// local clips, to hosted clips and even mixed.
pub mod query;
/// `unified` module handles the 'Unified' and 'Selected' clip system, which allows us to query and
/// display clips that are split across 2 different categories (local & hosted)
pub mod unified;
