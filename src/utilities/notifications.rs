use std::time::Duration;

#[derive(Debug, Clone)]
pub struct NotificationOptions {
    pub title: String,
    pub message: String,
    pub duration: Duration,
    pub icon_type: IconType,
}

impl Default for NotificationOptions {
    fn default() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
            duration: Duration::from_secs(5),
            icon_type: IconType::Info,
        }
    }
}

impl NotificationOptions {
    pub fn new<S: Into<String>>(title: S, message: S) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            duration: Duration::from_secs(5),
            icon_type: IconType::Info,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_icon(mut self, icon: IconType) -> Self {
        self.icon_type = icon;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IconType {
    Info,
    Warning,
    Error,
    Success,
}

pub struct Notifications;

impl Notifications {
    pub fn show(options: NotificationOptions) -> bool {
        Self::native_show(&options)
    }

    pub fn show_simple(title: &str, message: &str) -> bool {
        let options = NotificationOptions::new(title, message);
        Self::show(options)
    }

    pub fn show_info(title: &str, message: &str) -> bool {
        let options = NotificationOptions::new(title, message).with_icon(IconType::Info);
        Self::show(options)
    }

    pub fn show_warning(title: &str, message: &str) -> bool {
        let options = NotificationOptions::new(title, message).with_icon(IconType::Warning);
        Self::show(options)
    }

    pub fn show_error(title: &str, message: &str) -> bool {
        let options = NotificationOptions::new(title, message).with_icon(IconType::Error);
        Self::show(options)
    }

    pub fn show_success(title: &str, message: &str) -> bool {
        let options = NotificationOptions::new(title, message).with_icon(IconType::Success);
        Self::show(options)
    }

    fn native_show(_options: &NotificationOptions) -> bool {
        false
    }
}
