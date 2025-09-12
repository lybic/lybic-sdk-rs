#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSandboxResponseDtoSandbox {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Deprecated, use `expiresAt` instead.
    #[serde(rename = "expiredAt")]
    pub expired_at: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
}

impl GetSandboxResponseDtoSandbox {
    pub fn new(
        id: String,
        name: String,
        expired_at: String,
        expires_at: String,
        created_at: String,
        project_id: String,
    ) -> GetSandboxResponseDtoSandbox {
        GetSandboxResponseDtoSandbox {
            id,
            name,
            expired_at,
            expires_at,
            created_at,
            project_id,
        }
    }
}
