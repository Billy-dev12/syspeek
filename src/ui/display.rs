use colored::*;

pub fn tampil_box(judul: &str, baris: Vec<String>) {
    let w: usize = 38;
    let garis = "═".repeat(w);

    println!();
    println!("{}", format!("╔{}╗", garis).cyan());
    println!("{}", format!("║{:^width$}║", judul, width = w).cyan().bold());
    println!("{}", format!("╠{}╣", garis).cyan());
    for line in &baris {
        println!("{}", format!("║ {:<width$}║", line, width = w - 1).white());
    }
    println!("{}", format!("╚{}╝", garis).cyan());
    println!();
}
