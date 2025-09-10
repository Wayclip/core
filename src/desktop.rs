use crate::{log_to, Logger};
use std::env;
use std::process::{Command, Stdio};
use which::which;

#[derive(Debug)]
pub enum DesktopEnv {
    Hyprland,
    Sway,
    Gnome,
    Plasma,
    Xfce,
    Unknown,
}

pub fn detect_desktop() -> DesktopEnv {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
        || env::var("XDG_CURRENT_DESKTOP")
            .map(|d| d.contains("Hyprland"))
            .unwrap_or(false)
        || which("hyprctl").is_ok()
    {
        return DesktopEnv::Hyprland;
    }

    if env::var("DEBUGSOCK").is_ok() || which("swaymsg").is_ok() {
        return DesktopEnv::Sway;
    }

    if env::var("XDG_CURRENT_DESKTOP")
        .map(|d| d.to_lowercase().contains("gnome"))
        .unwrap_or(false)
    {
        return DesktopEnv::Gnome;
    }

    if env::var("XDG_CURRENT_DESKTOP")
        .map(|d| d.to_lowercase().contains("plasma"))
        .unwrap_or(false)
    {
        return DesktopEnv::Plasma;
    }

    if env::var("XDG_CURRENT_DESKTOP")
        .map(|d| d.to_lowercase().contains("xfce"))
        .unwrap_or(false)
    {
        return DesktopEnv::Xfce;
    }

    DesktopEnv::Unknown
}

pub async fn setup_trigger(trigger_path: &str, logger: &Logger) {
    match detect_desktop() {
        DesktopEnv::Hyprland => setup_hyprland(trigger_path, logger).await,
        DesktopEnv::Sway => setup_sway(trigger_path, logger).await,
        DesktopEnv::Gnome | DesktopEnv::Plasma | DesktopEnv::Xfce => {
            log_to!(*logger, Info, [DEBUG] =>
                "This desktop environment does not allow runtime keybind injection. \
                 Please bind Alt+C manually in system settings → keyboard shortcuts → to the path {trigger_path}"
            );
        }
        DesktopEnv::Unknown => {
            log_to!(*logger, Warn, [DEBUG] =>
                "Unknown compositor/desktop. Please bind Alt+C manually."
            );
        }
    }
}

async fn setup_hyprland(trigger_path: &str, logger: &Logger) {
    let output = Command::new("hyprctl")
        .args(["keyword", "bind", &format!("Alt_L,C,exec,{trigger_path}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match output {
        Ok(mut child) => {
            if let Ok(status) = child.wait() {
                if status.success() {
                    log_to!(*logger, Info, [HYPR] => "Hyprland bind added successfully");
                } else {
                    log_to!(*logger, Error, [HYPR] => "Hyprland bind failed with nonzero exit");
                }
            }
        }
        Err(e) => log_to!(*logger, Error, [HYPR] => "Failed to spawn hyprctl: {e}"),
    }
}

async fn setup_sway(trigger_path: &str, logger: &Logger) {
    let output = Command::new("swaymsg")
        .args(["bindsym", "Alt+c", &format!("exec {trigger_path}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match output {
        Ok(mut child) => {
            if let Ok(status) = child.wait() {
                if status.success() {
                    log_to!(*logger, Info, [DEBUG] => "Sway bind added successfully");
                } else {
                    log_to!(*logger, Error, [DEBUG] => "Sway bind failed with nonzero exit");
                }
            }
        }
        Err(e) => log_to!(*logger, Error, [DEBUG] => "Failed to spawn swaymsg: {e}"),
    }
}
