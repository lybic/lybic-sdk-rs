use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use async_trait::async_trait;
use http::Method;
use hyper_util::client::legacy::connect::Connect;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DashboardApiClient<C: Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> DashboardApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> DashboardApiClient<C> {
        DashboardApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait DashboardApi: Send + Sync {
    async fn get_stats(&self, org_id: &str) -> Result<crate::models::StatsResponseDto, Error<serde_json::Value>>;
}

#[async_trait]
impl<C: Connect + Clone + Send + Sync + 'static> DashboardApi for DashboardApiClient<C> {
    async fn get_stats(&self, org_id: &str) -> Result<crate::models::StatsResponseDto, Error<serde_json::Value>> {
        let mut req = __internal_request::Request::new(Method::GET, "/api/orgs/{orgId}/stats".to_string());
        req = req.with_path_param("orgId".to_string(), org_id.to_string());

        req.execute(self.configuration.borrow()).await
    }
}