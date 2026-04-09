# aperture

A fast, minimal text editor built for developers who value simplicity and aesthetics.

**aperture** combines the speed of native apps with a serene monochrome beige color palette. Perfect for distraction-free coding with essential features like syntax highlighting, git integration, and smart search.

Built with **Tauri v2**, **SvelteKit 5**, **Rust**, and **TypeScript**.

![Status](https://img.shields.io/badge/status-beta-green)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20wsl-blue)
![Language](https://img.shields.io/badge/languages-16+-brightgreen)

---

## 🎯 Why aperture?

- **⚡ Fast** - Native performance with Rust backend
- **🎨 Beautiful** - Carefully crafted monochrome beige aesthetic
- **🔧 Developer-focused** - Git integration, syntax highlighting, smart search
- **⌨️ Keyboard-first** - All features accessible via shortcuts
- **💾 Lightweight** - ~13MB binary, minimal resource usage
- **🐧 Linux-native** - Built for Linux and WSL2

---

## 📸 Screenshots

> Coming soon! aperture features a clean, distraction-free interface with a warm beige color palette.

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

## ✨ Features

### Editor
- **Multi-tab editing** with dirty state tracking
- **Syntax highlighting** for 16+ languages (JavaScript, TypeScript, Python, Rust, Go, Java, C/C++, Ruby, PHP, HTML, CSS, JSON, Markdown, YAML, Shell, SQL)
- **Line numbers** with synchronized scrolling
- **Smart Find** (Ctrl+F) with match counting and navigation
- **Font size controls** (Ctrl+/Ctrl-/Ctrl+0) that persist across sessions
- **Undo/Redo** (Ctrl+Z/Ctrl+Shift+Z)
- Tab support with spaces (2-space default)

### Navigation
- **File browser sidebar** with accordion-style folders (single-click expand, double-click navigate)
- **Quick open** (Ctrl+K) - fuzzy file search with `fd`
- **Zoxide integration** (Ctrl+O) - jump to frequently-used directories
- **CLI support** - open files and directories from terminal: `aperture file.txt`

### Git Integration
- **Git status indicators** in sidebar (M/U/A/D for modified, untracked, added, deleted)
- **Branch display** in footer
- Real-time git status updates

### Aesthetics
- **Monochrome beige palette** (10 carefully-chosen shades)
- Clean, distraction-free interface
- Consistent design language throughout

---

## 🚀 Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/BaoGeist/aperture.git
cd aperture
pnpm install
pnpm tauri build --no-bundle

# Add to PATH (add this to your ~/.zshrc or ~/.bashrc)
export PATH="$HOME/projects/aperture/src-tauri/target/release:$PATH"

# Reload shell
source ~/.zshrc  # or source ~/.bashrc
```

### Usage

```bash
# Open to home directory
aperture

# Open specific file
aperture file.txt

# Open multiple files in tabs
aperture file1.txt file2.txt file3.txt

# Open directory
aperture ~/projects/myapp

# Development mode
cd aperture
pnpm tauri dev
```

---

## ⌨️ Keyboard Shortcuts

All shortcuts use **Ctrl** (Linux-friendly):

### Files & Navigation
| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | New file (creates Untitled-1, Untitled-2, etc.) |
| `Ctrl+K` | Quick open (fuzzy file search) |
| `Ctrl+O` | Jump to directory (zoxide) |
| `Ctrl+S` | Save current file |
| `Ctrl+W` | Close current tab |

### Editing
| Shortcut | Action |
|----------|--------|
| `Ctrl+F` | Find in file (with match counting) |
| `Ctrl+A` | Select all text |
| `Ctrl+Z` | Undo |
| `Ctrl+Shift+Z` | Redo |
| `Tab` | Insert 2 spaces |

### View
| Shortcut | Action |
|----------|--------|
| `Ctrl++` or `Ctrl+=` | Increase font size |
| `Ctrl+-` | Decrease font size |
| `Ctrl+0` | Reset font size to default (14px) |

### Find Controls (when find is open)
| Shortcut | Action |
|----------|--------|
| `Enter` | Next match |
| `Shift+Enter` | Previous match |
| `Aa` button | Toggle case sensitivity |

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
| **Rust** | 1.75+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **Node.js** | 20+ | Use [fnm](https://github.com/Schniz/fnm) or [nvm](https://github.com/nvm-sh/nvm) |
| **pnpm** | 9+ | `corepack enable && corepack prepare pnpm@latest --activate` |

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

### Optional Tools (Enhanced Functionality)

```bash
# fd - for quick open (Ctrl+K)
sudo apt install fd-find
ln -s $(which fdfind) ~/.local/bin/fd

# zoxide - for directory jumping (Ctrl+O)
curl -sSfL https://raw.githubusercontent.com/ajeetdsouza/zoxide/main/install.sh | bash
echo 'eval "$(zoxide init zsh)"' >> ~/.zshrc  # or ~/.bashrc

# git - for branch display and status indicators
sudo apt install git
```

**Note:** aperture works without these tools, but features like Quick Open and Zoxide Jump will be disabled.

---

## 🏗️ Project Structure

```
aperture/
├── src/
│   ├── lib/                  # Svelte components
│   │   ├── Sidebar.svelte    # File browser with git status
│   │   ├── TabBar.svelte     # Multi-tab management
│   │   ├── Editor.svelte     # Text editor with syntax highlighting
│   │   ├── Footer.svelte     # Git branch display
│   │   ├── QuickOpen.svelte  # Fuzzy file finder (Ctrl+K)
│   │   ├── ZoxideJump.svelte # Directory jumper (Ctrl+O)
│   │   └── FindInFile.svelte # Smart search (Ctrl+F)
│   ├── routes/
│   │   ├── +page.svelte      # Main application
│   │   └── +layout.svelte    # Layout wrapper
│   ├── app.css               # Global styles + color palette
│   └── app.html              # HTML shell
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs            # Tauri commands (file ops, git, etc.)
│   │   └── main.rs           # Entry point + CLI arg handling
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # App configuration
├── SESSION.md                # 📖 Technical deep-dive (start here!)
├── ROADMAP.md                # 🗺️ Feature status and future plans
├── BUILDING.md               # 🏗️ Build and installation guide
└── QUICKSTART.md             # ⚡ Quick reference for developers
```

---

## 📦 Building

### Quick Build (No Package)

```bash
# Build release binary
pnpm tauri build --no-bundle

# Binary location
./src-tauri/target/release/aperture

# Add to PATH (optional)
export PATH="$HOME/projects/aperture/src-tauri/target/release:$PATH"
```

### Full Build (Creates .deb Package)

```bash
# Install xdg-utils (required for .deb)
sudo apt install xdg-utils

# Build with packaging
pnpm tauri build

# Install the .deb
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

### Development Mode

```bash
# Hot-reload development
pnpm tauri dev
```

---

## 🐛 Known Issues

- **WSL/WSLg:** Requires proper X11/WSLg setup. Run `wsl --shutdown` and restart if GUI doesn't appear.
- **GPU Warnings:** May show libEGL warnings on first run. Add user to `render` group: `sudo usermod -a -G render $USER`
- **Font Rendering:** Some systems may need additional font packages for optimal rendering.

See [SESSION.md](./SESSION.md) for troubleshooting details.

---

## 🔜 Roadmap

### Completed ✅
- Multi-tab editing
- Syntax highlighting (16+ languages)
- Line numbers
- Git status indicators
- CLI support
- Smart find with match counting
- Font size controls
- Undo/Redo
- New file creation

### Up Next
- **Smart Indent** - Context-aware indentation
- **Bracket Auto-Close** - Auto-close `{}`, `[]`, `()`, `""`
- **Word Wrap Toggle** - Enable/disable line wrapping
- **Recent Files** - Quick access to recently opened files
- **Settings Panel** - Customize theme and preferences

See [ROADMAP.md](./ROADMAP.md) for the complete feature list.

---

## 🤝 Contributing

aperture is in active development. Contributions welcome!

**Good First Issues:**
- Add more language support to syntax highlighting
- Implement word wrap toggle
- Add bracket auto-close for quotes and brackets
- Improve error messages and user feedback
- Add automated tests

**Before Contributing:**
1. Read [SESSION.md](./SESSION.md) for technical architecture
2. Check [ROADMAP.md](./ROADMAP.md) for planned features
3. See [BUILDING.md](./BUILDING.md) for development setup

---

## 📄 License

MIT License - see LICENSE file for details

---

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) - Rust-powered desktop apps
- Syntax highlighting by [Shiki](https://shiki.style/) - VS Code's highlighter
- Uses [fd](https://github.com/sharkdp/fd) for fast file finding
- Uses [zoxide](https://github.com/ajeetdsouza/zoxide) for smart directory jumping
- Inspired by minimalist editor design philosophy

---

## 📚 Learn More

| Resource | Description |
|----------|-------------|
| **[SESSION.md](./SESSION.md)** | 📖 Complete technical documentation - start here! |
| **[ROADMAP.md](./ROADMAP.md)** | 🗺️ Feature status and future plans |
| **[QUICKSTART.md](./QUICKSTART.md)** | ⚡ Quick reference for developers |
| **[BUILDING.md](./BUILDING.md)** | 🏗️ Build and installation guide |

---

**Made with ❤️ for developers who appreciate simplicity**
