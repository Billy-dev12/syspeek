use std::fs;

pub fn data_uptime() -> String {
    let data = fs::read_to_string("/proc/uptime").unwrap();
    let bagian: Vec<&str> = data.split_whitespace().collect();

    if bagian.len() < 1 {
        return "Uptime        : N/A".to_string();
    }

    let uptime_detik: f64 = bagian[0].parse().unwrap_or(0.0);
    let uptime_jam = uptime_detik / 3600.0;
    let uptime_hari = uptime_jam / 24.0;

    if uptime_hari >= 1.0 {
        format!("Uptime        : {:.1} hari", uptime_hari)
    } else {
        format!("Uptime        : {:.2} jam", uptime_jam)
    }
}
