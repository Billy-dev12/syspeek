# SysPeek

Program sederhana untuk memantau sistem Linux menggunakan Rust. Program ini membaca data langsung dari file `/proc/` di Linux untuk menampilkan informasi CPU, RAM, dan Uptime.

## Fitur

| Fungsi | File Sumber | Apa yang dilakukan |
|--------|-------------|---------------------|
| CPU | `/proc/stat` | Menghitung persentase penggunaan CPU |
| RAM | `/proc/meminfo` | Menampilkan total, terpakai, dan tersisa RAM (dalam GB) |
| Uptime | `/proc/uptime` | Menampilkan waktu aktif sistem (menit, jam, hari) |

## Struktur Folder

```
syspeek/
├── Cargo.toml
└── src/
    ├── main.rs              # File utama, memanggil semua fungsi
    ├── fungsi.rs            # Modul penghubung ke semua sub-modul
    └── fungsi/
        ├── cpu.rs           # Logika perhitungan CPU
        ├── ram.rs           # Logika pembacaan RAM
        └── uptime.rs        # Logika pembacaan Uptime
```

## Cara Menjalankan

### Prasyarat
- Linux (karena membaca file `/proc/`)
- Rust & Cargo sudah terinstall

### Build & Jalankan

```bash
# Build project
cargo build

# Jalankan langsung
cargo run
```

### Contoh Output

```
╔════════════════════════════════╗
║          🖥️  SYSPEEK           ║
╠════════════════════════════════╣
║ CPU Usage     : 15.19%         ║
║ Total RAM     : 9.63 GB        ║
║ Terpakai      : 5.43 GB        ║
║ Tersisa       : 4.21 GB        ║
║ Uptime        : 0.53 jam       ║
╚════════════════════════════════╝
```

## Penjelasan Kode

### `main.rs`
File入口 (entry point). Memanggil ketiga fungsi secara berurutan:
```rust
fungsi::cpu::cpu_proses();
fungsi::ram::data_ram();
fungsi::uptime::data_uptime();
```

### `fungsi.rs`
Modul penghubung yang mendeklarasikan semua sub-modul:
```rust
pub mod cpu;
pub mod ram;
pub mod uptime;
```

### `fungsi/cpu.rs`
Cara kerja:
1. Baca `/proc/stat` → ambil data CPU pertama (user, nice, system, idle)
2. Tidur 200ms
3. Baca lagi → ambil data CPU kedua
4. Hitung selisih antara data pertama dan kedua
5. Rumus: `(1 - (selisih_idle / selisih_total)) × 100`

### `fungsi/ram.rs`
Cara kerja:
1. Baca `/proc/meminfo`
2. Loop tiap baris, cari baris `MemTotal:` dan `MemAvailable:`
3. Ambil angkanya (satuan kB), konversi ke GB (bagi 1.048.576)
4. Hitung terpakai: `total - available`
5. Hitung persentase: `(terpakai / total) × 100`

### `fungsi/uptime.rs`
Cara kerja:
1. Baca `/proc/uptime` → ambil angka pertama (detik)
2. Konversi ke menit, jam, dan hari

## Konversi Satuan

| Dari | Ke | Rumus |
|------|----|-------|
| kB | GB | `kB / 1_048_576` (atau `/ 1024 / 1024`) |
| kB | MB | `kB / 1_024` |
| Detik | Menit | `detik / 60` |
| Menit | Jam | `menit / 60` |
| Jam | Hari | `jam / 24` |

## Belajar Dari Project Ini

Konsep Rust yang dipakai:
- **`std::fs::read_to_string`** — membaca isi file
- **`split_whitespace()`** — memecah teks jadi array per kata
- **`parse::<tipe>()`** — mengubah string jadi angka
- **`match`** — mengecek beberapa kondisi sekaligus (mirip switch-case)
- **`if` / `continue`** — logika percabangan
- **`for` loop** — mengulang tiap baris
- **`f64` / `as f64`** — bilangan desimal & type casting

## License

Projek belajar Rust.
