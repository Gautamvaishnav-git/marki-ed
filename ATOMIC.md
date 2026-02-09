# Atomic Tasks

## Day 1: Core MVP

### Phase 1: Foundation (30 min)
- [x] Install Rust + Tauri prerequisites
- [x] `npm create tauri-app` → Choose Svelte (not React)
- [x] Verify Hello World runs
- [x] Create minimal structure: `components/`, `lib/api.ts`

### Phase 2: Backend Essentials (1-2 hours)
- [x] Create 3 core Rust commands in `src-tauri/commands/`:
  - `read_file(path) -> String`
  - `write_file(path, content) -> Result`
  - `list_dir(path) -> Vec<String>` (flat, non-recursive)
- [x] Add workspace root locking (security)
- [x] Test commands via Tauri dev tools

### Phase 3: File Tree (1 hour)
- [x] Create `FileTree.svelte` - flat list only
- [x] Wire up `list_dir` command
- [x] Implement click → emit file path to parent mechanism
- [x] Use emoji indicators (📁 📝) - no icon libs

### Phase 4: Editor Core (2-3 hours)
- [ ] Install CodeMirror 6: `npm i @codemirror/state @codemirror/view @codemirror/lang-markdown`
- [ ] Create `Editor.svelte` with CodeMirror
- [ ] Implement file loading via `read_file` on selection
- [ ] Implement Ctrl+S → call `write_file` (no auto-save yet)
- [ ] Add basic markdown syntax highlighting

## Day 2: Preview + Polish

### Phase 5: Preview Pane (2 hours)
- [ ] Install `markdown-it` (not marked - more stable)
- [ ] Create `Preview.svelte` - hidden by default
- [ ] Add toggle button (show/hide preview)
- [ ] Implement markdown rendering on idle/debounce (not every keystroke)
- [ ] Implement split-pane layout (CSS Grid, keep simple)

### Phase 6: Essential Polish (2-3 hours)
- [ ] Implement Dark/light mode via CSS variables
- [ ] Implement Recent files in localStorage (5-file max)
- [ ] Add simple error handling (toast/alert on file errors)
- [ ] Create production build: `npm run tauri build`
- [ ] Test installer on target platform
