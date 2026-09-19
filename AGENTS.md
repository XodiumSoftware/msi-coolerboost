# MSI CoolerBoost — Claude Code Context

## Project at a Glance

- **Name:** msi-coolerboost
- **Type:** Noctalia v5 bar widget plugin
- **Language:** Luau scripts + `plugin.toml` manifest
- **Shell:** Noctalia on CachyOS + niri
- **Output:** One plugin (`xodium/msi-coolerboost`) installed as a Noctalia plugin source

## APIs & Tools

| Category | Technology | Purpose |
|----------|------------|---------|
| **Shell** | Noctalia v5 | Bar + widget host (plugin API level 3) |
| **Plugin scripts** | Luau | `widget.luau` entry, `noctalia.*` / `barWidget.*` APIs |
| **Compositor** | niri (KDL config) | Keybinds, window management |
| **Notifications** | `noctalia.notify` / `noctalia.notifyError` | In-shell feedback on toggle |
| **Privilege** | sudo | Run `isw` to toggle CoolerBoost |
| **Validation** | `noctalia plugins lint` | Offline manifest/settings cross-check |

## Installation

```bash
# Git source
noctalia msg plugins source add xodium git https://github.com/XodiumSoftware/msi-coolerboost.git
noctalia msg plugins enable xodium/msi-coolerboost

# Or a local path source for development (hot-reloads .luau edits)
noctalia msg plugins source add dev path /mnt/hdd/Projects/msi-coolerboost
noctalia msg plugins enable xodium/msi-coolerboost
```

## Architecture Overview

Niri sources its keybinds from `~/.config/niri/cfg/keybinds.kdl`; the widget is
hosted by Noctalia's own bar.

### Plugin (repo root is a Noctalia plugin source)

| File | Purpose |
|------|---------|
| `msi-coolerboost/plugin.toml` | Plugin manifest: `id = "xodium/msi-coolerboost"`, one `[[widget]]` entry `coolerboost` pointing at `widget.luau` |
| `msi-coolerboost/widget.luau` | Bar widget entry script. Toggles via `sudo isw -b on/off`, reads `/tmp/isw_coolerboost` for state, renders a propeller glyph (error color when ON), and handles the `"toggle"` IPC event via `onIpc` |
| `catalog.toml` | Source-repo catalog so the plugin can be listed and compat-checked from git without a full clone |

An entry is addressed `<author>/<plugin>:<entry>`, so a niri keybind can call:

```bash
noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle
```

The niri keybind uses the same toggle the widget uses; the widget only reflects
the state from the state file.

### State Management

- State file: `/tmp/isw_coolerboost`
- Created with content `"on"` = ON, absent = OFF
- Mirrors `isw` internal state
- Lives in `/tmp` so the state resets with the hardware on reboot; the widget
  re-reads it on every `update()` tick (1 s)

## niri Integration

**Keyboard shortcut (`~/.config/niri/cfg/keybinds.kdl`):**

```kdl
XF86Launch7 allow-when-locked=true hotkey-overlay-title="Toggle CoolerBoost" { spawn-sh "noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle"; }
```

**Autostart:** none needed — Noctalia loads the bar (and enabled plugins when
added to a bar) itself.

**Add the widget to a bar** via Settings → Bar → Add widget, or in
`~/.config/noctalia/config.toml`:

```toml
[widget.coolerboost]
type = "xodium/msi-coolerboost:coolerboost"

[bar.default]
end = ["coolerboost"]
```

## Documentation Guidelines

- **Plugin manifest** — Keep `plugin.toml` and `catalog.toml` valid and in sync
- **Luau entry** — Document non-obvious behavior with comments
- Plugin API reference: <https://docs.noctalia.dev/noctalia/plugins/development/>

## Claude Code Workflow

### Task Management

- Number tasks in name (e.g., "1. Add config file support")
- Ask before committing if changes need review

### After Making Edits

1. **README.md** — Update if usage/installation changes
2. **AGENTS.md** — Update if new tools/APIs added
3. **GUIDE.md** — Update end-user instructions
4. **Lint** — Run `noctalia plugins lint .` before committing

## CI/CD

Implemented via `.github/workflows/`:

| Job | Trigger | Purpose |
|-----|---------|---------|
| **Validate** | PR / push | Validate `plugin.toml` + `catalog.toml` TOML syntax and required keys |

A future improvement could run `noctalia plugins lint` on an Arch-based runner
with the `noctalia` package installed.
