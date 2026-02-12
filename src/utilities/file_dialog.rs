use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OpenDialogOptions {
    pub title: Option<String>,
    pub filters: Vec<FileFilter>,
    pub multi_selection: bool,
    pub directory: bool,
}

#[derive(Debug, Clone)]
pub struct SaveDialogOptions {
    pub title: Option<String>,
    pub default_name: Option<String>,
    pub filters: Vec<FileFilter>,
}

impl Default for OpenDialogOptions {
    fn default() -> Self {
        Self {
            title: None,
            filters: Vec::new(),
            multi_selection: false,
            directory: false,
        }
    }
}

impl Default for SaveDialogOptions {
    fn default() -> Self {
        Self {
            title: None,
            default_name: None,
            filters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DialogResult {
    pub success: bool,
    pub paths: Vec<PathBuf>,
    pub error: Option<String>,
}

pub struct FileDialog;

impl FileDialog {
    pub fn open_file(options: OpenDialogOptions) -> DialogResult {
        Self::native_open_file(options)
    }

    pub fn open_files(options: OpenDialogOptions) -> DialogResult {
        let mut opts = options;
        opts.multi_selection = true;
        Self::native_open_file(opts)
    }

    pub fn select_directory(options: OpenDialogOptions) -> DialogResult {
        let mut opts = options;
        opts.directory = true;
        Self::native_open_file(opts)
    }

    pub fn save_file(options: SaveDialogOptions) -> DialogResult {
        Self::native_save_file(options)
    }

    fn native_open_file(_options: OpenDialogOptions) -> DialogResult {
        DialogResult {
            success: false,
            paths: Vec::new(),
            error: Some("File dialog requires native bridge implementation".to_string()),
        }
    }

    fn native_save_file(_options: SaveDialogOptions) -> DialogResult {
        DialogResult {
            success: false,
            paths: Vec::new(),
            error: Some("File dialog requires native bridge implementation".to_string()),
        }
    }

    pub fn with_filter(name: &str, ext: &str) -> FileFilter {
        FileFilter {
            name: name.to_string(),
            extensions: vec![ext.to_string()],
        }
    }

    pub fn with_filters(name: &str, extensions: &[&str]) -> FileFilter {
        FileFilter {
            name: name.to_string(),
            extensions: extensions.iter().map(|s| s.to_string()).collect(),
        }
    }
}
