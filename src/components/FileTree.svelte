<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { listDir } from "$lib/api.v1";

    const dispatch = createEventDispatcher();

    let files = $state<string[]>([]);
    let error = $state<string | null>(null);

    async function loadFiles() {
        try {
            files = await listDir(".");
        } catch (e) {
            error = String(e);
            console.error("Failed to list dir:", e);
        }
    }

    function handleFileClick(fileEntry: string) {
        // entry is "📁 name" or "📝 name"
        // simplistic parsing for MVP
        const isDir = fileEntry.startsWith("📁 ");
        const name = fileEntry.substring(3);

        if (!isDir) {
            dispatch("file-selected", name);
        } else {
            // For now, simple flat list, maybe just log or ignore folder click
            console.log("Folder clicked:", name);
        }
    }

    onMount(() => {
        loadFiles();
    });
</script>

<div class="file-tree">
    <h3>Files</h3>
    {#if error}
        <div class="error">{error}</div>
    {/if}
    <ul>
        {#each files as file}
            <li>
                <button class="file-btn" onclick={() => handleFileClick(file)}>
                    {file}
                </button>
            </li>
        {/each}
    </ul>
    <button class="refresh-btn" onclick={loadFiles}>↻ Refresh</button>
</div>

<style>
    .file-tree {
        width: 250px;
        height: 100%;
        background-color: #f3f3f3;
        border-right: 1px solid #ddd;
        display: flex;
        flex-direction: column;
        padding: 10px;
        box-sizing: border-box;
        overflow-y: auto;
    }

    h3 {
        margin-top: 0;
        margin-bottom: 10px;
        font-size: 1.1em;
        color: #333;
    }

    ul {
        list-style: none;
        padding: 0;
        margin: 0;
        flex-grow: 1;
    }

    li {
        margin-bottom: 2px;
    }

    .file-btn {
        width: 100%;
        text-align: left;
        background: none;
        border: none;
        padding: 6px 8px;
        cursor: pointer;
        font-size: 0.95em;
        border-radius: 4px;
        color: #444;
        transition: background-color 0.1s;
        font-family: inherit;
    }

    .file-btn:hover {
        background-color: #e0e0e0;
    }

    .error {
        color: red;
        font-size: 0.8em;
        margin-bottom: 5px;
    }

    .refresh-btn {
        margin-top: 10px;
        padding: 5px;
        font-size: 0.8em;
    }

    @media (prefers-color-scheme: dark) {
        .file-tree {
            background-color: #252526;
            border-right-color: #333;
        }

        h3 {
            color: #ddd;
        }

        .file-btn {
            color: #ccc;
        }

        .file-btn:hover {
            background-color: #37373d;
        }
    }
</style>
