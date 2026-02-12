use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ShellOptions {
    pub working_dir: Option<PathBuf>,
    pub run_elevated: bool,
}

impl Default for ShellOptions {
    fn default() -> Self {
        Self {
            working_dir: None,
            run_elevated: false,
        }
    }
}

impl ShellOptions {
    pub fn with_working_dir<P: Into<PathBuf>>(dir: P) -> Self {
        Self {
            working_dir: Some(dir.into()),
            run_elevated: false,
        }
    }

    pub fn run_elevated(mut self) -> Self {
        self.run_elevated = true;
        self
    }
}

pub struct Shell;

impl Shell {
    pub fn open_url<S: AsRef<str>>(url: S) -> bool {
        let url_str = url.as_ref();
        Self::native_open_url(url_str)
    }

    pub fn open_file<P: AsRef<std::path::Path>>(path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        Self::native_open_file(&path_str)
    }

    pub fn open_directory<P: AsRef<std::path::Path>>(path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        Self::native_open_directory(&path_str)
    }

    pub fn open_with_default<S: AsRef<str>>(target: S, options: ShellOptions) -> bool {
        if Self::open_url(target.as_ref()) {
            return true;
        }
        if Self::open_file(target.as_ref()) {
            return true;
        }
        false
    }

    pub fn reveal_file<P: AsRef<std::path::Path>>(path: P) -> bool {
        let path_str = path.as_ref().to_string_lossy();
        Self::native_reveal_file(&path_str)
    }

    pub fn get_default_app_for_file<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
        let path_str = path.as_ref().to_string_lossy();
        Self::native_get_default_app(&path_str)
    }

    pub fn share_file<P: AsRef<std::path::Path>>(path: P) -> bool {
        Self::native_share_file(path.as_ref())
    }

    fn native_open_url(_url: &str) -> bool {
        false
    }

    fn native_open_file(_path: &str) -> bool {
        false
    }

    fn native_open_directory(_path: &str) -> bool {
        false
    }

    fn native_reveal_file(_path: &str) -> bool {
        false
    }

    fn native_get_default_app(_path: &str) -> Option<String> {
        None
    }

    fn native_share_file(_path: &std::path::Path) -> bool {
        false
    }
}
