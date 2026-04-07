# Building & Installing Aperture

## Quick Start

```bash
# Build production version
cd /home/bungeist/projects/aperture
pnpm tauri build

# Install on Linux (recommended)
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

## Build Command

```bash
pnpm tauri build
```

**What this creates:**
- Binary: `src-tauri/target/release/aperture`
- .deb package: `src-tauri/target/release/bundle/deb/aperture_*.deb`
- AppImage: `src-tauri/target/release/bundle/appimage/aperture_*.AppImage`

**Build time:** ~2-5 minutes (first build), ~30 seconds (subsequent builds)

## Installation Options

### Option 1: Install .deb Package (Recommended)

**Pros:** Integrates with system, appears in app menu, easy to uninstall

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

**Installs to:**
- Binary: `/usr/bin/aperture`
- Desktop entry: `/usr/share/applications/aperture.desktop`
- Icons: `/usr/share/icons/`

**Uninstall:**
```bash
sudo apt remove aperture
```

### Option 2: Copy Binary

**Pros:** Quick, no sudo needed, portable

```bash
mkdir -p ~/.local/bin
cp src-tauri/target/release/aperture ~/.local/bin/
chmod +x ~/.local/bin/aperture
```

**Create desktop entry:**
```bash
cat > ~/.local/share/applications/aperture.desktop << 'EOF'
[Desktop Entry]
Name=aperture
Comment=A minimal text editor with a serene aesthetic
Exec=/home/bungeist/.local/bin/aperture %f
Icon=text-editor
Terminal=false
Type=Application
Categories=Utility;TextEditor;Development;
MimeType=text/plain;text/markdown;
EOF

update-desktop-database ~/.local/share/applications/
```

**Uninstall:**
```bash
rm ~/.local/bin/aperture
rm ~/.local/share/applications/aperture.desktop
```

### Option 3: AppImage (Portable)

**Pros:** Self-contained, no installation needed, portable

```bash
# Make executable
chmod +x src-tauri/target/release/bundle/appimage/aperture_*.AppImage

# Run directly
./aperture_*.AppImage

# Or add to PATH
mv aperture_*.AppImage ~/.local/bin/aperture
```

**Uninstall:**
```bash
rm ~/.local/bin/aperture
```

## Usage

### Launch from Desktop

After installing, search for "aperture" in your app launcher and click to open.

### Launch from Terminal

```bash
# Open in current directory
aperture

# Open specific file (when CLI support is added)
aperture /path/to/file.txt

# Open directory (when CLI support is added)
aperture ~/projects
```

**Current limitation:** CLI arguments not yet implemented. App always opens to home directory.

## Build Artifacts

After building, you'll find:

```
src-tauri/target/release/
├── aperture                      # Main binary (~20MB)
└── bundle/
    ├── deb/
    │   └── aperture_0.1.0_amd64.deb   # Debian package (~10MB)
    └── appimage/
        └── aperture_0.1.0_amd64.AppImage  # AppImage (~30MB)
```

## Development vs Production

| Feature | Development (`pnpm tauri dev`) | Production (`pnpm tauri build`) |
|---------|-------------------------------|--------------------------------|
| Hot reload | ✅ Yes | ❌ No |
| DevTools | ✅ Available | ❌ Disabled |
| Startup time | 🐢 Slower | ⚡ Fast |
| Optimization | ❌ None | ✅ Full |
| Size | ~50MB+ | ~20MB |

## Cross-Platform Builds

**Linux (current):**
```bash
pnpm tauri build
```

**Windows (from WSL):**
```bash
pnpm tauri build --target x86_64-pc-windows-msvc
```

**macOS (requires macOS):**
```bash
pnpm tauri build --target aarch64-apple-darwin  # M1/M2
pnpm tauri build --target x86_64-apple-darwin   # Intel
```

## Troubleshooting

**Build fails:**
- Ensure all dependencies installed: `sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev ...`
- Clear cache: `rm -rf src-tauri/target`

**App won't start:**
- Check dependencies: `ldd ~/.local/bin/aperture`
- Run from terminal to see errors: `aperture`

**Icons missing:**
- Icons are in `src-tauri/icons/` - they're bundled automatically

## Size Optimization

Current binary is ~20MB. To reduce:

```toml
# Add to src-tauri/Cargo.toml under [profile.release]
[profile.release]
strip = true       # Strip symbols
lto = true        # Link-time optimization
opt-level = "z"   # Optimize for size
```

Trade-off: Smaller binary, slightly longer build time.
