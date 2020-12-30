use std::collections::HashMap;
use std::str::FromStr;

use http::{Error, HeaderValue, Request, Version};
use http::header::HeaderName;
use serde::{Deserialize, Serialize};

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

impl ApiGatewayRequest {
    pub fn as_http_request(&self) -> Result<Request<String>, Error> {
        let mut r = Request::builder()
            .method(self.http_method.as_str())
            .uri(self.path.as_str())
            .version(Version::HTTP_11);

        for (name, value) in self.headers.iter() {
            r = r.header(HeaderName::from_str(name.as_str()).unwrap(), HeaderValue::from_str(value.as_str()).unwrap());
        }


        let x = String::new();
        let b = match &self.body {
            None => &x,
            Some(v) => v
        };

        r.body(b.to_string())
    }
}


