use log::info;
use webui_rs::webui;
use crate::infrastructure::{get_system_info, get_uptime, parse_uptime_to_seconds};

pub fn setup_handlers(window: &mut webui::Window) {
    window.bind("get_system_info", |event| {
        info!("get_system_info called from frontend");

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
        info!("get_uptime called from frontend");

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

    info!("System info handlers set up successfully");
}
