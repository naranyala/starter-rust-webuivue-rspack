use crate::core::Plugin;
use crate::infrastructure::{get_system_info, get_uptime, parse_uptime_to_seconds};
use log::info;
use webui_rs::webui;

pub struct SystemInfoPlugin;

impl SystemInfoPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemInfoPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SystemInfoPlugin {
    fn name(&self) -> &str {
        "system_info"
    }

    fn register(&self, window: &mut webui::Window) {
        info!("Registering SystemInfoPlugin handlers...");

        window.bind("get_system_info", |event| {
            let sysinfo = get_system_info();

            let response = serde_json::json!({
                "success": true,
                "data": sysinfo
            });

            let js = format!(
                "window.dispatchEvent(new CustomEvent('sysinfo_response', {{ detail: {} }}))",
                response.to_string()
            );

            webui::Window::from_id(event.window).run_js(&js);
        });

        window.bind("get_uptime", |event| {
            let uptime_str = get_uptime();
            let uptime_seconds = parse_uptime_to_seconds(&uptime_str);

            let response = serde_json::json!({
                "success": true,
                "uptime_seconds": uptime_seconds
            });

            let js = format!(
                "window.dispatchEvent(new CustomEvent('uptime_response', {{ detail: {} }}))",
                response.to_string()
            );

            webui::Window::from_id(event.window).run_js(&js);
        });

        info!("SystemInfoPlugin handlers registered");
    }
}
