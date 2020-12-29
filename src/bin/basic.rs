use std::error::Error;

use lambda_runtime::error::HandlerError;
use lambda_runtime_core::{Context, lambda};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

/// Dump out the incoming request and reply with something stupid
fn lambda_handler(event: Vec<u8>, _ctx: Context) -> Result<Vec<u8>, HandlerError> {
    let x = String::from_utf8(event).unwrap();
    println!("{}", x);
    Ok("Hello".to_string().into_bytes())
}
