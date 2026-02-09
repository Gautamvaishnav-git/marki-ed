let invokeFunction: any = () => console.log('invoking nothing!');

if (!import.meta.env.SSR) {
    const { invoke } = await import('@tauri-apps/api/core');
    invokeFunction = invoke;
}

export async function invoke<T>(name: string, args?: any): Promise<T> {
    return await invokeFunction(name, args);
}


export async function readFile(path: string): Promise<string> {
    return await invoke("read_file", { path });
}

export async function writeFile(path: string, content: string): Promise<void> {
    await invoke("write_file", { path, content });
}

export async function listDir(path: string): Promise<string[]> {
    return await invoke("list_dir", { path });
}

