use crate::{
    models::{daemon::notifications::sound::NotificationSound, error::WayclipError},
    settings::notifications::NotificationSettings,
    t,
};
use strum_macros::FromRepr;

/// The sound module, will play sounds in addition to notifications being sent
pub mod sound;

/// The manager for sending out system-wide notifications
pub struct NotificationManager;

/// A single notification event
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum NotificationEvent {
    SaveSuccess,
    SaveError,
    DaemonStart,
    DaemonStop,
    /// Used soley for doctor
    Test,
}

/// Urgency of the notification (may not work on some compositors and distros)
#[derive(Debug, Clone, Copy, FromRepr)]
#[allow(missing_docs)]
pub enum Urgency {
    Normal = 1,
    Critical = 2,
}

impl From<Urgency> for notify_rust::Urgency {
    fn from(value: Urgency) -> Self {
        match value {
            Urgency::Normal => Self::Normal,
            Urgency::Critical => Self::Critical,
        }
    }
}

impl NotificationEvent {
    /// Retrive a translated summary of the notification based on the event
    pub fn get_summary(&self) -> String {
        match self {
            Self::DaemonStop => t!("notification.daemon_stop.summary"),
            Self::DaemonStart => t!("notification.daemon_start.summary"),
            Self::SaveError => t!("notification.save_error.summary"),
            Self::SaveSuccess => t!("notification.save_success.summary"),
            Self::Test => t!("notification.test.summary"),
        }
    }

    /// Retrive the translated body of the notification based on the event
    pub fn get_body(&self, content: String) -> String {
        match self {
            Self::SaveSuccess => t!("notification.save_success.body", content = content),
            Self::SaveError => t!("notification.save_error.body", content = content),
            Self::DaemonStart => t!("notification.daemon_start.body"),
            Self::DaemonStop => t!("notification.daemon_stop.body"),
            Self::Test => t!("notification.test.body", content = content),
        }
    }

    /// Get the wayclip icon, which will only work if icon is stored in /usr/share/icons/hicolor/
    pub fn get_icon(&self) -> &'static str {
        "wayclip"
    }

    /// The timeout, in milliseconds, for which the notification will stay on screen for
    pub fn get_timeout_ms(&self) -> i32 {
        match self {
            Self::DaemonStop | Self::DaemonStart => 1500,
            Self::SaveError => 4000,
            Self::SaveSuccess => 1000,
            Self::Test => 500,
        }
    }

    /// Maps the event to urgency
    pub fn get_urgency(&self) -> Urgency {
        match self {
            Self::SaveError => Urgency::Critical,
            _ => Urgency::Normal,
        }
    }
}

impl NotificationManager {
    /// Send outa test notification
    pub async fn test_notification(
        event: NotificationEvent,
        content: String,
    ) -> Result<(), WayclipError> {
        tokio::task::spawn_blocking(move || Self::send_notification(event, content)).await??;
        Ok(())
    }

    /// Send event to be processed and sent
    pub fn send_event(
        event: NotificationEvent,
        settings: &NotificationSettings,
        content: String,
    ) -> Result<(), WayclipError> {
        log::info!("Notification Event Triggered: {:?}", event);
        NotificationSound::process_audio_event(&event, settings);
        NotificationManager::process_message_event(&event, settings, content);

        Ok(())
    }

    fn process_message_event(
        event: &NotificationEvent,
        settings: &NotificationSettings,
        content: String,
    ) {
        let send_msg = match event {
            NotificationEvent::SaveError => settings.message.on_save_error,
            NotificationEvent::SaveSuccess => settings.message.on_save_success,
            NotificationEvent::DaemonStart => settings.message.on_daemon_start,
            NotificationEvent::DaemonStop => settings.message.on_daemon_stop,
            _ => false,
        };

        if send_msg {
            let event_clone = event.clone();
            let content_clone = content.clone();
            tokio::task::spawn_blocking(move || {
                if let Err(e) = Self::send_notification(event_clone, content_clone) {
                    log::error!("Notification Error: {}", e);
                }
            });
        }
    }

    // dont directly expose as pub, has to be done as blocking...
    fn send_notification(event: NotificationEvent, content: String) -> Result<(), WayclipError> {
        let summary = event.get_summary();
        let body = event.get_body(content);
        let urgency = event.get_urgency();
        let icon = event.get_icon();
        let timeout_ms = event.get_timeout_ms();

        notify_rust::Notification::new()
            .appname("Wayclip")
            .summary(&summary)
            .urgency(urgency.into())
            .body(&body)
            .icon(icon)
            .timeout(notify_rust::Timeout::Milliseconds(timeout_ms as u32))
            .show()
            .map_err(|e| WayclipError::Validation(e.to_string().into()))?;

        Ok(())
    }
}
