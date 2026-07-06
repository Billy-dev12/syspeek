use std::fs;
use std::thread::sleep; // Buat perintah tidur
use std::time::Duration; // Buat nentuin waktu (detik)

pub fn cpu_proses() {
    println!("Mohon tunggu 200 ms untuk menghitung...");

    let isi_1 = fs::read_to_string("/proc/stat").unwrap();
    let barisan_1 = isi_1.lines().next().unwrap();
    let bagian_1: Vec<&str> = barisan_1.split_whitespace().collect();
    
    let user_1: u64 = bagian_1[1].parse().unwrap();
    let nice_1: u64 = bagian_1[2].parse().unwrap();
    let system_1: u64 = bagian_1[3].parse().unwrap();
    let idle_1: u64 = bagian_1[4].parse().unwrap();
    
    let total_1 = user_1 + nice_1 + system_1 + idle_1;

    // ==========================================
    // 2. TIDUR DULU 1 DETIK
    // ==========================================
    sleep(Duration::from_millis(200));

    // ==========================================
    // 3. AMBIL DATA KEDUA (Detik ke-1)
    // ==========================================
    let isi_2 = fs::read_to_string("/proc/stat").unwrap();
    let barisan_2 = isi_2.lines().next().unwrap();
    let bagian_2: Vec<&str> = barisan_2.split_whitespace().collect();
    
    let user_2: u64 = bagian_2[1].parse().unwrap();
    let nice_2: u64 = bagian_2[2].parse().unwrap();
    let system_2: u64 = bagian_2[3].parse().unwrap();
    let idle_2: u64 = bagian_2[4].parse().unwrap();
    
    let total_2 = user_2 + nice_2 + system_2 + idle_2;

    // ==========================================
    // 4. HITUNG PERSENANNYA PAKE MATEMATIKA SIMPLE
    // ==========================================
    // Cari selisih angka antara data ke-2 dan data ke-1
    let selisih_total = (total_2 - total_1) as f64;
    let selisih_idle = (idle_2 - idle_1) as f64;

    // Rumus: (1 - (selisih nganggur / selisih total)) * 100
    let persen_cpu = (1.0 - (selisih_idle / selisih_total)) * 100.0;

    // Tampilkan hasil akhir yang dipahami manusia!
    println!("CPU Usage saat ini: {:.2}%", persen_cpu);
}