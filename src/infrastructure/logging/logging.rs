use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logging_with_config(
    log_file: Option<&str>,
    level: &str,
    append: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let file_path = log_file.map(|s| s.to_string());

    let mut builder = Builder::new();
    builder
        .filter(None, log_level)
        .write_style(env_logger::WriteStyle::Always);

    if let Some(ref path) = file_path {
        use std::fs::OpenOptions;
        let file = OpenOptions::new()
            .create(true)
            .append(append)
            .truncate(!append)
            .open(path)?;

        let file = std::sync::Mutex::new(file);

        builder.target(env_logger::Target::Pipe(Box::new(LogWriter { file: file })));
    } else {
        builder.target(env_logger::Target::Stderr);
    }

    builder.try_init()?;
    Ok(())
}

struct LogWriter {
    file: std::sync::Mutex<std::fs::File>,
}

impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let line = String::from_utf8_lossy(buf);

        if let Some(pos) = line.find("] [INFO] [") {
            let rest = &line[pos + 1..];
            if let Some(msg_start) = rest.find("] ") {
                let msg = &rest[msg_start + 2..];
                eprintln!("{}", msg.trim());
            }
        } else if let Some(pos) = line.find("] [WARN] [") {
            let rest = &line[pos + 1..];
            if let Some(msg_start) = rest.find("] ") {
                let msg = &rest[msg_start + 2..];
                eprintln!("[WARN] {}", msg.trim());
            }
        } else if let Some(pos) = line.find("] [ERROR] [") {
            let rest = &line[pos + 1..];
            if let Some(msg_start) = rest.find("] ") {
                let msg = &rest[msg_start + 2..];
                eprintln!("[ERROR] {}", msg.trim());
            }
        } else if let Some(pos) = line.find("] [DEBUG] [") {
            let rest = &line[pos + 1..];
            if let Some(msg_start) = rest.find("] ") {
                let msg = &rest[msg_start + 2..];
                eprintln!("[DEBUG] {}", msg.trim());
            }
        } else if let Some(pos) = line.find("] [TRACE] [") {
            let rest = &line[pos + 1..];
            if let Some(msg_start) = rest.find("] ") {
                let msg = &rest[msg_start + 2..];
                eprintln!("[TRACE] {}", msg.trim());
            }
        }

        let mut file = self.file.lock().unwrap();
        file.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut file = self.file.lock().unwrap();
        file.flush()?;
        Ok(())
    }
}

pub fn init_logging(level: &str, _target_level: &str) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let mut builder = Builder::new();
    builder.filter(None, log_level).format(|buf, record| {
        writeln!(
            buf,
            "[{}] [{}] [{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    let _ = builder.try_init();
}
