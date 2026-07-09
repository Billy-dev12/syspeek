# SysPeek

Tool CLI sederhana untuk memantau sistem Linux, dibuat dengan Rust.

Baca data langsung dari `/proc/` dan `/sys/` di Linux.

## Fitur

```
✅ CPU Usage
✅ RAM Usage
✅ Uptime
✅ Kernel Version
✅ Battery Information
✅ WiFi Information
```

## Coming Soon

```
⬜ Disk Usage
⬜ CPU Temperature
⬜ Network Traffic
⬜ Processes
⬜ Watch Mode
```

## Cara Pakai

```bash
# Clone
git clone https://github.com/Billy-dev12/syspeek.git
cd syspeek

# Build
cargo build --release

# Jalankan
./target/release/syspeek cpu
./target/release/syspeek all --watch
```

## Commands

| Command | Description |
|---------|-------------|
| `cpu` | CPU usage percentage |
| `ram` | RAM information |
| `uptime` | System uptime |
| `version` | Kernel version |
| `wireless` | WiFi signal info |
| `battery` | Battery information |
| `all` | Semua data (side-by-side) |
| `help` | Tampilkan bantuan |

## License

Projek belajar Rust. Dibuat oleh Billy.
