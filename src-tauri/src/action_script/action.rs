use crate::action_script::error::ParseError;
use crate::action_script::types::ActionType;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Action {
    action_type: ActionType,
    parameters: HashMap<String, String>,
}

impl Action {
    pub fn new(action_type: ActionType) -> Self {
        Self {
            action_type,
            parameters: HashMap::new(),
        }
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) {
        self.parameters.insert(key.to_string(), value.to_string());
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    pub fn to_string(&self) -> String {
        let mut parts = vec![format!("{:?}", self.action_type)];

        for (key, value) in &self.parameters {
            parts.push(format!("{}={}", key, value));
        }

        parts.join("; ")
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = s.split(';').map(str::trim).collect();
        if parts.is_empty() {
            return Err(ParseError::InvalidParameterFormat);
        }

        let action_type = ActionType::from_str(parts[0])?;
        let mut action = Action::new(action_type);

        for part in &parts[1..] {
            let kv: Vec<&str> = part.split('=').map(str::trim).collect();
            if kv.len() != 2 {
                return Err(ParseError::InvalidParameterFormat);
            }
            action.set_parameter(kv[0], kv[1]);
        }

        Ok(action)
    }
}
