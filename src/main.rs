mod fungsi;
use colored::*;

fn main() {
    let w: usize = 32;
    let garis = "═".repeat(w);

    let cpu = fungsi::cpu::cpu_proses();
    let ram_lines = fungsi::ram::data_ram();
    let uptime = fungsi::uptime::data_uptime();

    println!();
    println!("{}", format!("╔{}╗", garis).cyan());
    println!("{}", format!("║{:^width$}║", "🖥️  SYSPEEK", width = w).cyan().bold());
    println!("{}", format!("╠{}╣", garis).cyan());
    println!("{}", format!("║ {:<width$}║", cpu, width = w - 1).white());
    for line in &ram_lines {
        println!("{}", format!("║ {:<width$}║", line, width = w - 1).white());
    }
    println!("{}", format!("║ {:<width$}║", uptime, width = w - 1).white());
    println!("{}", format!("╚{}╝", garis).cyan());
    println!();
}
