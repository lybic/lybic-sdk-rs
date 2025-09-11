use std::sync::Arc;
use super::configuration::Configuration;
use hyper_util::client::legacy::connect::Connect;


// #[cfg(feature = "default-client")]
// pub fn new() -> LybicClient {
//     use hyper_util::client::legacy::Client;
//     use hyper_util::rt::TokioExecutor;
// 
//     let https = hyper_tls::HttpsConnector::new();
//     let client = Client::builder(TokioExecutor::new()).build(https);
//     let config = Configuration::new(client);
//     LybicClient::new_with_configuration(config)
// }

pub struct LybicClient {
    computer_use_api: Box<dyn crate::apis::ComputerUseApi>,
    dashboard_api: Box<dyn crate::apis::DashboardApi>,
    mcp_servers_api: Box<dyn crate::apis::McpServersApi>,
    project_api: Box<dyn crate::apis::ProjectApi>,
    sandbox_api: Box<dyn crate::apis::SandboxApi>,
}

impl LybicClient {
    pub fn new_with_configuration<C>(configuration: Configuration<C>) -> LybicClient
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        let arc = Arc::new(configuration);

        LybicClient {
            computer_use_api: Box::new(crate::apis::ComputerUseApiClient::new(arc.clone())),
            dashboard_api: Box::new(crate::apis::DashboardApiClient::new(arc.clone())),
            mcp_servers_api: Box::new(crate::apis::McpServersApiClient::new(arc.clone())),
            project_api: Box::new(crate::apis::ProjectApiClient::new(arc.clone())),
            sandbox_api: Box::new(crate::apis::SandboxApiClient::new(arc.clone())),
        }
    }

    pub fn computer_use_api(&self) -> &dyn crate::apis::ComputerUseApi {
        self.computer_use_api.as_ref()
    }

    pub fn dashboard_api(&self) -> &dyn crate::apis::DashboardApi {
        self.dashboard_api.as_ref()
    }

    pub fn mcp_servers_api(&self) -> &dyn crate::apis::McpServersApi {
        self.mcp_servers_api.as_ref()
    }

    pub fn project_api(&self) -> &dyn crate::apis::ProjectApi {
        self.project_api.as_ref()
    }

    pub fn sandbox_api(&self) -> &dyn crate::apis::SandboxApi {
        self.sandbox_api.as_ref()
    }
}