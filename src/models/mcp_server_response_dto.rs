#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct McpServerResponseDto {
    /// ID of the MCP server.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the MCP server.
    #[serde(rename = "name")]
    pub name: String,
    /// Creation date of the MCP server.
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Whether this is the default MCP server for the organization.
    #[serde(rename = "defaultMcpServer")]
    pub default_mcp_server: bool,
    /// Project ID to which the MCP server belongs.
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "policy")]
    pub policy: crate::models::McpServerResponseDtoPolicy,
    /// ID of the currently connected sandbox.
    #[serde(rename = "currentSandboxId")]
    pub current_sandbox_id: Option<String>,
}

impl McpServerResponseDto {
    pub fn new(
        id: String,
        name: String,
        created_at: String,
        default_mcp_server: bool,
        project_id: String,
        policy: crate::models::McpServerResponseDtoPolicy,
        current_sandbox_id: Option<String>,
    ) -> McpServerResponseDto {
        McpServerResponseDto {
            id,
            name,
            created_at,
            default_mcp_server,
            project_id,
            policy,
            current_sandbox_id,
        }
    }
}
