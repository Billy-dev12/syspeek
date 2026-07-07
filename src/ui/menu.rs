use dialoguer::{Select, theme::ColorfulTheme};

pub fn jalankan() {
    let menu_utama = vec![
        "📁 /proc — CPU, RAM, Uptime",
        "📁 /sys  — Baterai, Suhu (coming soon)",
        "📁 /proc/net — Jaringan",
        "🚪 Keluar",
    ];

    loop {
        let pilihan = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("🖥️  SYSPEEK — Pilih kategori:")
            .items(&menu_utama)
            .default(0)
            .interact()
            .unwrap();

        match pilihan {
            0 => menu_proc(),
            1 => {
                super::display::tampil_box(
                    "⚠️  Coming Soon",
                    vec!["Fitur /sys belum tersedia.".to_string()],
                );
            }
            2 => {
                menu_jaringan();
            }
            3 => break,
            _ => {}
        }
    }
}

fn menu_proc() {
    use crate::fungsi;

    let options = vec![
        "💻 CPU Usage",
        "💾 RAM",
        "🕐 Uptime",
        "🐧 Versi Kernel",
        "📊 Semua",
        "⬅️  Kembali",
    ];

    loop {
        let pilihan = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📁 /proc — Pilih data:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match pilihan {
            0 => {
                let data = fungsi::proc::cpu::cpu_proses();
                super::display::tampil_box("💻 CPU Usage", vec![data]);
            }
            1 => {
                let data = fungsi::proc::ram::data_ram();
                super::display::tampil_box("💾 RAM", data);
            }
            2 => {
                let data = fungsi::proc::uptime::data_uptime();
                super::display::tampil_box("🕐 Uptime", vec![data]);
            }
            3 => {
                let data = fungsi::proc::version::data_versi();
                super::display::tampil_box("🐧 Versi Kernel", vec![data]);
            }
            4 => {
                let mut all = Vec::new();
                all.push(fungsi::proc::cpu::cpu_proses());
                all.extend(fungsi::proc::ram::data_ram());
                all.push(fungsi::proc::uptime::data_uptime());
                all.push(fungsi::proc::version::data_versi());
                super::display::tampil_box("📊 Semua Data /proc", all);
            }
            5 => break,
            _ => {}
        }
    }
}

fn menu_jaringan() {
    use crate::fungsi;

    let options = vec![
        "📶 Wireless",
        "⬅️  Kembali",
    ];

    loop {
        let pilihan = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("📁 /proc/net — Pilih data:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match pilihan {
            0 => {
                let data_2 = fungsi::proc::wireles::read_wireless();
                super::display::tampil_box("📶 WiFi Info", data_2);
            }
            1 => break,
            _ => {}
        }
    }

}