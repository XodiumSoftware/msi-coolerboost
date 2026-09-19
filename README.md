<div id="readme-top"></div>

<h1 align="center">
  <br />
    <a href="https://xodium.org/">
        <img src="logo.svg" alt="IllyriaBridge Logo" width="200">
    </a>
  <br /><br />
  MSI CoolerBoost
  <br />
  <br />
</h1>

<h4 align="center">Noctalia bar widget for MSI laptop fan boost</h4><br />

<div align="center">

[![Contributors][contributors_shield_url]][contributors_url]
[![Issues][issues_shield_url]][issues_url]
[![License][license_shield_url]][license_url]
</div>

## Table of Contents

- [Guide](GUIDE.md)
- [Features](#features)
- [Code of Conduct][code_of_conduct_url]
- [Contributing][contributing_url]
- [License][license_url]
- [Security][security_url]

## Features

- Noctalia (v5) bar widget showing CoolerBoost status with click-to-toggle
- Desktop notifications on toggle
- Keyboard shortcut support through Noctalia's plugin IPC
- Installable from a git source, a local path source, or the state-dir drop-in

## Installation

Add this repo as a plugin source and enable the plugin:

```bash
noctalia msg plugins source add xodium git https://github.com/XodiumSoftware/msi-coolerboost.git
noctalia msg plugins enable xodium/msi-coolerboost
```

Or, for a local checkout (hot-reloads edits, ideal for development):

```bash
noctalia msg plugins source add dev path /path/to/msi-coolerboost
noctalia msg plugins enable xodium/msi-coolerboost
```

The widget then shows up in **Settings → Bar → Add widget** as
`xodium/msi-coolerboost:coolerboost`, or add it to a bar by hand:

```toml
[widget.coolerboost]
type = "xodium/msi-coolerboost:coolerboost"

[bar.default]
end = ["coolerboost"]
```

The propeller glyph uses the palette's error color when CoolerBoost is on and
the normal text color when off.

- **Left click:** toggle CoolerBoost

A niri keybind can run the same toggle the widget uses
(`~/.config/niri/cfg/keybinds.kdl`):

```kdl
XF86Launch7 allow-when-locked=true hotkey-overlay-title="Toggle CoolerBoost" { spawn-sh "noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle"; }
```

<p align="right"><a href="#readme-top">▲</a></p>

[code_of_conduct_url]: https://github.com/XodiumSoftware/msi-coolerboost?tab=coc-ov-file

[contributing_url]: https://github.com/XodiumSoftware/msi-coolerboost?tab=contributing-ov-file

[contributors_shield_url]: https://img.shields.io/github/contributors/XodiumSoftware/msi-coolerboost?style=for-the-badge&color=blue

[contributors_url]: https://github.com/XodiumSoftware/msi-coolerboost/graphs/contributors

[issues_shield_url]: https://img.shields.io/github/issues/XodiumSoftware/msi-coolerboost?style=for-the-badge&color=yellow

[issues_url]: https://github.com/XodiumSoftware/msi-coolerboost/issues

[license_shield_url]: https://img.shields.io/github/license/XodiumSoftware/msi-coolerboost?style=for-the-badge&color=green

[license_url]: https://github.com/XodiumSoftware/msi-coolerboost?tab=AGPL-3.0-1-ov-file

[security_url]: https://github.com/XodiumSoftware/msi-coolerboost?tab=security-ov-file
