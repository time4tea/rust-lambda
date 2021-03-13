use std::collections::HashMap;
use std::str::FromStr;

use http::{HeaderValue, Request, Version, Response};
use http::header::HeaderName;
use serde::{Deserialize, Serialize, Deserializer};
use serde_json;
use core::fmt;
use serde::export::Formatter;

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
    #[serde(deserialize_with="deser_hashmap")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with="deser_hashmapvec")]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with="deser_hashmap")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with="deser_hashmap")]
    pub stage_variables: HashMap<String, String>,
    pub is_base64_encoded: bool,
}

fn deser_hashmap<'de, D>(d: D) -> Result<HashMap<String,String>, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or( HashMap::new())
        })
}

fn deser_hashmapvec<'de, D>(d: D) -> Result<HashMap<String,Vec<String>>, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or( HashMap::new())
        })
}

// {"httpMethod": "GET",
//   "body": null,
//   "resource": "/",
//   "requestContext":
//        {"resourceId": "123456",
//         "apiId": "1234567890",
//         "resourcePath": "/", "httpMethod": "GET",
//         "requestId": "c6af9ac6-7b61-11e6-9a41-93e8deadbeef",
//         "accountId": "123456789012", "stage": "Prod",
//         "identity": {"apiKey": null, "userArn": null,
//                       "cognitoAuthenticationType": null, "caller": null,
//                        "userAgent": "Custom User Agent String", "user": null,
//                       "cognitoIdentityPoolId": null, "cognitoAuthenticationProvider": null,
//                       "sourceIp": "127.0.0.1", "accountId": null},
//        "extendedRequestId": null, "path": "/", "protocol": "HTTP/1.1",
//       "domainName": "localhost:3000",
//        "requestTimeEpoch": 1615632659, "requestTime": "13/Mar/2021:10:50:59 +0000"}, "queryStringParameters": null, "multiValueQueryStringParameters": null, "headers": {"Host": "localhost:3000", "User-Agent": "curl/7.68.0", "Accept": "*/*", "X-Forwarded-Proto": "http", "X-Forwarded-Port": "3000"}, "multiValueHeaders": {"Host": ["localhost:3000"], "User-Agent": ["curl/7.68.0"], "Accept": ["*/*"], "X-Forwarded-Proto": ["http"], "X-Forwarded-Port": ["3000"]}, "pathParameters": null, "stageVariables": null, "path": "/", "isBase64Encoded": false}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiGatewayResponse {
    pub status_code: i32,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub is_base64_encoded: bool,
}

#[cfg(test)]
mod tests {
    use crate::apigateway::{ApiGatewayResponse, ApiGatewayRequest};
    use std::collections::HashMap;

    #[test]
    fn deserialising() -> Result<(), std::io::Error> {
        let req = r#"{"httpMethod": "GET", "body": null, "resource": "/", "requestContext": {"resourceId": "123456", "apiId": "1234567890", "resourcePath": "/", "httpMethod": "GET", "requestId": "c6af9ac6-7b61-11e6-9a41-93e8deadbeef", "accountId": "123456789012", "stage": "Prod", "identity": {"apiKey": null, "userArn": null, "cognitoAuthenticationType": null, "caller": null, "userAgent": "Custom User Agent String", "user": null, "cognitoIdentityPoolId": null, "cognitoAuthenticationProvider": null, "sourceIp": "127.0.0.1", "accountId": null}, "extendedRequestId": null, "path": "/", "protocol": "HTTP/1.1", "domainName": "localhost:3000", "requestTimeEpoch": 1615632659, "requestTime": "13/Mar/2021:10:50:59 +0000"}, "queryStringParameters": null, "multiValueQueryStringParameters": null, "headers": {"Host": "localhost:3000", "User-Agent": "curl/7.68.0", "Accept": "*/*", "X-Forwarded-Proto": "http", "X-Forwarded-Port": "3000"}, "multiValueHeaders": {"Host": ["localhost:3000"], "User-Agent": ["curl/7.68.0"], "Accept": ["*/*"], "X-Forwarded-Proto": ["http"], "X-Forwarded-Port": ["3000"]}, "pathParameters": null, "stageVariables": null, "path": "/", "isBase64Encoded": false}"#;
        let _x: ApiGatewayRequest = serde_json::from_str(req)?;
        Ok(())
    }

    #[test]
    fn serialising() {
        let response = ApiGatewayResponse {
            body: "Hello".to_string(),
            headers: HashMap::new(),
            is_base64_encoded: false,
            status_code: 200,
        };

        let _result = serde_json::to_string(&response);

    }
}

impl fmt::Display for ApiGatewayRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl fmt::Display for ApiGatewayResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
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

