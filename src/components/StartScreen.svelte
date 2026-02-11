<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    // Using $props() for Svelte 5
    let { recentFiles = [] as string[] } = $props();

    function handleOpenFolder() {
        dispatch("open-folder");
    }

    function handleOpenFile(path: string) {
        dispatch("open-file", path);
    }

    function handleNewFile() {
        dispatch("new-file");
    }
</script>

<div class="start-screen">
    <div class="content">
        <h1>Marki-Ed</h1>
        <p class="subtitle">Fast Markdown Editor</p>

        <div class="actions">
            <button class="primary" onclick={handleNewFile}
                >New File (Ctrl+N)</button
            >
            <button onclick={handleOpenFolder}>Open Folder...</button>
        </div>

        {#if recentFiles.length > 0}
            <div class="recents">
                <h3>Recent Files</h3>
                <ul>
                    {#each recentFiles as file}
                        <li>
                            <button
                                class="link-btn"
                                onclick={() => handleOpenFile(file)}
                            >
                                {file}
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        {/if}

        <div class="shortcuts-hint">
            <p><code>Ctrl+P</code> to Search Files</p>
            <p><code>Ctrl+Shift+P</code> to Toggle Preview</p>
        </div>
    </div>
</div>

<style>
    .start-screen {
        height: 100%;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: #1e1e1e;
        color: #ccc;
    }

    .content {
        max-width: 500px;
        width: 100%;
        text-align: center;
    }

    h1 {
        font-size: 2.5em;
        margin-bottom: 0.2em;
        color: #fff;
    }

    .subtitle {
        color: #888;
        margin-bottom: 2em;
    }

    .actions {
        display: flex;
        gap: 10px;
        justify-content: center;
        margin-bottom: 3em;
    }

    button {
        padding: 8px 16px;
        background: #3c3c3c;
        border: 1px solid #454545;
        color: #fff;
        cursor: pointer;
        border-radius: 4px;
        font-size: 1em;
    }

    button:hover {
        background: #4c4c4c;
    }

    button.primary {
        background: #0078d4;
        border-color: #0078d4;
    }

    button.primary:hover {
        background: #006abc;
    }

    .recents {
        text-align: left;
        background: #252526;
        padding: 20px;
        border-radius: 6px;
        border: 1px solid #333;
    }

    h3 {
        margin-top: 0;
        font-size: 0.9em;
        text-transform: uppercase;
        color: #888;
        border-bottom: 1px solid #333;
        padding-bottom: 5px;
        margin-bottom: 10px;
    }

    ul {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    li {
        margin-bottom: 5px;
    }

    .link-btn {
        background: none;
        border: none;
        color: #0078d4;
        text-align: left;
        width: 100%;
        padding: 4px 0;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }

    .link-btn:hover {
        background: none;
        text-decoration: underline;
        color: #40a0ff;
    }

    .shortcuts-hint {
        margin-top: 30px;
        color: #666;
        font-size: 0.9em;
    }

    code {
        background: #333;
        padding: 2px 4px;
        border-radius: 3px;
    }
</style>
