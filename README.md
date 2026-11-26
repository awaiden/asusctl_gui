# ASUS Control (`asusctl_gui`)

A desktop control panel for ASUS laptops built with **Tauri 2**, **SvelteKit (Svelte 5)**, **Tailwind CSS v4**, and **Lucide icons**, driving the `asusctl` CLI.

## Features

- **System Info**: Hardware & supported `asusctl` interfaces (`info --show-supported`)
- **Power Profiles**: Active, AC, and Battery profile switching (`Quiet`, `Balanced`, `Performance`)
- **Keyboard & Aura**: Keyboard backlight brightness, Aura RGB effects, and Aura power zones
- **Battery**: Charge limit slider (20–100%) and one-shot full charge
- **Fan Curves**: Per-profile CPU/GPU/Mid fan curve editor, enable toggles, and reset to default (gracefully hidden/disabled if unsupported)
- **Armoury**: Firmware attributes (`asus-armoury` list, get, and set)
- **Extras**: Screenpad backlight/gamma and Slash LED bar controls

## Requirements

- `asusd` daemon running and `asusctl` installed in `PATH`
- [Bun](https://bun.sh/) and Rust toolchain (`cargo`)
- Linux Tauri system libraries (`webkit2gtk-4.1`, `gtk3`, `dbus-1` development packages)

## Development & Build

```bash
# Install frontend dependencies
bun install

# Run in development mode (launches Vite + Tauri window)
bun run tauri dev

# Type-check and build frontend
bun run check
bun run build

# Build production Tauri bundle
bun run tauri build

# Run Rust backend unit tests
cd src-tauri && cargo test
```
