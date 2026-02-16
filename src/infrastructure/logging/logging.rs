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

    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, log_level);

    if let Some(file_path) = log_file {
        use std::fs::OpenOptions;
        let file = OpenOptions::new()
            .create(true)
            .append(append)
            .truncate(!append)
            .open(file_path)?;

        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }

    builder.try_init()?;
    Ok(())
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
    builder
        .filter(None, log_level)
        .format(|buf, record| {
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