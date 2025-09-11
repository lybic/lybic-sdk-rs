use serde::{Serialize, Deserialize};
use super::computer_use_action::ComputerUseAction;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseActionResponseDto {
    #[serde(rename = "unknown", skip_serializing_if = "Option::is_none")]
    pub unknown: Option<String>,
    #[serde(rename = "thoughts", skip_serializing_if = "Option::is_none")]
    pub thoughts: Option<String>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "actions")]
    pub actions: Vec<ComputerUseAction>,
}
