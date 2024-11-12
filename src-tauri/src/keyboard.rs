pub mod commands {
    use device_query::{DeviceQuery, DeviceState};
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };
    use std::thread;
    use std::time::Duration;
    use tauri::{Emitter, Window};

    #[tauri::command]
    pub fn keyboard_click(key: char) {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        if key.is_uppercase() {
            enigo.key(Key::Shift, Press).unwrap();
            enigo
                .key(Key::Unicode(key.to_lowercase().next().unwrap()), Click)
                .unwrap();
            enigo.key(Key::Shift, Release).unwrap();
        } else {
            enigo.key(Key::Unicode(key), Click).unwrap();
        }
    }

    #[derive(Clone, serde::Serialize)]
    struct KeyboardEvent {
        key: String,
    }

    #[tauri::command]
    pub fn keyboard_listener(window: Window) {
        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut emitted_keys = std::collections::HashSet::new();

            loop {
                let keys = device_state.get_keys();

                emitted_keys.retain(|key| keys.contains(key));

                for key in keys {
                    if !emitted_keys.contains(&key) {
                        let keyboard_event = KeyboardEvent {
                            key: key.to_string(),
                        };

                        let _ = window.emit("key-click", keyboard_event);
                        emitted_keys.insert(key);
                    }
                }

                thread::sleep(Duration::from_millis(10)); // 100 Hz update rate
            }
        });
    }
}
