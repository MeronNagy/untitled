mod keyboard;
mod mouse;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            mouse::commands::mouse_click,
            keyboard::commands::keyboard_click,
            mouse::commands::start_tracking,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
