# Quick Reference - Aperture

## For New Developers

**Start here:**
1. Read [SESSION.md](./SESSION.md) - Complete technical deep-dive of what was built
2. Check [ROADMAP.md](./ROADMAP.md) - Feature status and future plans
3. Review [BUILDING.md](./BUILDING.md) - Build and installation guide

## Development Workflow

```bash
# First time setup
pnpm install

# Run dev mode (hot reload)
pnpm tauri dev

# Build production
pnpm tauri build

# Install locally
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

## Key Files to Know

| File | Purpose |
|------|---------|
| `src/routes/+page.svelte` | Main app, keyboard shortcuts, state management |
| `src/app.css` | Color palette (10 beige shades) |
| `src/lib/*.svelte` | UI components (Sidebar, Editor, TabBar, etc.) |
| `src-tauri/src/lib.rs` | Rust backend (file I/O, git, fd, zoxide) |
| `src-tauri/tauri.conf.json` | App config (window size, bundle settings) |

## Tech Stack

- **Frontend:** SvelteKit 5 + TypeScript (uses runes: `$state`, `$props`)
- **Backend:** Rust + Tauri v2
- **External Tools:** fd (file search), zoxide (directory jump), git

## Current Features

✅ Multi-tab editing  
✅ File browser sidebar  
✅ Quick open (`Ctrl+K`)  
✅ Zoxide jump (`Ctrl+O`)  
✅ Find in file (`Ctrl+F`)  
✅ Save/Close tabs  
✅ Git branch display  
✅ Beige color palette  

## Missing Features (High Value)

❌ Syntax highlighting  
❌ CLI support (`aperture file.txt`)  
❌ Line numbers  
❌ Better find (with highlights)  
❌ Git status in sidebar  

## Common Issues

### "fd not found" in Quick Open
```bash
sudo apt install fd-find
ln -s $(which fdfind) ~/.local/bin/fd
```

### Modals appear in top-right
Ensure modals are rendered **outside** `.app` container in `+page.svelte`.

### Layout shifts on load
Check `flex-direction: row` is set on `.app` class.

## Keyboard Shortcuts

All use `Ctrl` (Linux-friendly):

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` | Quick open (fuzzy file search) |
| `Ctrl+O` | Jump to directory (zoxide) |
| `Ctrl+F` | Find in file |
| `Ctrl+S` | Save current file |
| `Ctrl+W` | Close current tab |

## Quick Wins for Next Session

1. **Add line numbers** - Simple div alongside textarea
2. **CLI support** - Parse `std::env::args()` in Rust
3. **Syntax highlighting** - Integrate Shiki or CodeMirror
4. **Better find** - Highlight matches, show count
5. **New file button** - Create untitled files

## Color Palette Reference

```css
--beige-50: #faf8f5;  /* Lightest - main background */
--beige-100: #f5f1eb; /* Secondary background */
--beige-200: #ece5db; /* Tertiary background */
--beige-300: #dfd4c3; /* Borders, hover */
--beige-400: #cfc0a8; /* Accents */
--beige-500: #b8a68a; /* Focus states */
--beige-600: #9d8a6f; /* Tertiary text */
--beige-700: #7a6b56; /* Secondary text */
--beige-800: #5c5042; /* Accent text */
--beige-900: #3d3630; /* Primary text */
--beige-950: #2a241f; /* Darkest - added for contrast */
```

## Testing Before Release

- [ ] Open file from sidebar
- [ ] Edit and save (Ctrl+S)
- [ ] Dirty indicator appears/clears
- [ ] Quick open finds files
- [ ] Zoxide shows directories
- [ ] Multiple tabs work
- [ ] Build completes
- [ ] .deb installs successfully

## Debug Commands

```bash
# View Rust logs
RUST_LOG=debug pnpm tauri dev

# Test Tauri command (in DevTools F12)
await window.__TAURI__.invoke('read_file', { path: '/path/to/file' });

# Check zoxide database
zoxide query -l
```

## Resources

- [SESSION.md](./SESSION.md) - Full technical documentation
- [Tauri v2 Docs](https://tauri.app/v2/)
- [Svelte 5 Docs](https://svelte-5-preview.vercel.app/)
- [fd](https://github.com/sharkdp/fd)
- [zoxide](https://github.com/ajeetdsouza/zoxide)
