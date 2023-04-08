use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayProxyRequest,
        ApiGatewayProxyResponse
    }
};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Context, Error};

fn main() -> Result<(), Error> {
    println!("cold start");
    let processor = service_fn(handler);
    lambda_runtime::service_fn(processor);
    Ok(())
}

async fn handler(
    _: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    println!("handler");
    let response = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(("Boop").to_string())),
        is_base64_encoded: Some(false)
    };
    Ok(response)
}