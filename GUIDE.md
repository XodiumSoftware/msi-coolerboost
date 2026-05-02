# Installation Guide

## Table of Contents

- [Prerequisites](#prerequisites)
- [Build from Source](#build-from-source)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)

---

## Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- `isw` installed and configured for your MSI laptop
- Hyprland window manager
- System tray support in your status bar

### Install `isw`

```bash
# Arch Linux (AUR)
yay -S isw

# Configure for your laptop model
sudo isw -w
```

### Sudoers Setup

Add passwordless sudo for `isw`:

```bash
echo 'illyrius ALL=(ALL) NOPASSWD: /usr/bin/isw -b on, /usr/bin/isw -b off' | sudo tee /etc/sudoers.d/coolerboost
```

---

## Build from Source

```bash
# Clone the repository
git clone https://github.com/XodiumSoftware/msi-coolerboost.git
cd msi-coolerboost

# Build release binaries
cargo build --release
```

---

## Installation

### Manual Installation

```bash
# Copy binaries
cargo build --release
sudo cp target/release/tray /usr/local/bin/msi-coolerboost
sudo ln -sf msi-coolerboost /usr/local/bin/msi-coolerboost-toggle

# Copy desktop entry
sudo cp msi-coolerboost.desktop /usr/local/share/applications/
```

### Install Script

```bash
./install.sh
```

---

## Configuration

### Hyprland Keyboard Shortcut

Add to `~/.config/hypr/bindings.conf`:

```conf
# CoolerBoost Fan Toggle
bindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost-toggle
```

### Autostart

Add to `~/.config/hypr/autostart.conf`:

```conf
exec-once = uwsm-app -- msi-coolerboost tray
```

Reload Hyprland config:

```bash
hyprctl reload
```

---

## Usage

### System Tray

Run the tray application:

```bash
msi-coolerboost tray
```

**Features:**
- Click icon or select "Toggle CoolerBoost" to toggle
- Green icon = ON, Gray icon = OFF
- Shows notification on toggle
- Right-click for menu options

### Keyboard Shortcut

Press `Super + Ctrl + F` to toggle CoolerBoost instantly.

### Command Line

```bash
# Toggle via CLI
msi-coolerboost-toggle

# Check current status
ls /tmp/isw_coolerboost 2>/dev/null && echo "ON" || echo "OFF"
```

---

## Troubleshooting

### "Command not found"

Ensure `/usr/local/bin` is in your `$PATH`:

```bash
export PATH="$PATH:/usr/local/bin"
```

### "Failed to toggle"

Check sudoers is configured:

```bash
sudo -l | grep isw
```

Should show:
```
(ALL) NOPASSWD: /usr/bin/isw -b on, /usr/bin/isw -b off
```

### Tray icon not appearing

Ensure your status bar supports system tray icons (Waybar, etc.).

### "isw: command not found"

Install and configure `isw` for your specific MSI laptop model.
