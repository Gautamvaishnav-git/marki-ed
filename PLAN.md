## 🚀 **Atomic Tasks (Tight Timeline Edition)**

### **Day 1: Core MVP**

**Phase 1: Foundation (30 min)**
1. Install Rust + Tauri prerequisites
2. `npm create tauri-app` → **Choose Svelte** (not React)
3. Verify Hello World runs
4. Minimal structure: `components/`, `lib/api.ts`

**Phase 2: Backend Essentials (1-2 hours)**
5. Create **3 core Rust commands** in `src-tauri/commands/`:
   - `read_file(path) -> String`
   - `write_file(path, content) -> Result`
   - `list_dir(path) -> Vec<String>` (flat, non-recursive)
6. Add workspace root locking (security)
7. Test commands via Tauri dev tools

**Phase 3: File Tree (1 hour)**
8. Create `FileTree.svelte` - flat list only
9. Wire up `list_dir` command
10. Click → emit file path to parent
11. Use **emoji indicators** (📁 📝) - no icon libs

**Phase 4: Editor Core (2-3 hours)**
12. Install CodeMirror 6: `npm i @codemirror/state @codemirror/view @codemirror/lang-markdown`
13. Create `Editor.svelte` with CodeMirror
14. Load file via `read_file` on selection
15. Ctrl+S → call `write_file` (no auto-save yet)
16. Basic markdown syntax highlighting

---

### **Day 2: Preview + Polish**

**Phase 5: Preview Pane (2 hours)**
17. Install `markdown-it` (not marked - more stable)
18. Create `Preview.svelte` - hidden by default
19. Add toggle button (show/hide preview)
20. Render markdown on **idle/debounce** (not every keystroke)
21. Split-pane layout (CSS Grid, keep simple)

**Phase 6: Essential Polish (2-3 hours)**
22. Dark/light mode via **CSS variables** (easy win)
23. Recent files in localStorage (5-file max)
24. Simple error handling (toast/alert on file errors)
25. Production build: `npm run tauri build`
26. Test installer on target platform

---

## **What We're Skipping (For Now)**

❌ Recursive file tree  
❌ File watching  
❌ Search across files  
❌ Delete/Create UI (use `write_file("")` + OS file manager)  
❌ Fancy themes/syntax highlighting in preview  
❌ Monaco Editor  

*(Can add later if MVP proves successful)*

---

## **Key Technical Decisions Locked In**

| Choice | Why |
|--------|-----|
| **Svelte** | 40% smaller bundle than React, faster |
| **CodeMirror 6** | Perfect balance: feature-rich but lightweight (~50KB) |
| **markdown-it** | Stable, predictable, 1 dependency |
| **Emoji icons** | Zero deps, works everywhere |
| **Flat file tree** | 90% of use cases, 10% of complexity |
| **Manual save** | Simpler, more predictable than auto-save |

---

## **Rust Tips Integrated**

```rust
// commands/fs.rs
use tauri::State;
use std::path::PathBuf;

#[tauri::command]
pub async fn read_file(path: String, workspace: State<'_, PathBuf>) -> Result<String, String> {
    let full_path = workspace.join(&path);
    
    // Security: ensure within workspace
    if !full_path.starts_with(workspace.inner()) {
        return Err("Path outside workspace".into());
    }
    
    tokio::fs::read_to_string(full_path)
        .await
        .map_err(|e| e.to_string())
}
```

---

## **Estimated Timeline**

- **Day 1**: Working editor with file tree  
- **Day 2**: Preview + production build  
- **Total**: ~12-16 hours actual dev time

**This is shippable, fast, and under 5MB binary size.**
