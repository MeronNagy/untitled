mod action_script;
mod input;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            input::mouse::mouse_click,
            input::mouse::mouse_listener,
            input::keyboard::keyboard_click,
            input::keyboard::keyboard_listener,
            action_script::orchestrator::orchestrate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
