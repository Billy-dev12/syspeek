mod fungsi;

fn main() {
    println!("Mengaktifkan fungsi CPU, RAM, dan Uptime...");
    fungsi::cpu::cpu_proses();
    fungsi::ram::data_ram();
    print!("\n");
    fungsi::uptime::data_uptime();
}
