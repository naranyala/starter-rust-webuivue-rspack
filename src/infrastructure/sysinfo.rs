use serde_json::{json, Value};

pub fn get_system_info() -> Value {
    let mut sysinfo = serde_json::Map::new();

    sysinfo.insert(
        "os".to_string(),
        json!({
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "family": std::env::consts::FAMILY,
        }),
    );

    sysinfo.insert("memory".to_string(), get_memory_info());
    sysinfo.insert("cpu".to_string(), get_cpu_info());
    sysinfo.insert("disk".to_string(), get_disk_info());
    sysinfo.insert("uptime".to_string(), json!(get_uptime()));
    sysinfo.insert(
        "env_vars".to_string(),
        json!(std::env::vars_os().count()),
    );

    let current_dir = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    sysinfo.insert("cwd".to_string(), json!(current_dir));

    Value::Object(sysinfo)
}

pub fn get_memory_info() -> Value {
    let mut mem = serde_json::Map::new();

    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().split_whitespace().next();

                match key {
                    "MemTotal" => {
                        mem.insert("total_mb".to_string(), json!(parse_mem_value(value)));
                    }
                    "MemFree" => {
                        mem.insert("free_mb".to_string(), json!(parse_mem_value(value)));
                    }
                    "MemAvailable" => {
                        mem.insert("available_mb".to_string(), json!(parse_mem_value(value)));
                    }
                    "Buffers" => {
                        mem.insert("buffers_mb".to_string(), json!(parse_mem_value(value)));
                    }
                    "Cached" => {
                        mem.insert("cached_mb".to_string(), json!(parse_mem_value(value)));
                    }
                    _ => {}
                }
            }
        }
    }

    Value::Object(mem)
}

fn parse_mem_value(value: Option<&str>) -> f64 {
    match value {
        Some(v) => v.parse::<u64>().unwrap_or(0) as f64 / 1024.0,
        None => 0.0,
    }
}

pub fn get_cpu_info() -> Value {
    let mut cpu = serde_json::Map::new();

    if let Ok(count) = std::fs::read_to_string("/proc/cpuinfo") {
        let core_count = count.lines().filter(|l| l.starts_with("processor")).count();
        cpu.insert("cores".to_string(), json!(core_count));
    }

    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("Model") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    cpu.insert("model".to_string(), json!(parts[1].trim()));
                    break;
                }
            }
        }
    }

    cpu.insert("usage_percent".to_string(), json!(get_cpu_usage()));

    Value::Object(cpu)
}

pub fn get_cpu_usage() -> f64 {
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

pub fn get_disk_info() -> Value {
    let mut disks = Vec::new();

    if let Ok(output) = std::process::Command::new("df")
        .args(&["-h", "-P", "-x", "tmpfs", "-x", "devtmpfs"])
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    disks.push(json!({
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

    Value::Array(disks)
}

pub fn get_uptime() -> String {
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

pub fn parse_uptime_to_seconds(uptime_str: &str) -> u64 {
    if uptime_str == "unknown" {
        return 0;
    }

    let mut total_seconds = 0u64;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uptime_to_seconds_unknown() {
        assert_eq!(parse_uptime_to_seconds("unknown"), 0);
    }

    #[test]
    fn test_parse_uptime_to_seconds_single_unit() {
        assert_eq!(parse_uptime_to_seconds("1d"), 86400);
        assert_eq!(parse_uptime_to_seconds("2h"), 7200);
        assert_eq!(parse_uptime_to_seconds("30m"), 1800);
        assert_eq!(parse_uptime_to_seconds("45s"), 45);
    }

    #[test]
    fn test_parse_uptime_to_seconds_full_format() {
        assert_eq!(parse_uptime_to_seconds("1d 2h 30m 45s"), 95445);
    }

    #[test]
    fn test_parse_uptime_to_seconds_empty() {
        assert_eq!(parse_uptime_to_seconds(""), 0);
    }

    #[test]
    fn test_parse_uptime_to_seconds_partial() {
        assert_eq!(parse_uptime_to_seconds("1d 2h"), 93600);
        assert_eq!(parse_uptime_to_seconds("30m 45s"), 1845);
    }

    #[test]
    fn test_parse_mem_value() {
        assert_eq!(parse_mem_value(Some("16384000")), 16000.0);
        assert_eq!(parse_mem_value(Some("8192000")), 8000.0);
        assert_eq!(parse_mem_value(None), 0.0);
        assert_eq!(parse_mem_value(Some("invalid")), 0.0);
    }

    #[test]
    fn test_get_cpu_usage_returns_valid_range() {
        let usage = get_cpu_usage();
        assert!(usage >= 0.0);
        assert!(usage <= 100.0);
    }
}
