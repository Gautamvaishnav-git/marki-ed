<script lang="ts">
  import { readFile } from "$lib/api.v1";
  import FileTree from "../components/FileTree.svelte";

  let status = $state("Select a file...");
  let fileContent = $state("");

  async function handleFileSelected(event: CustomEvent<string>) {
    const filename = event.detail;
    status = `Loading ${filename}...`;
    try {
      fileContent = await readFile(filename);
      status = `Viewing: ${filename}`;
    } catch (e) {
      status = `Error reading ${filename}: ${e}`;
      console.error(e);
    }
  }
</script>

<div class="app-container">
  <aside class="sidebar">
    <FileTree on:file-selected={handleFileSelected} />
  </aside>
  <main class="main-content">
    <div class="status-bar">{status}</div>
    <div class="editor-area">
      <pre>{fileContent}</pre>
    </div>
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    overflow: hidden;
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
  }

  .main-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .status-bar {
    padding: 5px 10px;
    background-color: #f0f0f0;
    border-bottom: 1px solid #ddd;
    font-size: 0.9em;
    color: #555;
  }

  .editor-area {
    flex-grow: 1;
    padding: 20px;
    overflow: auto;
    background-color: #fff;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    font-family: "Consolas", "Monaco", monospace;
  }

  @media (prefers-color-scheme: dark) {
    .status-bar {
      background-color: #333;
      border-bottom-color: #444;
      color: #aaa;
    }

    .editor-area {
      background-color: #1e1e1e;
      color: #d4d4d4;
    }
  }
</style>
