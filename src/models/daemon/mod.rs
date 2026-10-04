use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

/// Contains methods for clients to communicate with the daemon
pub mod client;
/// Contains methods related to checking the health of system and installed packages
pub mod doctor;
/// Contains methods related to sending notifications and playing sound on the system
pub mod notifications;
/// Contains pipewire related methods, locked behind a feature
pub mod pipewire;

/// Default systemd service name for the Wayclip daemon
pub const DEFAULT_SYSTEMD_SERVICE: &str = "wayclip-daemon.service";
/// Default D-Bus service name for the Wayclip daemon
pub const DEFAULT_DBUS_SERVICE: &str = "org.wayclip.Daemon";
/// Default clipboard synchronization mode
pub const DEFAULT_MODE: &str = "replace";
/// Default D-Bus object path for the Wayclip daemon
pub const DEFAULT_INTERFACE_PATH: &str = "/org/wayclip/Daemon";
/// Default D-Bus interface name for the Wayclip daemon
pub const DEFAULT_DBUS_INTERFACE: &str = "org.wayclip.Daemon1";

/// All the possible states a deamon can be in
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[allow(missing_docs)]
pub enum DaemonStatus {
    Active,
    Inactive,
    Saving,
    Activating,
    Deactivating,
    Failed,
}
