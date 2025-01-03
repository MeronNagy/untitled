use crate::action_script::action::Action;
use crate::action_script::error::ParseError;
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

        for (line_number, line) in content.lines().enumerate() {
            let line = line.trim();
            if !line.is_empty() {
                let action = Action::from_str(line)
                    .map_err(|err| ParseError::with_line_info(line_number + 1, err))?;
                script.add_action(action);
            }
        }

        Ok(script)
    }

    pub fn as_string(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.as_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Not used atm
    //    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ParseError> {
    //        let content = fs::read_to_string(path)?;
    //        Self::from_string(&content)
    //    }
    //    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ParseError> {
    //        fs::write(path, self.as_string())?;
    //        Ok(())
    //    }
}

impl fmt::Display for ActionScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl IntoIterator for ActionScript {
    type Item = Action;
    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}
