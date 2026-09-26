/// `clips` module is responsible for containing all methods and operations related to managing
/// local and hosted clips. This includes uploading, trimming, deleting and more
pub mod clips;
/// `os_keyring` module is primarily used internally to allow for safekeeping of the user's
/// `jwt_token` and `refresh_token` once logged into their Wayclip account
pub mod os_keyring;
/// `users` module is used for handling user-related actions. These actions are quite limited, but
/// they usually directly interact with the UsersHttpClient
pub mod users;

/// the `cache` module provides methods and data structures to efficiently store and cache items on
/// your local disk. It uses a HashMap along with different keys to access any one piece of data.
pub mod cache;
