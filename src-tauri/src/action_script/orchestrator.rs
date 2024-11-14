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
pub fn interrupt_orchestration() {
    INTERRUPT_ORCHESTRATION.store(true, Ordering::Relaxed);
}

#[tauri::command]
pub async fn orchestrate(script: String, replay_count: i32) -> Result<(), String> {
    if replay_count < 1 {
        return Err("Replay count has to be 1 or greater.".to_string());
    }

    INTERRUPT_ORCHESTRATION.store(false, Ordering::Relaxed);

    let result = task::spawn_blocking(move || run_script(&script, replay_count)).await;

    match result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e.to_string()),
    }
}

fn run_script(script: &str, replay_count: i32) -> Result<(), String> {
    let action_script = ActionScript::from_string(script).map_err(|e| e.to_string())?;

    let actions: Vec<_> = action_script.into_iter().collect();

    for _ in 0..replay_count {
        if INTERRUPT_ORCHESTRATION.load(Ordering::Relaxed) {
            return Err("Execution cancelled.".to_string());
        }

        execute_action_script(&actions)?;
    }

    Ok(())
}

fn execute_action_script(actions: &[Action]) -> Result<(), String> {
    for action in actions {
        if INTERRUPT_ORCHESTRATION.load(Ordering::Relaxed) {
            return Err("Execution cancelled.".to_string());
        }

        execute_action(action)?;
        handle_delay(action)?;
    }

    Ok(())
}

fn handle_delay(action: &Action) -> Result<(), String> {
    let delay = action.get_integer_parameter("Delay").unwrap_or(0);
    if delay <= 0 {
        return Ok(());
    }

    let sleep_interval = 50u64;
    let mut elapsed_time = 0u64;

    while elapsed_time < delay as u64 {
        if INTERRUPT_ORCHESTRATION.load(Ordering::Relaxed) {
            return Err("Execution cancelled.".to_string());
        }

        thread::sleep(Duration::from_millis(sleep_interval));
        elapsed_time += sleep_interval;
    }

    Ok(())
}

fn execute_action(action: &Action) -> Result<(), String> {
    match action.action_type {
        ActionType::LeftClick => {
            execute_left_click(action)?;
        }
    }

    Ok(())
}

fn execute_left_click(action: &Action) -> Result<(), String> {
    mouse::mouse_click(
        action.get_integer_parameter("X")?,
        action.get_integer_parameter("Y")?,
    );

    Ok(())
}
