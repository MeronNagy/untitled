use enigo::{Button, Coordinate::Abs, Direction::Click, Enigo, Mouse, Settings};
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Window};

#[derive(Clone, serde::Serialize)]
pub struct Position {
    x: i32,
    y: i32,
}

#[tauri::command]
pub fn mouse_click(x: i32, y: i32) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let point = enigo.location().unwrap();
    if point.0 != x || point.1 != y {
        enigo.move_mouse(x, y, Abs).unwrap();
    }

    enigo.button(Button::Left, Click).unwrap();
}

#[tauri::command]
pub fn mouse_listener(window: Window) {
    thread::spawn(move || {
        let enigo = Enigo::new(&Settings::default()).unwrap();

        loop {
            let point = enigo.location().unwrap();
            let _ = window.emit(
                "mouse-move",
                Position {
                    x: point.0,
                    y: point.1,
                },
            );
            thread::sleep(Duration::from_millis(16)); // ~62.5fps update rate
        }
    });
}
