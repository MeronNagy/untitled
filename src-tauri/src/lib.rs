mod action_script;
mod input;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let alt_s_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyS);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            if shortcut == &alt_s_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        action_script::orchestrator::interrupt_orchestration()
                                    }
                                    ShortcutState::Released => {}
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(alt_s_shortcut)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            input::mouse::mouse_click,
            input::mouse::mouse_listener,
            input::keyboard::keyboard_click,
            input::keyboard::keyboard_listener,
            action_script::orchestrator::orchestrate,
            action_script::orchestrator::interrupt_orchestration,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
