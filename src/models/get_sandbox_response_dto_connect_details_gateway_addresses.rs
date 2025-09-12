#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSandboxResponseDtoConnectDetailsGatewayAddresses {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "port")]
    pub port: f32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preferredProviders")]
    pub preferred_providers: PreferredProviders,
    #[serde(rename = "gatewayType")]
    pub gateway_type: GatewayType,
}

impl GetSandboxResponseDtoConnectDetailsGatewayAddresses {
    pub fn new(
        address: String,
        port: f32,
        name: String,
        preferred_providers: PreferredProviders,
        gateway_type: GatewayType,
    ) -> GetSandboxResponseDtoConnectDetailsGatewayAddresses {
        GetSandboxResponseDtoConnectDetailsGatewayAddresses {
            address,
            port,
            name,
            preferred_providers,
            gateway_type,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PreferredProviders {
    #[serde(rename = "CHINA_TELECOM")]
    CHINATELECOM,
    #[serde(rename = "CHINA_UNICOM")]
    CHINAUNICOM,
    #[serde(rename = "CHINA_MOBILE")]
    CHINAMOBILE,
    #[serde(rename = "GLOBAL_BGP")]
    GLOBALBGP,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
    #[serde(rename = "4")]
    _4,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GatewayType {
    #[serde(rename = "KCP")]
    KCP,
    #[serde(rename = "QUIC")]
    QUIC,
    #[serde(rename = "WEB_TRANSPORT")]
    WEBTRANSPORT,
    #[serde(rename = "4")]
    _4,
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "6")]
    _6,
}
