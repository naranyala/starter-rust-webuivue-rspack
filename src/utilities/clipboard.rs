use std::fmt;

#[derive(Debug, Clone)]
pub struct ClipboardContent {
    pub text: Option<String>,
}

impl fmt::Display for ClipboardContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.text {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "(empty)"),
        }
    }
}

pub struct Clipboard;

impl Clipboard {
    pub fn set_text<S: Into<String>>(text: S) -> bool {
        let text = text.into();
        Self::native_set_text(&text)
    }

    pub fn get_text() -> ClipboardContent {
        let text = Self::native_get_text();
        ClipboardContent { text }
    }

    pub fn clear() -> bool {
        Self::set_text("")
    }

    pub fn has_text() -> bool {
        Self::get_text().text.is_some()
    }

    fn native_set_text(_text: &str) -> bool {
        false
    }

    fn native_get_text() -> Option<String> {
        None
    }
}
