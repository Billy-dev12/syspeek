use crate::fungsi;
use crate::ui::display;
use std::env;
use std::thread;
use std::time::Duration;

pub fn jalankan() {
    let args: Vec<String> = env::args().collect();
    let mode_watch = args.iter().any(|a| a == "--watch");

    let command = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .map(|s| s.as_str());

    // Validasi command dulu
    if !command_valid(command) {
        match command {
            Some(cmd) => eprintln!("Error: command '{}' tidak dikenal.\n", cmd),
            None => tampilkan_help(),
        }
        return;
    }

    if mode_watch {
        // Hitung jumlah baris output dulu
        let jumlah_baris = hitung_baris(command);

        // Clear screen sekali
        print!("\x1B[2J\x1B[1;1H");
        println!("SysPeek versi 1.0.2\n");

        // Tampilin data pertama kali
        jalankan_command(command);

        // Loop: update tanpa kedip
        loop {
            // Move cursor naik sebanyak jumlah baris data
            for _ in 0..jumlah_baris {
                print!("\x1B[1A"); // naik 1 baris
                print!("\x1B[2K"); // clear baris itu
            }

            // Tampilin data baru
            jalankan_command(command);

            thread::sleep(Duration::from_secs(2));
        }
    } else {
        jalankan_command(command);
    }
}

fn command_valid(command: Option<&str>) -> bool {
    matches!(command, Some("cpu") | Some("ram") | Some("uptime") | Some("version") | Some("wireless") | Some("all") | Some("help") | None)
}

fn hitung_baris(command: Option<&str>) -> usize {
    match command {
        Some("cpu") | Some("uptime") | Some("version") => 1,
        Some("ram") => 3,
        Some("wireless") => 4,  // Interface, Link, Sinyal, Noise
        Some("all") => 7, // cpu(1) + ram(3) + uptime(1) + version(1) + wireless(4) - 2 (overlap)
        _ => 1,
    }
}

fn jalankan_command(command: Option<&str>) {
    match command {
        Some("cpu") => {
            println!("{}", display::tampil_cpu());
        }
        Some("ram") => {
            for baris in display::tampil_ram() {
                println!("{}", baris);
            }
        }
        Some("uptime") => {
            println!("{}", fungsi::proc::uptime::data_uptime());
        }
        Some("version") => {
            println!("{}", fungsi::proc::version::data_versi());
        }
        Some("wireless") => {
            for baris in fungsi::proc::wireles::read_wireless() {
                println!("{}", baris);
            }
        }
        Some("all") => {
            println!("{}", display::tampil_cpu());
            for baris in display::tampil_ram() {
                println!("{}", baris);
            }
            println!("{}", fungsi::proc::uptime::data_uptime());
            println!("{}", fungsi::proc::version::data_versi());
        }
        Some("help") | None => tampilkan_help(),
        Some(cmd) => eprintln!("Error: command '{}' tidak dikenal.\n", cmd),
    }
}

fn tampilkan_help() {
    println!("syspeek — Linux system info tool\n");
    println!("Usage: syspeek <command> [flags]\n");
    println!("Commands:");
    println!("  cpu       CPU usage percentage");
    println!("  ram       RAM information");
    println!("  uptime    System uptime");
    println!("  version   Kernel version");
    println!("  wireless  WiFi signal info");
    println!("  all       Show all info");
    println!("  help      Show this help\n");
    println!("Flags:");
    println!("  --watch   Auto-refresh setiap 2 detik (Ctrl+C untuk keluar)");
}
