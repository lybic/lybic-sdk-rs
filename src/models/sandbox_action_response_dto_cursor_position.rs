#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxActionResponseDtoCursorPosition {
    /// The x position of the cursor.
    #[serde(rename = "x")]
    pub x: f32,
    /// The y position of the cursor.
    #[serde(rename = "y")]
    pub y: f32,
    /// The width of the screen.
    #[serde(rename = "screenWidth")]
    pub screen_width: f32,
    /// The height of the screen.
    #[serde(rename = "screenHeight")]
    pub screen_height: f32,
    /// The index of the screen.
    #[serde(rename = "screenIndex")]
    pub screen_index: f32,
}

impl SandboxActionResponseDtoCursorPosition {
    pub fn new(
        x: f32,
        y: f32,
        screen_width: f32,
        screen_height: f32,
        screen_index: f32,
    ) -> SandboxActionResponseDtoCursorPosition {
        SandboxActionResponseDtoCursorPosition {
            x,
            y,
            screen_width,
            screen_height,
            screen_index,
        }
    }
}
