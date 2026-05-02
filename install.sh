#!/bin/bash
set -e

echo "Building msi-coolerboost..."
cd "$(dirname "$0")"
cargo build --release

echo "Installing binaries..."
sudo cp target/release/tray /usr/local/bin/msi-coolerboost
cd /usr/local/bin
sudo ln -sf msi-coolerboost msi-coolerboost-toggle

echo "Installing desktop entry..."
sudo cp msi-coolerboost.desktop /usr/local/share/applications/ || mkdir -p ~/.local/share/applications && cp msi-coolerboost.desktop ~/.local/share/applications/

echo "Updating Hyprland config..."
# Update the binding to use the new binary name
if [ -f "$HOME/.config/hypr/bindings.conf" ]; then
    sed -i 's/~\/.local\/bin\/coolerboost/msi-coolerboost-toggle/g' "$HOME/.config/hypr/bindings.conf"
fi

echo ""
echo "Installation complete!"
echo ""
echo "Usage:"
echo "  msi-coolerboost tray    - Run system tray app"
echo "  msi-coolerboost-toggle  - Toggle CoolerBoost (for keyboard shortcuts)"
echo ""
echo "Add this to your Hyprland autostart.conf to run on login:"
echo "  exec-once = uwsm-app -- msi-coolerboost tray"
