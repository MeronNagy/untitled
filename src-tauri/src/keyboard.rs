pub mod commands {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };
    use rdev::{listen, EventType};
    use std::thread;
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
    struct KeyEvent {
        key: String,
        event_type: String,
    }

    #[tauri::command]
    pub fn keyboard_listener(window: Window) {
        thread::spawn(move || {
            if let Err(error) = listen(move |event| match event.event_type {
                EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                    let event_type = match event.event_type {
                        EventType::KeyPress(_) => "keypress",
                        EventType::KeyRelease(_) => "keyrelease",
                        _ => unreachable!(),
                    };

                    let key_event = KeyEvent {
                        key: format!("{:?}", key),
                        event_type: event_type.to_string(),
                    };

                    let _ = window.emit("key-click", key_event);
                }
                _ => (),
            }) {
                eprintln!("Error: {:?}", error);
            }
        });
    }
}
