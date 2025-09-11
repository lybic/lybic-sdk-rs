use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use async_trait::async_trait;
use http::Method;
use hyper_util::client::legacy::connect::Connect;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct ProjectApiClient<C: Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> ProjectApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ProjectApiClient<C> {
        ProjectApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait ProjectApi: Send + Sync {
    async fn create_project(&self, org_id: &str, create_project_dto: Option<crate::models::CreateProjectDto>) -> Result<crate::models::SingleProjectResponseDto, Error<serde_json::Value>>;
    async fn delete_project(&self, project_id: &str, org_id: &str) -> Result<serde_json::Value, Error<serde_json::Value>>;
    async fn list_projects(&self, org_id: &str) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>>;
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> ProjectApi for ProjectApiClient<C> {
    async fn create_project(&self, org_id: &str, create_project_dto: Option<crate::models::CreateProjectDto>) -> Result<crate::models::SingleProjectResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::POST, "/api/orgs/{orgId}/projects".to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());
        req = req.with_body_param(create_project_dto);

        req.execute(self.configuration.borrow()).await
    }

    async fn delete_project(&self, project_id: &str, org_id: &str) -> Result<serde_json::Value, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::DELETE, "/api/orgs/{orgId}/projects/{projectId}".to_string());
        req = req.with_path_param("projectId".to_string(), project_id.to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }

    async fn list_projects(&self, org_id: &str) -> Result<Vec<serde_json::Value>, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::GET, "/api/orgs/{orgId}/projects".to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }
}