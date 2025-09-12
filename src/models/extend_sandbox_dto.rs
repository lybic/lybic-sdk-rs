#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendSandboxDto {
    /// The new max life time of the sandbox (relative to the current time) in seconds. Should not less than 30 seconds or more than 24 hours. Note that the total maximum lifetime of a sandbox should not longer than 13 days.
    #[serde(rename = "maxLifeSeconds", skip_serializing_if = "Option::is_none")]
    pub max_life_seconds: Option<f32>,
}

impl ExtendSandboxDto {
    pub fn new() -> ExtendSandboxDto {
        ExtendSandboxDto {
            max_life_seconds: None,
        }
    }
}
