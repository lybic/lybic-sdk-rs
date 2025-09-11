use http;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    RequestError(http::Error),
    UriError(http::uri::InvalidUri),
    HyperError(hyper_util::client::legacy::Error),
    BodyError(hyper::Error),
    SerdeError(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: http::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(http::StatusCode, &'de [u8])> for Error<T>
where
    T: serde::Deserialize<'de>,
{
    fn from(e: (http::StatusCode, &'de [u8])) -> Self {
        if e.1.is_empty() {
            return Error::ApiError(ApiError {
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError {
                code: e.0,
                content: Some(t),
            }),
            Err(e) => Error::from(e),
        }
    }
}

impl<T> From<hyper_util::client::legacy::Error> for Error<T> {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::HyperError(e)
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        Error::BodyError(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeError(e)
    }
}

impl<T> From<http::uri::InvalidUri> for Error<T> {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::UriError(e)
    }
}

impl<T> From<http::Error> for Error<T> {
    fn from(e: http::Error) -> Self {
        Error::RequestError(e)
    }
}

mod request;

mod computer_use_api;
pub use self::computer_use_api::{ComputerUseApi, ComputerUseApiClient};
mod dashboard_api;
pub use self::dashboard_api::{DashboardApi, DashboardApiClient};
mod mcp_servers_api;
pub use self::mcp_servers_api::{McpServersApi, McpServersApiClient};
mod project_api;
pub use self::project_api::{ProjectApi, ProjectApiClient};
mod sandbox_api;
pub use self::sandbox_api::{SandboxApi, SandboxApiClient};

pub mod configuration;
pub mod client;