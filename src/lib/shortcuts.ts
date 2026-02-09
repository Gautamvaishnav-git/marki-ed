export enum Action {
    NEW_FILE = "NEW_FILE",
    NEW_FOLDER = "NEW_FOLDER",
    SAVE = "SAVE",
    TOGGLE_PREVIEW = "TOGGLE_PREVIEW",
    TOGGLE_AUTOSAVE = "TOGGLE_AUTOSAVE",
}

type Handler = () => void;

const handlers: Record<string, Handler> = {};

export function registerAction(action: Action, handler: Handler) {
    handlers[action] = handler;
}

export function removeAction(action: Action) {
    delete handlers[action];
}

export function handleKeyDown(e: KeyboardEvent) {
    // Mac uses Meta, Windows uses Ctrl usually.
    // We can allow both for convenience or detect OS.
    const mod = e.ctrlKey || e.metaKey;
    const shift = e.shiftKey;
    const key = e.key.toLowerCase();

    if (!mod) return;

    let action: Action | null = null;

    if (key === 'n') {
        if (shift) action = Action.NEW_FOLDER;
        else action = Action.NEW_FILE;
    } else if (key === 's') {
        if (!shift) action = Action.SAVE;
    } else if (key === 'p') {
        if (!shift) action = Action.TOGGLE_PREVIEW;
    } else if (key === ',') {
        if (!shift) action = Action.TOGGLE_AUTOSAVE;
    }

    if (action && handlers[action]) {
        e.preventDefault();
        e.stopPropagation();
        handlers[action]();
    }
}
