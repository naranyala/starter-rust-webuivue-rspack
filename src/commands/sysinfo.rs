use log::info;
use webui_rs::webui;

pub fn setup_handlers(window: &mut webui::Window) {
    window.bind("get_system_info", |event| {
        info!("get_system_info called from frontend");

        let sysinfo = get_system_info();

        let response = serde_json::json!({
            "success": true,
            "data": sysinfo
        });

        let js = format!(
            "window.dispatchEvent(new CustomEvent('sysinfo_response', {{ detail: {} }}))",
            response.to_string()
        );

        webui::Window::from_id(event.window).run_js(&js);
    });

    window.bind("get_uptime", |event| {
        info!("get_uptime called from frontend");

        let uptime_str = get_uptime();
        let uptime_seconds = parse_uptime_to_seconds(&uptime_str);

        let response = serde_json::json!({
            "success": true,
            "uptime_seconds": uptime_seconds
        });

        let js = format!(
            "window.dispatchEvent(new CustomEvent('uptime_response', {{ detail: {} }}))",
            response.to_string()
        );

        webui::Window::from_id(event.window).run_js(&js);
    });

    info!("System info handlers set up successfully");
}

fn get_system_info() -> serde_json::Value {
    let mut sysinfo = serde_json::Map::new();

    sysinfo.insert(
        "os".to_string(),
        serde_json::json!({
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "family": std::env::consts::FAMILY,
        }),
    );

    let mem_info = get_memory_info();
    sysinfo.insert("memory".to_string(), mem_info);

    let cpu_info = get_cpu_info();
    sysinfo.insert("cpu".to_string(), cpu_info);

    let disk_info = get_disk_info();
    sysinfo.insert("disk".to_string(), disk_info);

    let uptime = get_uptime();
    sysinfo.insert("uptime".to_string(), serde_json::json!(uptime));

    sysinfo.insert(
        "env_vars".to_string(),
        serde_json::json!(std::env::vars_os().count()),
    );

    let current_dir = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    sysinfo.insert("cwd".to_string(), serde_json::json!(current_dir));

    serde_json::Value::Object(sysinfo)
}

fn get_memory_info() -> serde_json::Value {
    let mut mem = serde_json::Map::new();

    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().split_whitespace().next();

                match key {
                    "MemTotal" => {
                        mem.insert(
                            "total_mb".to_string(),
                            serde_json::json!(parse_mem_value(value)),
                        );
                    }
                    "MemFree" => {
                        mem.insert(
                            "free_mb".to_string(),
                            serde_json::json!(parse_mem_value(value)),
                        );
                    }
                    "MemAvailable" => {
                        mem.insert(
                            "available_mb".to_string(),
                            serde_json::json!(parse_mem_value(value)),
                        );
                    }
                    "Buffers" => {
                        mem.insert(
                            "buffers_mb".to_string(),
                            serde_json::json!(parse_mem_value(value)),
                        );
                    }
                    "Cached" => {
                        mem.insert(
                            "cached_mb".to_string(),
                            serde_json::json!(parse_mem_value(value)),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    serde_json::Value::Object(mem)
}

fn parse_mem_value(value: Option<&str>) -> f64 {
    match value {
        Some(v) => v.parse::<u64>().unwrap_or(0) as f64 / 1024.0,
        None => 0.0,
    }
}

fn get_cpu_info() -> serde_json::Value {
    let mut cpu = serde_json::Map::new();

    if let Ok(count) = std::fs::read_to_string("/proc/cpuinfo") {
        let core_count = count.lines().filter(|l| l.starts_with("processor")).count();
        cpu.insert("cores".to_string(), serde_json::json!(core_count));
    }

    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("Model") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    cpu.insert("model".to_string(), serde_json::json!(parts[1].trim()));
                    break;
                }
            }
        }
    }

    cpu.insert(
        "usage_percent".to_string(),
        serde_json::json!(get_cpu_usage()),
    );

    serde_json::Value::Object(cpu)
}

fn get_cpu_usage() -> f64 {
    if let Ok(content) = std::fs::read_to_string("/proc/stat") {
        let lines: Vec<&str> = content.lines().collect();
        if let Some(first_line) = lines.first() {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 8 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total = user + system + idle;

                if total > 0 {
                    return ((user + system) as f64 / total as f64) * 100.0;
                }
            }
        }
    }
    0.0
}

fn get_disk_info() -> serde_json::Value {
    let mut disks = Vec::new();

    if let Ok(output) = std::process::Command::new("df")
        .args(&["-h", "-P", "-x", "tmpfs", "-x", "devtmpfs"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    disks.push(serde_json::json!({
                        "filesystem": parts[0],
                        "size": parts[1],
                        "used": parts[2],
                        "available": parts[3],
                        "use_percent": parts[4],
                        "mount": parts[5],
                    }));
                }
            }
        }
    }

    serde_json::Value::Array(disks)
}

fn get_uptime() -> String {
    if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if let Some(uptime_str) = parts.first() {
            if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                let days = (uptime_secs / 86400.0) as u64;
                let hours = ((uptime_secs % 86400.0) / 3600.0) as u64;
                let minutes = ((uptime_secs % 3600.0) / 60.0) as u64;
                let seconds = (uptime_secs % 60.0) as u64;

                return format!("{}d {}h {}m {}s", days, hours, minutes, seconds);
            }
        }
    }
    "unknown".to_string()
}

fn parse_uptime_to_seconds(uptime_str: &str) -> u64 {
    if uptime_str == "unknown" {
        return 0;
    }

    let mut total_seconds = 0;

    let parts: Vec<&str> = uptime_str.split_whitespace().collect();
    for part in parts {
        if part.ends_with('d') {
            if let Ok(days) = part.trim_end_matches('d').parse::<u64>() {
                total_seconds += days * 86400;
            }
        } else if part.ends_with('h') {
            if let Ok(hours) = part.trim_end_matches('h').parse::<u64>() {
                total_seconds += hours * 3600;
            }
        } else if part.ends_with('m') {
            if let Ok(minutes) = part.trim_end_matches('m').parse::<u64>() {
                total_seconds += minutes * 60;
            }
        } else if part.ends_with('s') {
            if let Ok(seconds) = part.trim_end_matches('s').parse::<u64>() {
                total_seconds += seconds;
            }
        }
    }

    total_seconds
}
