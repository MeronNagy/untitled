use crate::action_script::error::ParseError;
use crate::action_script::types::ActionType;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    parameters: HashMap<String, String>,
}

impl Action {
    pub fn new(action_type: ActionType) -> Self {
        Self {
            action_type,
            parameters: HashMap::new(),
        }
    }

    fn validate_integer_parameter(value: &str) -> Result<i32, ParseError> {
        value.parse::<i32>().map_err(|_| ParseError::InvalidParameterValue {
            parameter: value.to_string(),
            reason: "Value must be an integer".to_string(),
        })
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ParseError> {
        // Validate that the parameter is allowed
        match key {
            "X" | "Y" => {
                // Validate that the value is an integer
                Self::validate_integer_parameter(value)?;
                self.parameters.insert(key.to_string(), value.to_string());
                Ok(())
            }
            _ => Err(ParseError::InvalidParameter(key.to_string())),
        }
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }

    fn validate_required_parameters(&self) -> Result<(), ParseError> {
        if self.action_type == ActionType::LeftClick {
            // Check for required X parameter
            if !self.parameters.contains_key("X") {
                return Err(ParseError::MissingParameter("X".to_string()));
            }

            // Check for required Y parameter
            if !self.parameters.contains_key("Y") {
                return Err(ParseError::MissingParameter("Y".to_string()));
            }

            // Validate that X and Y are valid integers
            for param in ["X", "Y"] {
                if let Some(value) = self.parameters.get(param) {
                    Self::validate_integer_parameter(value)?;
                }
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut parts = vec![format!("{:?}", self.action_type)];

        for (key, value) in &self.parameters {
            parts.push(format!("{}={}", key, value));
        }

        parts.join("; ")
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = s.split(';')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        let action_type = ActionType::from_str(parts[0])?;
        let mut action = Action::new(action_type);

        for part in &parts[1..] {
            let kv: Vec<&str> = part.split('=').map(str::trim).collect();
            if kv.len() != 2 {
                return Err(ParseError::InvalidFormat(part.to_string()));
            }
            action.set_parameter(kv[0], kv[1])?;
        }

        // Validate required parameters after all parameters are set
        action.validate_required_parameters()?;

        Ok(action)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}