use std::collections::HashMap;
use std::str::FromStr;

use http::{HeaderValue, Request, Version, Response};
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

impl From<Response<String>> for ApiGatewayResponse {
    fn from(value: Response<String>) -> Self {

        let mut headers: HashMap<String,String> = HashMap::new();
        for ( key, value) in  value.headers().iter() {
            headers.insert(key.to_string(), String::from(value.to_str().unwrap()));
        }

        ApiGatewayResponse {
            body: value.body().clone(),
            status_code: value.status().as_u16() as i32,
            is_base64_encoded: false,
            headers
        }
    }
}

impl From<ApiGatewayRequest> for Request<String> {
    fn from(value: ApiGatewayRequest) -> Self {
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

        r.body(b.to_string()).unwrap()
    }
}

