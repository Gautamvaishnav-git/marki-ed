<script lang="ts">
  import { readFile, writeFile } from "$lib/api.v1";
  import FileTree from "../components/FileTree.svelte";
  import Editor from "../components/Editor.svelte";

  let status = $state("Select a file...");
  let fileContent = $state("");
  let currentFile = $state<string | null>(null);
  let showPreview = $state(false); // Placeholder for phase 5

  async function handleFileSelected(event: CustomEvent<string>) {
    const filename = event.detail;
    // Even if currentFile is same, we might want to reload if it changed on disk?
    // But for performance, skip if same.
    if (filename === currentFile) return;

    status = `Loading ${filename}...`;
    try {
      // Clear content to ensure editor resets, especially if new file is empty
      fileContent = "";

      const content = await readFile(filename);
      fileContent = content;
      currentFile = filename;
      status = `Viewing: ${filename}`;
    } catch (e) {
      status = `Error reading ${filename}: ${e}`;
      console.error(e);
    }
  }

  async function handleSave(content: string) {
    if (!currentFile) {
      status = "No file selected to save.";
      return;
    }

    status = `Saving ${currentFile}...`;
    try {
      await writeFile(currentFile, content);
      fileContent = content;
      status = `Saved ${currentFile}!`;

      // Clear "Saved!" message after 2 seconds
      setTimeout(() => {
        if (status.startsWith("Saved")) {
          status = `Viewing: ${currentFile}`;
        }
      }, 2000);
    } catch (e) {
      status = `Error saving ${currentFile}: ${e}`;
      console.error(e);
    }
  }

  function togglePreview() {
    showPreview = !showPreview;
    // TODO: Implement actual split pane in Phase 5
    status = showPreview
      ? "Preview Mode (Pending)"
      : `Viewing: ${currentFile || "..."}`;
  }
</script>

<div class="app-container">
  <aside class="sidebar">
    <FileTree on:file-selected={handleFileSelected} />
  </aside>
  <main class="main-content">
    <div class="status-bar">
      <span class="status-text">{status}</span>
      <button class="icon-btn" onclick={togglePreview} title="Toggle Preview">
        {showPreview ? "👁️‍🗨️" : "👁️"}
      </button>
    </div>
    <div class="editor-area">
      {#key currentFile}
        <Editor content={fileContent} onSave={handleSave} />
      {/key}
    </div>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    overflow: hidden;
    background-color: #1e1e1e; /* Dark theme base */
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

  .editor-area {
    flex-grow: 1;
    overflow: hidden; /* Editor handles scroll */
    background-color: #282c34;
  }
</style>
