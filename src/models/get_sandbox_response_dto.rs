#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSandboxResponseDto {
    #[serde(rename = "sandbox")]
    pub sandbox: crate::models::GetSandboxResponseDtoSandbox,
    #[serde(rename = "connectDetails")]
    pub connect_details: crate::models::GetSandboxResponseDtoConnectDetails,
}

impl GetSandboxResponseDto {
    pub fn new(sandbox: crate::models::GetSandboxResponseDtoSandbox, connect_details: crate::models::GetSandboxResponseDtoConnectDetails) -> GetSandboxResponseDto {
        GetSandboxResponseDto {
            sandbox,
            connect_details,
        }
    }
}


