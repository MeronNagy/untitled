use crate::action_script::action::Action;
use crate::action_script::script::ActionScript;
use crate::action_script::types::ActionType;
use crate::input::mouse;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tokio::task;

static INTERRUPT_ORCHESTRATION: AtomicBool = AtomicBool::new(false);

#[tauri::command]
pub async fn orchestrate(script: String) -> Result<(), String> {
    INTERRUPT_ORCHESTRATION.store(false, Ordering::Relaxed);

    let result = task::spawn_blocking(move || {
        let action_script = ActionScript::from_string(&script).map_err(|e| e.to_string())?;

        for action in action_script.into_iter() {
            if INTERRUPT_ORCHESTRATION.load(Ordering::Relaxed) {
                return Err("Execution cancelled.".to_string());
            }

            execute_action(&action).map_err(|e| e.to_string())?;

            let delay = action.get_integer_parameter("Delay").unwrap_or(0);
            if delay > 0 {
                let sleep_interval = 50u64; // milliseconds
                let mut elapsed_time = 0u64;

                while elapsed_time < delay as u64 {
                    if INTERRUPT_ORCHESTRATION.load(Ordering::Relaxed) {
                        return Err("Execution cancelled.".to_string());
                    }

                    thread::sleep(Duration::from_millis(sleep_interval));
                    elapsed_time += sleep_interval;
                }
            }
        }

        Ok(())
    })
    .await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e.to_string()),
    }
}

fn execute_action(action: &Action) -> Result<(), String> {
    match action.action_type {
        ActionType::LeftClick => {
            mouse::mouse_click(
                action.get_integer_parameter("X")?,
                action.get_integer_parameter("Y")?,
            );
        }
    }

    Ok(())
}

#[tauri::command]
pub fn interrupt_orchestration() {
    INTERRUPT_ORCHESTRATION.store(true, Ordering::Relaxed);
}
