use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::sync::Arc;

use async_trait::async_trait;
use http::Method;
use hyper_util::client::legacy::connect::Connect;

use super::request as __internal_request;
use super::{Error, configuration};

pub struct McpServersApiClient<C: Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> McpServersApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> McpServersApiClient<C> {
        McpServersApiClient { configuration }
    }
}

#[async_trait]
pub trait McpServersApi: Send + Sync {
    async fn create_mcp_server(
        &self,
        org_id: &str,
        create_mcp_server_dto: Option<crate::models::CreateMcpServerDto>,
    ) -> Result<crate::models::McpServerResponseDto, Error<serde_json::Value>>;
    async fn delete_mcp_server(
        &self,
        mcp_server_id: &str,
        org_id: &str,
    ) -> Result<serde_json::Value, Error<serde_json::Value>>;
    async fn get_default_mcp_server(
        &self,
        org_id: &str,
    ) -> Result<crate::models::McpServerResponseDto, Error<serde_json::Value>>;
    async fn list_mcp_servers(
        &self,
        org_id: &str,
    ) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>>;
    async fn set_mcp_server_to_sandbox(
        &self,
        mcp_server_id: &str,
        org_id: &str,
        set_mcp_server_to_sandbox_response_dto: Option<
            crate::models::SetMcpServerToSandboxResponseDto,
        >,
    ) -> Result<serde_json::Value, Error<serde_json::Value>>;
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> McpServersApi for McpServersApiClient<C> {
    async fn create_mcp_server(
        &self,
        org_id: &str,
        create_mcp_server_dto: Option<crate::models::CreateMcpServerDto>,
    ) -> Result<crate::models::McpServerResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(
            Method::POST,
            "/api/orgs/{orgId}/mcp-servers".to_string(),
        );
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(create_mcp_server_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn delete_mcp_server(
        &self,
        mcp_server_id: &str,
        org_id: &str,
    ) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(
            Method::DELETE,
            "/api/orgs/{orgId}/mcp-servers/{mcpServerId}".to_string(),
        );
        req = req.with_path_param("mcpServerId".to_string(), mcp_server_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn get_default_mcp_server(
        &self,
        org_id: &str,
    ) -> Result<crate::models::McpServerResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            "/api/orgs/{orgId}/mcp-servers/default".to_string(),
        );
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn list_mcp_servers(
        &self,
        org_id: &str,
    ) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(
            Method::GET,
            "/api/orgs/{orgId}/mcp-servers".to_string(),
        );
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn set_mcp_server_to_sandbox(
        &self,
        mcp_server_id: &str,
        org_id: &str,
        set_mcp_server_to_sandbox_response_dto: Option<
            crate::models::SetMcpServerToSandboxResponseDto,
        >,
    ) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(
            Method::POST,
            "/api/orgs/{orgId}/mcp-servers/{mcpServerId}/sandbox".to_string(),
        );
        req = req.with_path_param("mcpServerId".to_string(), mcp_server_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(set_mcp_server_to_sandbox_response_dto);

        req.execute(self.configuration.borrow()).await
    }
}
