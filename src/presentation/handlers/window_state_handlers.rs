use log::{debug, info, error};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;
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

// Thread-safe storage for window states
lazy_static! {
    static ref WINDOW_STATES: Mutex<Vec<WindowStateChangePayload>> = Mutex::new(Vec::new());
}

pub fn setup_window_state_handlers(window: &mut webui::Window) {
    info!("Setting up window state change handlers...");

    // Handler for window state changes from frontend
    window.bind("window_state_changed", |event| {
        let win_id = event.window;
        debug!("Window state change event received on window {}", win_id);

        // Get the event data
        let data_str = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };

        debug!("Raw window state data: {}", data_str);

        // Parse the window state change payload
        match serde_json::from_str::<WindowStateChangePayload>(&data_str) {
            Ok(payload) => {
                info!("Window state change - ID: {}, Title: '{}', State: {} -> {}", 
                      payload.window_id, 
                      payload.window_title, 
                      payload.previous_state.as_deref().unwrap_or("unknown"), 
                      payload.new_state);

                // Store the state change in our history
                if let Ok(mut states) = WINDOW_STATES.lock() {
                    states.push(payload.clone());
                    
                    // Keep only the most recent 100 entries
                    if states.len() > 100 {
                        let excess = states.len() - 100;
                        states.drain(0..excess);
                    }
                }

                // Log the state change
                log_window_state_change(&payload);

                // Send success response back to frontend
                let response = serde_json::json!({
                    "success": true,
                    "message": format!("Window {} state updated to {}", payload.window_id, payload.new_state),
                    "window_id": payload.window_id,
                    "new_state": payload.new_state
                });

                // Send response back to frontend
                let js_code = format!(
                    "if(window._webui_window_state_callback) window._webui_window_state_callback({});",
                    response.to_string()
                );
                
                webui::Window::from_id(win_id as usize).run_js(&js_code);
            }
            Err(e) => {
                error!("Failed to parse window state change payload: {}", e);
                error!("Problematic data: {}", data_str);
                
                // Send error response back to frontend
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

    // Handler to get window state history
    window.bind("get_window_states", |event| {
        let win_id = event.window;
        debug!("Get window states request received on window {}", win_id);

        // Get the current window states
        let states = WINDOW_STATES.lock().unwrap().clone();
        
        let response = serde_json::json!({
            "success": true,
            "data": states,
            "count": states.len()
        });

        // Send response back to frontend
        let js_code = format!(
            "if(window._webui_get_window_states_callback) window._webui_get_window_states_callback({});",
            response.to_string()
        );
        
        webui::Window::from_id(win_id as usize).run_js(&js_code);
    });

    info!("Window state change handlers set up successfully");
}

fn log_window_state_change(payload: &WindowStateChangePayload) {
    // Determine the appropriate emoji based on the new state
    let state_icon = match payload.new_state.as_str() {
        "opened" => "📂",
        "focused" => "👁️",
        "minimized" => "🗕️",
        "restored" => "🗖️", 
        "maximized" => "🗙️",
        "closed" => "❌",
        "hidden" => "🙈",
        "shown" => "🐵",
        _ => "🪟", // Default window icon
    };

    info!("[WINDOW TRACKING] {} Window State Change", state_icon);
    info!("  ID: {} | Title: '{}' | Component: '{}'", payload.window_id, payload.window_title, payload.component);
    info!("  Previous: {} -> New: {}", 
          payload.previous_state.as_deref().unwrap_or("none"), 
          payload.new_state);
    info!("  Timestamp: {}", payload.timestamp);
    info!("  --------------------------------------------------");
}

// Helper function to get current window states (for use by other parts of the application)
pub fn get_current_window_states() -> Vec<WindowStateChangePayload> {
    WINDOW_STATES.lock().unwrap().clone()
}

// Helper function to get the state of a specific window
pub fn get_window_state(window_id: i64) -> Option<WindowStateChangePayload> {
    let states = WINDOW_STATES.lock().unwrap();
    states.iter()
        .find(|state| state.window_id == window_id)
        .cloned()
}