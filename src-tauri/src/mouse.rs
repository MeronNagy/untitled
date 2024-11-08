pub mod commands {
    use tauri::{command, Window, Emitter};
    use winapi::um::winuser::{
        mouse_event, SetCursorPos, GetCursorPos, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    };
    use winapi::shared::windef::POINT;
    use std::thread;
    use std::time::Duration;

    #[derive(Clone, serde::Serialize)]
    pub struct Position {
        x: i32,
        y: i32,
    }

    #[command]
    pub fn click(x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y);
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }
    }

    #[command]
    pub fn start_tracking(window: Window) {
        thread::spawn(move || {
            loop {
                unsafe {
                    let mut point = POINT { x: 0, y: 0 };
                    if GetCursorPos(&mut point) != 0 {
                        let _ = window.emit("mouse-move", Position {
                            x: point.x,
                            y: point.y,
                        });
                    }
                }
                thread::sleep(Duration::from_millis(16)); // ~62.5fps update rate
            }
        });
    }
}