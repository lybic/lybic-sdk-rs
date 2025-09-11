#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatsResponseDto {
    /// Number of MCP servers
    #[serde(rename = "mcpServers")]
    pub mcp_servers: f32,
    /// Number of sandboxes
    #[serde(rename = "sandboxes")]
    pub sandboxes: f32,
    /// Number of projects
    #[serde(rename = "projects")]
    pub projects: f32,
}

impl StatsResponseDto {
    pub fn new(mcp_servers: f32, sandboxes: f32, projects: f32) -> StatsResponseDto {
        StatsResponseDto {
            mcp_servers,
            sandboxes,
            projects,
        }
    }
}


