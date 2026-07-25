# MSI CoolerBoost — Claude Code Context

## Project at a Glance

- **Name:** msi-coolerboost
- **Type:** System tray application
- **Language:** Rust
- **Build Tool:** Cargo
- **GUI Framework:** ksni
- **Output:** One binary (`msi-coolerboost`) with `tray` and `toggle` subcommands

## APIs & Tools

| Category | Technology | Purpose |
|----------|------------|---------|
| **GUI** | ksni | System tray icon and menu |
| **Config Paths** | dirs | XDG directories |
| **Notifications** | notify-rust | Desktop notifications |

## Quick Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run system tray
cargo run -- tray

# Run toggle (CLI)
cargo run -- toggle
# or simply:
cargo run

# Install with systemd autostart
./install.sh --systemd
```

## Architecture Overview

### Binaries

- **`msi-coolerboost tray`** (`src/tray_mode.rs`) — System tray GUI
- **`msi-coolerboost toggle`** (`src/toggle_mode.rs` and `src/main.rs`) — CLI toggle command

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
bindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost toggle
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

Implemented via `.github/workflows/rust.yml`:

| Job | Trigger | Purpose |
|-----|---------|---------|
| **Lint** | PR / push | `cargo clippy` + `cargo fmt` |
| **Test** | PR / push | `cargo test` |
| **Build** | PR / push / release | Release binary `msi-coolerboost` |
| **Nightly Release** | Push to `main` | Updates `nightly` tag with latest binaries |
| **Stable Release** | Release published | Attaches binaries to the GitHub release |
| **Docs** | Push to `main` | Builds and deploys rustdoc to GitHub Pages |
| **AUR Publish** | Release / manual | Updates AUR `PKGBUILD` and `.SRCINFO` |

Builds use `sccache` with the GitHub Actions cache backend. The release binary name matches the package name in `PKGBUILD` (`msi-coolerboost`).
