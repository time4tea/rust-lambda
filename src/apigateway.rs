use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayRequest {
    pub http_method: Option<String>,
    pub body: Option<String>,
    pub resource: Option<String>,
    pub version: Option<String>,
    pub path: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub multi_value_headers: Option<HashMap<String, Vec<String>>>,
    pub query_string_parameters: Option<HashMap<String, String>>,
    pub multi_value_query_string_parameters: Option<HashMap<String, Vec<String>>>,
    pub path_parameters: Option<HashMap<String, String>>,
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
