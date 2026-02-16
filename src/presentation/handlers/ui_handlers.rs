use log::info;
use webui_rs::webui;

pub fn setup_ui_handlers(window: &mut webui::Window) {
    info!("Setting up UI handlers...");

    window.bind("open_folder", |_event| {
        info!("Open folder button clicked!");
    });

    window.bind("organize_images", |_event| {
        info!("Organize images button clicked!");
    });

    window.bind("ping_backend", |event| {
        let win_id = event.window;
        info!("Ping received from window {}", win_id);

        let response = serde_json::json!({
            "success": true,
            "message": "pong",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "server": "rust-backend"
        });

        let js_code = format!(
            "if(window._webui_pong) window._webui_pong({});",
            response.to_string()
        );

        let win = webui::Window::from_id(win_id as usize);
        let _ = win.run_js(&js_code);
    });

    window.bind("get_port_info", |event| {
        let win_id = event.window;
        info!("Port info request from window {}", win_id);

        let response = serde_json::json!({
            "success": true,
            "port": "dynamic",
            "protocol": "webui"
        });

        let js_code = format!(
            "if(window._webui_port_info) window._webui_port_info({});",
            response.to_string()
        );

        let win = webui::Window::from_id(win_id as usize);
        let _ = win.run_js(&js_code);
    });

    info!("UI handlers set up successfully");
}
