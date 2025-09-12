#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSandboxDto {
    /// The name of the sandbox.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The maximum life time of the sandbox in seconds. Default is 1 hour, max is 1 day.
    #[serde(rename = "maxLifeSeconds", skip_serializing_if = "Option::is_none")]
    pub max_life_seconds: Option<f32>,
    /// The project id to use for the sandbox. Use default if not provided.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The spec of the sandbox. Use default if not provided.
    #[serde(rename = "specId", skip_serializing_if = "Option::is_none")]
    pub spec_id: Option<String>,
    /// The datacenter id to use for the sandbox. Use default if not provided.
    #[serde(rename = "datacenterId", skip_serializing_if = "Option::is_none")]
    pub datacenter_id: Option<String>,
}

impl CreateSandboxDto {
    pub fn new() -> CreateSandboxDto {
        CreateSandboxDto {
            name: None,
            max_life_seconds: None,
            project_id: None,
            spec_id: None,
            datacenter_id: None,
        }
    }
}
