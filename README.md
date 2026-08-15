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

<h4 align="center">Omarchy 4 bar widget for MSI laptop fan boost</h4><br />

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

- Omarchy 4 bar widget for CoolerBoost status and toggle
- Desktop notifications on toggle
- Keyboard shortcut support through the Omarchy shell IPC
- No extra binary or CLI to install

## Installation

Install the plugin straight from git:

```bash
omarchy plugin add https://github.com/XodiumSoftware/msi-coolerboost.git --enable
```

The widget shows a fan icon that uses the theme's active/urgent color when
CoolerBoost is on.

- **Left click:** toggle CoolerBoost
- **Right click:** refresh the configured shortcut tooltip

A keyboard shortcut can run the same toggle the widget uses:

```lua
o.bind("XF86Launch7", "Toggle CoolerBoost", 'bash -c \'if [ -f /tmp/isw_coolerboost ]; then sudo isw -b off && rm -f /tmp/isw_coolerboost && notify-send -u low CoolerBoost OFF "Fan boost disabled"; else sudo isw -b on && echo on > /tmp/isw_coolerboost && notify-send -u low CoolerBoost ON "Fan boost enabled"; fi\'')
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
