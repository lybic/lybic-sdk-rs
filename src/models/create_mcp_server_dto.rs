#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMcpServerDto {
    /// Name of the MCP server.
    #[serde(rename = "name")]
    pub name: String,
    /// Project to which the MCP server belongs to.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The maximum lifetime of a sandbox.
    #[serde(rename = "sandboxMaxLifetimeSeconds", skip_serializing_if = "Option::is_none")]
    pub sandbox_max_lifetime_seconds: Option<f32>,
    /// The maximum idle time of a sandbox.
    #[serde(rename = "sandboxMaxIdleTimeSeconds", skip_serializing_if = "Option::is_none")]
    pub sandbox_max_idle_time_seconds: Option<f32>,
    /// Whether to create a new sandbox automatically when old sandbox is deleted. If not, new sandboxes will be created when calling computer use tools.
    #[serde(rename = "sandboxAutoCreation", skip_serializing_if = "Option::is_none")]
    pub sandbox_auto_creation: Option<bool>,
    /// Whether to expose recreate tool to LLMs.
    #[serde(rename = "sandboxExposeRecreateTool", skip_serializing_if = "Option::is_none")]
    pub sandbox_expose_recreate_tool: Option<bool>,
    /// Whether to expose restart tool to LLMs.
    #[serde(rename = "sandboxExposeRestartTool", skip_serializing_if = "Option::is_none")]
    pub sandbox_expose_restart_tool: Option<bool>,
    /// Whether to expose delete tool to LLMs.
    #[serde(rename = "sandboxExposeDeleteTool", skip_serializing_if = "Option::is_none")]
    pub sandbox_expose_delete_tool: Option<bool>,
}

impl CreateMcpServerDto {
    pub fn new(name: String) -> CreateMcpServerDto {
        CreateMcpServerDto {
            name,
            project_id: None,
            sandbox_max_lifetime_seconds: None,
            sandbox_max_idle_time_seconds: None,
            sandbox_auto_creation: None,
            sandbox_expose_recreate_tool: None,
            sandbox_expose_restart_tool: None,
            sandbox_expose_delete_tool: None,
        }
    }
}


