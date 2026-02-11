<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { walkDir } from "$lib/api.v1";

    const dispatch = createEventDispatcher();

    let query = $state("");
    let files: string[] = $state([]);
    let results: string[] = $state([]);
    let selectedIndex = $state(0);
    let error = $state<string | null>(null);
    let inputEl: HTMLInputElement;

    onMount(async () => {
        try {
            files = await walkDir(".");
            filterFiles();
            inputEl?.focus();
        } catch (e) {
            error = String(e);
        }
    });

    function filterFiles() {
        if (!query) {
            results = files.slice(0, 20); // Show recent or top 20
        } else {
            // Simple fuzzy-ish search
            const lowerQuery = query.toLowerCase();
            results = files
                .filter((f) => f.toLowerCase().includes(lowerQuery))
                .sort((a, b) => {
                    // Prioritize startsWith
                    const aStarts = a.toLowerCase().startsWith(lowerQuery);
                    const bStarts = b.toLowerCase().startsWith(lowerQuery);
                    if (aStarts && !bStarts) return -1;
                    if (!aStarts && bStarts) return 1;
                    return a.length - b.length; // Shortest first
                })
                .slice(0, 20); // Limit results
        }
        selectedIndex = 0;
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "ArrowDown") {
            e.preventDefault();
            selectedIndex = (selectedIndex + 1) % results.length;
        } else if (e.key === "ArrowUp") {
            e.preventDefault();
            selectedIndex =
                (selectedIndex - 1 + results.length) % results.length;
        } else if (e.key === "Enter") {
            e.preventDefault();
            if (results[selectedIndex]) {
                selectFile(results[selectedIndex]);
            }
        } else if (e.key === "Escape") {
            dispatch("close");
        }
    }

    function selectFile(file: string) {
        dispatch("select", file);
    }

    $effect(() => {
        if (query !== undefined) filterFiles();
    });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
    class="modal-overlay"
    onclick={() => dispatch("close")}
    role="button"
    tabindex="0"
>
    <div
        class="modal"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        tabindex="-1"
    >
        <input
            bind:this={inputEl}
            bind:value={query}
            onkeydown={handleKeydown}
            type="text"
            placeholder="Search files..."
            autofocus
        />
        {#if error}
            <div class="error">{error}</div>
        {/if}
        <div class="results">
            {#each results as file, i}
                <button
                    class="result-item"
                    class:selected={i === selectedIndex}
                    onclick={() => selectFile(file)}
                    onmouseenter={() => (selectedIndex = i)}
                >
                    {file}
                </button>
            {/each}
            {#if results.length === 0 && query}
                <div class="no-results">No matching files</div>
            {/if}
        </div>
    </div>
</div>

<style>
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 2000;
        display: flex;
        justify-content: center;
        align-items: flex-start;
        padding-top: 50px;
    }

    .modal {
        width: 500px;
        background: #252526;
        border: 1px solid #454545;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        border-radius: 6px;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    input {
        width: 100%;
        padding: 10px;
        background: #3c3c3c;
        border: none;
        border-bottom: 1px solid #454545;
        color: #fff;
        font-size: 1.1em;
        outline: none;
        box-sizing: border-box;
    }

    .results {
        max-height: 400px;
        overflow-y: auto;
    }

    .result-item {
        display: block;
        width: 100%;
        text-align: left;
        padding: 8px 10px;
        background: none;
        border: none;
        color: #ccc;
        cursor: pointer;
        font-family: inherit;
        border-bottom: 1px solid #2d2d2d;
    }

    .result-item:last-child {
        border-bottom: none;
    }

    .result-item.selected {
        background-color: #094771;
        color: #fff;
    }

    .result-item:hover {
        background-color: #2a2d2e;
    }

    .result-item.selected:hover {
        background-color: #094771;
    }

    .error {
        color: #f48771;
        padding: 5px;
    }

    .no-results {
        padding: 10px;
        color: #888;
        font-style: italic;
    }
</style>
