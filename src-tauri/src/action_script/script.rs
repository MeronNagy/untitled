use crate::action_script::action::Action;
use crate::action_script::error::ParseError;
use std::fs;
use std::path::Path;
use std::fmt;

pub struct ActionScript {
    actions: Vec<Action>,
}

impl ActionScript {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn from_string(content: &str) -> Result<Self, ParseError> {
        let mut script = ActionScript::new();

        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                let action = Action::from_str(line)?;
                script.add_action(action);
            }
        }

        Ok(script)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
        let content = fs::read_to_string(path)?;
        Self::from_string(&content)
    }

    pub fn to_string(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ParseError> {
        fs::write(path, self.to_string())?;
        Ok(())
    }
}

impl fmt::Display for ActionScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl IntoIterator for ActionScript {
    type Item = Action;
    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}
