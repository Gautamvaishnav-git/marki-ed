use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn read_file(path: String, workspace: State<'_, PathBuf>) -> Result<String, String> {
    let full_path = workspace.join(&path);

    // Security check
    if !full_path.starts_with(workspace.inner()) {
        return Err("Path outside workspace".into());
    }

    tokio::fs::read_to_string(full_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(
    path: String,
    content: String,
    workspace: State<'_, PathBuf>,
) -> Result<(), String> {
    let full_path = workspace.join(&path);

    // Security check
    if !full_path.starts_with(workspace.inner()) {
        return Err("Path outside workspace".into());
    }

    // Ensure parent directory exists
    if let Some(parent) = full_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }

    tokio::fs::write(full_path, content)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_dir(path: String, workspace: State<'_, PathBuf>) -> Result<Vec<String>, String> {
    let full_path = workspace.join(&path);

    // Security check
    if !full_path.starts_with(workspace.inner()) {
        return Err("Path outside workspace".into());
    }

    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(full_path)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(entry) = read_dir.next_entry().await.map_err(|e| e.to_string())? {
        let file_type = entry
            .file_type()
            .await
            .map_err(|e: std::io::Error| e.to_string())?;

        let prefix = if file_type.is_dir() { "📁 " } else { "📝 " };
        let name = entry.file_name().to_string_lossy().to_string();

        // For MVP, simplistic list
        entries.push(format!("{}{}", prefix, name));
    }

    // Sort directories first, then files? Or just simple sort for now
    entries.sort();

    Ok(entries)
}
