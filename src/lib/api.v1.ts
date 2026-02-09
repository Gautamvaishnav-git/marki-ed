import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export async function readFile(path: string): Promise<string> {
    return await invoke("read_file", { path });
}

export async function writeFile(path: string, content: string): Promise<void> {
    await invoke("write_file", { path, content });
}

export async function listDir(path: string): Promise<string[]> {
    return await invoke("list_dir", { path });
}

export async function setWorkspace(path: string): Promise<void> {
    await invoke("set_workspace", { path });
}

export async function openFolder(): Promise<string | null> {
    const selected = await open({
        directory: true,
        multiple: false,
        recursive: true,
    });

    if (selected === null) {
        return null;
    }

    if (Array.isArray(selected)) {
        return selected[0];
    }

    return selected;
}

export async function createDir(path: string): Promise<void> {
    await invoke("create_dir", { path });
}

export async function deleteNode(path: string): Promise<void> {
    await invoke("delete_node", { path });
}
