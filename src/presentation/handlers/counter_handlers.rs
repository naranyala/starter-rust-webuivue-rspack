use log::{debug, info};
use webui_rs::webui;

pub fn setup_counter_handlers(window: &mut webui::Window) {
    window.bind("increment_counter", |event| {
        // Get the element name that triggered the event
        let element_name = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };

        // Log the increment operation
        info!(
            "Counter incremented in Rust backend - Element: {}",
            element_name
        );
        debug!(
            "Increment event details - element: {}, window: {}",
            element_name, event.window
        );
    });

    window.bind("reset_counter", |event| {
        // Get the element name that triggered the event
        let element_name = unsafe {
            std::ffi::CStr::from_ptr(event.element)
                .to_string_lossy()
                .into_owned()
        };

        // Log the reset operation
        info!("Counter reset in Rust backend - Element: {}", element_name);
        debug!(
            "Reset event details - element: {}, window: {}",
            element_name, event.window
        );
    });
}