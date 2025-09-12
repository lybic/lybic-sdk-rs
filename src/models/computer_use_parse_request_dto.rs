#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseParseRequestDto {
    /// The model to parse the input for
    #[serde(rename = "model")]
    pub model: Model,
    /// The text content to parse
    #[serde(rename = "textContent")]
    pub text_content: String,
}

impl ComputerUseParseRequestDto {
    pub fn new(model: Model, text_content: String) -> ComputerUseParseRequestDto {
        ComputerUseParseRequestDto {
            model,
            text_content,
        }
    }
}

/// The model to parse the input for
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "ui-tars")]
    UiTars,
    #[serde(rename = "seed")]
    Seed,
    #[serde(rename = "oai-compute-use")]
    OaiComputeUse,
}
