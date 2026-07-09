use std::fs;
use crate::ui::display::buat_bar;

fn baca_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn jalankan_batre() -> Vec<String> {
    let base = "/sys/class/power_supply/BAT0";

    let kapasitas = baca_file(&format!("{}/capacity", base));
    let status = baca_file(&format!("{}/status", base));
    let charge_now = baca_file(&format!("{}/charge_now", base));
    let charge_full = baca_file(&format!("{}/charge_full", base));
    let cycle = baca_file(&format!("{}/cycle_count", base));

    // charge_now & charge_full dalam µAh, convert ke mAh
    let now_mah = charge_now.parse::<f64>().unwrap_or(0.0) / 1000.0;
    let full_mah = charge_full.parse::<f64>().unwrap_or(0.0) / 1000.0;
    let persen = kapasitas.parse::<f64>().unwrap_or(0.0);

    vec![
        format!("Baterai       : {} {}%", buat_bar(persen, 12), kapasitas),
        format!("Status        : {}", status),
        format!("Sisa          : {:.0}/{:.0} mAh", now_mah, full_mah),
        format!("Siklus        : {} kali", cycle),
    ]
}