use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

pub struct WindowUtility;

impl WindowUtility {
    pub fn set_title<S: AsRef<str>>(title: S) -> bool {
        Self::native_set_title(title.as_ref())
    }

    pub fn get_title() -> String {
        Self::native_get_title()
    }

    pub fn set_size(width: u32, height: u32) -> bool {
        Self::native_set_size(width, height)
    }

    pub fn set_position(x: i32, y: i32) -> bool {
        Self::native_set_position(x, y)
    }

    pub fn minimize() -> bool {
        Self::native_set_state(WindowState::Minimized)
    }

    pub fn maximize() -> bool {
        Self::native_set_state(WindowState::Maximized)
    }

    pub fn restore() -> bool {
        Self::native_set_state(WindowState::Normal)
    }

    pub fn toggle_fullscreen() -> bool {
        Self::native_set_state(WindowState::Fullscreen)
    }

    pub fn show() -> bool {
        Self::native_show(true)
    }

    pub fn hide() -> bool {
        Self::native_show(false)
    }

    pub fn close() {
        Self::native_close();
    }

    pub fn flash(duration: Duration) -> bool {
        Self::native_flash(duration)
    }

    pub fn bring_to_front() -> bool {
        Self::native_bring_to_front()
    }

    fn native_set_title(_title: &str) -> bool {
        false
    }

    fn native_get_title() -> String {
        String::new()
    }

    fn native_set_size(_width: u32, _height: u32) -> bool {
        false
    }

    fn native_set_position(_x: i32, _y: i32) -> bool {
        false
    }

    fn native_set_state(_state: WindowState) -> bool {
        false
    }

    fn native_show(_visible: bool) -> bool {
        false
    }

    fn native_close() {}

    fn native_flash(_duration: Duration) -> bool {
        false
    }

    fn native_bring_to_front() -> bool {
        false
    }
}
