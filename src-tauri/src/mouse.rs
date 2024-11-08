pub mod commands {
    use winapi::um::winuser::{
        mouse_event, SetCursorPos, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    };

    #[tauri::command]
    pub fn click(x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y);
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }
    }
}