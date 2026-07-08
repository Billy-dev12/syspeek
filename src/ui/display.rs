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
