# aperture

A minimal desktop text editor with a serene, monochrome light beige aesthetic.

Built with **Tauri v2**, **SvelteKit 5**, and **TypeScript**.

![Status](https://img.shields.io/badge/status-alpha-orange)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20wsl-blue)

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[SESSION.md](./SESSION.md)** | 📖 **Start here!** Complete technical deep-dive of what was built |
| **[QUICKSTART.md](./QUICKSTART.md)** | ⚡ Quick reference for developers |
| **[ROADMAP.md](./ROADMAP.md)** | 🗺️ Feature status and future plans |
| **[BUILDING.md](./BUILDING.md)** | 🏗️ Build and installation guide |

---

## ⚡ Quick Start

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build

# Install (Linux)
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

---

## ✨ Current Features

- ✅ Multi-tab editing with dirty state tracking
- ✅ File browser sidebar with **accordion-style folder expansion**
- ✅ **Line numbers** in editor
- ✅ **Syntax highlighting** for 16+ languages (Shiki)
- ✅ Quick open with fuzzy search (`Ctrl+K`) - uses `fd`
- ✅ Zoxide directory jumping (`Ctrl+O`)
- ✅ Find in file overlay (`Ctrl+F`)
- ✅ Git branch display in footer
- ✅ **Git status indicators** in sidebar (M/U/A/D)
- ✅ Save files (`Ctrl+S`)
- ✅ Close tabs (`Ctrl+W`)
- ✅ Monochrome beige color palette

## 🚧 Missing Features

See [ROADMAP.md](./ROADMAP.md) for complete list.

**High priority:**
- ❌ CLI support (`aperture file.txt`)
- ❌ Better find with match highlighting

---

## ⌨️ Keyboard Shortcuts

All use `Ctrl` (Linux-friendly):

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` | Quick open (fuzzy file search) |
| `Ctrl+O` | Jump to directory (zoxide) |
| `Ctrl+F` | Find in file |
| `Ctrl+S` | Save current file |
| `Ctrl+W` | Close current tab |

---

## 🎨 Color Palette

Monochrome beige scale (10 shades):

| Color | Hex | Usage |
|-------|-----|-------|
| Beige 50 | `#faf8f5` | Primary background |
| Beige 100 | `#f5f1eb` | Secondary background |
| Beige 200 | `#ece5db` | Tertiary background |
| Beige 300 | `#dfd4c3` | Borders, hover states |
| Beige 400 | `#cfc0a8` | Accents |
| Beige 500 | `#b8a68a` | Focus states |
| Beige 600 | `#9d8a6f` | Tertiary text |
| Beige 700 | `#7a6b56` | Secondary text |
| Beige 800 | `#5c5042` | Accent text |
| Beige 900 | `#3d3630` | Primary text |
| Beige 950 | `#2a241f` | Darkest (contrast) |

All customizable via CSS variables in `src/app.css`.

---

## 🛠️ Tech Stack

- **Frontend:** SvelteKit 5 + TypeScript
- **Backend:** Rust via Tauri v2
- **Styling:** Custom CSS with semantic variables
- **External Tools:**
  - `fd` - Fast file finding
  - `zoxide` - Smart directory jumping
  - `git` - Version control info

---

## 📋 Prerequisites

### Required

| Tool | Version | Install |
|------|---------|---------|
| Rust | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 20+ | Use [fnm](https://github.com/Schniz/fnm) or [nvm](https://github.com/nvm-sh/nvm) |
| pnpm | 9+ | `corepack enable && corepack prepare pnpm@latest --activate` |

### Linux System Dependencies

```bash
sudo apt update && sudo apt install -y \
  build-essential \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libssl-dev \
  pkg-config
```

### Optional (for full functionality)

```bash
# fd - for quick open (Ctrl+K)
sudo apt install fd-find
ln -s $(which fdfind) ~/.local/bin/fd

# zoxide - for directory jumping (Ctrl+O)
curl -sSfL https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh | bash
echo 'eval "$(zoxide init zsh)"' >> ~/.zshrc

# git - for branch display
sudo apt install git
```

---

## 🏗️ Project Structure

```
aperture/
├── src/
│   ├── lib/              # Svelte components
│   │   ├── Sidebar.svelte
│   │   ├── TabBar.svelte
│   │   ├── Editor.svelte
│   │   ├── Footer.svelte
│   │   ├── QuickOpen.svelte
│   │   ├── ZoxideJump.svelte
│   │   └── FindInFile.svelte
│   ├── routes/
│   │   ├── +page.svelte      # Main app
│   │   └── +layout.svelte    # Layout wrapper
│   ├── app.css               # Global styles + palette
│   └── app.html              # HTML shell
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs            # Tauri commands
│   │   └── main.rs           # Entry point
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # App configuration
├── SESSION.md                # Technical deep-dive
├── ROADMAP.md                # Feature roadmap
├── BUILDING.md               # Build guide
└── QUICKSTART.md             # Quick reference
```

---

## 🐛 Known Issues

See [SESSION.md](./SESSION.md) for complete list.

**Common:**
- Quick open requires `fd` in PATH
- Modals need to be outside `.app` container
- Layout needs explicit `flex-direction: row`

---

## 🚀 Usage

### Launch from Desktop
After installing via .deb, search for "aperture" in your app launcher.

### Launch from Terminal
```bash
aperture
```

**Note:** CLI arguments (`aperture file.txt`) not yet implemented. See roadmap.

---

## 📦 Build & Install

See [BUILDING.md](./BUILDING.md) for detailed instructions.

**Quick:**
```bash
pnpm tauri build
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

---

## 🤝 Contributing

This is an alpha project. See [SESSION.md](./SESSION.md) for technical details and [ROADMAP.md](./ROADMAP.md) for planned features.

**Quick wins for contributors:**
- Add line numbers
- Implement CLI argument parsing
- Integrate syntax highlighting
- Improve find functionality
- Add git status indicators

---

## 📄 License

MIT

---

## 🙏 Acknowledgments

- Inspired by [Mug](https://github.com/wrgoldstein/mug)
- Built with [Tauri](https://tauri.app/)
- Uses [fd](https://github.com/sharkdp/fd) and [zoxide](https://github.com/ajeetdsouza/zoxide)

---

**For developers:** Read [SESSION.md](./SESSION.md) first for complete context!
