use crate::action_script::error::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    LeftClick,
}

impl FromStr for ActionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LeftClick" => Ok(ActionType::LeftClick),
            _ => Err(ParseError::InvalidActionType(s.to_string())),
        }
    }
}