# SysPeek 🖥️

Tool CLI interaktif untuk memantau sistem Linux, dibuat dengan Rust.

Baca data langsung dari `/proc/` dan `/sys/` di Linux, tampil dalam kotak berwarna dengan menu navigasi pakai arrow key.

## Fitur

| Kategori | Modul | File Sumber | Apa yang ditampilkan |
|----------|-------|-------------|----------------------|
| CPU | `proc/cpu` | `/proc/stat` | Persentase penggunaan CPU |
| RAM | `proc/ram` | `/proc/meminfo` | Total, terpakai, tersisa (GB) |
| Uptime | `proc/uptime` | `/proc/uptime` | Waktu aktif sistem (jam/hari) |
| Versi Kernel | `proc/version` | `/proc/version` | Nama versi kernel |
| WiFi | `proc/wireles` | `/proc/net/wireless` | Interface, sinyal, link quality |
| Baterai | `sys/` | `/sys/class/power_supply/` | *(coming soon)* |
| Suhu | `sys/` | `/sys/class/thermal/` | *(coming soon)* |

## Demo

```
$ cargo run

🖥️  SYSPEEK — Pilih kategori:
> 📁 /proc — CPU, RAM, Uptime
  📁 /sys  — Baterai, Suhu (coming soon)
  📁 /proc/net — Jaringan
  🚪 Keluar
```

Pilih `/proc` → `/proc/net` → `📶 Wireless`:

```
╔══════════════════════════════════════════╗
║              📶 WiFi Info                ║
╠══════════════════════════════════════════╣
║ Interface  : wlan0                       ║
║ Link       : 70%                         ║
║ Sinyal     : -35 dBm (Sangat Kuat 💪)   ║
║ Noise      : -256 dBm                    ║
╚══════════════════════════════════════════╝
```

## Struktur Folder

```
syspeek/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs                     # Entry point → panggil menu
    ├── fungsi.rs                   # pub mod proc; pub mod sys;
    ├── fungsi/
    │   ├── proc.rs                 # Deklarasi modul /proc/
    │   ├── proc/
    │   │   ├── cpu.rs              # Hitung CPU usage
    │   │   ├── ram.rs              # Baca RAM (GB)
    │   │   ├── uptime.rs           # Baca uptime
    │   │   ├── version.rs          # Baca versi kernel
    │   │   ├── wireles.rs          # Baca WiFi info
    │   │   └── partisi.rs          # (placeholder)
    │   └── sys.rs                  # Placeholder untuk /sys/
    ├── ui.rs                       # pub mod display; pub mod menu;
    └── ui/
        ├── display.rs              # Fungsi tampil_box()
        └── menu.rs                 # Menu interaktif (dialoguer)
```

## Cara Pakai

### Prasyarat
- Linux (karena baca file `/proc/` dan `/sys/`)
- Rust & Cargo terinstall

### Build & Run

```bash
# Clone repo
git clone https://github.com/Billy-dev12/syspeek.git
cd syspeek

# Jalankan
cargo run
```

### Navigasi Menu

| Aksi | Tombol |
|------|--------|
| Pilih | `↑` `↓` lalu `Enter` |
| Kembali | Pilih `⬅️ Kembali` |
| Keluar | Pilih `🚪 Keluar` |

## Arsitektur

### Flow Program

```
main()
  └── ui::menu::jalankan()
        ├── Menu Utama
        │   ├── /proc → menu_proc()
        │   │   ├── CPU   → tampil_box()
        │   │   ├── RAM   → tampil_box()
        │   │   ├── Uptime → tampil_box()
        │   │   ├── Versi → tampil_box()
        │   │   └── Semua → tampil_box()
        │   ├── /sys → "Coming Soon"
        │   ├── /proc/net → menu_jaringan()
        │   │   └── WiFi → tampil_box()
        │   └── Keluar → break
        └── ...
```

### Pola Fungsi

Setiap modul data mengembalikan `Vec<String>`, lalu di-render oleh `display::tampil_box()`:

```rust
// fungsi/proc/ram.rs
pub fn data_ram() -> Vec<String> {
    vec![
        format!("Total RAM     : {:.2} GB", total_gb),
        format!("Terpakai      : {:.2} GB", terpakai_gb),
        format!("Tersisa       : {:.2} GB", available_gb),
    ]
}

// ui/menu.rs
let data = fungsi::proc::ram::data_ram();
super::display::tampil_box("💾 RAM", data);
```

## Dependencies

| Crate | Versi | Fungsi |
|-------|-------|--------|
| `colored` | 2 | Warna output terminal |
| `dialoguer` | 0.12 | Menu interaktif (arrow key + select) |

## Konsep Rust yang Dipakai

| Konsep | Contoh Penggunaan |
|--------|-------------------|
| `std::fs::read_to_string()` | Baca isi file `/proc/` |
| `split_whitespace()` | Pecah teks jadi array |
| `parse::<tipe>()` | String → angka |
| `match` | Cek beberapa kondisi |
| `Vec<String>` | Return data dinamis |
| `format!()` | Format string |
| Module system | `mod`, `pub mod` |
| `colored` crate | `.cyan()`, `.bold()`, `.green()` |
| `dialoguer` crate | `Select::with_theme().interact()` |

## Konversi Satuan

| Dari | Ke | Rumus |
|------|----|-------|
| kB | GB | `kB / 1_048_576` |
| dBm | Status | `-50..0` = Kuat, `< -85` = Lemah |
| Detik | Jam | `detik / 3600` |

## Roadmap

- [x] CPU monitoring
- [x] RAM monitoring
- [x] Uptime display
- [x] Versi kernel
- [x] WiFi info
- [x] Menu interaktif
- [x] Box display dengan warna
- [ ] Baterai info (`/sys/class/power_supply/`)
- [ ] Suhu CPU (`/sys/class/thermal/`)
- [ ] CPU frequency (`/sys/devices/system/cpu/`)
- [ ] GitHub Actions auto-release

## License

Projek belajar Rust. Dibuat dengan ❤️ oleh Billy.
