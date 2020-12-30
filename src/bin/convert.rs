use std::error::Error;

use http::Request;
use lambda_runtime::{Context, error::HandlerError, lambda};

use lambda_utils::apigateway::ApiGatewayRequest;
use lambda_utils::apigateway::ApiGatewayResponse;
use http::header::HeaderValue;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {

    let runtime = Runtime::new().unwrap();

    lambda!(lambda_handler, runtime);
    Ok(())
}

/// respond to an API Gateway Proxy Request with some static content
fn lambda_handler(req: ApiGatewayRequest, _c: Context) -> Result<ApiGatewayResponse, HandlerError> {

    let http_response = http::Response::builder()
        .status(200)
        .header(http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body("Hello".to_string())
        .unwrap();

    let request = Request::from(req);
    println!("{:?}", request);

    Ok(ApiGatewayResponse::from(http_response))
}

