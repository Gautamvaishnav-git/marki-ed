export const MAX_RECENTS = 10;

export function getRecentFiles(): string[] {
    try {
        const stored = localStorage.getItem("recentFiles");
        return stored ? JSON.parse(stored) : [];
    } catch {
        return [];
    }
}

export function addToRecents(path: string) {
    let recents = getRecentFiles();
    // Remove if exists to bubble to top
    recents = recents.filter((p) => p !== path);
    // Add to top
    recents.unshift(path);
    // Limit
    recents = recents.slice(0, MAX_RECENTS);
    localStorage.setItem("recentFiles", JSON.stringify(recents));
}
