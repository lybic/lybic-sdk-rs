#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxActionResponseDto {
    /// The screenshot of the sandbox after the action is executed.
    #[serde(rename = "screenShot", skip_serializing_if = "Option::is_none")]
    pub screen_shot: Option<String>,
    #[serde(rename = "cursorPosition", skip_serializing_if = "Option::is_none")]
    pub cursor_position: Option<crate::models::SandboxActionResponseDtoCursorPosition>,
}

impl SandboxActionResponseDto {
    pub fn new() -> SandboxActionResponseDto {
        SandboxActionResponseDto {
            screen_shot: None,
            cursor_position: None,
        }
    }
}
