# Installation Guide

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

- `isw` installed and configured for your MSI laptop
- CachyOS with niri and Noctalia v5
- Passwordless sudo for `isw -b on` and `isw -b off`

### Install `isw`

```bash
# Arch / CachyOS (AUR)
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

## Installation

### From git

Add this repo as a plugin source and enable the plugin:

```bash
noctalia msg plugins source add xodium git https://github.com/XodiumSoftware/msi-coolerboost.git
noctalia msg plugins enable xodium/msi-coolerboost
```

### From a local checkout (development)

Point a path source at the repo; edits to `widget.luau` hot-reload:

```bash
noctalia msg plugins source add dev path /mnt/hdd/Projects/msi-coolerboost
noctalia msg plugins enable xodium/msi-coolerboost
```

Alternatively, symlink the plugin directory into the state-dir drop-in:

```bash
mkdir -p ~/.local/share/noctalia/plugins
ln -s /mnt/hdd/Projects/msi-coolerboost/msi-coolerboost ~/.local/share/noctalia/plugins/msi-coolerboost
noctalia msg plugins enable xodium/msi-coolerboost
```

### Add the widget to a bar

Either via **Settings → Bar → Add widget** (pick
`xodium/msi-coolerboost:coolerboost`), or by hand in
`~/.config/noctalia/config.toml`:

```toml
[widget.coolerboost]
type = "xodium/msi-coolerboost:coolerboost"

[bar.default]
end = ["coolerboost"]
```

Verify the plugin is enabled:

```bash
noctalia msg plugins list
```

---

## Configuration

### niri Keyboard Shortcut

Add to `~/.config/niri/cfg/keybinds.kdl` inside the `binds` block:

```kdl
XF86Launch7 allow-when-locked=true hotkey-overlay-title="Toggle CoolerBoost" { spawn-sh "noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle"; }
```

niri hot-reloads its config automatically.

---

## Usage

### Noctalia Bar Widget

The propeller icon appears in the bar section you chose. It uses the palette's
error color while CoolerBoost is on.

- **Left click:** toggle CoolerBoost
- **Tooltip:** shows `MSI CoolerBoost: ON/OFF`

### Keyboard Shortcut

Press the key you bound (`XF86Launch7` is the MSI CoolerBoost button).
The keybind dispatches a `toggle` event to every live widget instance, so it
works from anywhere while Noctalia is running.

---

## Troubleshooting

### "CoolerBoost failed to toggle"

Check sudoers is configured:

```bash
sudo -l | grep isw
```

Should show:

```
(ALL) NOPASSWD: /usr/bin/isw -b on, /usr/bin/isw -b off
```

Test the toggle directly:

```bash
noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle
```

### Bar widget not appearing

Confirm the plugin is enabled, then check the widget was added to the bar
(Settings → Bar, or `[widget.coolerboost]` + `[bar.default]` in `config.toml`):

```bash
noctalia msg plugins list
```

Validate the plugin structure offline:

```bash
noctalia plugins lint /mnt/hdd/Projects/msi-coolerboost
```

### "isw: command not found"

Install and configure `isw` for your specific MSI laptop model.
