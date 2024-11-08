pub mod commands {
    use winapi::um::winuser::{INPUT_u, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, SendInput};

    #[tauri::command]
    pub fn press(key: char) {
        let vk_code = if key.is_ascii_alphabetic() {
            key.to_ascii_uppercase() as u16
        } else {
            match key {
                '1'..='9' => key as u16,
                '0' => 0x30,
                ' ' => 0x20,
                '\n' => 0x0D, // Enter key
                '\t' => 0x09, // Tab key
                _ => {
                    println!("Unsupported character: {}", key);
                    return;
                }
            }
        };

        unsafe {
            let input_press = INPUT {
                type_: INPUT_KEYBOARD,
                u: {
                    let mut u: INPUT_u = std::mem::zeroed();
                    *u.ki_mut() = std::mem::zeroed();
                    u.ki_mut().wVk = vk_code;
                    u.ki_mut().dwFlags = 0;
                    u.ki_mut().time = 0;
                    u.ki_mut().dwExtraInfo = 0;
                    u
                },
            };

            let input_release = INPUT {
                type_: INPUT_KEYBOARD,
                u: {
                    let mut u: INPUT_u = std::mem::zeroed();
                    *u.ki_mut() = std::mem::zeroed();
                    u.ki_mut().wVk = vk_code;
                    u.ki_mut().dwFlags = KEYEVENTF_KEYUP;
                    u.ki_mut().time = 0;
                    u.ki_mut().dwExtraInfo = 0;
                    u
                },
            };

            let mut inputs = vec![input_press, input_release];

            println!("Sending input: {}", vk_code);
            SendInput(
                inputs.len() as u32,
                inputs.as_mut_ptr(),
                size_of::<INPUT>() as i32,
            );
        }
    }
}