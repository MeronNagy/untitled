use crate::action_script::error::ParseError;
use crate::action_script::types::ActionType;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ParamValue {
    Integer(i32),
    // not used atm String(String),
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: ActionType,
    parameters: HashMap<String, ParamValue>,
}

impl Action {
    pub fn new(action_type: ActionType) -> Self {
        Self {
            action_type,
            parameters: HashMap::new(),
        }
    }

    pub fn get_integer_parameter(&self, key: &str) -> Result<i32, String> {
        match self.parameters.get(key) {
            Some(ParamValue::Integer(value)) => Ok(*value),
            // not used atm Some(_) => Err(format!("Parameter {} is not an integer", key)),
            None => Err(format!("Parameter {} not found", key)),
        }
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ParseError> {
        match key {
            "X" | "Y" => {
                let int_value = Self::parse_string_to_int(value)?;
                self.parameters
                    .insert(key.to_string(), ParamValue::Integer(int_value));
                Ok(())
            }
            "Delay" => {
                let int_value = Self::parse_string_to_positive_int(value)?;
                self.parameters
                    .insert(key.to_string(), ParamValue::Integer(int_value));
                Ok(())
            }
            _ => Err(ParseError::InvalidParameter(key.to_string())),
        }
    }

    fn parse_string_to_int(value: &str) -> Result<i32, ParseError> {
        value
            .parse::<i32>()
            .map_err(|_| ParseError::InvalidParameterValue {
                parameter: value.to_string(),
                reason: "Value must be an integer".to_string(),
            })
    }

    fn parse_string_to_positive_int(value: &str) -> Result<i32, ParseError> {
        let parsed_value = Self::parse_string_to_int(value)?;

        if parsed_value <= 0 {
            return Err(ParseError::InvalidParameterValue {
                parameter: value.to_string(),
                reason: "Value must be a positive integer".to_string(),
            });
        }

        Ok(parsed_value)
    }



    fn validate_required_parameters(&self) -> Result<(), ParseError> {
        if self.action_type == ActionType::LeftClick {
            self.validate_integer_param_exists("X")?;
            self.validate_integer_param_exists("Y")?;
            self.validate_integer_param_exists("Delay")?;
        }

        Ok(())
    }

    fn validate_integer_param_exists(&self, name: &str) -> Result<(), ParseError> {
        self.parameters
            .get(name)
            .and_then(|param| matches!(param, ParamValue::Integer(_)).then_some(()))
            .ok_or_else(|| ParseError::MissingParameter(name.to_string()))
    }

    pub fn as_string(&self) -> String {
        let mut parts = vec![format!("{:?}", self.action_type)];

        for (key, value) in &self.parameters {
            let value_str = match value {
                ParamValue::Integer(i) => i.to_string(),
                // not used atm ParamValue::String(s) => s.clone(),
            };
            parts.push(format!("{}={}", key, value_str));
        }

        parts.join("; ")
    }

    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = s
            .split(';')
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

        action.validate_required_parameters()?;
        Ok(action)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
