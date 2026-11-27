#!/usr/bin/env bash
set -euo pipefail

REPO="awaiden/asusctl_gui"
FORCE_APPIMAGE=false

for arg in "$@"; do
  case "$arg" in
    --appimage) FORCE_APPIMAGE=true ;;
    *)
      echo "Unknown option: $arg" >&2
      exit 1
      ;;
  esac
done

command -v curl >/dev/null 2>&1 || {
  echo "Error: curl is required." >&2
  exit 1
}

ARCH="$(uname -m)"
if [[ "$ARCH" != "x86_64" && "$ARCH" != "amd64" ]]; then
  echo "Error: unsupported architecture '$ARCH' (only x86_64/amd64 is supported)." >&2
  exit 1
fi

if [[ -n "${VERSION:-}" ]]; then
  API_URL="https://api.github.com/repos/${REPO}/releases/tags/${VERSION}"
else
  API_URL="https://api.github.com/repos/${REPO}/releases/latest"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

RELEASE_JSON="$(curl -fsSL "$API_URL")"

find_asset() {
  local ext="$1"
  printf '%s\n' "$RELEASE_JSON" \
    | grep -Eo '"browser_download_url":[[:space:]]*"[^"]+"' \
    | sed -E 's/.*"([^"]+)"$/\1/' \
    | grep -E "\.${ext}$" \
    | head -n 1 || true
}

install_desktop_entry() {
  local exec_path="$1"
  local data_dir="${XDG_DATA_HOME:-$HOME/.local/share}"
  local apps_dir="${data_dir}/applications"
  local icon_dir="${data_dir}/icons/hicolor/512x512/apps"
  local icon_path="${icon_dir}/asusctl_gui.png"
  local desktop_file="${apps_dir}/com.awaiden.asusctl-gui.desktop"
  local ref="${VERSION:-main}"
  local icon_value="asusctl_gui"

  mkdir -p "$apps_dir" "$icon_dir"

  if curl -fsSL -o "$icon_path" "https://raw.githubusercontent.com/${REPO}/${ref}/src-tauri/icons/icon.png" \
    || { [[ "$ref" != "main" ]] && curl -fsSL -o "$icon_path" "https://raw.githubusercontent.com/${REPO}/main/src-tauri/icons/icon.png"; }; then
    if [[ -s "$icon_path" ]]; then
      icon_value="$icon_path"
    fi
  fi

  cat > "$desktop_file" <<EOF
[Desktop Entry]
Type=Application
Name=ASUS Control
GenericName=ASUS Hardware Control
Comment=Desktop control panel for ASUS Linux laptops
Exec=${exec_path}
Icon=${icon_value}
Terminal=false
Categories=System;Settings;HardwareSettings;Utility;
Keywords=asus;asusctl;rog;tuf;fan;battery;aura;rgb;power;
StartupWMClass=asusctl_gui
StartupNotify=true
EOF

  command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$apps_dir" 2>/dev/null || true
  command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -f -t "${data_dir}/icons/hicolor" 2>/dev/null || true
  echo "Created desktop entry at $desktop_file"
}

install_pkg() {
  local ext="$1"
  local url
  url="$(find_asset "$ext")"
  if [[ -z "$url" ]]; then
    return 1
  fi
  local file="${TMP_DIR}/$(basename "$url")"
  echo "Downloading $(basename "$url")..."
  curl -fL --progress-bar -o "$file" "$url"

  case "$ext" in
    deb)
      if command -v apt-get >/dev/null 2>&1; then
        sudo apt-get install -y "$file"
      else
        sudo dpkg -i "$file"
      fi
      ;;
    rpm)
      if command -v dnf >/dev/null 2>&1; then
        sudo dnf install -y "$file"
      else
        sudo rpm -Uvh "$file"
      fi
      ;;
    AppImage)
      local dest_dir="${INSTALL_DIR:-$HOME/.local/bin}"
      mkdir -p "$dest_dir"
      local dest="${dest_dir}/asusctl_gui"
      mv "$file" "$dest"
      chmod +x "$dest"
      echo "Installed to $dest"
      install_desktop_entry "$dest"
      case ":$PATH:" in
        *":$dest_dir:"*) ;;
        *) echo "Note: add '$dest_dir' to your PATH to run 'asusctl_gui' directly." ;;
      esac
      ;;
  esac
}

if [[ "$FORCE_APPIMAGE" == "true" ]]; then
  install_pkg "AppImage" || {
    echo "Error: no .AppImage asset found in release." >&2
    exit 1
  }
elif command -v apt-get >/dev/null 2>&1 || command -v dpkg >/dev/null 2>&1; then
  install_pkg "deb" || install_pkg "AppImage" || {
    echo "Error: no .deb or .AppImage asset found in release." >&2
    exit 1
  }
elif command -v dnf >/dev/null 2>&1 || command -v rpm >/dev/null 2>&1; then
  install_pkg "rpm" || install_pkg "AppImage" || {
    echo "Error: no .rpm or .AppImage asset found in release." >&2
    exit 1
  }
else
  install_pkg "AppImage" || {
    echo "Error: no .AppImage asset found in release." >&2
    exit 1
  }
fi

echo "ASUS Control installed successfully."
