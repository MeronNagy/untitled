use crate::action_script::error::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    MoveMouse,
    LeftClick,
    Keystroke,
}

impl FromStr for ActionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MoveMouse" => Ok(ActionType::MoveMouse),
            "LeftClick" => Ok(ActionType::LeftClick),
            "Keystroke" => Ok(ActionType::Keystroke),
            _ => Err(ParseError::InvalidActionType(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimulationType {
    Physical,
    Direct,
}

impl FromStr for SimulationType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Physical" => Ok(SimulationType::Physical),
            "Direct" => Ok(SimulationType::Direct),
            _ => Err(ParseError::InvalidSimulationType(s.to_string())),
        }
    }
}
