use crate::action_script::action::Action;
use crate::action_script::script::ActionScript;

#[tauri::command]
pub fn orchestrate(script: String) -> Result<(), String>{
    let action_script = ActionScript::from_string(&script)?;
    println!("{}", action_script);

    Ok(())
}

fn execute_action_script(script: ActionScript) {

}

fn execute_action(action: Action) {
}
