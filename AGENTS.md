# MSI CoolerBoost — Claude Code Context

## Project at a Glance

- **Name:** msi-coolerboost
- **Type:** System tray application
- **Language:** Rust
- **Build Tool:** Cargo
- **GUI Framework:** tray-icon + winit
- **Output:** Two binaries (tray GUI + CLI toggle)

## APIs & Tools

| Category | Technology | Purpose |
|----------|------------|---------|
| **GUI** | tray-icon | System tray icon and menu |
| **Windowing** | winit | Event loop and window management |
| **Notifications** | notify-rust | Desktop notifications |
| **Image** | image | Icon generation (RGBA) |
| **Config Parsing** | regex | Hyprland config parsing |

## Quick Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run system tray
cargo run --bin tray

# Run toggle (CLI)
cargo run --bin toggle
```

## Architecture Overview

### Binaries

- **`msi-coolerboost tray`** (`src/bin/tray.rs`) — System tray GUI
- **`msi-coolerboost-toggle`** (`src/bin/toggle.rs`) — CLI toggle command

### Library (`src/lib.rs`)

Core functionality shared between binaries:

| Function | Purpose |
|----------|---------|
| `check_status()` | Check CoolerBoost state via file |
| `toggle()` | Toggle via `isw`, show notification |
| `get_current_shortcut()` | Parse Hyprland config |
| `show_notification()` | Desktop notifications |
| `create_icon_rgba()` | Generate status icon |

### State Management

- State file: `/tmp/isw_coolerboost`
- Created = ON, absent = OFF
- Mirrors `isw` internal state

### Icon Colors

- **ON:** Green (#4CAF50)
- **OFF:** Gray (#757575)

## Hyprland Integration

**Keyboard shortcut (bindings.conf):**
```conf
bindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost-toggle
```

**Autostart (autostart.conf):**
```conf
exec-once = uwsm-app -- msi-coolerboost tray
```

## Key Conventions

- `unsafe_code` is forbidden
- All Clippy warnings enabled
- Release profile: LTO + strip symbols
- State stored in tmpfs (cleared on reboot)

## Documentation Guidelines

- **Public APIs** — Document with `///`
- **Binaries** — Explain purpose in module docs

## Claude Code Workflow

### Task Management

- Number tasks in name (e.g., "1. Add config file support")
- Ask before committing if changes need review

### After Making Edits

1. **README.md** — Update if usage/installation changes
2. **AGENTS.md** — Update if new tools/APIs added

## CI/CD

No CI/CD currently. Consider:
- GitHub Actions for builds on PR/push
- Release workflow for publishing binaries
