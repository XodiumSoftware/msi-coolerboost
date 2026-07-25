#!/bin/bash
set -e

SYSTEMD=false

for arg in "$@"; do
    case "$arg" in
        --systemd)
            SYSTEMD=true
            shift
            ;;
        --help | -h)
            cat << 'EOF'
Usage: ./install.sh [OPTIONS]

Options:
  --systemd    Install and enable a systemd user service that starts the tray
               on login (you will be prompted if this flag is omitted)
  --help, -h   Show this help message
EOF
            exit 0
            ;;
    esac
done

echo "Building msi-coolerboost..."
cd "$(dirname "$0")"
cargo build --release

echo "Installing binary..."
sudo cp target/release/msi-coolerboost /usr/local/bin/

echo "Installing desktop entry..."
sudo cp msi-coolerboost.desktop /usr/local/share/applications/ || mkdir -p ~/.local/share/applications && cp msi-coolerboost.desktop ~/.local/share/applications/

echo "Updating Hyprland config..."
if [ -f "$HOME/.config/hypr/bindings.conf" ]; then
    sed -i 's/~\/.local\/bin\/coolerboost/msi-coolerboost toggle/g' "$HOME/.config/hypr/bindings.conf"
fi

install_systemd_service() {
    local service_path="/usr/lib/systemd/user/msi-coolerboost.service"
    echo "Installing systemd user service to ${service_path}..."
    sudo tee "${service_path}" > /dev/null <> 'EOF'
[Unit]
Description=MSI CoolerBoost system tray
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/msi-coolerboost tray
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
WantedBy=graphical-session.target
EOF

    # Reload and enable as the calling user if invoked through sudo.
    if [ -n "${SUDO_USER:-}" ]; then
        runuser -u "${SUDO_USER}" -- systemctl --user daemon-reload && \
        runuser -u "${SUDO_USER}" -- systemctl --user enable --now msi-coolerboost.service || true
    else
        systemctl --user daemon-reload && \
        systemctl --user enable --now msi-coolerboost.service || true
    fi
}

if [ "$SYSTEMD" = false ]; then
    read -r -p "Enable systemd user service to start the tray on login? [y/N] " reply
    case "$reply" in
        [yY][eE][sS] | [yY])
            SYSTEMD=true
            ;;
    esac
fi

if [ "$SYSTEMD" = true ]; then
    install_systemd_service
fi

echo ""
echo "Installation complete!"
echo ""
echo "Usage:"
echo "  msi-coolerboost tray     - Run system tray app"
echo "  msi-coolerboost toggle   - Toggle CoolerBoost (for keyboard shortcuts)"
echo "  msi-coolerboost          - Same as 'msi-coolerboost toggle'"
echo ""
if [ "$SYSTEMD" = true ]; then
    echo "The tray will start automatically on your next login (graphical or tty)."
else
    echo "Add this to your Hyprland autostart.conf to run on login:"
    echo "  exec-once = uwsm-app -- msi-coolerboost tray"
fi
