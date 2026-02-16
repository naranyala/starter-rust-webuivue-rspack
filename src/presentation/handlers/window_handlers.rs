#![allow(dead_code)]

use crate::infrastructure::event_bus::{AppEvent, emit};
use crate::infrastructure::logging;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;
use webui_rs::webui;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowStatePayload {
    pub window_id: i64,
    pub window_title: String,
    pub component: String,
    #[serde(rename = "previous_state")]
    pub previous_state: Option<String>,
    #[serde(rename = "new_state")]
    pub new_state: String,
    pub timestamp: String,
}

fn send_response(window_id: usize, event_name: &str, response: &serde_json::Value) {
    let js = format!(
        "window.dispatchEvent(new CustomEvent('{}', {{ detail: {} }}))",
        event_name,
        response.to_string()
    );
    webui::Window::from_id(window_id).run_js(&js);
}

lazy_static! {
    static ref WINDOW_HISTORY: Mutex<Vec<WindowStatePayload>> = Mutex::new(Vec::new());
}

pub fn setup_window_handlers(window: &mut webui::Window) {
    info!("[WindowHandler] Setting up handlers...");
    
    // Window state change handler
    window.bind("window_state_changed", |event| {
        let window_id = event.get_window().id;

        let data_str = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };
        
        info!("[WindowHandler] Received: {}", data_str);
        
        // Parse payload
        match serde_json::from_str::<WindowStatePayload>(&data_str) {
            Ok(payload) => {
                info!("[WindowHandler] Parsed: id={}, state={}", payload.window_id, payload.new_state);
                
                // Store in history
                if let Ok(mut history) = WINDOW_HISTORY.lock() {
                    history.push(payload.clone());
                    if history.len() > 100 {
                        history.remove(0);
                    }
                }

                // Log
                log_window_state_change(&payload);

                // Emit to event bus
                let payload_clone = payload.clone();
                tokio::spawn(async move {
                    let evt = AppEvent::WindowStateChanged {
                        window_id: payload_clone.window_id,
                        window_title: payload_clone.window_title.clone(),
                        component: payload_clone.component.clone(),
                        previous_state: payload_clone.previous_state.clone(),
                        new_state: payload_clone.new_state.clone(),
                    };
                    emit(evt).await;
                });
                
                // Send success response to frontend via CustomEvent
                let response = serde_json::json!({
                    "success": true,
                    "message": format!("Window {} {}", payload.window_id, payload.new_state),
                    "data": payload
                });
                send_response(window_id as usize, "window_state_response", &response);
            }
            Err(e) => {
                info!("[WindowHandler] Parse error: {}", e);
                let response = serde_json::json!({
                    "success": false,
                    "error": format!("Parse error: {}", e)
                });
                send_response(window_id as usize, "window_state_response", &response);
            }
        }
    });

    // Ping handler - sends pong event back
    window.bind("ping_backend", |event| {
        let window_id = event.get_window().id;
        info!("[WindowHandler] Ping received");
        
        // Send pong event
        let js = r#"
            window.dispatchEvent(new CustomEvent('backend_pong', { 
                detail: { timestamp: Date.now() } 
            }));
        "#;
        webui::Window::from_id(window_id as usize).run_js(js);
    });

    // Get history
    window.bind("get_window_state_history", |event| {
        let window_id = event.get_window().id;

        let history = WINDOW_HISTORY.lock().unwrap();
        let response = serde_json::json!({
            "success": true,
            "data": *history,
            "count": history.len()
        });

        send_response(window_id as usize, "window_history_response", &response);
    });

    // Get port information
    window.bind("get_port_info", |event| {
        let window_id = event.get_window().id;
        info!("[WindowHandler] Get port info requested");

        // Note: WebUI doesn't expose the actual port directly in the Rust bindings
        // This is a limitation of the current WebUI Rust bindings
        // We'll return a placeholder that indicates the port is dynamically assigned
        let response = serde_json::json!({
            "success": true,
            "port": "dynamic",  // WebUI handles port assignment internally
            "message": "Port is dynamically assigned by WebUI"
        });

        send_response(window_id as usize, "port_info_response", &response);
    });

    info!("[WindowHandler] Handlers ready");
    logging::log_section("🖥️ Window Handlers Initialized");
}

fn log_window_state_change(payload: &WindowStatePayload) {
    let icon = match payload.new_state.as_str() {
        "opened" => "📂",
        "focused" => "👁️",
        "minimized" => "🗕️",
        "restored" => "🗖️",
        "maximized" => "🗙️",
        "closed" => "❌",
        _ => "🪟",
    };

    logging::log_section(&format!("{} Window State", icon));
    logging::log_key_value("ID", &payload.window_id.to_string());
    logging::log_key_value("Title", &payload.window_title);
    logging::log_key_value("State", &payload.new_state);
    
    info!("[Window] {} - '{}' -> {}", payload.window_id, payload.window_title, payload.new_state);
}
