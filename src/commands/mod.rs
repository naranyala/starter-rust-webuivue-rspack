pub mod ui;
pub mod counter;
pub mod sysinfo;
pub mod window_state;

use webui_rs::webui;

pub fn setup_all_handlers(window: &mut webui::Window) {
    ui::setup_handlers(window);
    counter::setup_handlers(window);
    sysinfo::setup_handlers(window);
    window_state::setup_handlers(window);
}
