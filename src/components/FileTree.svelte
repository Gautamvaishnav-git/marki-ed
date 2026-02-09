<script lang="ts">
    import { onMount, createEventDispatcher, onDestroy } from "svelte";
    import {
        listDir,
        openFolder,
        setWorkspace,
        createDir,
        writeFile,
        deleteNode,
        renameNode,
    } from "$lib/api.v1";
    import { Action, registerAction, removeAction } from "$lib/shortcuts";

    const dispatch = createEventDispatcher();

    let { selectedFile = null } = $props();

    interface TreeNode {
        name: string;
        path: string;
        type: "dir" | "file";
        children?: TreeNode[];
        expanded: boolean;
    }

    // Root level nodes
    let rootNodes = $state<TreeNode[]>([]);
    let showMarkdownOnly = $state(true);
    let error = $state<string | null>(null);

    // Context Menu State
    let contextMenu = $state<{
        visible: boolean;
        x: number;
        y: number;
        node: TreeNode | null;
    }>({
        visible: false,
        x: 0,
        y: 0,
        node: null,
    });

    // Helper to check if a file is markdown
    function isMarkdown(name: string): boolean {
        return (
            name.toLowerCase().endsWith(".md") ||
            name.toLowerCase().endsWith(".markdown")
        );
    }

    // Helper to load nodes for a given path
    async function fetchNodes(path: string): Promise<TreeNode[]> {
        const rawFiles = await listDir(path);
        return rawFiles.map((entry) => {
            const isDir = entry.startsWith("📁 ");
            const name = entry.substring(3);
            const childPath = path === "." ? name : `${path}/${name}`;

            return {
                name,
                path: childPath,
                type: isDir ? "dir" : "file",
                expanded: false,
                children: undefined,
            };
        });
    }

    async function loadRoots() {
        try {
            error = null;
            rootNodes = await fetchNodes(".");
        } catch (e) {
            error = String(e);
            console.error("Failed to load root:", e);
        }
    }

    async function refreshTree() {
        await loadRoots();
    }

    async function handleOpenFolder() {
        try {
            const path = await openFolder();
            if (path) {
                await setWorkspace(path);
                localStorage.setItem("lastWorkspace", path); // PERSISTENCE
                await loadRoots();
            }
        } catch (e) {
            error = String(e);
            console.error("Failed to open folder:", e);
        }
    }

    async function handleNewFile(contextNode?: TreeNode | null) {
        // If no context node, try to infer from selectedFile
        let effectivePath = contextNode?.path;
        if (!effectivePath && selectedFile) {
            effectivePath = selectedFile;
        }

        const parentPath = effectivePath
            ? effectivePath.includes("/") // simplistic check, assumes files have extension or isDir we know?
                ? // Actually we don't know if selectedFile is dir or file here easily without looking up node.
                  // But selectedFile is usually a file in this app.
                  effectivePath.split("/").slice(0, -1).join("/")
                : "."
            : ".";

        // If contextNode provided and is dir, use it directly
        if (contextNode?.type === "dir") {
            // Re-derive for directory context
            // logic above was trying to be too clever. Let's simplify.
        }

        // Correct logic:
        let base = ".";
        if (contextNode) {
            base =
                contextNode.type === "dir"
                    ? contextNode.path
                    : contextNode.path.includes("/")
                      ? contextNode.path.split("/").slice(0, -1).join("/")
                      : ".";
        } else if (selectedFile) {
            // Assume selectedFile is a file (since we only select files mostly)
            base = selectedFile.includes("/")
                ? selectedFile.split("/").slice(0, -1).join("/")
                : ".";
        }

        const name = prompt("Enter file name (e.g. note.md):");
        if (!name) return;

        const fullPath = base === "." ? name : `${base}/${name}`;

        try {
            await writeFile(fullPath, "");
            await refreshTree();
            dispatch("file-selected", fullPath);
        } catch (e) {
            error = `Failed to create file: ${e}`;
        }
        closeContextMenu();
    }

    async function handleNewFolder(contextNode?: TreeNode | null) {
        let base = ".";
        if (contextNode) {
            base =
                contextNode.type === "dir"
                    ? contextNode.path
                    : contextNode.path.includes("/")
                      ? contextNode.path.split("/").slice(0, -1).join("/")
                      : ".";
        } else if (selectedFile) {
            base = selectedFile.includes("/")
                ? selectedFile.split("/").slice(0, -1).join("/")
                : ".";
        }

        const name = prompt("Enter folder name:");
        if (!name) return;

        const fullPath = base === "." ? name : `${base}/${name}`;

        try {
            await createDir(fullPath);
            await refreshTree();
        } catch (e) {
            error = `Failed to create folder: ${e}`;
        }
        closeContextMenu();
    }

    async function handleDelete(node: TreeNode) {
        if (!confirm(`Are you sure you want to delete ${node.name}?`)) return;

        try {
            await deleteNode(node.path);
            await refreshTree();
        } catch (e) {
            error = `Failed to delete: ${e}`;
        }
        closeContextMenu();
    }

    async function handleRename(node: TreeNode) {
        const newName = prompt("Enter new name:", node.name);
        if (!newName || newName === node.name) return;

        const parentPath = node.path.includes("/")
            ? node.path.split("/").slice(0, -1).join("/")
            : ".";
        const newPath =
            parentPath === "." ? newName : `${parentPath}/${newName}`;

        try {
            await renameNode(node.path, newPath);
            await refreshTree();
            // If the renamed file was selected, update selection?
            // Actually parent might need to know, but for now just refresh tree.
            if (selectedFile === node.path) {
                dispatch("file-selected", newPath);
            }
        } catch (e) {
            error = `Failed to rename: ${e}`;
        }
        closeContextMenu();
    }

    function handleContextMenu(event: MouseEvent, node: TreeNode) {
        event.preventDefault();
        event.stopPropagation();
        contextMenu = {
            visible: true,
            x: event.clientX,
            y: event.clientY,
            node: node,
        };
        document.addEventListener("click", closeContextMenu, { once: true });
    }

    function closeContextMenu() {
        contextMenu.visible = false;
        contextMenu.node = null;
    }

    async function toggleExpand(node: TreeNode) {
        if (node.type !== "dir") return;

        if (node.expanded) {
            node.expanded = false;
        } else {
            if (!node.children) {
                try {
                    node.children = await fetchNodes(node.path);
                } catch (e) {
                    console.error(`Failed to load ${node.path}:`, e);
                }
            }
            node.expanded = true;
        }
    }

    function handleFileClick(node: TreeNode) {
        if (node.type === "file") {
            dispatch("file-selected", node.path);
        } else {
            toggleExpand(node);
        }
    }

    onMount(() => {
        loadRoots();
        registerAction(Action.NEW_FILE, () => handleNewFile());
        registerAction(Action.NEW_FOLDER, () => handleNewFolder());

        return () => {
            removeAction(Action.NEW_FILE);
            removeAction(Action.NEW_FOLDER);
        };
    });
</script>

<div
    class="file-tree"
    oncontextmenu={(e) => {
        e.preventDefault();
    }}
    role="tree"
    tabindex="0"
>
    <div class="header">
        <div class="title-row">
            <h3>Files</h3>
            <div class="actions">
                <button
                    class="icon-btn"
                    onclick={() => handleNewFile(null)}
                    title="New File at Root">📝+</button
                >
                <button
                    class="icon-btn"
                    onclick={() => handleNewFolder(null)}
                    title="New Folder at Root">📁+</button
                >
                <button
                    class="icon-btn"
                    onclick={handleOpenFolder}
                    title="Open Folder">📂</button
                >
            </div>
        </div>
        <label class="filter-toggle" title="Show only Markdown files">
            <input type="checkbox" bind:checked={showMarkdownOnly} />
            <span>MD Only</span>
        </label>
    </div>

    {#if error}
        <div class="error">{error}</div>
    {/if}

    <div class="tree-content">
        {#snippet treeNode(node: TreeNode)}
            {@const isVisible =
                node.type === "dir" ||
                !showMarkdownOnly ||
                isMarkdown(node.name)}
            {@const isSelected = selectedFile === node.path}

            {#if isVisible}
                <div class="tree-item">
                    <button
                        class="node-btn {node.type}"
                        class:expanded={node.expanded}
                        class:selected={isSelected}
                        onclick={() => handleFileClick(node)}
                        oncontextmenu={(e) => handleContextMenu(e, node)}
                        title={node.path}
                    >
                        <span class="icon">
                            {#if node.type === "dir"}
                                {node.expanded ? "📂" : "📁"}
                            {:else}
                                📝
                            {/if}
                        </span>
                        <span class="name">{node.name}</span>
                    </button>

                    {#if node.type === "dir" && node.expanded && node.children}
                        <div class="children">
                            {#each node.children as child}
                                {@render treeNode(child)}
                            {/each}
                        </div>
                    {/if}
                </div>
            {/if}
        {/snippet}

        {#each rootNodes as node}
            {@render treeNode(node)}
        {/each}
    </div>

    <div class="footer">
        <button class="refresh-btn" onclick={loadRoots}>↻</button>
    </div>

    {#if contextMenu.visible}
        <div
            class="context-menu"
            style="top: {contextMenu.y}px; left: {contextMenu.x}px;"
        >
            <button onclick={() => handleNewFile(contextMenu.node)}
                >New File</button
            >
            <button onclick={() => handleNewFolder(contextMenu.node)}
                >New Folder</button
            >
            <div class="separator"></div>
            <button
                class="danger"
                onclick={() =>
                    contextMenu.node && handleDelete(contextMenu.node)}
                >Delete</button
            >
            <button
                onclick={() =>
                    contextMenu.node && handleRename(contextMenu.node)}
                >Rename</button
            >
        </div>
    {/if}
</div>

<style>
    .file-tree {
        width: 250px;
        height: 100%;
        background-color: #252526;
        border-right: 1px solid #333;
        display: flex;
        flex-direction: column;
        padding: 10px;
        box-sizing: border-box;
        color: #ccc;
        font-family: "Segoe UI", sans-serif;
        user-select: none;
        position: relative;
    }

    .header {
        margin-bottom: 10px;
        padding-bottom: 5px;
        border-bottom: 1px solid #333;
    }

    .title-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 5px;
    }

    h3 {
        margin: 0;
        font-size: 0.9em;
        font-weight: 600;
        color: #ddd;
        text-transform: uppercase;
    }

    .actions {
        display: flex;
        gap: 2px;
    }

    .icon-btn {
        background: transparent;
        border: 1px solid transparent;
        color: #aaa;
        cursor: pointer;
        font-size: 1em;
        padding: 0 3px;
        border-radius: 3px;
    }
    .icon-btn:hover {
        background: #37373d;
        color: #fff;
    }

    .filter-toggle {
        display: flex;
        align-items: center;
        font-size: 0.8em;
        cursor: pointer;
        color: #aaa;
    }
    .filter-toggle input {
        margin-right: 4px;
    }
    .filter-toggle:hover {
        color: #fff;
    }

    .tree-content {
        flex-grow: 1;
        overflow-y: auto;
    }

    .tree-item {
        display: flex;
        flex-direction: column;
    }

    .node-btn {
        display: flex;
        align-items: center;
        width: 100%;
        text-align: left;
        background: none;
        border: none;
        padding: 3px 6px;
        cursor: pointer;
        font-size: 0.9em;
        border-radius: 3px;
        color: #ccc;
        font-family: inherit;
        white-space: nowrap;
        overflow: hidden;
    }

    .node-btn:hover {
        background-color: #37373d;
        color: #fff;
    }

    .node-btn.selected {
        background-color: #094771; /* VS Code selected style */
        color: #fff;
    }

    .icon {
        margin-right: 6px;
        width: 16px;
        text-align: center;
        font-size: 0.9em;
    }

    .name {
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .children {
        margin-left: 14px;
        border-left: 1px solid #444;
    }

    .footer {
        border-top: 1px solid #333;
        padding-top: 5px;
        display: flex;
        justify-content: flex-end;
    }

    .refresh-btn {
        background: transparent;
        border: none;
        color: #888;
        cursor: pointer;
        padding: 2px 5px;
    }
    .refresh-btn:hover {
        color: #fff;
    }

    .error {
        color: #f48771;
        font-size: 0.8em;
        padding: 5px;
        background: #3a1d1d;
        margin-bottom: 5px;
        border-radius: 4px;
    }

    .context-menu {
        position: fixed;
        background: #252526;
        border: 1px solid #454545;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
        border-radius: 4px;
        padding: 4px 0;
        z-index: 1000;
        min-width: 120px;
    }

    .context-menu button {
        display: block;
        width: 100%;
        text-align: left;
        background: none;
        border: none;
        padding: 6px 12px;
        color: #ccc;
        cursor: pointer;
        font-family: inherit;
        font-size: 0.9em;
    }

    .context-menu button:hover {
        background-color: #094771;
        color: #fff;
    }

    .context-menu .separator {
        height: 1px;
        background-color: #454545;
        margin: 4px 0;
    }

    .context-menu button.danger:hover {
        background-color: #710909;
    }
</style>
