use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub working_dir: Option<PathBuf>,
    pub timeout: Option<Duration>,
    pub env_vars: Vec<(String, String)>,
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            working_dir: None,
            timeout: None,
            env_vars: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
}

pub struct ProcessUtility;

impl ProcessUtility {
    pub fn run_command<S: AsRef<str>>(
        program: S,
        args: &[S],
        options: ProcessOptions,
    ) -> ProcessResult {
        let program_str = program.as_ref();

        let mut cmd = Command::new(program_str);

        for arg in args {
            cmd.arg(arg.as_ref());
        }

        if let Some(dir) = &options.working_dir {
            cmd.current_dir(dir);
        }

        for (key, value) in &options.env_vars {
            cmd.env(key, value);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let start = std::time::Instant::now();

        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                return ProcessResult {
                    success: false,
                    exit_code: -1,
                    stdout: String::new(),
                    stderr: format!("Failed to execute: {}", e),
                    duration_ms: start.elapsed().as_millis(),
                };
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        ProcessResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout,
            stderr,
            duration_ms: start.elapsed().as_millis(),
        }
    }

    pub fn run_shell_command<S: AsRef<str>>(command: S) -> ProcessResult {
        let (shell, flag) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let args = vec![flag, command.as_ref()];

        ProcessUtility::run_command(shell, &args, ProcessOptions::default())
    }

    pub fn open_path<S: AsRef<str>>(path: S) -> bool {
        let path_str = path.as_ref();

        let result = if cfg!(windows) {
            ProcessUtility::run_command(
                "cmd",
                &["/C", "start", "", path_str],
                ProcessOptions::default(),
            )
        } else if cfg!(target_os = "macos") {
            ProcessUtility::run_command("open", &[path_str], ProcessOptions::default())
        } else {
            ProcessUtility::run_command("xdg-open", &[path_str], ProcessOptions::default())
        };

        result.success
    }

    pub fn get_system_info() -> String {
        let os = if cfg!(windows) {
            "Windows"
        } else if cfg!(target_os = "macos") {
            "macOS"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else {
            "Unknown"
        };

        format!("OS: {}", os)
    }
}
