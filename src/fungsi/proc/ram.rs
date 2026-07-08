use std::fs;

/// Return (terpakai_gb, total_gb) sebagai tuple f64
pub fn ram_stats() -> (f64, f64) {
    let data = fs::read_to_string("/proc/meminfo").unwrap();

    let mut total: u64 = 0;
    let mut available: u64 = 0;

    for baris in data.lines() {
        let bagian: Vec<&str> = baris.split_whitespace().collect();

        if bagian.len() < 2 {
            continue;
        }

        match bagian[0] {
            "MemTotal:" => {
                total = bagian[1].parse().unwrap_or(0);
            }
            "MemAvailable:" => {
                available = bagian[1].parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    let total_gb = total as f64 / 1_048_576.0;
    let terpakai_gb = (total - available) as f64 / 1_048_576.0;

    (terpakai_gb, total_gb)
}


