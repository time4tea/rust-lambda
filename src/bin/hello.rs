use std::collections::HashMap;
use std::error::Error;

use lambda_runtime::{Context, error::HandlerError, lambda};

use lambda_utils::apigateway::ApiGatewayRequest;
use lambda_utils::apigateway::ApiGatewayResponse;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(_e: ApiGatewayRequest, _c: Context) -> Result<ApiGatewayResponse, HandlerError> {
    Ok(ApiGatewayResponse {
        body: "Hello".to_string(),
        headers: HashMap::new(),
        is_base64_encoded: false,
        status_code: 200,
    })
}
