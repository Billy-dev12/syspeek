# Changelog

## [1.0.2] - 2026-07-07

### Added
- Subcommand CLI (`syspeek <command>`)
- Loading bar untuk CPU & RAM
- Flag `--watch` untuk auto-refresh setiap 2 detik
- Validasi command (invalid command langsung error)
- Module `src/ui/display.rs` untuk display functions

### Commands
- `cpu` — CPU usage percentage
- `ram` — RAM info (total/terpakai/tersisa)
- `uptime` — System uptime
- `version` — Kernel version
- `wireless` — WiFi signal info
- `all` — Semua data sekaligus
- `help` — Tampilkan bantuan

### Removed
- TUI menu interaktif (dialoguer)
- Box display format
- Module `src/ui/menu.rs`

### Changed
- Output plain text, tanpa box
- Pisah UI logic ke `src/ui/display.rs`

---

## [1.0.1] - 2026-07-07

### Added
- Wireless module (`/proc/net/wireless`)
- Partition module placeholder

---

## [1.0.0] - 2026-07-07

### Added
- Initial release
- CPU monitoring (`/proc/stat`)
- RAM monitoring (`/proc/meminfo`)
- Uptime monitoring (`/proc/uptime`)
- Kernel version (`/proc/version`)
- Interactive menu with dialoguer
- Colored box display
