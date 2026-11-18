#!/usr/bin/env fish
set -e

# 1. Build release binary
cargo build --release

# 2. Install binary to user bin
mkdir -p ~/.local/bin
cp target/release/apex_clicker ~/.local/bin/apex_clicker

# 3. Automatically create the desktop entry & menu icon (Fish-compatible)
mkdir -p ~/.local/share/applications
echo "[Desktop Entry]
Name=Apex Auto Clicker
Comment=Fast and lightweight auto clicker
Exec=$HOME/.local/bin/apex_clicker
Icon=$HOME/apex_clicker/src/apex_clicker.png
Terminal=false
Type=Application
Categories=Utility;" > ~/.local/share/applications/apex_clicker.desktop

chmod +x ~/.local/share/applications/apex_clicker.desktop
update-desktop-database ~/.local/share/applications/ 2>/dev/null

echo "Apex Auto Clicker successfully installed and registered as an app!"
