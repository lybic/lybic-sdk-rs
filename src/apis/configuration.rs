use http::header::{HeaderMap, HeaderValue};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::Connect;
use std::env;

pub struct Configuration<C: Connect> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: Client<C, Full<Bytes>>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    pub headers: HeaderMap<HeaderValue>,
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl<C: Connect + Clone + Send + Sync + 'static> Configuration<C> {
    pub fn new(client: Client<C, Full<Bytes>>) -> Configuration<C> {
        let mut headers = HeaderMap::new();
        if let Ok(api_key) = env::var("LYBIC_API_KEY") {
            if !api_key.is_empty() {
                if let Ok(header_value) = HeaderValue::from_str(&api_key) {
                    headers.insert("x-api-key", header_value);
                }
            }
        }

        Configuration {
            base_path: env::var("LYBIC_API_ENDPOINT")
                .unwrap_or_else(|_| "https://api.lybic.cn".to_owned()),
            user_agent: Some("lybic-sdk-rust".to_owned()),
            client,
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
            headers,
        }
    }
}
