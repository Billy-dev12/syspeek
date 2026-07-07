use std::fs;

pub fn read_wireless() -> Vec<String> {
    let path = "/proc/net/wireless";
    let content = fs::read_to_string(path).expect("Gagal membaca file /proc/net/wireless");
    let lines: Vec<&str> = content.lines().collect();

    if lines.len() < 3 {
        return vec!["Tidak ada data wireless.".to_string()];
    }

    let data_line = lines[2];
    let parts: Vec<&str> = data_line.split_whitespace().collect();

    if parts.len() < 5 {
        return vec!["Format data wireless tidak sesuai.".to_string()];
    }

    let interface = parts[0].trim_end_matches(':');
    let link_quality: f32 = parts[2].parse().unwrap_or(0.0);
    let signal_level: f32 = parts[3].parse().unwrap_or(0.0);
    let noise_level: f32 = parts[4].parse().unwrap_or(0.0);

    vec![
        format!("Interface  : {}", interface),
        format!("Link       : {}%", link_quality),
        format!(
            "Sinyal     : {} dBm {}",
            signal_level,
            keterangan_sinyal(signal_level)
        ),
        format!("Noise      : {} dBm", noise_level),
    ]
}

fn keterangan_sinyal(dbm: f32) -> &'static str {
    match dbm as i32 {
        -50..=0 => "(Sangat Kuat 💪)",
        -67..=-51 => "(Kuat 🟢)",
        -75..=-68 => "(Sedang 🟡)",
        -85..=-76 => "(Lemah 🟠)",
        _ => "(Sangat Lemah 🔴)",
    }
}
