#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSandboxResponseDtoConnectDetails {
    #[serde(rename = "gatewayAddresses")]
    pub gateway_addresses: Vec<crate::models::GetSandboxResponseDtoConnectDetailsGatewayAddresses>,
    #[serde(rename = "certificateHashBase64")]
    pub certificate_hash_base64: String,
    #[serde(rename = "endUserToken")]
    pub end_user_token: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
}

impl GetSandboxResponseDtoConnectDetails {
    pub fn new(gateway_addresses: Vec<crate::models::GetSandboxResponseDtoConnectDetailsGatewayAddresses>, certificate_hash_base64: String, end_user_token: String, room_id: String) -> GetSandboxResponseDtoConnectDetails {
        GetSandboxResponseDtoConnectDetails {
            gateway_addresses,
            certificate_hash_base64,
            end_user_token,
            room_id,
        }
    }
}


