use crate::core::Plugin;
use lazy_static::lazy_static;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use webui_rs::webui;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStateChangePayload {
    pub window_id: i64,
    pub window_title: String,
    pub component: String,
    pub previous_state: Option<String>,
    pub new_state: String,
    pub timestamp: String,
}

lazy_static! {
    static ref WINDOW_STATES: Mutex<Vec<WindowStateChangePayload>> = Mutex::new(Vec::new());
}

pub struct WindowTrackingPlugin;

impl WindowTrackingPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowTrackingPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for WindowTrackingPlugin {
    fn name(&self) -> &str {
        "window_tracking"
    }

    fn register(&self, window: &mut webui::Window) {
        info!("Registering WindowTrackingPlugin handlers...");

        window.bind("window_state_changed", |event| {
            let win_id = event.window;
            debug!("Window state change event received on window {}", win_id);

            let data_str = unsafe {
                std::ffi::CStr::from_ptr(event.element)
                    .to_string_lossy()
                    .into_owned()
            };

            debug!("Raw window state data: {}", data_str);

            match serde_json::from_str::<WindowStateChangePayload>(&data_str) {
                Ok(payload) => {
                    info!("Window state change - ID: {}, Title: '{}', State: {} -> {}", 
                          payload.window_id, 
                          payload.window_title, 
                          payload.previous_state.as_deref().unwrap_or("unknown"), 
                          payload.new_state);

                    if let Ok(mut states) = WINDOW_STATES.lock() {
                        states.push(payload.clone());
                        if states.len() > 100 {
                            let excess = states.len() - 100;
                            states.drain(0..excess);
                        }
                    }

                    log_window_state_change(&payload);

                    let response = serde_json::json!({
                        "success": true,
                        "message": format!("Window {} state updated to {}", payload.window_id, payload.new_state),
                        "window_id": payload.window_id,
                        "new_state": payload.new_state
                    });

                    let js_code = format!(
                        "if(window._webui_window_state_callback) window._webui_window_state_callback({});",
                        response.to_string()
                    );
                    
                    webui::Window::from_id(win_id as usize).run_js(&js_code);
                }
                Err(e) => {
                    error!("Failed to parse window state change payload: {}", e);
                    error!("Problematic data: {}", data_str);
                    
                    let error_response = serde_json::json!({
                        "success": false,
                        "error": format!("Invalid payload: {}", e)
                    });
                    
                    let js_code = format!(
                        "if(window._webui_window_state_error) window._webui_window_state_error({});",
                        error_response.to_string()
                    );
                    
                    webui::Window::from_id(win_id as usize).run_js(&js_code);
                }
            }
        });

        window.bind("get_window_states", |event| {
            let win_id = event.window;
            debug!("Get window states request received on window {}", win_id);

            let states = WINDOW_STATES.lock().unwrap().clone();
            
            let response = serde_json::json!({
                "success": true,
                "data": states,
                "count": states.len()
            });

            let js_code = format!(
                "if(window._webui_get_window_states_callback) window._webui_get_window_states_callback({});",
                response.to_string()
            );
            
            webui::Window::from_id(win_id as usize).run_js(&js_code);
        });

        info!("WindowTrackingPlugin handlers registered");
    }
}

fn log_window_state_change(payload: &WindowStateChangePayload) {
    let state_icon = match payload.new_state.as_str() {
        "opened" => "[Opened]",
        "focused" => "[Focused]",
        "minimized" => "[Minimized]",
        "restored" => "[Restored]",
        "maximized" => "[Maximized]",
        "closed" => "[Closed]",
        "hidden" => "🙈",
        "shown" => "🐵",
        _ => "🪟",
    };

    info!("[WINDOW TRACKING] {} Window State Change", state_icon);
    info!(
        "  ID: {} | Title: '{}' | Component: '{}'",
        payload.window_id, payload.window_title, payload.component
    );
    info!(
        "  Previous: {} -> New: {}",
        payload.previous_state.as_deref().unwrap_or("none"),
        payload.new_state
    );
    info!("  Timestamp: {}", payload.timestamp);
    info!("  --------------------------------------------------");
}
