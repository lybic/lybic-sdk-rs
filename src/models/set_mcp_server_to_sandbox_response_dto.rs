#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetMcpServerToSandboxResponseDto {
    /// The ID of the sandbox to connect the MCP server to.
    #[serde(rename = "sandboxId")]
    pub sandbox_id: Option<String>,
}

impl SetMcpServerToSandboxResponseDto {
    pub fn new(sandbox_id: Option<String>) -> SetMcpServerToSandboxResponseDto {
        SetMcpServerToSandboxResponseDto { sandbox_id }
    }
}
