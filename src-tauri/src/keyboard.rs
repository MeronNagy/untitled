pub mod commands {
    use enigo::{
        Direction::{Click, Press, Release},
        Enigo, Key, Keyboard, Settings,
    };

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
}
