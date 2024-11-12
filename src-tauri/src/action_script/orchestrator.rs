use crate::action_script::action::Action;
use crate::action_script::script::ActionScript;
use crate::action_script::types::ActionType;
use crate::input::mouse;

#[tauri::command]
pub fn orchestrate(script: String) -> Result<(), String> {
    let action_script = ActionScript::from_string(&script)?;

    for action in action_script.into_iter() {
        println!("Orchestrating {}", action);
        execute_action(action)?;
    }

    Ok(())
}

fn execute_action(action: Action) -> Result<(), String> {
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
