# Aperture - Feature Status & Roadmap

## ✅ Implemented Features

- [x] Multi-tab editing with dirty state tracking
- [x] File browser sidebar with directory navigation
- [x] **Accordion-style folder expansion** (NEW!)
- [x] Quick open with fuzzy search (`Ctrl+K`) - requires `fd`
- [x] Find in file overlay (`Ctrl+F`)
- [x] Git branch display in footer
- [x] **Git status indicators in sidebar** (NEW!)
- [x] Zoxide directory jumping (`Ctrl+O`)
- [x] Save files (`Ctrl+S`)
- [x] Close tabs (`Ctrl+W`)
- [x] Tab indentation (2 spaces)
- [x] **Line numbers** (NEW!)
- [x] **Syntax highlighting** (NEW!) - 16+ languages with Shiki
- [x] Monochrome light beige color palette
- [x] Responsive layout (sidebar, tabs, editor, footer)

## 🚧 Missing Features (From Original Plan)

### High Priority (Remaining)
- [ ] **CLI Launcher** - `aperture file.txt`, `aperture ~/projects`
- [ ] **Smart Find** - Highlight matches, jump between results
- [x] ~~**Syntax Highlighting**~~ ✅ DONE - Using Shiki for 20+ languages
- [x] ~~**Line Numbers**~~ ✅ DONE - Shows line numbers in editor
- [x] ~~**Git Status in Sidebar**~~ ✅ DONE - Show modified/untracked files

### Medium Priority
- [ ] **Smart Indent** - Auto-indent based on file type
- [ ] **Bracket Auto-Close** - Auto-close `{}`, `[]`, `()`, `""`
- [ ] **Word Wrap Toggle** - Enable/disable line wrapping
- [ ] **Undo/Redo** - `Ctrl+Z` / `Ctrl+Shift+Z`
- [x] ~~**Select All**~~ ✅ DONE - `Ctrl+A` (native browser support)
- [ ] **Font Size Controls** - `Ctrl++`, `Ctrl+-`, `Ctrl+0`

### Low Priority
- [ ] **Markdown Enhancements** - Bold headers, styled blockquotes
- [ ] **Multiple Cursors** - `Ctrl+D` to select next occurrence
- [ ] **Recent Files** - Quick access to recently opened
- [ ] **Settings Panel** - Customize theme, font, keybindings
- [ ] **Split View** - Edit two files side by side

## 🎨 Current Tech Stack

- **Frontend**: SvelteKit 5 + TypeScript
- **Backend**: Rust (Tauri v2)
- **Styling**: Custom CSS with CSS variables
- **External Tools**:
  - `fd` - Fast file finding (for quick open)
  - `zoxide` - Smart directory jumping
  - `git` - Version control info

## 📦 Build & Install

See [BUILDING.md](./BUILDING.md) for detailed build and installation instructions.

**Quick build:**
```bash
pnpm tauri build
```

**Quick install:**
```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/aperture_*.deb
```

## 🎯 Next Steps

If you want to continue development, I recommend implementing features in this order:

1. **CLI Support** - Most useful for daily workflow
2. **Syntax Highlighting** - Makes it a proper code editor
3. **Line Numbers** - Expected in any text editor
4. **Better Find** - Improve search functionality
5. **Smart Indent** - Quality of life improvement

## 📝 Notes

- Currently optimized for Linux/WSL
- Monochrome beige palette is fully customizable via CSS variables
- All keyboard shortcuts use `Ctrl` (Linux-friendly)
