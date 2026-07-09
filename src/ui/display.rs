/// Buat loading bar dari persentase
/// 
/// # Contoh
/// ```
/// let bar = buat_bar(50.0, 16);
/// // return: "[████████░░░░░░░░]"
/// ```
pub fn buat_bar(persen: f64, panjang: usize) -> String {
    let terisi = (persen / 100.0 * panjang as f64) as usize;
    let kosong = panjang - terisi;
    format!("[{}{}]", "█".repeat(terisi), "░".repeat(kosong))
}

/// Format output CPU dengan loading bar
pub fn tampil_cpu() -> String {
    let persen = crate::fungsi::proc::cpu::cpu_persen();
    format!("CPU Usage     : {} {:.2}%", buat_bar(persen, 16), persen)
}

/// Format output RAM dengan loading bar
pub fn tampil_ram() -> Vec<String> {
    let (terpakai, total) = crate::fungsi::proc::ram::ram_stats();
    let persen = (terpakai / total) * 100.0;
    
    vec![
        format!("Total RAM     : {:.2} GB", total),
        format!("Terpakai      : {} {:.2} GB ({:.1}%)", buat_bar(persen, 12), terpakai, persen),
        format!("Tersisa       : {:.2} GB", total - terpakai),
    ]
}

/// Tampilan side-by-side: proc (kiri) | sys (kanan)
pub fn tampil_all_side_by_side() {
    // Data proc (kiri)
    let mut kiri: Vec<String> = Vec::new();
    kiri.push(tampil_cpu());
    kiri.extend(tampil_ram());
    kiri.push(crate::fungsi::proc::uptime::data_uptime());
    kiri.push(crate::fungsi::proc::version::data_versi());

    // Data sys (kanan)
    let kanan: Vec<String> = crate::fungsi::sys::batre::jalankan_batre();

    // Cari baris terpanjang di kiri
    let max_kiri = kiri.iter().map(|s| s.len()).max().unwrap_or(0);
    let lebar_kiri = max_kiri + 2; // tambah padding

    // Gabungkan baris per baris
    let max_baris = kiri.len().max(kanan.len());
    for i in 0..max_baris {
        let kiri_str = if i < kiri.len() {
            format!("{:<width$}", kiri[i], width = lebar_kiri)
        } else {
            " ".repeat(lebar_kiri)
        };

        let kanan_str = if i < kanan.len() {
            format!("| {}", kanan[i])
        } else {
            String::new()
        };

        println!("{}{}", kiri_str, kanan_str);
    }
}
