use super::{Error, configuration};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_util::client::legacy::connect::Connect;
use serde::Serialize;
use std::collections::HashMap;

pub(crate) struct ApiKey {
    pub in_header: bool,
    pub in_query: bool,
    pub param_name: String,
}

impl ApiKey {
    fn key(&self, prefix: &Option<String>, key: &str) -> String {
        match prefix {
            None => key.to_owned(),
            Some(prefix) => format!("{} {}", prefix, key),
        }
    }
}

#[allow(dead_code)]
pub(crate) enum Auth {
    None,
    ApiKey(ApiKey),
}

pub(crate) struct Request {
    auth: Auth,
    method: http::Method,
    path: String,
    query_params: HashMap<String, String>,
    no_return_type: bool,
    path_params: HashMap<String, String>,
    form_params: HashMap<String, String>,
    header_params: HashMap<String, String>,
    serialized_body: Option<String>,
}

impl Request {
    pub fn new(method: http::Method, path: String) -> Self {
        Request {
            auth: Auth::None,
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            form_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub fn with_body_param<T: Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(serde_json::to_string(&param).unwrap());
        self
    }

    pub fn with_header_param(mut self, basename: String, param: String) -> Self {
        self.header_params.insert(basename, param);
        self
    }

    pub fn with_query_param(mut self, basename: String, param: String) -> Self {
        self.query_params.insert(basename, param);
        self
    }

    pub fn with_path_param(mut self, basename: String, param: String) -> Self {
        self.path_params.insert(basename, param);
        self
    }

    pub fn with_form_param(mut self, basename: String, param: String) -> Self {
        self.form_params.insert(basename, param);
        self
    }

    pub fn returns_nothing(mut self) -> Self {
        self.no_return_type = true;
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub async fn execute<C, U>(
        self,
        conf: &configuration::Configuration<C>,
    ) -> Result<U, Error<serde_json::Value>>
    where
        C: Connect + Clone + Send + Sync + 'static,
        U: Sized + Default, // 添加 Default 约束
        for<'de> U: serde::Deserialize<'de>,
    {
        let mut path = self.path;
        for (k, v) in self.path_params {
            path = path.replace(&format!("{{{}}}", k), &v);
        }

        let uri: http::Uri = format!(
            "{}{}?={}",
            conf.base_path,
            path,
            self.query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&")
        )
        .parse()
        .map_err(Error::from)?;

        let mut req_builder = http::Request::builder().method(self.method).uri(uri);

        // Headers
        if let Some(headers) = req_builder.headers_mut() {
            // Add headers from configuration
            headers.extend(conf.headers.clone());

            // Add headers from request builder
            for (key, value) in self.header_params {
                if let Ok(header_value) = http::header::HeaderValue::from_str(&value) {
                    headers.insert(
                        http::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                        header_value,
                    );
                }
            }

            // Add auth headers
            match self.auth {
                Auth::ApiKey(apikey) => {
                    if let Some(ref key) = conf.api_key {
                        let val = apikey.key(&key.prefix, &key.key);
                        if apikey.in_header {
                            if let Ok(header_value) = http::header::HeaderValue::from_str(&val) {
                                headers.insert(
                                    http::header::HeaderName::from_bytes(
                                        apikey.param_name.as_bytes(),
                                    )
                                    .unwrap(),
                                    header_value,
                                );
                            }
                        }
                    }
                }
                Auth::None => {}
            }

            if let Some(ref user_agent) = conf.user_agent {
                if let Ok(header_value) = http::header::HeaderValue::from_str(user_agent) {
                    headers.insert(http::header::USER_AGENT, header_value);
                }
            }
        }

        let body = if !self.form_params.is_empty() {
            if let Some(headers) = req_builder.headers_mut() {
                headers.insert(
                    http::header::CONTENT_TYPE,
                    http::header::HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
            }
            let mut enc = url::form_urlencoded::Serializer::new("".to_owned());
            for (k, v) in self.form_params {
                enc.append_pair(&k, &v);
            }
            Full::new(Bytes::from(enc.finish()))
        } else if let Some(body) = self.serialized_body {
            if let Some(headers) = req_builder.headers_mut() {
                headers.insert(
                    http::header::CONTENT_TYPE,
                    http::header::HeaderValue::from_static("application/json"),
                );
            }
            Full::new(Bytes::from(body))
        } else {
            Full::new(Bytes::default())
        };

        let req = req_builder.body(body)?;

        let resp = conf.client.request(req).await?;

        let (parts, body) = resp.into_parts();
        let status = parts.status;
        let body_bytes = body.collect().await?.to_bytes();

        if !status.is_success() {
            return Err(Error::from((status, &*body_bytes)));
        }

        Ok(if self.no_return_type {
            U::default()
        } else {
            serde_json::from_slice(&body_bytes)?
        })
    }
}
