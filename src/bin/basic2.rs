use std::error::Error;

use lambda_runtime::error::HandlerError;
use lambda_runtime_core::{Context, lambda};

use lambda_utils::apigateway::ApiGatewayResponse;
use lambda_utils::apigateway::ApiGatewayRequest;

use serde_json;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

/// Dump out the incoming request and reply with something stupid
fn lambda_handler(event: Vec<u8>, _ctx: Context) -> Result<Vec<u8>, HandlerError> {
    let x: ApiGatewayRequest = serde_json::from_slice(event.as_slice()).unwrap();
    println!("{}", x);

    let y = ApiGatewayResponse {
        status_code: 200,
        headers: Default::default(),
        body: "Hello, World!".to_string(),
        is_base64_encoded: false
    };

    Ok(serde_json::to_vec(&y)?)
}
