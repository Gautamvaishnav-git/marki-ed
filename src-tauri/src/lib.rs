mod commands;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize workspace state to current working directory
            // In a real app, this might be set via a dialog or config
            let cwd = std::env::current_dir().expect("failed to get current dir");
            app.manage(cwd);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::fs::read_file,
            commands::fs::write_file,
            commands::fs::list_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
