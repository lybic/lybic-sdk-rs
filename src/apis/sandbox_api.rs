use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use async_trait::async_trait;
use http::Method;
use hyper_util::client::legacy::connect::Connect;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct SandboxApiClient<C: Connect> {
    configuration: Arc<configuration::Configuration<C>>, 
}

impl<C: Connect> SandboxApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> SandboxApiClient<C> {
        SandboxApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait SandboxApi: Send + Sync {
    async fn create_sandbox(&self, org_id: &str, create_sandbox_dto: Option<crate::models::CreateSandboxDto>) -> Result<crate::models::GetSandboxResponseDto, Error<serde_json::Value>>;
    async fn delete_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<serde_json::Value, Error<serde_json::Value>>;
    async fn execute_computer_use_action(&self, sandbox_id: &str, org_id: &str, computer_use_action_dto: Option<crate::models::ComputerUseActionDto>) -> Result<crate::models::SandboxActionResponseDto, Error<serde_json::Value>>;
    async fn extend_sandbox(&self, sandbox_id: &str, org_id: &str, extend_sandbox_dto: Option<crate::models::ExtendSandboxDto>) -> Result<serde_json::Value, Error<serde_json::Value>>;
    async fn extend_sandbox_0(&self, sandbox_id: &str, org_id: &str, extend_sandbox_dto: Option<crate::models::ExtendSandboxDto>) -> Result<serde_json::Value, Error<serde_json::Value>>;
    async fn get_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<crate::models::GetSandboxResponseDto, Error<serde_json::Value>>;
    async fn list_sandboxes(&self, org_id: &str) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>>;
    async fn preview_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<crate::models::SandboxActionResponseDto, Error<serde_json::Value>>;
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> SandboxApi for SandboxApiClient<C> {
    async fn create_sandbox(&self, org_id: &str, create_sandbox_dto: Option<crate::models::CreateSandboxDto>) -> Result<crate::models::GetSandboxResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/sandboxes".to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(create_sandbox_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn delete_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::DELETE, "/api/orgs/{orgId}/sandboxes/{sandboxId}".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn execute_computer_use_action(&self, sandbox_id: &str, org_id: &str, computer_use_action_dto: Option<crate::models::ComputerUseActionDto>) -> Result<crate::models::SandboxActionResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/sandboxes/{sandboxId}/actions/computer-use".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(computer_use_action_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn extend_sandbox(&self, sandbox_id: &str, org_id: &str, extend_sandbox_dto: Option<crate::models::ExtendSandboxDto>) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/sandboxes/{sandboxId}/extend".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(extend_sandbox_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn extend_sandbox_0(&self, sandbox_id: &str, org_id: &str, extend_sandbox_dto: Option<crate::models::ExtendSandboxDto>) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/sandboxes/{sandboxId}/extend".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(extend_sandbox_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn get_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<crate::models::GetSandboxResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::GET, "/api/orgs/{orgId}/sandboxes/{sandboxId}".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn list_sandboxes(&self, org_id: &str) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::GET, "/api/orgs/{orgId}/sandboxes".to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn preview_sandbox(&self, sandbox_id: &str, org_id: &str) -> Result<crate::models::SandboxActionResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/sandboxes/{sandboxId}/preview".to_string());
        req = req.with_path_param("sandboxId".to_string(), sandbox_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }
}
