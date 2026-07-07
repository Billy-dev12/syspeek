use std::fs;

pub fn data_versi() -> String {
    let data = fs::read_to_string("/proc/version").unwrap();
    let bagian: Vec<&str> = data.split_whitespace().collect();

    if bagian.len() < 3 {
        return "Versi Kernel  : N/A".to_string();
    }

    format!("Versi Kernel  : {}", bagian[2])
}
