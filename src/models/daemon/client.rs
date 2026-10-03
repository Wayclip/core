use zbus::{Connection, proxy};

use crate::models::{
    daemon::{DEFAULT_MODE, DEFAULT_SYSTEMD_SERVICE, DaemonStatus},
    error::WayclipError,
};

/// Client used to manage the Wayclip daemon
///
/// Uses D-Bus and systemd to control the daemon
pub struct DaemonClient {
    connection: zbus::Connection,
}

impl DaemonClient {
    /// Creates a new daemon client
    pub async fn new() -> Result<Self, WayclipError> {
        let connection = Connection::session().await?;
        Ok(Self { connection })
    }

    /// Starts the Wayclip daemon
    pub async fn start_daemon(&self) -> Result<(), WayclipError> {
        let systemd = SystemdManagerProxy::new(&self.connection).await?;
        systemd
            .start_unit(DEFAULT_SYSTEMD_SERVICE, DEFAULT_MODE)
            .await?;
        Ok(())
    }

    /// Stops the Wayclip daemon
    pub async fn stop_daemon(&self) -> Result<(), WayclipError> {
        let systemd = SystemdManagerProxy::new(&self.connection).await?;
        systemd
            .stop_unit(DEFAULT_SYSTEMD_SERVICE, DEFAULT_MODE)
            .await?;
        Ok(())
    }

    /// Restarts the Wayclip daemon
    pub async fn restart_daemon(&self) -> Result<(), WayclipError> {
        let systemd = SystemdManagerProxy::new(&self.connection).await?;
        systemd
            .restart_unit(DEFAULT_SYSTEMD_SERVICE, DEFAULT_MODE)
            .await?;
        Ok(())
    }

    /// Gets a proxy for talking to the Wayclip daemon
    pub async fn get_proxy(&self) -> Result<DaemonProxy<'_>, WayclipError> {
        let proxy = DaemonProxy::new(&self.connection).await?;
        Ok(proxy)
    }

    /// Enables the daemon to start automatically
    pub async fn enable_autostart(&self) -> Result<(), WayclipError> {
        let systemd = SystemdManagerProxy::new(&self.connection).await?;
        systemd
            .enable_unit_files(vec![DEFAULT_SYSTEMD_SERVICE], false, true)
            .await?;
        Ok(())
    }

    /// Disables automatic startup for the daemon
    pub async fn disable_autostart(&self) -> Result<(), WayclipError> {
        let systemd = SystemdManagerProxy::new(&self.connection).await?;
        systemd
            .disable_unit_files(vec![DEFAULT_SYSTEMD_SERVICE], false)
            .await?;
        Ok(())
    }
}

/// Proxy used to talk to the Wayclip daemon
#[proxy(
    interface = "org.wayclip.Daemon1",
    default_service = "org.wayclip.Daemon",
    default_path = "/org/wayclip/Daemon"
)]
pub trait Daemon {
    /// Gets the current daemon status
    async fn get_status(&self) -> zbus::fdo::Result<DaemonStatus>;

    /// Saves the current clipboard
    async fn save_clip(&self) -> zbus::fdo::Result<String>;

    /// Saves the current clipboard with a custom name
    async fn save_clip_with_custom_name(&self, forced_name: String) -> zbus::fdo::Result<String>;

    /// Shuts down the Wayclip daemon
    async fn shutdown(&self) -> zbus::fdo::Result<()>;
}

/// Proxy used to talk to systemd
#[proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
pub trait SystemdManager {
    /// Starts a systemd service
    async fn start_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Stops a systemd service
    async fn stop_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Restarts a systemd service
    async fn restart_unit(
        &self,
        name: &str,
        mode: &str,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Enables one or more systemd services
    async fn enable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;

    /// Disables one or more systemd services
    async fn disable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;
}
