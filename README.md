# sysrat 🐀

A fast, intuitive, and modern TUI (Terminal User Interface) management tool for systemd services, powered by Rust, `zbus`, and `ratatui`.

`sysrat` allows you to create, monitor, control (start, stop, restart), and destroy systemd processes natively and safely, without calling external binaries.

## 🚀 Features

- **Native D-Bus Integration:** Speaks directly to systemd via `zbus` and the official D-Bus interface—no hacky `systemctl` shell spawning.
- **Beautiful TUI:** A responsive, lightweight, keyboard-driven dashboard built with `ratatui`.
- **Full Lifecycle Management:** 
  - **Monitor:** Real-time unit state tracking.
  - **Control:** Instantly `start`, `stop`, or `restart` system services.
  - **Forge & Destroy:** Interactively generate, enable, and completely wipe out custom `.service` unit configurations.
- **Blazing Fast & Safe:** Written in 100% pure Rust, ensuring memory safety and absolute efficiency.

## 🛠️ Tech Stack

- **Core Logic:** Rust (Asynchronous with Tokio)
- **systemd Interaction:** Native D-Bus bindings via `zbus_systemd`
- **Interface:** `ratatui` (TUI)

## 📦 Installation & Usage

*(Coming soon / Work in progress)*

```toml
# Add sysrat to your system
cargo install sysrat
