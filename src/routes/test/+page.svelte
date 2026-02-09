<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { readFile, writeFile, listDir } from "$lib/api";

  let status = $state("Waiting...");
  let fileList = $state<string[]>([]);

  async function testBackend() {
    try {
      status = "Testing Write...";
      await writeFile("test_backend.txt", "Hello from Tauri backend!");
      
      status = "Testing Read...";
      const content = await readFile("test_backend.txt");
      if (content !== "Hello from Tauri backend!") throw new Error("Read content mismatch");

      status = "Testing List...";
      fileList = await listDir(".");
      
      if (!fileList.some(f => f.includes("test_backend.txt"))) throw new Error("File not found in list");

      status = "Success! Backend works.";
    } catch (e) {
      status = "Error: " + e;
      console.error(e);
    }
  }
</script>

<main>
  <h1>Backend Test</h1>
  <button onclick={testBackend}>Run Tests</button>
  <p>Status: {status}</p>
  <ul>
    {#each fileList as file}
      <li>{file}</li>
    {/each}
  </ul>
</main>
