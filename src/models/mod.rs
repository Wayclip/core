/// `auth` module responsible for models relating to authentication, primarily via device and user
/// code
pub mod auth;
/// `clips` module contains all the models that are assosciated with clips, such as hosted, local,
/// unified, tags
pub mod clips;
/// `error` module is responsible for providing WayclipError enum, handling internal and external
/// third-party errors
pub mod error;
/// `input` module contains models for keyboard & controller input
pub mod input;
/// `nutype` module creates structs and `utoipa` traits to make sure all the inputs to the API is
/// validated and sanitised. Most of the types defined in this module, are used inside the API
pub mod nutype;
/// `query` module is responsibel for housing the flexible types required for quering data from the API
pub mod query;
/// `users` module contains models used by API which contains the user & storage limit
pub mod users;
