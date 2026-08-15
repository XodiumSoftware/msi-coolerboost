# MSI CoolerBoost — Claude Code Context

## Project at a Glance

- **Name:** msi-coolerboost
- **Type:** Omarchy 4 bar widget plugin
- **Language:** Quickshell QML
- **GUI Framework:** Quickshell
- **Output:** One plugin (`xodium.msi-coolerboost`) installed via `omarchy plugin add`

## APIs & Tools

| Category | Technology | Purpose |
|----------|------------|---------|
| **GUI** | QtQuick + Quickshell | Omarchy 4 bar widget |
| **Notifications** | notify-send | Desktop notifications |
| **Privilege** | sudo | Run `isw` to toggle CoolerBoost |

## Installation

```bash
omarchy plugin add https://github.com/XodiumSoftware/msi-coolerboost.git --enable
```

## Architecture Overview

### Plugin (repo root)

| File | Purpose |
|------|---------|
| `manifest.json` | Quickshell plugin manifest (`id: xodium.msi-coolerboost`) |
| `BarWidget.qml` | Self-contained bar widget using only `QtQuick`, `Quickshell`, and `Quickshell.Io`. Toggles via `sudo isw -b on/off` and reads `/tmp/isw_coolerboost` for state. |

The Hyprland keybinding uses the same shell toggle the widget uses; the
widget only reads the shortcut for display.

The plugin exposes an IPC target `xodium.msi-coolerboost` with a `toggle()`
method, so a Hyprland keyboard shortcut can call:

```bash
omarchy-shell xodium.msi-coolerboost toggle
```

### State Management

- State file: `/tmp/isw_coolerboost`
- Created with content `"on"` = ON, absent = OFF
- Mirrors `isw` internal state

## Hyprland Integration

**Keyboard shortcut (bindings.lua for Omarchy 4 / Hyprland 0.56):**
```lua
o.bind("SUPER CTRL, F", "Toggle CoolerBoost", "omarchy-shell xodium.msi-coolerboost toggle")
```

**Omarchy 4 autostart:** the bar widget is loaded by `omarchy-shell`; no
autostart entry is needed. Enable it with:
```bash
omarchy plugin enable xodium.msi-coolerboost --section right
```

## Documentation Guidelines

- **Plugin manifest** — Keep `manifest.json` valid and up to date
- **Bar widget** — Document non-obvious QML behavior with comments

## Claude Code Workflow

### Task Management

- Number tasks in name (e.g., "1. Add config file support")
- Ask before committing if changes need review

### After Making Edits

1. **README.md** — Update if usage/installation changes
2. **AGENTS.md** — Update if new tools/APIs added
3. **GUIDE.md** — Update end-user instructions

## CI/CD

Implemented via `.github/workflows/ci.yml`:

| Job | Trigger | Purpose |
|-----|---------|---------|
| **Validate** | PR / push | Validate `manifest.json` with `jq` |

A future improvement could run `omarchy-plugin-validate` on an Omarchy runner
and lint the QML with Quickshell tooling.
