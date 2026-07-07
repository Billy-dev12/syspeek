mod fungsi;
use colored::*;

fn main() {
    let w: usize = 32;
    let garis = "═".repeat(w);

    let cpu = fungsi::proc::cpu::cpu_proses();
    let ram_lines = fungsi::proc::ram::data_ram();
    let uptime = fungsi::proc::uptime::data_uptime();
    let version = fungsi::proc::version::data_versi();

    println!();
    println!("{}", format!("╔{}╗", garis).cyan());
    println!("{}", format!("║{:^width$}║", "🖥️  SYSPEEK", width = w).cyan().bold());
    println!("{}", format!("╠{}╣", garis).cyan());
    println!("{}", format!("║ {:<width$}║", cpu, width = w - 1).white());
    for line in &ram_lines {
        println!("{}", format!("║ {:<width$}║", line, width = w - 1).white());
    }
    println!("{}", format!("║ {:<width$}║", uptime, width = w - 1).white());
    println!("{}", format!("║ {:<width$}║", version, width = w - 1).white());
    println!("{}", format!("╚{}╝", garis).cyan());
    println!();
}
