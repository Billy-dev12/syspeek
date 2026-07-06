use std::fs;

pub fn data_ram() -> Vec<String> {
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

    let terpakai = total - available;
    let total_gb = total as f64 / 1_048_576.0;
    let available_gb = available as f64 / 1_048_576.0;
    let terpakai_gb = terpakai as f64 / 1_048_576.0;

    vec![
        format!("Total RAM     : {:.2} GB", total_gb),
        format!("Terpakai      : {:.2} GB", terpakai_gb),
        format!("Tersisa       : {:.2} GB", available_gb),
    ]
}
