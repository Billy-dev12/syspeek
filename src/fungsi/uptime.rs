use std::fs;

pub fn data_uptime() {
    let data = fs::read_to_string("/proc/uptime").unwrap();
    let bagian: Vec<&str> = data.split_whitespace().collect();

    if bagian.len() < 1 {
        println!("Data uptime tidak tersedia.");
        return;
    }

    let uptime_detik: f64 = bagian[0].parse().unwrap_or(0.0);
    let uptime_menit = uptime_detik / 60.0;
    let uptime_jam = uptime_menit / 60.0;
    let uptime_hari = uptime_jam / 24.0;
    
    print!("\nUptime: {:.2} menit", uptime_menit);
    print!("\nUptime: {:.2} jam", uptime_jam);
    print!("\nUptime: {:.2} hari", uptime_hari);
}