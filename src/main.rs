
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LambdaRequest {
    full_name: String,
    message: Option<String>,
}


// private Integer statusCode;
// private Map<String, String> headers;
// private String body;
// private Boolean isBase64Encoded;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ApiGatewayResponse {
    status_code: i32,
    headers: HashMap<String, String>,
    body: String,
    is_base64_encoded: bool
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LambdaResponse {
    lambda_request: LambdaRequest,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(e: LambdaRequest, _c: Context) -> Result<LambdaResponse, HandlerError> {
    let mut e = e.clone();
    e.full_name = format!("Hello {name}!", name = e.full_name);
    let msg = match e.message {
        Some(msg) => format!("Your message is '{msg}'.", msg = msg),
        None => format!("You have no message."),
    };
    e.message = Some(msg);
    Ok(LambdaResponse { lambda_request: e })
}
