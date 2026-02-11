<script lang="ts">
  import { onMount } from "svelte";
  import { readFile, writeFile, setWorkspace } from "$lib/api.v1";
  import FileTree from "../components/FileTree.svelte";
  import Editor from "../components/Editor.svelte";
  import Preview from "../components/Preview.svelte";
  import QuickOpen from "../components/QuickOpen.svelte";
  import StartScreen from "../components/StartScreen.svelte";
  import { getRecentFiles, addToRecents } from "$lib/persistence";
  import {
    Action,
    registerAction,
    removeAction,
    handleKeyDown,
  } from "$lib/shortcuts";

  let status = $state("Select a file...");
  let fileContent = $state("");
  let currentFile = $state<string | null>(null);
  let showPreview = $state(false);
  let showSidebar = $state(true);
  let showQuickOpen = $state(false);
  let recentFiles = $state<string[]>([]);

  // Autosave State
  let autosaveEnabled = $state(false);
  let autosaveTimer: number | null = null;
  const AUTOSAVE_DELAY = 2000; // 2 seconds

  // Resizing State
  let isResizing = $state(false);
  let editorWidth = $state(50); // percentage

  async function handleFileSelected(event: CustomEvent<string>) {
    const filename = event.detail;
    if (filename === currentFile) return;

    status = `Loading ${filename}...`;
    try {
      fileContent = "";

      const content = await readFile(filename);
      fileContent = content;
      currentFile = filename;
      status = `Viewing: ${filename}`;

      addToRecents(filename);
      recentFiles = getRecentFiles();
    } catch (e) {
      status = `Error reading ${filename}: ${e}`;
      console.error(e);
    }
  }

  // Generic Save Function
  async function performSave(
    filename: string,
    content: string,
    isAuto: boolean = false,
  ) {
    status = isAuto ? `Autosaving ${filename}...` : `Saving ${filename}...`;
    try {
      await writeFile(filename, content);
      // Don't update fileContent here as it might conflict with typing if not careful,
      // but for now it's fine since we just saved what is there.
      // Actually, better NOT to update fileContent to avoid loop if Editor syncs back.
      // Editor pushes changes -> fileContent updates? No, currently one way prop.
      // But for this app structure, Editor state is source of truth during edit.

      status = isAuto ? `Autosaved ${filename}` : `Saved ${filename}!`;

      setTimeout(() => {
        if (status.startsWith("Saved") || status.startsWith("Autosaved")) {
          status = `Viewing: ${filename}`;
        }
      }, 2000);
    } catch (e) {
      status = `Error saving ${filename}: ${e}`;
      console.error(e);
    }
  }

  async function handleSave(content: string) {
    if (!currentFile) {
      status = "No file selected to save.";
      return;
    }
    await performSave(currentFile, content);
  }

  function handleEditorChange(newContent: string) {
    // Update local content state? Maybe not needed if we trust Editor state.
    // But we might need it for Preview update if we bind it?
    fileContent = newContent; // Sync for Preview

    if (autosaveEnabled && currentFile) {
      if (autosaveTimer) clearTimeout(autosaveTimer);
      autosaveTimer = setTimeout(() => {
        if (currentFile) performSave(currentFile, newContent, true);
      }, AUTOSAVE_DELAY);
    }
  }

  function togglePreview() {
    showPreview = !showPreview;
  }

  function toggleSidebar() {
    showSidebar = !showSidebar;
  }

  function toggleQuickOpen() {
    showQuickOpen = !showQuickOpen;
  }

  function toggleAutosave() {
    autosaveEnabled = !autosaveEnabled;
    localStorage.setItem("autosaveEnabled", String(autosaveEnabled));
    status = autosaveEnabled ? "Autosave Enabled" : "Autosave Disabled";
    setTimeout(() => {
      if (status.startsWith("Autosave"))
        status = `Viewing: ${currentFile || "..."}`;
    }, 2000);
  }

  // Workspace Persistence
  async function loadLastWorkspace() {
    const last = localStorage.getItem("lastWorkspace");
    if (last) {
      try {
        console.log("Restoring workspace:", last);
        await setWorkspace(last);
        // We rely on FileTree to load roots on mount,
        // but we need to ensure it knows the workspace is set.
        // FileTree calls loadRoots onMount, which calls listDir(".").
        // listDir uses backend state. So if we await setWorkspace first, we are good.
        // But FileTree is child... so onMount here happens AFTER FileTree onMount?
        // No, parent onMount happens after child onMount usually.
        // We can trigger FileTree refresh? Or better, pass workspace as prop?
        // Or simpler: FileTree loads roots, if it fails/empty, maybe it retries?
        // Actually, FileTree onMount calls loadRoots immediately.
        // If we set workspace here in Parent onMount, we might race.
        // Fix: We should probably trigger a refresh.
        // Hack: Force remount of FileTree? Or expose a method.
        // Better: Just let FileTree handle its own persistence? No, global app state.
        // Let's pass a key to FileTree.
        // Or use an event.
        // For now, let's just accept the race or reload page?
        // Actually, we can just use the `api.v1` to set workspace before app mounts if possible?
        // Standard way: Parent sets up env, then renders children.

        // We can put FileTree in a {#await} block?
      } catch (e) {
        console.error("Failed to restore workspace", e);
      }
    }

    const autosave = localStorage.getItem("autosaveEnabled");
    if (autosave === "true") autosaveEnabled = true;
  }

  // Resizing Logic
  function startResize() {
    isResizing = true;
  }

  function stopResize() {
    isResizing = false;
  }

  function onMouseMove(e: MouseEvent) {
    if (!isResizing) return;
    const container = document.querySelector(".workspace-area");
    if (!container) return;

    const rect = container.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const width = rect.width;

    let newPercent = (x / width) * 100;
    // Clamp
    if (newPercent < 10) newPercent = 10;
    if (newPercent > 90) newPercent = 90;

    editorWidth = newPercent;
  }

  let workspaceReady = $state(false);

  onMount(() => {
    loadLastWorkspace().then(() => {
      workspaceReady = true;
      recentFiles = getRecentFiles();
    });

    window.addEventListener("mouseup", stopResize);
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("keydown", handleKeyDown);

    // Register Global Actions
    registerAction(Action.SAVE, () => handleSave(fileContent));
    registerAction(Action.TOGGLE_PREVIEW, togglePreview);
    registerAction(Action.TOGGLE_SIDEBAR, toggleSidebar);
    registerAction(Action.SEARCH, toggleQuickOpen);
    registerAction(Action.TOGGLE_AUTOSAVE, toggleAutosave);

    return () => {
      window.removeEventListener("mouseup", stopResize);
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("keydown", handleKeyDown);

      removeAction(Action.SAVE);
      removeAction(Action.TOGGLE_PREVIEW);
      removeAction(Action.TOGGLE_SIDEBAR);
      removeAction(Action.SEARCH);
      removeAction(Action.TOGGLE_AUTOSAVE);
    };
  });
</script>

<div class="app-container">
  <aside class="sidebar" class:hidden={!showSidebar}>
    {#if workspaceReady}
      <FileTree
        on:file-selected={handleFileSelected}
        selectedFile={currentFile}
      />
    {/if}
  </aside>
  <main class="main-content">
    <header class="status-bar">
      <span class="status-text">{status}</span>
      <div class="controls">
        <button
          class="icon-btn"
          onclick={toggleAutosave}
          title={autosaveEnabled ? "Disable Autosave" : "Enable Autosave"}
        >
          {autosaveEnabled ? "💾✅" : "💾❌"}
        </button>
        <button
          class="icon-btn"
          onclick={togglePreview}
          title={showPreview ? "Hide Preview" : "Show Preview"}
        >
          {showPreview ? "👁️‍🗨️" : "👁️"}
        </button>
      </div>
    </header>
    <div class="workspace-area">
      {#if !currentFile}
        <StartScreen
          {recentFiles}
          on:open-folder={() => {
            // We need to expose handleOpenFolder from somewhere or just use api directly?
            // FileTree has logic. Let's just key it.
            // Actually FileTree logic is inside FileTree.
            // We should move openFolder logic to parent or import it.
            // It is imported from api.v1.
            // But we want to update the workspace.
            // Let's use the API directly here.
            import("$lib/api.v1").then((mod) => {
              mod.openFolder().then((path) => {
                if (path) {
                  mod.setWorkspace(path).then(() => {
                    localStorage.setItem("lastWorkspace", path);
                    // Trigger reload? Or just trust FileTree to update if we could signal it.
                    // Simplest: window.location.reload();
                    window.location.reload();
                  });
                }
              });
            });
          }}
          on:open-file={(e) => handleFileSelected(e)}
          on:new-file={() => {
            // Trigger new file action?
            // We don't have direct access to FileTree methods.
            // We can just show a prompt here.
            // But better to just let user use the shortcut or sidebar.
            // Let's just simulate Ctr+N? No.
            alert("Use the sidebar or Ctrl+N to create a new file.");
          }}
        />
      {:else}
        <div
          class="pane editor-pane"
          style:width={showPreview ? `${editorWidth}%` : "100%"}
        >
          {#key currentFile}
            <Editor
              content={fileContent}
              onSave={handleSave}
              onChange={handleEditorChange}
            />
          {/key}
        </div>

        {#if showPreview}
          <div class="resizer" onmousedown={startResize}></div>
          <div class="pane preview-pane" style:width={`${100 - editorWidth}%`}>
            <Preview content={fileContent} />
          </div>
        {/if}
      {/if}
    </div>
  </main>

  {#if showQuickOpen}
    <QuickOpen
      on:select={(e) => {
        handleFileSelected(e); // Event detail is file path
        showQuickOpen = false;
      }}
      on:close={() => (showQuickOpen = false)}
    />
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    overflow: hidden;
    background-color: #1e1e1e;
    color: #ccc;
  }

  .app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .sidebar {
    width: 250px;
    height: 100%;
    flex-shrink: 0;
    border-right: 1px solid #333;
    transition: margin-left 0.2s ease-in-out;
  }

  .sidebar.hidden {
    margin-left: -250px;
  }

  .main-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .status-bar {
    padding: 0 10px;
    background-color: #252526;
    border-bottom: 1px solid #333;
    font-size: 0.9em;
    color: #aaa;
    height: 35px;
    flex-shrink: 0;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .status-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .controls {
    display: flex;
    gap: 10px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #aaa;
    cursor: pointer;
    font-size: 1.1em;
    padding: 4px;
    border-radius: 3px;
  }
  .icon-btn:hover {
    background-color: #37373d;
    color: #fff;
  }

  .workspace-area {
    flex-grow: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .pane {
    height: 100%;
    overflow: hidden;
  }

  .preview-pane {
    background-color: #1e1e1e;
  }

  .resizer {
    width: 5px;
    cursor: col-resize;
    background-color: #333;
    z-index: 10;
  }
  .resizer:hover {
    background-color: #007fd4;
  }
</style>
