use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;

use http::Request;
use lambda_runtime::{Context, error::HandlerError, lambda};

use lambda_utils::apigateway::ApiGatewayRequest;
use lambda_utils::apigateway::ApiGatewayResponse;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

/// respond to an API Gateway Proxy Request with some static content
fn lambda_handler(req: ApiGatewayRequest, _c: Context) -> Result<ApiGatewayResponse, HandlerError> {
    let response = Request::try_from(req)
        .map(|r| {
            println!("{:?}", r);
            ApiGatewayResponse {
                body: "Hello".to_string(),
                headers: HashMap::new(),
                is_base64_encoded: false,
                status_code: 200,
            }
        }).unwrap();
    Ok(response)
}

