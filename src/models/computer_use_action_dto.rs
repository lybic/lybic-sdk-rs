use super::computer_use_action::ComputerUseAction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseActionDto {
    #[serde(rename = "action")]
    pub action: ComputerUseAction,
    #[serde(rename = "includeScreenShot")]
    pub include_screen_shot: bool,
    #[serde(rename = "includeCursorPosition")]
    pub include_cursor_position: bool,
    #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}
