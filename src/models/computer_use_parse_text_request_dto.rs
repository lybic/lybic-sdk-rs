#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseParseTextRequestDto {
    /// The text content to parse
    #[serde(rename = "textContent")]
    pub text_content: String,
}

impl ComputerUseParseTextRequestDto {
    pub fn new(text_content: String) -> ComputerUseParseTextRequestDto {
        ComputerUseParseTextRequestDto {
            text_content,
        }
    }
}


