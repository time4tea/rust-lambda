use std::collections::HashMap;
use std::str::FromStr;

use http::{HeaderValue, Request, Version};
use http::header::HeaderName;
use serde::{Deserialize, Serialize};
use serde::export::TryFrom;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayRequest {
    pub http_method: String,
    pub body: Option<String>,
    pub resource: String,
    pub version: Option<String>,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub multi_value_headers: HashMap<String, Vec<String>>,
    pub query_string_parameters: Option<HashMap<String, String>>,
    pub multi_value_query_string_parameters: Option<HashMap<String, Vec<String>>>,
    pub path_parameters: HashMap<String, String>,
    pub stage_variables: Option<HashMap<String, String>>,
    pub is_base64_encoded: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayResponse {
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub is_base64_encoded: bool,
}

impl TryFrom<ApiGatewayRequest> for Request<String> {
    type Error = http::Error;

    fn try_from(value: ApiGatewayRequest) -> Result<Self, Self::Error> {
        let mut r = Request::builder()
            .method(value.http_method.as_str())
            .uri(value.path.as_str())
            .version(Version::HTTP_11);

        for (name, value) in value.headers.iter() {
            r = r.header(HeaderName::from_str(name.as_str()).unwrap(), HeaderValue::from_str(value.as_str()).unwrap());
        }

        let x = String::new();
        let b = match &value.body {
            None => &x,
            Some(v) => v
        };

        r.body(b.to_string())
    }
}

