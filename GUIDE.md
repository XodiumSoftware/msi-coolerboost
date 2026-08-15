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
- Omarchy 4 (Quattro)
- Passwordless sudo for `isw -b on` and `isw -b off`

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

## Installation

Install the plugin straight from git:

```bash
omarchy plugin add https://github.com/XodiumSoftware/msi-coolerboost.git --enable
```

Place it in the desired bar section if `--enable` did not pick one:

```bash
omarchy plugin enable xodium.msi-coolerboost --section right
```

No autostart entry is needed; the bar widget is loaded by `omarchy-shell`.

---

## Configuration

### Hyprland Keyboard Shortcut

On Omarchy 4 / Hyprland 0.56, add to `~/.config/hypr/bindings.lua`:

```lua
o.bind("XF86Launch7", "Toggle CoolerBoost", 'bash -c \'if [ -f /tmp/isw_coolerboost ]; then sudo isw -b off && rm -f /tmp/isw_coolerboost && notify-send -u low CoolerBoost OFF "Fan boost disabled"; else sudo isw -b on && echo on > /tmp/isw_coolerboost && notify-send -u low CoolerBoost ON "Fan boost enabled"; fi\'')
```

Reload Hyprland config:

```bash
hyprctl reload
```

---

## Usage

### Omarchy 4 Bar Widget

The fan icon appears in the bar section you chose (right by default).

- **Left click:** toggle CoolerBoost
- **Right click:** refresh the configured shortcut tooltip

### Keyboard Shortcut

Press the key you bound to `omarchy-shell xodium.msi-coolerboost toggle`.

---

## Troubleshooting

### "Failed to toggle"

Check sudoers is configured:

```bash
sudo -l | grep isw
```

Should show:
```
(ALL) NOPASSWD: /usr/bin/isw -b on, /usr/bin/isw -b off
```

### Bar widget not appearing

Confirm the plugin is installed and enabled:

```bash
omarchy plugin list
omarchy plugin enable xodium.msi-coolerboost --section right
```

If the shell was already running, rescan plugins:

```bash
omarchy-shell shell rescanPlugins
```

### "isw: command not found"

Install and configure `isw` for your specific MSI laptop model.
