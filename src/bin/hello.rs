use std::collections::HashMap;
use std::error::Error;

use lambda_runtime::{Context, error::HandlerError, lambda};

use lambda_utils::apigateway::ApiGatewayResponse;
use lambda_utils::apigateway::ApiGatewayRequest;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

/// respond to an API Gateway Proxy Request with some static content
fn lambda_handler(_e: ApiGatewayRequest, _c: Context) -> Result<ApiGatewayResponse, HandlerError> {
    Ok(ApiGatewayResponse {
        body: "Hello".to_string(),
        headers: HashMap::new(),
        is_base64_encoded: false,
        status_code: 200,
    })
}
