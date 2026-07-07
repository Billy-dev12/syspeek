use std::env;
use crate::fungsi;

pub fn jalankan() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("cpu") => {
            println!("{}", fungsi::proc::cpu::cpu_proses());
        }
        Some("ram") => {
            for baris in fungsi::proc::ram::data_ram() {
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
            println!("{}", fungsi::proc::cpu::cpu_proses());
            for baris in fungsi::proc::ram::data_ram() {
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
    println!("Usage: syspeek <command>\n");
    println!("Commands:");
    println!("  cpu       CPU usage percentage");
    println!("  ram       RAM information");
    println!("  uptime    System uptime");
    println!("  version   Kernel version");
    println!("  wireless  WiFi signal info");
    println!("  all       Show all info");
    println!("  help      Show this help");
}