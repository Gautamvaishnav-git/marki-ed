<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { EditorState } from "@codemirror/state";
    import { EditorView, keymap, lineNumbers } from "@codemirror/view";
    import { markdown } from "@codemirror/lang-markdown";
    import {
        defaultKeymap,
        history,
        historyKeymap,
    } from "@codemirror/commands";
    import { oneDark } from "@codemirror/theme-one-dark";

    // Using $props() for Svelte 5
    let {
        content = "",
        onSave = (text: string) => {},
        onChange = (text: string) => {},
    } = $props();

    let editorContainer: HTMLDivElement;
    let view: EditorView;

    function initEditor() {
        const startState = EditorState.create({
            doc: content,
            extensions: [
                keymap.of([
                    ...defaultKeymap,
                    ...historyKeymap,
                    {
                        key: "Mod-s",
                        run: () => {
                            if (view) {
                                onSave(view.state.doc.toString());
                            }
                            return true;
                        },
                    },
                ]),
                history(),
                markdown(),
                oneDark,
                EditorView.lineWrapping,
                lineNumbers(),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        onChange(update.state.doc.toString());
                    }
                }),
            ],
        });

        view = new EditorView({
            state: startState,
            parent: editorContainer,
        });
    }

    // Effect to sync content prop to editor
    $effect(() => {
        // Only update if the content prop is different from the current editor state
        if (view && content !== view.state.doc.toString()) {
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: content,
                },
            });
        }
    });

    onMount(() => {
        initEditor();
    });

    onDestroy(() => {
        if (view) view.destroy();
    });
</script>

<div class="editor-wrapper" bind:this={editorContainer}></div>

<style>
    .editor-wrapper {
        height: 100%;
        width: 100%;
        overflow: hidden; /* CodeMirror handles scroll */
        font-size: 16px;
        background-color: #282c34; /* Match One Dark background roughly */
    }

    /* Ensure CodeMirror takes full height */
    :global(.cm-editor) {
        height: 100%;
    }

    :global(.cm-scroller) {
        overflow: auto;
    }
</style>
