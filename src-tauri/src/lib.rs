mod mouse;
mod keyboard;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            mouse::commands::click,
            keyboard::commands::press,
            mouse::commands::start_tracking,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
