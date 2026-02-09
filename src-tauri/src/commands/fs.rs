use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn read_file(
    path: String,
    workspace: State<'_, Mutex<PathBuf>>,
) -> Result<String, String> {
    let workspace_path = workspace.lock().map_err(|e| e.to_string())?.clone();
    let full_path = workspace_path.join(&path);

    // Security check
    if !full_path.starts_with(&workspace_path) {
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
    workspace: State<'_, Mutex<PathBuf>>,
) -> Result<(), String> {
    let workspace_path = workspace.lock().map_err(|e| e.to_string())?.clone();
    let full_path = workspace_path.join(&path);

    // Security check
    if !full_path.starts_with(&workspace_path) {
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
pub async fn list_dir(
    path: String,
    workspace: State<'_, Mutex<PathBuf>>,
) -> Result<Vec<String>, String> {
    let workspace_path = workspace.lock().map_err(|e| e.to_string())?.clone();
    let full_path = workspace_path.join(&path);

    // Security check
    if !full_path.starts_with(&workspace_path) {
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

        entries.push(format!("{}{}", prefix, name));
    }

    entries.sort();

    Ok(entries)
}

#[tauri::command]
pub async fn set_workspace(
    path: String,
    workspace: State<'_, Mutex<PathBuf>>,
) -> Result<(), String> {
    let mut workspace_path = workspace.lock().map_err(|e| e.to_string())?;
    *workspace_path = PathBuf::from(path);
    Ok(())
}

#[tauri::command]
pub async fn create_dir(path: String, workspace: State<'_, Mutex<PathBuf>>) -> Result<(), String> {
    let workspace_path = workspace.lock().map_err(|e| e.to_string())?.clone();
    let full_path = workspace_path.join(&path);

    // Security check
    if !full_path.starts_with(&workspace_path) {
        return Err("Path outside workspace".into());
    }

    tokio::fs::create_dir_all(full_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_node(path: String, workspace: State<'_, Mutex<PathBuf>>) -> Result<(), String> {
    let workspace_path = workspace.lock().map_err(|e| e.to_string())?.clone();
    let full_path = workspace_path.join(&path);

    // Security check
    if !full_path.starts_with(&workspace_path) {
        return Err("Path outside workspace".into());
    }

    let metadata = tokio::fs::metadata(&full_path)
        .await
        .map_err(|e| e.to_string())?;

    if metadata.is_dir() {
        tokio::fs::remove_dir_all(full_path)
            .await
            .map_err(|e| e.to_string())
    } else {
        tokio::fs::remove_file(full_path)
            .await
            .map_err(|e| e.to_string())
    }
}
