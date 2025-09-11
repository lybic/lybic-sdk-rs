use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use async_trait::async_trait;
use http::Method;
use hyper_util::client::legacy::connect::Connect;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct ComputerUseApiClient<C: Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> ComputerUseApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ComputerUseApiClient<C> {
        ComputerUseApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait ComputerUseApi: Send + Sync {
    async fn parse_model_output(&self, computer_use_parse_request_dto: Option<crate::models::ComputerUseParseRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>>;
    async fn parse_model_text_output(&self, _type: &str, computer_use_parse_text_request_dto: Option<crate::models::ComputerUseParseTextRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>>;
    async fn parse_model_text_output_0(&self, _type: &str, computer_use_parse_text_request_dto: Option<crate::models::ComputerUseParseTextRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>>;
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> ComputerUseApi for ComputerUseApiClient<C> {
    async fn parse_model_output(&self, computer_use_parse_request_dto: Option<crate::models::ComputerUseParseRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/computer-use/parse".to_string());
        req = req.with_body_param(computer_use_parse_request_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn parse_model_text_output(&self, _type: &str, computer_use_parse_text_request_dto: Option<crate::models::ComputerUseParseTextRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/computer-use/parse/{type}".to_string());
        req = req.with_path_param("type".to_string(), _type.to_string());
        req = req.with_body_param(computer_use_parse_text_request_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn parse_model_text_output_0(&self, _type: &str, computer_use_parse_text_request_dto: Option<crate::models::ComputerUseParseTextRequestDto>) -> Result<crate::models::ComputerUseActionResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/computer-use/parse/{type}".to_string());
        req = req.with_path_param("type".to_string(), _type.to_string());
        req = req.with_body_param(computer_use_parse_text_request_dto);

        req.execute(self.configuration.borrow()).await
    }
}