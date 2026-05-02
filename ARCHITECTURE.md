# ARCHITECTURE.md

This file provides guidance when working with code in this repository.

## Project Overview

MSI CoolerBoost is a Rust system tray application for controlling MSI laptop fan boost via `isw`. It provides a visual indicator of fan status and allows quick toggling through the system tray or keyboard shortcuts.

## Build & Run Commands

```bash
# Build (debug)
cargo build

# Build (release, with LTO + strip)
cargo build --release

# Run system tray
cargo run --bin tray

# Run toggle (CLI)
cargo run --bin toggle
```

## Architecture

### Binaries

The project produces two binaries from the same codebase:

- **`msi-coolerboost`** (`src/bin/tray.rs`) — System tray GUI application
- **`msi-coolerboost-toggle`** (`src/bin/toggle.rs`) — CLI toggle command (symlink)

### Entry Points

#### Tray Binary (`src/bin/tray.rs`)

The system tray application:
- Creates a system tray icon using `tray-icon` crate
- Shows current CoolerBoost status (green=ON, gray=OFF)
- Provides context menu with toggle, status, and quit options
- Handles menu events to toggle state or quit

#### Toggle Binary (`src/bin/toggle.rs`)

Simple CLI wrapper:
- Calls `msi_coolerboost::toggle()` from the library
- Exits immediately after toggling
- Designed for keyboard shortcuts and scripts

### Library (`src/lib.rs`)

Shared functionality between binaries:

| Function | Purpose |
|----------|---------|
| `check_status()` | Check if CoolerBoost is enabled via state file |
| `toggle()` | Toggle CoolerBoost via `isw` command, show notification |
| `get_current_shortcut()` | Parse current keyboard shortcut from Hyprland config |
| `set_shortcut()` | Update Hyprland config with new shortcut |
| `show_notification()` | Display desktop notification via `notify-rust` |
| `create_icon_rgba()` | Generate status icon (green/gray circle) |

### Project Structure

```
src/
├── lib.rs              # Shared library functions
└── bin/
    ├── tray.rs         # System tray GUI application
    └── toggle.rs       # CLI toggle command
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tray-icon` | System tray icon and menu |
| `winit` | Event loop and windowing |
| `notify-rust` | Desktop notifications |
| `regex` | Parsing Hyprland config |
| `image` | Generating status icons |

### Key Conventions

- No unsafe code (`#![forbid(unsafe_code)]`)
- All Clippy warnings enabled
- Release profile enables LTO and strips symbols
- State stored in `/tmp/isw_coolerboost` (tmpfs, cleared on reboot)

## Hyprland Integration

### Keyboard Shortcuts

The library reads/writes `~/.config/hypr/bindings.conf`:

```conf
# CoolerBoost Fan Toggle
bindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost-toggle
```

### Autostart

Add to `~/.config/hypr/autostart.conf`:

```conf
exec-once = uwsm-app -- msi-coolerboost tray
```

## Icon Generation

Status icons are generated programmatically:
- Size: 64x64 pixels
- ON: Green circle (#4CAF50)
- OFF: Gray circle (#757575)
- Anti-aliased edge with alpha gradient

## State Management

CoolerBoost state is tracked via a file:
- Path: `/tmp/isw_coolerboost`
- Created when ON, removed when OFF
- Mirrors `isw` internal state

## Notifications

Desktop notifications shown on toggle:
- Title: "CoolerBoost ON/OFF"
- Body: "Fan boost enabled/disabled"
- Duration: 2 seconds
- Uses `notify-rust` crate

## Requirements

- `isw` installed and configured for your MSI laptop
- Sudo access for `isw -b on/off` commands
- Hyprland (for keyboard shortcuts)
- System tray support (status bar/panel)
