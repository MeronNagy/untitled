use crate::action_script::action::Action;
use crate::action_script::script::ActionScript;
use crate::action_script::types::ActionType;

#[tauri::command]
pub fn orchestrate(script: String) -> Result<(), String> {
    let action_script = ActionScript::from_string(&script)?;

    for action in action_script.into_iter() {
        execute_action(action)?;
    }

    Ok(())
}

fn execute_action(action: Action) -> Result<(), String> {
    match action.action_type {
        ActionType::LeftClick => {
            let x = action.get_parameter("X");
        },
    }
    Ok(())
}
