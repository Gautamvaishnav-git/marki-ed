<script lang="ts">
    import { onMount } from "svelte";
    import MarkdownIt from "markdown-it";

    let { content = "" } = $props();

    let md = new MarkdownIt({
        html: true,
        linkify: true,
        typographer: true,
    });

    let rendered = $derived(md.render(content));
</script>

<div class="preview-container">
    <div class="markdown-body">
        {@html rendered}
    </div>
</div>

<style>
    .preview-container {
        height: 100%;
        width: 100%;
        overflow-y: auto;
        background-color: #1e1e1e;
        color: #ddd;
        padding: 20px;
        box-sizing: border-box;
        border-left: 1px solid #333;
    }

    /* Basic Markdown Styles matching VS Code dark theme roughly */
    :global(.markdown-body) {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica,
            Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
        font-size: 16px;
        line-height: 1.5;
    }

    :global(.markdown-body h1, .markdown-body h2, .markdown-body h3) {
        margin-top: 24px;
        margin-bottom: 16px;
        font-weight: 600;
        line-height: 1.25;
        color: #fff;
    }
    :global(.markdown-body h1) {
        font-size: 2em;
        border-bottom: 1px solid #333;
        padding-bottom: 0.3em;
    }
    :global(.markdown-body h2) {
        font-size: 1.5em;
        border-bottom: 1px solid #333;
        padding-bottom: 0.3em;
    }
    :global(.markdown-body p) {
        margin-top: 0;
        margin-bottom: 16px;
    }

    :global(.markdown-body code) {
        padding: 0.2em 0.4em;
        margin: 0;
        font-size: 85%;
        background-color: rgba(110, 118, 129, 0.4);
        border-radius: 6px;
        font-family:
            ui-monospace,
            SFMono-Regular,
            SF Mono,
            Menlo,
            Consolas,
            Liberation Mono,
            monospace;
    }

    :global(.markdown-body pre) {
        padding: 16px;
        overflow: auto;
        font-size: 85%;
        line-height: 1.45;
        background-color: #161b22;
        border-radius: 6px;
    }
    :global(.markdown-body pre code) {
        background-color: transparent;
        padding: 0;
    }

    :global(.markdown-body blockquote) {
        padding: 0 1em;
        color: #8b949e;
        border-left: 0.25em solid #30363d;
        margin: 0 0 16px 0;
    }

    :global(.markdown-body ul, .markdown-body ol) {
        padding-left: 2em;
        margin-top: 0;
        margin-bottom: 16px;
    }

    :global(.markdown-body a) {
        color: #58a6ff;
        text-decoration: none;
    }
    :global(.markdown-body a:hover) {
        text-decoration: underline;
    }

    :global(.markdown-body img) {
        max-width: 100%;
        box-sizing: content-box;
        background-color: #fff; /* Ensure transparent images show up */
    }

    :global(.markdown-body hr) {
        height: 0.25em;
        padding: 0;
        margin: 24px 0;
        background-color: #30363d;
        border: 0;
    }
</style>
