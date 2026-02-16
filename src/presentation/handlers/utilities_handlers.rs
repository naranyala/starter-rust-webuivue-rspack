use log::{debug, info};
use webui_rs::webui;

pub fn setup_utilities_handlers(window: &mut webui::Window) {
    setup_clipboard_handlers(window);
    setup_process_handlers(window);
    setup_notification_handlers(window);
    setup_window_handlers(window);
    setup_shell_handlers(window);
    setup_file_dialog_handlers(window);
}

fn setup_clipboard_handlers(window: &mut webui::Window) {
    window.bind("clipboard_set_text", |event| {
        info!("Clipboard set text requested");
        debug!("Clipboard event - window: {}", event.window);
    });

    window.bind("clipboard_get_text", |event| {
        info!("Clipboard get text requested");
        debug!("Clipboard get event - window: {}", event.window);
    });

    window.bind("clipboard_clear", |event| {
        info!("Clipboard clear requested");
        debug!("Clipboard clear event - window: {}", event.window);
    });
}

fn setup_process_handlers(window: &mut webui::Window) {
    window.bind("process_run_command", |event| {
        info!("Process command execution requested");
        debug!("Process event - window: {}", event.window);
    });

    window.bind("process_run_shell", |event| {
        info!("Shell command execution requested");
        debug!("Shell process event - window: {}", event.window);
    });

    window.bind("process_open_path", |event| {
        info!("Open path requested");
        debug!("Open path event - window: {}", event.window);
    });
}

fn setup_notification_handlers(window: &mut webui::Window) {
    window.bind("notification_show", |event| {
        info!("Notification requested");
        debug!("Notification event - window: {}", event.window);
    });

    window.bind("notification_show_info", |event| {
        info!("Info notification requested");
        debug!("Info notification event - window: {}", event.window);
    });

    window.bind("notification_show_warning", |event| {
        info!("Warning notification requested");
        debug!("Warning notification event - window: {}", event.window);
    });

    window.bind("notification_show_error", |event| {
        info!("Error notification requested");
        debug!("Error notification event - window: {}", event.window);
    });

    window.bind("notification_show_success", |event| {
        info!("Success notification requested");
        debug!("Success notification event - window: {}", event.window);
    });
}

fn setup_window_handlers(window: &mut webui::Window) {
    window.bind("window_set_title", |event| {
        info!("Window title change requested");
        debug!("Window title event - window: {}", event.window);
    });

    window.bind("window_minimize", |event| {
        info!("Window minimize requested");
        debug!("Window minimize event - window: {}", event.window);
    });

    window.bind("window_maximize", |event| {
        info!("Window maximize requested");
        debug!("Window maximize event - window: {}", event.window);
    });

    window.bind("window_restore", |event| {
        info!("Window restore requested");
        debug!("Window restore event - window: {}", event.window);
    });

    window.bind("window_toggle_fullscreen", |event| {
        info!("Window fullscreen toggle requested");
        debug!("Window fullscreen event - window: {}", event.window);
    });

    window.bind("window_close", |event| {
        info!("Window close requested");
        debug!("Window close event - window: {}", event.window);
    });
}

fn setup_shell_handlers(window: &mut webui::Window) {
    window.bind("shell_open_url", |event| {
        info!("Open URL requested");
        debug!("Shell open URL event - window: {}", event.window);
    });

    window.bind("shell_open_file", |event| {
        info!("Open file requested");
        debug!("Shell open file event - window: {}", event.window);
    });

    window.bind("shell_open_directory", |event| {
        info!("Open directory requested");
        debug!("Shell open directory event - window: {}", event.window);
    });

    window.bind("shell_reveal_file", |event| {
        info!("Reveal file in finder requested");
        debug!("Shell reveal file event - window: {}", event.window);
    });
}

fn setup_file_dialog_handlers(window: &mut webui::Window) {
    window.bind("file_dialog_open", |event| {
        info!("File open dialog requested");
        debug!("File dialog open event - window: {}", event.window);
    });

    window.bind("file_dialog_open_multiple", |event| {
        info!("Multiple file selection dialog requested");
        debug!("File dialog multiple event - window: {}", event.window);
    });

    window.bind("file_dialog_select_directory", |event| {
        info!("Directory selection dialog requested");
        debug!("File dialog directory event - window: {}", event.window);
    });

    window.bind("file_dialog_save", |event| {
        info!("File save dialog requested");
        debug!("File dialog save event - window: {}", event.window);
    });
}
