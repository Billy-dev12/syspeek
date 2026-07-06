use std::fs;

pub fn data_ram() {
    let data = fs::read_to_string("/proc/meminfo").unwrap();

    let mut total: u64 = 0;
    let mut available: u64 = 0;

    for baris in data.lines() {
        let bagian: Vec<&str> = baris.split_whitespace().collect();

        if bagian.len() < 2 {
            continue; // Lewati baris yang tidak memiliki cukup data
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
    let persen = (terpakai as f64 / total as f64) * 100.0;
    // konversi semua kb menjadi gb 
    let total_gb = total as f64 / 1_048_576.0;
    let available_gb = available as f64 / 1_048_576.0;
    let terpakai_gb = terpakai as f64 / 1_048_576.0;
    print!("\nTotal RAM: {:.2} GB", total_gb);
    print!("\nRAM Terpakai: {:.2} GB", terpakai_gb);
    print!("\nRAM Tersisa: {:.2} GB", available_gb);
    print!("\nPersentase RAM Terpakai: {:.2}%", persen);
}